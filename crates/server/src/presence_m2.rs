//! M2 server: reliable control stream plus bounded Presence v2 datagrams.
use anyhow::Result;
use gameverse_protocol::{
    adapter::SessionConfig,
    control_v2::{Capabilities, ControlMessage, RealtimeMessage, RESOURCE_API_VERSION, VERSION},
    presence_v2::{self as p, Appearance, CombatPresentation, Locomotion, PlayerFrame, Transform},
    SessionId,
};
use gameverse_runtime::{replication::World, STEP_MS};
use gameverse_transport::{
    presence_v2::{read_realtime, send_frame},
    quinn, read_control, write_control, HANDSHAKE_TIMEOUT,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    sync::watch,
    task::JoinSet,
    time::{timeout, MissedTickBehavior},
};

struct Peer {
    connection: quinn::Connection,
}
#[derive(Default)]
struct State {
    world: World,
    peers: BTreeMap<SessionId, Peer>,
    accepted: u64,
    disconnects: u64,
    received_datagrams: u64,
    sent_datagrams: u64,
    dropped_datagrams: u64,
}
struct Guard {
    state: Arc<Mutex<State>>,
    session: SessionId,
}
impl Drop for Guard {
    fn drop(&mut self) {
        if let Ok(mut state) = self.state.lock() {
            state.peers.remove(&self.session);
            let _ = state.world.disconnect(self.session);
            state.disconnects += 1;
        }
    }
}

fn initial_frame(config: &SessionConfig) -> PlayerFrame {
    PlayerFrame {
        sequence: 1,
        client_tick: 0,
        transform: Transform {
            position: config.spawn,
            rotation: [0.0, 0.0, 0.0, 1.0],
            velocity: [0.0; 3],
        },
        appearance: Some(Appearance {
            model_hash: config.model_hash,
        }),
        locomotion: Locomotion::Idle,
        combat: CombatPresentation {
            aiming: false,
            shooting: false,
            reloading: false,
            dead: false,
            weapon_hash: 0,
            aim_target: None,
        },
        vehicle: None,
    }
}

async fn reject(send: &mut quinn::SendStream, code: &str, reason: &str) -> Result<()> {
    write_control(
        send,
        &ControlMessage::Reject {
            code: code.into(),
            reason: reason.into(),
        },
    )
    .await?;
    send.finish().await?;
    Ok(())
}

async fn session(connecting: quinn::Connecting, state: Arc<Mutex<State>>) -> Result<()> {
    let connection = timeout(HANDSHAKE_TIMEOUT, connecting).await??;
    let (mut send, mut recv) = timeout(HANDSHAKE_TIMEOUT, connection.accept_bi()).await??;
    let hello = timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await??;
    let accepted = matches!(&hello, ControlMessage::ClientHello { control_versions, presence_versions, capabilities: Capabilities { datagrams: true, resource_api: RESOURCE_API_VERSION, .. }, .. } if control_versions.contains(&VERSION) && presence_versions.contains(&p::VERSION));
    if !accepted {
        reject(
            &mut send,
            "unsupported_capabilities",
            "M2 control, Presence v2, resource API v1 and QUIC datagrams are required",
        )
        .await?;
        return Ok(());
    }
    let config = SessionConfig::default();
    let registration = {
        let mut state = state.lock().unwrap();
        match state.world.connect() {
            Ok((session, entity)) => match state.world.publish(session, initial_frame(&config)) {
                Ok(_) => {
                    state.peers.insert(
                        session,
                        Peer {
                            connection: connection.clone(),
                        },
                    );
                    state.accepted += 1;
                    Ok((session, entity))
                }
                Err(error) => Err(error),
            },
            Err(error) => Err(error),
        }
    };
    let (session, entity) = match registration {
        Ok(value) => value,
        Err(_) => {
            reject(&mut send, "server_full", "server capacity reached").await?;
            return Ok(());
        }
    };
    let _guard = Guard {
        state: state.clone(),
        session,
    };
    write_control(
        &mut send,
        &ControlMessage::ServerHello {
            control_version: VERSION,
            presence_version: p::VERSION,
            session,
            entity,
            max_players: p::MAX_PLAYERS as u16,
            tick_hz: (1000 / STEP_MS) as u16,
        },
    )
    .await?;
    write_control(&mut send, &ControlMessage::SessionBegin { config }).await?;
    match timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await?? {
        ControlMessage::SpawnReady => write_control(&mut send, &ControlMessage::SpawnAck).await?,
        _ => anyhow::bail!("expected spawn_ready"),
    }
    let mut realtime_window = tokio::time::Instant::now();
    let mut realtime_count = 0_u16;
    loop {
        tokio::select! {
            control = read_control(&mut recv) => match control? {
                ControlMessage::Disconnect { .. } => break,
                _ => anyhow::bail!("unexpected control message after spawn"),
            },
            realtime = read_realtime(&connection) => match realtime? {
                RealtimeMessage::Player { frame } => {
                    let now = tokio::time::Instant::now();
                    if now.duration_since(realtime_window) >= Duration::from_secs(1) {
                        realtime_window = now;
                        realtime_count = 0;
                    }
                    realtime_count += 1;
                    if realtime_count > 40 {
                        state.lock().unwrap().dropped_datagrams += 1;
                        continue;
                    }
                    let mut state = state.lock().unwrap();
                    state.world.publish(session, frame)?;
                    state.received_datagrams += 1;
                }
                _ => anyhow::bail!("client sent a server-only realtime message"),
            },
            _ = connection.closed() => break,
        }
    }
    connection.close(0_u32.into(), b"session ended");
    Ok(())
}

pub async fn run(
    endpoint: quinn::Endpoint,
    mut shutdown: watch::Receiver<bool>,
) -> Result<serde_json::Value> {
    let state = Arc::new(Mutex::new(State::default()));
    let mut tasks = JoinSet::new();
    let mut ticker = tokio::time::interval(Duration::from_millis(STEP_MS));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);
    ticker.tick().await;
    loop {
        if *shutdown.borrow() {
            break;
        }
        tokio::select! {
            changed = shutdown.changed() => if changed.is_err() || *shutdown.borrow() { break; },
            Some(connecting) = endpoint.accept() => {
                let state = state.clone();
                tasks.spawn(async move { if let Err(error) = session(connecting, state).await { eprintln!("m2 session: {error}"); } });
            },
            Some(result) = tasks.join_next(), if !tasks.is_empty() => { result?; },
            _ = ticker.tick() => {
                let mut state = state.lock().unwrap();
                state.world.step();
                let sessions: Vec<_> = state.peers.keys().copied().collect();
                for session in sessions {
                    let frame = state.world.delta(session)?;
                    if frame.deltas.is_empty() { continue; }
                    let connection = state.peers[&session].connection.clone();
                    if send_frame(&connection, &frame).is_ok() { state.sent_datagrams += 1; } else { state.dropped_datagrams += 1; }
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
        serde_json::json!({"event":"m2_shutdown","players":state.peers.len(),"accepted_sessions":state.accepted,"disconnects":state.disconnects,"received_datagrams":state.received_datagrams,"sent_datagrams":state.sent_datagrams,"dropped_datagrams":state.dropped_datagrams}),
    )
}
