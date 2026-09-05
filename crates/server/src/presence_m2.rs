//! M2 server: reliable control stream plus bounded Presence v2 datagrams.
use anyhow::Result;
use gameverse_protocol::{
    adapter::SessionConfig,
    control_v2::{
        AuthMode, Capabilities, CharacterSummary, ControlMessage, RealtimeMessage,
        RESOURCE_API_VERSION, VERSION,
    },
    presence_v2::{self as p, Appearance, CombatPresentation, Locomotion, PlayerFrame, Transform},
    SessionId,
};
use gameverse_rp::persistence::{
    AccountRepository, CharacterRepository, EconomyRepository, InventoryRepository, JobRepository,
    PostgresStore, SessionRepository,
};
use gameverse_runtime::{replication::World, STEP_MS};
use gameverse_transport::{
    presence_v2::{read_realtime_with_len, send_frame},
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

use crate::session_m2::{Phase, SessionMachine};

fn supported_game_build(edition: &str, build: &str) -> bool {
    edition == "enhanced" && build == gameverse_protocol::adapter::GAME_VERSION
}

fn require_game_build_attestation(attested: bool) -> std::result::Result<(), &'static str> {
    attested
        .then_some(())
        .ok_or("the GTA adapter must attest the running build before spawn")
}

struct Peer {
    connection: quinn::Connection,
    outbound_baseline: u64,
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
    received_bytes: u64,
    sent_bytes: u64,
    max_tick_micros: u64,
}

#[derive(Clone, Debug, Default)]
pub struct MetricsSnapshot {
    pub players: usize,
    pub accepted_sessions: u64,
    pub disconnects: u64,
    pub received_datagrams: u64,
    pub sent_datagrams: u64,
    pub dropped_datagrams: u64,
    pub received_bytes: u64,
    pub sent_bytes: u64,
    pub max_tick_micros: u64,
}

#[derive(Clone, Default)]
pub struct MetricsHandle(Arc<Mutex<MetricsSnapshot>>);
impl MetricsHandle {
    pub fn snapshot(&self) -> MetricsSnapshot {
        self.0.lock().expect("metrics lock poisoned").clone()
    }
    fn update(&self, state: &State) {
        *self.0.lock().expect("metrics lock poisoned") = MetricsSnapshot {
            players: state.peers.len(),
            accepted_sessions: state.accepted,
            disconnects: state.disconnects,
            received_datagrams: state.received_datagrams,
            sent_datagrams: state.sent_datagrams,
            dropped_datagrams: state.dropped_datagrams,
            received_bytes: state.received_bytes,
            sent_bytes: state.sent_bytes,
            max_tick_micros: state.max_tick_micros,
        };
    }
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
                            outbound_baseline: 0,
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
            server_id: "gameverse-alpha".into(),
            max_players: p::MAX_PLAYERS as u16,
            tick_hz: (1000 / STEP_MS) as u16,
        },
    )
    .await?;
    write_control(
        &mut send,
        &ControlMessage::SessionBegin {
            session,
            entity,
            config,
        },
    )
    .await?;
    match timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await?? {
        ControlMessage::SpawnReady { request_id } => {
            write_control(&mut send, &ControlMessage::SpawnAck { request_id }).await?
        }
        _ => anyhow::bail!("expected spawn_ready"),
    }
    let mut realtime_window = tokio::time::Instant::now();
    let mut realtime_count = 0_u16;
    loop {
        tokio::select! {
            control = read_control(&mut recv) => match control? {
                ControlMessage::Disconnect { .. } => {
                    write_control(&mut send, &ControlMessage::DisconnectAck).await?;
                    send.finish().await?;
                    break;
                }
                _ => anyhow::bail!("unexpected control message after spawn"),
            },
            realtime = read_realtime_with_len(&connection) => {
                let (message, bytes) = realtime?;
                match message {
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
                    state.received_bytes += bytes as u64;
                }
                _ => anyhow::bail!("client sent a server-only realtime message"),
            }},
            _ = connection.closed() => break,
        }
    }
    connection.close(0_u32.into(), b"session ended");
    Ok(())
}

