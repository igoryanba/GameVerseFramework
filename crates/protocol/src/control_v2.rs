//! Reliable M2 control plane. High-frequency state uses realtime datagrams.
use crate::{adapter::SessionConfig, presence_v2, EntityId, Error, SessionId, MAX_FRAME};
use serde::{Deserialize, Serialize};

pub const VERSION: u16 = 2;
pub const RESOURCE_API_VERSION: u16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthMode {
    Register,
    Login,
}

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
        server_id: String,
        max_players: u16,
        tick_hz: u16,
    },
    SessionBegin {
        session: SessionId,
        entity: EntityId,
        config: SessionConfig,
    },
    SpawnReady {
        request_id: String,
    },
    SpawnAck {
        request_id: String,
    },
    AuthRequest {
        request_id: String,
        mode: AuthMode,
        login: String,
        password: String,
        invite: Option<String>,
    },
    AuthResume {
        request_id: String,
        refresh_token: String,
    },
    Logout {
        request_id: String,
        refresh_token: String,
    },
    LogoutResult {
        request_id: String,
    },
    AuthResult {
        request_id: String,
        account_id: u64,
        access_token: String,
        refresh_token: String,
        expires_at_ms: u64,
    },
    CharacterList {
        request_id: String,
        characters: Vec<CharacterSummary>,
    },
    CreateCharacter {
        request_id: String,
        first_name: String,
        last_name: String,
        model_hash: u32,
    },
    SelectCharacter {
        request_id: String,
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
        request_id: String,
        message: String,
    },
    ChatMessage {
        source: Option<CharacterSummary>,
        channel: String,
        message: String,
    },
    InventoryCommand {
        request_id: String,
        action: String,
        item_id: u32,
        quantity: u32,
        idempotency_key: String,
    },
    InventorySnapshot {
        request_id: String,
        revision: u64,
        items: Vec<InventoryItem>,
    },
    EconomyResult {
        request_id: String,
        transaction_id: u64,
        cash: i64,
        bank: i64,
    },
    JobCommand {
        request_id: String,
        action: String,
        route: String,
        idempotency_key: Option<String>,
    },
    JobState {
        request_id: String,
        active_route: Option<String>,
        revision: u64,
    },
    ShopCatalog {
        request_id: String,
        shop: String,
        items: Vec<ShopItem>,
    },
    ShopCommand {
        request_id: String,
        shop: String,
        item_id: u32,
        quantity: u32,
        idempotency_key: String,
    },
    Disconnect {
        reason: String,
    },
    Reject {
        code: String,
        reason: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShopItem {
    pub item_id: u32,
    pub name: String,
    pub price: i64,
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
                server_id,
                max_players,
                tick_hz,
            } => {
                *control_version == VERSION
                    && *presence_version == presence_v2::VERSION
                    && valid_text(server_id, 128)
                    && *max_players > 0
                    && *max_players <= presence_v2::MAX_PLAYERS as u16
                    && *tick_hz > 0
                    && *tick_hz <= 60
            }
            Self::SessionBegin {
                session,
                entity,
                config,
            } => *session > 0 && valid_entity(entity) && config.valid(),
            Self::AuthRequest {
                request_id,
                mode,
                login,
                password,
                invite,
            } => {
                valid_request_id(request_id)
                    && (3..=64).contains(&login.len())
                    && (8..=256).contains(&password.len())
                    && match mode {
                        AuthMode::Register => {
                            invite.as_ref().is_some_and(|value| valid_text(value, 256))
                        }
                        AuthMode::Login => invite.is_none(),
                    }
            }
            Self::AuthResume {
                request_id,
                refresh_token,
            }
            | Self::Logout {
                request_id,
                refresh_token,
            } => valid_request_id(request_id) && valid_text(refresh_token, 256),
            Self::AuthResult {
                request_id,
                account_id,
                access_token,
                refresh_token,
                expires_at_ms,
            } => {
                valid_request_id(request_id)
                    && *account_id > 0
                    && !access_token.is_empty()
                    && access_token.len() <= 4096
                    && valid_text(refresh_token, 256)
                    && *expires_at_ms > 0
            }
            Self::CharacterList {
                request_id,
                characters,
            } => {
                valid_request_id(request_id)
                    && characters.len() <= 3
                    && characters.iter().all(valid_character)
            }
            Self::CreateCharacter {
                request_id,
                first_name,
                last_name,
                model_hash,
            } => {
                valid_request_id(request_id)
                    && valid_name(first_name)
                    && valid_name(last_name)
                    && *model_hash != 0
            }
            Self::SelectCharacter {
                request_id,
                character_id,
            } => valid_request_id(request_id) && *character_id > 0,
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
            Self::ChatCommand {
                request_id,
                message,
            } => valid_request_id(request_id) && valid_text(message, 512),
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
                request_id,
                action,
                item_id,
                quantity,
                idempotency_key,
            } => {
                valid_request_id(request_id)
                    && valid_text(action, 32)
                    && if action == "snapshot" {
                        *item_id == 0 && *quantity == 0 && idempotency_key.is_empty()
                    } else {
                        *item_id > 0
                            && (1..=100).contains(quantity)
                            && valid_text(idempotency_key, 128)
                    }
            }
            Self::InventorySnapshot {
                request_id, items, ..
            } => {
                valid_request_id(request_id)
                    && items.len() <= 256
                    && items
                        .iter()
                        .all(|item| item.item_id > 0 && item.quantity > 0)
            }
            Self::EconomyResult {
                request_id,
                transaction_id,
                cash,
                bank,
            } => valid_request_id(request_id) && *transaction_id > 0 && *cash >= 0 && *bank >= 0,
            Self::JobCommand {
                request_id,
                action,
                route,
                idempotency_key,
            } => {
                valid_request_id(request_id)
                    && valid_text(action, 32)
                    && valid_text(route, 64)
                    && idempotency_key
                        .as_ref()
                        .is_none_or(|key| valid_text(key, 128))
            }
            Self::JobState {
                request_id,
                active_route,
                ..
            } => {
                valid_request_id(request_id)
                    && active_route
                        .as_ref()
                        .is_none_or(|route| valid_text(route, 64))
            }
            Self::ShopCatalog {
                request_id,
                shop,
                items,
            } => {
                valid_request_id(request_id)
                    && valid_text(shop, 64)
                    && items.len() <= 256
                    && items.iter().all(|item| {
                        item.item_id > 0 && valid_text(&item.name, 64) && item.price > 0
                    })
            }
            Self::ShopCommand {
                request_id,
                shop,
                item_id,
                quantity,
                idempotency_key,
            } => {
                valid_request_id(request_id)
                    && valid_text(shop, 64)
                    && *item_id > 0
                    && (1..=100).contains(quantity)
                    && valid_text(idempotency_key, 128)
            }
            Self::Disconnect { reason } => !reason.is_empty() && reason.len() <= 256,
            Self::Reject { code, reason } => {
                !code.is_empty() && code.len() <= 64 && !reason.is_empty() && reason.len() <= 256
            }
            Self::SpawnReady { request_id } | Self::SpawnAck { request_id } => {
                valid_request_id(request_id)
            }
            Self::LogoutResult { request_id } => valid_request_id(request_id),
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
fn valid_request_id(value: &str) -> bool {
    valid_text(value, 128)
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
            request_id: "request-1".into(),
            action: "buy".into(),
            item_id: 1,
            quantity: 0,
            idempotency_key: "purchase-1".into(),
        })
        .is_err());
    }
}
