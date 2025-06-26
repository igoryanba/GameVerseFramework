//! Error types for GameVerse native function wrappers

use thiserror::Error;

/// Errors that can occur when calling native functions
#[derive(Error, Debug, Clone)]
pub enum NativeError {
    #[error("Invalid player ID: {0:?}")]
    InvalidPlayer(super::types::PlayerId),
    
    #[error("Invalid ped entity: {0:?}")]
    InvalidPed(super::types::PedEntity),
    
    #[error("Invalid vehicle entity: {0:?}")]
    InvalidVehicle(super::types::VehicleEntity),
    
    #[error("Invalid entity ID: {0:?}")]
    InvalidEntity(super::types::EntityId),
    
    #[error("Invalid object entity: {0:?}")]
    InvalidObject(super::types::ObjectEntity),
    
    #[error("Invalid blip ID: {0:?}")]
    InvalidBlip(super::types::BlipId),
    
    #[error("Invalid camera ID: {0:?}")]
    InvalidCamera(super::types::CameraId),
    
    #[error("Invalid position: {0:?}")]
    InvalidPosition(nalgebra::Vector3<f32>),
    
    #[error("Native function returned null")]
    NullReturn,
    
    #[error("Native function call failed: {0}")]
    CallFailed(String),
    
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Feature not available on this platform")]
    PlatformNotSupported,
}

/// Result type for native function calls
pub type NativeResult<T> = Result<T, NativeError>;

/// Validation helpers
pub mod validation {
    use super::*;
    use crate::types::*;
    
    /// Validate player ID
    pub fn validate_player_id(player_id: PlayerId) -> NativeResult<()> {
        if player_id.0 < 0 || player_id.0 > 255 {
            Err(NativeError::InvalidPlayer(player_id))
        } else {
            Ok(())
        }
    }
    
    /// Validate entity handle
    pub fn validate_entity(entity: EntityId) -> NativeResult<()> {
        if entity.0 == 0 {
            Err(NativeError::InvalidEntity(entity))
        } else {
            Ok(())
        }
    }
    
    /// Validate position coordinates
    pub fn validate_position(pos: nalgebra::Vector3<f32>) -> NativeResult<()> {
        if pos.x.is_finite() && pos.y.is_finite() && pos.z.is_finite() {
            Ok(())
        } else {
            Err(NativeError::InvalidPosition(pos))
        }
    }
}
