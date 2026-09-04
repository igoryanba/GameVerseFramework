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
        let (session, entity) = match timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await?? {
            ControlMessage::ServerHello {
                control_version: VERSION,
                presence_version: p::VERSION,
                session,
                entity,
                ..
            } => (session, entity),
            ControlMessage::Reject { code, reason } => {
                anyhow::bail!("server rejected {code}: {reason}")
            }
            _ => anyhow::bail!("invalid M2 server hello"),
        };
        let config = match timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await?? {
            ControlMessage::SessionBegin { config } => config,
            _ => anyhow::bail!("missing session configuration"),
        };
        write_control(&mut send, &ControlMessage::SpawnReady).await?;
        anyhow::ensure!(
            matches!(
                timeout(HANDSHAKE_TIMEOUT, read_control(&mut recv)).await??,
                ControlMessage::SpawnAck
            ),
            "missing spawn acknowledgement"
        );
        Ok(Self {
            session,
            entity,
            config,
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
