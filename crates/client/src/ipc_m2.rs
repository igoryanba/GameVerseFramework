//! Windows bridge joining the untrusted UI, GTA adapter and M2 QUIC.
use crate::{
    ipc,
    m2::{from_legacy, AlphaAuthentication, Client, InteractiveClient, NewCharacter, Replica},
    ui::{self, UiRequest, UiResponse},
};
use anyhow::Result;
#[cfg(any(windows, test))]
use gameverse_protocol::bootstrap;
use gameverse_protocol::{
    adapter::{self, Message},
    control_v2::ControlMessage,
    EntityId,
};
use serde::Deserialize;
use serde_json::json;
use std::{
    collections::BTreeSet,
    net::SocketAddr,
    path::Path,
    time::{Duration, Instant},
};
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadHalf, WriteHalf},
    sync::mpsc,
    time::timeout,
};

#[cfg(windows)]
use tokio::net::windows::named_pipe::{NamedPipeServer, ServerOptions};

const DEADLINE: Duration = Duration::from_secs(8);
const INTERACTIVE_INPUT_DEADLINE: Duration = Duration::from_secs(10 * 60);

struct ReaderGuard(tokio::task::JoinHandle<()>);
impl Drop for ReaderGuard {
    fn drop(&mut self) {
        self.0.abort();
    }
}

#[cfg(windows)]
pub async fn run(
    adapter_pipe: &str,
    ui_pipe: &str,
    bootstrap_pipe: &str,
    server: SocketAddr,
    cert: &Path,
    duration: Duration,
) -> Result<()> {
    run_with_bootstrap_mode(
        adapter_pipe,
        ui_pipe,
        bootstrap_pipe,
        server,
        cert,
        duration,
        true,
    )
    .await
}

/// Research mode: records the verified frontend and waits for the user to enter
/// the local Story world. It never requests an automatic world transition.
#[cfg(windows)]
pub async fn run_telemetry(
    adapter_pipe: &str,
    ui_pipe: &str,
    bootstrap_pipe: &str,
    server: SocketAddr,
    cert: &Path,
    duration: Duration,
) -> Result<()> {
    run_with_bootstrap_mode(
        adapter_pipe,
        ui_pipe,
        bootstrap_pipe,
        server,
        cert,
        duration,
        false,
    )
    .await
}

#[cfg(windows)]
async fn run_with_bootstrap_mode(
    adapter_pipe: &str,
    ui_pipe: &str,
    bootstrap_pipe: &str,
    server: SocketAddr,
    cert: &Path,
    duration: Duration,
    automatic_world: bool,
) -> Result<()> {
    anyhow::ensure!(
        valid_pipe(adapter_pipe)
            && valid_pipe(ui_pipe)
            && valid_pipe(bootstrap_pipe)
            && adapter_pipe != ui_pipe
            && adapter_pipe != bootstrap_pipe
            && ui_pipe != bootstrap_pipe
            && !duration.is_zero(),
        "invalid pipe or duration"
    );
    let finish = Instant::now() + duration;
    let mut adapter_listener = listener(adapter_pipe, true)?;
    let mut ui_listener = listener(ui_pipe, true)?;
    let mut bootstrap_listener = listener(bootstrap_pipe, true)?;
    println!(
        "{}",
        json!({"event":"m2_pipe_ready","adapter_pipe":adapter_pipe,"ui_pipe":ui_pipe,"bootstrap_pipe":bootstrap_pipe})
    );
    loop {
        let left = finish.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Ok(());
        }
        timeout(left, ui_listener.connect()).await??;
        let mut ui_stream = std::mem::replace(&mut ui_listener, listener(ui_pipe, false)?);
        {
            let (mut ui_rx, mut ui_tx) = tokio::io::split(&mut ui_stream);
            ui_handshake(&mut ui_rx, &mut ui_tx, "auth_required").await?;
        }
        // Authentication and character selection deliberately happen before GTA
        // starts. The launcher starts the game only after it receives `reserved`.
        let interactive = InteractiveClient::connect(server, cert, None).await?;
        let (pending, reservation_request_id) =
            preflight_session(&mut ui_stream, interactive).await?;
        let left = finish.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Ok(());
        }
        timeout(left, bootstrap_listener.connect()).await??;
        let mut bootstrap_stream =
            std::mem::replace(&mut bootstrap_listener, listener(bootstrap_pipe, false)?);
        let result = async {
            bootstrap_gate_after_handshake(&mut bootstrap_stream, &mut ui_stream, automatic_world)
                .await?;
            timeout(
                finish.saturating_duration_since(Instant::now()),
                adapter_listener.connect(),
            )
            .await??;
            let adapter = std::mem::replace(&mut adapter_listener, listener(adapter_pipe, false)?);
            activate_reserved_session(adapter, ui_stream, pending, &reservation_request_id, finish)
                .await
        }
        .await;
        if let Err(error) = result {
            eprintln!(
                "{}",
                json!({"event":"m2_session_disconnected","error":error.to_string()})
            );
        }
    }
}

