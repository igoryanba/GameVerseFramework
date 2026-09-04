//! Bounded, game-independent M0 wire contract. Length prefix is u32 big endian.
pub mod adapter;
pub mod control_v2;
pub mod presence;
pub mod presence_v2;
use serde::{Deserialize, Serialize};

pub const VERSION: u16 = 0;
pub const MAX_FRAME: usize = 64 * 1024;
pub const MAX_PLAYERS: usize = 2;
pub type SessionId = u64;
pub type Tick = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityId {
    pub slot: u32,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entity {
    pub id: EntityId,
    pub position: [f32; 2],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub tick: Tick,
    pub ack: u64,
    pub entities: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Message {
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
    Input {
        sequence: u64,
        direction: [f32; 2],
    },
    Snapshot {
        state: Snapshot,
    },
    Disconnect {
        reason: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("frame length must be 1..=65536 bytes")]
    Length,
    #[error("invalid message: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid message values")]
    Values,
}

pub fn frame_length(prefix: [u8; 4]) -> Result<usize, Error> {
    let n = u32::from_be_bytes(prefix) as usize;
    if n == 0 || n > MAX_FRAME {
        return Err(Error::Length);
    }
    Ok(n)
}

pub fn valid_direction(direction: [f32; 2]) -> bool {
    direction
        .iter()
        .all(|x| x.is_finite() && (-1.0..=1.0).contains(x))
}

impl Message {
    pub fn validate(&self) -> Result<(), Error> {
        let valid = match self {
            Self::Input {
                sequence,
                direction,
            } => *sequence > 0 && valid_direction(*direction),
            Self::Snapshot { state } => {
                state.entities.len() <= MAX_PLAYERS
                    && state
                        .entities
                        .iter()
                        .all(|e| e.position.iter().all(|x| x.is_finite()))
                    && state
                        .entities
                        .iter()
                        .enumerate()
                        .all(|(i, e)| !state.entities[..i].iter().any(|p| p.id == e.id))
            }
            Self::Reject { reason } | Self::Disconnect { reason } => reason.len() <= 256,
            _ => true,
        };
        if valid {
            Ok(())
        } else {
            Err(Error::Values)
        }
    }
}

pub fn encode(message: &Message) -> Result<Vec<u8>, Error> {
    message.validate()?;
    let body = serde_json::to_vec(message)?;
    frame_length((body.len() as u32).to_be_bytes())?;
    let mut frame = Vec::with_capacity(body.len() + 4);
    frame.extend_from_slice(&(body.len() as u32).to_be_bytes());
    frame.extend(body);
    Ok(frame)
}

pub fn decode(body: &[u8]) -> Result<Message, Error> {
    if body.is_empty() || body.len() > MAX_FRAME {
        return Err(Error::Length);
    }
    let message: Message = serde_json::from_slice(body)?;
    message.validate()?;
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn golden_hello_and_roundtrip() {
        let frame = encode(&Message::Hello { version: 0 }).unwrap();
        let body = br#"{"type":"hello","version":0}"#;
        assert_eq!(&frame[4..], body);
        assert_eq!(
            frame_length(frame[..4].try_into().unwrap()).unwrap(),
            body.len()
        );
        assert_eq!(decode(body).unwrap(), Message::Hello { version: 0 });
    }
    #[test]
    fn rejects_bad_frames_and_values() {
        for n in [0, 65537, u32::MAX] {
            assert!(frame_length(n.to_be_bytes()).is_err());
        }
        for b in [
            b"{".as_slice(),
            br#"{"type":"unknown"}"#,
            br#"{"type":"hello","version":0,"extra":1}"#,
        ] {
            assert!(decode(b).is_err());
        }
        for direction in [[2.0, 0.0], [f32::NAN, 0.0], [f32::INFINITY, 0.0]] {
            assert!(encode(&Message::Input {
                sequence: 1,
                direction
            })
            .is_err());
        }
    }
}
