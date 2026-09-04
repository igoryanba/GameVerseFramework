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
#[serde(deny_unknown_fields)]
pub struct CharacterSummary {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub model_hash: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryItem {
    pub item_id: u32,
    pub quantity: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceEvent {
    pub resource: String,
    pub name: String,
    pub source: Option<u64>,
    pub target: Option<u64>,
    pub arguments: Vec<serde_json::Value>,
    pub correlation_id: Option<String>,
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
    AuthRequest {
        login: String,
        password: String,
        invite: Option<String>,
    },
    AuthResult {
        account_id: u64,
        access_token: String,
        expires_at_ms: u64,
    },
    CharacterList {
        characters: Vec<CharacterSummary>,
    },
    CreateCharacter {
        first_name: String,
        last_name: String,
        model_hash: u32,
    },
    SelectCharacter {
        character_id: u64,
    },
    EntityCreate {
        entity: EntityId,
    },
    EntityDestroy {
        entity: EntityId,
    },
    EntityStreamOut {
        entity: EntityId,
    },
    VehicleCreate {
        id: presence_v2::VehicleId,
        model_hash: u32,
        frame: presence_v2::VehicleFrame,
    },
    VehicleOwnership {
        id: presence_v2::VehicleId,
        owner: Option<SessionId>,
    },
    VehicleOccupancy {
        id: presence_v2::VehicleId,
        occupants: Vec<(i8, EntityId)>,
    },
    ResourceEvent {
        event: ResourceEvent,
    },
    ChatCommand {
        message: String,
    },
    ChatMessage {
        source: Option<CharacterSummary>,
        channel: String,
        message: String,
    },
    InventoryCommand {
        action: String,
        item_id: u32,
        quantity: u32,
        idempotency_key: String,
    },
    InventorySnapshot {
        revision: u64,
        items: Vec<InventoryItem>,
    },
    EconomyResult {
        transaction_id: u64,
        cash: i64,
        bank: i64,
    },
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
            Self::AuthRequest {
                login,
                password,
                invite,
            } => {
                (3..=64).contains(&login.len())
                    && (8..=256).contains(&password.len())
                    && invite.as_ref().is_none_or(|value| value.len() <= 256)
            }
            Self::AuthResult {
                account_id,
                access_token,
                expires_at_ms,
            } => {
                *account_id > 0
                    && !access_token.is_empty()
                    && access_token.len() <= 4096
                    && *expires_at_ms > 0
            }
            Self::CharacterList { characters } => {
                characters.len() <= 3 && characters.iter().all(valid_character)
            }
            Self::CreateCharacter {
                first_name,
                last_name,
                model_hash,
            } => valid_name(first_name) && valid_name(last_name) && *model_hash != 0,
            Self::SelectCharacter { character_id } => *character_id > 0,
            Self::EntityCreate { entity }
            | Self::EntityDestroy { entity }
            | Self::EntityStreamOut { entity } => valid_entity(entity),
            Self::VehicleCreate {
                id,
                model_hash,
                frame,
            } => valid_vehicle(*id) && *model_hash != 0 && frame.valid(),
            Self::VehicleOwnership { id, owner } => {
                valid_vehicle(*id) && owner.is_none_or(|value| value > 0)
            }
            Self::VehicleOccupancy { id, occupants } => {
                valid_vehicle(*id)
                    && occupants.len() <= 16
                    && occupants
                        .iter()
                        .all(|(seat, entity)| (-1..=15).contains(seat) && valid_entity(entity))
            }
            Self::ResourceEvent { event } => valid_resource_event(event),
            Self::ChatCommand { message } => valid_text(message, 512),
            Self::ChatMessage {
                source,
                channel,
                message,
            } => {
                source.as_ref().is_none_or(valid_character)
                    && valid_text(channel, 32)
                    && valid_text(message, 512)
            }
            Self::InventoryCommand {
                action,
                item_id,
                quantity,
                idempotency_key,
            } => {
                valid_text(action, 32)
                    && *item_id > 0
                    && (1..=100).contains(quantity)
                    && valid_text(idempotency_key, 128)
            }
            Self::InventorySnapshot { items, .. } => {
                items.len() <= 256
                    && items
                        .iter()
                        .all(|item| item.item_id > 0 && item.quantity > 0)
            }
            Self::EconomyResult {
                transaction_id,
                cash,
                bank,
            } => *transaction_id > 0 && *cash >= 0 && *bank >= 0,
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

fn valid_text(value: &str, max: usize) -> bool {
    !value.trim().is_empty() && value.len() <= max
}
fn valid_name(value: &str) -> bool {
    (2..=32).contains(&value.chars().count())
        && value
            .chars()
            .all(|character| character.is_alphabetic() || character == '-' || character == '\'')
}
fn valid_entity(value: &EntityId) -> bool {
    value.slot < presence_v2::MAX_PLAYERS as u32 && value.generation > 0
}
fn valid_vehicle(value: presence_v2::VehicleId) -> bool {
    value.slot < presence_v2::MAX_VEHICLES as u32 && value.generation > 0
}
fn valid_character(value: &CharacterSummary) -> bool {
    value.id > 0
        && valid_name(&value.first_name)
        && valid_name(&value.last_name)
        && value.model_hash != 0
}
fn valid_resource_event(value: &ResourceEvent) -> bool {
    valid_text(&value.resource, 128)
        && valid_text(&value.name, 128)
        && value.arguments.len() <= 64
        && value
            .correlation_id
            .as_ref()
            .is_none_or(|id| valid_text(id, 128))
        && serde_json::to_vec(&value.arguments).is_ok_and(|bytes| bytes.len() <= 64 * 1024)
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

    #[test]
    fn alpha_commands_are_bounded_before_transport() {
        let event = ControlMessage::ResourceEvent {
            event: ResourceEvent {
                resource: "alpha".into(),
                name: "ping".into(),
                source: Some(1),
                target: None,
                arguments: vec![serde_json::Value::String("x".repeat(64 * 1024))],
                correlation_id: Some("request-1".into()),
            },
        };
        assert!(encode(&event).is_err());
        assert!(encode(&ControlMessage::InventoryCommand {
            action: "buy".into(),
            item_id: 1,
            quantity: 0,
            idempotency_key: "purchase-1".into(),
        })
        .is_err());
    }
}
