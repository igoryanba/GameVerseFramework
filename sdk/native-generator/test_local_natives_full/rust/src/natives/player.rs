//! PLAYER native functions
//!
//! Functions for the player category

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
NativeDB Added Parameter 2: Any p1
```

pub fn _special_ability_activate_safe(player: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let player_any_str = serde_json::to_string(&player)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "player", e)))?;
    let player_any_str_cstr = std::ffi::CString::new(player_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "player", e)))?;
    
    debug!("Calling native function: _SPECIAL_ABILITY_ACTIVATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SPECIAL_ABILITY_ACTIVATE(crate::utils::any_to_c_void_ptr(player))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SPECIAL_ABILITY_ACTIVATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_special_ability_activate_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _special_ability_activate_raw(player: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SPECIAL_ABILITY_ACTIVATE(player)
}

/// ### Warning
This native will return `0` if the last vehicle the player was in was destroyed.

pub fn get_players_last_vehicle_safe() -> NativeResult<Vehicle> {
    
    debug!("Calling native function: GET_PLAYERS_LAST_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYERS_LAST_VEHICLE()
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: GET_PLAYERS_LAST_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_players_last_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_players_last_vehicle_raw() -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYERS_LAST_VEHICLE()
}

/// Gets the ped for a specified player index.

pub fn get_player_ped_safe(playerId: Player) -> NativeResult<Ped> {
    
    debug!("Calling native function: GET_PLAYER_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_PED(playerId)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: GET_PLAYER_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_ped_raw(playerId: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_PED(playerId)
}

/// ## Parameters
* **p0**:

pub fn _0x6e4361ff3e8cd7ca_safe(p0: serde_json::Value) -> NativeResult<serde_json::Value> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x6E4361FF3E8CD7CA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x6E4361FF3E8CD7CA(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0x6E4361FF3E8CD7CA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x6e4361ff3e8cd7ca_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x6e4361ff3e8cd7ca_raw(p0: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x6E4361FF3E8CD7CA(p0)
}

/// ## Parameters
* **player**: 
* **multiplier**:

pub fn set_player_noise_multiplier_safe(player: Player, multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_NOISE_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_NOISE_MULTIPLIER(player, multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_NOISE_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_noise_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_noise_multiplier_raw(player: i32, multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_NOISE_MULTIPLIER(player, multiplier)
}

/// ## Parameters
* **player**: 
* **distance**:

pub fn _set_player_fall_distance_safe(player: Player, distance: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PLAYER_FALL_DISTANCE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PLAYER_FALL_DISTANCE(player, distance)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PLAYER_FALL_DISTANCE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_player_fall_distance_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_player_fall_distance_raw(player: i32, distance: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PLAYER_FALL_DISTANCE(player, distance)
}

/// ```
Achievements from 0-57
more achievements came with update 1.29 (freemode events update), I'd say that they now go to 60, but I'll need to check.
```

pub fn give_achievement_to_player_safe(achievement: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: GIVE_ACHIEVEMENT_TO_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GIVE_ACHIEVEMENT_TO_PLAYER(achievement)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GIVE_ACHIEVEMENT_TO_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `give_achievement_to_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn give_achievement_to_player_raw(achievement: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GIVE_ACHIEVEMENT_TO_PLAYER(achievement)
}

/// ```
This can be between 1.0f - 14.9f   
You can change the max in IDA from 15.0. I say 15.0 as the function blrs if what you input is greater than or equal to 15.0 hence why it's 14.9 max default.  
On PC the multiplier can be between 0.0f and 50.0f (inclusive).  
```

pub fn set_air_drag_multiplier_for_players_vehicle_safe(player: Player, multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_AIR_DRAG_MULTIPLIER_FOR_PLAYERS_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_AIR_DRAG_MULTIPLIER_FOR_PLAYERS_VEHICLE(player, multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_AIR_DRAG_MULTIPLIER_FOR_PLAYERS_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_air_drag_multiplier_for_players_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_air_drag_multiplier_for_players_vehicle_raw(player: i32, multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_AIR_DRAG_MULTIPLIER_FOR_PLAYERS_VEHICLE(player, multiplier)
}

/// ## Parameters
* **player**: 
* **state**:

pub fn set_player_bluetooth_state_safe() -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_BLUETOOTH_STATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_BLUETOOTH_STATE()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_BLUETOOTH_STATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_bluetooth_state_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_bluetooth_state_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_BLUETOOTH_STATE()
}

/// ## Parameters
* **player**: 
* **modifier**:

pub fn set_player_weapon_defense_modifier_safe(player: Player, modifier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_WEAPON_DEFENSE_MODIFIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_WEAPON_DEFENSE_MODIFIER(player, modifier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_WEAPON_DEFENSE_MODIFIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_weapon_defense_modifier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_weapon_defense_modifier_raw(player: i32, modifier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_WEAPON_DEFENSE_MODIFIER(player, modifier)
}

/// ```
Default is 100. Use player id and not ped id. For instance: PLAYER::SET_PLAYER_MAX_ARMOUR(PLAYER::PLAYER_ID(), 100); // main_persistent.ct4  
```

pub fn set_player_max_armour_safe(player: Player, value: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_MAX_ARMOUR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_MAX_ARMOUR(player, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_MAX_ARMOUR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_max_armour_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_max_armour_raw(player: i32, value: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_MAX_ARMOUR(player, value)
}

/// ```
NativeDB Added Parameter 2: Any p1
```

pub fn special_ability_unlock_safe(playerModel: u32) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_ABILITY_UNLOCK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_ABILITY_UNLOCK(playerModel)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_ABILITY_UNLOCK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_ability_unlock_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_ability_unlock_raw(playerModel: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_ABILITY_UNLOCK(playerModel)
}

/// ## Parameters
* **player**: 
* **entity**:

pub fn is_player_targetting_entity_safe(player: Player, entity: Entity) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_TARGETTING_ENTITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_TARGETTING_ENTITY(player, entity)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_TARGETTING_ENTITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_targetting_entity_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_targetting_entity_raw(player: i32, entity: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_TARGETTING_ENTITY(player, entity)
}

/// Sets whether the player is able to do drive-bys in vehicle (shooting & aiming in vehicles), this also includes middle finger taunts.

This is a toggle, it does not have to be ran every frame.

pub fn set_player_can_do_drive_by_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_CAN_DO_DRIVE_BY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_CAN_DO_DRIVE_BY(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_CAN_DO_DRIVE_BY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_can_do_drive_by_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_can_do_drive_by_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_CAN_DO_DRIVE_BY(player, toggle)
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn set_disable_ambient_melee_move_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_DISABLE_AMBIENT_MELEE_MOVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DISABLE_AMBIENT_MELEE_MOVE(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DISABLE_AMBIENT_MELEE_MOVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_disable_ambient_melee_move_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_disable_ambient_melee_move_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DISABLE_AMBIENT_MELEE_MOVE(player, toggle)
}

/// ```
Gets a value indicating whether the specified player is currently aiming freely.  
```

pub fn is_player_free_aiming_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_FREE_AIMING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_FREE_AIMING(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_FREE_AIMING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_free_aiming_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_free_aiming_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_FREE_AIMING(player)
}

/// ```
Tints:  
None = -1,  
Rainbow = 0,  
Red = 1,  
SeasideStripes = 2,  
WidowMaker = 3,  
Patriot = 4,  
Blue = 5,  
Black = 6,  
Hornet = 7,  
AirFocce = 8,  
Desert = 9,  
Shadow = 10,  
HighAltitude = 11,  
Airbone = 12,  
Sunrise = 13,  
```

pub fn get_player_reserve_parachute_tint_index_safe(player: Player, index: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_PLAYER_RESERVE_PARACHUTE_TINT_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_RESERVE_PARACHUTE_TINT_INDEX(player, index)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_PLAYER_RESERVE_PARACHUTE_TINT_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_reserve_parachute_tint_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_reserve_parachute_tint_index_raw(player: i32, index: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_RESERVE_PARACHUTE_TINT_INDEX(player, index)
}

/// ```
NativeDB Added Parameter 2: Any p1
```

pub fn special_ability_lock_safe(playerModel: u32) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_ABILITY_LOCK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_ABILITY_LOCK(playerModel)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_ABILITY_LOCK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_ability_lock_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_ability_lock_raw(playerModel: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_ABILITY_LOCK(playerModel)
}

/// ```
Returns the group ID the player is member of.  
```

pub fn get_player_group_safe(player: Player) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PLAYER_GROUP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_GROUP(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PLAYER_GROUP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_group_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_group_raw(player: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_GROUP(player)
}

/// ## Parameters
* **player**:

pub fn get_time_since_player_drove_on_pavement_safe(player: Player) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_TIME_SINCE_PLAYER_DROVE_ON_PAVEMENT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_TIME_SINCE_PLAYER_DROVE_ON_PAVEMENT(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_TIME_SINCE_PLAYER_DROVE_ON_PAVEMENT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_time_since_player_drove_on_pavement_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_time_since_player_drove_on_pavement_raw(player: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_TIME_SINCE_PLAYER_DROVE_ON_PAVEMENT(player)
}

/// Returns the players name from a specified player index

pub fn get_player_name_safe(player: Player) -> NativeResult<String> {
    
    debug!("Calling native function: GET_PLAYER_NAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_NAME(player)
    };
    
    
    Ok(crate::utils::c_string_to_rust_string(result))
}

/// Raw native function: GET_PLAYER_NAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_name_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_name_raw(player: i32) -> *const std::os::raw::c_char {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_NAME(player)
}

/// ## Parameters
* **p0**:

pub fn _0x7e07c78925d5fd96_safe(p0: serde_json::Value) -> NativeResult<serde_json::Value> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x7E07C78925D5FD96");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x7E07C78925D5FD96(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0x7E07C78925D5FD96
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x7e07c78925d5fd96_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x7e07c78925d5fd96_raw(p0: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x7E07C78925D5FD96(p0)
}

/// ```c
enum eViolationType {
  // Checks if the player is driving on pedestrians walk ways
  VT_PAVED_PEDESTRIAN_AREAS = 0,
  // Checks if the player is running through red lights
  // This takes some time to return true.
  VT_RUNNING_REDS = 1,
  // checks if the player is driving on the wrong side of the road
  VT_AGAINST_TRAFFIC = 2
};
```

Used solely in "Al Di Napoli" with type 2 for a voiceline.

pub fn _is_player_driving_dangerously_safe(player: Player, type: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PLAYER_DRIVING_DANGEROUSLY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PLAYER_DRIVING_DANGEROUSLY(player, type)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PLAYER_DRIVING_DANGEROUSLY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_player_driving_dangerously_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_player_driving_dangerously_raw(player: i32, type: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PLAYER_DRIVING_DANGEROUSLY(player, type)
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn _set_player_invincible_keep_ragdoll_enabled_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PLAYER_INVINCIBLE_KEEP_RAGDOLL_ENABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PLAYER_INVINCIBLE_KEEP_RAGDOLL_ENABLED(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PLAYER_INVINCIBLE_KEEP_RAGDOLL_ENABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_player_invincible_keep_ragdoll_enabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_player_invincible_keep_ragdoll_enabled_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PLAYER_INVINCIBLE_KEEP_RAGDOLL_ENABLED(player, toggle)
}

/// ```
Returns the Player's Invincible status.  
This function will always return false if 0x733A643B5B0C53C1 is used to set the invincibility status. To always get the correct result, use this:  
	bool IsPlayerInvincible(Player player)  
	{  
auto addr = getScriptHandleBaseAddress(GET_PLAYER_PED(player));	  
if (addr)  
{  
	DWORD flag = *(DWORD *)(addr + 0x188);  
	return ((flag & (1 << 8)) != 0) || ((flag & (1 << 9)) != 0);  
}  
return false;  
	}  
============================================================  
This has bothered me for too long, whoever may come across this, where did anyone ever come up with this made up hash? 0x733A643B5B0C53C1 I've looked all over old hash list, and this nativedb I can not find that PC hash anywhere. What native name is it now or was it?  
```

pub fn get_player_invincible_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_PLAYER_INVINCIBLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_INVINCIBLE(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PLAYER_INVINCIBLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_invincible_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_invincible_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_INVINCIBLE(player)
}

/// ```
NativeDB Added Parameter 3: BOOL p2
```

pub fn set_player_melee_weapon_damage_modifier_safe(player: Player, modifier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_MELEE_WEAPON_DAMAGE_MODIFIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_MELEE_WEAPON_DAMAGE_MODIFIER(player, modifier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_MELEE_WEAPON_DAMAGE_MODIFIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_melee_weapon_damage_modifier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_melee_weapon_damage_modifier_raw(player: i32, modifier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_MELEE_WEAPON_DAMAGE_MODIFIER(player, modifier)
}

/// ## Parameters
* **multiplier**:

pub fn set_wanted_level_multiplier_safe(multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_WANTED_LEVEL_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_WANTED_LEVEL_MULTIPLIER(multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_WANTED_LEVEL_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_wanted_level_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_wanted_level_multiplier_raw(multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_WANTED_LEVEL_MULTIPLIER(multiplier)
}

/// ```
Does exactly the same thing as PLAYER_ID()  
```

pub fn network_player_id_to_int_safe() -> NativeResult<i64> {
    
    debug!("Calling native function: NETWORK_PLAYER_ID_TO_INT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::NETWORK_PLAYER_ID_TO_INT()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: NETWORK_PLAYER_ID_TO_INT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `network_player_id_to_int_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn network_player_id_to_int_raw() -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::NETWORK_PLAYER_ID_TO_INT()
}

/// ```
Returns the time since the character was arrested in (ms) milliseconds.  
example  
var time = Function.call<int>(Hash.GET_TIME_SINCE_LAST_ARREST();  
UI.DrawSubtitle(time.ToString());  
if player has not been arrested, the int returned will be -1.  
```

pub fn get_time_since_last_arrest_safe() -> NativeResult<i64> {
    
    debug!("Calling native function: GET_TIME_SINCE_LAST_ARREST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_TIME_SINCE_LAST_ARREST()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_TIME_SINCE_LAST_ARREST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_time_since_last_arrest_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_time_since_last_arrest_raw() -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_TIME_SINCE_LAST_ARREST()
}

/// ASSISTED_MOVEMENT_CLOSE_ROUTE native function

pub fn assisted_movement_close_route_safe() -> NativeResult<()> {
    
    debug!("Calling native function: ASSISTED_MOVEMENT_CLOSE_ROUTE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ASSISTED_MOVEMENT_CLOSE_ROUTE()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ASSISTED_MOVEMENT_CLOSE_ROUTE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `assisted_movement_close_route_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn assisted_movement_close_route_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ASSISTED_MOVEMENT_CLOSE_ROUTE()
}

/// ## Parameters
* **player**:

