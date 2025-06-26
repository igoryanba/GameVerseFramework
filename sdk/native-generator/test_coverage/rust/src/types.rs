//! GameVerse native function types
//!
//! Type-safe wrappers for game entities and values

use serde::{Deserialize, Serialize};
use nalgebra::Vector3;

/// Player identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub i32);

/// Ped entity handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PedEntity(pub i32);

/// Vehicle entity handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VehicleEntity(pub i32);

/// Generic entity handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub i32);

/// Object entity handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectEntity(pub i32);

/// Blip identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlipId(pub i32);

/// Camera identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CameraId(pub i32);

// Conversion traits for type safety
impl From<PedEntity> for EntityId {
    fn from(ped: PedEntity) -> Self {
        EntityId(ped.0)
    }
}

impl From<VehicleEntity> for EntityId {
    fn from(vehicle: VehicleEntity) -> Self {
        EntityId(vehicle.0)
    }
}

impl From<ObjectEntity> for EntityId {
    fn from(object: ObjectEntity) -> Self {
        EntityId(object.0)
    }
}

impl TryFrom<EntityId> for PedEntity {
    type Error = crate::errors::NativeError;
    
    fn try_from(entity: EntityId) -> Result<Self, Self::Error> {
        // TODO: Add entity type validation
        Ok(PedEntity(entity.0))
    }
}

impl TryFrom<EntityId> for VehicleEntity {
    type Error = crate::errors::NativeError;
    
    fn try_from(entity: EntityId) -> Result<Self, Self::Error> {
        // TODO: Add entity type validation
        Ok(VehicleEntity(entity.0))
    }
}

/// 3D position vector
pub type Position = Vector3<f32>;

/// 3D rotation vector (in degrees)
pub type Rotation = Vector3<f32>;

/// RGBA color value
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }
}


// ===== Auto-generated pseudo-types =====