async fn alpha_session(
    connecting: quinn::Connecting,
    state: Arc<Mutex<State>>,
    store: PostgresStore,
) -> Result<()> {
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
    // ClientHello is preflight metadata. Only a distinct message sent after
    // SessionBegin can attest the adapter that actually started with GTA.
    let mut build_attested = false;
    let mut machine = SessionMachine::new(0);
    machine.negotiated(1)?;
    write_control(
        &mut send,
        &ControlMessage::ServerHello {
            control_version: VERSION,
            presence_version: p::VERSION,
            server_id: "gameverse-alpha".into(),
            max_players: p::MAX_PLAYERS as u16,
            tick_hz: (1000 / STEP_MS) as u16,
        },
    )
    .await?;

    let started = tokio::time::Instant::now();
    let mut registration = None;
    let mut registration_guard = None;
    loop {
        let control = tokio::select! {
            control = timeout(machine.timeout(), read_control(&mut recv)) => control??,
            datagram = connection.read_datagram() => {
                datagram?;
                state.lock().unwrap().dropped_datagrams += 1;
                continue;
            }
        };
        let now_ms = started.elapsed().as_millis() as u64 + 1;
        match control {
            ControlMessage::AuthRequest {
                request_id,
                mode,
                login,
                password,
                invite,
            } => {
                let account_id = match mode {
                    AuthMode::Register => {
                        store
                            .register_account(
                                invite.as_deref().unwrap_or_default(),
                                &login,
                                &password,
                            )
                            .await
                    }
                    AuthMode::Login => {
                        store
                            .authenticate(&login, &password)
                            .await
                            .and_then(|value| {
                                value.ok_or_else(|| anyhow::anyhow!("invalid credentials"))
                            })
                    }
                };
                let account_id = match account_id {
                    Ok(value) => value,
                    Err(_) => {
                        reject(
                            &mut send,
                            "authentication_failed",
                            "credentials or invite were rejected",
                        )
                        .await?;
                        return Ok(());
                    }
                };
                let grant = store
                    .issue_session(account_id, std::time::SystemTime::now())
                    .await?;
                machine.authenticated(account_id, now_ms)?;
                write_control(
                    &mut send,
                    &ControlMessage::AuthResult {
                        request_id,
                        account_id,
                        access_token: grant.tokens.access_token,
                        refresh_token: grant.tokens.refresh_token,
                        expires_at_ms: grant.tokens.access_expires_at_ms,
                    },
                )
                .await?;
            }
            ControlMessage::AuthResume {
                request_id,
                refresh_token,
            } => {
                let grant = match store
                    .resume_session(&refresh_token, std::time::SystemTime::now())
                    .await
                {
                    Ok(value) => value,
                    Err(_) => {
                        reject(
                            &mut send,
                            "authentication_failed",
                            "refresh session was rejected",
                        )
                        .await?;
                        return Ok(());
                    }
                };
                machine.authenticated(grant.account_id, now_ms)?;
                write_control(
                    &mut send,
                    &ControlMessage::AuthResult {
                        request_id,
                        account_id: grant.account_id,
                        access_token: grant.tokens.access_token,
                        refresh_token: grant.tokens.refresh_token,
                        expires_at_ms: grant.tokens.access_expires_at_ms,
                    },
                )
                .await?;
            }
            ControlMessage::CharacterList {
                request_id,
                characters,
            } if characters.is_empty() => {
                let account_id = machine
                    .account_id()
                    .ok_or_else(|| anyhow::anyhow!("not authenticated"))?;
                let characters = store
                    .characters(account_id)
                    .await?
                    .iter()
                    .map(character_summary)
                    .collect();
                write_control(
                    &mut send,
                    &ControlMessage::CharacterList {
                        request_id,
                        characters,
                    },
                )
                .await?;
            }
            ControlMessage::CreateCharacter {
                request_id,
                first_name,
                last_name,
                model_hash,
            } => {
                let account_id = machine
                    .account_id()
                    .ok_or_else(|| anyhow::anyhow!("not authenticated"))?;
                store
                    .create_character(account_id, &first_name, &last_name, model_hash)
                    .await?;
                let characters = store
                    .characters(account_id)
                    .await?
                    .iter()
                    .map(character_summary)
                    .collect();
                write_control(
                    &mut send,
                    &ControlMessage::CharacterList {
                        request_id,
                        characters,
                    },
                )
                .await?;
            }
            ControlMessage::SelectCharacter { character_id, .. } => {
                let account_id = machine
                    .account_id()
                    .ok_or_else(|| anyhow::anyhow!("not authenticated"))?;
                let character = store
                    .characters(account_id)
                    .await?
                    .into_iter()
                    .find(|value| value.id == character_id)
                    .ok_or_else(|| anyhow::anyhow!("character not found"))?;
                machine.select_character(character.id, character.account_id, now_ms)?;
                let config = SessionConfig {
                    character_id: Some(character.id),
                    spawn: character.position,
                    heading: character.heading,
                    model_hash: character.model_hash,
                    instance_id: character.instance_id,
                    appearance: character.appearance,
                };
                let created = {
                    let mut state = state.lock().unwrap();
                    let (session, entity) = state.world.connect()?;
                    state.world.publish(session, initial_frame(&config))?;
                    state.peers.insert(
                        session,
                        Peer {
                            connection: connection.clone(),
                            outbound_baseline: 0,
                        },
                    );
                    state.accepted += 1;
                    (session, entity)
                };
                machine.begin_spawn(now_ms)?;
                write_control(
                    &mut send,
                    &ControlMessage::SessionBegin {
                        session: created.0,
                        entity: created.1,
                        config,
                    },
                )
                .await?;
                registration = Some(created);
                registration_guard = Some(Guard {
                    state: state.clone(),
                    session: created.0,
                });
            }
            ControlMessage::GameBuildAttestation { edition, build }
                if machine.phase() == Phase::SpawnPending =>
            {
                if !supported_game_build(&edition, &build) {
                    reject(
                        &mut send,
                        "unsupported_gta_build",
                        "the running GTA edition or build is not supported",
                    )
                    .await?;
                    return Ok(());
                }
                build_attested = true;
            }
            ControlMessage::SpawnReady { request_id } => {
                if let Err(reason) = require_game_build_attestation(build_attested) {
                    reject(&mut send, "gta_build_not_attested", reason).await?;
                    return Ok(());
                }
                machine.spawn_ready(now_ms)?;
                write_control(&mut send, &ControlMessage::SpawnAck { request_id }).await?;
                break;
            }
            ControlMessage::Logout {
                request_id,
                refresh_token,
            } => {
                store.revoke_session(&refresh_token).await?;
                write_control(&mut send, &ControlMessage::LogoutResult { request_id }).await?;
                send.finish().await?;
                return Ok(());
            }
            _ => {
                reject(
                    &mut send,
                    "invalid_session_state",
                    "command is not valid in the current session state",
                )
                .await?;
                return Ok(());
            }
        }
    }

    let (session, _) = registration.ok_or_else(|| anyhow::anyhow!("spawn without entity"))?;
    let _guard = registration_guard.ok_or_else(|| anyhow::anyhow!("spawn without guard"))?;
    let character_id = machine
        .character_id()
        .ok_or_else(|| anyhow::anyhow!("spawn without character"))?;
    let saved_heading = {
        let account_id = machine
            .account_id()
            .ok_or_else(|| anyhow::anyhow!("spawn without account"))?;
        store
            .characters(account_id)
            .await?
            .into_iter()
            .find(|character| character.id == character_id)
            .map(|character| character.heading)
            .ok_or_else(|| anyhow::anyhow!("selected character disappeared"))?
    };
    let mut last_position = None;
    let mut realtime_window = tokio::time::Instant::now();
    let mut realtime_count = 0_u16;
    loop {
        tokio::select! {
            control = read_control(&mut recv) => match control? {
                ControlMessage::Disconnect { .. } => {
                    write_control(&mut send, &ControlMessage::DisconnectAck).await?;
                    send.finish().await?;
                    break;
                }
                ControlMessage::Logout { request_id, refresh_token } => {
                    store.revoke_session(&refresh_token).await?;
                    write_control(&mut send, &ControlMessage::LogoutResult { request_id }).await?;
                    send.finish().await?;
                    break;
                }
                ControlMessage::ChatCommand { request_id, message } => {
                    write_control(&mut send, &ControlMessage::ChatMessage {
                        source: None,
                        channel: format!("local:{request_id}"),
                        message,
                    }).await?;
                }
                ControlMessage::InventoryCommand { request_id, action, .. } if action == "snapshot" => {
                    let items = store.inventory(character_id).await?.into_iter().map(|(item_id, quantity)| gameverse_protocol::control_v2::InventoryItem { item_id, quantity }).collect();
                    write_control(&mut send, &ControlMessage::InventorySnapshot { request_id, revision: 0, items }).await?;
                }
                ControlMessage::ShopCatalog { request_id, shop, items } if items.is_empty() => {
                    let items = store.shop_catalog(&shop).await?.into_iter().map(|item| gameverse_protocol::control_v2::ShopItem {
                        item_id: item.item_id,
                        name: item.name,
                        price: item.price,
                    }).collect();
                    write_control(&mut send, &ControlMessage::ShopCatalog { request_id, shop, items }).await?;
                }
                ControlMessage::JobCommand { request_id, action, route, idempotency_key: _ } if action == "start" => {
                    store.start_delivery(character_id, &route).await?;
                    write_control(&mut send, &ControlMessage::JobState { request_id, active_route: Some(route), revision: 1 }).await?;
                }
                ControlMessage::JobCommand { request_id, action, route, idempotency_key } if action == "finish" => {
                    let receipt = store.finish_delivery(character_id, &route, idempotency_key.as_deref().unwrap_or_default()).await?;
                    write_control(&mut send, &ControlMessage::EconomyResult { request_id, transaction_id: receipt.transaction_id, cash: receipt.cash, bank: receipt.bank }).await?;
                }
                ControlMessage::ShopCommand { request_id, shop, item_id, quantity, idempotency_key } => {
                    let receipt = store.buy(character_id, &shop, item_id, quantity, &idempotency_key).await?;
                    write_control(&mut send, &ControlMessage::EconomyResult { request_id, transaction_id: receipt.transaction_id, cash: receipt.cash, bank: receipt.bank }).await?;
                }
                _ => anyhow::bail!("unexpected control message after spawn"),
            },
            realtime = read_realtime_with_len(&connection) => {
                let (message, bytes) = realtime?;
                match message {
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
                        last_position = Some(frame.transform.position);
                        state.world.publish(session, frame)?;
                        state.received_datagrams += 1;
                        state.received_bytes += bytes as u64;
                    }
                    _ => anyhow::bail!("client sent a server-only realtime message"),
                }
            },
            _ = connection.closed() => break,
        }
    }
    if let Some(position) = last_position {
        store
            .save_position(character_id, position, saved_heading)
            .await?;
    }
    connection.close(0_u32.into(), b"session ended");
    Ok(())
}