async fn preflight_session<U>(
    ui_stream: &mut U,
    mut interactive: InteractiveClient,
) -> Result<(crate::m2::PendingClient, String)>
where
    U: AsyncRead + AsyncWrite + Unpin,
{
    let mut authenticated = false;
    loop {
        let request: UiRequest =
            timeout(INTERACTIVE_INPUT_DEADLINE, ui::read(&mut *ui_stream)).await??;
        if !request.valid() {
            ui::write(
                &mut *ui_stream,
                &UiResponse::error(
                    request.request_id,
                    "invalid_request",
                    "Некорректная команда интерфейса",
                ),
            )
            .await?;
            continue;
        }
        match handle_bootstrap(&mut interactive, authenticated, &request).await {
            Ok(Bootstrap::Response(response, now_authenticated)) => {
                authenticated = now_authenticated;
                ui::write(&mut *ui_stream, &response).await?;
            }
            Ok(Bootstrap::Select(character_id)) => {
                let pending = interactive
                    .select_character(&request.request_id, character_id)
                    .await?;
                ui::write(
                    &mut *ui_stream,
                    &UiResponse::success(
                        &request.request_id,
                        json!({
                            "stage":"reserved",
                            "character_id":pending.config.character_id
                        }),
                    ),
                )
                .await?;
                return Ok((pending, request.request_id));
            }
            Err(error) => {
                ui::write(
                    &mut *ui_stream,
                    &UiResponse::error(&request.request_id, "request_failed", public_error(&error)),
                )
                .await?;
            }
        }
    }
}

async fn activate_reserved_session<A, U>(
    adapter_stream: A,
    ui_stream: U,
    mut pending: crate::m2::PendingClient,
    reservation_request_id: &str,
    finish: Instant,
) -> Result<()>
where
    A: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    U: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let (mut adapter_rx, mut adapter_tx) = tokio::io::split(adapter_stream);
    let (ui_rx, mut ui_tx) = tokio::io::split(ui_stream);
    let adapter_hello = timeout(DEADLINE, ipc::read(&mut adapter_rx)).await??;
    anyhow::ensure!(
        matches!(
            adapter_hello,
            Message::AdapterHello {
                version: adapter::VERSION,
                ..
            }
        ),
        "unsupported adapter protocol"
    );
    match timeout(DEADLINE, ipc::read(&mut adapter_rx)).await?? {
        Message::GameInfo { edition, build }
            if edition == "enhanced" && build == adapter::GAME_VERSION =>
        {
            pending.attest_game_build(&edition, &build).await?;
        }
        _ => anyhow::bail!("unsupported GTA edition or build"),
    }
    ipc::write(
        &mut adapter_tx,
        &Message::SessionBegin {
            session: pending.session,
            entity: pending.entity,
            config: pending.config.clone(),
        },
    )
    .await?;
    loop {
        match timeout(Duration::from_secs(30), ipc::read(&mut adapter_rx)).await?? {
            Message::AdapterStatus { event, .. } if event == "session_ready" => break,
            Message::AdapterError { code, message }
            | Message::BootstrapFailure { code, message } => {
                anyhow::bail!("adapter {code}: {message}")
            }
            Message::AdapterHeartbeat { game_ready: true } | Message::AdapterStatus { .. } => {}
            _ => anyhow::bail!("adapter did not confirm session bootstrap"),
        }
    }
    let client = pending.spawn_ready(reservation_request_id).await?;
    ipc::write(
        &mut adapter_tx,
        &Message::SessionActive {
            session: client.session,
        },
    )
    .await?;
    ui::write(
        &mut ui_tx,
        &UiResponse::success(
            reservation_request_id,
            json!({
                "stage":"active",
                "session":client.session,
                "entity":client.entity,
                "character_id":client.config.character_id
            }),
        ),
    )
    .await?;
    serve_active(client, adapter_rx, adapter_tx, ui_rx, ui_tx, finish).await
}

