//! M1 presence v1. Client-owned transforms, server-owned identity and lifecycle.
//! This is deliberately separate from the deterministic M0 input protocol.
use crate::{EntityId, SessionId, Tick, MAX_FRAME, MAX_PLAYERS};
use serde::{Deserialize, Serialize};

pub const VERSION: u16 = 1;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerState {
    pub timestamp_ms: u64,
    pub position: [f32; 3],
    /// GTA quaternion order: x, y, z, w.
    pub rotation: [f32; 4],
    pub velocity: [f32; 3],
    pub model_hash: u32,
    pub health: u16,
    pub armor: u16,
    /// on_foot=1, running=2, sprinting=4, jumping=8, ragdoll=16, aiming=32,
    /// falling=64, shooting=128, reloading=256.
    pub movement: u16,
    pub weapon_hash: u32,
}
impl PlayerState {
    pub fn valid(&self) -> bool {
        let norm: f32 = self.rotation.iter().map(|x| x * x).sum();
        self.position
            .iter()
            .all(|x| x.is_finite() && x.abs() <= 20_000.0)
            && self
                .velocity
                .iter()
                .all(|x| x.is_finite() && x.abs() <= 500.0)
            && self.rotation.iter().all(|x| x.is_finite())
            && (norm - 1.0).abs() <= 0.02
            && self.model_hash != 0
            && self.health <= 1000
            && self.armor <= 1000
            && self.movement & !511 == 0
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entity {
    pub id: EntityId,
    pub state: PlayerState,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub tick: Tick,
    pub ack: u64,
    pub entities: Vec<Entity>,
}
impl Snapshot {
    pub fn valid(&self) -> bool {
        self.entities.len() <= MAX_PLAYERS
            && self.entities.iter().enumerate().all(|(i, e)| {
                e.id.slot < MAX_PLAYERS as u32
                    && e.id.generation > 0
                    && e.state.valid()
                    && !self.entities[..i].iter().any(|old| old.id == e.id)
            })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Message {
    Heartbeat,
    Hello {
        version: u16,
    },
    Welcome {
        version: u16,
        session: SessionId,
        entity: EntityId,
    },
    Reject {
        reason: String,
    },
    PlayerState {
        sequence: u64,
        state: PlayerState,
    },
    Snapshot {
        state: Snapshot,
    },
    Disconnect {
        reason: String,
    },
}
impl Message {
    pub fn valid(&self) -> bool {
        match self {
            Self::PlayerState { sequence, state } => *sequence > 0 && state.valid(),
            Self::Snapshot { state } => state.valid(),
            Self::Welcome {
                session, entity, ..
            } => *session > 0 && entity.generation > 0 && entity.slot < MAX_PLAYERS as u32,
            Self::Reject { reason } | Self::Disconnect { reason } => reason.len() <= 256,
            Self::Hello { .. } | Self::Heartbeat => true,
        }
    }
}

pub fn encode(message: &Message) -> Result<Vec<u8>, crate::Error> {
    if !message.valid() {
        return Err(crate::Error::Values);
    }
    let body = serde_json::to_vec(message)?;
    if body.is_empty() || body.len() > MAX_FRAME {
        return Err(crate::Error::Length);
    }
    let mut frame = (body.len() as u32).to_be_bytes().to_vec();
    frame.extend(body);
    Ok(frame)
}
pub fn decode(body: &[u8]) -> Result<Message, crate::Error> {
    if body.is_empty() || body.len() > MAX_FRAME {
        return Err(crate::Error::Length);
    }
    let message: Message = serde_json::from_slice(body)?;
    if !message.valid() {
        return Err(crate::Error::Values);
    }
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;
    pub fn state() -> PlayerState {
        PlayerState {
            timestamp_ms: 1,
            position: [1.0, 2.0, 3.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            velocity: [0.0; 3],
            model_hash: 0x705e61f2,
            health: 200,
            armor: 0,
            movement: 1,
            weapon_hash: 0xa2719263,
        }
    }
    #[test]
    fn validates_transform_and_rejects_spoofed_identity() {
        let mut s = state();
        assert!(s.valid());
        s.rotation = [0.0; 4];
        assert!(!s.valid());
        s = state();
        s.position[2] = f32::NAN;
        assert!(!s.valid());
        let m = Message::PlayerState {
            sequence: 1,
            state: state(),
        };
        let wire = encode(&m).unwrap();
        assert_eq!(decode(&wire[4..]).unwrap(), m);
        let mut json = serde_json::to_value(m).unwrap();
        json["entity_id"] = serde_json::json!({"slot":1,"generation":1});
        assert!(decode(&serde_json::to_vec(&json).unwrap()).is_err());
    }
}
