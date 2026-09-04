//! Presence v2 component contract. V1 remains available for M1 compatibility.
use crate::{EntityId, Error, MAX_FRAME};
use serde::{Deserialize, Serialize};

pub const VERSION: u16 = 2;
pub const MAX_PLAYERS: usize = 32;
pub const INTEREST_RADIUS: f32 = 400.0;
pub const MAX_VEHICLES: usize = 128;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClientHello {
    pub supported_versions: Vec<u16>,
}

pub fn negotiate_version(hello: &ClientHello) -> Option<u16> {
    [VERSION, crate::presence::VERSION]
        .into_iter()
        .find(|version| hello.supported_versions.contains(version))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Locomotion {
    Idle,
    Walk,
    Run,
    Sprint,
    Jump,
    Fall,
    Ragdoll,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub velocity: [f32; 3],
}
impl Transform {
    pub fn valid(&self) -> bool {
        let norm: f32 = self.rotation.iter().map(|v| v * v).sum();
        self.position
            .iter()
            .all(|v| v.is_finite() && v.abs() <= 20_000.0)
            && self
                .velocity
                .iter()
                .all(|v| v.is_finite() && v.abs() <= 500.0)
            && self.rotation.iter().all(|v| v.is_finite())
            && (norm - 1.0).abs() <= 0.02
    }
    pub fn distance_squared(&self, other: &Self) -> f32 {
        self.position
            .iter()
            .zip(other.position)
            .map(|(a, b)| (a - b).powi(2))
            .sum()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Appearance {
    pub model_hash: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VehicleId {
    pub slot: u32,
    pub generation: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombatPresentation {
    pub aiming: bool,
    pub shooting: bool,
    pub reloading: bool,
    pub dead: bool,
    pub weapon_hash: u32,
    pub aim_target: Option<[f32; 3]>,
}
impl CombatPresentation {
    pub fn valid(&self) -> bool {
        self.aim_target
            .is_none_or(|p| p.iter().all(|v| v.is_finite() && v.abs() <= 20_000.0))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VehicleOccupancy {
    pub vehicle: VehicleId,
    pub seat: i8,
}
impl VehicleOccupancy {
    pub fn valid(&self) -> bool {
        self.vehicle.slot < MAX_VEHICLES as u32
            && self.vehicle.generation > 0
            && (-1..=15).contains(&self.seat)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VehicleFrame {
    pub sequence: u64,
    pub transform: Transform,
    pub steering: f32,
    pub throttle: f32,
    pub brake: f32,
    pub gear: i8,
    pub engine_health: f32,
    pub body_health: f32,
}
impl VehicleFrame {
    pub fn valid(&self) -> bool {
        self.sequence > 0
            && self.transform.valid()
            && [self.steering, self.throttle, self.brake]
                .iter()
                .all(|value| value.is_finite() && (-1.0..=1.0).contains(value))
            && (-1..=10).contains(&self.gear)
            && self.engine_health.is_finite()
            && (-4_000.0..=1_000.0).contains(&self.engine_health)
            && self.body_health.is_finite()
            && (0.0..=1_000.0).contains(&self.body_health)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerFrame {
    pub sequence: u64,
    pub client_tick: u64,
    pub transform: Transform,
    pub appearance: Option<Appearance>,
    pub locomotion: Locomotion,
    pub combat: CombatPresentation,
    pub vehicle: Option<VehicleOccupancy>,
}
impl PlayerFrame {
    pub fn valid(&self) -> bool {
        self.sequence > 0
            && self.transform.valid()
            && self.appearance.as_ref().is_none_or(|a| a.model_hash != 0)
            && self.combat.valid()
            && self.vehicle.as_ref().is_none_or(VehicleOccupancy::valid)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeltaKind {
    Upsert,
    StreamOut,
    Destroy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentKind {
    Appearance,
    Vehicle,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityDelta {
    pub id: EntityId,
    pub kind: DeltaKind,
    pub transform: Option<Transform>,
    pub appearance: Option<Appearance>,
    pub locomotion: Option<Locomotion>,
    pub combat: Option<CombatPresentation>,
    pub vehicle: Option<VehicleOccupancy>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cleared: Vec<ComponentKind>,
}
impl EntityDelta {
    pub fn valid(&self) -> bool {
        if self.id.slot >= MAX_PLAYERS as u32 || self.id.generation == 0 {
            return false;
        }
        let unique_clears = self
            .cleared
            .iter()
            .enumerate()
            .all(|(i, component)| !self.cleared[..i].contains(component));
        match self.kind {
            DeltaKind::Upsert => {
                unique_clears
                    && (self.transform.is_some()
                        || self.appearance.is_some()
                        || self.locomotion.is_some()
                        || self.combat.is_some()
                        || self.vehicle.is_some()
                        || !self.cleared.is_empty())
                    && self.transform.as_ref().is_none_or(Transform::valid)
                    && self.appearance.as_ref().is_none_or(|a| a.model_hash != 0)
                    && self.combat.as_ref().is_none_or(CombatPresentation::valid)
                    && self.vehicle.as_ref().is_none_or(VehicleOccupancy::valid)
            }
            DeltaKind::StreamOut | DeltaKind::Destroy => {
                self.transform.is_none()
                    && self.appearance.is_none()
                    && self.locomotion.is_none()
                    && self.combat.is_none()
                    && self.vehicle.is_none()
                    && self.cleared.is_empty()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServerFrame {
    pub server_tick: u64,
    pub baseline: u64,
    pub deltas: Vec<EntityDelta>,
}
impl ServerFrame {
    pub fn valid(&self) -> bool {
        self.deltas.len() <= MAX_PLAYERS
            && self.deltas.iter().all(EntityDelta::valid)
            && self
                .deltas
                .iter()
                .enumerate()
                .all(|(i, d)| !self.deltas[..i].iter().any(|p| p.id == d.id))
    }
}

pub fn encode_frame(frame: &ServerFrame) -> Result<Vec<u8>, Error> {
    if !frame.valid() {
        return Err(Error::Values);
    }
    let bytes = serde_json::to_vec(frame)?;
    if bytes.is_empty() || bytes.len() > MAX_FRAME {
        return Err(Error::Length);
    }
    Ok(bytes)
}
pub fn decode_frame(bytes: &[u8]) -> Result<ServerFrame, Error> {
    if bytes.is_empty() || bytes.len() > MAX_FRAME {
        return Err(Error::Length);
    }
    let frame: ServerFrame = serde_json::from_slice(bytes)?;
    if !frame.valid() {
        return Err(Error::Values);
    }
    Ok(frame)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn transform(x: f32) -> Transform {
        Transform {
            position: [x, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            velocity: [0.0; 3],
        }
    }
    #[test]
    fn negotiates_highest_shared_version_and_keeps_v1() {
        assert_eq!(
            negotiate_version(&ClientHello {
                supported_versions: vec![1, 2]
            }),
            Some(2)
        );
        assert_eq!(
            negotiate_version(&ClientHello {
                supported_versions: vec![1]
            }),
            Some(1)
        );
        assert_eq!(
            negotiate_version(&ClientHello {
                supported_versions: vec![99]
            }),
            None
        );
    }
    #[test]
    fn validates_components_and_delta_lifecycle() {
        let id = EntityId {
            slot: 1,
            generation: 1,
        };
        let frame = ServerFrame {
            server_tick: 7,
            baseline: 3,
            deltas: vec![EntityDelta {
                id,
                kind: DeltaKind::Upsert,
                transform: Some(transform(2.0)),
                appearance: Some(Appearance { model_hash: 1 }),
                locomotion: Some(Locomotion::Run),
                combat: None,
                vehicle: None,
                cleared: vec![],
            }],
        };
        let encoded = encode_frame(&frame).unwrap();
        assert_eq!(decode_frame(&encoded).unwrap(), frame);
        let invalid = ServerFrame {
            server_tick: 8,
            baseline: 3,
            deltas: vec![EntityDelta {
                id,
                kind: DeltaKind::StreamOut,
                transform: Some(transform(2.0)),
                appearance: None,
                locomotion: None,
                combat: None,
                vehicle: None,
                cleared: vec![],
            }],
        };
        assert!(encode_frame(&invalid).is_err());
    }
    #[test]
    fn matches_csharp_golden_fixture() {
        let fixture = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../adapters/gta5/protocol/presence-v2.json"
        ))
        .trim()
        .as_bytes();
        let frame = decode_frame(fixture).expect("C# and Rust must share the Presence v2 fixture");
        assert_eq!(frame.server_tick, 7);
        assert_eq!(frame.deltas[0].locomotion, Some(Locomotion::Run));
        assert_eq!(encode_frame(&frame).unwrap(), fixture);
    }
}
