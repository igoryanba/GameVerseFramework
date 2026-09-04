//! M2 client session using reliable bootstrap and Presence v2 datagrams.
use anyhow::Result;
use gameverse_protocol::{
    adapter::SessionConfig,
    control_v2::{Capabilities, ControlMessage, RealtimeMessage, RESOURCE_API_VERSION, VERSION},
    presence_v2::{self as p, PlayerFrame, ServerFrame},
    EntityId, SessionId,
};
use gameverse_transport::{
    client_endpoint,
    presence_v2::{read_realtime, send_realtime},
    quinn, read_control, write_control, HANDSHAKE_TIMEOUT,
};
use std::{collections::BTreeMap, net::SocketAddr, path::Path};
use tokio::time::timeout;

pub struct Client {
    pub session: SessionId,
    pub entity: EntityId,
    pub config: SessionConfig,
    pub account_id: Option<u64>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    endpoint: quinn::Endpoint,
    connection: quinn::Connection,
    send: quinn::SendStream,
    recv: quinn::RecvStream,
}
impl Client {
    pub async fn connect(
        address: SocketAddr,
        cert: &Path,
        gta_build: Option<String>,
    ) -> Result<Self> {
        let endpoint = client_endpoint(cert)?;
        let connection =
            timeout(HANDSHAKE_TIMEOUT, endpoint.connect(address, "localhost")?).await??;
        let (mut send, mut recv) = timeout(HANDSHAKE_TIMEOUT, connection.open_bi()).await??;
        write_control(
            &mut send,
            &ControlMessage::ClientHello {
                control_versions: vec![VERSION],
                presence_versions: vec![p::VERSION, 1],
                client_build: env!("CARGO_PKG_VERSION").into(),
                capabilities: Capabilities {
                    datagrams: true,
                    resource_api: RESOURCE_API_VERSION,
                    gta_edition: gta_build.as_ref().map(|_| "enhanced".into()),
                    gta_build,
                },
            },
        )
        .await?;
        match timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await?? {
            ControlMessage::ServerHello {
                control_version: VERSION,
                presence_version: p::VERSION,
                ..
            } => {}
            ControlMessage::Reject { code, reason } => {
                anyhow::bail!("server rejected {code}: {reason}")
            }
            _ => anyhow::bail!("invalid M2 server hello"),
        };
        let (session, entity, config) =
            match timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await?? {
                ControlMessage::SessionBegin {
                    session,
                    entity,
                    config,
                } => (session, entity, config),
                _ => anyhow::bail!("missing session configuration"),
            };
        write_control(
            &mut send,
            &ControlMessage::SpawnReady {
                request_id: "diagnostic-spawn".into(),
            },
        )
        .await?;
        anyhow::ensure!(
            matches!(
                timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await??,
                ControlMessage::SpawnAck { request_id } if request_id == "diagnostic-spawn"
            ),
            "missing spawn acknowledgement"
        );
        Ok(Self {
            session,
            entity,
            config,
            account_id: None,
            access_token: None,
            refresh_token: None,
            endpoint,
            connection,
            send,
            recv,
        })
    }