/// Developer-only fallback for diagnosing the managed adapter after manually
/// entering Story Mode. Production launchers must use `run` and native bootstrap.
#[cfg(windows)]
pub async fn run_manual(
    adapter_pipe: &str,
    ui_pipe: &str,
    server: SocketAddr,
    cert: &Path,
    duration: Duration,
) -> Result<()> {
    anyhow::ensure!(
        valid_pipe(adapter_pipe)
            && valid_pipe(ui_pipe)
            && adapter_pipe != ui_pipe
            && !duration.is_zero(),
        "invalid pipe or duration"
    );
    let finish = Instant::now() + duration;
    let mut adapter_listener = listener(adapter_pipe, true)?;
    let mut ui_listener = listener(ui_pipe, true)?;
    println!(
        "{}",
        json!({"event":"m2_pipe_ready","adapter_pipe":adapter_pipe,"ui_pipe":ui_pipe,"mode":"developer_manual_story"})
    );
    loop {
        let left = finish.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Ok(());
        }
        let (adapter_result, ui_result) = tokio::join!(
            timeout(left, adapter_listener.connect()),
            timeout(left, ui_listener.connect())
        );
        if adapter_result.is_err() || ui_result.is_err() {
            return Ok(());
        }
        adapter_result??;
        ui_result??;
        let adapter = std::mem::replace(&mut adapter_listener, listener(adapter_pipe, false)?);
        let ui_stream = std::mem::replace(&mut ui_listener, listener(ui_pipe, false)?);
        if let Err(error) = serve_streams(adapter, ui_stream, server, cert, finish).await {
            eprintln!(
                "{}",
                json!({"event":"m2_session_disconnected","error":error.to_string(),"mode":"developer_manual_story"})
            );
        }
    }
}

#[cfg(windows)]
fn valid_pipe(pipe: &str) -> bool {
    pipe.starts_with(r"\\.\pipe\") && pipe.len() <= 240
}

#[cfg(windows)]
fn listener(pipe: &str, first: bool) -> Result<NamedPipeServer> {
    let mut options = ServerOptions::new();
    options.reject_remote_clients(true);
    if first {
        options.first_pipe_instance(true);
    }
    Ok(options.create(pipe)?)
}

/// Runs one interactive bridge session over arbitrary bounded local streams.
/// Windows production uses named pipes; integration tests use in-memory streams.
pub async fn serve_streams<A, U>(
    adapter_stream: A,
    ui_stream: U,
    server: SocketAddr,
    cert: &Path,
    finish: Instant,
) -> Result<()>
where
    A: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    U: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let mut ui_stream = ui_stream;
    {
        let (mut ui_rx, mut ui_tx) = tokio::io::split(&mut ui_stream);
        ui_handshake(&mut ui_rx, &mut ui_tx, "auth_required").await?;
    }
    let interactive = InteractiveClient::connect(server, cert, None).await?;
    let (pending, request_id) = preflight_session(&mut ui_stream, interactive).await?;
    activate_reserved_session(adapter_stream, ui_stream, pending, &request_id, finish).await
}

async fn ui_handshake(
    ui_rx: &mut (impl AsyncRead + Unpin),
    ui_tx: &mut (impl AsyncWrite + Unpin),
    stage: &str,
) -> Result<()> {
    let hello: UiRequest = timeout(DEADLINE, ui::read(ui_rx)).await??;
    anyhow::ensure!(
        hello.valid() && hello.command == "ui.hello",
        "invalid UI handshake"
    );
    ui::write(ui_tx, &UiResponse::success(&hello.request_id, json!({
        "bridge_build":env!("CARGO_PKG_VERSION"), "stage":stage,
        "capabilities":["auth","characters","chat","inventory","shop","job","native_bootstrap"]
    }))).await
}

#[cfg(test)]
async fn bootstrap_gate<B, U>(
    bootstrap_stream: &mut B,
    ui_stream: &mut U,
    automatic_world: bool,
) -> Result<()>
where
    B: AsyncRead + AsyncWrite + Unpin,
    U: AsyncRead + AsyncWrite + Unpin,
{
    let (mut ui_rx, mut ui_tx) = tokio::io::split(ui_stream);
    ui_handshake(&mut ui_rx, &mut ui_tx, "waiting_for_game").await?;
    bootstrap_gate_inner(bootstrap_stream, &mut ui_tx, automatic_world).await
}

