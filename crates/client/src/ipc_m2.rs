//! Windows GTA named-pipe bridge backed by the M2 QUIC session.
use crate::{
    ipc,
    m2::{from_legacy, Client, Replica},
};
use anyhow::Result;
use gameverse_protocol::{
    adapter::{self, Message},
    EntityId,
};
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

pub async fn run(pipe: &str, server: SocketAddr, cert: &Path, duration: Duration) -> Result<()> {
    anyhow::ensure!(
        pipe.starts_with(r"\\.\pipe\") && !duration.is_zero(),
        "invalid pipe or duration"
    );
    let finish = Instant::now() + duration;
    let mut listener = ServerOptions::new()
        .first_pipe_instance(true)
        .reject_remote_clients(true)
        .create(pipe)?;
    println!(
        "{}",
        serde_json::json!({"event":"m2_pipe_ready","pipe":pipe})
    );
    loop {
        let left = finish.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Ok(());
        }
        tokio::select! { result=timeout(left,listener.connect())=>{if result.is_err(){return Ok(());}result??;}, _=tokio::signal::ctrl_c()=>return Ok(()) }
        let next = ServerOptions::new()
            .reject_remote_clients(true)
            .create(pipe)?;
        let connected = std::mem::replace(&mut listener, next);
        if let Err(error) = serve(connected, server, cert, finish).await {
            eprintln!(
                "{}",
                serde_json::json!({"event":"m2_adapter_disconnected","error":error.to_string()})
            );
        }
    }
}

async fn serve(
    stream: tokio::net::windows::named_pipe::NamedPipeServer,
    server: SocketAddr,
    cert: &Path,
    finish: Instant,
) -> Result<()> {
    let (mut rx, mut tx) = tokio::io::split(stream);
    let hello = timeout(DEADLINE, ipc::read(&mut rx)).await??;
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
    let info = timeout(DEADLINE, ipc::read(&mut rx)).await??;
    let build = match info {
        Message::GameInfo { edition, build } if edition == "enhanced" => build,
        _ => anyhow::bail!("unsupported GTA edition"),
    };
    let client = Client::connect(server, cert, Some(build)).await?;
    timeout(
        DEADLINE,
        ipc::write(
            &mut tx,
            &Message::SessionBegin {
                session: client.session,
                entity: client.entity,
                config: client.config.clone(),
            },
        ),
    )
    .await??;
    let (sender, mut incoming) = mpsc::channel(128);
    let _reader = ReaderGuard(tokio::spawn(async move {
        loop {
            let result = timeout(DEADLINE, ipc::read(&mut rx))
                .await
                .map_err(anyhow::Error::from)
                .and_then(|v| v);
            let failed = result.is_err();
            if sender.try_send(result).is_err() || failed {
                break;
            }
        }
    }));
    let mut replica = Replica::default();
    let mut known = BTreeSet::<EntityId>::new();
    let mut sequence = 1_u64;
    let mut frame = tokio::time::interval(Duration::from_millis(50));
    loop {
        if Instant::now() >= finish {
            break;
        }
        tokio::select! {
            value=incoming.recv()=>match value.ok_or_else(||anyhow::anyhow!("adapter reader stopped"))?? {
                Message::LocalPlayerState{sequence:local_sequence,state}=>{if local_sequence>sequence{sequence=local_sequence;}else{sequence+=1;}client.publish(from_legacy(sequence,state))?;}
                Message::AdapterHeartbeat{game_ready:false}=>anyhow::bail!("game unavailable"),
                Message::AdapterHeartbeat{game_ready:true}|Message::AdapterStatus{..}|Message::AdapterError{..}=>{},
                _=>anyhow::bail!("unexpected adapter message"),
            },
            network=client.read_frame()=>{replica.apply(network?)?;},
            _=frame.tick()=>{
                let entities=replica.legacy_entities(); let present:BTreeSet<_>=entities.iter().map(|value|value.id).filter(|id|*id!=client.entity).collect();
                for id in known.difference(&present){timeout(DEADLINE,ipc::write(&mut tx,&Message::RemoteEntityDestroy{id:*id})).await??;}
                for entity in entities.into_iter().filter(|value|value.id!=client.entity){let message=if known.contains(&entity.id){Message::RemoteEntityUpdate{entity}}else{Message::RemoteEntityCreate{entity}};timeout(DEADLINE,ipc::write(&mut tx,&message)).await??;}
                known=present; timeout(DEADLINE,ipc::write(&mut tx,&Message::AdapterHeartbeat{game_ready:true})).await??;
            },
            _=tokio::signal::ctrl_c()=>break,
        }
    }
    client.close().await
}