    pub async fn connect_alpha(
        address: SocketAddr,
        cert: &Path,
        authentication: AlphaAuthentication,
        character: NewCharacter,
    ) -> Result<Self> {
        let endpoint = client_endpoint(cert)?;
        let connection =
            timeout(HANDSHAKE_TIMEOUT, endpoint.connect(address, "localhost")?).await??;
        let (mut send, mut recv) = timeout(HANDSHAKE_TIMEOUT, connection.open_bi()).await??;
        write_control(
            &mut send,
            &ControlMessage::ClientHello {
                control_versions: vec![VERSION],
                presence_versions: vec![p::VERSION],
                client_build: env!("CARGO_PKG_VERSION").into(),
                capabilities: Capabilities {
                    datagrams: true,
                    resource_api: RESOURCE_API_VERSION,
                    gta_edition: None,
                    gta_build: None,
                },
            },
        )
        .await?;
        match timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await?? {
            ControlMessage::ServerHello {
                control_version: VERSION,
                presence_version: p::VERSION,
                ..
            } => {}
            ControlMessage::Reject { code, reason } => {
                anyhow::bail!("server rejected {code}: {reason}")
            }
            _ => anyhow::bail!("invalid alpha server hello"),
        }

        let auth_request_id = "alpha-auth".to_string();
        let auth_message = match authentication {
            AlphaAuthentication::Register {
                login,
                password,
                invite,
            } => ControlMessage::AuthRequest {
                request_id: auth_request_id.clone(),
                mode: gameverse_protocol::control_v2::AuthMode::Register,
                login,
                password,
                invite: Some(invite),
            },
            AlphaAuthentication::Login { login, password } => ControlMessage::AuthRequest {
                request_id: auth_request_id.clone(),
                mode: gameverse_protocol::control_v2::AuthMode::Login,
                login,
                password,
                invite: None,
            },
            AlphaAuthentication::Resume { refresh_token } => ControlMessage::AuthResume {
                request_id: auth_request_id.clone(),
                refresh_token,
            },
        };
        write_control(&mut send, &auth_message).await?;
        let (account_id, access_token, refresh_token) =
            match timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await?? {
                ControlMessage::AuthResult {
                    request_id,
                    account_id,
                    access_token,
                    refresh_token,
                    ..
                } if request_id == auth_request_id => (account_id, access_token, refresh_token),
                ControlMessage::Reject { code, reason } => {
                    anyhow::bail!("authentication rejected {code}: {reason}")
                }
                _ => anyhow::bail!("invalid authentication result"),
            };

        write_control(
            &mut send,
            &ControlMessage::CharacterList {
                request_id: "alpha-characters".into(),
                characters: vec![],
            },
        )
        .await?;
        let mut characters = match timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await?? {
            ControlMessage::CharacterList {
                request_id,
                characters,
            } if request_id == "alpha-characters" => characters,
            _ => anyhow::bail!("invalid character list"),
        };
        if characters.is_empty() {
            write_control(
                &mut send,
                &ControlMessage::CreateCharacter {
                    request_id: "alpha-create-character".into(),
                    first_name: character.first_name,
                    last_name: character.last_name,
                    model_hash: character.model_hash,
                },
            )
            .await?;
            characters = match timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await?? {
                ControlMessage::CharacterList {
                    request_id,
                    characters,
                } if request_id == "alpha-create-character" => characters,
                _ => anyhow::bail!("invalid create-character response"),
            };
        }
        let selected = characters
            .first()
            .ok_or_else(|| anyhow::anyhow!("account has no character"))?;
        write_control(
            &mut send,
            &ControlMessage::SelectCharacter {
                request_id: "alpha-select-character".into(),
                character_id: selected.id,
            },
        )
        .await?;
        let (session, entity, config) =
            match timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await?? {
                ControlMessage::SessionBegin {
                    session,
                    entity,
                    config,
                } => (session, entity, config),
                _ => anyhow::bail!("missing alpha session configuration"),
            };
        write_control(
            &mut send,
            &ControlMessage::SpawnReady {
                request_id: "alpha-spawn".into(),
            },
        )
        .await?;
        anyhow::ensure!(
            matches!(
                timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await??,
                ControlMessage::SpawnAck { request_id } if request_id == "alpha-spawn"
            ),
            "missing alpha spawn acknowledgement"
        );
        Ok(Self {
            session,
            entity,
            config,
            account_id: Some(account_id),
            access_token: Some(access_token),
            refresh_token: Some(refresh_token),
            endpoint,
            connection,
            send,
            recv,
        })
    }
    pub fn publish(&self, frame: PlayerFrame) -> Result<()> {
        send_realtime(&self.connection, &RealtimeMessage::Player { frame }).map(|_| ())
    }
    pub async fn read_frame(&self) -> Result<ServerFrame> {
        match read_realtime(&self.connection).await? {
            RealtimeMessage::Server { frame } => Ok(frame),
            _ => anyhow::bail!("unexpected realtime message"),
        }
    }
    pub async fn read_control(&mut self) -> Result<ControlMessage> {
        read_control(&mut self.recv).await
    }
    pub async fn inventory(&mut self, request_id: &str) -> Result<Vec<(u32, u32)>> {
        write_control(
            &mut self.send,
            &ControlMessage::InventoryCommand {
                request_id: request_id.into(),
                action: "snapshot".into(),
                item_id: 0,
                quantity: 0,
                idempotency_key: String::new(),
            },
        )
        .await?;
        match read_control(&mut self.recv).await? {
            ControlMessage::InventorySnapshot {
                request_id: response_id,
                items,
                ..
            } if response_id == request_id => Ok(items
                .into_iter()
                .map(|item| (item.item_id, item.quantity))
                .collect()),
            message => anyhow::bail!("unexpected inventory response: {message:?}"),
        }
    }

