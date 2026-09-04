//! Dedicated M0 server. All game state is owned here, never by clients.
pub mod presence;
pub mod presence_m2;
pub mod session_m2;
use anyhow::Result;
use gameverse_protocol::{Message, SessionId, Snapshot, VERSION};
use gameverse_runtime::{World, STEP_MS};
use gameverse_transport::{
    quinn, read_message, write_message, HANDSHAKE_TIMEOUT, IDLE_TIMEOUT, INPUT_CAPACITY,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    sync::{mpsc, watch, Semaphore},
    task::JoinSet,
    time::{timeout, MissedTickBehavior},
};

struct Peer {
    input: mpsc::Receiver<Message>,
    snapshots: watch::Sender<Snapshot>,
    connection: quinn::Connection,
}

#[derive(Default)]
struct State {
    world: World,
    peers: BTreeMap<SessionId, Peer>,
    max_input_depth: usize,
    accepted: u64,
    disconnects: u64,
    snapshots: u64,
}

struct SessionGuard {
    state: Arc<Mutex<State>>,
    session: SessionId,
}
impl Drop for SessionGuard {
    fn drop(&mut self) {
        let mut state = self.state.lock().unwrap();
        state.peers.remove(&self.session);
        state.world.disconnect(self.session);
        state.disconnects += 1;
    }
}

async fn session(connecting: quinn::Connecting, state: Arc<Mutex<State>>) -> Result<()> {
    // One deadline covers TLS, opening the stream and the first application frame.
    let (connection, mut send, mut recv, hello) = timeout(HANDSHAKE_TIMEOUT, async {
        let connection = connecting.await?;
        let (send, mut recv) = connection.accept_bi().await?;
        let hello = read_message(&mut recv).await?;
        Ok::<_, anyhow::Error>((connection, send, recv, hello))
    })
    .await??;
    if hello != (Message::Hello { version: VERSION }) {
        write_message(
            &mut send,
            &Message::Reject {
                reason: "unsupported protocol or handshake".into(),
            },
        )
        .await?;
        send.finish().await?;
        return Ok(());
    }
    let (input_tx, input_rx) = mpsc::channel(INPUT_CAPACITY);
    let registration = {
        let mut state = state.lock().unwrap();
        state.world.connect().map(|(id, entity)| {
            let initial = state.world.snapshot(id).unwrap();
            let (snapshots, rx) = watch::channel(initial);
            state.peers.insert(
                id,
                Peer {
                    input: input_rx,
                    snapshots,
                    connection: connection.clone(),
                },
            );
            state.accepted += 1;
            (id, entity, rx)
        })
    };
    let (id, entity, mut snapshots) = match registration {
        Ok(value) => value,
        Err(_) => {
            write_message(
                &mut send,
                &Message::Reject {
                    reason: "server full".into(),
                },
            )
            .await?;
            send.finish().await?;
            return Ok(());
        }
    };
    let _guard = SessionGuard {
        state: state.clone(),
        session: id,
    };
    let result: Result<()> = async {
        write_message(
            &mut send,
            &Message::Welcome {
                version: VERSION,
                session: id,
                entity,
            },
        )
        .await?;
        let initial = snapshots.borrow_and_update().clone();
        write_message(&mut send, &Message::Snapshot { state: initial }).await?;
        let receive = async {
            loop {
                let message = timeout(IDLE_TIMEOUT, read_message(&mut recv)).await??;
                match message {
                    Message::Input { .. } => {
                        let depth = INPUT_CAPACITY - input_tx.capacity() + 1;
                        input_tx
                            .try_send(message)
                            .map_err(|_| anyhow::anyhow!("input queue overflow"))?;
                        let mut state = state.lock().unwrap();
                        state.max_input_depth = state.max_input_depth.max(depth);
                    }
                    Message::Disconnect { .. } => return Ok::<(), anyhow::Error>(()),
                    _ => anyhow::bail!("unexpected client message"),
                }
            }
        };
        let transmit = async {
            loop {
                snapshots.changed().await?;
                let snapshot = snapshots.borrow_and_update().clone();
                timeout(
                    IDLE_TIMEOUT,
                    write_message(&mut send, &Message::Snapshot { state: snapshot }),
                )
                .await??;
                state.lock().unwrap().snapshots += 1;
            }
            #[allow(unreachable_code)]
            Ok::<(), anyhow::Error>(())
        };
        tokio::select! { r = receive => r, r = transmit => r }
    }
    .await;
    connection.close(0_u32.into(), b"session ended");
    result
}

/// Run until shutdown becomes true. Endpoint ownership ensures all connections close.
pub async fn run(
    endpoint: quinn::Endpoint,
    mut shutdown: watch::Receiver<bool>,
) -> Result<serde_json::Value> {
    let state = Arc::new(Mutex::new(State::default()));
    let pending = Arc::new(Semaphore::new(16));
    let mut tasks = JoinSet::new();
    let mut ticker = tokio::time::interval(Duration::from_millis(STEP_MS));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);
    ticker.tick().await;
    loop {
        if *shutdown.borrow() {
            break;
        }
        tokio::select! {
            changed = shutdown.changed() => { if changed.is_err() || *shutdown.borrow() { break; } }
            Some(connecting) = endpoint.accept() => {
                if let Ok(permit) = pending.clone().try_acquire_owned() {
                    let state = state.clone();
                    tasks.spawn(async move {
                        let _permit = permit;
                        if let Err(err) = session(connecting, state).await { eprintln!("session: {err}"); }
                    });
                }
            }
            Some(result) = tasks.join_next(), if !tasks.is_empty() => { result?; }
            _ = ticker.tick() => {
                let mut state = state.lock().unwrap();
                let State { world, peers, .. } = &mut *state;
                for (id, peer) in peers.iter_mut() {
                    // Budget each session independently; command traffic cannot postpone ticks.
                    for _ in 0..INPUT_CAPACITY {
                        match peer.input.try_recv() {
                            Ok(Message::Input { sequence, direction }) => {
                                if world.input(*id, sequence, direction).is_err() {
                                    peer.connection.close(1_u32.into(), b"invalid input");
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }
                }
                world.step();
                if world.tick() % 2 == 0 {
                    for (id, peer) in peers.iter() { peer.snapshots.send_replace(world.snapshot(*id)?); }
                }
                if world.tick() % 200 == 0 {
                    println!("{}", serde_json::json!({"event":"metrics", "tick":world.tick(), "players":world.players(), "max_input_depth":state.max_input_depth, "snapshots":state.snapshots}));
                }
            }
        }
    }
    endpoint.close(0_u32.into(), b"server shutdown");
    tasks.abort_all();
    while tasks.join_next().await.is_some() {}
    endpoint.wait_idle().await;
    let state = state.lock().unwrap();
    Ok(
        serde_json::json!({ "event":"shutdown", "ticks":state.world.tick(), "players":state.world.players(), "accepted_sessions":state.accepted, "disconnects":state.disconnects, "snapshots":state.snapshots, "max_input_depth":state.max_input_depth }),
    )
}
