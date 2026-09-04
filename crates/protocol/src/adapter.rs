//! Named-pipe protocol v1. All GTA calls remain in the adapter script tick.
use crate::{
    presence::{Entity, PlayerState},
    EntityId, Error, MAX_FRAME,
};
use serde::{Deserialize, Serialize};
pub const VERSION: u16 = 1;
pub const DEFAULT_PIPE: &str = r"\\.\pipe\gameverse-gta-v1";
pub const GAME_VERSION: &str = "1.0.1158.13";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionConfig {
    pub spawn: [f32; 3],
    pub heading: f32,
    pub model_hash: u32,
    pub instance_id: u32,
}
impl SessionConfig {
    pub fn valid(&self) -> bool {
        self.spawn
            .iter()
            .all(|v| v.is_finite() && v.abs() <= 20_000.0)
            && self.heading.is_finite()
            && self.model_hash != 0
    }
}
impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            // Los Santos International Airport, outside the Story prologue area.
            spawn: [-1037.7, -2737.7, 20.17],
            heading: 330.0,
            model_hash: 0x705e61f2,
            instance_id: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Message {
    AdapterHello {
        version: u16,
        backend: String,
    },
    GameInfo {
        edition: String,
        build: String,
    },
    SessionBegin {
        session: u64,
        entity: EntityId,
        config: SessionConfig,
    },
    LocalPlayerState {
        sequence: u64,
        state: PlayerState,
    },
    RemoteEntityCreate {
        entity: Entity,
    },
    RemoteEntityUpdate {
        entity: Entity,
    },
    RemoteEntityDestroy {
        id: EntityId,
    },
    AdapterHeartbeat {
        game_ready: bool,
    },
    AdapterStatus {
        event: String,
        id: Option<EntityId>,
    },
    AdapterError {
        code: String,
        message: String,
    },
    Reset {
        reason: String,
    },
}
impl Message {
    pub fn valid(&self) -> bool {
        let id_valid = |id: &EntityId| id.slot < crate::MAX_PLAYERS as u32 && id.generation > 0;
        match self {
            Self::AdapterHello { backend, .. } => backend.len() <= 64,
            Self::GameInfo { edition, build } => edition.len() <= 32 && build.len() <= 64,
            Self::SessionBegin {
                session,
                entity,
                config,
            } => *session > 0 && id_valid(entity) && config.valid(),
            Self::LocalPlayerState { sequence, state } => *sequence > 0 && state.valid(),
            Self::RemoteEntityCreate { entity } | Self::RemoteEntityUpdate { entity } => {
                id_valid(&entity.id) && entity.state.valid()
            }
            Self::RemoteEntityDestroy { id } => id_valid(id),
            Self::AdapterStatus { event, id } => {
                event.len() <= 64 && id.as_ref().is_none_or(id_valid)
            }
            Self::AdapterError { code, message } => code.len() <= 64 && message.len() <= 512,
            Self::Reset { reason } => reason.len() <= 256,
            Self::AdapterHeartbeat { .. } => true,
        }
    }
}
pub fn encode(message: &Message) -> Result<Vec<u8>, Error> {
    if !message.valid() {
        return Err(Error::Values);
    }
    let body = serde_json::to_vec(message)?;
    if body.is_empty() || body.len() > MAX_FRAME {
        return Err(Error::Length);
    }
    let mut bytes = (body.len() as u32).to_be_bytes().to_vec();
    bytes.extend(body);
    Ok(bytes)
}
pub fn decode(body: &[u8]) -> Result<Message, Error> {
    if body.is_empty() || body.len() > MAX_FRAME {
        return Err(Error::Length);
    }
    let message: Message = serde_json::from_slice(body)?;
    if !message.valid() {
        return Err(Error::Values);
    }
    Ok(message)
}