    pub async fn start_delivery(&mut self, request_id: &str, route: &str) -> Result<()> {
        write_control(
            &mut self.send,
            &ControlMessage::JobCommand {
                request_id: request_id.into(),
                action: "start".into(),
                route: route.into(),
                idempotency_key: None,
            },
        )
        .await?;
        anyhow::ensure!(
            matches!(
                read_control(&mut self.recv).await?,
                ControlMessage::JobState { request_id: response_id, active_route: Some(active), .. }
                    if response_id == request_id && active == route
            ),
            "invalid start-delivery response"
        );
        Ok(())
    }

    pub async fn finish_delivery(
        &mut self,
        request_id: &str,
        route: &str,
        idempotency_key: &str,
    ) -> Result<(u64, i64, i64)> {
        write_control(
            &mut self.send,
            &ControlMessage::JobCommand {
                request_id: request_id.into(),
                action: "finish".into(),
                route: route.into(),
                idempotency_key: Some(idempotency_key.into()),
            },
        )
        .await?;
        economy_response(&mut self.recv, request_id).await
    }

    pub async fn buy(
        &mut self,
        request_id: &str,
        shop: &str,
        item_id: u32,
        quantity: u32,
        idempotency_key: &str,
    ) -> Result<(u64, i64, i64)> {
        write_control(
            &mut self.send,
            &ControlMessage::ShopCommand {
                request_id: request_id.into(),
                shop: shop.into(),
                item_id,
                quantity,
                idempotency_key: idempotency_key.into(),
            },
        )
        .await?;
        economy_response(&mut self.recv, request_id).await
    }
    pub async fn close(mut self) -> Result<()> {
        write_control(
            &mut self.send,
            &ControlMessage::Disconnect {
                reason: "client shutdown".into(),
            },
        )
        .await?;
        self.send.finish().await?;
        self.connection.close(0_u32.into(), b"client shutdown");
        self.endpoint.wait_idle().await;
        Ok(())
    }
}

async fn economy_response(
    recv: &mut quinn::RecvStream,
    request_id: &str,
) -> Result<(u64, i64, i64)> {
    match read_control(recv).await? {
        ControlMessage::EconomyResult {
            request_id: response_id,
            transaction_id,
            cash,
            bank,
        } if response_id == request_id => Ok((transaction_id, cash, bank)),
        message => anyhow::bail!("unexpected economy response: {message:?}"),
    }
}

pub enum AlphaAuthentication {
    Register {
        login: String,
        password: String,
        invite: String,
    },
    Login {
        login: String,
        password: String,
    },
    Resume {
        refresh_token: String,
    },
}

pub struct NewCharacter {
    pub first_name: String,
    pub last_name: String,
    pub model_hash: u32,
}

