//! Reliable M2 control plane. High-frequency state uses realtime datagrams.
use crate::{adapter::SessionConfig, presence_v2, EntityId, Error, SessionId, MAX_FRAME};
use serde::{Deserialize, Serialize};

pub const VERSION: u16 = 2;
pub const RESOURCE_API_VERSION: u16 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Capabilities {
    pub datagrams: bool,
    pub resource_api: u16,
    pub gta_edition: Option<String>,
    pub gta_build: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ControlMessage {
    ClientHello {
        control_versions: Vec<u16>,
        presence_versions: Vec<u16>,
        client_build: String,
        capabilities: Capabilities,
    },
    ServerHello {
        control_version: u16,
        presence_version: u16,
        session: SessionId,
        entity: EntityId,
        max_players: u16,
        tick_hz: u16,
    },
    SessionBegin {
        config: SessionConfig,
    },
    SpawnReady,
    SpawnAck,
    Disconnect {
        reason: String,
    },
    Reject {
        code: String,
        reason: String,
    },
}

impl ControlMessage {
    pub fn validate(&self) -> Result<(), Error> {
        let valid = match self {
            Self::ClientHello {
                control_versions,
                presence_versions,
                client_build,
                capabilities,
            } => {
                !control_versions.is_empty()
                    && control_versions.len() <= 8
                    && !presence_versions.is_empty()
                    && presence_versions.len() <= 8
                    && !client_build.is_empty()
                    && client_build.len() <= 64
                    && capabilities.resource_api <= RESOURCE_API_VERSION
                    && capabilities
                        .gta_edition
                        .as_ref()
                        .is_none_or(|v| v.len() <= 32)
                    && capabilities
                        .gta_build
                        .as_ref()
                        .is_none_or(|v| v.len() <= 64)
            }
            Self::ServerHello {
                control_version,
                presence_version,
                session,
                entity,
                max_players,
                tick_hz,
            } => {
                *control_version == VERSION
                    && *presence_version == presence_v2::VERSION
                    && *session > 0
                    && entity.generation > 0
                    && *max_players > 0
                    && *max_players <= presence_v2::MAX_PLAYERS as u16
                    && *tick_hz > 0
                    && *tick_hz <= 60
            }
            Self::SessionBegin { config } => config.valid(),
            Self::Disconnect { reason } => !reason.is_empty() && reason.len() <= 256,
            Self::Reject { code, reason } => {
                !code.is_empty() && code.len() <= 64 && !reason.is_empty() && reason.len() <= 256
            }
            Self::SpawnReady | Self::SpawnAck => true,
        };
        if valid {
            Ok(())
        } else {
            Err(Error::Values)
        }
    }
}

pub fn encode(message: &ControlMessage) -> Result<Vec<u8>, Error> {
    message.validate()?;
    let body = serde_json::to_vec(message)?;
    if body.is_empty() || body.len() > MAX_FRAME {
        return Err(Error::Length);
    }
    let mut output = Vec::with_capacity(body.len() + 4);
    output.extend_from_slice(&(body.len() as u32).to_be_bytes());
    output.extend_from_slice(&body);
    Ok(output)
}
pub fn decode(body: &[u8]) -> Result<ControlMessage, Error> {
    if body.is_empty() || body.len() > MAX_FRAME {
        return Err(Error::Length);
    }
    let message: ControlMessage = serde_json::from_slice(body)?;
    message.validate()?;
    Ok(message)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum RealtimeMessage {
    Player {
        frame: presence_v2::PlayerFrame,
    },
    Server {
        frame: presence_v2::ServerFrame,
    },
    Vehicle {
        id: presence_v2::VehicleId,
        frame: presence_v2::VehicleFrame,
    },
}
impl RealtimeMessage {
    pub fn validate(&self) -> Result<(), Error> {
        let valid = match self {
            Self::Player { frame } => frame.valid(),
            Self::Server { frame } => frame.valid(),
            Self::Vehicle { id, frame } => {
                id.slot < presence_v2::MAX_VEHICLES as u32 && id.generation > 0 && frame.valid()
            }
        };
        if valid {
            Ok(())
        } else {
            Err(Error::Values)
        }
    }
}
pub fn encode_realtime(message: &RealtimeMessage) -> Result<Vec<u8>, Error> {
    message.validate()?;
    let bytes = serde_json::to_vec(message)?;
    if bytes.is_empty() || bytes.len() > 16 * 1024 {
        return Err(Error::Length);
    }
    Ok(bytes)
}
pub fn decode_realtime(bytes: &[u8]) -> Result<RealtimeMessage, Error> {
    if bytes.is_empty() || bytes.len() > 16 * 1024 {
        return Err(Error::Length);
    }
    let message: RealtimeMessage = serde_json::from_slice(bytes)?;
    message.validate()?;
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn negotiable_hello_roundtrips_and_rejects_unbounded_values() {
        let hello = ControlMessage::ClientHello {
            control_versions: vec![2],
            presence_versions: vec![1, 2],
            client_build: "test".into(),
            capabilities: Capabilities {
                datagrams: true,
                resource_api: 1,
                gta_edition: Some("enhanced".into()),
                gta_build: Some("test".into()),
            },
        };
        let encoded = encode(&hello).unwrap();
        assert_eq!(decode(&encoded[4..]).unwrap(), hello);
        let invalid = ControlMessage::ClientHello {
            control_versions: vec![],
            presence_versions: vec![2],
            client_build: "test".into(),
            capabilities: Capabilities {
                datagrams: true,
                resource_api: 1,
                gta_edition: None,
                gta_build: None,
            },
        };
        assert!(encode(&invalid).is_err());
    }
}