#[cfg(windows)]
async fn bootstrap_gate_after_handshake<B, U>(
    bootstrap_stream: &mut B,
    ui_stream: &mut U,
    automatic_world: bool,
) -> Result<()>
where
    B: AsyncRead + AsyncWrite + Unpin,
    U: AsyncRead + AsyncWrite + Unpin,
{
    let (_, mut ui_tx) = tokio::io::split(ui_stream);
    bootstrap_gate_inner(bootstrap_stream, &mut ui_tx, automatic_world).await
}

#[cfg(any(windows, test))]
async fn bootstrap_gate_inner<B, U>(
    bootstrap_stream: &mut B,
    ui_tx: &mut U,
    automatic_world: bool,
) -> Result<()>
where
    B: AsyncRead + AsyncWrite + Unpin,
    U: AsyncWrite + Unpin,
{
    let (mut bootstrap_rx, mut bootstrap_tx) = tokio::io::split(bootstrap_stream);
    let mut hello_seen = false;
    let mut world_loader_capable = false;
    let mut last_time = 0_u64;
    let stage_timeout = if automatic_world {
        Duration::from_secs(100)
    } else {
        Duration::from_secs(15 * 60)
    };
    loop {
        let message: bootstrap::Message =
            timeout(stage_timeout, ui::read(&mut bootstrap_rx)).await??;
        anyhow::ensure!(message.valid(), "invalid native bootstrap message");
        match message {
            bootstrap::Message::BootstrapHello {
                gta_edition,
                gta_build,
                fingerprint,
                capabilities,
                ..
            } => {
                anyhow::ensure!(
                    gta_edition == "enhanced"
                        && gta_build == adapter::GAME_VERSION
                        && fingerprint.eq_ignore_ascii_case(
                            "0C52864D4521D9C9D441348AA1156958792DDE8825D0297C851753F167336401"
                        ),
                    "unsupported native bootstrap fingerprint"
                );
                hello_seen = true;
                world_loader_capable = capabilities.iter().any(|value| value == "world_loader");
                ui::write(
                    &mut bootstrap_tx,
                    &bootstrap::Message::BootstrapCommand {
                        schema_version: bootstrap::VERSION,
                        command: bootstrap::Command::StartTelemetry,
                    },
                )
                .await?;
            }
            bootstrap::Message::TelemetryHelloV1 {
                gta_build,
                fingerprint,
                ..
            } => {
                anyhow::ensure!(
                    hello_seen
                        && gta_build == adapter::GAME_VERSION
                        && fingerprint.eq_ignore_ascii_case(
                            "0C52864D4521D9C9D441348AA1156958792DDE8825D0297C851753F167336401"
                        ),
                    "unsupported telemetry identity"
                );
            }
            bootstrap::Message::TelemetrySnapshotV1 { snapshot, .. } => {
                anyhow::ensure!(hello_seen, "telemetry preceded bootstrap identity");
                ui::write(
                    ui_tx,
                    &UiResponse::success(
                        "bridge-stage",
                        json!({
                            "stage": format!("telemetry_{}", snapshot.stage),
                            "message": "GameVerse проверяет загруженные компоненты"
                        }),
                    ),
                )
                .await?;
            }
            bootstrap::Message::TelemetryCandidatesV1 { candidates, .. } => {
                anyhow::ensure!(
                    hello_seen,
                    "candidate telemetry preceded bootstrap identity"
                );
                // Candidate RVAs are local research data. Deliberately expose only
                // the bounded count to the player-facing launcher.
                ui::write(
                    ui_tx,
                    &UiResponse::success(
                        "bridge-stage",
                        json!({
                            "stage": "telemetry_candidates_observed",
                            "message": format!(
                                "GameVerse проверил {} исследовательских кандидатов",
                                candidates.len()
                            )
                        }),
                    ),
                )
                .await?;
            }
            bootstrap::Message::BootstrapStage {
                monotonic_ms,
                stage,
                ..
            } => {
                anyhow::ensure!(monotonic_ms >= last_time, "non-monotonic bootstrap clock");
                last_time = monotonic_ms;
                ui::write(ui_tx, &UiResponse::success("bridge-stage", json!({
                    "stage": format!("bootstrap_{}", serde_json::to_value(stage)?.as_str().unwrap_or("failed")),
                    "message": "GameVerse готовит игровой мир"
                }))).await?;
                if stage == bootstrap::Stage::FrontendReady {
                    anyhow::ensure!(hello_seen, "bootstrap stage preceded hello");
                    if automatic_world && world_loader_capable {
                        ui::write(
                            &mut bootstrap_tx,
                            &bootstrap::Message::BootstrapCommand {
                                schema_version: bootstrap::VERSION,
                                command: bootstrap::Command::BeginWorld,
                            },
                        )
                        .await?;
                    } else {
                        ui::write(ui_tx, &UiResponse::success("bridge-stage", json!({
                            "stage":"telemetry_waiting_for_manual_story",
                            "message":"Для исследовательского trace вручную откройте Story Mode"
                        }))).await?;
                    }
                }
                if stage == bootstrap::Stage::WorldReady
                    || ((!automatic_world || !world_loader_capable)
                        && stage == bootstrap::Stage::AdapterReady)
                {
                    return Ok(());
                }
                if stage == bootstrap::Stage::Failed {
                    anyhow::bail!("native bootstrap entered failed state");
                }
            }
            bootstrap::Message::BootstrapFailure { code, message, .. } => {
                ui::write(
                    ui_tx,
                    &UiResponse::success(
                        "bridge-stage",
                        json!({"stage":"failed","message":&message,"error_code":&code}),
                    ),
                )
                .await?;
                anyhow::bail!("native bootstrap {code}: {message}");
            }
            bootstrap::Message::BootstrapCommand { .. } => {
                anyhow::bail!("unexpected bootstrap command")
            }
        }
    }
}