pub fn get_player_max_armour_safe(player: Player) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PLAYER_MAX_ARMOUR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_MAX_ARMOUR(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PLAYER_MAX_ARMOUR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_max_armour_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_max_armour_raw(player: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_MAX_ARMOUR(player)
}

/// ## Parameters
* **player**:

pub fn get_player_wanted_centre_position_safe(player: Player) -> NativeResult<Vector3> {
    
    debug!("Calling native function: GET_PLAYER_WANTED_CENTRE_POSITION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_WANTED_CENTRE_POSITION(player)
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: GET_PLAYER_WANTED_CENTRE_POSITION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_wanted_centre_position_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_wanted_centre_position_raw(player: i32) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_WANTED_CENTRE_POSITION(player)
}

/// ADD_*

```
NativeDB Introduced: v1868
```

pub fn _0x9097eb6d4bb9a12a_safe(player: Player, entity: Entity) -> NativeResult<()> {
    
    debug!("Calling native function: _0x9097EB6D4BB9A12A");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x9097EB6D4BB9A12A(player, entity)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x9097EB6D4BB9A12A
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x9097eb6d4bb9a12a_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x9097eb6d4bb9a12a_raw(player: i32, entity: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x9097EB6D4BB9A12A(player, entity)
}

/// ```
Returns true if an unk value is greater than 0.0f  
```

pub fn is_player_battle_aware_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_BATTLE_AWARE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_BATTLE_AWARE(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_BATTLE_AWARE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_battle_aware_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_battle_aware_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_BATTLE_AWARE(player)
}

/// ## Parameters
* **player**:

pub fn is_player_targetting_anything_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_TARGETTING_ANYTHING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_TARGETTING_ANYTHING(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_TARGETTING_ANYTHING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_targetting_anything_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_targetting_anything_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_TARGETTING_ANYTHING(player)
}

/// ## Parameters
* **player**:

pub fn clear_player_parachute_pack_model_override_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PLAYER_PARACHUTE_PACK_MODEL_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PLAYER_PARACHUTE_PACK_MODEL_OVERRIDE(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PLAYER_PARACHUTE_PACK_MODEL_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_player_parachute_pack_model_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_player_parachute_pack_model_override_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PLAYER_PARACHUTE_PACK_MODEL_OVERRIDE(player)
}

/// ## Parameters
* **player**: 
* **r**: 
* **g**: 
* **b**:

pub fn get_player_parachute_smoke_trail_color_safe(player: Player, r: *mut i64, g: *mut i64, b: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_PLAYER_PARACHUTE_SMOKE_TRAIL_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_PARACHUTE_SMOKE_TRAIL_COLOR(player, r, g, b)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_PLAYER_PARACHUTE_SMOKE_TRAIL_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_parachute_smoke_trail_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_parachute_smoke_trail_color_raw(player: i32, r: *mut i64, g: *mut i64, b: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_PARACHUTE_SMOKE_TRAIL_COLOR(player, r, g, b)
}

/// ## Parameters
* **player**:

pub fn get_player_has_reserve_parachute_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_PLAYER_HAS_RESERVE_PARACHUTE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_HAS_RESERVE_PARACHUTE(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PLAYER_HAS_RESERVE_PARACHUTE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_has_reserve_parachute_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_has_reserve_parachute_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_HAS_RESERVE_PARACHUTE(player)
}

/// ## Parameters
* **player**:

pub fn get_time_since_player_hit_ped_safe(player: Player) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_TIME_SINCE_PLAYER_HIT_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_TIME_SINCE_PLAYER_HIT_PED(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_TIME_SINCE_PLAYER_HIT_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_time_since_player_hit_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_time_since_player_hit_ped_raw(player: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_TIME_SINCE_PLAYER_HIT_PED(player)
}

/// Returns the player index for the local player.

pub fn player_id_safe() -> NativeResult<Player> {
    
    debug!("Calling native function: PLAYER_ID");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::PLAYER_ID()
    };
    
    
    Ok(Ok(Player(result)))
}

/// Raw native function: PLAYER_ID
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `player_id_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn player_id_raw() -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::PLAYER_ID()
}

/// ```
Returns TRUE if it found an entity in your crosshair within range of your weapon. Assigns the handle of the target to the *entity that you pass it.  
Returns false if no entity found.  
```

pub fn get_entity_player_is_free_aiming_at_safe(player: Player, entity: *mut i32) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_ENTITY_PLAYER_IS_FREE_AIMING_AT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_ENTITY_PLAYER_IS_FREE_AIMING_AT(player, entity)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_ENTITY_PLAYER_IS_FREE_AIMING_AT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_entity_player_is_free_aiming_at_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_entity_player_is_free_aiming_at_raw(player: i32, entity: *mut i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_ENTITY_PLAYER_IS_FREE_AIMING_AT(player, entity)
}

/// ## Parameters
* **player**:

pub fn has_player_damaged_at_least_one_ped_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_PLAYER_DAMAGED_AT_LEAST_ONE_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_PLAYER_DAMAGED_AT_LEAST_ONE_PED(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_PLAYER_DAMAGED_AT_LEAST_ONE_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_player_damaged_at_least_one_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_player_damaged_at_least_one_ped_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_PLAYER_DAMAGED_AT_LEAST_ONE_PED(player)
}

/// ## Return value

pub fn get_cause_of_most_recent_force_cleanup_safe() -> NativeResult<i64> {
    
    debug!("Calling native function: GET_CAUSE_OF_MOST_RECENT_FORCE_CLEANUP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_CAUSE_OF_MOST_RECENT_FORCE_CLEANUP()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_CAUSE_OF_MOST_RECENT_FORCE_CLEANUP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_cause_of_most_recent_force_cleanup_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_cause_of_most_recent_force_cleanup_raw() -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_CAUSE_OF_MOST_RECENT_FORCE_CLEANUP()
}

/// Establishes a reset flag to prevent the player from entering any vehicle. Not that this native must be called every frame.

pub fn set_player_may_not_enter_any_vehicle_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_MAY_NOT_ENTER_ANY_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_MAY_NOT_ENTER_ANY_VEHICLE(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_MAY_NOT_ENTER_ANY_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_may_not_enter_any_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_may_not_enter_any_vehicle_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_MAY_NOT_ENTER_ANY_VEHICLE(player)
}

/// ```
Affects the range of auto aim target.  
```

pub fn set_player_lockon_range_override_safe(player: Player, range: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_LOCKON_RANGE_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_LOCKON_RANGE_OVERRIDE(player, range)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_LOCKON_RANGE_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_lockon_range_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_lockon_range_override_raw(player: i32, range: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_LOCKON_RANGE_OVERRIDE(player, range)
}

/// ```
NativeDB Introduced: v2372
```

pub fn _clear_player_reserve_parachute_model_override_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: _CLEAR_PLAYER_RESERVE_PARACHUTE_MODEL_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_CLEAR_PLAYER_RESERVE_PARACHUTE_MODEL_OVERRIDE(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _CLEAR_PLAYER_RESERVE_PARACHUTE_MODEL_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_clear_player_reserve_parachute_model_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _clear_player_reserve_parachute_model_override_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_CLEAR_PLAYER_RESERVE_PARACHUTE_MODEL_OVERRIDE(player)
}

/// Make the player impervious to all forms of damage.

pub fn set_player_invincible_safe(player: Player, bInvincible: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_INVINCIBLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_INVINCIBLE(player, bInvincible)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_INVINCIBLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_invincible_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_invincible_raw(player: i32, bInvincible: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_INVINCIBLE(player, bInvincible)
}

/// ```
6 matches across 4 scripts. 5 occurrences were 240. The other was 255.  
```

pub fn set_player_cloth_lock_counter_safe(value: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_CLOTH_LOCK_COUNTER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_CLOTH_LOCK_COUNTER(value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_CLOTH_LOCK_COUNTER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_cloth_lock_counter_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_cloth_lock_counter_raw(value: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_CLOTH_LOCK_COUNTER(value)
}

/// ```
Also known as _RECHARGE_SPECIAL_ABILITY
```

```
NativeDB Added Parameter 3: Any p2
```

pub fn special_ability_fill_meter_safe(player: Player, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_ABILITY_FILL_METER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_ABILITY_FILL_METER(player, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_ABILITY_FILL_METER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_ability_fill_meter_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_ability_fill_meter_raw(player: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_ABILITY_FILL_METER(player, p1)
}

/// ```
Seems to only appear in scripts used in Singleplayer.  
Always used like this in scripts  
PLAYER::_BC9490CA15AEA8FB(PLAYER::PLAYER_ID());  
```

pub fn _0xbc9490ca15aea8fb_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: _0xBC9490CA15AEA8FB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xBC9490CA15AEA8FB(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xBC9490CA15AEA8FB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xbc9490ca15aea8fb_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xbc9490ca15aea8fb_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xBC9490CA15AEA8FB(player)
}

/// ## Parameters
* **player**:

pub fn clear_player_parachute_model_override_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PLAYER_PARACHUTE_MODEL_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PLAYER_PARACHUTE_MODEL_OVERRIDE(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PLAYER_PARACHUTE_MODEL_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_player_parachute_model_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_player_parachute_model_override_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PLAYER_PARACHUTE_MODEL_OVERRIDE(player)
}

/// For Steam.
Does nothing and always returns false in the retail version of the game.

pub fn _set_achievement_progress_safe(achievement: i64, progress: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _SET_ACHIEVEMENT_PROGRESS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_ACHIEVEMENT_PROGRESS(achievement, progress)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _SET_ACHIEVEMENT_PROGRESS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_achievement_progress_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_achievement_progress_raw(achievement: i64, progress: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_ACHIEVEMENT_PROGRESS(achievement, progress)
}

/// ## Parameters
* **player**:

pub fn report_police_spotted_player_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: REPORT_POLICE_SPOTTED_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REPORT_POLICE_SPOTTED_PLAYER(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REPORT_POLICE_SPOTTED_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `report_police_spotted_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn report_police_spotted_player_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REPORT_POLICE_SPOTTED_PLAYER(player)
}

/// REMOVE_*

```
NativeDB Introduced: v1868
```

pub fn _0x9f260bfb59adbca3_safe(player: Player, entity: Entity) -> NativeResult<()> {
    
    debug!("Calling native function: _0x9F260BFB59ADBCA3");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x9F260BFB59ADBCA3(player, entity)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x9F260BFB59ADBCA3
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x9f260bfb59adbca3_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x9f260bfb59adbca3_raw(player: i32, entity: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x9F260BFB59ADBCA3(player, entity)
}

/// ```
NativeDB Introduced: v323
```

pub fn reset_world_boundary_for_player_safe() -> NativeResult<()> {
    
    debug!("Calling native function: RESET_WORLD_BOUNDARY_FOR_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_WORLD_BOUNDARY_FOR_PLAYER()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_WORLD_BOUNDARY_FOR_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_world_boundary_for_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_world_boundary_for_player_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_WORLD_BOUNDARY_FOR_PLAYER()
}

/// ```
Sets whether this player can be hassled by gangs.  
```

pub fn set_player_can_be_hassled_by_gangs_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_CAN_BE_HASSLED_BY_GANGS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_CAN_BE_HASSLED_BY_GANGS(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_CAN_BE_HASSLED_BY_GANGS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_can_be_hassled_by_gangs_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_can_be_hassled_by_gangs_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_CAN_BE_HASSLED_BY_GANGS(player, toggle)
}

/// ```
PLAYER::REPORT_CRIME(PLAYER::PLAYER_ID(), 37, PLAYER::GET_WANTED_LEVEL_THRESHOLD(1));  
From am_armybase.ysc.c4:  
PLAYER::REPORT_CRIME(PLAYER::PLAYER_ID(4), 36, PLAYER::GET_WANTED_LEVEL_THRESHOLD(4));  
-----  
This was taken from the GTAV.exe v1.334. The function is called sub_140592CE8. For a full decompilation of the function, see here: pastebin.com/09qSMsN7   
-----  
crimeType:  
1: Firearms possession  
2: Person running a red light ("5-0-5")  
3: Reckless driver  
4: Speeding vehicle (a "5-10")  
5: Traffic violation (a "5-0-5")  
6: Motorcycle rider without a helmet  
7: Vehicle theft (a "5-0-3")  
8: Grand Theft Auto  
9: ???  
10: ???  
11: Assault on a civilian (a "2-40")  
12: Assault on an officer  
13: Assault with a deadly weapon (a "2-45")  
14: Officer shot (a "2-45")  
15: Pedestrian struck by a vehicle  
16: Officer struck by a vehicle  
17: Helicopter down (an "AC"?)  
18: Civilian on fire (a "2-40")  
19: Officer set on fire (a "10-99")  
20: Car on fire  
21: Air unit down (an "AC"?)  
22: An explosion (a "9-96")  
23: A stabbing (a "2-45") (also something else I couldn't understand)  
24: Officer stabbed (also something else I couldn't understand)  
25: Attack on a vehicle ("MDV"?)  
26: Damage to property  
27: Suspect threatening officer with a firearm  
28: Shots fired  
29: ???  
30: ???  
31: ???  
32: ???  
33: ???  
34: A "2-45"  
35: ???  
36: A "9-25"  
37: ???  
38: ???  
39: ???  
40: ???  
41: ???  
42: ???  
43: Possible disturbance  
44: Civilian in need of assistance  
45: ???  
46: ???  
```

pub fn report_crime_safe(player: Player, crimeType: i64, wantedLvlThresh: i64) -> NativeResult<()> {
    
    debug!("Calling native function: REPORT_CRIME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REPORT_CRIME(player, crimeType, wantedLvlThresh)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REPORT_CRIME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `report_crime_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn report_crime_raw(player: i32, crimeType: i64, wantedLvlThresh: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REPORT_CRIME(player, crimeType, wantedLvlThresh)
}

/// ```
example:  
PLAYER::SET_PLAYER_PARACHUTE_MODEL_OVERRIDE(PLAYER::PLAYER_ID(), 0x73268708);  
```

pub fn set_player_parachute_model_override_safe(player: Player, model: u32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_PARACHUTE_MODEL_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_PARACHUTE_MODEL_OVERRIDE(player, model)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_PARACHUTE_MODEL_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_parachute_model_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_parachute_model_override_raw(player: i32, model: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_PARACHUTE_MODEL_OVERRIDE(player, model)
}

/// ## Parameters
* **player**: 
* **ped**:

pub fn can_ped_hear_player_safe(player: Player, ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: CAN_PED_HEAR_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CAN_PED_HEAR_PLAYER(player, ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CAN_PED_HEAR_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `can_ped_hear_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn can_ped_hear_player_raw(player: i32, ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CAN_PED_HEAR_PLAYER(player, ped)
}

/// ## Parameters
* **player**:

pub fn set_player_has_reserve_parachute_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_HAS_RESERVE_PARACHUTE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_HAS_RESERVE_PARACHUTE(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_HAS_RESERVE_PARACHUTE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_has_reserve_parachute_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_has_reserve_parachute_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_HAS_RESERVE_PARACHUTE(player)
}

/// ## Parameters
* **player**:

pub fn get_player_sprint_stamina_remaining_safe(player: Player) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_PLAYER_SPRINT_STAMINA_REMAINING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_SPRINT_STAMINA_REMAINING(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PLAYER_SPRINT_STAMINA_REMAINING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_sprint_stamina_remaining_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_sprint_stamina_remaining_raw(player: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_SPRINT_STAMINA_REMAINING(player)
}

/// ```
NativeDB Added Parameter 2: Any p1
```

pub fn _special_ability_deplete_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _SPECIAL_ABILITY_DEPLETE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SPECIAL_ABILITY_DEPLETE(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SPECIAL_ABILITY_DEPLETE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_special_ability_deplete_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _special_ability_deplete_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SPECIAL_ABILITY_DEPLETE(p0)
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn set_everyone_ignore_player_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_EVERYONE_IGNORE_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_EVERYONE_IGNORE_PLAYER(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_EVERYONE_IGNORE_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_everyone_ignore_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_everyone_ignore_player_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_EVERYONE_IGNORE_PLAYER(player, toggle)
}

/// ```
NativeDB Introduced: v2372
```

pub fn _set_player_reserve_parachute_model_override_safe(player: Player, model: u32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PLAYER_RESERVE_PARACHUTE_MODEL_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PLAYER_RESERVE_PARACHUTE_MODEL_OVERRIDE(player, model)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PLAYER_RESERVE_PARACHUTE_MODEL_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_player_reserve_parachute_model_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_player_reserve_parachute_model_override_raw(player: i32, model: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PLAYER_RESERVE_PARACHUTE_MODEL_OVERRIDE(player, model)
}

/// ```
Remnant from GTA IV. Does nothing in GTA V.
```

pub fn get_wanted_level_radius_safe(player: Player) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_WANTED_LEVEL_RADIUS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_WANTED_LEVEL_RADIUS(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_WANTED_LEVEL_RADIUS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_wanted_level_radius_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_wanted_level_radius_raw(player: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_WANTED_LEVEL_RADIUS(player)
}

/// ## Parameters
* **player**: 
* **value**:

pub fn set_player_stealth_perception_modifier_safe(player: Player, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_STEALTH_PERCEPTION_MODIFIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_STEALTH_PERCEPTION_MODIFIER(player, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_STEALTH_PERCEPTION_MODIFIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_stealth_perception_modifier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_stealth_perception_modifier_raw(player: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_STEALTH_PERCEPTION_MODIFIER(player, value)
}

/// Set the player's current team.

pub fn set_player_team_safe(player: Player, team: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_TEAM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_TEAM(player, team)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_TEAM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_team_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_team_raw(player: i32, team: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_TEAM(player, team)
}

/// ```
p1 was always true.
```

```
NativeDB Added Parameter 3: Any p2
```

pub fn special_ability_deplete_meter_safe(player: Player, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_ABILITY_DEPLETE_METER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_ABILITY_DEPLETE_METER(player, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_ABILITY_DEPLETE_METER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_ability_deplete_meter_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_ability_deplete_meter_raw(player: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_ABILITY_DEPLETE_METER(player, p1)
}

/// Set the model for a specific Player. Note that this will destroy the current Ped for the Player and create a new one, any reference to the old ped will be invalid after calling this.

As per usual, make sure to request the model first and wait until it has loaded.

pub fn set_player_model_safe(player: Player, model: u32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_MODEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_MODEL(player, model)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_MODEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_model_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_model_raw(player: i32, model: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_MODEL(player, model)
}

/// ## Parameters
* **player**:

pub fn get_player_wanted_level_safe(player: Player) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PLAYER_WANTED_LEVEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_WANTED_LEVEL(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PLAYER_WANTED_LEVEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_wanted_level_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_wanted_level_raw(player: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_WANTED_LEVEL(player)
}

/// ```
This has been found in use in the decompiled files.  
```

pub fn _0xad73ce5a09e42d12_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: _0xAD73CE5A09E42D12");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xAD73CE5A09E42D12(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xAD73CE5A09E42D12
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xad73ce5a09e42d12_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xad73ce5a09e42d12_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xAD73CE5A09E42D12(player)
}

/// ```
NativeDB Added Parameter 3: Any p2
```

pub fn _set_special_ability_safe(player: Player, p1: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_SPECIAL_ABILITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_SPECIAL_ABILITY(player, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_SPECIAL_ABILITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_special_ability_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_special_ability_raw(player: i32, p1: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_SPECIAL_ABILITY(player, p1)
}

/// ```
Sets your targeting mode.
0 = Assisted Aim - Full
1 = Assisted Aim - Partial
2 = Free Aim - Assisted
3 = Free Aim
```

pub fn set_player_targeting_mode_safe(targetMode: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_TARGETING_MODE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_TARGETING_MODE(targetMode)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_TARGETING_MODE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_targeting_mode_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_targeting_mode_raw(targetMode: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_TARGETING_MODE(targetMode)
}

/// ```
Used with radios:
void sub_cf383(auto _a0) {
    if ((a_0)==1) {
        if (MISC::IS_BIT_SET((g_240005._f1), 3)) {
            PLAYER::_2F7CEB6520288061(0);
            AUDIO::SET_AUDIO_FLAG("AllowRadioDuringSwitch", 0);
            AUDIO::SET_MOBILE_PHONE_RADIO_STATE(0);
            AUDIO::SET_AUDIO_FLAG("MobileRadioInGame", 0);
        }
        sub_cf3f6(1);
    } else {
        if (MISC::IS_BIT_SET((g_240005._f1), 3)) {
            PLAYER::_2F7CEB6520288061(1);
            AUDIO::SET_AUDIO_FLAG("AllowRadioDuringSwitch", 1);
            AUDIO::SET_MOBILE_PHONE_RADIO_STATE(1);
            AUDIO::SET_AUDIO_FLAG("MobileRadioInGame", 1);
        }
        sub_cf3f6(0);
    }
}
SET_PLAYER_S*
```

pub fn _0x2f7ceb6520288061_safe(p0: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x2F7CEB6520288061");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x2F7CEB6520288061(p0)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x2F7CEB6520288061
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x2f7ceb6520288061_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x2f7ceb6520288061_raw(p0: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x2F7CEB6520288061(p0)
}

/// ```
Multiplier goes up to 1.49 any value above will be completely overruled by the game and the multiplier will not take effect, this can be edited in memory however.  
Just call it one time, it is not required to be called once every tick.  
Note: At least the IDA method if you change the max float multiplier from 1.5 it will change it for both this and SWIM above. I say 1.5 as the function blrs if what you input is greater than or equal to 1.5 hence why it's 1.49 max default.  
It is not possible to "decrease" speed. Anything below 1 will be ignored.  
```

pub fn set_run_sprint_multiplier_for_player_safe(player: Player, multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_RUN_SPRINT_MULTIPLIER_FOR_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_RUN_SPRINT_MULTIPLIER_FOR_PLAYER(player, multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_RUN_SPRINT_MULTIPLIER_FOR_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_run_sprint_multiplier_for_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_run_sprint_multiplier_for_player_raw(player: i32, multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_RUN_SPRINT_MULTIPLIER_FOR_PLAYER(player, multiplier)
}

/// ## Parameters
* **player**:

pub fn are_player_flashing_stars_about_to_drop_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: ARE_PLAYER_FLASHING_STARS_ABOUT_TO_DROP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ARE_PLAYER_FLASHING_STARS_ABOUT_TO_DROP(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: ARE_PLAYER_FLASHING_STARS_ABOUT_TO_DROP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `are_player_flashing_stars_about_to_drop_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn are_player_flashing_stars_about_to_drop_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ARE_PLAYER_FLASHING_STARS_ABOUT_TO_DROP(player)
}

/// ## Parameters
* **player**:

pub fn reset_player_input_gait_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: RESET_PLAYER_INPUT_GAIT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_PLAYER_INPUT_GAIT(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_PLAYER_INPUT_GAIT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_player_input_gait_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_player_input_gait_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_PLAYER_INPUT_GAIT(player)
}

/// Seems to lock the underwater timer of the specified player. Set `percentage` to `50.0` will reduce the value of [GET_PLAYER_UNDERWATER_TIME_REMAINING](#_0xA1FCF8E6AF40B731) to 5.0.

If you want to increase the underwater time for ped, use [SET_PED_MAX_TIME_UNDERWATER](#_0x6BA428C528D9E522) instead.

Using this native after [SET_PED_MAX_TIME_UNDERWATER](#_0x6BA428C528D9E522) **WILL NOT** get what you want. For example, if you set the max time underwater to `100.0` seconds using [SET_PED_MAX_TIME_UNDERWATER](#_0x6BA428C528D9E522) and then call this native and set the `percentage` to 50.0, you will not get `50.0`, instead `2.0`.

pub fn _set_player_underwater_time_remaining_safe(player: Player, percentage: f32) -> NativeResult<serde_json::Value> {
    
    debug!("Calling native function: _SET_PLAYER_UNDERWATER_TIME_REMAINING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PLAYER_UNDERWATER_TIME_REMAINING(player, percentage)
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _SET_PLAYER_UNDERWATER_TIME_REMAINING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_player_underwater_time_remaining_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_player_underwater_time_remaining_raw(player: i32, percentage: f32) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PLAYER_UNDERWATER_TIME_REMAINING(player, percentage)
}

/// ## Parameters
* **player**:

pub fn get_player_current_stealth_noise_safe(player: Player) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_PLAYER_CURRENT_STEALTH_NOISE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_CURRENT_STEALTH_NOISE(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PLAYER_CURRENT_STEALTH_NOISE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_current_stealth_noise_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_current_stealth_noise_raw(player: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_CURRENT_STEALTH_NOISE(player)
}

/// ## Parameters
* **player**:

pub fn reset_player_stamina_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: RESET_PLAYER_STAMINA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_PLAYER_STAMINA(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_PLAYER_STAMINA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_player_stamina_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_player_stamina_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_PLAYER_STAMINA(player)
}

/// ## Parameters
* **player1**: 
* **player2**: 
* **toggle**:

pub fn _0x55fcc0c390620314_safe(player1: Player, player2: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x55FCC0C390620314");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x55FCC0C390620314(player1, player2, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x55FCC0C390620314
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x55fcc0c390620314_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x55fcc0c390620314_raw(player1: i32, player2: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x55FCC0C390620314(player1, player2, toggle)
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn set_ignore_low_priority_shocking_events_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_IGNORE_LOW_PRIORITY_SHOCKING_EVENTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_IGNORE_LOW_PRIORITY_SHOCKING_EVENTS(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_IGNORE_LOW_PRIORITY_SHOCKING_EVENTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ignore_low_priority_shocking_events_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ignore_low_priority_shocking_events_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_IGNORE_LOW_PRIORITY_SHOCKING_EVENTS(player, toggle)
}

/// ```
this function is hard-coded to always return 0.  
```

pub fn is_player_logging_in_np_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_LOGGING_IN_NP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_LOGGING_IN_NP()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_LOGGING_IN_NP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_logging_in_np_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_logging_in_np_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_LOGGING_IN_NP()
}

/// ```
Values around 1.0f to 2.0f used in game scripts.  
```

pub fn set_player_sneaking_noise_multiplier_safe(player: Player, multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_SNEAKING_NOISE_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_SNEAKING_NOISE_MULTIPLIER(player, multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_SNEAKING_NOISE_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_sneaking_noise_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_sneaking_noise_multiplier_raw(player: i32, multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_SNEAKING_NOISE_MULTIPLIER(player, multiplier)
}

/// ## Parameters
* **player**: 
* **model**:

pub fn set_player_parachute_pack_model_override_safe(player: Player, model: u32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_PARACHUTE_PACK_MODEL_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_PARACHUTE_PACK_MODEL_OVERRIDE(player, model)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_PARACHUTE_PACK_MODEL_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_parachute_pack_model_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_parachute_pack_model_override_raw(player: i32, model: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_PARACHUTE_PACK_MODEL_OVERRIDE(player, model)
}

/// ## Parameters
* **player**:

pub fn _update_player_teleport_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: _UPDATE_PLAYER_TELEPORT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_UPDATE_PLAYER_TELEPORT(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _UPDATE_PLAYER_TELEPORT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_update_player_teleport_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _update_player_teleport_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_UPDATE_PLAYER_TELEPORT(player)
}

/// ```
Appears only 3 times in the scripts, more specifically in michael1.ysc
-
This can be used to prevent dying if you are "out of the world"
```

pub fn extend_world_boundary_for_player_safe(x: f32, y: f32, z: f32) -> NativeResult<()> {
    
    debug!("Calling native function: EXTEND_WORLD_BOUNDARY_FOR_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::EXTEND_WORLD_BOUNDARY_FOR_PLAYER(x, y, z)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: EXTEND_WORLD_BOUNDARY_FOR_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `extend_world_boundary_for_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn extend_world_boundary_for_player_raw(x: f32, y: f32, z: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::EXTEND_WORLD_BOUNDARY_FOR_PLAYER(x, y, z)
}

/// ## Parameters
* **player**: 
* **ped**: 
* **b2**: 
* **resetDamage**:

pub fn change_player_ped_safe(player: Player, ped: Ped, b2: bool, resetDamage: bool) -> NativeResult<()> {
    
    debug!("Calling native function: CHANGE_PLAYER_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CHANGE_PLAYER_PED(player, ped, b2, resetDamage)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CHANGE_PLAYER_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `change_player_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn change_player_ped_raw(player: i32, ped: i32, b2: bool, resetDamage: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CHANGE_PLAYER_PED(player, ped, b2, resetDamage)
}

/// ```
Tints:  
None = -1,  
Rainbow = 0,  
Red = 1,  
SeasideStripes = 2,  
WidowMaker = 3,  
Patriot = 4,  
Blue = 5,  
Black = 6,  
Hornet = 7,  
AirFocce = 8,  
Desert = 9,  
Shadow = 10,  
HighAltitude = 11,  
Airbone = 12,  
Sunrise = 13,  
```

pub fn set_player_reserve_parachute_tint_index_safe(player: Player, index: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_RESERVE_PARACHUTE_TINT_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_RESERVE_PARACHUTE_TINT_INDEX(player, index)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_RESERVE_PARACHUTE_TINT_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_reserve_parachute_tint_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_reserve_parachute_tint_index_raw(player: i32, index: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_RESERVE_PARACHUTE_TINT_INDEX(player, index)
}

/// ```
NativeDB Introduced: v2372
```

pub fn _get_player_parachute_model_override_safe(player: Player) -> NativeResult<u32> {
    
    debug!("Calling native function: _GET_PLAYER_PARACHUTE_MODEL_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_PLAYER_PARACHUTE_MODEL_OVERRIDE(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_PLAYER_PARACHUTE_MODEL_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_player_parachute_model_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_player_parachute_model_override_raw(player: i32) -> u32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_PLAYER_PARACHUTE_MODEL_OVERRIDE(player)
}

/// ```
This executes at the same as speed as PLAYER::SET_PLAYER_WANTED_LEVEL(player, 0, false);  
PLAYER::GET_PLAYER_WANTED_LEVEL(player); executes in less than half the time. Which means that it's worth first checking if the wanted level needs to be cleared before clearing. However, this is mostly about good code practice and can important in other situations. The difference in time in this example is negligible.  
```

pub fn clear_player_wanted_level_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PLAYER_WANTED_LEVEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PLAYER_WANTED_LEVEL(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PLAYER_WANTED_LEVEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_player_wanted_level_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_player_wanted_level_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PLAYER_WANTED_LEVEL(player)
}

/// ## Parameters
* **playerModel**:

pub fn is_special_ability_unlocked_safe(playerModel: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_SPECIAL_ABILITY_UNLOCKED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_SPECIAL_ABILITY_UNLOCKED(playerModel)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_SPECIAL_ABILITY_UNLOCKED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_special_ability_unlocked_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_special_ability_unlocked_raw(playerModel: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_SPECIAL_ABILITY_UNLOCKED(playerModel)
}

/// ASSISTED_MOVEMENT_FLUSH_ROUTE native function

pub fn assisted_movement_flush_route_safe() -> NativeResult<()> {
    
    debug!("Calling native function: ASSISTED_MOVEMENT_FLUSH_ROUTE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ASSISTED_MOVEMENT_FLUSH_ROUTE()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ASSISTED_MOVEMENT_FLUSH_ROUTE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `assisted_movement_flush_route_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn assisted_movement_flush_route_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ASSISTED_MOVEMENT_FLUSH_ROUTE()
}

/// This is to make the player walk without accepting input.

Call this native every frame so you can control the direction of your ped.

pub fn simulate_player_input_gait_safe(player: Player, amount: f32, gaitType: i64, rotationSpeed: f32, p4: bool, p5: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SIMULATE_PLAYER_INPUT_GAIT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SIMULATE_PLAYER_INPUT_GAIT(player, amount, gaitType, rotationSpeed, p4, p5)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SIMULATE_PLAYER_INPUT_GAIT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `simulate_player_input_gait_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn simulate_player_input_gait_raw(player: i32, amount: f32, gaitType: i64, rotationSpeed: f32, p4: bool, p5: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SIMULATE_PLAYER_INPUT_GAIT(player, amount, gaitType, rotationSpeed, p4, p5)
}

/// ## Parameters
* **player**: 
* **ms**: Time since last bullet fired
* **p2**: Always false

pub fn _has_player_been_shot_by_cop_safe(player: Player, ms: i64, p2: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: _HAS_PLAYER_BEEN_SHOT_BY_COP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_HAS_PLAYER_BEEN_SHOT_BY_COP(player, ms, p2)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _HAS_PLAYER_BEEN_SHOT_BY_COP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_has_player_been_shot_by_cop_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _has_player_been_shot_by_cop_raw(player: i32, ms: i64, p2: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_HAS_PLAYER_BEEN_SHOT_BY_COP(player, ms, p2)
}

/// ```
p1 appears as 5, 10, 15, 25, or 30. p2 is always true.
```

```
NativeDB Added Parameter 4: Any p3
```

pub fn special_ability_charge_absolute_safe(player: Player, p1: i64, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_ABILITY_CHARGE_ABSOLUTE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_ABILITY_CHARGE_ABSOLUTE(player, p1, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_ABILITY_CHARGE_ABSOLUTE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_ability_charge_absolute_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_ability_charge_absolute_raw(player: i32, p1: i64, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_ABILITY_CHARGE_ABSOLUTE(player, p1, p2)
}

/// ```
Max value is 1.0  
```

pub fn set_wanted_level_difficulty_safe(player: Player, difficulty: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_WANTED_LEVEL_DIFFICULTY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_WANTED_LEVEL_DIFFICULTY(player, difficulty)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_WANTED_LEVEL_DIFFICULTY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_wanted_level_difficulty_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_wanted_level_difficulty_raw(player: i32, difficulty: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_WANTED_LEVEL_DIFFICULTY(player, difficulty)
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn set_dispatch_cops_for_player_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_DISPATCH_COPS_FOR_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DISPATCH_COPS_FOR_PLAYER(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DISPATCH_COPS_FOR_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_dispatch_cops_for_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_dispatch_cops_for_player_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DISPATCH_COPS_FOR_PLAYER(player, toggle)
}

/// ## Parameters
* **player**:

pub fn _0xfac75988a7d078d3_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: _0xFAC75988A7D078D3");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xFAC75988A7D078D3(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xFAC75988A7D078D3
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xfac75988a7d078d3_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xfac75988a7d078d3_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xFAC75988A7D078D3(player)
}

/// ## Parameters
* **player**:

pub fn get_time_since_player_drove_against_traffic_safe(player: Player) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_TIME_SINCE_PLAYER_DROVE_AGAINST_TRAFFIC");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_TIME_SINCE_PLAYER_DROVE_AGAINST_TRAFFIC(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_TIME_SINCE_PLAYER_DROVE_AGAINST_TRAFFIC
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_time_since_player_drove_against_traffic_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_time_since_player_drove_against_traffic_raw(player: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_TIME_SINCE_PLAYER_DROVE_AGAINST_TRAFFIC(player)
}

/// The native ensures the 'modifier' parameter is 0.1 or greater.

pub fn set_player_weapon_damage_modifier_safe(player: Player, modifier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_WEAPON_DAMAGE_MODIFIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_WEAPON_DAMAGE_MODIFIER(player, modifier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_WEAPON_DAMAGE_MODIFIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_weapon_damage_modifier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_weapon_damage_modifier_raw(player: i32, modifier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_WEAPON_DAMAGE_MODIFIER(player, modifier)
}

/// ## Parameters
* **multiplier**:

pub fn set_special_ability_multiplier_safe(multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SPECIAL_ABILITY_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SPECIAL_ABILITY_MULTIPLIER(multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SPECIAL_ABILITY_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_special_ability_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_special_ability_multiplier_raw(multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SPECIAL_ABILITY_MULTIPLIER(multiplier)
}

/// ## Parameters
* **player**:

pub fn _0x9edd76e87d5d51ba_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: _0x9EDD76E87D5D51BA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x9EDD76E87D5D51BA(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x9EDD76E87D5D51BA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x9edd76e87d5d51ba_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x9edd76e87d5d51ba_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x9EDD76E87D5D51BA(player)
}

/// ## Parameters
* **playerId**: The local index of the player to check (see [PLAYER_ID](#_0x4F8644AF03D0E0D6))

pub fn get_is_player_driving_on_highway_safe(playerId: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_IS_PLAYER_DRIVING_ON_HIGHWAY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_IS_PLAYER_DRIVING_ON_HIGHWAY(playerId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_IS_PLAYER_DRIVING_ON_HIGHWAY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_is_player_driving_on_highway_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_is_player_driving_on_highway_raw(playerId: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_IS_PLAYER_DRIVING_ON_HIGHWAY(playerId)
}

/// ```
NativeDB Introduced: v1290
```

pub fn _0x237440e46d918649_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x237440E46D918649");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x237440E46D918649(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x237440E46D918649
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x237440e46d918649_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x237440e46d918649_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x237440E46D918649(p0)
}

/// ```
modifier's min value is 0.1
```

pub fn set_player_melee_weapon_defense_modifier_safe(player: Player, modifier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_MELEE_WEAPON_DEFENSE_MODIFIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_MELEE_WEAPON_DEFENSE_MODIFIER(player, modifier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_MELEE_WEAPON_DEFENSE_MODIFIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_melee_weapon_defense_modifier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_melee_weapon_defense_modifier_raw(player: i32, modifier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_MELEE_WEAPON_DEFENSE_MODIFIER(player, modifier)
}

/// ## Parameters
* **player**:

pub fn are_player_stars_greyed_out_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: ARE_PLAYER_STARS_GREYED_OUT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ARE_PLAYER_STARS_GREYED_OUT(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: ARE_PLAYER_STARS_GREYED_OUT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `are_player_stars_greyed_out_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn are_player_stars_greyed_out_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ARE_PLAYER_STARS_GREYED_OUT(player)
}

/// Returns the entity handle for the local player ped. Note that this entity handle will change after using commands such as SET\_PLAYER\_MODEL.

pub fn player_ped_id_safe() -> NativeResult<Ped> {
    
    debug!("Calling native function: PLAYER_PED_ID");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::PLAYER_PED_ID()
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: PLAYER_PED_ID
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `player_ped_id_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn player_ped_id_raw() -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::PLAYER_PED_ID()
}

/// ```
PLAYER::FORCE_CLEANUP_FOR_ALL_THREADS_WITH_THIS_NAME("pb_prostitute", 1); // Found in decompilation  
```

pub fn force_cleanup_for_all_threads_with_this_name_safe(name: String, cleanupFlags: i64) -> NativeResult<()> {
    let name_cstr = std::ffi::CString::new(name.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "name", e)))?;
    
    debug!("Calling native function: FORCE_CLEANUP_FOR_ALL_THREADS_WITH_THIS_NAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::FORCE_CLEANUP_FOR_ALL_THREADS_WITH_THIS_NAME(crate::utils::rust_to_c_string(name), cleanupFlags)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: FORCE_CLEANUP_FOR_ALL_THREADS_WITH_THIS_NAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `force_cleanup_for_all_threads_with_this_name_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn force_cleanup_for_all_threads_with_this_name_raw(name: *const std::os::raw::c_char, cleanupFlags: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::FORCE_CLEANUP_FOR_ALL_THREADS_WITH_THIS_NAME(name, cleanupFlags)
}

/// ## Parameters
* **duration**:

pub fn start_firing_amnesty_safe(duration: i64) -> NativeResult<()> {
    
    debug!("Calling native function: START_FIRING_AMNESTY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::START_FIRING_AMNESTY(duration)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: START_FIRING_AMNESTY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `start_firing_amnesty_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn start_firing_amnesty_raw(duration: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::START_FIRING_AMNESTY(duration)
}

/// ## Parameters
* **player**:

pub fn can_player_start_mission_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: CAN_PLAYER_START_MISSION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CAN_PLAYER_START_MISSION(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CAN_PLAYER_START_MISSION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `can_player_start_mission_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn can_player_start_mission_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CAN_PLAYER_START_MISSION(player)
}

/// ```
Returns the time since the character died in (ms) milliseconds.  
example  
var time = Function.call<int>(Hash.GET_TIME_SINCE_LAST_DEATH();  
UI.DrawSubtitle(time.ToString());  
if player has not died, the int returned will be -1.  
```

pub fn get_time_since_last_death_safe() -> NativeResult<i64> {
    
    debug!("Calling native function: GET_TIME_SINCE_LAST_DEATH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_TIME_SINCE_LAST_DEATH()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_TIME_SINCE_LAST_DEATH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_time_since_last_death_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_time_since_last_death_raw() -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_TIME_SINCE_LAST_DEATH()
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn set_player_leave_ped_behind_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_LEAVE_PED_BEHIND");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_LEAVE_PED_BEHIND(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_LEAVE_PED_BEHIND
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_leave_ped_behind_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_leave_ped_behind_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_LEAVE_PED_BEHIND(player, toggle)
}

/// ## Parameters
* **achievement**:

pub fn has_achievement_been_passed_safe(achievement: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_ACHIEVEMENT_BEEN_PASSED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_ACHIEVEMENT_BEEN_PASSED(achievement)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_ACHIEVEMENT_BEEN_PASSED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_achievement_been_passed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_achievement_been_passed_raw(achievement: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_ACHIEVEMENT_BEEN_PASSED(achievement)
}

/// ```
NativeDB Added Parameter 2: Any p1
```

pub fn is_special_ability_enabled_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_SPECIAL_ABILITY_ENABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_SPECIAL_ABILITY_ENABLED(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_SPECIAL_ABILITY_ENABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_special_ability_enabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_special_ability_enabled_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_SPECIAL_ABILITY_ENABLED(player)
}

/// ## Parameters
* **player**:

pub fn has_player_been_spotted_in_stolen_vehicle_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_PLAYER_BEEN_SPOTTED_IN_STOLEN_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_PLAYER_BEEN_SPOTTED_IN_STOLEN_VEHICLE(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_PLAYER_BEEN_SPOTTED_IN_STOLEN_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_player_been_spotted_in_stolen_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_player_been_spotted_in_stolen_vehicle_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_PLAYER_BEEN_SPOTTED_IN_STOLEN_VEHICLE(player)
}

/// ```
NativeDB Introduced: v2372
```

pub fn _get_player_reserve_parachute_model_override_safe(player: Player) -> NativeResult<u32> {
    
    debug!("Calling native function: _GET_PLAYER_RESERVE_PARACHUTE_MODEL_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_PLAYER_RESERVE_PARACHUTE_MODEL_OVERRIDE(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_PLAYER_RESERVE_PARACHUTE_MODEL_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_player_reserve_parachute_model_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_player_reserve_parachute_model_override_raw(player: i32) -> u32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_PLAYER_RESERVE_PARACHUTE_MODEL_OVERRIDE(player)
}

/// ## Parameters
* **player**:

pub fn is_player_dead_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_DEAD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_DEAD(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_DEAD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_dead_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_dead_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_DEAD(player)
}

/// ```
2 occurrences in agency_heist3a. p1 was 0.7f then 0.4f.  
```

pub fn _0xdd2620b7b9d16ff1_safe(player: Player, p1: f32) -> NativeResult<bool> {
    
    debug!("Calling native function: _0xDD2620B7B9D16FF1");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xDD2620B7B9D16FF1(player, p1)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0xDD2620B7B9D16FF1
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xdd2620b7b9d16ff1_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xdd2620b7b9d16ff1_raw(player: i32, p1: f32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xDD2620B7B9D16FF1(player, p1)
}

/// ```
Simply returns whatever is passed to it (Regardless of whether the handle is valid or not).  
```

pub fn int_to_playerindex_safe(value: i64) -> NativeResult<Player> {
    
    debug!("Calling native function: INT_TO_PLAYERINDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::INT_TO_PLAYERINDEX(value)
    };
    
    
    Ok(Ok(Player(result)))
}

/// Raw native function: INT_TO_PLAYERINDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `int_to_playerindex_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn int_to_playerindex_raw(value: i64) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::INT_TO_PLAYERINDEX(value)
}

/// ## Parameters
* **maxWantedLevel**:

pub fn set_max_wanted_level_safe(maxWantedLevel: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_MAX_WANTED_LEVEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_MAX_WANTED_LEVEL(maxWantedLevel)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_MAX_WANTED_LEVEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_max_wanted_level_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_max_wanted_level_raw(maxWantedLevel: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_MAX_WANTED_LEVEL(maxWantedLevel)
}

/// ## Parameters
* **player**:

pub fn clear_player_parachute_variation_override_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PLAYER_PARACHUTE_VARIATION_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PLAYER_PARACHUTE_VARIATION_OVERRIDE(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PLAYER_PARACHUTE_VARIATION_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_player_parachute_variation_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_player_parachute_variation_override_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PLAYER_PARACHUTE_VARIATION_OVERRIDE(player)
}

/// ## Parameters
* **player**:

pub fn has_player_left_the_world_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_PLAYER_LEFT_THE_WORLD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_PLAYER_LEFT_THE_WORLD(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_PLAYER_LEFT_THE_WORLD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_player_left_the_world_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_player_left_the_world_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_PLAYER_LEFT_THE_WORLD(player)
}

/// ## Parameters
* **player**: 
* **r**: 
* **g**: 
* **b**:

pub fn get_player_rgb_colour_safe(player: Player, r: *mut i64, g: *mut i64, b: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_PLAYER_RGB_COLOUR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_RGB_COLOUR(player, r, g, b)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_PLAYER_RGB_COLOUR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_rgb_colour_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_rgb_colour_raw(player: i32, r: *mut i64, g: *mut i64, b: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_RGB_COLOUR(player, r, g, b)
}

/// ```
Gets the number of players in the current session.
If not multiplayer, always returns 1.
```

pub fn get_number_of_players_safe() -> NativeResult<i64> {
    
    debug!("Calling native function: GET_NUMBER_OF_PLAYERS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_NUMBER_OF_PLAYERS()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_NUMBER_OF_PLAYERS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_number_of_players_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_number_of_players_raw() -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_NUMBER_OF_PLAYERS()
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x2f41a3bae005e5fa_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p1_any_str = serde_json::to_string(&p1)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p1", e)))?;
    let p1_any_str_cstr = std::ffi::CString::new(p1_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p1", e)))?;
    
    debug!("Calling native function: _0x2F41A3BAE005E5FA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x2F41A3BAE005E5FA(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x2F41A3BAE005E5FA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x2f41a3bae005e5fa_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x2f41a3bae005e5fa_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x2F41A3BAE005E5FA(p0, p1)
}

/// ```
Name between DISABLE_ALL_CONTROL_ACTIONS and DISABLE_CONTROL_ACTION
```

pub fn _0x5501b7a5cdb79d37_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: _0x5501B7A5CDB79D37");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x5501B7A5CDB79D37(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x5501B7A5CDB79D37
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x5501b7a5cdb79d37_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x5501b7a5cdb79d37_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x5501B7A5CDB79D37(player)
}

/// ```
NativeDB Added Parameter 2: Any p1
```

pub fn is_special_ability_meter_full_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_SPECIAL_ABILITY_METER_FULL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_SPECIAL_ABILITY_METER_FULL(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_SPECIAL_ABILITY_METER_FULL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_special_ability_meter_full_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_special_ability_meter_full_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_SPECIAL_ABILITY_METER_FULL(player)
}

/// Always returns false.

```
NativeDB Introduced: v1868
```

pub fn _0xdcc07526b8ec45af_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: _0xDCC07526B8EC45AF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xDCC07526B8EC45AF(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0xDCC07526B8EC45AF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xdcc07526b8ec45af_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xdcc07526b8ec45af_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xDCC07526B8EC45AF(player)
}

/// ## Parameters
* **player**: the target player
* **wantedLevel**: the wanted level 1-5
* **delayedResponse**: false = 0-10sec police spawn response time, true = 10-20sec police spawn response time

pub fn set_player_wanted_level_safe(player: Player, wantedLevel: i64, delayedResponse: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_WANTED_LEVEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_WANTED_LEVEL(player, wantedLevel, delayedResponse)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_WANTED_LEVEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_wanted_level_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_wanted_level_raw(player: i32, wantedLevel: i64, delayedResponse: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_WANTED_LEVEL(player, wantedLevel, delayedResponse)
}

/// ```
Return true while player is being arrested / busted.  
If atArresting is set to 1, this function will return 1 when player is being arrested (while player is putting his hand up, but still have control)  
If atArresting is set to 0, this function will return 1 only when the busted screen is shown.  
```

pub fn is_player_being_arrested_safe(player: Player, atArresting: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_BEING_ARRESTED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_BEING_ARRESTED(player, atArresting)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_BEING_ARRESTED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_being_arrested_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_being_arrested_raw(player: i32, atArresting: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_BEING_ARRESTED(player, atArresting)
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn set_auto_give_parachute_when_enter_plane_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_AUTO_GIVE_PARACHUTE_WHEN_ENTER_PLANE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_AUTO_GIVE_PARACHUTE_WHEN_ENTER_PLANE(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_AUTO_GIVE_PARACHUTE_WHEN_ENTER_PLANE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_auto_give_parachute_when_enter_plane_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_auto_give_parachute_when_enter_plane_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_AUTO_GIVE_PARACHUTE_WHEN_ENTER_PLANE(player, toggle)
}

/// ```
Returns true when the player is not able to control the cam i.e. when running a benchmark test, switching the player or viewing a cutscene.  
Note: I am not 100% sure if the native actually checks if the cam control is disabled but it seems promising.  
```

pub fn _is_player_cam_control_disabled_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PLAYER_CAM_CONTROL_DISABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PLAYER_CAM_CONTROL_DISABLED()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PLAYER_CAM_CONTROL_DISABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_player_cam_control_disabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_player_cam_control_disabled_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PLAYER_CAM_CONTROL_DISABLED()
}

/// ```
Purpose of the BOOL currently unknown.  
Both, true and false, work  
```

pub fn display_system_signin_ui_safe(unk: bool) -> NativeResult<()> {
    
    debug!("Calling native function: DISPLAY_SYSTEM_SIGNIN_UI");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DISPLAY_SYSTEM_SIGNIN_UI(unk)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DISPLAY_SYSTEM_SIGNIN_UI
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `display_system_signin_ui_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn display_system_signin_ui_raw(unk: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DISPLAY_SYSTEM_SIGNIN_UI(unk)
}

/// ```
Only 1 match. ob_sofa_michael.  
PLAYER::PLAYER_ATTACH_VIRTUAL_BOUND(-804.5928f, 173.1801f, 71.68436f, 0f, 0f, 0.590625f, 1f, 0.7f);1.0.335.2, 1.0.350.1/2, 1.0.372.2, 1.0.393.2, 1.0.393.4, 1.0.463.1;  
```

pub fn player_attach_virtual_bound_safe(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32) -> NativeResult<()> {
    
    debug!("Calling native function: PLAYER_ATTACH_VIRTUAL_BOUND");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::PLAYER_ATTACH_VIRTUAL_BOUND(p0, p1, p2, p3, p4, p5, p6, p7)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: PLAYER_ATTACH_VIRTUAL_BOUND
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `player_attach_virtual_bound_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn player_attach_virtual_bound_raw(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::PLAYER_ATTACH_VIRTUAL_BOUND(p0, p1, p2, p3, p4, p5, p6, p7)
}

/// ```
normalizedValue is from 0.0 - 1.0
p2 is always 1
```

```
NativeDB Added Parameter 4: Any p3
```

pub fn special_ability_charge_normalized_safe(player: Player, normalizedValue: f32, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_ABILITY_CHARGE_NORMALIZED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_ABILITY_CHARGE_NORMALIZED(player, normalizedValue, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_ABILITY_CHARGE_NORMALIZED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_ability_charge_normalized_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_ability_charge_normalized_raw(player: i32, normalizedValue: f32, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_ABILITY_CHARGE_NORMALIZED(player, normalizedValue, p2)
}

/// ```
Disables something. Used only once in R* scripts (freemode.ysc).
DISABLE_PLAYER_*
```

pub fn _0xb885852c39cc265d_safe() -> NativeResult<()> {
    
    debug!("Calling native function: _0xB885852C39CC265D");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xB885852C39CC265D()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xB885852C39CC265D
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xb885852c39cc265d_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xb885852c39cc265d_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xB885852C39CC265D()
}

/// ```
Tints:  
None = -1,  
Rainbow = 0,  
Red = 1,  
SeasideStripes = 2,  
WidowMaker = 3,  
Patriot = 4,  
Blue = 5,  
Black = 6,  
Hornet = 7,  
AirFocce = 8,  
Desert = 9,  
Shadow = 10,  
HighAltitude = 11,  
Airbone = 12,  
Sunrise = 13,  
```

pub fn get_player_parachute_tint_index_safe(player: Player, tintIndex: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_PLAYER_PARACHUTE_TINT_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_PARACHUTE_TINT_INDEX(player, tintIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_PLAYER_PARACHUTE_TINT_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_parachute_tint_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_parachute_tint_index_raw(player: i32, tintIndex: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_PARACHUTE_TINT_INDEX(player, tintIndex)
}

/// ```
This has been found in use in the decompiled files.  
```

pub fn _0x4669b3ed80f24b4e_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: _0x4669B3ED80F24B4E");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x4669B3ED80F24B4E(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x4669B3ED80F24B4E
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x4669b3ed80f24b4e_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x4669b3ed80f24b4e_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x4669B3ED80F24B4E(player)
}

/// ```
modifier's min value is 0.1
```

pub fn set_player_vehicle_defense_modifier_safe(player: Player, modifier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_VEHICLE_DEFENSE_MODIFIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_VEHICLE_DEFENSE_MODIFIER(player, modifier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_VEHICLE_DEFENSE_MODIFIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_vehicle_defense_modifier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_vehicle_defense_modifier_raw(player: i32, modifier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_VEHICLE_DEFENSE_MODIFIER(player, modifier)
}

/// ## Parameters
* **player**:

pub fn _0x36f1b38855f2a8df_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: _0x36F1B38855F2A8DF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x36F1B38855F2A8DF(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x36F1B38855F2A8DF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x36f1b38855f2a8df_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x36f1b38855f2a8df_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x36F1B38855F2A8DF(player)
}

/// ```
Used to toggle the square up aim.
```

pub fn set_player_lockon_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_LOCKON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_LOCKON(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_LOCKON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_lockon_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_lockon_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_LOCKON(player, toggle)
}

/// Disables vehicle rewards for the current frame.

pub fn disable_player_vehicle_rewards_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: DISABLE_PLAYER_VEHICLE_REWARDS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DISABLE_PLAYER_VEHICLE_REWARDS(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DISABLE_PLAYER_VEHICLE_REWARDS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `disable_player_vehicle_rewards_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn disable_player_vehicle_rewards_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DISABLE_PLAYER_VEHICLE_REWARDS(player)
}

/// ## Parameters
* **player**:

pub fn is_player_free_for_ambient_task_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_FREE_FOR_AMBIENT_TASK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_FREE_FOR_AMBIENT_TASK(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_FREE_FOR_AMBIENT_TASK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_free_for_ambient_task_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_free_for_ambient_task_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_FREE_FOR_AMBIENT_TASK(player)
}

/// ## Parameters
* **player**:

pub fn is_player_pressing_horn_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_PRESSING_HORN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_PRESSING_HORN(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_PRESSING_HORN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_pressing_horn_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_pressing_horn_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_PRESSING_HORN(player)
}

/// ```
NativeDB Added Parameter 2: Any p1
```

pub fn special_ability_deactivate_fast_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_ABILITY_DEACTIVATE_FAST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_ABILITY_DEACTIVATE_FAST(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_ABILITY_DEACTIVATE_FAST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_ability_deactivate_fast_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_ability_deactivate_fast_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_ABILITY_DEACTIVATE_FAST(player)
}

/// ```
Drft  
```

pub fn get_wanted_level_threshold_safe(wantedLevel: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_WANTED_LEVEL_THRESHOLD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_WANTED_LEVEL_THRESHOLD(wantedLevel)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_WANTED_LEVEL_THRESHOLD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_wanted_level_threshold_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_wanted_level_threshold_raw(wantedLevel: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_WANTED_LEVEL_THRESHOLD(wantedLevel)
}

/// ## Parameters
* **player**: 
* **p1**:

pub fn _0xd821056b9acf8052_safe(player: Player, p1: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p1_any_str = serde_json::to_string(&p1)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p1", e)))?;
    let p1_any_str_cstr = std::ffi::CString::new(p1_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p1", e)))?;
    
    debug!("Calling native function: _0xD821056B9ACF8052");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xD821056B9ACF8052(player, crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xD821056B9ACF8052
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xd821056b9acf8052_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xd821056b9acf8052_raw(player: i32, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xD821056B9ACF8052(player, p1)
}

/// ```
Sets whether this player can take cover.
```

pub fn set_player_can_use_cover_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_CAN_USE_COVER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_CAN_USE_COVER(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_CAN_USE_COVER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_can_use_cover_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_can_use_cover_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_CAN_USE_COVER(player, toggle)
}

/// ```
Returns profile setting 243.
GET_*
```

pub fn _0xcb645e85e97ea48b_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: _0xCB645E85E97EA48B");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xCB645E85E97EA48B()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0xCB645E85E97EA48B
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xcb645e85e97ea48b_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xcb645e85e97ea48b_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xCB645E85E97EA48B()
}

/// ```
Gets a value indicating whether the specified player is currently aiming freely at the specified entity.  
```

pub fn is_player_free_aiming_at_entity_safe(player: Player, entity: Entity) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_FREE_AIMING_AT_ENTITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_FREE_AIMING_AT_ENTITY(player, entity)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_FREE_AIMING_AT_ENTITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_free_aiming_at_entity_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_free_aiming_at_entity_raw(player: i32, entity: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_FREE_AIMING_AT_ENTITY(player, entity)
}

/// ```
2 matches in 1 script - am_hold_up
Used in multiplayer scripts?
```

pub fn _0x0032a6dba562c518_safe() -> NativeResult<()> {
    
    debug!("Calling native function: _0x0032A6DBA562C518");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x0032A6DBA562C518()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x0032A6DBA562C518
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x0032a6dba562c518_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x0032a6dba562c518_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x0032A6DBA562C518()
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn set_player_simulate_aiming_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_SIMULATE_AIMING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_SIMULATE_AIMING(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_SIMULATE_AIMING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_simulate_aiming_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_simulate_aiming_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_SIMULATE_AIMING(player, toggle)
}

/// ## Parameters
* **player**: 
* **modifier**:

pub fn _set_player_weapon_defense_modifier_2_safe(player: Player, modifier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PLAYER_WEAPON_DEFENSE_MODIFIER_2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PLAYER_WEAPON_DEFENSE_MODIFIER_2(player, modifier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PLAYER_WEAPON_DEFENSE_MODIFIER_2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_player_weapon_defense_modifier_2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_player_weapon_defense_modifier_2_raw(player: i32, modifier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PLAYER_WEAPON_DEFENSE_MODIFIER_2(player, modifier)
}

/// ```
Returns the same as PLAYER_ID and NETWORK_PLAYER_ID_TO_INT  
```

pub fn get_player_index_safe() -> NativeResult<Player> {
    
    debug!("Calling native function: GET_PLAYER_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_INDEX()
    };
    
    
    Ok(Ok(Player(result)))
}

/// Raw native function: GET_PLAYER_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_index_raw() -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_INDEX()
}

/// ## Parameters
* **player**: 
* **enabled**:

pub fn set_player_can_leave_parachute_smoke_trail_safe(player: Player, enabled: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_CAN_LEAVE_PARACHUTE_SMOKE_TRAIL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_CAN_LEAVE_PARACHUTE_SMOKE_TRAIL(player, enabled)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_CAN_LEAVE_PARACHUTE_SMOKE_TRAIL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_can_leave_parachute_smoke_trail_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_can_leave_parachute_smoke_trail_raw(player: i32, enabled: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_CAN_LEAVE_PARACHUTE_SMOKE_TRAIL(player, enabled)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x2382ab11450ae7ba_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p1_any_str = serde_json::to_string(&p1)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p1", e)))?;
    let p1_any_str_cstr = std::ffi::CString::new(p1_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p1", e)))?;
    
    debug!("Calling native function: _0x2382AB11450AE7BA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x2382AB11450AE7BA(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x2382AB11450AE7BA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x2382ab11450ae7ba_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x2382ab11450ae7ba_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x2382AB11450AE7BA(p0, p1)
}

/// ```
Found in "director_mode", "fm_bj_race_controler", "fm_deathmatch_controler", "fm_impromptu_dm_controler", "fm_race_controler", "gb_deathmatch".  
```

pub fn _0xcac57395b151135f_safe(player: Player, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xCAC57395B151135F");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xCAC57395B151135F(player, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xCAC57395B151135F
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xcac57395b151135f_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xcac57395b151135f_raw(player: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xCAC57395B151135F(player, p1)
}

/// ## Parameters
* **player**:

pub fn set_all_random_peds_flee_this_frame_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: SET_ALL_RANDOM_PEDS_FLEE_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_ALL_RANDOM_PEDS_FLEE_THIS_FRAME(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_ALL_RANDOM_PEDS_FLEE_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_all_random_peds_flee_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_all_random_peds_flee_this_frame_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_ALL_RANDOM_PEDS_FLEE_THIS_FRAME(player)
}

/// ## Parameters
* **player**: 
* **r**: 
* **g**: 
* **b**:

pub fn set_player_parachute_smoke_trail_color_safe(player: Player, r: i64, g: i64, b: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_PARACHUTE_SMOKE_TRAIL_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_PARACHUTE_SMOKE_TRAIL_COLOR(player, r, g, b)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_PARACHUTE_SMOKE_TRAIL_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_parachute_smoke_trail_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_parachute_smoke_trail_color_raw(player: i32, r: i64, g: i64, b: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_PARACHUTE_SMOKE_TRAIL_COLOR(player, r, g, b)
}

/// ## Parameters
* **player**:

pub fn is_player_ready_for_cutscene_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_READY_FOR_CUTSCENE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_READY_FOR_CUTSCENE(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_READY_FOR_CUTSCENE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_ready_for_cutscene_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_ready_for_cutscene_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_READY_FOR_CUTSCENE(player)
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn set_player_forced_zoom_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_FORCED_ZOOM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_FORCED_ZOOM(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_FORCED_ZOOM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_forced_zoom_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_forced_zoom_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_FORCED_ZOOM(player, toggle)
}

/// Teleports the player to the given coordinates.

If findCollisionLand is true it will try to find the Z value for you, this however has a timeout of 100 frames.

When trying to find the Z value the native will take longer the higher the difference from the given Z to the ground, this combined with the timeout can cause the teleport to just teleport to the given Z value, so try to estimate the z value, so don't just pass in 1000.0.

Also if you're in a vehicle and teleportWithVehicle is true it will not find the Z value for you.

pub fn start_player_teleport_safe(player: Player, x: f32, y: f32, z: f32, heading: f32, teleportWithVehicle: bool, findCollisionLand: bool, p7: bool) -> NativeResult<()> {
    
    debug!("Calling native function: START_PLAYER_TELEPORT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::START_PLAYER_TELEPORT(player, x, y, z, heading, teleportWithVehicle, findCollisionLand, p7)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: START_PLAYER_TELEPORT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `start_player_teleport_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn start_player_teleport_raw(player: i32, x: f32, y: f32, z: f32, heading: f32, teleportWithVehicle: bool, findCollisionLand: bool, p7: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::START_PLAYER_TELEPORT(player, x, y, z, heading, teleportWithVehicle, findCollisionLand, p7)
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn set_player_forced_aim_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_FORCED_AIM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_FORCED_AIM(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_FORCED_AIM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_forced_aim_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_forced_aim_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_FORCED_AIM(player, toggle)
}

/// ```
Simply returns whatever is passed to it (Regardless of whether the handle is valid or not).  
--------------------------------------------------------  
if (NETWORK::NETWORK_IS_PARTICIPANT_ACTIVE(PLAYER::INT_TO_PARTICIPANTINDEX(i)))  
```

pub fn int_to_participantindex_safe(value: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: INT_TO_PARTICIPANTINDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::INT_TO_PARTICIPANTINDEX(value)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: INT_TO_PARTICIPANTINDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `int_to_participantindex_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn int_to_participantindex_raw(value: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::INT_TO_PARTICIPANTINDEX(value)
}

/// ```
NativeDB Introduced: v1180
```

pub fn _get_number_of_players_in_team_safe(team: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_NUMBER_OF_PLAYERS_IN_TEAM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_NUMBER_OF_PLAYERS_IN_TEAM(team)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_NUMBER_OF_PLAYERS_IN_TEAM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_number_of_players_in_team_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_number_of_players_in_team_raw(team: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_NUMBER_OF_PLAYERS_IN_TEAM(team)
}

/// ```
Checks whether the specified player has a Ped, the Ped is not dead, is not injured and is not arrested.  
```

pub fn is_player_playing_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_PLAYING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_PLAYING(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_PLAYING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_playing_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_playing_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_PLAYING(player)
}

/// ```
PLAYER::0xBF6993C7(rPtr((&l_122) + 71)); // Found in decompilation
***
In "am_hold_up.ysc" used once:
l_8d._f47 = MISC::GET_RANDOM_FLOAT_IN_RANGE(18.0, 28.0);
PLAYER::_B45EFF719D8427A6((l_8d._f47));
```

pub fn _0xb45eff719d8427a6_safe(p0: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0xB45EFF719D8427A6");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xB45EFF719D8427A6(p0)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xB45EFF719D8427A6
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xb45eff719d8427a6_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xb45eff719d8427a6_raw(p0: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xB45EFF719D8427A6(p0)
}

/// ```
Returns profile setting 237.
GET_*
```

pub fn _0xb9cf1f793a9f1bf1_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: _0xB9CF1F793A9F1BF1");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xB9CF1F793A9F1BF1()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0xB9CF1F793A9F1BF1
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xb9cf1f793a9f1bf1_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xb9cf1f793a9f1bf1_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xB9CF1F793A9F1BF1()
}

/// ## Parameters
* **player**: 
* **p1**:

pub fn _0x31e90b8873a4cd3b_safe(player: Player, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0x31E90B8873A4CD3B");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x31E90B8873A4CD3B(player, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x31E90B8873A4CD3B
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x31e90b8873a4cd3b_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x31e90b8873a4cd3b_raw(player: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x31E90B8873A4CD3B(player, p1)
}

/// ## Parameters
* **player**:

pub fn reset_player_arrest_state_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: RESET_PLAYER_ARREST_STATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_PLAYER_ARREST_STATE(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_PLAYER_ARREST_STATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_player_arrest_state_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_player_arrest_state_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_PLAYER_ARREST_STATE(player)
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn set_auto_give_scuba_gear_when_exit_vehicle_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_AUTO_GIVE_SCUBA_GEAR_WHEN_EXIT_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_AUTO_GIVE_SCUBA_GEAR_WHEN_EXIT_VEHICLE(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_AUTO_GIVE_SCUBA_GEAR_WHEN_EXIT_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_auto_give_scuba_gear_when_exit_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_auto_give_scuba_gear_when_exit_vehicle_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_AUTO_GIVE_SCUBA_GEAR_WHEN_EXIT_VEHICLE(player, toggle)
}

/// RESTORE_PLAYER_STAMINA native function

pub fn restore_player_stamina_safe(player: Player, percentage: f32) -> NativeResult<()> {
    
    debug!("Calling native function: RESTORE_PLAYER_STAMINA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESTORE_PLAYER_STAMINA(player, percentage)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESTORE_PLAYER_STAMINA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `restore_player_stamina_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn restore_player_stamina_raw(player: i32, percentage: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESTORE_PLAYER_STAMINA(player, percentage)
}

/// ```
Returns true if the player is riding a train.  
```

pub fn is_player_riding_train_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_RIDING_TRAIN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_RIDING_TRAIN(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_RIDING_TRAIN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_riding_train_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_riding_train_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_RIDING_TRAIN(player)
}

/// ```
# Predominant call signatures  
PLAYER::SET_PLAYER_WANTED_CENTRE_POSITION(PLAYER::PLAYER_ID(), ENTITY::GET_ENTITY_COORDS(PLAYER::PLAYER_PED_ID(), 1));  
# Parameter value ranges  
P0: PLAYER::PLAYER_ID()  
P1: ENTITY::GET_ENTITY_COORDS(PLAYER::PLAYER_PED_ID(), 1)  
P2: Not set by any call  
```

pub fn set_player_wanted_centre_position_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_WANTED_CENTRE_POSITION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_WANTED_CENTRE_POSITION(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_WANTED_CENTRE_POSITION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_wanted_centre_position_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_wanted_centre_position_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_WANTED_CENTRE_POSITION(player)
}

/// ## Parameters
* **id**: 
* **cleanupFlags**:

pub fn force_cleanup_for_thread_with_this_id_safe(id: i64, cleanupFlags: i64) -> NativeResult<()> {
    
    debug!("Calling native function: FORCE_CLEANUP_FOR_THREAD_WITH_THIS_ID");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::FORCE_CLEANUP_FOR_THREAD_WITH_THIS_ID(id, cleanupFlags)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: FORCE_CLEANUP_FOR_THREAD_WITH_THIS_ID
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `force_cleanup_for_thread_with_this_id_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn force_cleanup_for_thread_with_this_id_raw(id: i64, cleanupFlags: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::FORCE_CLEANUP_FOR_THREAD_WITH_THIS_ID(id, cleanupFlags)
}

/// ```
Gets the player's team.  
Does nothing in singleplayer.  
```

pub fn get_player_team_safe(player: Player) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PLAYER_TEAM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_TEAM(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PLAYER_TEAM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_team_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_team_raw(player: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_TEAM(player)
}

/// ## Parameters
* **player**:

pub fn get_player_underwater_time_remaining_safe(player: Player) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_PLAYER_UNDERWATER_TIME_REMAINING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_UNDERWATER_TIME_REMAINING(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PLAYER_UNDERWATER_TIME_REMAINING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_underwater_time_remaining_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_underwater_time_remaining_raw(player: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_UNDERWATER_TIME_REMAINING(player)
}

/// ```
NativeDB Introduced: v1604
```

pub fn _0x70a382adec069dd3_safe(coordX: f32, coordY: f32, coordZ: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0x70A382ADEC069DD3");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x70A382ADEC069DD3(coordX, coordY, coordZ)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x70A382ADEC069DD3
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x70a382adec069dd3_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x70a382adec069dd3_raw(coordX: f32, coordY: f32, coordZ: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x70A382ADEC069DD3(coordX, coordY, coordZ)
}

/// ```
For Steam.
Always returns 0 in retail version of the game.
```

pub fn _get_achievement_progress_safe(achievement: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_ACHIEVEMENT_PROGRESS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_ACHIEVEMENT_PROGRESS(achievement)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_ACHIEVEMENT_PROGRESS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_achievement_progress_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_achievement_progress_raw(achievement: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_ACHIEVEMENT_PROGRESS(achievement)
}

/// ```
NativeDB Introduced: v2060
```

pub fn _set_wanted_level_hidden_evasion_time_safe(player: Player, wantedLevel: i64, lossTime: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_WANTED_LEVEL_HIDDEN_EVASION_TIME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_WANTED_LEVEL_HIDDEN_EVASION_TIME(player, wantedLevel, lossTime)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_WANTED_LEVEL_HIDDEN_EVASION_TIME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_wanted_level_hidden_evasion_time_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_wanted_level_hidden_evasion_time_raw(player: i32, wantedLevel: i64, lossTime: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_WANTED_LEVEL_HIDDEN_EVASION_TIME(player, wantedLevel, lossTime)
}

/// ```
NativeDB Added Parameter 2: Any p1
```

pub fn special_ability_reset_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_ABILITY_RESET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_ABILITY_RESET(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_ABILITY_RESET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_ability_reset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_ability_reset_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_ABILITY_RESET(player)
}

/// ## Parameters
* **player**:

pub fn get_time_since_player_hit_vehicle_safe(player: Player) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_TIME_SINCE_PLAYER_HIT_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_TIME_SINCE_PLAYER_HIT_VEHICLE(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_TIME_SINCE_PLAYER_HIT_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_time_since_player_hit_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_time_since_player_hit_vehicle_raw(player: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_TIME_SINCE_PLAYER_HIT_VEHICLE(player)
}

/// ## Return value

pub fn is_player_teleport_active_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_TELEPORT_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_TELEPORT_ACTIVE()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_TELEPORT_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_teleport_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_teleport_active_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_TELEPORT_ACTIVE()
}

/// ```
Gets the maximum wanted level the player can get.  
Ranges from 0 to 5.  
```

pub fn get_max_wanted_level_safe() -> NativeResult<i64> {
    
    debug!("Calling native function: GET_MAX_WANTED_LEVEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_MAX_WANTED_LEVEL()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_MAX_WANTED_LEVEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_max_wanted_level_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_max_wanted_level_raw() -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_MAX_WANTED_LEVEL()
}

/// ```
modifier's min value is 0.1
```

pub fn set_player_vehicle_damage_modifier_safe(player: Player, modifier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_VEHICLE_DAMAGE_MODIFIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_VEHICLE_DAMAGE_MODIFIER(player, modifier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_VEHICLE_DAMAGE_MODIFIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_vehicle_damage_modifier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_vehicle_damage_modifier_raw(player: i32, modifier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_VEHICLE_DAMAGE_MODIFIER(player, modifier)
}

/// ## Parameters
* **player**: 
* **tintIndex**:

pub fn get_player_parachute_pack_tint_index_safe(player: Player, tintIndex: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_PLAYER_PARACHUTE_PACK_TINT_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_PARACHUTE_PACK_TINT_INDEX(player, tintIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_PLAYER_PARACHUTE_PACK_TINT_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_parachute_pack_tint_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_parachute_pack_tint_index_raw(player: i32, tintIndex: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_PARACHUTE_PACK_TINT_INDEX(player, tintIndex)
}

/// ## Parameters
* **player**:

pub fn is_player_bluetooth_enable_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_BLUETOOTH_ENABLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_BLUETOOTH_ENABLE(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_BLUETOOTH_ENABLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_bluetooth_enable_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_bluetooth_enable_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_BLUETOOTH_ENABLE(player)
}

/// ## Parameters
* **player**:

pub fn reset_wanted_level_difficulty_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: RESET_WANTED_LEVEL_DIFFICULTY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_WANTED_LEVEL_DIFFICULTY(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_WANTED_LEVEL_DIFFICULTY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_wanted_level_difficulty_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_wanted_level_difficulty_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_WANTED_LEVEL_DIFFICULTY(player)
}

/// ```
IS_*
```

pub fn _0x690a61a6d13583f6_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: _0x690A61A6D13583F6");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x690A61A6D13583F6(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0x690A61A6D13583F6
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x690a61a6d13583f6_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x690a61a6d13583f6_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x690A61A6D13583F6(player)
}

/// ## Parameters
* **player**:

pub fn clear_player_has_damaged_at_least_one_non_animal_ped_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PLAYER_HAS_DAMAGED_AT_LEAST_ONE_NON_ANIMAL_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PLAYER_HAS_DAMAGED_AT_LEAST_ONE_NON_ANIMAL_PED(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PLAYER_HAS_DAMAGED_AT_LEAST_ONE_NON_ANIMAL_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_player_has_damaged_at_least_one_non_animal_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_player_has_damaged_at_least_one_non_animal_ped_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PLAYER_HAS_DAMAGED_AT_LEAST_ONE_NON_ANIMAL_PED(player)
}

/// ## Parameters
* **player**: 
* **p2**:

pub fn remove_player_helmet_safe(player: Player, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_PLAYER_HELMET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_PLAYER_HELMET(player, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_PLAYER_HELMET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_player_helmet_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_player_helmet_raw(player: i32, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_PLAYER_HELMET(player, p2)
}

/// ```
If toggle is set to false:
 The police won't be shown on the (mini)map
If toggle is set to true:
 The police will be shown on the (mini)map
```

pub fn set_police_radar_blips_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_POLICE_RADAR_BLIPS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_POLICE_RADAR_BLIPS(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_POLICE_RADAR_BLIPS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_police_radar_blips_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_police_radar_blips_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_POLICE_RADAR_BLIPS(toggle)
}

/// ## Parameters
* **player**: 
* **p1**:

pub fn set_player_cloth_pin_frames_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_CLOTH_PIN_FRAMES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_CLOTH_PIN_FRAMES(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_CLOTH_PIN_FRAMES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_cloth_pin_frames_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_cloth_pin_frames_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_CLOTH_PIN_FRAMES(player)
}

/// ```
Every occurrence of p1 & p2 were both true.
```

```
NativeDB Added Parameter 4: Any p3
```

pub fn special_ability_charge_small_safe(player: Player, p1: bool, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_ABILITY_CHARGE_SMALL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_ABILITY_CHARGE_SMALL(player, p1, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_ABILITY_CHARGE_SMALL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_ability_charge_small_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_ability_charge_small_raw(player: i32, p1: bool, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_ABILITY_CHARGE_SMALL(player, p1, p2)
}

/// ## Parameters
* **player**: 
* **wantedLevel**:

pub fn is_player_wanted_level_greater_safe(player: Player, wantedLevel: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_WANTED_LEVEL_GREATER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_WANTED_LEVEL_GREATER(player, wantedLevel)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_WANTED_LEVEL_GREATER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_wanted_level_greater_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_wanted_level_greater_raw(player: i32, wantedLevel: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_WANTED_LEVEL_GREATER(player, wantedLevel)
}

/// Inhibits the player from using any method of combat including melee and firearms.  
NOTE: Only disables the firing for one frame

pub fn disable_player_firing_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: DISABLE_PLAYER_FIRING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DISABLE_PLAYER_FIRING(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DISABLE_PLAYER_FIRING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `disable_player_firing_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn disable_player_firing_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DISABLE_PLAYER_FIRING(player, toggle)
}

/// ## Return value

pub fn is_system_ui_being_displayed_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: IS_SYSTEM_UI_BEING_DISPLAYED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_SYSTEM_UI_BEING_DISPLAYED()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_SYSTEM_UI_BEING_DISPLAYED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_system_ui_being_displayed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_system_ui_being_displayed_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_SYSTEM_UI_BEING_DISPLAYED()
}

/// This multiplier is reset to `1.0` every time the player ped is changed, often times via [`SET_PLAYER_MODEL`](#_0x00A1CADD00108836) or [`CHANGE_PLAYER_PED`](#_0x048189FAC643DEEE).

pub fn set_player_health_recharge_multiplier_safe(player: Player, regenRate: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_HEALTH_RECHARGE_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_HEALTH_RECHARGE_MULTIPLIER(player, regenRate)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_HEALTH_RECHARGE_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_health_recharge_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_health_recharge_multiplier_raw(player: i32, regenRate: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_HEALTH_RECHARGE_MULTIPLIER(player, regenRate)
}

/// ```
NativeDB Introduced: v2060
```

pub fn _0x823ec8e82ba45986_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x823EC8E82BA45986");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x823EC8E82BA45986(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x823EC8E82BA45986
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x823ec8e82ba45986_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x823ec8e82ba45986_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x823EC8E82BA45986(p0)
}

/// ```
used with 1,2,8,64,128 in the scripts  
```

pub fn force_cleanup_safe(cleanupFlags: i64) -> NativeResult<()> {
    
    debug!("Calling native function: FORCE_CLEANUP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::FORCE_CLEANUP(cleanupFlags)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: FORCE_CLEANUP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `force_cleanup_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn force_cleanup_raw(cleanupFlags: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::FORCE_CLEANUP(cleanupFlags)
}

/// ```
NativeDB Added Parameter 2: Any p1
```

pub fn special_ability_charge_on_mission_failed_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_ABILITY_CHARGE_ON_MISSION_FAILED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_ABILITY_CHARGE_ON_MISSION_FAILED(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_ABILITY_CHARGE_ON_MISSION_FAILED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_ability_charge_on_mission_failed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_ability_charge_on_mission_failed_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_ABILITY_CHARGE_ON_MISSION_FAILED(player)
}

/// ```
Assigns the handle of locked-on melee target to *entity that you pass it.  
Returns false if no entity found.  
```

pub fn get_player_target_entity_safe(player: Player, entity: *mut i32) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_PLAYER_TARGET_ENTITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_TARGET_ENTITY(player, entity)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PLAYER_TARGET_ENTITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_target_entity_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_target_entity_raw(player: i32, entity: *mut i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_TARGET_ENTITY(player, entity)
}

/// ```
NativeDB Introduced: v1604
```

pub fn _0x7148e0f43d11f0d9_safe() -> NativeResult<()> {
    
    debug!("Calling native function: _0x7148E0F43D11F0D9");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x7148E0F43D11F0D9()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x7148E0F43D11F0D9
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x7148e0f43d11f0d9_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x7148e0f43d11f0d9_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x7148E0F43D11F0D9()
}

/// ## Parameters
* **player**:

pub fn is_player_script_control_on_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_SCRIPT_CONTROL_ON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_SCRIPT_CONTROL_ON(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_SCRIPT_CONTROL_ON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_script_control_on_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_script_control_on_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_SCRIPT_CONTROL_ON(player)
}

/// ```
tints 0- 13
0 - unkown
1 - unkown
2 - unkown
3 - unkown
4 - unkown
```

pub fn set_player_parachute_pack_tint_index_safe(player: Player, tintIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_PARACHUTE_PACK_TINT_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_PARACHUTE_PACK_TINT_INDEX(player, tintIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_PARACHUTE_PACK_TINT_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_parachute_pack_tint_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_parachute_pack_tint_index_raw(player: i32, tintIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_PARACHUTE_PACK_TINT_INDEX(player, tintIndex)
}

/// ```
Returns TRUE if the player ('s ped) is climbing at the moment.  
```

pub fn is_player_climbing_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_CLIMBING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_CLIMBING(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_CLIMBING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_climbing_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_climbing_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_CLIMBING(player)
}

/// ## Parameters
* **player**:

pub fn get_player_sprint_time_remaining_safe(player: Player) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_PLAYER_SPRINT_TIME_REMAINING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_SPRINT_TIME_REMAINING(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PLAYER_SPRINT_TIME_REMAINING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_sprint_time_remaining_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_sprint_time_remaining_raw(player: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_SPRINT_TIME_REMAINING(player)
}

/// ```
p1 appears to always be 1 (only comes up twice)
```

```
NativeDB Added Parameter 3: Any p2
```

pub fn special_ability_charge_continuous_safe(player: Player, p2: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_ABILITY_CHARGE_CONTINUOUS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_ABILITY_CHARGE_CONTINUOUS(player, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_ABILITY_CHARGE_CONTINUOUS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_ability_charge_continuous_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_ability_charge_continuous_raw(player: i32, p2: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_ABILITY_CHARGE_CONTINUOUS(player, p2)
}

/// ```
2 matches. p1 was always true.
```

```
NativeDB Added Parameter 4: Any p3
```

pub fn special_ability_charge_large_safe(player: Player, p1: bool, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_ABILITY_CHARGE_LARGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_ABILITY_CHARGE_LARGE(player, p1, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_ABILITY_CHARGE_LARGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_ability_charge_large_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_ability_charge_large_raw(player: i32, p1: bool, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_ABILITY_CHARGE_LARGE(player, p1, p2)
}

/// ```
1.0.335.2, 1.0.350.1/2, 1.0.372.2, 1.0.393.2, 1.0.393.4, 1.0.463.1;  
```

pub fn player_detach_virtual_bound_safe() -> NativeResult<()> {
    
    debug!("Calling native function: PLAYER_DETACH_VIRTUAL_BOUND");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::PLAYER_DETACH_VIRTUAL_BOUND()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: PLAYER_DETACH_VIRTUAL_BOUND
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `player_detach_virtual_bound_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn player_detach_virtual_bound_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::PLAYER_DETACH_VIRTUAL_BOUND()
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn set_player_sprint_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_SPRINT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_SPRINT(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_SPRINT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_sprint_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_sprint_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_SPRINT(player, toggle)
}

/// ```
Flags:
SPC_AMBIENT_SCRIPT = (1 << 1),
SPC_CLEAR_TASKS = (1 << 2),
SPC_REMOVE_FIRES = (1 << 3),
SPC_REMOVE_EXPLOSIONS = (1 << 4),
SPC_REMOVE_PROJECTILES = (1 << 5),
SPC_DEACTIVATE_GADGETS = (1 << 6),
SPC_REENABLE_CONTROL_ON_DEATH = (1 << 7),
SPC_LEAVE_CAMERA_CONTROL_ON = (1 << 8),
SPC_ALLOW_PLAYER_DAMAGE = (1 << 9),
SPC_DONT_STOP_OTHER_CARS_AROUND_PLAYER = (1 << 10),
SPC_PREVENT_EVERYBODY_BACKOFF = (1 << 11),
SPC_ALLOW_PAD_SHAKE = (1 << 12)
See: https://alloc8or.re/gta5/doc/enums/eSetPlayerControlFlag.txt
```

pub fn set_player_control_safe(player: Player, bHasControl: bool, flags: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_CONTROL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_CONTROL(player, bHasControl, flags)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_CONTROL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_control_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_control_raw(player: i32, bHasControl: bool, flags: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_CONTROL(player, bHasControl, flags)
}

/// ```
The player will be ignored by the police if toggle is set to true  
```

pub fn set_police_ignore_player_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_POLICE_IGNORE_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_POLICE_IGNORE_PLAYER(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_POLICE_IGNORE_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_police_ignore_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_police_ignore_player_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_POLICE_IGNORE_PLAYER(player, toggle)
}

/// ## Parameters
* **targetLevel**:

pub fn set_player_target_level_safe(targetLevel: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_TARGET_LEVEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_TARGET_LEVEL(targetLevel)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_TARGET_LEVEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_target_level_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_target_level_raw(targetLevel: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_TARGET_LEVEL(targetLevel)
}

/// ```
Only 1 match. Both p1 & p2 were true.
```

```
NativeDB Added Parameter 4: Any p3
```

pub fn special_ability_charge_medium_safe(player: Player, p1: bool, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_ABILITY_CHARGE_MEDIUM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_ABILITY_CHARGE_MEDIUM(player, p1, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_ABILITY_CHARGE_MEDIUM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_ability_charge_medium_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_ability_charge_medium_raw(player: i32, p1: bool, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_ABILITY_CHARGE_MEDIUM(player, p1, p2)
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn set_player_force_skip_aim_intro_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_FORCE_SKIP_AIM_INTRO");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_FORCE_SKIP_AIM_INTRO(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_FORCE_SKIP_AIM_INTRO
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_force_skip_aim_intro_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_force_skip_aim_intro_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_FORCE_SKIP_AIM_INTRO(player, toggle)
}

/// ```
NativeDB Added Parameter 2: Any p1
```

pub fn is_special_ability_active_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_SPECIAL_ABILITY_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_SPECIAL_ABILITY_ACTIVE(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_SPECIAL_ABILITY_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_special_ability_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_special_ability_active_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_SPECIAL_ABILITY_ACTIVE(player)
}

/// ## Parameters
* **cleanupFlags**:

pub fn has_force_cleanup_occurred_safe(cleanupFlags: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_FORCE_CLEANUP_OCCURRED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_FORCE_CLEANUP_OCCURRED(cleanupFlags)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_FORCE_CLEANUP_OCCURRED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_force_cleanup_occurred_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_force_cleanup_occurred_raw(cleanupFlags: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_FORCE_CLEANUP_OCCURRED(cleanupFlags)
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn give_player_ragdoll_control_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: GIVE_PLAYER_RAGDOLL_CONTROL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GIVE_PLAYER_RAGDOLL_CONTROL(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GIVE_PLAYER_RAGDOLL_CONTROL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `give_player_ragdoll_control_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn give_player_ragdoll_control_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GIVE_PLAYER_RAGDOLL_CONTROL(player, toggle)
}

/// ```
Can the player control himself, used to disable controls for player for things like a cutscene.  
---  
You can't disable controls with this, use SET_PLAYER_CONTROL(...) for this.  
```

pub fn is_player_control_on_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_CONTROL_ON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_CONTROL_ON(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_CONTROL_ON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_control_on_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_control_on_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_CONTROL_ON(player)
}

/// ```
Swim speed multiplier.  
Multiplier goes up to 1.49  
Just call it one time, it is not required to be called once every tick. - Note copied from below native.  
Note: At least the IDA method if you change the max float multiplier from 1.5 it will change it for both this and RUN_SPRINT below. I say 1.5 as the function blrs if what you input is greater than or equal to 1.5 hence why it's 1.49 max default.  
```

pub fn set_swim_multiplier_for_player_safe(player: Player, multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SWIM_MULTIPLIER_FOR_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SWIM_MULTIPLIER_FOR_PLAYER(player, multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SWIM_MULTIPLIER_FOR_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_swim_multiplier_for_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_swim_multiplier_for_player_raw(player: i32, multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SWIM_MULTIPLIER_FOR_PLAYER(player, multiplier)
}

/// ## Parameters
* **player**: The local player ID.
* **limit**: A value between 0.0 and 1.0, 0.5 is default.

pub fn _set_player_health_recharge_limit_safe(player: Player, limit: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PLAYER_HEALTH_RECHARGE_LIMIT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PLAYER_HEALTH_RECHARGE_LIMIT(player, limit)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PLAYER_HEALTH_RECHARGE_LIMIT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_player_health_recharge_limit_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_player_health_recharge_limit_raw(player: i32, limit: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PLAYER_HEALTH_RECHARGE_LIMIT(player, limit)
}

/// Sets whether all random peds will run away from the player if they are agitated (threatened) (bool=true), or if they will stand their ground (bool=false).

pub fn set_all_random_peds_flee_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_ALL_RANDOM_PEDS_FLEE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_ALL_RANDOM_PEDS_FLEE(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_ALL_RANDOM_PEDS_FLEE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_all_random_peds_flee_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_all_random_peds_flee_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_ALL_RANDOM_PEDS_FLEE(player, toggle)
}

/// ```
- This is called after SET_ALL_RANDOM_PEDS_FLEE_THIS_FRAME
```

pub fn _0xc3376f42b1faccc6_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: _0xC3376F42B1FACCC6");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xC3376F42B1FACCC6(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xC3376F42B1FACCC6
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xc3376f42b1faccc6_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xc3376f42b1faccc6_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xC3376F42B1FACCC6(player)
}

/// ```
Every occurrence was either 0 or 2.  
```

pub fn set_player_cloth_package_index_safe(index: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_CLOTH_PACKAGE_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_CLOTH_PACKAGE_INDEX(index)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_CLOTH_PACKAGE_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_cloth_package_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_cloth_package_index_raw(index: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_CLOTH_PACKAGE_INDEX(index)
}

/// ## Parameters
* **player**:

pub fn has_player_damaged_at_least_one_non_animal_ped_safe(player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_PLAYER_DAMAGED_AT_LEAST_ONE_NON_ANIMAL_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_PLAYER_DAMAGED_AT_LEAST_ONE_NON_ANIMAL_PED(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_PLAYER_DAMAGED_AT_LEAST_ONE_NON_ANIMAL_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_player_damaged_at_least_one_non_animal_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_player_damaged_at_least_one_non_animal_ped_raw(player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_PLAYER_DAMAGED_AT_LEAST_ONE_NON_ANIMAL_PED(player)
}

/// ## Parameters
* **player**: the target player
* **wantedLevel**: the wanted level 1-5
* **delayedResponse**: false = 0-10sec police spawn response time, true = 10-20sec police spawn response time

pub fn set_player_wanted_level_no_drop_safe(player: Player, wantedLevel: i64, delayedResponse: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_WANTED_LEVEL_NO_DROP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_WANTED_LEVEL_NO_DROP(player, wantedLevel, delayedResponse)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_WANTED_LEVEL_NO_DROP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_wanted_level_no_drop_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_wanted_level_no_drop_raw(player: i32, wantedLevel: i64, delayedResponse: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_WANTED_LEVEL_NO_DROP(player, wantedLevel, delayedResponse)
}

/// ```
example:  
flags: 0-6  
PLAYER::SET_PLAYER_RESET_FLAG_PREFER_REAR_SEATS(PLAYER::PLAYER_ID(), 6);  
wouldnt the flag be the seatIndex?  
```

pub fn set_player_reset_flag_prefer_rear_seats_safe(player: Player, flags: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_RESET_FLAG_PREFER_REAR_SEATS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_RESET_FLAG_PREFER_REAR_SEATS(player, flags)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_RESET_FLAG_PREFER_REAR_SEATS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_reset_flag_prefer_rear_seats_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_reset_flag_prefer_rear_seats_raw(player: i32, flags: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_RESET_FLAG_PREFER_REAR_SEATS(player, flags)
}

/// ```
NativeDB Introduced: v1290
```

pub fn _0x7bae68775557ae0b_safe(p0: serde_json::Value, p1: serde_json::Value, p2: serde_json::Value, p3: serde_json::Value, p4: serde_json::Value, p5: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p1_any_str = serde_json::to_string(&p1)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p1", e)))?;
    let p1_any_str_cstr = std::ffi::CString::new(p1_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p1", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p2_any_str = serde_json::to_string(&p2)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p2", e)))?;
    let p2_any_str_cstr = std::ffi::CString::new(p2_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p2", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p3_any_str = serde_json::to_string(&p3)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p3", e)))?;
    let p3_any_str_cstr = std::ffi::CString::new(p3_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p3", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p4_any_str = serde_json::to_string(&p4)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p4", e)))?;
    let p4_any_str_cstr = std::ffi::CString::new(p4_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p4", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p5_any_str = serde_json::to_string(&p5)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p5", e)))?;
    let p5_any_str_cstr = std::ffi::CString::new(p5_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p5", e)))?;
    
    debug!("Calling native function: _0x7BAE68775557AE0B");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x7BAE68775557AE0B(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1), crate::utils::any_to_c_void_ptr(p2), crate::utils::any_to_c_void_ptr(p3), crate::utils::any_to_c_void_ptr(p4), crate::utils::any_to_c_void_ptr(p5))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x7BAE68775557AE0B
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x7bae68775557ae0b_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x7bae68775557ae0b_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void, p2: *mut std::os::raw::c_void, p3: *mut std::os::raw::c_void, p4: *mut std::os::raw::c_void, p5: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x7BAE68775557AE0B(p0, p1, p2, p3, p4, p5)
}

/// ```
Forces any pending wanted level to be applied to the specified player immediately.  
Call SET_PLAYER_WANTED_LEVEL with the desired wanted level, followed by SET_PLAYER_WANTED_LEVEL_NOW.  
Second parameter is unknown (always false).  
```

pub fn set_player_wanted_level_now_safe(player: Player, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_WANTED_LEVEL_NOW");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_WANTED_LEVEL_NOW(player, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_WANTED_LEVEL_NOW
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_wanted_level_now_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_wanted_level_now_raw(player: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_WANTED_LEVEL_NOW(player, p1)
}

/// ```
NativeDB Introduced: v2372
```

pub fn _get_wanted_level_parole_duration_safe() -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_WANTED_LEVEL_PAROLE_DURATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_WANTED_LEVEL_PAROLE_DURATION()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_WANTED_LEVEL_PAROLE_DURATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_wanted_level_parole_duration_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_wanted_level_parole_duration_raw() -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_WANTED_LEVEL_PAROLE_DURATION()
}

/// ```
SET_PLAYER_MAX_*
```

pub fn _0x8d768602adef2245_safe(player: Player, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0x8D768602ADEF2245");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x8D768602ADEF2245(player, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x8D768602ADEF2245
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x8d768602adef2245_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x8d768602adef2245_raw(player: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x8D768602ADEF2245(player, p1)
}

/// ```
p1 was always 5.  
p4 was always false.  
```

pub fn set_player_parachute_variation_override_safe(player: Player, p1: i64, p2: serde_json::Value, p3: serde_json::Value, p4: bool) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p2_any_str = serde_json::to_string(&p2)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p2", e)))?;
    let p2_any_str_cstr = std::ffi::CString::new(p2_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p2", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p3_any_str = serde_json::to_string(&p3)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p3", e)))?;
    let p3_any_str_cstr = std::ffi::CString::new(p3_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p3", e)))?;
    
    debug!("Calling native function: SET_PLAYER_PARACHUTE_VARIATION_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_PARACHUTE_VARIATION_OVERRIDE(player, p1, crate::utils::any_to_c_void_ptr(p2), crate::utils::any_to_c_void_ptr(p3), p4)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_PARACHUTE_VARIATION_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_parachute_variation_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_parachute_variation_override_raw(player: i32, p1: i64, p2: *mut std::os::raw::c_void, p3: *mut std::os::raw::c_void, p4: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_PARACHUTE_VARIATION_OVERRIDE(player, p1, p2, p3, p4)
}

/// ```
NativeDB Added Parameter 2: Any p1
```

pub fn _0xffee8fa29ab9a18e_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: _0xFFEE8FA29AB9A18E");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xFFEE8FA29AB9A18E(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xFFEE8FA29AB9A18E
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xffee8fa29ab9a18e_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xffee8fa29ab9a18e_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xFFEE8FA29AB9A18E(player)
}

/// ## Parameters
* **player**:

pub fn clear_player_has_damaged_at_least_one_ped_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PLAYER_HAS_DAMAGED_AT_LEAST_ONE_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PLAYER_HAS_DAMAGED_AT_LEAST_ONE_PED(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PLAYER_HAS_DAMAGED_AT_LEAST_ONE_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_player_has_damaged_at_least_one_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_player_has_damaged_at_least_one_ped_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PLAYER_HAS_DAMAGED_AT_LEAST_ONE_PED(player)
}

/// ```
NativeDB Introduced: v1180
```

pub fn _set_player_homing_rocket_disabled_safe(player: Player, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PLAYER_HOMING_ROCKET_DISABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PLAYER_HOMING_ROCKET_DISABLED(player, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PLAYER_HOMING_ROCKET_DISABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_player_homing_rocket_disabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_player_homing_rocket_disabled_raw(player: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PLAYER_HOMING_ROCKET_DISABLED(player, p1)
}

/// ```
NativeDB Added Parameter 3: Any p2
```

pub fn enable_special_ability_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: ENABLE_SPECIAL_ABILITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ENABLE_SPECIAL_ABILITY(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ENABLE_SPECIAL_ABILITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `enable_special_ability_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn enable_special_ability_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ENABLE_SPECIAL_ABILITY(player, toggle)
}

/// Suppresses a crime for a given player for this frame only.

pub fn suppress_crime_this_frame_safe(player: Player, crimeType: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SUPPRESS_CRIME_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SUPPRESS_CRIME_THIS_FRAME(player, crimeType)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SUPPRESS_CRIME_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `suppress_crime_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn suppress_crime_this_frame_raw(player: i32, crimeType: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SUPPRESS_CRIME_THIS_FRAME(player, crimeType)
}

/// ## Parameters
* **player**:

pub fn _get_player_health_recharge_limit_safe(player: Player) -> NativeResult<f32> {
    
    debug!("Calling native function: _GET_PLAYER_HEALTH_RECHARGE_LIMIT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_PLAYER_HEALTH_RECHARGE_LIMIT(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_PLAYER_HEALTH_RECHARGE_LIMIT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_player_health_recharge_limit_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_player_health_recharge_limit_raw(player: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_PLAYER_HEALTH_RECHARGE_LIMIT(player)
}

/// It returns true if the player is online, suggesting they are also logged in locally. Note that this is an alias for `NETWORK_IS_SIGNED_ONLINE`.

pub fn is_player_online_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYER_ONLINE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYER_ONLINE()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYER_ONLINE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_player_online_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_player_online_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYER_ONLINE()
}

/// ```
NativeDB Added Parameter 2: Any p1
```

pub fn special_ability_deactivate_safe(player: Player) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_ABILITY_DEACTIVATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_ABILITY_DEACTIVATE(player)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_ABILITY_DEACTIVATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_ability_deactivate_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_ability_deactivate_raw(player: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_ABILITY_DEACTIVATE(player)
}

/// ## Parameters
* **player**: 
* **toggle**:

pub fn _0xde45d1a1ef45ee61_safe(player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xDE45D1A1EF45EE61");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xDE45D1A1EF45EE61(player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xDE45D1A1EF45EE61
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xde45d1a1ef45ee61_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xde45d1a1ef45ee61_raw(player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xDE45D1A1EF45EE61(player, toggle)
}

/// ## Parameters
* **player**:

pub fn get_player_fake_wanted_level_safe(player: Player) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PLAYER_FAKE_WANTED_LEVEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_FAKE_WANTED_LEVEL(player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PLAYER_FAKE_WANTED_LEVEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_fake_wanted_level_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_fake_wanted_level_raw(player: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_FAKE_WANTED_LEVEL(player)
}

/// ```
Disables the player's teleportation  
```

pub fn stop_player_teleport_safe() -> NativeResult<()> {
    
    debug!("Calling native function: STOP_PLAYER_TELEPORT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::STOP_PLAYER_TELEPORT()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: STOP_PLAYER_TELEPORT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `stop_player_teleport_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn stop_player_teleport_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::STOP_PLAYER_TELEPORT()
}

/// ```
Does the same like PLAYER::GET_PLAYER_PED
```

pub fn get_player_ped_script_index_safe(player: Player) -> NativeResult<Ped> {
    
    debug!("Calling native function: GET_PLAYER_PED_SCRIPT_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_PED_SCRIPT_INDEX(player)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: GET_PLAYER_PED_SCRIPT_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_ped_script_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_ped_script_index_raw(player: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_PED_SCRIPT_INDEX(player)
}

/// ```
Tints:  
None = -1,  
Rainbow = 0,  
Red = 1,  
SeasideStripes = 2,  
WidowMaker = 3,  
Patriot = 4,  
Blue = 5,  
Black = 6,  
Hornet = 7,  
AirFocce = 8,  
Desert = 9,  
Shadow = 10,  
HighAltitude = 11,  
Airbone = 12,  
Sunrise = 13,  
```

pub fn set_player_parachute_tint_index_safe(player: Player, tintIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_PARACHUTE_TINT_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_PARACHUTE_TINT_INDEX(player, tintIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_PARACHUTE_TINT_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_parachute_tint_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_parachute_tint_index_raw(player: i32, tintIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_PARACHUTE_TINT_INDEX(player, tintIndex)
}

/// Limit the player to only enter this vehicle. Note set vehicle to false if you want them to access any vehicle.

pub fn set_player_may_only_enter_this_vehicle_safe(player: Player, vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYER_MAY_ONLY_ENTER_THIS_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYER_MAY_ONLY_ENTER_THIS_VEHICLE(player, vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYER_MAY_ONLY_ENTER_THIS_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_player_may_only_enter_this_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_player_may_only_enter_this_vehicle_raw(player: i32, vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYER_MAY_ONLY_ENTER_THIS_VEHICLE(player, vehicle)
}