fn character_summary(character: &gameverse_rp::Character) -> CharacterSummary {
    CharacterSummary {
        id: character.id,
        first_name: character.first_name.clone(),
        last_name: character.last_name.clone(),
        model_hash: character.model_hash,
    }
}

pub async fn run(
    endpoint: quinn::Endpoint,
    shutdown: watch::Receiver<bool>,
) -> Result<serde_json::Value> {
    run_inner(endpoint, None, shutdown, None).await
}

pub async fn run_with_metrics(
    endpoint: quinn::Endpoint,
    shutdown: watch::Receiver<bool>,
    metrics: MetricsHandle,
) -> Result<serde_json::Value> {
    run_inner(endpoint, None, shutdown, Some(metrics)).await
}

pub async fn run_alpha(
    endpoint: quinn::Endpoint,
    store: PostgresStore,
    shutdown: watch::Receiver<bool>,
) -> Result<serde_json::Value> {
    run_inner(endpoint, Some(store), shutdown, None).await
}

pub async fn run_alpha_with_metrics(
    endpoint: quinn::Endpoint,
    store: PostgresStore,
    shutdown: watch::Receiver<bool>,
    metrics: MetricsHandle,
) -> Result<serde_json::Value> {
    run_inner(endpoint, Some(store), shutdown, Some(metrics)).await
}