enum Bootstrap {
    Response(UiResponse, bool),
    Select(u64),
}

#[derive(Deserialize)]
struct AuthPayload {
    login: String,
    password: String,
    invite: Option<String>,
}
#[derive(Deserialize)]
struct ResumePayload {
    refresh_token: String,
}
#[derive(Deserialize)]
struct CharacterPayload {
    first_name: String,
    last_name: String,
    #[serde(default = "default_model")]
    model_hash: u32,
}
#[derive(Deserialize)]
struct SelectPayload {
    character_id: u64,
}
fn default_model() -> u32 {
    0x705e61f2
}

async fn handle_bootstrap(
    client: &mut InteractiveClient,
    authenticated: bool,
    request: &UiRequest,
) -> Result<Bootstrap> {
    match request.command.as_str() {
        "ui.ready" => Ok(Bootstrap::Response(
            UiResponse::success(&request.request_id, json!({"stage":"auth_required"})),
            authenticated,
        )),
        "auth.login" | "auth.register" => {
            let payload: AuthPayload = serde_json::from_value(request.payload.clone())?;
            let auth = if request.command == "auth.register" {
                AlphaAuthentication::Register {
                    login: payload.login,
                    password: payload.password,
                    invite: payload
                        .invite
                        .ok_or_else(|| anyhow::anyhow!("invite_required"))?,
                }
            } else {
                AlphaAuthentication::Login {
                    login: payload.login,
                    password: payload.password,
                }
            };
            let (account_id, access_token, refresh_token) =
                client.authenticate(&request.request_id, auth).await?;
            Ok(Bootstrap::Response(
                UiResponse::success(
                    &request.request_id,
                    json!({
                        "stage":"character_required",
                        "account_id":account_id,
                        "access_token":access_token,
                        "refresh_token":refresh_token
                    }),
                ),
                true,
            ))
        }
        "auth.resume" | "session.reconnect" => {
            let payload: ResumePayload = serde_json::from_value(request.payload.clone())?;
            let (account_id, access_token, refresh_token) = client
                .authenticate(
                    &request.request_id,
                    AlphaAuthentication::Resume {
                        refresh_token: payload.refresh_token,
                    },
                )
                .await?;
            Ok(Bootstrap::Response(
                UiResponse::success(
                    &request.request_id,
                    json!({
                        "stage":"character_required",
                        "account_id":account_id,
                        "access_token":access_token,
                        "refresh_token":refresh_token
                    }),
                ),
                true,
            ))
        }
        "characters.list" if authenticated => {
            let characters = client.characters(&request.request_id).await?;
            Ok(Bootstrap::Response(
                UiResponse::success(&request.request_id, json!({"characters":characters})),
                true,
            ))
        }
        "characters.create" if authenticated => {
            let value: CharacterPayload = serde_json::from_value(request.payload.clone())?;
            let characters = client
                .create_character(
                    &request.request_id,
                    NewCharacter {
                        first_name: value.first_name,
                        last_name: value.last_name,
                        model_hash: value.model_hash,
                    },
                )
                .await?;
            Ok(Bootstrap::Response(
                UiResponse::success(&request.request_id, json!({"characters":characters})),
                true,
            ))
        }
        "characters.select" if authenticated => {
            let value: SelectPayload = serde_json::from_value(request.payload.clone())?;
            Ok(Bootstrap::Select(value.character_id))
        }
        _ => Ok(Bootstrap::Response(
            UiResponse::error(
                &request.request_id,
                "invalid_session_state",
                "Команда недоступна на текущем этапе",
            ),
            authenticated,
        )),
    }
}

