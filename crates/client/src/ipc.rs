use anyhow::Result;
use gameverse_protocol::{
    adapter::{decode, encode, Message},
    frame_length,
};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
pub async fn read<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Message> {
    let mut prefix = [0; 4];
    reader.read_exact(&mut prefix).await?;
    let mut bytes = vec![0; frame_length(prefix)?];
    reader.read_exact(&mut bytes).await?;
    Ok(decode(&bytes)?)
}
pub async fn write<W: AsyncWrite + Unpin>(writer: &mut W, message: &Message) -> Result<()> {
    writer.write_all(&encode(message)?).await?;
    writer.flush().await?;
    Ok(())
}

#[cfg(windows)]
pub mod bridge {
    use super::*;
    use crate::presence::{Client, Replica};
    use gameverse_protocol::{adapter, EntityId};
    use std::{
        collections::BTreeSet,
        net::SocketAddr,
        path::Path,
        time::{Duration, Instant},
    };
    use tokio::{net::windows::named_pipe::ServerOptions, sync::mpsc, time::timeout};
    const DEADLINE: Duration = Duration::from_secs(5);
    struct ReaderGuard(tokio::task::JoinHandle<()>);
    impl Drop for ReaderGuard {
        fn drop(&mut self) {
            self.0.abort();
        }
    }

    pub async fn run(
        pipe: &str,
        server: SocketAddr,
        cert: &Path,
        duration: Duration,
    ) -> Result<()> {
        anyhow::ensure!(
            pipe.starts_with(r"\\.\pipe\") && !duration.is_zero(),
            "invalid pipe or duration"
        );
        let finish = Instant::now() + duration;
        // First-instance flag avoids silently attaching to a pipe owned by another process.
        let mut listener = ServerOptions::new()
            .first_pipe_instance(true)
            .reject_remote_clients(true)
            .create(pipe)?;
        println!("{}", serde_json::json!({"event":"pipe_ready","pipe":pipe}));
        loop {
            let left = finish.saturating_duration_since(Instant::now());
            if left.is_zero() {
                return Ok(());
            }
            tokio::select! {
                result=timeout(left,listener.connect())=> {if result.is_err(){return Ok(());}result??;},
                _=tokio::signal::ctrl_c()=>return Ok(()),
            }
            let next = ServerOptions::new()
                .reject_remote_clients(true)
                .create(pipe)?;
            let connected = std::mem::replace(&mut listener, next);
            let result = tokio::select! {
                result=timeout(finish.saturating_duration_since(Instant::now()),serve(connected,server,cert))=>result,
                _=tokio::signal::ctrl_c()=>return Ok(()),
            };
            match result {
                Ok(Err(error)) => eprintln!(
                    "{}",
                    serde_json::json!({"event":"adapter_disconnected","error":error.to_string()})
                ),
                Err(_) => return Ok(()),
                Ok(Ok(())) => {}
            }
        }
    }
    async fn serve(
        stream: tokio::net::windows::named_pipe::NamedPipeServer,
        server: SocketAddr,
        cert: &Path,
    ) -> Result<()> {
        let (mut rx, mut tx) = tokio::io::split(stream);
        let hello = timeout(DEADLINE, read(&mut rx)).await??;
        anyhow::ensure!(
            matches!(
                hello,
                Message::AdapterHello {
                    version: adapter::VERSION,
                    ..
                }
            ),
            "unsupported adapter protocol"
        );
        let info = timeout(DEADLINE, read(&mut rx)).await??;
        match &info {
            Message::GameInfo { edition, build }
                if edition == "enhanced" && build == adapter::GAME_VERSION => {}
            _ => anyhow::bail!("unsupported GTA build"),
        }
        println!(
            "{}",
            serde_json::json!({"event":"ipc_connected","hello":hello,"game":info})
        );
        let mut client = Client::connect(server, cert).await?;
        timeout(
            DEADLINE,
            write(
                &mut tx,
                &Message::SessionBegin {
                    session: client.session,
                    entity: client.entity,
                },
            ),
        )
        .await??;
        let (sender, mut incoming) = mpsc::channel(128);
        let _reader = ReaderGuard(tokio::spawn(async move {
            loop {
                let result = match timeout(DEADLINE, read(&mut rx)).await {
                    Ok(r) => r,
                    Err(e) => Err(e.into()),
                };
                let failed = result.is_err();
                if sender.try_send(result).is_err() || failed {
                    break;
                }
            }
        }));
        let mut replica = Replica::default();
        replica.apply(client.snapshots.borrow_and_update().clone(), Instant::now());
        let mut known = BTreeSet::<EntityId>::new();
        let mut last_sequence = 0;
        let mut local_frames = 0_u64;
        let mut frame = tokio::time::interval(Duration::from_millis(50));
        frame.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            tokio::select! {
                value=incoming.recv()=>{
                    let message=value.ok_or_else(||anyhow::anyhow!("adapter reader stopped"))??;
                    match message {
                        Message::LocalPlayerState{sequence,state}=>{
                            if sequence<=last_sequence {continue;}last_sequence=sequence;client.publish(state.clone()).await?;local_frames+=1;
                            if local_frames==1 || local_frames.is_multiple_of(200) {println!("{}",serde_json::json!({"event":"local_player_state_received","frames":local_frames,"state":state}));}
                        }
                        Message::AdapterHeartbeat{game_ready:false}=>anyhow::bail!("game unavailable; resetting session"),
                        Message::AdapterHeartbeat{game_ready:true}=>{client.heartbeat().await?;},
                        Message::AdapterStatus{..}|Message::AdapterError{..}=>println!("{}",serde_json::json!({"event":"adapter_report","report":message})),
                        _=>anyhow::bail!("unexpected adapter message"),
                    }
                }
                changed=client.snapshots.changed()=>{changed?;replica.apply(client.snapshots.borrow_and_update().clone(),Instant::now());}
                _=frame.tick()=>{
                    if let Some(snapshot)=replica.render(Instant::now()) {
                        let remote:Vec<_>=snapshot.entities.into_iter().filter(|e|e.id!=client.entity).collect();
                        let present:BTreeSet<_>=remote.iter().map(|e|e.id).collect();
                        for id in known.difference(&present) {timeout(DEADLINE,write(&mut tx,&Message::RemoteEntityDestroy{id:*id})).await??;}
                        for entity in remote {let message=if known.contains(&entity.id){Message::RemoteEntityUpdate{entity}}else{Message::RemoteEntityCreate{entity}};timeout(DEADLINE,write(&mut tx,&message)).await??;}
                        known=present;
                    }
                    timeout(DEADLINE,write(&mut tx,&Message::AdapterHeartbeat{game_ready:true})).await??;
                }
            }
        }
    }
}