async fn run_inner(
    endpoint: quinn::Endpoint,
    store: Option<PostgresStore>,
    mut shutdown: watch::Receiver<bool>,
    metrics: Option<MetricsHandle>,
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
                let store = store.clone();
                tasks.spawn(async move {
                    let result = match store {
                        Some(store) => alpha_session(connecting, state, store).await,
                        None => session(connecting, state).await,
                    };
                    if let Err(error) = result { eprintln!("m2 session: {error}"); }
                });
            },
            Some(result) = tasks.join_next(), if !tasks.is_empty() => { result?; },
            _ = ticker.tick() => {
                let tick_started = std::time::Instant::now();
                let mut state = state.lock().unwrap();
                state.world.step();
                let sessions: Vec<_> = state.peers.keys().copied().collect();
                for session in sessions {
                    let frame = state.world.delta(session)?;
                    if frame.deltas.is_empty() { continue; }
                    let connection = state.peers[&session].connection.clone();
                    let maximum = connection.max_datagram_size().unwrap_or(1_200).min(gameverse_transport::presence_v2::MAX_DATAGRAM);
                    let chunks = split_frame(frame, maximum)?;
                    for mut chunk in chunks {
                        let peer = state.peers.get_mut(&session).expect("session collected from peers");
                        peer.outbound_baseline = peer.outbound_baseline.saturating_add(1);
                        chunk.baseline = peer.outbound_baseline;
                        match send_frame(&connection, &chunk) {
                            Ok(bytes) => { state.sent_datagrams += 1; state.sent_bytes += bytes as u64; },
                            Err(_) => state.dropped_datagrams += 1,
                        }
                    }
                }
                state.max_tick_micros = state.max_tick_micros.max(tick_started.elapsed().as_micros() as u64);
                if let Some(metrics) = &metrics { metrics.update(&state); }
            }
        }
    }
    endpoint.close(0_u32.into(), b"server shutdown");
    tasks.abort_all();
    while tasks.join_next().await.is_some() {}
    endpoint.wait_idle().await;
    let state = state.lock().unwrap();
    if let Some(metrics) = &metrics {
        metrics.update(&state);
    }
    Ok(
        serde_json::json!({"event":"m2_shutdown","players":state.peers.len(),"accepted_sessions":state.accepted,"disconnects":state.disconnects,"received_datagrams":state.received_datagrams,"sent_datagrams":state.sent_datagrams,"dropped_datagrams":state.dropped_datagrams,"received_bytes":state.received_bytes,"sent_bytes":state.sent_bytes,"max_tick_micros":state.max_tick_micros}),
    )
}