async fn serve_active(
    mut client: Client,
    mut adapter_rx: ReadHalf<impl AsyncRead + AsyncWrite + Unpin + Send + 'static>,
    mut adapter_tx: WriteHalf<impl AsyncRead + AsyncWrite + Unpin + Send + 'static>,
    mut ui_rx: ReadHalf<impl AsyncRead + AsyncWrite + Unpin + Send + 'static>,
    mut ui_tx: WriteHalf<impl AsyncRead + AsyncWrite + Unpin + Send + 'static>,
    finish: Instant,
) -> Result<()> {
    let (adapter_sender, mut adapter_incoming) = mpsc::channel(128);
    let _adapter_reader = ReaderGuard(tokio::spawn(async move {
        loop {
            let result = timeout(DEADLINE, ipc::read(&mut adapter_rx))
                .await
                .map_err(anyhow::Error::from)
                .and_then(|value| value);
            let failed = result.is_err();
            if adapter_sender.try_send(result).is_err() || failed {
                break;
            }
        }
    }));
    let (ui_sender, mut ui_incoming) = mpsc::channel(256);
    let _ui_reader = ReaderGuard(tokio::spawn(async move {
        loop {
            let result = ui::read::<UiRequest>(&mut ui_rx).await;
            let failed = result.is_err();
            if ui_sender.try_send(result).is_err() || failed {
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
            value=adapter_incoming.recv()=>match value {
                None | Some(Err(_)) => break,
                Some(Ok(message)) => match message {
                Message::LocalPlayerState{sequence:local_sequence,state}=>{
                    if local_sequence>sequence{sequence=local_sequence;}else{sequence+=1;}
                    client.publish(from_legacy(sequence,state))?;
                }
                Message::AdapterHeartbeat{game_ready:false}=>anyhow::bail!("game unavailable"),
                Message::AdapterHeartbeat{game_ready:true}|Message::AdapterStatus{..}|Message::AdapterError{..}|Message::BootstrapFailure{..}=>{},
                _=>anyhow::bail!("unexpected adapter message"),
                }
            },
            value=ui_incoming.recv()=>{
                let request=match value { Some(Ok(request))=>request, None|Some(Err(_))=>break };
                let response=match handle_active(&mut client,&request).await {
                    Ok(value)=>value,
                    Err(error)=>UiResponse::error(&request.request_id,"request_failed",public_error(&error)),
                };
                ui::write(&mut ui_tx,&response).await?;
                if matches!(request.command.as_str(), "auth.logout" | "session.end") && response.ok {
                    let reason = if request.command == "auth.logout" { "logout" } else { "launcher_closed" };
                    ipc::write(&mut adapter_tx,&Message::SessionEnd { reason:reason.into() }).await?;
                    return Ok(());
                }
            },
            network=client.read_frame()=>{replica.apply(network?)?;},
            _=frame.tick()=>{
                let entities=replica.legacy_entities();
                let present:BTreeSet<_>=entities.iter().map(|value|value.id).filter(|id|*id!=client.entity).collect();
                for id in known.difference(&present){
                    timeout(DEADLINE,ipc::write(&mut adapter_tx,&Message::RemoteEntityDestroy{id:*id})).await??;
                }
                for entity in entities.into_iter().filter(|value|value.id!=client.entity){
                    let message=if known.contains(&entity.id){Message::RemoteEntityUpdate{entity}}else{Message::RemoteEntityCreate{entity}};
                    timeout(DEADLINE,ipc::write(&mut adapter_tx,&message)).await??;
                }
                known=present;
                timeout(DEADLINE,ipc::write(&mut adapter_tx,&Message::AdapterHeartbeat{game_ready:true})).await??;
            },
            _=tokio::signal::ctrl_c()=>break,
        }
    }
    let _ = timeout(
        DEADLINE,
        ipc::write(
            &mut adapter_tx,
            &Message::SessionEnd {
                reason: "bridge_session_closed".into(),
            },
        ),
    )
    .await;
    client.close().await
}

#[derive(Deserialize)]
struct TextPayload {
    message: String,
}
#[derive(Deserialize)]
struct JobPayload {
    #[serde(default = "default_route")]
    route: String,
    idempotency_key: Option<String>,
}
#[derive(Deserialize)]
struct BuyPayload {
    #[serde(default = "default_shop")]
    shop: String,
    item_id: u32,
    #[serde(default = "one")]
    quantity: u32,
    idempotency_key: Option<String>,
}
fn default_route() -> String {
    "alpha-route".into()
}
fn default_shop() -> String {
    "market".into()
}
fn one() -> u32 {
    1
}

async fn handle_active(client: &mut Client, request: &UiRequest) -> Result<UiResponse> {
    let payload = match request.command.as_str() {
        "chat.send" => {
            let value: TextPayload = serde_json::from_value(request.payload.clone())?;
            client
                .exchange(ControlMessage::ChatCommand {
                    request_id: request.request_id.clone(),
                    message: value.message,
                })
                .await?
        }
        "inventory.request" => ControlMessage::InventorySnapshot {
            request_id: request.request_id.clone(),
            revision: 0,
            items: client
                .inventory(&request.request_id)
                .await?
                .into_iter()
                .map(
                    |(item_id, quantity)| gameverse_protocol::control_v2::InventoryItem {
                        item_id,
                        quantity,
                    },
                )
                .collect(),
        },
        "job.start" => {
            let value: JobPayload = serde_json::from_value(request.payload.clone())?;
            client
                .start_delivery(&request.request_id, &value.route)
                .await?;
            return Ok(UiResponse::success(
                &request.request_id,
                json!({"active_route":value.route}),
            ));
        }
        "job.finish" => {
            let value: JobPayload = serde_json::from_value(request.payload.clone())?;
            let key = value
                .idempotency_key
                .unwrap_or_else(|| request.request_id.clone());
            let (transaction_id, cash, bank) = client
                .finish_delivery(&request.request_id, &value.route, &key)
                .await?;
            return Ok(UiResponse::success(
                &request.request_id,
                json!({"transaction_id":transaction_id,"cash":cash,"bank":bank}),
            ));
        }
        "shop.catalog" => {
            let items = client.shop_catalog(&request.request_id, "market").await?;
            return Ok(UiResponse::success(
                &request.request_id,
                json!({"shop":"market","items":items}),
            ));
        }
        "shop.buy" => {
            let value: BuyPayload = serde_json::from_value(request.payload.clone())?;
            let key = value
                .idempotency_key
                .unwrap_or_else(|| request.request_id.clone());
            let (transaction_id, cash, bank) = client
                .buy(
                    &request.request_id,
                    &value.shop,
                    value.item_id,
                    value.quantity,
                    &key,
                )
                .await?;
            return Ok(UiResponse::success(
                &request.request_id,
                json!({"transaction_id":transaction_id,"cash":cash,"bank":bank}),
            ));
        }
        "auth.logout" => {
            client.logout(&request.request_id).await?;
            return Ok(UiResponse::success(
                &request.request_id,
                json!({"stage":"auth_required"}),
            ));
        }
        "session.end" => {
            return Ok(UiResponse::success(
                &request.request_id,
                json!({"stage":"stopping"}),
            ));
        }
        _ => {
            return Ok(UiResponse::error(
                &request.request_id,
                "invalid_session_state",
                "Команда недоступна после входа в мир",
            ))
        }
    };
    Ok(UiResponse::success(
        &request.request_id,
        serde_json::to_value(payload)?,
    ))
}

fn public_error(error: &anyhow::Error) -> String {
    let text = error.to_string();
    if text.len() <= 256 {
        text
    } else {
        "Операция не выполнена".into()
    }
}

#[cfg(test)]
mod bootstrap_tests {
    use super::*;

    fn stage(value: bootstrap::Stage, monotonic_ms: u64) -> bootstrap::Message {
        bootstrap::Message::BootstrapStage {
            schema_version: bootstrap::VERSION,
            monotonic_ms,
            stage: value,
        }
    }

    #[tokio::test]
    async fn native_gate_requires_verified_identity_and_world_ready() {
        let (mut bootstrap_host, mut bootstrap_peer) = tokio::io::duplex(4096);
        let (mut ui_host, mut ui_peer) = tokio::io::duplex(4096);
        let gate =
            tokio::spawn(
                async move { bootstrap_gate(&mut bootstrap_host, &mut ui_host, true).await },
            );
        ui::write(
            &mut ui_peer,
            &UiRequest {
                schema_version: ui::VERSION,
                request_id: "native-gate-test".into(),
                command: "ui.hello".into(),
                payload: json!({}),
            },
        )
        .await
        .unwrap();
        assert!(ui::read::<UiResponse>(&mut ui_peer).await.unwrap().ok);

        ui::write(&mut bootstrap_peer, &stage(bootstrap::Stage::Loaded, 1))
            .await
            .unwrap();
        assert!(ui::read::<UiResponse>(&mut ui_peer).await.unwrap().ok);
        ui::write(
            &mut bootstrap_peer,
            &bootstrap::Message::BootstrapHello {
                schema_version: bootstrap::VERSION,
                bootstrap_build: "0.1.0".into(),
                gta_edition: "enhanced".into(),
                gta_build: adapter::GAME_VERSION.into(),
                fingerprint: "0C52864D4521D9C9D441348AA1156958792DDE8825D0297C851753F167336401"
                    .into(),
                capabilities: vec!["telemetry".into(), "world_loader".into()],
            },
        )
        .await
        .unwrap();
        assert_eq!(
            ui::read::<bootstrap::Message>(&mut bootstrap_peer)
                .await
                .unwrap(),
            bootstrap::Message::BootstrapCommand {
                schema_version: bootstrap::VERSION,
                command: bootstrap::Command::StartTelemetry
            }
        );
        ui::write(
            &mut bootstrap_peer,
            &bootstrap::Message::TelemetryHelloV1 {
                schema_version: bootstrap::VERSION,
                probe_build: "0.1.0".into(),
                gta_build: adapter::GAME_VERSION.into(),
                fingerprint: "0C52864D4521D9C9D441348AA1156958792DDE8825D0297C851753F167336401"
                    .into(),
                capabilities: vec!["pe_sections".into()],
            },
        )
        .await
        .unwrap();
        for (value, timestamp) in [
            (bootstrap::Stage::Verified, 2),
            (bootstrap::Stage::FrontendReady, 3),
        ] {
            ui::write(&mut bootstrap_peer, &stage(value, timestamp))
                .await
                .unwrap();
            assert!(ui::read::<UiResponse>(&mut ui_peer).await.unwrap().ok);
        }
        assert_eq!(
            ui::read::<bootstrap::Message>(&mut bootstrap_peer)
                .await
                .unwrap(),
            bootstrap::Message::BootstrapCommand {
                schema_version: bootstrap::VERSION,
                command: bootstrap::Command::BeginWorld
            }
        );
        ui::write(&mut bootstrap_peer, &stage(bootstrap::Stage::WorldReady, 4))
            .await
            .unwrap();
        assert!(ui::read::<UiResponse>(&mut ui_peer).await.unwrap().ok);
        gate.await.unwrap().unwrap();
    }

    #[cfg(windows)]
    #[tokio::test]
    async fn ui_handshake_completes_before_native_bootstrap_connects() {
        use tokio::net::windows::named_pipe::ClientOptions;

        let unique = format!(
            "{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let adapter_pipe = format!(r"\\.\pipe\gameverse-adapter-order-{unique}");
        let ui_pipe = format!(r"\\.\pipe\gameverse-ui-order-{unique}");
        let bootstrap_pipe = format!(r"\\.\pipe\gameverse-bootstrap-order-{unique}");
        let runner_adapter = adapter_pipe.clone();
        let runner_ui = ui_pipe.clone();
        let runner_bootstrap = bootstrap_pipe.clone();
        let runner = tokio::spawn(async move {
            run_telemetry(
                &runner_adapter,
                &runner_ui,
                &runner_bootstrap,
                "127.0.0.1:9".parse().unwrap(),
                Path::new("unused-certificate.der"),
                Duration::from_secs(10),
            )
            .await
        });

        let mut client = None;
        for _ in 0..100 {
            match ClientOptions::new().open(&ui_pipe) {
                Ok(value) => {
                    client = Some(value);
                    break;
                }
                Err(error) if matches!(error.raw_os_error(), Some(2 | 231)) => {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
                Err(error) => panic!("open UI pipe: {error}"),
            }
        }
        let mut client = client.expect("UI pipe did not become available");
        ui::write(
            &mut client,
            &UiRequest {
                schema_version: ui::VERSION,
                request_id: "startup-order".into(),
                command: "ui.hello".into(),
                payload: json!({}),
            },
        )
        .await
        .unwrap();
        let response = timeout(Duration::from_secs(2), ui::read::<UiResponse>(&mut client))
            .await
            .expect("UI handshake waited for native bootstrap")
            .unwrap();
        assert!(response.ok);
        assert_eq!(response.payload["stage"], "auth_required");
        runner.abort();
    }
}