#[derive(Default)]
pub struct Replica {
    baseline: u64,
    entities: BTreeMap<EntityId, PlayerFrame>,
}
impl Replica {
    pub fn apply(&mut self, frame: ServerFrame) -> Result<bool> {
        anyhow::ensure!(frame.valid(), "invalid server frame");
        if frame.baseline <= self.baseline {
            return Ok(false);
        }
        for delta in frame.deltas {
            match delta.kind {
                p::DeltaKind::StreamOut | p::DeltaKind::Destroy => {
                    self.entities.remove(&delta.id);
                }
                p::DeltaKind::Upsert => {
                    if let Some(current) = self.entities.get_mut(&delta.id) {
                        if let Some(value) = delta.transform {
                            current.transform = value;
                        }
                        if let Some(value) = delta.appearance {
                            current.appearance = Some(value);
                        }
                        if let Some(value) = delta.locomotion {
                            current.locomotion = value;
                        }
                        if let Some(value) = delta.combat {
                            current.combat = value;
                        }
                        if let Some(value) = delta.vehicle {
                            current.vehicle = Some(value);
                        }
                        for cleared in delta.cleared {
                            match cleared {
                                p::ComponentKind::Appearance => current.appearance = None,
                                p::ComponentKind::Vehicle => current.vehicle = None,
                            }
                        }
                    } else {
                        let transform = delta
                            .transform
                            .ok_or_else(|| anyhow::anyhow!("new entity delta lacks transform"))?;
                        let locomotion = delta
                            .locomotion
                            .ok_or_else(|| anyhow::anyhow!("new entity delta lacks locomotion"))?;
                        let combat = delta
                            .combat
                            .ok_or_else(|| anyhow::anyhow!("new entity delta lacks combat"))?;
                        self.entities.insert(
                            delta.id,
                            PlayerFrame {
                                sequence: frame.server_tick.max(1),
                                client_tick: frame.server_tick,
                                transform,
                                appearance: delta.appearance,
                                locomotion,
                                combat,
                                vehicle: delta.vehicle,
                            },
                        );
                    }
                }
            }
        }
        self.baseline = frame.baseline;
        Ok(true)
    }
    pub fn entities(&self) -> &BTreeMap<EntityId, PlayerFrame> {
        &self.entities
    }
    pub fn legacy_entities(&self) -> Vec<gameverse_protocol::presence::Entity> {
        self.entities
            .iter()
            .filter_map(|(id, frame)| {
                frame.appearance.as_ref().map(|appearance| {
                    let movement = match frame.locomotion {
                        p::Locomotion::Idle | p::Locomotion::Walk => 1,
                        p::Locomotion::Run => 1 | 2,
                        p::Locomotion::Sprint => 1 | 4,
                        p::Locomotion::Jump => 1 | 8,
                        p::Locomotion::Fall => 1 | 64,
                        p::Locomotion::Ragdoll => 1 | 16,
                    } | if frame.combat.aiming { 32 } else { 0 }
                        | if frame.combat.shooting { 128 } else { 0 }
                        | if frame.combat.reloading { 256 } else { 0 };
                    gameverse_protocol::presence::Entity {
                        id: *id,
                        state: gameverse_protocol::presence::PlayerState {
                            timestamp_ms: frame.client_tick,
                            position: frame.transform.position,
                            rotation: frame.transform.rotation,
                            velocity: frame.transform.velocity,
                            model_hash: appearance.model_hash,
                            health: if frame.combat.dead { 0 } else { 200 },
                            armor: 0,
                            movement,
                            weapon_hash: frame.combat.weapon_hash,
                        },
                    }
                })
            })
            .collect()
    }
}

pub fn from_legacy(sequence: u64, state: gameverse_protocol::presence::PlayerState) -> PlayerFrame {
    let locomotion = if state.movement & 16 != 0 {
        p::Locomotion::Ragdoll
    } else if state.movement & 8 != 0 {
        p::Locomotion::Jump
    } else if state.movement & 64 != 0 {
        p::Locomotion::Fall
    } else if state.movement & 4 != 0 {
        p::Locomotion::Sprint
    } else if state.movement & 2 != 0 {
        p::Locomotion::Run
    } else if state.velocity.iter().map(|v| v * v).sum::<f32>() > 0.04 {
        p::Locomotion::Walk
    } else {
        p::Locomotion::Idle
    };
    PlayerFrame {
        sequence,
        client_tick: state.timestamp_ms,
        transform: p::Transform {
            position: state.position,
            rotation: state.rotation,
            velocity: state.velocity,
        },
        appearance: Some(p::Appearance {
            model_hash: state.model_hash,
        }),
        locomotion,
        combat: p::CombatPresentation {
            aiming: state.movement & 32 != 0,
            shooting: state.movement & 128 != 0,
            reloading: state.movement & 256 != 0,
            dead: state.health == 0,
            weapon_hash: state.weapon_hash,
            aim_target: None,
        },
        vehicle: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replica_applies_delta_once_and_converts_for_existing_adapter() {
        let id = EntityId {
            slot: 1,
            generation: 1,
        };
        let input = gameverse_protocol::presence::PlayerState {
            timestamp_ms: 7,
            position: [1.0, 2.0, 3.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            velocity: [1.0, 0.0, 0.0],
            model_hash: 5,
            health: 200,
            armor: 0,
            movement: 3,
            weapon_hash: 9,
        };
        let player = from_legacy(2, input);
        let mut replica = Replica::default();
        let frame = ServerFrame {
            server_tick: 2,
            baseline: 1,
            deltas: vec![p::EntityDelta {
                id,
                kind: p::DeltaKind::Upsert,
                transform: Some(player.transform.clone()),
                appearance: player.appearance.clone(),
                locomotion: Some(player.locomotion),
                combat: Some(player.combat.clone()),
                vehicle: None,
                cleared: vec![],
            }],
        };
        assert!(replica.apply(frame.clone()).unwrap());
        assert!(!replica.apply(frame).unwrap());
        assert_eq!(replica.legacy_entities()[0].state.model_hash, 5);
    }
}