fn split_frame(frame: p::ServerFrame, maximum: usize) -> Result<Vec<p::ServerFrame>> {
    let mut chunks = Vec::new();
    let mut current = p::ServerFrame {
        server_tick: frame.server_tick,
        baseline: 1,
        deltas: Vec::new(),
    };
    for delta in frame.deltas {
        current.deltas.push(delta);
        let encoded = gameverse_protocol::control_v2::encode_realtime(&RealtimeMessage::Server {
            frame: current.clone(),
        })?;
        if encoded.len() > maximum {
            let last = current.deltas.pop().expect("delta was just pushed");
            anyhow::ensure!(
                !current.deltas.is_empty(),
                "single entity delta exceeds negotiated QUIC datagram size"
            );
            chunks.push(current);
            current = p::ServerFrame {
                server_tick: frame.server_tick,
                baseline: 1,
                deltas: vec![last],
            };
            let single =
                gameverse_protocol::control_v2::encode_realtime(&RealtimeMessage::Server {
                    frame: current.clone(),
                })?;
            anyhow::ensure!(
                single.len() <= maximum,
                "single entity delta exceeds negotiated QUIC datagram size"
            );
        }
    }
    if !current.deltas.is_empty() {
        chunks.push(current);
    }
    Ok(chunks)
}

#[cfg(test)]
mod frame_chunk_tests {
    use super::*;

    #[test]
    fn spawn_requires_the_running_adapter_to_attest_a_supported_build() {
        assert!(require_game_build_attestation(false).is_err());
        assert!(require_game_build_attestation(true).is_ok());
        assert!(supported_game_build(
            "enhanced",
            gameverse_protocol::adapter::GAME_VERSION
        ));
        assert!(!supported_game_build(
            "legacy",
            gameverse_protocol::adapter::GAME_VERSION
        ));
        assert!(!supported_game_build("enhanced", "unknown"));
    }

    #[test]
    fn chunks_large_frames_below_the_quic_limit() {
        let delta = p::EntityDelta {
            id: gameverse_protocol::EntityId {
                slot: 0,
                generation: 1,
            },
            kind: p::DeltaKind::Upsert,
            transform: Some(p::Transform {
                position: [0.0; 3],
                rotation: [0.0, 0.0, 0.0, 1.0],
                velocity: [0.0; 3],
            }),
            appearance: Some(p::Appearance { model_hash: 1 }),
            locomotion: Some(p::Locomotion::Idle),
            combat: Some(p::CombatPresentation {
                aiming: false,
                shooting: false,
                reloading: false,
                dead: false,
                weapon_hash: 0,
                aim_target: None,
            }),
            vehicle: None,
            cleared: Vec::new(),
        };
        let mut deltas = Vec::new();
        for slot in 0..31 {
            let mut value = delta.clone();
            value.id.slot = slot;
            deltas.push(value);
        }
        let chunks = split_frame(
            p::ServerFrame {
                server_tick: 1,
                baseline: 1,
                deltas,
            },
            1_200,
        )
        .unwrap();
        assert!(chunks.len() > 1);
        assert_eq!(
            chunks.iter().map(|chunk| chunk.deltas.len()).sum::<usize>(),
            31
        );
        assert!(chunks
            .iter()
            .all(|chunk| gameverse_protocol::control_v2::encode_realtime(
                &RealtimeMessage::Server {
                    frame: chunk.clone()
                }
            )
            .unwrap()
            .len()
                <= 1_200));
    }
}
