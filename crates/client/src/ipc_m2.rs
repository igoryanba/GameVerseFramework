//! Windows bridge joining the untrusted UI, GTA adapter and M2 QUIC.
use crate::{
    ipc,
    m2::{from_legacy, AlphaAuthentication, Client, InteractiveClient, NewCharacter, Replica},
    ui::{self, UiRequest, UiResponse},
};
use anyhow::Result;
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
    net::windows::named_pipe::{NamedPipeServer, ServerOptions},
    sync::mpsc,
    time::timeout,
};

const DEADLINE: Duration = Duration::from_secs(8);

struct ReaderGuard(tokio::task::JoinHandle<()>);
impl Drop for ReaderGuard {
    fn drop(&mut self) {
        self.0.abort();
    }
}

pub async fn run(
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
        json!({"event":"m2_pipe_ready","adapter_pipe":adapter_pipe,"ui_pipe":ui_pipe})
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
        if let Err(error) = serve(adapter, ui_stream, server, cert, finish).await {
            eprintln!(
                "{}",
                json!({"event":"m2_session_disconnected","error":error.to_string()})
            );
        }
    }
}

fn valid_pipe(pipe: &str) -> bool {
    pipe.starts_with(r"\\.\pipe\") && pipe.len() <= 240
}

fn listener(pipe: &str, first: bool) -> Result<NamedPipeServer> {
    let mut options = ServerOptions::new();
    options.reject_remote_clients(true);
    if first {
        options.first_pipe_instance(true);
    }
    Ok(options.create(pipe)?)
}

async fn serve(
    adapter_stream: NamedPipeServer,
    ui_stream: NamedPipeServer,
    server: SocketAddr,
    cert: &Path,
    finish: Instant,
) -> Result<()> {
    let (mut adapter_rx, mut adapter_tx) = tokio::io::split(adapter_stream);
    let (mut ui_rx, mut ui_tx) = tokio::io::split(ui_stream);

    let hello: UiRequest = timeout(DEADLINE, ui::read(&mut ui_rx)).await??;
    anyhow::ensure!(
        hello.valid() && hello.command == "ui.hello",
        "invalid UI handshake"
    );
    ui::write(
        &mut ui_tx,
        &UiResponse::success(
            &hello.request_id,
            json!({
                "bridge_build":env!("CARGO_PKG_VERSION"),
                "stage":"waiting_for_adapter",
                "capabilities":["auth","characters","chat","inventory","shop","job"]
            }),
        ),
    )
    .await?;

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
    let build = match timeout(DEADLINE, ipc::read(&mut adapter_rx)).await?? {
        Message::GameInfo { edition, build } if edition == "enhanced" => build,
        _ => anyhow::bail!("unsupported GTA edition"),
    };
    let mut interactive = Some(InteractiveClient::connect(server, cert, Some(build)).await?);
    let mut authenticated = false;
    loop {
        let request: UiRequest = timeout(DEADLINE * 8, ui::read(&mut ui_rx)).await??;
        if !request.valid() {
            ui::write(
                &mut ui_tx,
                &UiResponse::error(
                    request.request_id,
                    "invalid_request",
                    "Некорректная команда интерфейса",
                ),
            )
            .await?;
            continue;
        }
        let result = handle_bootstrap(
            interactive.as_mut().expect("interactive client"),
            authenticated,
            &request,
        )
        .await;
        match result {
            Ok(Bootstrap::Response(response, now_authenticated)) => {
                authenticated = now_authenticated;
                ui::write(&mut ui_tx, &response).await?;
            }
            Ok(Bootstrap::Select(character_id)) => {
                let pending = interactive
                    .take()
                    .expect("interactive client")
                    .select_character(&request.request_id, character_id)
                    .await?;
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
                        Message::AdapterError { code, message } => {
                            anyhow::bail!("adapter {code}: {message}")
                        }
                        Message::AdapterHeartbeat { game_ready: true }
                        | Message::AdapterStatus { .. } => {}
                        _ => anyhow::bail!("adapter did not confirm session bootstrap"),
                    }
                }
                let client = pending.spawn_ready(&request.request_id).await?;
                ui::write(
                    &mut ui_tx,
                    &UiResponse::success(
                        &request.request_id,
                        json!({
                            "stage":"active",
                            "session":client.session,
                            "entity":client.entity,
                            "character_id":client.config.character_id
                        }),
                    ),
                )
                .await?;
                return serve_active(client, adapter_rx, adapter_tx, ui_rx, ui_tx, finish).await;
            }
            Err(error) => {
                ui::write(
                    &mut ui_tx,
                    &UiResponse::error(&request.request_id, "request_failed", public_error(&error)),
                )
                .await?
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
    mut adapter_rx: tokio::io::ReadHalf<NamedPipeServer>,
    mut adapter_tx: tokio::io::WriteHalf<NamedPipeServer>,
    mut ui_rx: tokio::io::ReadHalf<NamedPipeServer>,
    mut ui_tx: tokio::io::WriteHalf<NamedPipeServer>,
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
            value=adapter_incoming.recv()=>match value.ok_or_else(||anyhow::anyhow!("adapter reader stopped"))?? {
                Message::LocalPlayerState{sequence:local_sequence,state}=>{
                    if local_sequence>sequence{sequence=local_sequence;}else{sequence+=1;}
                    client.publish(from_legacy(sequence,state))?;
                }
                Message::AdapterHeartbeat{game_ready:false}=>anyhow::bail!("game unavailable"),
                Message::AdapterHeartbeat{game_ready:true}|Message::AdapterStatus{..}|Message::AdapterError{..}=>{},
                _=>anyhow::bail!("unexpected adapter message"),
            },
            value=ui_incoming.recv()=>{
                let request=value.ok_or_else(||anyhow::anyhow!("UI reader stopped"))??;
                let response=match handle_active(&mut client,&request).await {
                    Ok(value)=>value,
                    Err(error)=>UiResponse::error(&request.request_id,"request_failed",public_error(&error)),
                };
                ui::write(&mut ui_tx,&response).await?;
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
    "general".into()
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
            let items = client.shop_catalog(&request.request_id, "general").await?;
            return Ok(UiResponse::success(
                &request.request_id,
                json!({"shop":"general","items":items}),
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
