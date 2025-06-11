//! PLAYER native functions
//!
//! Player-related native functions

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// Gets the ped entity for the specified player

pub fn _safe(playerId: ) -> NativeResult<> {
    
    debug!("Calling native function: GET_PLAYER_PED");
    
    // SAFETY: All parameters have been validated
    let result = unsafe {
        crate::raw::GET_PLAYER_PED(playerId.0)
    };
    
    
    Ok((result))
}

/// Raw native function: GET_PLAYER_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_safe` instead.
pub unsafe fn _raw(playerId: ) ->  {
    crate::raw::GET_PLAYER_PED(playerId)
}

/// Gets the name of the specified player

pub fn _safe(playerId: ) -> NativeResult<> {
    
    debug!("Calling native function: GET_PLAYER_NAME");
    
    // SAFETY: All parameters have been validated
    let result = unsafe {
        crate::raw::GET_PLAYER_NAME(playerId.0)
    };
    
    
    Ok((result))
}

/// Raw native function: GET_PLAYER_NAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_safe` instead.
pub unsafe fn _raw(playerId: ) ->  {
    crate::raw::GET_PLAYER_NAME(playerId)
}

