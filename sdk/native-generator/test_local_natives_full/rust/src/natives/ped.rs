//! PED native functions
//!
//! Functions for the ped category

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
* **ped**: 
* **modelHash**:

pub fn is_ped_model_safe(ped: Ped, modelHash: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_MODEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_MODEL(ped, modelHash)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_MODEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_model_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_model_raw(ped: i32, modelHash: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_MODEL(ped, modelHash)
}

/// ## Return value

pub fn is_pedheadshot_img_upload_available_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PEDHEADSHOT_IMG_UPLOAD_AVAILABLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PEDHEADSHOT_IMG_UPLOAD_AVAILABLE()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PEDHEADSHOT_IMG_UPLOAD_AVAILABLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_pedheadshot_img_upload_available_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_pedheadshot_img_upload_available_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PEDHEADSHOT_IMG_UPLOAD_AVAILABLE()
}

/// ```
returns whether or not a ped is visible within your FOV, not this check auto's to false after a certain distance.  
Target needs to be tracked.. won't work otherwise.  
```

pub fn is_tracked_ped_visible_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_TRACKED_PED_VISIBLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_TRACKED_PED_VISIBLE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_TRACKED_PED_VISIBLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_tracked_ped_visible_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_tracked_ped_visible_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_TRACKED_PED_VISIBLE(ped)
}

/// **This native does absolutely nothing, just a nullsub**

pub fn _0xb282749d5e028163_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0xB282749D5E028163");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xB282749D5E028163(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xB282749D5E028163
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xb282749d5e028163_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xb282749d5e028163_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xB282749D5E028163(p0, p1)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**:

pub fn set_ped_defensive_area_direction_safe(ped: Ped, p1: f32, p2: f32, p3: f32, p4: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_DEFENSIVE_AREA_DIRECTION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_DEFENSIVE_AREA_DIRECTION(ped, p1, p2, p3, p4)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_DEFENSIVE_AREA_DIRECTION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_defensive_area_direction_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_defensive_area_direction_raw(ped: i32, p1: f32, p2: f32, p3: f32, p4: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_DEFENSIVE_AREA_DIRECTION(ped, p1, p2, p3, p4)
}

/// ## Parameters
* **ped**:

pub fn is_ped_getting_into_a_vehicle_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_GETTING_INTO_A_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_GETTING_INTO_A_VEHICLE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_GETTING_INTO_A_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_getting_into_a_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_getting_into_a_vehicle_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_GETTING_INTO_A_VEHICLE(ped)
}

/// ```
SET_PED_ALLOW*
toggle was always false except in one instance (b678).
The one time this is set to true seems to do with when you fail the mission.
```

pub fn _0xf2bebcdfafdaa19e_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xF2BEBCDFAFDAA19E");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xF2BEBCDFAFDAA19E(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xF2BEBCDFAFDAA19E
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xf2bebcdfafdaa19e_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xf2bebcdfafdaa19e_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xF2BEBCDFAFDAA19E(toggle)
}

/// Ped no longer takes critical damage modifiers if set to FALSE.

Example: Headshotting a player no longer one shots them. Instead they will take the same damage as a torso shot.

pub fn set_ped_suffers_critical_hits_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_SUFFERS_CRITICAL_HITS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_SUFFERS_CRITICAL_HITS(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_SUFFERS_CRITICAL_HITS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_suffers_critical_hits_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_suffers_critical_hits_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_SUFFERS_CRITICAL_HITS(ped, toggle)
}

/// ```
Function just returns 0  
void __fastcall ped__get_mount(NativeContext *a1)  
{  
  NativeContext *v1; // rbx@1  
  v1 = a1;  
  GetAddressOfPedFromScriptHandle(a1->Args->Arg1);  
  v1->Returns->Item1= 0;  
}  
```

pub fn get_mount_safe(ped: Ped) -> NativeResult<Ped> {
    
    debug!("Calling native function: GET_MOUNT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_MOUNT(ped)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: GET_MOUNT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_mount_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_mount_raw(ped: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_MOUNT(ped)
}

/// ## Parameters
* **ped**:

pub fn set_ped_should_play_immediate_scenario_exit_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_SHOULD_PLAY_IMMEDIATE_SCENARIO_EXIT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_SHOULD_PLAY_IMMEDIATE_SCENARIO_EXIT(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_SHOULD_PLAY_IMMEDIATE_SCENARIO_EXIT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_should_play_immediate_scenario_exit_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_should_play_immediate_scenario_exit_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_SHOULD_PLAY_IMMEDIATE_SCENARIO_EXIT(ped)
}

/// ## Parameters
* **ped**: 
* **value**:

pub fn set_ped_visual_field_max_angle_safe(ped: Ped, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_VISUAL_FIELD_MAX_ANGLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_VISUAL_FIELD_MAX_ANGLE(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_VISUAL_FIELD_MAX_ANGLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_visual_field_max_angle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_visual_field_max_angle_raw(ped: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_VISUAL_FIELD_MAX_ANGLE(ped, value)
}

/// ## Parameters
* **ped**: 
* **speedX**: 
* **speedY**:

pub fn _get_ped_current_movement_speed_safe(ped: Ped, speedX: *mut f32, speedY: *mut f32) -> NativeResult<bool> {
    
    debug!("Calling native function: _GET_PED_CURRENT_MOVEMENT_SPEED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_PED_CURRENT_MOVEMENT_SPEED(ped, speedX, speedY)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_PED_CURRENT_MOVEMENT_SPEED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_ped_current_movement_speed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_ped_current_movement_speed_raw(ped: i32, speedX: *mut f32, speedY: *mut f32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_PED_CURRENT_MOVEMENT_SPEED(ped, speedX, speedY)
}

/// ## Parameters
* **ped**: 
* **collection**: 
* **overlay**:

pub fn add_ped_decoration_from_hashes_in_corona_safe(ped: Ped, collection: u32, overlay: u32) -> NativeResult<()> {
    
    debug!("Calling native function: ADD_PED_DECORATION_FROM_HASHES_IN_CORONA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ADD_PED_DECORATION_FROM_HASHES_IN_CORONA(ped, collection, overlay)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ADD_PED_DECORATION_FROM_HASHES_IN_CORONA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `add_ped_decoration_from_hashes_in_corona_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn add_ped_decoration_from_hashes_in_corona_raw(ped: i32, collection: u32, overlay: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ADD_PED_DECORATION_FROM_HASHES_IN_CORONA(ped, collection, overlay)
}

/// ```
Only called once in the scripts:
if (sub_1abd() && (!PED::_A3F3564A5B3646C0(l_8C))) {
    if (sub_52e3("RESNA_CELLR", 0)) {
        PED::SET_PED_CAN_PLAY_GESTURE_ANIMS(l_8C, 1);
        PED::SET_PED_CAN_PLAY_AMBIENT_ANIMS(l_8C, 1);
        PED::SET_PED_CAN_PLAY_VISEME_ANIMS(l_8C, 1, 0);
        l_184 += 1;
    }
}
Checks something related to the mobile phone task.
IS_*
```

pub fn _0xa3f3564a5b3646c0_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: _0xA3F3564A5B3646C0");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xA3F3564A5B3646C0(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0xA3F3564A5B3646C0
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xa3f3564a5b3646c0_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xa3f3564a5b3646c0_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xA3F3564A5B3646C0(ped)
}

/// ## Parameters
* **ped**:

pub fn is_ped_taking_off_helmet_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_TAKING_OFF_HELMET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_TAKING_OFF_HELMET(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_TAKING_OFF_HELMET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_taking_off_helmet_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_taking_off_helmet_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_TAKING_OFF_HELMET(ped)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn _set_ped_can_play_injured_anims_safe(ped: Ped, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PED_CAN_PLAY_INJURED_ANIMS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PED_CAN_PLAY_INJURED_ANIMS(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PED_CAN_PLAY_INJURED_ANIMS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_ped_can_play_injured_anims_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_ped_can_play_injured_anims_raw(ped: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PED_CAN_PLAY_INJURED_ANIMS(ped, p1)
}

/// ```
NativeDB Added Parameter 5: Any p4
```

pub fn set_ped_vehicle_forced_seat_usage_safe(ped: Ped, vehicle: Vehicle, seatIndex: i64, flags: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_VEHICLE_FORCED_SEAT_USAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_VEHICLE_FORCED_SEAT_USAGE(ped, vehicle, seatIndex, flags)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_VEHICLE_FORCED_SEAT_USAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_vehicle_forced_seat_usage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_vehicle_forced_seat_usage_raw(ped: i32, vehicle: i32, seatIndex: i64, flags: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_VEHICLE_FORCED_SEAT_USAGE(ped, vehicle, seatIndex, flags)
}

/// ## Parameters
* **ped**:

pub fn get_player_ped_is_following_safe(ped: Ped) -> NativeResult<Player> {
    
    debug!("Calling native function: GET_PLAYER_PED_IS_FOLLOWING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PLAYER_PED_IS_FOLLOWING(ped)
    };
    
    
    Ok(Ok(Player(result)))
}

/// Raw native function: GET_PLAYER_PED_IS_FOLLOWING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_player_ped_is_following_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_player_ped_is_following_raw(ped: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PLAYER_PED_IS_FOLLOWING(ped)
}

/// ```
Checks if the specified unknown flag is set in the ped's model.  
The engine itself seems to exclusively check for flags 1 and 4 (Might be inlined code of the check that checks for other flags).  
Game scripts exclusively check for flags 1 and 4.  
```

pub fn _0x46b05bcae43856b0_safe(ped: Ped, flag: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _0x46B05BCAE43856B0");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x46B05BCAE43856B0(ped, flag)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0x46B05BCAE43856B0
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x46b05bcae43856b0_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x46b05bcae43856b0_raw(ped: i32, flag: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x46B05BCAE43856B0(ped, flag)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_steers_around_peds_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_STEERS_AROUND_PEDS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_STEERS_AROUND_PEDS(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_STEERS_AROUND_PEDS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_steers_around_peds_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_steers_around_peds_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_STEERS_AROUND_PEDS(ped, toggle)
}

/// ```
Returns the Entity (Ped, Vehicle, or ?Object?) that killed the 'ped'  
Is best to check if the Ped is dead before asking for its killer.  
```

pub fn get_ped_source_of_death_safe(ped: Ped) -> NativeResult<Entity> {
    
    debug!("Calling native function: GET_PED_SOURCE_OF_DEATH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_SOURCE_OF_DEATH(ped)
    };
    
    
    Ok(Ok(Entity(result)))
}

/// Raw native function: GET_PED_SOURCE_OF_DEATH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_source_of_death_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_source_of_death_raw(ped: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_SOURCE_OF_DEATH(ped)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**: 
* **p5**:

pub fn set_ped_bounds_orientation_safe(ped: Ped, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_BOUNDS_ORIENTATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_BOUNDS_ORIENTATION(ped, p1, p2, p3, p4, p5)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_BOUNDS_ORIENTATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_bounds_orientation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_bounds_orientation_raw(ped: i32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_BOUNDS_ORIENTATION(ped, p1, p2, p3, p4, p5)
}

/// ```
shootRate 0-1000  
```

pub fn set_ped_shoot_rate_safe(ped: Ped, shootRate: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_SHOOT_RATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_SHOOT_RATE(ped, shootRate)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_SHOOT_RATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_shoot_rate_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_shoot_rate_raw(ped: i32, shootRate: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_SHOOT_RATE(ped, shootRate)
}

/// ```
100 would equal attack  
less then 50ish would mean run away  
Only the values 0, 1 and 2 occur in the decompiled scripts. Most likely refers directly to the values also described in combatbehaviour.meta:  
0: CA_Poor  
1: CA_Average  
2: CA_Professional  
Tested this and got the same results as the first explanation here. Could not find any difference between 0, 1 and 2.  
```

pub fn set_ped_combat_ability_safe(ped: Ped, p1: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_COMBAT_ABILITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_COMBAT_ABILITY(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_COMBAT_ABILITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_combat_ability_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_combat_ability_raw(ped: i32, p1: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_COMBAT_ABILITY(ped, p1)
}

/// ## Parameters
* **ped**: 
* **vehicle**: 
* **seatIndex**: See eSeatPosition declared in [`IS_VEHICLE_SEAT_FREE`](#_0x22AC59A870E6A669). -2 for the first available seat.

pub fn set_ped_into_vehicle_safe(ped: Ped, vehicle: Vehicle, seatIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_INTO_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_INTO_VEHICLE(ped, vehicle, seatIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_INTO_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_into_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_into_vehicle_raw(ped: i32, vehicle: i32, seatIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_INTO_VEHICLE(ped, vehicle, seatIndex)
}

/// ## Parameters
* **groupId**:

pub fn does_group_exist_safe(groupId: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: DOES_GROUP_EXIST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DOES_GROUP_EXIST(groupId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: DOES_GROUP_EXIST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `does_group_exist_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn does_group_exist_raw(groupId: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DOES_GROUP_EXIST(groupId)
}

/// ## Return value

pub fn spawnpoints_is_search_failed_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: SPAWNPOINTS_IS_SEARCH_FAILED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPAWNPOINTS_IS_SEARCH_FAILED()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: SPAWNPOINTS_IS_SEARCH_FAILED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `spawnpoints_is_search_failed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn spawnpoints_is_search_failed_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPAWNPOINTS_IS_SEARCH_FAILED()
}

/// Clipsets:
"facials@gen_female@base"
"facials@gen_male@base"
"facials@p_m_zero@base"

Typically followed with [SET_FACIAL_IDLE_ANIM_OVERRIDE](#_0xFFC24B988B938B38):
"mood_drunk_1"
"mood_stressed_1"
"mood_happy_1"
"mood_talking_1"

```
NativeDB Introduced: v1493
```

pub fn _set_facial_clipset_override_safe(ped: Ped, animDict: String) -> NativeResult<()> {
    let animDict_cstr = std::ffi::CString::new(animDict.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animDict", e)))?;
    
    debug!("Calling native function: _SET_FACIAL_CLIPSET_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_FACIAL_CLIPSET_OVERRIDE(ped, crate::utils::rust_to_c_string(animDict))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_FACIAL_CLIPSET_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_facial_clipset_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_facial_clipset_override_raw(ped: i32, animDict: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_FACIAL_CLIPSET_OVERRIDE(ped, animDict)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_play_gesture_anims_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_PLAY_GESTURE_ANIMS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_PLAY_GESTURE_ANIMS(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_PLAY_GESTURE_ANIMS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_play_gesture_anims_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_play_gesture_anims_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_PLAY_GESTURE_ANIMS(ped, toggle)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_be_targeted_when_injured_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_BE_TARGETED_WHEN_INJURED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_BE_TARGETED_WHEN_INJURED(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_BE_TARGETED_WHEN_INJURED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_be_targeted_when_injured_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_be_targeted_when_injured_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_BE_TARGETED_WHEN_INJURED(ped, toggle)
}

/// Allows marine animals to survive outside of water (R* is using it for sharks).

```
NativeDB Introduced: v3407
```

pub fn _set_ped_survives_being_out_of_water_safe(ped: Ped, toggle: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: _SET_PED_SURVIVES_BEING_OUT_OF_WATER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PED_SURVIVES_BEING_OUT_OF_WATER(ped, toggle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _SET_PED_SURVIVES_BEING_OUT_OF_WATER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_ped_survives_being_out_of_water_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_ped_survives_being_out_of_water_raw(ped: i32, toggle: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PED_SURVIVES_BEING_OUT_OF_WATER(ped, toggle)
}

/// ## Parameters
* **ped**: The ped handle we're setting this for.
* **value**: The new max health.

pub fn set_ped_max_health_safe(ped: Ped, value: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_MAX_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_MAX_HEALTH(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_MAX_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_max_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_max_health_raw(ped: i32, value: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_MAX_HEALTH(ped, value)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x2f3c3d9f50681de4_safe(p0: serde_json::Value, p1: bool) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x2F3C3D9F50681DE4");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x2F3C3D9F50681DE4(crate::utils::any_to_c_void_ptr(p0), p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x2F3C3D9F50681DE4
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x2f3c3d9f50681de4_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x2f3c3d9f50681de4_raw(p0: *mut std::os::raw::c_void, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x2F3C3D9F50681DE4(p0, p1)
}

/// ## Parameters
* **ped1**: 
* **ped2**:

pub fn can_ped_see_hated_ped_safe(ped1: Ped, ped2: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: CAN_PED_SEE_HATED_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CAN_PED_SEE_HATED_PED(ped1, ped2)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CAN_PED_SEE_HATED_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `can_ped_see_hated_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn can_ped_see_hated_ped_raw(ped1: i32, ped2: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CAN_PED_SEE_HATED_PED(ped1, ped2)
}

/// ## Parameters
* **ped**:

pub fn _is_ped_helmet_unk_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PED_HELMET_UNK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PED_HELMET_UNK(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PED_HELMET_UNK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_ped_helmet_unk_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_ped_helmet_unk_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PED_HELMET_UNK(ped)
}

/// ## Parameters
* **ped**: 
* **target**: 
* **xOffset**: 
* **yOffset**: 
* **zOffset**: 
* **radius**: 
* **p6**:

pub fn set_ped_defensive_sphere_attached_to_vehicle_safe(ped: Ped, target: Vehicle, xOffset: f32, yOffset: f32, zOffset: f32, radius: f32, p6: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_DEFENSIVE_SPHERE_ATTACHED_TO_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_DEFENSIVE_SPHERE_ATTACHED_TO_VEHICLE(ped, target, xOffset, yOffset, zOffset, radius, p6)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_DEFENSIVE_SPHERE_ATTACHED_TO_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_defensive_sphere_attached_to_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_defensive_sphere_attached_to_vehicle_raw(ped: i32, target: i32, xOffset: f32, yOffset: f32, zOffset: f32, radius: f32, p6: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_DEFENSIVE_SPHERE_ATTACHED_TO_VEHICLE(ped, target, xOffset, yOffset, zOffset, radius, p6)
}

/// Applies blood damage to a ped with specific parameters for zone, UV offsets, rotation, scale, and initial aging.

```
NativeDB Introduced: v323
```

pub fn apply_ped_blood_specific_safe(ped: Ped, component: i64, u: f32, v: f32, rotation: f32, scale: f32, forcedFrame: i64, preAge: f32) -> NativeResult<()> {
    
    debug!("Calling native function: APPLY_PED_BLOOD_SPECIFIC");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::APPLY_PED_BLOOD_SPECIFIC(ped, component, u, v, rotation, scale, forcedFrame, preAge)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: APPLY_PED_BLOOD_SPECIFIC
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `apply_ped_blood_specific_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn apply_ped_blood_specific_raw(ped: i32, component: i64, u: f32, v: f32, rotation: f32, scale: f32, forcedFrame: i64, preAge: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::APPLY_PED_BLOOD_SPECIFIC(ped, component, u, v, rotation, scale, forcedFrame, preAge)
}

/// [Animations list](https://alexguirre.github.io/animations-list/)

pub fn get_anim_initial_offset_position_safe(animDict: String, animName: String, x: f32, y: f32, z: f32, xRot: f32, yRot: f32, zRot: f32, p8: f32, p9: i64) -> NativeResult<Vector3> {
    let animDict_cstr = std::ffi::CString::new(animDict.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animDict", e)))?;
    let animName_cstr = std::ffi::CString::new(animName.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animName", e)))?;
    
    debug!("Calling native function: GET_ANIM_INITIAL_OFFSET_POSITION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_ANIM_INITIAL_OFFSET_POSITION(crate::utils::rust_to_c_string(animDict), crate::utils::rust_to_c_string(animName), x, y, z, xRot, yRot, zRot, p8, p9)
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: GET_ANIM_INITIAL_OFFSET_POSITION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_anim_initial_offset_position_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_anim_initial_offset_position_raw(animDict: *const std::os::raw::c_char, animName: *const std::os::raw::c_char, x: f32, y: f32, z: f32, xRot: f32, yRot: f32, zRot: f32, p8: f32, p9: i64) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_ANIM_INITIAL_OFFSET_POSITION(animDict, animName, x, y, z, xRot, yRot, zRot, p8, p9)
}

/// This native is used to set prop variation on a ped. Components, drawables and textures IDs are related to the ped model.

pub fn set_ped_prop_index_safe(ped: Ped, componentId: i64, drawableId: i64, textureId: i64, attach: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_PROP_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_PROP_INDEX(ped, componentId, drawableId, textureId, attach)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_PROP_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_prop_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_prop_index_raw(ped: i32, componentId: i64, drawableId: i64, textureId: i64, attach: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_PROP_INDEX(ped, componentId, drawableId, textureId, attach)
}

/// ## Parameters
* **ped**: The ped handle.
* **componentId**: The component id to get the texture variation from. Refer to [SET_PED_COMPONENT_VARIATION](#_0x262B14F48D29DE80).

pub fn get_ped_texture_variation_safe(ped: Ped, componentId: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_TEXTURE_VARIATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_TEXTURE_VARIATION(ped, componentId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_TEXTURE_VARIATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_texture_variation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_texture_variation_raw(ped: i32, componentId: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_TEXTURE_VARIATION(ped, componentId)
}

/// ```
Sets the range at which members will automatically leave the group.  
```

pub fn set_group_separation_range_safe(groupHandle: i64, separationRange: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_GROUP_SEPARATION_RANGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_GROUP_SEPARATION_RANGE(groupHandle, separationRange)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_GROUP_SEPARATION_RANGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_group_separation_range_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_group_separation_range_raw(groupHandle: i64, separationRange: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_GROUP_SEPARATION_RANGE(groupHandle, separationRange)
}

/// ## Parameters
* **ped**: 
* **x**: 
* **y**: 
* **z**: 
* **p4**:

pub fn is_ped_heading_towards_position_safe(ped: Ped, x: f32, y: f32, z: f32, p4: f32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_HEADING_TOWARDS_POSITION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_HEADING_TOWARDS_POSITION(ped, x, y, z, p4)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_HEADING_TOWARDS_POSITION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_heading_towards_position_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_heading_towards_position_raw(ped: i32, x: f32, y: f32, z: f32, p4: f32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_HEADING_TOWARDS_POSITION(ped, x, y, z, p4)
}

/// ```
The pointer is to a padded struct that matches the arguments to SET_PED_HEAD_BLEND_DATA(...). There are 4 bytes of padding after each field.  
pass this struct in the second parameter   
typedef struct  
{  
        int shapeFirst, shapeSecond, shapeThird;   
        int skinFirst, skinSecond, skinThird;   
	float shapeMix, skinMix, thirdMix;  
} headBlendData;  
```

pub fn get_ped_head_blend_data_safe(ped: Ped, headBlendData: serde_json::Value) -> NativeResult<bool> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let headBlendData_any_str = serde_json::to_string(&headBlendData)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "headBlendData", e)))?;
    let headBlendData_any_str_cstr = std::ffi::CString::new(headBlendData_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "headBlendData", e)))?;
    
    debug!("Calling native function: GET_PED_HEAD_BLEND_DATA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_HEAD_BLEND_DATA(ped, crate::utils::any_to_c_void_ptr(headBlendData))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_HEAD_BLEND_DATA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_head_blend_data_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_head_blend_data_raw(ped: i32, headBlendData: *mut std::os::raw::c_void) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_HEAD_BLEND_DATA(ped, headBlendData)
}

/// ## Parameters
* **ped**:

pub fn set_ped_increased_avoidance_radius_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_INCREASED_AVOIDANCE_RADIUS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_INCREASED_AVOIDANCE_RADIUS(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_INCREASED_AVOIDANCE_RADIUS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_increased_avoidance_radius_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_increased_avoidance_radius_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_INCREASED_AVOIDANCE_RADIUS(ped)
}

/// ## Parameters
* **ped**:

pub fn drop_ambient_prop_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: DROP_AMBIENT_PROP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DROP_AMBIENT_PROP(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DROP_AMBIENT_PROP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `drop_ambient_prop_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn drop_ambient_prop_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DROP_AMBIENT_PROP(ped)
}

/// ## Parameters
* **ped**:

pub fn get_ped_max_health_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_MAX_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_MAX_HEALTH(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_MAX_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_max_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_max_health_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_MAX_HEALTH(ped)
}

/// ```
vb.net
Dim ped_handle As Integer
                    With Game.Player.Character
                        Dim pos As Vector3 = .Position + .ForwardVector * 3
                        ped_handle = Native.Function.Call(Of Integer)(Hash.CREATE_RANDOM_PED, pos.X, pos.Y, pos.Z)
                    End With
Creates a Ped at the specified location, returns the Ped Handle.
Ped will not act until SET_PED_AS_NO_LONGER_NEEDED is called.
```

pub fn create_random_ped_safe(posX: f32, posY: f32, posZ: f32) -> NativeResult<Ped> {
    
    debug!("Calling native function: CREATE_RANDOM_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CREATE_RANDOM_PED(posX, posY, posZ)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: CREATE_RANDOM_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `create_random_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn create_random_ped_raw(posX: f32, posY: f32, posZ: f32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CREATE_RANDOM_PED(posX, posY, posZ)
}

/// ```
Enable/disable ped shadow (ambient occlusion). https://gfycat.com/thankfulesteemedgecko
```

pub fn set_ped_ao_blob_rendering_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_AO_BLOB_RENDERING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_AO_BLOB_RENDERING(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_AO_BLOB_RENDERING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_ao_blob_rendering_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_ao_blob_rendering_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_AO_BLOB_RENDERING(ped, toggle)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_dies_in_sinking_vehicle_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_DIES_IN_SINKING_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_DIES_IN_SINKING_VEHICLE(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_DIES_IN_SINKING_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_dies_in_sinking_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_dies_in_sinking_vehicle_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_DIES_IN_SINKING_VEHICLE(ped, toggle)
}

/// Preview: https://gfycat.com/MaleRareAmazonparrot

pub fn set_head_blend_palette_color_safe(ped: Ped, r: i64, g: i64, b: i64, id: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_HEAD_BLEND_PALETTE_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_HEAD_BLEND_PALETTE_COLOR(ped, r, g, b, id)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_HEAD_BLEND_PALETTE_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_head_blend_palette_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_head_blend_palette_color_raw(ped: i32, r: i64, g: i64, b: i64, id: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_HEAD_BLEND_PALETTE_COLOR(ped, r, g, b, id)
}

/// ## Parameters
* **ped**:

pub fn was_ped_knocked_out_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: WAS_PED_KNOCKED_OUT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::WAS_PED_KNOCKED_OUT(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: WAS_PED_KNOCKED_OUT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `was_ped_knocked_out_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn was_ped_knocked_out_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::WAS_PED_KNOCKED_OUT(ped)
}

/// ## Parameters
* **ped**:

pub fn is_ped_being_jacked_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_BEING_JACKED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_BEING_JACKED(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_BEING_JACKED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_being_jacked_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_being_jacked_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_BEING_JACKED(ped)
}

/// Verifies whether ped was eliminated through stealth.

pub fn was_ped_killed_by_stealth_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: WAS_PED_KILLED_BY_STEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::WAS_PED_KILLED_BY_STEALTH(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: WAS_PED_KILLED_BY_STEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `was_ped_killed_by_stealth_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn was_ped_killed_by_stealth_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::WAS_PED_KILLED_BY_STEALTH(ped)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_motion_blur_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_MOTION_BLUR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_MOTION_BLUR(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_MOTION_BLUR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_motion_blur_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_motion_blur_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_MOTION_BLUR(ped, toggle)
}

/// ```
Deletes the specified ped, then sets the handle pointed to by the pointer to NULL.  
```

pub fn delete_ped_safe(ped: *mut i32) -> NativeResult<()> {
    
    debug!("Calling native function: DELETE_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DELETE_PED(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DELETE_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `delete_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn delete_ped_raw(ped: *mut i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DELETE_PED(ped)
}

/// ## Parameters
* **ped**: The ped handle.
* **componentId**: The component id to get the prop index from. Refer to [SET_PED_COMPONENT_VARIATION](#_0x262B14F48D29DE80).

pub fn get_ped_prop_index_safe(ped: Ped, componentId: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_PROP_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_PROP_INDEX(ped, componentId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_PROP_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_prop_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_prop_index_raw(ped: i32, componentId: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_PROP_INDEX(ped, componentId)
}

/// ```
Returns true if the ped passed through the parenthesis is wearing a helmet.  
```

pub fn is_ped_wearing_helmet_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_WEARING_HELMET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_WEARING_HELMET(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_WEARING_HELMET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_wearing_helmet_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_wearing_helmet_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_WEARING_HELMET(ped)
}

/// ```
IS_PED_*

Returns true if the ped is currently opening a door (CTaskOpenDoor).
```

pub fn _is_ped_opening_a_door_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PED_OPENING_A_DOOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PED_OPENING_A_DOOR(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PED_OPENING_A_DOOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_ped_opening_a_door_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_ped_opening_a_door_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PED_OPENING_A_DOOR(ped)
}

/// ## Parameters
* **modifier**:

pub fn set_ai_melee_weapon_damage_modifier_safe(modifier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_AI_MELEE_WEAPON_DAMAGE_MODIFIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_AI_MELEE_WEAPON_DAMAGE_MODIFIER(modifier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_AI_MELEE_WEAPON_DAMAGE_MODIFIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ai_melee_weapon_damage_modifier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ai_melee_weapon_damage_modifier_raw(modifier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_AI_MELEE_WEAPON_DAMAGE_MODIFIER(modifier)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_stay_in_vehicle_when_jacked_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_STAY_IN_VEHICLE_WHEN_JACKED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_STAY_IN_VEHICLE_WHEN_JACKED(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_STAY_IN_VEHICLE_WHEN_JACKED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_stay_in_vehicle_when_jacked_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_stay_in_vehicle_when_jacked_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_STAY_IN_VEHICLE_WHEN_JACKED(ped, toggle)
}

/// ## Parameters
* **ped**:

pub fn _is_ped_shader_effect_valid_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PED_SHADER_EFFECT_VALID");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PED_SHADER_EFFECT_VALID(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PED_SHADER_EFFECT_VALID
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_ped_shader_effect_valid_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_ped_shader_effect_valid_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PED_SHADER_EFFECT_VALID(ped)
}

/// ```
Same as SET_PED_ARMOUR, but ADDS 'amount' to the armor the Ped already has.  
```

pub fn add_armour_to_ped_safe(ped: Ped, amount: i64) -> NativeResult<()> {
    
    debug!("Calling native function: ADD_ARMOUR_TO_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ADD_ARMOUR_TO_PED(ped, amount)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ADD_ARMOUR_TO_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `add_armour_to_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn add_armour_to_ped_raw(ped: i32, amount: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ADD_ARMOUR_TO_PED(ped, amount)
}

/// ```
p1: Only "CODE_HUMAN_STAND_COWER" found in the b617d scripts.  
```

pub fn set_ped_cower_hash_safe(ped: Ped, p1: String) -> NativeResult<()> {
    let p1_cstr = std::ffi::CString::new(p1.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "p1", e)))?;
    
    debug!("Calling native function: SET_PED_COWER_HASH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_COWER_HASH(ped, crate::utils::rust_to_c_string(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_COWER_HASH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_cower_hash_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_cower_hash_raw(ped: i32, p1: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_COWER_HASH(ped, p1)
}

/// ```
It will revive/cure the injured ped. The condition is ped must not be dead.  
Upon setting and converting the health int, found, if health falls below 5, the ped will lay on the ground in pain(Maximum default health is 100).  
This function is well suited there.  
```

pub fn revive_injured_ped_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: REVIVE_INJURED_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REVIVE_INJURED_PED(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REVIVE_INJURED_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `revive_injured_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn revive_injured_ped_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REVIVE_INJURED_PED(ped)
}

/// ```
NOTE: Debugging functions are not present in the retail version of the game.  
*untested but char *name could also be a hash for a localized string  
```

pub fn set_ped_name_debug_safe(ped: Ped, name: String) -> NativeResult<()> {
    let name_cstr = std::ffi::CString::new(name.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "name", e)))?;
    
    debug!("Calling native function: SET_PED_NAME_DEBUG");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_NAME_DEBUG(ped, crate::utils::rust_to_c_string(name))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_NAME_DEBUG
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_name_debug_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_name_debug_raw(ped: i32, name: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_NAME_DEBUG(ped, name)
}

/// ## Parameters
* **ped**: 
* **groupId**:

pub fn set_ped_as_group_leader_safe(ped: Ped, groupId: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_AS_GROUP_LEADER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_AS_GROUP_LEADER(ped, groupId)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_AS_GROUP_LEADER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_as_group_leader_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_as_group_leader_raw(ped: i32, groupId: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_AS_GROUP_LEADER(ped, groupId)
}

/// Creates a copy of the passed ped, optionally setting it as local and/or shallow-copying the head blend data.

pub fn clone_ped_safe(ped: Ped) -> NativeResult<Ped> {
    
    debug!("Calling native function: CLONE_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLONE_PED(ped)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: CLONE_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clone_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clone_ped_raw(ped: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLONE_PED(ped)
}

/// ## Parameters
* **ped**:

pub fn request_ped_visibility_tracking_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: REQUEST_PED_VISIBILITY_TRACKING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REQUEST_PED_VISIBILITY_TRACKING(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REQUEST_PED_VISIBILITY_TRACKING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `request_ped_visibility_tracking_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn request_ped_visibility_tracking_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REQUEST_PED_VISIBILITY_TRACKING(ped)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn _0x061cb768363d6424_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x061CB768363D6424");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x061CB768363D6424(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x061CB768363D6424
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x061cb768363d6424_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x061cb768363d6424_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x061CB768363D6424(ped, toggle)
}

/// ```
The function specifically verifies the value is equal to, or less than 1.0f. If it is greater than 1.0f, the function does nothing at all.  
```

pub fn set_driver_ability_safe(driver: Ped, ability: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_DRIVER_ABILITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DRIVER_ABILITY(driver, ability)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DRIVER_ABILITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_driver_ability_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_driver_ability_raw(driver: i32, ability: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DRIVER_ABILITY(driver, ability)
}

/// ## Parameters
* **ped**:

pub fn is_ped_swimming_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_SWIMMING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_SWIMMING(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_SWIMMING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_swimming_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_swimming_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_SWIMMING(ped)
}

/// ```
Turns the desired ped into a cop. If you use this on the player ped, you will become almost invisible to cops dispatched for you. You will also report your own crimes, get a generic cop voice, get a cop-vision-cone on the radar, and you will be unable to shoot at other cops. SWAT and Army will still shoot at you. Toggling ped as "false" has no effect; you must change p0's ped model to disable the effect.  
```

pub fn set_ped_as_cop_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_AS_COP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_AS_COP(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_AS_COP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_as_cop_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_as_cop_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_AS_COP(ped, toggle)
}

/// ## Parameters
* **ped**:

pub fn get_ped_armour_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_ARMOUR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_ARMOUR(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_ARMOUR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_armour_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_armour_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_ARMOUR(ped)
}

/// ## Parameters
* **randomInt**: 
* **x**: 
* **y**: 
* **z**:

pub fn spawnpoints_get_search_result_safe(randomInt: i64, x: *mut f32, y: *mut f32, z: *mut f32) -> NativeResult<()> {
    
    debug!("Calling native function: SPAWNPOINTS_GET_SEARCH_RESULT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPAWNPOINTS_GET_SEARCH_RESULT(randomInt, x, y, z)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPAWNPOINTS_GET_SEARCH_RESULT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `spawnpoints_get_search_result_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn spawnpoints_get_search_result_raw(randomInt: i64, x: *mut f32, y: *mut f32, z: *mut f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPAWNPOINTS_GET_SEARCH_RESULT(randomInt, x, y, z)
}

/// ## Parameters
* **ped**: 
* **multiplier**:

pub fn set_ped_lod_multiplier_safe(ped: Ped, multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_LOD_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_LOD_MULTIPLIER(ped, multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_LOD_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_lod_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_lod_multiplier_raw(ped: i32, multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_LOD_MULTIPLIER(ped, multiplier)
}

/// REMOVE_SCENARIO_BLOCKING_AREAS native function

pub fn remove_scenario_blocking_areas_safe() -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_SCENARIO_BLOCKING_AREAS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_SCENARIO_BLOCKING_AREAS()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_SCENARIO_BLOCKING_AREAS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_scenario_blocking_areas_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_scenario_blocking_areas_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_SCENARIO_BLOCKING_AREAS()
}

/// When this ped receives its next script task, they will exit from their scenario using the normal scenario exit.
Exiting the scenario may take several frames while the ped is playing the exit animation.
If the ped is not currently using a scenario at the time of the command or 0,0,0 is specified as the reaction position,
then the ped will by default attempt to direct their exit forwards.

pub fn _set_ped_should_play_directed_scenario_exit_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: _SET_PED_SHOULD_PLAY_DIRECTED_SCENARIO_EXIT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PED_SHOULD_PLAY_DIRECTED_SCENARIO_EXIT(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _SET_PED_SHOULD_PLAY_DIRECTED_SCENARIO_EXIT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_ped_should_play_directed_scenario_exit_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_ped_should_play_directed_scenario_exit_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PED_SHOULD_PLAY_DIRECTED_SCENARIO_EXIT(ped)
}

/// ## Parameters
* **ped**:

pub fn is_ped_in_any_taxi_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_ANY_TAXI");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_ANY_TAXI(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_ANY_TAXI
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_any_taxi_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_any_taxi_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_ANY_TAXI(ped)
}

/// ## Parameters
* **ped**:

pub fn clear_ped_drive_by_clipset_override_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_DRIVE_BY_CLIPSET_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_DRIVE_BY_CLIPSET_OVERRIDE(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_DRIVE_BY_CLIPSET_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_drive_by_clipset_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_drive_by_clipset_override_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_DRIVE_BY_CLIPSET_OVERRIDE(ped)
}

/// ## Parameters
* **ped**:

pub fn get_seat_ped_is_trying_to_enter_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_SEAT_PED_IS_TRYING_TO_ENTER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_SEAT_PED_IS_TRYING_TO_ENTER(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_SEAT_PED_IS_TRYING_TO_ENTER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_seat_ped_is_trying_to_enter_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_seat_ped_is_trying_to_enter_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_SEAT_PED_IS_TRYING_TO_ENTER(ped)
}

/// ```
It simply makes the said ped to cower behind cover object(wall, desk, car)  
Peds flee attributes must be set to not to flee, first. Else, most of the peds, will just flee from gunshot sounds or any other panic situations.  
```

pub fn set_ped_can_cower_in_cover_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_COWER_IN_COVER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_COWER_IN_COVER(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_COWER_IN_COVER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_cower_in_cover_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_cower_in_cover_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_COWER_IN_COVER(ped, toggle)
}

/// ## Parameters
* **ped**: The ped handle.
* **componentId**: The component id to get the palette variation from. Refer to [SET_PED_COMPONENT_VARIATION](#_0x262B14F48D29DE80).

pub fn get_ped_palette_variation_safe(ped: Ped, componentId: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_PALETTE_VARIATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_PALETTE_VARIATION(ped, componentId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_PALETTE_VARIATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_palette_variation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_palette_variation_raw(ped: i32, componentId: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_PALETTE_VARIATION(ped, componentId)
}

/// ```
PED::SET_PED_IN_VEHICLE_CONTEXT(l_128, MISC::GET_HASH_KEY("MINI_PROSTITUTE_LOW_PASSENGER"));
PED::SET_PED_IN_VEHICLE_CONTEXT(l_128, MISC::GET_HASH_KEY("MINI_PROSTITUTE_LOW_RESTRICTED_PASSENGER"));
PED::SET_PED_IN_VEHICLE_CONTEXT(l_3212, MISC::GET_HASH_KEY("MISS_FAMILY1_JIMMY_SIT"));
PED::SET_PED_IN_VEHICLE_CONTEXT(l_3212, MISC::GET_HASH_KEY("MISS_FAMILY1_JIMMY_SIT_REAR"));
PED::SET_PED_IN_VEHICLE_CONTEXT(l_95, MISC::GET_HASH_KEY("MISS_FAMILY2_JIMMY_BICYCLE"));
PED::SET_PED_IN_VEHICLE_CONTEXT(num3, MISC::GET_HASH_KEY("MISSFBI2_MICHAEL_DRIVEBY"));
PED::SET_PED_IN_VEHICLE_CONTEXT(PLAYER::PLAYER_PED_ID(), MISC::GET_HASH_KEY("MISS_ARMENIAN3_FRANKLIN_TENSE"));
PED::SET_PED_IN_VEHICLE_CONTEXT(PLAYER::PLAYER_PED_ID(), MISC::GET_HASH_KEY("MISSFBI5_TREVOR_DRIVING"));
```

pub fn set_ped_in_vehicle_context_safe(ped: Ped, context: u32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_IN_VEHICLE_CONTEXT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_IN_VEHICLE_CONTEXT(ped, context)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_IN_VEHICLE_CONTEXT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_in_vehicle_context_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_in_vehicle_context_raw(ped: i32, context: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_IN_VEHICLE_CONTEXT(ped, context)
}

/// ## Parameters
* **ped**: 
* **flagId**:

pub fn get_ped_reset_flag_safe(ped: Ped, flagId: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_PED_RESET_FLAG");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_RESET_FLAG(ped, flagId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_RESET_FLAG
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_reset_flag_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_reset_flag_raw(ped: i32, flagId: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_RESET_FLAG(ped, flagId)
}

/// ## Parameters
* **ped**:

pub fn has_ped_preload_variation_data_finished_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_PED_PRELOAD_VARIATION_DATA_FINISHED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_PED_PRELOAD_VARIATION_DATA_FINISHED(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_PED_PRELOAD_VARIATION_DATA_FINISHED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_ped_preload_variation_data_finished_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_ped_preload_variation_data_finished_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_PED_PRELOAD_VARIATION_DATA_FINISHED(ped)
}

/// ```
bit 15 (0x8000) = force cower
```

pub fn set_ped_flee_attributes_safe(ped: Ped, attributeFlags: i64, enable: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_FLEE_ATTRIBUTES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_FLEE_ATTRIBUTES(ped, attributeFlags, enable)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_FLEE_ATTRIBUTES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_flee_attributes_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_flee_attributes_raw(ped: i32, attributeFlags: i64, enable: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_FLEE_ATTRIBUTES(ped, attributeFlags, enable)
}

/// ```
Used with freemode (online) characters.
```

pub fn get_ped_head_overlay_num_safe(overlayID: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_HEAD_OVERLAY_NUM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_HEAD_OVERLAY_NUM(overlayID)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_HEAD_OVERLAY_NUM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_head_overlay_num_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_head_overlay_num_raw(overlayID: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_HEAD_OVERLAY_NUM(overlayID)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn _0x425aecf167663f48_safe(ped: Ped, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x425AECF167663F48");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x425AECF167663F48(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x425AECF167663F48
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x425aecf167663f48_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x425aecf167663f48_raw(ped: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x425AECF167663F48(ped, p1)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn clear_ped_alternate_walk_anim_safe(ped: Ped, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_ALTERNATE_WALK_ANIM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_ALTERNATE_WALK_ANIM(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_ALTERNATE_WALK_ANIM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_alternate_walk_anim_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_alternate_walk_anim_raw(ped: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_ALTERNATE_WALK_ANIM(ped, p1)
}

/// PED::SET_PED_RESET_FLAG(PLAYER::PLAYER_PED_ID(), 240, 1);
Known values:

pub fn set_ped_reset_flag_safe(ped: Ped, flagId: i64, doReset: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_RESET_FLAG");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_RESET_FLAG(ped, flagId, doReset)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_RESET_FLAG
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_reset_flag_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_reset_flag_raw(ped: i32, flagId: i64, doReset: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_RESET_FLAG(ped, flagId, doReset)
}

/// ```
This is the SET_CHAR_DUCKING from GTA IV, that makes Peds duck. This function does nothing in GTA V. It cannot set the ped as ducking in vehicles, and IS_PED_DUCKING will always return false.  
```

pub fn set_ped_ducking_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_DUCKING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_DUCKING(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_DUCKING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_ducking_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_ducking_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_DUCKING(ped, toggle)
}

/// ## Return value

pub fn has_pedheadshot_img_upload_succeeded_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_PEDHEADSHOT_IMG_UPLOAD_SUCCEEDED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_PEDHEADSHOT_IMG_UPLOAD_SUCCEEDED()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_PEDHEADSHOT_IMG_UPLOAD_SUCCEEDED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_pedheadshot_img_upload_succeeded_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_pedheadshot_img_upload_succeeded_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_PEDHEADSHOT_IMG_UPLOAD_SUCCEEDED()
}

/// ## Parameters
* **x**: 
* **y**: 
* **z**: 
* **p3**: 
* **p4**: 
* **p5**: 
* **p6**: 
* **interiorFlags**: 
* **scale**: 
* **duration**:

pub fn spawnpoints_start_search_in_angled_area_safe(x: f32, y: f32, z: f32, p3: f32, p4: f32, p5: f32, p6: f32, interiorFlags: i64, scale: f32, duration: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SPAWNPOINTS_START_SEARCH_IN_ANGLED_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPAWNPOINTS_START_SEARCH_IN_ANGLED_AREA(x, y, z, p3, p4, p5, p6, interiorFlags, scale, duration)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPAWNPOINTS_START_SEARCH_IN_ANGLED_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `spawnpoints_start_search_in_angled_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn spawnpoints_start_search_in_angled_area_raw(x: f32, y: f32, z: f32, p3: f32, p4: f32, p5: f32, p6: f32, interiorFlags: i64, scale: f32, duration: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPAWNPOINTS_START_SEARCH_IN_ANGLED_AREA(x, y, z, p3, p4, p5, p6, interiorFlags, scale, duration)
}

/// ```
enable or disable the gravity of a ped  
Examples:  
PED::SET_PED_GRAVITY(PLAYER::PLAYER_PED_ID(), 0x00000001);  
PED::SET_PED_GRAVITY(Local_289[iVar0 /*20*/], 0x00000001);  
```

pub fn set_ped_gravity_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_GRAVITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_GRAVITY(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_GRAVITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_gravity_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_gravity_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_GRAVITY(ped, toggle)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x0b3e35ac043707d9_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x0B3E35AC043707D9");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x0B3E35AC043707D9(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x0B3E35AC043707D9
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x0b3e35ac043707d9_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x0b3e35ac043707d9_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x0B3E35AC043707D9(p0, p1)
}

/// ## Parameters
* **value**:

pub fn set_ai_weapon_damage_modifier_safe(value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_AI_WEAPON_DAMAGE_MODIFIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_AI_WEAPON_DAMAGE_MODIFIER(value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_AI_WEAPON_DAMAGE_MODIFIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ai_weapon_damage_modifier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ai_weapon_damage_modifier_raw(value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_AI_WEAPON_DAMAGE_MODIFIER(value)
}

/// CLEAR_PED_NON_CREATION_AREA native function

pub fn clear_ped_non_creation_area_safe() -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_NON_CREATION_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_NON_CREATION_AREA()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_NON_CREATION_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_non_creation_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_non_creation_area_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_NON_CREATION_AREA()
}

/// ## Parameters
* **sceneID**:

pub fn get_synchronized_scene_phase_safe(sceneID: i64) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_SYNCHRONIZED_SCENE_PHASE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_SYNCHRONIZED_SCENE_PHASE(sceneID)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_SYNCHRONIZED_SCENE_PHASE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_synchronized_scene_phase_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_synchronized_scene_phase_raw(sceneID: i64) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_SYNCHRONIZED_SCENE_PHASE(sceneID)
}

/// Sets the various freemode face features, e.g. nose length, chin shape.

pub fn _set_ped_face_feature_safe(ped: Ped, index: i64, scale: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PED_FACE_FEATURE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PED_FACE_FEATURE(ped, index, scale)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PED_FACE_FEATURE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_ped_face_feature_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_ped_face_feature_raw(ped: i32, index: i64, scale: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PED_FACE_FEATURE(ped, index, scale)
}

/// ## Parameters
* **scenarioBlockingIndex**: the index of the Scenario blocking area
* **bNetwork**: Optionally networked to all other players

pub fn remove_scenario_blocking_area_safe(scenarioBlockingIndex: i64, bNetwork: bool) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_SCENARIO_BLOCKING_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_SCENARIO_BLOCKING_AREA(scenarioBlockingIndex, bNetwork)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_SCENARIO_BLOCKING_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_scenario_blocking_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_scenario_blocking_area_raw(scenarioBlockingIndex: i64, bNetwork: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_SCENARIO_BLOCKING_AREA(scenarioBlockingIndex, bNetwork)
}

/// ```
NativeDB Introduced: v1493
```

pub fn _clear_facial_clipset_override_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: _CLEAR_FACIAL_CLIPSET_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_CLEAR_FACIAL_CLIPSET_OVERRIDE(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _CLEAR_FACIAL_CLIPSET_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_clear_facial_clipset_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _clear_facial_clipset_override_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_CLEAR_FACIAL_CLIPSET_OVERRIDE(ped)
}

/// ```
NativeDB Introduced: v1868
```

pub fn _0xc30bdaee47256c13_safe(p0: serde_json::Value) -> NativeResult<serde_json::Value> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xC30BDAEE47256C13");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xC30BDAEE47256C13(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0xC30BDAEE47256C13
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xc30bdaee47256c13_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xc30bdaee47256c13_raw(p0: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xC30BDAEE47256C13(p0)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x820e9892a77e97cd_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x820E9892A77E97CD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x820E9892A77E97CD(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x820E9892A77E97CD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x820e9892a77e97cd_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x820e9892a77e97cd_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x820E9892A77E97CD(p0, p1)
}

/// These combat attributes seem to be the same as the BehaviourFlags from combatbehaviour.meta.

So far, these are the equivalents found:

```c
enum eCombatAttribute
{
  CA_INVALID = -1,	
  // AI will only use cover if this is set
  CA_USE_COVER = 0,
  // AI will only use vehicles if this is set
  CA_USE_VEHICLE = 1,
  // AI will only driveby from a vehicle if this is set
  CA_DO_DRIVEBYS = 2,
  // Will be forced to stay in a ny vehicel if this isn't set
  CA_LEAVE_VEHICLES = 3,
  // This ped can make decisions on whether to strafe or not based on distance to destination, recent bullet events, etc.
  CA_CAN_USE_DYNAMIC_STRAFE_DECISIONS	= 4,
  // Ped will always fight upon getting threat response task
  CA_ALWAYS_FIGHT = 5,
  // If in combat and in a vehicle, the ped will flee rather than attacking
  CA_FLEE_WHILST_IN_VEHICLE = 6,
  // If in combat and chasing in a vehicle, the ped will keep a distance behind rather than ramming
  CA_JUST_FOLLOW_VEHICLE = 7,
  // Deprecated
  CA_PLAY_REACTION_ANIMS = 8,
  // Peds will scan for and react to dead peds found
  CA_WILL_SCAN_FOR_DEAD_PEDS = 9,
  // Deprecated
  CA_IS_A_GUARD = 10,
  // The ped will seek cover only 
  CA_JUST_SEEK_COVER = 11,
  // Ped will only blind fire when in cover
  CA_BLIND_FIRE_IN_COVER = 12,
  // Ped may advance
  CA_AGGRESSIVE = 13,
  // Ped can investigate events such as distant gunfire, footsteps, explosions etc
  CA_CAN_INVESTIGATE = 14,
  // Ped can use a radio to call for backup (happens after a reaction)
  CA_CAN_USE_RADIO = 15,
  // Deprecated
  CA_CAN_CAPTURE_ENEMY_PEDS = 16,
  // Ped will always flee upon getting threat response task
  CA_ALWAYS_FLEE = 17,
  // Ped can do unarmed taunts in vehicle
  CA_CAN_TAUNT_IN_VEHICLE = 20,
  // Ped will be able to chase their targets if both are on foot and the target is running away
  CA_CAN_CHASE_TARGET_ON_FOOT = 21,
  // Ped can drag injured peds to safety
  CA_WILL_DRAG_INJURED_PEDS_TO_SAFETY = 22,
  // Ped will require LOS to the target it is aiming at before shooting
  CA_REQUIRES_LOS_TO_SHOOT = 23,
  // Ped is allowed to use proximity based fire rate (increasing fire rate at closer distances)
  CA_USE_PROXIMITY_FIRING_RATE = 24,
  // Normally peds can switch briefly to a secondary target in combat, setting this will prevent that
  CA_DISABLE_SECONDARY_TARGET = 25,
  // This will disable the flinching combat entry reactions for peds, instead only playing the turn and aim anims
  CA_DISABLE_ENTRY_REACTIONS = 26,
  // Force ped to be 100% accurate in all situations (added by Jay Reinebold)
  CA_PERFECT_ACCURACY = 27,
  // If we don't have cover and can't see our target it's possible we will advance, even if the target is in cover
  CA_CAN_USE_FRUSTRATED_ADVANCE	= 28,
  // This will have the ped move to defensive areas and within attack windows before performing the cover search
  CA_MOVE_TO_LOCATION_BEFORE_COVER_SEARCH = 29,
  // Allow shooting of our weapon even if we don't have LOS (this isn't X-ray vision as it only affects weapon firing)
  CA_CAN_SHOOT_WITHOUT_LOS = 30,
  // Ped will try to maintain a min distance to the target, even if using defensive areas (currently only for cover finding + usage) 
  CA_MAINTAIN_MIN_DISTANCE_TO_TARGET = 31,
  // Allows ped to use steamed variations of peeking anims
  CA_CAN_USE_PEEKING_VARIATIONS	= 34,
  // Disables pinned down behaviors
  CA_DISABLE_PINNED_DOWN = 35,
  // Disables pinning down others
  CA_DISABLE_PIN_DOWN_OTHERS = 36,
  // When defensive area is reached the area is cleared and the ped is set to use defensive combat movement
  CA_OPEN_COMBAT_WHEN_DEFENSIVE_AREA_IS_REACHED = 37,
  // Disables bullet reactions
  CA_DISABLE_BULLET_REACTIONS = 38,
  // Allows ped to bust the player
  CA_CAN_BUST = 39,
  // This ped is ignored by other peds when wanted
  CA_IGNORED_BY_OTHER_PEDS_WHEN_WANTED = 40,
  // Ped is allowed to "jack" vehicles when needing to chase a target in combat
  CA_CAN_COMMANDEER_VEHICLES = 41,
  // Ped is allowed to flank
  CA_CAN_FLANK = 42,
  // Ped will switch to advance if they can't find cover
  CA_SWITCH_TO_ADVANCE_IF_CANT_FIND_COVER = 43,
  // Ped will switch to defensive if they are in cover
  CA_SWITCH_TO_DEFENSIVE_IF_IN_COVER = 44,
  // Ped will clear their primary defensive area when it is reached
  CA_CLEAR_PRIMARY_DEFENSIVE_AREA_WHEN_REACHED = 45,
  // Ped is allowed to fight armed peds when not armed
  CA_CAN_FIGHT_ARMED_PEDS_WHEN_NOT_ARMED = 46,
  // Ped is not allowed to use tactical points if set to use defensive movement (will only use cover)
  CA_ENABLE_TACTICAL_POINTS_WHEN_DEFENSIVE = 47,
  // Ped cannot adjust cover arcs when testing cover safety (atm done on corner cover points when  ped usingdefensive area + no LOS)
  CA_DISABLE_COVER_ARC_ADJUSTMENTS = 48,
  // Ped may use reduced accuracy with large number of enemies attacking the same local player target
  CA_USE_ENEMY_ACCURACY_SCALING	= 49,
  // Ped is allowed to charge the enemy position
  CA_CAN_CHARGE = 50,
  // When defensive area is reached the area is cleared and the ped is set to use will advance movement
  CA_REMOVE_AREA_SET_WILL_ADVANCE_WHEN_DEFENSIVE_AREA_REACHED = 51,
  // Use the vehicle attack mission during combat (only works on driver)
  CA_USE_VEHICLE_ATTACK = 52,
  // Use the vehicle attack mission during combat if the vehicle has mounted guns (only works on driver)
  CA_USE_VEHICLE_ATTACK_IF_VEHICLE_HAS_MOUNTED_GUNS = 53,
  // Always equip best weapon in combat
  CA_ALWAYS_EQUIP_BEST_WEAPON = 54,
  // Ignores in water at depth visibility check
  CA_CAN_SEE_UNDERWATER_PEDS = 55,
  // Will prevent this ped from aiming at any AI targets that are in helicopters
  CA_DISABLE_AIM_AT_AI_TARGETS_IN_HELIS = 56,
  // Disables peds seeking due to no clear line of sight
  CA_DISABLE_SEEK_DUE_TO_LINE_OF_SIGHT = 57,
  // To be used when releasing missions peds if we don't want them fleeing from combat (mission peds already prevent flee)
  CA_DISABLE_FLEE_FROM_COMBAT = 58,
  // Disables target changes during vehicle pursuit
  CA_DISABLE_TARGET_CHANGES_DURING_VEHICLE_PURSUIT = 59,
  // Ped may throw a smoke grenade at player loitering in combat
  CA_CAN_THROW_SMOKE_GRENADE = 60,
  // Will clear a set defensive area if that area cannot be reached
  CA_CLEAR_AREA_SET_DEFENSIVE_IF_DEFENSIVE_CANNOT_BE_REACHED = 62,
  // Disable block from pursue during vehicle chases
  CA_DISABLE_BLOCK_FROM_PURSUE_DURING_VEHICLE_CHASE = 64,
  // Disable spin out during vehicle chases
  CA_DISABLE_SPIN_OUT_DURING_VEHICLE_CHASE = 65,
  // Disable cruise in front during block during vehicle chases
  CA_DISABLE_CRUISE_IN_FRONT_DURING_BLOCK_DURING_VEHICLE_CHASE = 66,
  // Makes it more likely that the ped will continue targeting a target with blocked los for a few seconds
  CA_CAN_IGNORE_BLOCKED_LOS_WEIGHTING = 67,
  // Disables the react to buddy shot behaviour.
  CA_DISABLE_REACT_TO_BUDDY_SHOT = 68,
  // Prefer pathing using navmesh over road nodes
  CA_PREFER_NAVMESH_DURING_VEHICLE_CHASE = 69,
  // Ignore road edges when avoiding
  CA_ALLOWED_TO_AVOID_OFFROAD_DURING_VEHICLE_CHASE = 70,
  // Permits ped to charge a target outside the assigned defensive area.
  CA_PERMIT_CHARGE_BEYOND_DEFENSIVE_AREA = 71,
  // This ped will switch to an RPG if target is in a vehicle, otherwise will use alternate weapon.
  CA_USE_ROCKETS_AGAINST_VEHICLES_ONLY = 72,
  // Disables peds moving to a tactical point without clear los
  CA_DISABLE_TACTICAL_POINTS_WITHOUT_CLEAR_LOS = 73,
  // Disables pull alongside during vehicle chase
  CA_DISABLE_PULL_ALONGSIDE_DURING_VEHICLE_CHASE = 74,
  // If set on a ped, they will not flee when all random peds flee is set to TRUE (they are still able to flee due to other reasons)
  CA_DISABLE_ALL_RANDOMS_FLEE = 78,
  // This ped will send out a script DeadPedSeenEvent when they see a dead ped
  CA_WILL_GENERATE_DEAD_PED_SEEN_SCRIPT_EVENTS = 79,
  // This will use the receiving peds sense range rather than the range supplied to the communicate event
  CA_USE_MAX_SENSE_RANGE_WHEN_RECEIVING_EVENTS = 80,
  // When aiming from a vehicle the ped will only aim at targets on his side of the vehicle
  CA_RESTRICT_IN_VEHICLE_AIMING_TO_CURRENT_SIDE = 81,
  // LOS to the target is blocked we return to our default position and direction until we have LOS (no aiming)
  CA_USE_DEFAULT_BLOCKED_LOS_POSITION_AND_DIRECTION = 82,
  // LOS to the target is blocked we return to our default position and direction until we have LOS (no aiming)
  CA_REQUIRES_LOS_TO_AIM = 83,
  // Allow vehicles spawned infront of target facing away to enter cruise and wait to block approaching target
  CA_CAN_CRUISE_AND_BLOCK_IN_VEHICLE = 84,
  // Peds flying aircraft will prefer to target other aircraft over entities on the ground
  CA_PREFER_AIR_COMBAT_WHEN_IN_AIRCRAFT = 85,
  //Allow peds flying aircraft to use dog fighting behaviours
  CA_ALLOW_DOG_FIGHTING = 86,
  // This will make the weight of targets who aircraft vehicles be reduced greatly compared to targets on foot or in ground based vehicles
  CA_PREFER_NON_AIRCRAFT_TARGETS = 87,
  //When peds are tasked to go to combat, they keep searching for a known target for a while before forcing an unknown one
  CA_PREFER_KNOWN_TARGETS_WHEN_COMBAT_CLOSEST_TARGET = 88,
  // Only allow mounted weapons to fire if within the correct attack angle (default 25-degree cone). On a flag in order to keep exiting behaviour and only fix in specific cases.
  CA_FORCE_CHECK_ATTACK_ANGLE_FOR_MOUNTED_GUNS = 89,
  // Blocks the firing state for passenger-controlled mounted weapons. Existing flags CA_USE_VEHICLE_ATTACK and CA_USE_VEHICLE_ATTACK_IF_VEHICLE_HAS_MOUNTED_GUNS only work for drivers.
  CA_BLOCK_FIRE_FOR_VEHICLE_PASSENGER_MOUNTED_GUNS = 90 
};
```

pub fn set_ped_combat_attributes_safe(ped: Ped, attributeIndex: i64, enabled: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_COMBAT_ATTRIBUTES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_COMBAT_ATTRIBUTES(ped, attributeIndex, enabled)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_COMBAT_ATTRIBUTES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_combat_attributes_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_combat_attributes_raw(ped: i32, attributeIndex: i64, enabled: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_COMBAT_ATTRIBUTES(ped, attributeIndex, enabled)
}

/// ## Parameters
* **ped**: 
* **event**:

pub fn is_ped_responding_to_event_safe(ped: Ped, event: serde_json::Value) -> NativeResult<bool> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let event_any_str = serde_json::to_string(&event)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "event", e)))?;
    let event_any_str_cstr = std::ffi::CString::new(event_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "event", e)))?;
    
    debug!("Calling native function: IS_PED_RESPONDING_TO_EVENT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_RESPONDING_TO_EVENT(ped, crate::utils::any_to_c_void_ptr(event))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_RESPONDING_TO_EVENT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_responding_to_event_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_responding_to_event_raw(ped: i32, event: *mut std::os::raw::c_void) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_RESPONDING_TO_EVENT(ped, event)
}

/// ## Parameters
* **ped**: The ped handle.
* **componentId**: The component id you want to get the drawable variatons of. Refer to [SET_PED_COMPONENT_VARIATION](#_0x262B14F48D29DE80)

pub fn get_number_of_ped_drawable_variations_safe(ped: Ped, componentId: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_NUMBER_OF_PED_DRAWABLE_VARIATIONS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_NUMBER_OF_PED_DRAWABLE_VARIATIONS(ped, componentId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_NUMBER_OF_PED_DRAWABLE_VARIATIONS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_number_of_ped_drawable_variations_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_number_of_ped_drawable_variations_raw(ped: i32, componentId: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_NUMBER_OF_PED_DRAWABLE_VARIATIONS(ped, componentId)
}

/// ## Parameters
* **p0**:

pub fn _0x5407b7288d0478b7_safe(p0: serde_json::Value) -> NativeResult<i64> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x5407B7288D0478B7");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x5407B7288D0478B7(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0x5407B7288D0478B7
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x5407b7288d0478b7_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x5407b7288d0478b7_raw(p0: *mut std::os::raw::c_void) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x5407B7288D0478B7(p0)
}

/// ## Parameters
* **ped**:

pub fn _freeze_ped_camera_rotation_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: _FREEZE_PED_CAMERA_ROTATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_FREEZE_PED_CAMERA_ROTATION(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _FREEZE_PED_CAMERA_ROTATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_freeze_ped_camera_rotation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _freeze_ped_camera_rotation_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_FREEZE_PED_CAMERA_ROTATION(ped)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **p2**:

pub fn get_dead_ped_pickup_coords_safe(ped: Ped, p1: f32, p2: f32) -> NativeResult<Vector3> {
    
    debug!("Calling native function: GET_DEAD_PED_PICKUP_COORDS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_DEAD_PED_PICKUP_COORDS(ped, p1, p2)
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: GET_DEAD_PED_PICKUP_COORDS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_dead_ped_pickup_coords_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_dead_ped_pickup_coords_raw(ped: i32, p1: f32, p2: f32) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_DEAD_PED_PICKUP_COORDS(ped, p1, p2)
}

/// ## Return value

pub fn spawnpoints_is_search_active_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: SPAWNPOINTS_IS_SEARCH_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPAWNPOINTS_IS_SEARCH_ACTIVE()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: SPAWNPOINTS_IS_SEARCH_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `spawnpoints_is_search_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn spawnpoints_is_search_active_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPAWNPOINTS_IS_SEARCH_ACTIVE()
}

/// ## Parameters
* **colorId**:

pub fn _is_ped_hair_color_valid_2_safe(colorId: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PED_HAIR_COLOR_VALID_2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PED_HAIR_COLOR_VALID_2(colorId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PED_HAIR_COLOR_VALID_2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_ped_hair_color_valid_2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_ped_hair_color_valid_2_raw(colorId: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PED_HAIR_COLOR_VALID_2(colorId)
}

/// ## Parameters
* **ped**:

pub fn is_ped_in_high_cover_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_HIGH_COVER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_HIGH_COVER(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_HIGH_COVER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_high_cover_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_high_cover_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_HIGH_COVER(ped)
}

/// ## Parameters
* **ped**: 
* **clipSet**:

pub fn set_ped_weapon_movement_clipset_safe(ped: Ped, clipSet: String) -> NativeResult<()> {
    let clipSet_cstr = std::ffi::CString::new(clipSet.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "clipSet", e)))?;
    
    debug!("Calling native function: SET_PED_WEAPON_MOVEMENT_CLIPSET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_WEAPON_MOVEMENT_CLIPSET(ped, crate::utils::rust_to_c_string(clipSet))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_WEAPON_MOVEMENT_CLIPSET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_weapon_movement_clipset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_weapon_movement_clipset_raw(ped: i32, clipSet: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_WEAPON_MOVEMENT_CLIPSET(ped, clipSet)
}

/// ## Parameters
* **toggle**:

pub fn set_create_random_cops_on_scenarios_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CREATE_RANDOM_COPS_ON_SCENARIOS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CREATE_RANDOM_COPS_ON_SCENARIOS(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CREATE_RANDOM_COPS_ON_SCENARIOS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_create_random_cops_on_scenarios_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_create_random_cops_on_scenarios_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CREATE_RANDOM_COPS_ON_SCENARIOS(toggle)
}

/// ```
_SET_PED_HEAD_* - _SET_PED_HEARING_*

_SET_PED_HEALTH_...
```

```
NativeDB Introduced: v2699
```

pub fn _0xb3352e018d6f89df_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xB3352E018D6F89DF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xB3352E018D6F89DF(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xB3352E018D6F89DF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xb3352e018d6f89df_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xb3352e018d6f89df_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xB3352E018D6F89DF(toggle)
}

/// Sets whether a pedestrian should wear a helmet.

pub fn set_ped_helmet_safe(ped: Ped, bEnable: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_HELMET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_HELMET(ped, bEnable)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_HELMET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_helmet_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_helmet_raw(ped: i32, bEnable: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_HELMET(ped, bEnable)
}

/// ```
This native refers to the field of vision the ped has below them, starting at 0 degrees. The angle value should be negative.  
```

pub fn set_ped_visual_field_min_elevation_angle_safe(ped: Ped, angle: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_VISUAL_FIELD_MIN_ELEVATION_ANGLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_VISUAL_FIELD_MIN_ELEVATION_ANGLE(ped, angle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_VISUAL_FIELD_MIN_ELEVATION_ANGLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_visual_field_min_elevation_angle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_visual_field_min_elevation_angle_raw(ped: i32, angle: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_VISUAL_FIELD_MIN_ELEVATION_ANGLE(ped, angle)
}

/// ```
gtaforums.com/topic/885580-ped-headshotmugshot-txd/  
```

pub fn is_pedheadshot_ready_safe(id: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PEDHEADSHOT_READY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PEDHEADSHOT_READY(id)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PEDHEADSHOT_READY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_pedheadshot_ready_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_pedheadshot_ready_raw(id: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PEDHEADSHOT_READY(id)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn set_scripted_anim_seat_offset_safe(ped: Ped, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SCRIPTED_ANIM_SEAT_OFFSET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SCRIPTED_ANIM_SEAT_OFFSET(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SCRIPTED_ANIM_SEAT_OFFSET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_scripted_anim_seat_offset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_scripted_anim_seat_offset_raw(ped: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SCRIPTED_ANIM_SEAT_OFFSET(ped, p1)
}

/// ```
0: Freedom to move
1: Circle Around Leader
2: Alternative Circle Around Leader  
3: Line, with Leader at center  
4: Arrow Formation
5: "V" Formation
6: Line Follow Formation
7: Single Formation
8: Pairwise
```

pub fn set_group_formation_safe(groupId: i64, formationType: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_GROUP_FORMATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_GROUP_FORMATION(groupId, formationType)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_GROUP_FORMATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_group_formation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_group_formation_raw(groupId: i64, formationType: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_GROUP_FORMATION(groupId, formationType)
}

/// ## Parameters
* **ped**:

pub fn clear_ped_parachute_pack_variation_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_PARACHUTE_PACK_VARIATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_PARACHUTE_PACK_VARIATION(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_PARACHUTE_PACK_VARIATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_parachute_pack_variation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_parachute_pack_variation_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_PARACHUTE_PACK_VARIATION(ped)
}

/// ```
Returns whether the specified ped is reloading.  
```

pub fn is_ped_reloading_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_RELOADING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_RELOADING(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_RELOADING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_reloading_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_reloading_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_RELOADING(ped)
}

/// ```
Maximum possible amount of money on MP is 2000. ~JX  
-----------------------------------------------------------------------------  
Maximum amount that a ped can theoretically have is 65535 (0xFFFF) since the amount is stored as an unsigned short (uint16_t) value.  
```

pub fn set_ped_money_safe(ped: Ped, amount: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_MONEY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_MONEY(ped, amount)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_MONEY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_money_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_money_raw(ped: i32, amount: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_MONEY(ped, amount)
}

/// ## Parameters
* **vehicle**: 
* **returnHandle**:

pub fn create_random_ped_as_driver_safe(vehicle: Vehicle, returnHandle: bool) -> NativeResult<Ped> {
    
    debug!("Calling native function: CREATE_RANDOM_PED_AS_DRIVER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CREATE_RANDOM_PED_AS_DRIVER(vehicle, returnHandle)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: CREATE_RANDOM_PED_AS_DRIVER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `create_random_ped_as_driver_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn create_random_ped_as_driver_raw(vehicle: i32, returnHandle: bool) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CREATE_RANDOM_PED_AS_DRIVER(vehicle, returnHandle)
}

/// ## Parameters
* **groupHandle**:

pub fn reset_group_formation_default_spacing_safe(groupHandle: i64) -> NativeResult<()> {
    
    debug!("Calling native function: RESET_GROUP_FORMATION_DEFAULT_SPACING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_GROUP_FORMATION_DEFAULT_SPACING(groupHandle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_GROUP_FORMATION_DEFAULT_SPACING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_group_formation_default_spacing_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_group_formation_default_spacing_raw(groupHandle: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_GROUP_FORMATION_DEFAULT_SPACING(groupHandle)
}

/// ```
damages a ped with the given amount  
----  
armorFirst means it will damage/lower the armor first before damaging the player.  
setting damageAmount to a negative amount will cause the player or the armor (depending on armorFirst) to be healed by damageAmount instead.  
```

```
NativeDB Added Parameter 4: Any p3
```

pub fn apply_damage_to_ped_safe(ped: Ped, damageAmount: i64, armorFirst: bool) -> NativeResult<()> {
    
    debug!("Calling native function: APPLY_DAMAGE_TO_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::APPLY_DAMAGE_TO_PED(ped, damageAmount, armorFirst)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: APPLY_DAMAGE_TO_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `apply_damage_to_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn apply_damage_to_ped_raw(ped: i32, damageAmount: i64, armorFirst: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::APPLY_DAMAGE_TO_PED(ped, damageAmount, armorFirst)
}

/// Console/PC structure definitions and example: pastebin.com/SsFej963

For FiveM/Cfx.Re use-cases refer to: [`GET_GAME_POOL`](#_0x2B9D4F50).

pub fn get_ped_nearby_peds_safe(ped: Ped, sizeAndPeds: *mut i64, ignore: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_NEARBY_PEDS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_NEARBY_PEDS(ped, sizeAndPeds, ignore)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_NEARBY_PEDS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_nearby_peds_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_nearby_peds_raw(ped: i32, sizeAndPeds: *mut i64, ignore: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_NEARBY_PEDS(ped, sizeAndPeds, ignore)
}

/// ## Parameters
* **ped**:

pub fn is_ped_ducking_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_DUCKING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_DUCKING(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_DUCKING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_ducking_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_ducking_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_DUCKING(ped)
}

/// [Animations list](https://alexguirre.github.io/animations-list/)

pub fn is_scripted_scenario_ped_using_conditional_anim_safe(ped: Ped, animDict: String, anim: String) -> NativeResult<bool> {
    let animDict_cstr = std::ffi::CString::new(animDict.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animDict", e)))?;
    let anim_cstr = std::ffi::CString::new(anim.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "anim", e)))?;
    
    debug!("Calling native function: IS_SCRIPTED_SCENARIO_PED_USING_CONDITIONAL_ANIM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_SCRIPTED_SCENARIO_PED_USING_CONDITIONAL_ANIM(ped, crate::utils::rust_to_c_string(animDict), crate::utils::rust_to_c_string(anim))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_SCRIPTED_SCENARIO_PED_USING_CONDITIONAL_ANIM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_scripted_scenario_ped_using_conditional_anim_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_scripted_scenario_ped_using_conditional_anim_raw(ped: i32, animDict: *const std::os::raw::c_char, anim: *const std::os::raw::c_char) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_SCRIPTED_SCENARIO_PED_USING_CONDITIONAL_ANIM(ped, animDict, anim)
}

/// ## Parameters
* **groupID**:

pub fn get_ped_as_group_leader_safe(groupID: i64) -> NativeResult<Ped> {
    
    debug!("Calling native function: GET_PED_AS_GROUP_LEADER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_AS_GROUP_LEADER(groupID)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: GET_PED_AS_GROUP_LEADER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_as_group_leader_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_as_group_leader_raw(groupID: i64) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_AS_GROUP_LEADER(groupID)
}

/// ## Parameters
* **ped**:

pub fn reset_ped_weapon_movement_clipset_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: RESET_PED_WEAPON_MOVEMENT_CLIPSET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_PED_WEAPON_MOVEMENT_CLIPSET(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_PED_WEAPON_MOVEMENT_CLIPSET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_ped_weapon_movement_clipset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_ped_weapon_movement_clipset_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_PED_WEAPON_MOVEMENT_CLIPSET(ped)
}

/// ## Parameters
* **ped**:

pub fn reset_ped_visible_damage_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: RESET_PED_VISIBLE_DAMAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_PED_VISIBLE_DAMAGE(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_PED_VISIBLE_DAMAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_ped_visible_damage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_ped_visible_damage_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_PED_VISIBLE_DAMAGE(ped)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_dies_instantly_in_water_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_DIES_INSTANTLY_IN_WATER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_DIES_INSTANTLY_IN_WATER(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_DIES_INSTANTLY_IN_WATER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_dies_instantly_in_water_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_dies_instantly_in_water_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_DIES_INSTANTLY_IN_WATER(ped, toggle)
}

/// **This native does absolutely nothing, just a nullsub**

pub fn _0x1216e0bfa72cc703_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x1216E0BFA72CC703");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x1216E0BFA72CC703(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x1216E0BFA72CC703
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x1216e0bfa72cc703_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x1216e0bfa72cc703_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x1216E0BFA72CC703(p0, p1)
}

/// ```c
enum eNMFallType {
    TYPE_FROM_HIGH = 0,
    TYPE_OVER_WALL = 1,
    TYPE_DOWN_STAIRS = 2,
    TYPE_DIE_TYPES = 3,
    TYPE_DIE_FROM_HIGH = 4,
    TYPE_DIE_OVER_WALL = 5,
    TYPE_DIE_DOWN_STAIRS = 6
}
```

```
Return variable is never used in R*'s scripts.  
Not sure what p2 does. It seems like it would be a time judging by it's usage in R*'s scripts, but didn't seem to affect anything in my testings.  
x, y, and z are coordinates, most likely to where the ped will fall.  
p7 is probably the force of the fall, but untested, so I left the variable name the same.  
p8 to p13 are always 0f in R*'s scripts.  
(Simplified) Example of the usage of the function from R*'s scripts:  
ped::set_ped_to_ragdoll_with_fall(ped, 1500, 2000, 1, -entity::get_entity_forward_vector(ped), 1f, 0f, 0f, 0f, 0f, 0f, 0f);  
```

pub fn set_ped_to_ragdoll_with_fall_safe(ped: Ped, minTime: i64, maxTime: i64, nFallType: i64, dirX: f32, dirY: f32, dirZ: f32, fGroundHeight: f32, grab1X: f32, grab1Y: f32, grab1Z: f32, grab2X: f32, grab2Y: f32, grab2Z: f32) -> NativeResult<bool> {
    
    debug!("Calling native function: SET_PED_TO_RAGDOLL_WITH_FALL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_TO_RAGDOLL_WITH_FALL(ped, minTime, maxTime, nFallType, dirX, dirY, dirZ, fGroundHeight, grab1X, grab1Y, grab1Z, grab2X, grab2Y, grab2Z)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: SET_PED_TO_RAGDOLL_WITH_FALL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_to_ragdoll_with_fall_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_to_ragdoll_with_fall_raw(ped: i32, minTime: i64, maxTime: i64, nFallType: i64, dirX: f32, dirY: f32, dirZ: f32, fGroundHeight: f32, grab1X: f32, grab1Y: f32, grab1Z: f32, grab2X: f32, grab2Y: f32, grab2Z: f32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_TO_RAGDOLL_WITH_FALL(ped, minTime, maxTime, nFallType, dirX, dirY, dirZ, fGroundHeight, grab1X, grab1Y, grab1Z, grab2X, grab2Y, grab2Z)
}

/// ## Parameters
* **id**:

pub fn request_pedheadshot_img_upload_safe(id: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: REQUEST_PEDHEADSHOT_IMG_UPLOAD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REQUEST_PEDHEADSHOT_IMG_UPLOAD(id)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: REQUEST_PEDHEADSHOT_IMG_UPLOAD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `request_pedheadshot_img_upload_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn request_pedheadshot_img_upload_raw(id: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REQUEST_PEDHEADSHOT_IMG_UPLOAD(id)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**:

pub fn knock_off_ped_prop_safe(ped: Ped, p1: bool, p2: bool, p3: bool, p4: bool) -> NativeResult<()> {
    
    debug!("Calling native function: KNOCK_OFF_PED_PROP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::KNOCK_OFF_PED_PROP(ped, p1, p2, p3, p4)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: KNOCK_OFF_PED_PROP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `knock_off_ped_prop_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn knock_off_ped_prop_raw(ped: i32, p1: bool, p2: bool, p3: bool, p4: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::KNOCK_OFF_PED_PROP(ped, p1, p2, p3, p4)
}

/// ```
There seem to be 26 flags  
```

pub fn clear_ragdoll_blocking_flags_safe(ped: Ped, flags: i64) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_RAGDOLL_BLOCKING_FLAGS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_RAGDOLL_BLOCKING_FLAGS(ped, flags)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_RAGDOLL_BLOCKING_FLAGS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ragdoll_blocking_flags_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ragdoll_blocking_flags_raw(ped: i32, flags: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_RAGDOLL_BLOCKING_FLAGS(ped, flags)
}

/// ```
Creates a new NaturalMotion message.  
startImmediately: If set to true, the character will perform the message the moment it receives it by GIVE_PED_NM_MESSAGE. If false, the Ped will get the message but won't perform it yet. While it's a boolean value, if negative, the message will not be initialized.  
messageId: The ID of the NaturalMotion message.  
If a message already exists, this function does nothing. A message exists until the point it has been successfully dispatched by GIVE_PED_NM_MESSAGE.  
```

pub fn create_nm_message_safe(startImmediately: bool, messageId: i64) -> NativeResult<()> {
    
    debug!("Calling native function: CREATE_NM_MESSAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CREATE_NM_MESSAGE(startImmediately, messageId)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CREATE_NM_MESSAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `create_nm_message_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn create_nm_message_raw(startImmediately: bool, messageId: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CREATE_NM_MESSAGE(startImmediately, messageId)
}

/// ```
Ped will no longer get angry when you stay near him.  
```

pub fn remove_ped_defensive_area_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_PED_DEFENSIVE_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_PED_DEFENSIVE_AREA(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_PED_DEFENSIVE_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_ped_defensive_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_ped_defensive_area_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_PED_DEFENSIVE_AREA(ped, toggle)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**: 
* **p5**: 
* **p6**:

pub fn _0x2f074c904d85129e_safe(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0x2F074C904D85129E");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x2F074C904D85129E(p0, p1, p2, p3, p4, p5, p6)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x2F074C904D85129E
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x2f074c904d85129e_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x2f074c904d85129e_raw(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x2F074C904D85129E(p0, p1, p2, p3, p4, p5, p6)
}

/// ```
i could be time. Only example in the decompiled scripts uses it as -1.
```

pub fn set_ped_pinned_down_safe(ped: Ped, pinned: bool, i: i64) -> NativeResult<serde_json::Value> {
    
    debug!("Calling native function: SET_PED_PINNED_DOWN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_PINNED_DOWN(ped, pinned, i)
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: SET_PED_PINNED_DOWN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_pinned_down_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_pinned_down_raw(ped: i32, pinned: bool, i: i64) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_PINNED_DOWN(ped, pinned, i)
}

/// ## Parameters
* **ped**: 
* **groupId**:

pub fn is_ped_group_member_safe(ped: Ped, groupId: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_GROUP_MEMBER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_GROUP_MEMBER(ped, groupId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_GROUP_MEMBER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_group_member_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_group_member_raw(ped: i32, groupId: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_GROUP_MEMBER(ped, groupId)
}

/// Remove a helmet from a ped

pub fn remove_ped_helmet_safe(ped: Ped, instantly: bool) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_PED_HELMET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_PED_HELMET(ped, instantly)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_PED_HELMET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_ped_helmet_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_ped_helmet_raw(ped: i32, instantly: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_PED_HELMET(ped, instantly)
}

/// ```
NativeDB Introduced: v1290
```

pub fn _is_ped_body_blemish_valid_safe(colorID: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PED_BODY_BLEMISH_VALID");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PED_BODY_BLEMISH_VALID(colorID)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PED_BODY_BLEMISH_VALID
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_ped_body_blemish_valid_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_ped_body_blemish_valid_raw(colorID: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PED_BODY_BLEMISH_VALID(colorID)
}

/// ```
Prevents the ped from going limp.  
[Example: Can prevent peds from falling when standing on moving vehicles.]  
```

pub fn can_ped_ragdoll_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: CAN_PED_RAGDOLL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CAN_PED_RAGDOLL(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CAN_PED_RAGDOLL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `can_ped_ragdoll_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn can_ped_ragdoll_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CAN_PED_RAGDOLL(ped)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **p2**:

pub fn set_ped_can_smash_glass_safe(ped: Ped, p1: bool, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_SMASH_GLASS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_SMASH_GLASS(ped, p1, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_SMASH_GLASS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_smash_glass_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_smash_glass_raw(ped: i32, p1: bool, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_SMASH_GLASS(ped, p1, p2)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn request_ped_vehicle_visibility_tracking_safe(ped: Ped, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: REQUEST_PED_VEHICLE_VISIBILITY_TRACKING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REQUEST_PED_VEHICLE_VISIBILITY_TRACKING(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REQUEST_PED_VEHICLE_VISIBILITY_TRACKING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `request_ped_vehicle_visibility_tracking_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn request_ped_vehicle_visibility_tracking_raw(ped: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REQUEST_PED_VEHICLE_VISIBILITY_TRACKING(ped, p1)
}

/// ```c
enum ePedMotionState
{
    MOTIONSTATE_NONE = -294553821, // MotionState_None
    MOTIONSTATE_IDLE = -1871534317, // MotionState_Idle
    MOTIONSTATE_WALK = -668482597, // MotionState_Walk
    MOTIONSTATE_RUN = -530524, // MotionState_Run
    MOTIONSTATE_SPRINT = -1115154469, // MotionState_Sprint
    MOTIONSTATE_CROUCH_IDLE = 1140525470, // MotionState_Crouch_Idle
    MOTIONSTATE_CROUCH_WALK = 147004056, // MotionState_Crouch_Walk
    MOTIONSTATE_CROUCH_RUN = 898879241, // MotionState_Crouch_Run
    MOTIONSTATE_DONOTHING = 247561816, // MotionState_DoNothing
    MOTIONSTATE_ANIMATEDVELOCITY = 1427811395, // MotionState_AnimatedVelocity
    MOTIONSTATE_INVEHICLE = -1797663347, // MotionState_InVehicle
    MOTIONSTATE_AIMING = 1063765679, // MotionState_Aiming
    MOTIONSTATE_DIVING_IDLE = 1212730861, // MotionState_Diving_Idle
    MOTIONSTATE_DIVING_SWIM = -1855028596, // MotionState_Diving_Swim
    MOTIONSTATE_SWIMMING_TREADWATER = -776007225, // MotionState_Swimming_TreadWater
    MOTIONSTATE_DEAD = 230360860, // MotionState_Dead
    MOTIONSTATE_STEALTH_IDLE = 1110276645, // MotionState_Stealth_Idle
    MOTIONSTATE_STEALTH_WALK = 69908130, // MotionState_Stealth_Walk
    MOTIONSTATE_STEALTH_RUN = -83133983, // MotionState_Stealth_Run
    MOTIONSTATE_PARACHUTING = -1161760501, // MotionState_Parachuting
    MOTIONSTATE_ACTIONMODE_IDLE = -633298724, // MotionState_ActionMode_Idle
    MOTIONSTATE_ACTIONMODE_WALK = -762290521, // MotionState_ActionMode_Walk
    MOTIONSTATE_ACTIONMODE_RUN = 834330132, // MotionState_ActionMode_Run
    MOTIONSTATE_JETPACK = 1398696542 // MotionState_Jetpack
}
```

pub fn force_ped_motion_state_safe(ped: Ped, motionStateHash: u32, shouldReset: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: FORCE_PED_MOTION_STATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::FORCE_PED_MOTION_STATE(ped, motionStateHash, shouldReset)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: FORCE_PED_MOTION_STATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `force_ped_motion_state_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn force_ped_motion_state_raw(ped: i32, motionStateHash: u32, shouldReset: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::FORCE_PED_MOTION_STATE(ped, motionStateHash, shouldReset)
}

/// ```
Clears the relationship between two groups. This should be called twice (once for each group).  
Relationship types:  
0 = Companion  
1 = Respect  
2 = Like  
3 = Neutral  
4 = Dislike  
5 = Hate  
255 = Pedestrians  
(Credits: Inco)  
Example:  
PED::CLEAR_RELATIONSHIP_BETWEEN_GROUPS(2, l_1017, 0xA49E591C);  
PED::CLEAR_RELATIONSHIP_BETWEEN_GROUPS(2, 0xA49E591C, l_1017);  
```

pub fn clear_relationship_between_groups_safe(relationship: i64, group1: u32, group2: u32) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_RELATIONSHIP_BETWEEN_GROUPS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_RELATIONSHIP_BETWEEN_GROUPS(relationship, group1, group2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_RELATIONSHIP_BETWEEN_GROUPS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_relationship_between_groups_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_relationship_between_groups_raw(relationship: i64, group1: u32, group2: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_RELATIONSHIP_BETWEEN_GROUPS(relationship, group1, group2)
}

/// For more info please refer to [this](https://gtaforums.com/topic/858970-all-gtao-face-ids-pedset-ped-head-blend-data-explained) topic.

pub fn set_ped_head_blend_data_safe(ped: Ped, shapeFirstID: i64, shapeSecondID: i64, shapeThirdID: i64, skinFirstID: i64, skinSecondID: i64, skinThirdID: i64, shapeMix: f32, skinMix: f32, thirdMix: f32, isParent: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_HEAD_BLEND_DATA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_HEAD_BLEND_DATA(ped, shapeFirstID, shapeSecondID, shapeThirdID, skinFirstID, skinSecondID, skinThirdID, shapeMix, skinMix, thirdMix, isParent)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_HEAD_BLEND_DATA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_head_blend_data_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_head_blend_data_raw(ped: i32, shapeFirstID: i64, shapeSecondID: i64, shapeThirdID: i64, skinFirstID: i64, skinSecondID: i64, skinThirdID: i64, shapeMix: f32, skinMix: f32, thirdMix: f32, isParent: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_HEAD_BLEND_DATA(ped, shapeFirstID, shapeSecondID, shapeThirdID, skinFirstID, skinSecondID, skinThirdID, shapeMix, skinMix, thirdMix, isParent)
}

/// Checks if the component variation is valid, this works great for randomizing components using loops.

pub fn is_ped_component_variation_valid_safe(ped: Ped, componentId: i64, drawableId: i64, textureId: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_COMPONENT_VARIATION_VALID");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_COMPONENT_VARIATION_VALID(ped, componentId, drawableId, textureId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_COMPONENT_VARIATION_VALID
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_component_variation_valid_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_component_variation_valid_raw(ped: i32, componentId: i64, drawableId: i64, textureId: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_COMPONENT_VARIATION_VALID(ped, componentId, drawableId, textureId)
}

/// ## Parameters
* **ped**:

pub fn get_ped_helmet_stored_hat_prop_index_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_HELMET_STORED_HAT_PROP_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_HELMET_STORED_HAT_PROP_INDEX(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_HELMET_STORED_HAT_PROP_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_helmet_stored_hat_prop_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_helmet_stored_hat_prop_index_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_HELMET_STORED_HAT_PROP_INDEX(ped)
}

/// Used one time in fmmc_launcher.c instead of CLONE_PED because ?

pub fn _clone_ped_ex_safe(ped: Ped) -> NativeResult<Ped> {
    
    debug!("Calling native function: _CLONE_PED_EX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_CLONE_PED_EX(ped)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: _CLONE_PED_EX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_clone_ped_ex_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _clone_ped_ex_raw(ped: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_CLONE_PED_EX(ped)
}

/// ```
Ids
0 - Head
1 - Beard
2 - Hair
3 - Torso
4 - Legs
5 - Hands
6 - Foot
7 - Scarfs/Neck Accessories
8 - Accessories 1
9 - Accessories 2
10- Decals
11 - Auxiliary parts for torso
```

pub fn get_ped_drawable_variation_safe(ped: Ped, componentId: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_DRAWABLE_VARIATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_DRAWABLE_VARIATION(ped, componentId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_DRAWABLE_VARIATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_drawable_variation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_drawable_variation_raw(ped: i32, componentId: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_DRAWABLE_VARIATION(ped, componentId)
}

/// ## Parameters
* **ped**:

pub fn is_ped_running_melee_task_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_RUNNING_MELEE_TASK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_RUNNING_MELEE_TASK(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_RUNNING_MELEE_TASK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_running_melee_task_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_running_melee_task_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_RUNNING_MELEE_TASK(ped)
}

/// ```
Used for freemode (online) characters. 
Called after SET_PED_HEAD_OVERLAY().  
```

pub fn _set_ped_head_overlay_color_safe(ped: Ped, overlayID: i64, colorType: i64, colorID: i64, secondColorID: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PED_HEAD_OVERLAY_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PED_HEAD_OVERLAY_COLOR(ped, overlayID, colorType, colorID, secondColorID)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PED_HEAD_OVERLAY_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_ped_head_overlay_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_ped_head_overlay_color_raw(ped: i32, overlayID: i64, colorType: i64, colorID: i64, secondColorID: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PED_HEAD_OVERLAY_COLOR(ped, overlayID, colorType, colorID, secondColorID)
}

/// ## Parameters
* **ped**: 
* **team**: 
* **toggle**:

pub fn set_ped_can_be_targetted_by_team_safe(ped: Ped, team: i64, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_BE_TARGETTED_BY_TEAM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_BE_TARGETTED_BY_TEAM(ped, team, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_BE_TARGETTED_BY_TEAM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_be_targetted_by_team_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_be_targetted_by_team_raw(ped: i32, team: i64, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_BE_TARGETTED_BY_TEAM(ped, team, toggle)
}

/// ```
SET_PED_ALLOW*
```

pub fn _0x49e50bdb8ba4dab2_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x49E50BDB8BA4DAB2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x49E50BDB8BA4DAB2(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x49E50BDB8BA4DAB2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x49e50bdb8ba4dab2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x49e50bdb8ba4dab2_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x49E50BDB8BA4DAB2(ped, toggle)
}

/// ## Parameters
* **id**:

pub fn release_pedheadshot_img_upload_safe(id: i64) -> NativeResult<()> {
    
    debug!("Calling native function: RELEASE_PEDHEADSHOT_IMG_UPLOAD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RELEASE_PEDHEADSHOT_IMG_UPLOAD(id)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RELEASE_PEDHEADSHOT_IMG_UPLOAD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `release_pedheadshot_img_upload_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn release_pedheadshot_img_upload_raw(id: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RELEASE_PEDHEADSHOT_IMG_UPLOAD(id)
}

/// ```
Gets a value indicating whether this ped's health is below its fatally injured threshold. The default threshold is 100.  
If the handle is invalid, the function returns true.  
```

pub fn is_ped_fatally_injured_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_FATALLY_INJURED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_FATALLY_INJURED(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_FATALLY_INJURED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_fatally_injured_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_fatally_injured_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_FATALLY_INJURED(ped)
}

/// ```
Sets Ped Default Clothes  
```

pub fn set_ped_default_component_variation_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_DEFAULT_COMPONENT_VARIATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_DEFAULT_COMPONENT_VARIATION(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_DEFAULT_COMPONENT_VARIATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_default_component_variation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_default_component_variation_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_DEFAULT_COMPONENT_VARIATION(ped)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn set_ped_can_torso_vehicle_ik_safe(ped: Ped, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_TORSO_VEHICLE_IK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_TORSO_VEHICLE_IK(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_TORSO_VEHICLE_IK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_torso_vehicle_ik_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_torso_vehicle_ik_raw(ped: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_TORSO_VEHICLE_IK(ped, p1)
}

/// ```
FIRING_PATTERN_BURST_FIRE = 0xD6FF6D61 ( 1073727030 )  
FIRING_PATTERN_BURST_FIRE_IN_COVER = 0x026321F1 ( 40051185 )  
FIRING_PATTERN_BURST_FIRE_DRIVEBY = 0xD31265F2 ( -753768974 )  
FIRING_PATTERN_FROM_GROUND = 0x2264E5D6 ( 577037782 )  
FIRING_PATTERN_DELAY_FIRE_BY_ONE_SEC = 0x7A845691 ( 2055493265 )  
FIRING_PATTERN_FULL_AUTO = 0xC6EE6B4C ( -957453492 )  
FIRING_PATTERN_SINGLE_SHOT = 0x5D60E4E0 ( 1566631136 )  
FIRING_PATTERN_BURST_FIRE_PISTOL = 0xA018DB8A ( -1608983670 )  
FIRING_PATTERN_BURST_FIRE_SMG = 0xD10DADEE ( 1863348768 )  
FIRING_PATTERN_BURST_FIRE_RIFLE = 0x9C74B406 ( -1670073338 )  
FIRING_PATTERN_BURST_FIRE_MG = 0xB573C5B4 ( -1250703948 )  
FIRING_PATTERN_BURST_FIRE_PUMPSHOTGUN = 0x00BAC39B ( 12239771 )  
FIRING_PATTERN_BURST_FIRE_HELI = 0x914E786F ( -1857128337 )  
FIRING_PATTERN_BURST_FIRE_MICRO = 0x42EF03FD ( 1122960381 )  
FIRING_PATTERN_SHORT_BURSTS = 0x1A92D7DF ( 445831135 )  
FIRING_PATTERN_SLOW_FIRE_TANK = 0xE2CA3A71 ( -490063247 )  
if anyone is interested firing pattern info: pastebin.com/Px036isB  
```

pub fn set_ped_firing_pattern_safe(ped: Ped, patternHash: u32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_FIRING_PATTERN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_FIRING_PATTERN(ped, patternHash)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_FIRING_PATTERN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_firing_pattern_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_firing_pattern_raw(ped: i32, patternHash: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_FIRING_PATTERN(ped, patternHash)
}

/// ```
xyz - relative to the world origin.  
```

pub fn is_cop_ped_in_area_3d_safe(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_COP_PED_IN_AREA_3D");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_COP_PED_IN_AREA_3D(x1, y1, z1, x2, y2, z2)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_COP_PED_IN_AREA_3D
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_cop_ped_in_area_3d_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_cop_ped_in_area_3d_raw(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_COP_PED_IN_AREA_3D(x1, y1, z1, x2, y2, z2)
}

/// ```
This only will teleport the ped to the group leader if the group leader teleports (sets coords).  
Only works in singleplayer  
```

pub fn set_ped_can_teleport_to_group_leader_safe(pedHandle: Ped, groupHandle: i64, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_TELEPORT_TO_GROUP_LEADER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_TELEPORT_TO_GROUP_LEADER(pedHandle, groupHandle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_TELEPORT_TO_GROUP_LEADER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_teleport_to_group_leader_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_teleport_to_group_leader_raw(pedHandle: i32, groupHandle: i64, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_TELEPORT_TO_GROUP_LEADER(pedHandle, groupHandle, toggle)
}

/// ## Parameters
* **ped**:

pub fn is_ped_doing_driveby_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_DOING_DRIVEBY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_DOING_DRIVEBY(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_DOING_DRIVEBY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_doing_driveby_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_doing_driveby_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_DOING_DRIVEBY(ped)
}

/// ```
FORCE_*
```

pub fn _0xed3c76adfa6d07c4_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: _0xED3C76ADFA6D07C4");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xED3C76ADFA6D07C4(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xED3C76ADFA6D07C4
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xed3c76adfa6d07c4_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xed3c76adfa6d07c4_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xED3C76ADFA6D07C4(ped)
}

/// ## Parameters
* **ped**: 
* **clipSet**:

pub fn set_ped_strafe_clipset_safe(ped: Ped, clipSet: String) -> NativeResult<()> {
    let clipSet_cstr = std::ffi::CString::new(clipSet.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "clipSet", e)))?;
    
    debug!("Calling native function: SET_PED_STRAFE_CLIPSET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_STRAFE_CLIPSET(ped, crate::utils::rust_to_c_string(clipSet))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_STRAFE_CLIPSET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_strafe_clipset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_strafe_clipset_raw(ped: i32, clipSet: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_STRAFE_CLIPSET(ped, clipSet)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_dies_in_vehicle_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_DIES_IN_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_DIES_IN_VEHICLE(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_DIES_IN_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_dies_in_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_dies_in_vehicle_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_DIES_IN_VEHICLE(ped, toggle)
}

/// ## Parameters
* **ped**: 
* **itemSet**:

pub fn set_ped_preferred_cover_set_safe(ped: Ped, itemSet: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let itemSet_any_str = serde_json::to_string(&itemSet)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "itemSet", e)))?;
    let itemSet_any_str_cstr = std::ffi::CString::new(itemSet_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "itemSet", e)))?;
    
    debug!("Calling native function: SET_PED_PREFERRED_COVER_SET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_PREFERRED_COVER_SET(ped, crate::utils::any_to_c_void_ptr(itemSet))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_PREFERRED_COVER_SET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_preferred_cover_set_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_preferred_cover_set_raw(ped: i32, itemSet: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_PREFERRED_COVER_SET(ped, itemSet)
}

/// ```
GET_*
```

pub fn _0x1e77fa7a62ee6c4c_safe(p0: serde_json::Value) -> NativeResult<serde_json::Value> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x1E77FA7A62EE6C4C");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x1E77FA7A62EE6C4C(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0x1E77FA7A62EE6C4C
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x1e77fa7a62ee6c4c_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x1e77fa7a62ee6c4c_raw(p0: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x1E77FA7A62EE6C4C(p0)
}

/// ## Parameters
* **ped**: 
* **modelHash**:

pub fn is_ped_in_model_safe(ped: Ped, modelHash: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_MODEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_MODEL(ped, modelHash)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_MODEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_model_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_model_raw(ped: i32, modelHash: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_MODEL(ped, modelHash)
}

/// Examines whether the ped is engaged in combat; when given a target ped index, it confirms if the ped is actively fighting the specified target, returning true if engaged and false if not.

pub fn is_ped_in_combat_safe(ped: Ped, target: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_COMBAT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_COMBAT(ped, target)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_COMBAT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_combat_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_combat_raw(ped: i32, target: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_COMBAT(ped, target)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_dies_in_water_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_DIES_IN_WATER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_DIES_IN_WATER(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_DIES_IN_WATER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_dies_in_water_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_dies_in_water_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_DIES_IN_WATER(ped, toggle)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0xa9b61a329bfdcbea_safe(p0: serde_json::Value, p1: bool) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xA9B61A329BFDCBEA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xA9B61A329BFDCBEA(crate::utils::any_to_c_void_ptr(p0), p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xA9B61A329BFDCBEA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xa9b61a329bfdcbea_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xa9b61a329bfdcbea_raw(p0: *mut std::os::raw::c_void, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xA9B61A329BFDCBEA(p0, p1)
}

/// ## Parameters
* **ped**:

pub fn have_all_streaming_requests_completed_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: HAVE_ALL_STREAMING_REQUESTS_COMPLETED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAVE_ALL_STREAMING_REQUESTS_COMPLETED(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAVE_ALL_STREAMING_REQUESTS_COMPLETED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `have_all_streaming_requests_completed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn have_all_streaming_requests_completed_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAVE_ALL_STREAMING_REQUESTS_COMPLETED(ped)
}

/// ## Parameters
* **ped**:

pub fn is_ped_being_stealth_killed_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_BEING_STEALTH_KILLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_BEING_STEALTH_KILLED(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_BEING_STEALTH_KILLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_being_stealth_killed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_being_stealth_killed_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_BEING_STEALTH_KILLED(ped)
}

/// ```
This is only called once in the scripts.
sub_1CD9(&l_49, 0, getElem(3, &l_34, 4), "MICHAEL", 0, 1);
                    sub_1CA8("WORLD_HUMAN_SMOKING", 2);
                    PED::SET_PED_PRIMARY_LOOKAT(getElem(3, &l_34, 4), PLAYER::PLAYER_PED_ID());
```

pub fn set_ped_primary_lookat_safe(ped: Ped, lookAt: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_PRIMARY_LOOKAT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_PRIMARY_LOOKAT(ped, lookAt)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_PRIMARY_LOOKAT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_primary_lookat_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_primary_lookat_raw(ped: i32, lookAt: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_PRIMARY_LOOKAT(ped, lookAt)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn set_ped_reserve_parachute_tint_index_safe(ped: Ped, p1: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p1_any_str = serde_json::to_string(&p1)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p1", e)))?;
    let p1_any_str_cstr = std::ffi::CString::new(p1_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p1", e)))?;
    
    debug!("Calling native function: SET_PED_RESERVE_PARACHUTE_TINT_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_RESERVE_PARACHUTE_TINT_INDEX(ped, crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_RESERVE_PARACHUTE_TINT_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_reserve_parachute_tint_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_reserve_parachute_tint_index_raw(ped: i32, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_RESERVE_PARACHUTE_TINT_INDEX(ped, p1)
}

/// ## Parameters
* **ped**:

pub fn is_ped_performing_dependent_combo_limit_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_PERFORMING_DEPENDENT_COMBO_LIMIT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_PERFORMING_DEPENDENT_COMBO_LIMIT(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_PERFORMING_DEPENDENT_COMBO_LIMIT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_performing_dependent_combo_limit_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_performing_dependent_combo_limit_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_PERFORMING_DEPENDENT_COMBO_LIMIT(ped)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **p2**:

pub fn hide_ped_blood_damage_by_zone_safe(ped: Ped, p1: serde_json::Value, p2: bool) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p1_any_str = serde_json::to_string(&p1)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p1", e)))?;
    let p1_any_str_cstr = std::ffi::CString::new(p1_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p1", e)))?;
    
    debug!("Calling native function: HIDE_PED_BLOOD_DAMAGE_BY_ZONE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HIDE_PED_BLOOD_DAMAGE_BY_ZONE(ped, crate::utils::any_to_c_void_ptr(p1), p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: HIDE_PED_BLOOD_DAMAGE_BY_ZONE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `hide_ped_blood_damage_by_zone_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn hide_ped_blood_damage_by_zone_raw(ped: i32, p1: *mut std::os::raw::c_void, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HIDE_PED_BLOOD_DAMAGE_BY_ZONE(ped, p1, p2)
}

/// This native sets a scuba mask for freemode models and an oxygen bottle for player_* models. It works on freemode and player_* models.

pub fn _set_ped_scuba_gear_variation_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PED_SCUBA_GEAR_VARIATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PED_SCUBA_GEAR_VARIATION(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PED_SCUBA_GEAR_VARIATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_ped_scuba_gear_variation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_ped_scuba_gear_variation_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PED_SCUBA_GEAR_VARIATION(ped)
}

/// ## Parameters
* **ped**:

pub fn get_ped_enveff_scale_safe(ped: Ped) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_PED_ENVEFF_SCALE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_ENVEFF_SCALE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_ENVEFF_SCALE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_enveff_scale_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_enveff_scale_raw(ped: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_ENVEFF_SCALE(ped)
}

/// ## Parameters
* **sceneID**: 
* **toggle**:

pub fn set_synchronized_scene_looped_safe(sceneID: i64, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SYNCHRONIZED_SCENE_LOOPED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SYNCHRONIZED_SCENE_LOOPED(sceneID, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SYNCHRONIZED_SCENE_LOOPED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_synchronized_scene_looped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_synchronized_scene_looped_raw(sceneID: i64, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SYNCHRONIZED_SCENE_LOOPED(sceneID, toggle)
}

/// ```
NativeDB Introduced: v1290
```

pub fn _0xad27d957598e49e9_safe(ped: Ped, p1: serde_json::Value, p2: f32) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p1_any_str = serde_json::to_string(&p1)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p1", e)))?;
    let p1_any_str_cstr = std::ffi::CString::new(p1_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p1", e)))?;
    
    debug!("Calling native function: _0xAD27D957598E49E9");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xAD27D957598E49E9(ped, crate::utils::any_to_c_void_ptr(p1), p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xAD27D957598E49E9
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xad27d957598e49e9_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xad27d957598e49e9_raw(ped: i32, p1: *mut std::os::raw::c_void, p2: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xAD27D957598E49E9(ped, p1, p2)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0xceda60a74219d064_safe(p0: serde_json::Value, p1: bool) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xCEDA60A74219D064");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xCEDA60A74219D064(crate::utils::any_to_c_void_ptr(p0), p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xCEDA60A74219D064
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xceda60a74219d064_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xceda60a74219d064_raw(p0: *mut std::os::raw::c_void, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xCEDA60A74219D064(p0, p1)
}

/// ## Parameters
* **ped**: 
* **x1**: 
* **y1**: 
* **z1**: 
* **x2**: 
* **y2**: 
* **z2**: 
* **p7**: 
* **p8**:

pub fn is_ped_shooting_in_area_safe(ped: Ped, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, p7: bool, p8: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_SHOOTING_IN_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_SHOOTING_IN_AREA(ped, x1, y1, z1, x2, y2, z2, p7, p8)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_SHOOTING_IN_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_shooting_in_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_shooting_in_area_raw(ped: i32, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, p7: bool, p8: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_SHOOTING_IN_AREA(ped, x1, y1, z1, x2, y2, z2, p7, p8)
}

/// Determines if a ped is dead. Contrary to what the name might suggest, it does not always detect when a ped is in the 'dying' phase (transitioning to death). The exception is when `checkMeleeDeathFlags` is set to `true`, which then includes peds in the midst of melee takedown moves as being in a dying state, even if the death task has not yet started.

```
NativeDB Introduced: v323
```

pub fn is_ped_dead_or_dying_safe(ped: Ped, checkMeleeDeathFlags: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_DEAD_OR_DYING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_DEAD_OR_DYING(ped, checkMeleeDeathFlags)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_DEAD_OR_DYING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_dead_or_dying_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_dead_or_dying_raw(ped: i32, checkMeleeDeathFlags: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_DEAD_OR_DYING(ped, checkMeleeDeathFlags)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_blocks_pathing_when_dead_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_BLOCKS_PATHING_WHEN_DEAD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_BLOCKS_PATHING_WHEN_DEAD(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_BLOCKS_PATHING_WHEN_DEAD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_blocks_pathing_when_dead_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_blocks_pathing_when_dead_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_BLOCKS_PATHING_WHEN_DEAD(ped, toggle)
}

/// ## Parameters
* **ped**: 
* **outTintIndex**:

pub fn get_ped_parachute_tint_index_safe(ped: Ped, outTintIndex: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_PED_PARACHUTE_TINT_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_PARACHUTE_TINT_INDEX(ped, outTintIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_PED_PARACHUTE_TINT_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_parachute_tint_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_parachute_tint_index_raw(ped: i32, outTintIndex: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_PARACHUTE_TINT_INDEX(ped, outTintIndex)
}

/// ```
PED::REGISTER_TARGET(l_216, PLAYER::PLAYER_PED_ID()); from re_prisonbreak.txt.  
l_216 = RECSBRobber1  
```

pub fn register_target_safe(ped: Ped, target: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: REGISTER_TARGET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REGISTER_TARGET(ped, target)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REGISTER_TARGET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `register_target_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn register_target_raw(ped: i32, target: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REGISTER_TARGET(ped, target)
}

/// ## Parameters
* **ped**:

pub fn disable_ped_heatscale_override_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: DISABLE_PED_HEATSCALE_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DISABLE_PED_HEATSCALE_OVERRIDE(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DISABLE_PED_HEATSCALE_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `disable_ped_heatscale_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn disable_ped_heatscale_override_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DISABLE_PED_HEATSCALE_OVERRIDE(ped)
}

/// Gives the ped a helmet. Can be removed by invoking [`REMOVE_PED_HELMET`](#_0xA7B2458D0AD6DED8).

```c
enum ePedCompFlags {
  PV_FLAG_NONE                  = 0, // 0
  PV_FLAG_BULKY                 = 1, // 1<<0
  PV_FLAG_JOB                   = 2, // 1<<1
  PV_FLAG_SUNNY                 = 4, // 1<<2
  PV_FLAG_WET                   = 8, // 1<<3
  PV_FLAG_COLD                  = 16, // 1<<4
  PV_FLAG_NOT_IN_CAR            = 32, // 1<<5
  PV_FLAG_BIKE_ONLY             = 64, // 1<<6
  PV_FLAG_NOT_INDOORS           = 128, // 1<<7
  PV_FLAG_FIRE_RETARDENT        = 256, // 1<<8
  PV_FLAG_ARMOURED              = 512, // 1<<9
  PV_FLAG_LIGHTLY_ARMOURED      = 1024, // 1<<10
  PV_FLAG_HIGH_DETAIL           = 2048, // 1<<11
  PV_FLAG_DEFAULT_HELMET        = 4096, // 1<<12
  PV_FLAG_RANDOM_HELMET         = 8192, // 1<<13
  PV_FLAG_SCRIPT_HELMET         = 16384, // 1<<14
  PV_FLAG_FLIGHT_HELMET         = 32768, // 1<<15
  PV_FLAG_HIDE_IN_FIRST_PERSON  = 65536, // 1<<16
  PV_FLAG_USE_PHYSICS_HAT_2     = 131072, // 1<<17
  PV_FLAG_PILOT_HELMET          = 262144 // 1<<18
};
```

pub fn give_ped_helmet_safe(ped: Ped, cannotRemove: bool, helmetFlag: i64, textureIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: GIVE_PED_HELMET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GIVE_PED_HELMET(ped, cannotRemove, helmetFlag, textureIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GIVE_PED_HELMET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `give_ped_helmet_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn give_ped_helmet_raw(ped: i32, cannotRemove: bool, helmetFlag: i64, textureIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GIVE_PED_HELMET(ped, cannotRemove, helmetFlag, textureIndex)
}

/// Input: Makeup color index, value between 0 and 63 (inclusive).
Output: RGB values for the makeup color specified in the input.

This is used with the makeup color swatches scaleform.

Use [`_0x4852FC386E2E1BB5`](#_0x4852FC386E2E1BB5) to get the hair colors.

pub fn _get_ped_makeup_rgb_color_safe(makeupColorIndex: i64, outR: *mut i64, outG: *mut i64, outB: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: _GET_PED_MAKEUP_RGB_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_PED_MAKEUP_RGB_COLOR(makeupColorIndex, outR, outG, outB)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _GET_PED_MAKEUP_RGB_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_ped_makeup_rgb_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_ped_makeup_rgb_color_raw(makeupColorIndex: i64, outR: *mut i64, outG: *mut i64, outB: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_PED_MAKEUP_RGB_COLOR(makeupColorIndex, outR, outG, outB)
}

/// ```c
enum ePedDecorationZone
{
	ZONE_TORSO = 0,
	ZONE_HEAD = 1,
	ZONE_LEFT_ARM = 2,
	ZONE_RIGHT_ARM = 3,
	ZONE_LEFT_LEG = 4,
	ZONE_RIGHT_LEG = 5,
	ZONE_UNKNOWN = 6,
	ZONE_NONE = 7
};
```

pub fn get_ped_decoration_zone_from_hashes_safe(collection: u32, overlay: u32) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_DECORATION_ZONE_FROM_HASHES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_DECORATION_ZONE_FROM_HASHES(collection, overlay)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_DECORATION_ZONE_FROM_HASHES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_decoration_zone_from_hashes_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_decoration_zone_from_hashes_raw(collection: u32, overlay: u32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_DECORATION_ZONE_FROM_HASHES(collection, overlay)
}

/// ## Parameters
* **ped**: 
* **posX**: 
* **posY**: 
* **posZ**:

pub fn set_ped_coords_no_gang_safe(ped: Ped, posX: f32, posY: f32, posZ: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_COORDS_NO_GANG");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_COORDS_NO_GANG(ped, posX, posY, posZ)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_COORDS_NO_GANG
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_coords_no_gang_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_coords_no_gang_raw(ped: i32, posX: f32, posY: f32, posZ: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_COORDS_NO_GANG(ped, posX, posY, posZ)
}

/// ## Parameters
* **ped**: 
* **heatScale**:

pub fn set_ped_heatscale_override_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_HEATSCALE_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_HEATSCALE_OVERRIDE(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_HEATSCALE_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_heatscale_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_heatscale_override_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_HEATSCALE_OVERRIDE(ped)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_steers_around_vehicles_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_STEERS_AROUND_VEHICLES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_STEERS_AROUND_VEHICLES(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_STEERS_AROUND_VEHICLES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_steers_around_vehicles_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_steers_around_vehicles_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_STEERS_AROUND_VEHICLES(ped, toggle)
}

/// ```
Setting ped to true allows the ped to shoot "friendlies".  
p2 set to true when toggle is also true seams to make peds permanently unable to aim at, even if you set p2 back to false.  
p1 = false & p2 = false for unable to aim at.  
p1 = true & p2 = false for able to aim at.  
```

pub fn set_can_attack_friendly_safe(ped: Ped, toggle: bool, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CAN_ATTACK_FRIENDLY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CAN_ATTACK_FRIENDLY(ped, toggle, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CAN_ATTACK_FRIENDLY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_can_attack_friendly_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_can_attack_friendly_raw(ped: i32, toggle: bool, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CAN_ATTACK_FRIENDLY(ped, toggle, p2)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_switch_weapon_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_SWITCH_WEAPON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_SWITCH_WEAPON(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_SWITCH_WEAPON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_switch_weapon_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_switch_weapon_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_SWITCH_WEAPON(ped, toggle)
}

/// ```
Gets a value indicating whether the specified ped is in the specified vehicle.  
If 'atGetIn' is false, the function will not return true until the ped is sitting in the vehicle and is about to close the door. If it's true, the function returns true the moment the ped starts to get onto the seat (after opening the door). Eg. if false, and the ped is getting into a submersible, the function will not return true until the ped has descended down into the submersible and gotten into the seat, while if it's true, it'll return true the moment the hatch has been opened and the ped is about to descend into the submersible.  
```

pub fn is_ped_in_vehicle_safe(ped: Ped, vehicle: Vehicle, atGetIn: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_VEHICLE(ped, vehicle, atGetIn)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_vehicle_raw(ped: i32, vehicle: i32, atGetIn: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_VEHICLE(ped, vehicle, atGetIn)
}

/// ## Parameters
* **ped**:

pub fn is_ped_diving_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_DIVING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_DIVING(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_DIVING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_diving_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_diving_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_DIVING(ped)
}

/// ```c
enum ePedBoneId : uint16_t
{
    SKEL_ROOT = 0x0,
    SKEL_Pelvis = 0x2E28,
    SKEL_L_Thigh = 0xE39F,
    SKEL_L_Calf = 0xF9BB,
    SKEL_L_Foot = 0x3779,
    SKEL_L_Toe0 = 0x83C,
    EO_L_Foot = 0x84C5,
    EO_L_Toe = 0x68BD,
    IK_L_Foot = 0xFEDD,
    PH_L_Foot = 0xE175,
    MH_L_Knee = 0xB3FE,
    SKEL_R_Thigh = 0xCA72,
    SKEL_R_Calf = 0x9000,
    SKEL_R_Foot = 0xCC4D,
    SKEL_R_Toe0 = 0x512D,
    EO_R_Foot = 0x1096,
    EO_R_Toe = 0x7163,
    IK_R_Foot = 0x8AAE,
    PH_R_Foot = 0x60E6,
    MH_R_Knee = 0x3FCF,
    RB_L_ThighRoll = 0x5C57,
    RB_R_ThighRoll = 0x192A,
    SKEL_Spine_Root = 0xE0FD,
    SKEL_Spine0 = 0x5C01,
    SKEL_Spine1 = 0x60F0,
    SKEL_Spine2 = 0x60F1,
    SKEL_Spine3 = 0x60F2,
    SKEL_L_Clavicle = 0xFCD9,
    SKEL_L_UpperArm = 0xB1C5,
    SKEL_L_Forearm = 0xEEEB,
    SKEL_L_Hand = 0x49D9,
    SKEL_L_Finger00 = 0x67F2,
    SKEL_L_Finger01 = 0xFF9,
    SKEL_L_Finger02 = 0xFFA,
    SKEL_L_Finger10 = 0x67F3,
    SKEL_L_Finger11 = 0x1049,
    SKEL_L_Finger12 = 0x104A,
    SKEL_L_Finger20 = 0x67F4,
    SKEL_L_Finger21 = 0x1059,
    SKEL_L_Finger22 = 0x105A,
    SKEL_L_Finger30 = 0x67F5,
    SKEL_L_Finger31 = 0x1029,
    SKEL_L_Finger32 = 0x102A,
    SKEL_L_Finger40 = 0x67F6,
    SKEL_L_Finger41 = 0x1039,
    SKEL_L_Finger42 = 0x103A,
    PH_L_Hand = 0xEB95,
    IK_L_Hand = 0x8CBD,
    RB_L_ForeArmRoll = 0xEE4F,
    RB_L_ArmRoll = 0x1470,
    MH_L_Elbow = 0x58B7,
    SKEL_R_Clavicle = 0x29D2,
    SKEL_R_UpperArm = 0x9D4D,
    SKEL_R_Forearm = 0x6E5C,
    SKEL_R_Hand = 0xDEAD,
    SKEL_R_Finger00 = 0xE5F2,
    SKEL_R_Finger01 = 0xFA10,
    SKEL_R_Finger02 = 0xFA11,
    SKEL_R_Finger10 = 0xE5F3,
    SKEL_R_Finger11 = 0xFA60,
    SKEL_R_Finger12 = 0xFA61,
    SKEL_R_Finger20 = 0xE5F4,
    SKEL_R_Finger21 = 0xFA70,
    SKEL_R_Finger22 = 0xFA71,
    SKEL_R_Finger30 = 0xE5F5,
    SKEL_R_Finger31 = 0xFA40,
    SKEL_R_Finger32 = 0xFA41,
    SKEL_R_Finger40 = 0xE5F6,
    SKEL_R_Finger41 = 0xFA50,
    SKEL_R_Finger42 = 0xFA51,
    PH_R_Hand = 0x6F06,
    IK_R_Hand = 0x188E,
    RB_R_ForeArmRoll = 0xAB22,
    RB_R_ArmRoll = 0x90FF,
    MH_R_Elbow = 0xBB0,
    SKEL_Neck_1 = 0x9995,
    SKEL_Head = 0x796E,
    IK_Head = 0x322C,
    FACIAL_facialRoot = 0xFE2C,
    FB_L_Brow_Out_000 = 0xE3DB,
    FB_L_Lid_Upper_000 = 0xB2B6,
    FB_L_Eye_000 = 0x62AC,
    FB_L_CheekBone_000 = 0x542E,
    FB_L_Lip_Corner_000 = 0x74AC,
    FB_R_Lid_Upper_000 = 0xAA10,
    FB_R_Eye_000 = 0x6B52,
    FB_R_CheekBone_000 = 0x4B88,
    FB_R_Brow_Out_000 = 0x54C,
    FB_R_Lip_Corner_000 = 0x2BA6,
    FB_Brow_Centre_000 = 0x9149,
    FB_UpperLipRoot_000 = 0x4ED2,
    FB_UpperLip_000 = 0xF18F,
    FB_L_Lip_Top_000 = 0x4F37,
    FB_R_Lip_Top_000 = 0x4537,
    FB_Jaw_000 = 0xB4A0,
    FB_LowerLipRoot_000 = 0x4324,
    FB_LowerLip_000 = 0x508F,
    FB_L_Lip_Bot_000 = 0xB93B,
    FB_R_Lip_Bot_000 = 0xC33B,
    FB_Tongue_000 = 0xB987,
    RB_Neck_1 = 0x8B93,
    SPR_L_Breast = 0xFC8E,
    SPR_R_Breast = 0x885F,
    IK_Root = 0xDD1C,
    SKEL_Neck_2 = 0x5FD4,
    SKEL_Pelvis1 = 0xD003,
    SKEL_PelvisRoot = 0x45FC,
    SKEL_SADDLE = 0x9524,
    MH_L_CalfBack = 0x1013,
    MH_L_ThighBack = 0x600D,
    SM_L_Skirt = 0xC419,
    MH_R_CalfBack = 0xB013,
    MH_R_ThighBack = 0x51A3,
    SM_R_Skirt = 0x7712,
    SM_M_BackSkirtRoll = 0xDBB,
    SM_L_BackSkirtRoll = 0x40B2,
    SM_R_BackSkirtRoll = 0xC141,
    SM_M_FrontSkirtRoll = 0xCDBB,
    SM_L_FrontSkirtRoll = 0x9B69,
    SM_R_FrontSkirtRoll = 0x86F1,
    SM_CockNBalls_ROOT = 0xC67D,
    SM_CockNBalls = 0x9D34,
    MH_L_Finger00 = 0x8C63,
    MH_L_FingerBulge00 = 0x5FB8,
    MH_L_Finger10 = 0x8C53,
    MH_L_FingerTop00 = 0xA244,
    MH_L_HandSide = 0xC78A,
    MH_Watch = 0x2738,
    MH_L_Sleeve = 0x933C,
    MH_R_Finger00 = 0x2C63,
    MH_R_FingerBulge00 = 0x69B8,
    MH_R_Finger10 = 0x2C53,
    MH_R_FingerTop00 = 0xEF4B,
    MH_R_HandSide = 0x68FB,
    MH_R_Sleeve = 0x92DC,
    FACIAL_jaw = 0xB21,
    FACIAL_underChin = 0x8A95,
    FACIAL_L_underChin = 0x234E,
    FACIAL_chin = 0xB578,
    FACIAL_chinSkinBottom = 0x98BC,
    FACIAL_L_chinSkinBottom = 0x3E8F,
    FACIAL_R_chinSkinBottom = 0x9E8F,
    FACIAL_tongueA = 0x4A7C,
    FACIAL_tongueB = 0x4A7D,
    FACIAL_tongueC = 0x4A7E,
    FACIAL_tongueD = 0x4A7F,
    FACIAL_tongueE = 0x4A80,
    FACIAL_L_tongueE = 0x35F2,
    FACIAL_R_tongueE = 0x2FF2,
    FACIAL_L_tongueD = 0x35F1,
    FACIAL_R_tongueD = 0x2FF1,
    FACIAL_L_tongueC = 0x35F0,
    FACIAL_R_tongueC = 0x2FF0,
    FACIAL_L_tongueB = 0x35EF,
    FACIAL_R_tongueB = 0x2FEF,
    FACIAL_L_tongueA = 0x35EE,
    FACIAL_R_tongueA = 0x2FEE,
    FACIAL_chinSkinTop = 0x7226,
    FACIAL_L_chinSkinTop = 0x3EB3,
    FACIAL_chinSkinMid = 0x899A,
    FACIAL_L_chinSkinMid = 0x4427,
    FACIAL_L_chinSide = 0x4A5E,
    FACIAL_R_chinSkinMid = 0xF5AF,
    FACIAL_R_chinSkinTop = 0xF03B,
    FACIAL_R_chinSide = 0xAA5E,
    FACIAL_R_underChin = 0x2BF4,
    FACIAL_L_lipLowerSDK = 0xB9E1,
    FACIAL_L_lipLowerAnalog = 0x244A,
    FACIAL_L_lipLowerThicknessV = 0xC749,
    FACIAL_L_lipLowerThicknessH = 0xC67B,
    FACIAL_lipLowerSDK = 0x7285,
    FACIAL_lipLowerAnalog = 0xD97B,
    FACIAL_lipLowerThicknessV = 0xC5BB,
    FACIAL_lipLowerThicknessH = 0xC5ED,
    FACIAL_R_lipLowerSDK = 0xA034,
    FACIAL_R_lipLowerAnalog = 0xC2D9,
    FACIAL_R_lipLowerThicknessV = 0xC6E9,
    FACIAL_R_lipLowerThicknessH = 0xC6DB,
    FACIAL_nose = 0x20F1,
    FACIAL_L_nostril = 0x7322,
    FACIAL_L_nostrilThickness = 0xC15F,
    FACIAL_noseLower = 0xE05A,
    FACIAL_L_noseLowerThickness = 0x79D5,
    FACIAL_R_noseLowerThickness = 0x7975,
    FACIAL_noseTip = 0x6A60,
    FACIAL_R_nostril = 0x7922,
    FACIAL_R_nostrilThickness = 0x36FF,
    FACIAL_noseUpper = 0xA04F,
    FACIAL_L_noseUpper = 0x1FB8,
    FACIAL_noseBridge = 0x9BA3,
    FACIAL_L_nasolabialFurrow = 0x5ACA,
    FACIAL_L_nasolabialBulge = 0xCD78,
    FACIAL_L_cheekLower = 0x6907,
    FACIAL_L_cheekLowerBulge1 = 0xE3FB,
    FACIAL_L_cheekLowerBulge2 = 0xE3FC,
    FACIAL_L_cheekInner = 0xE7AB,
    FACIAL_L_cheekOuter = 0x8161,
    FACIAL_L_eyesackLower = 0x771B,
    FACIAL_L_eyeball = 0x1744,
    FACIAL_L_eyelidLower = 0x998C,
    FACIAL_L_eyelidLowerOuterSDK = 0xFE4C,
    FACIAL_L_eyelidLowerOuterAnalog = 0xB9AA,
    FACIAL_L_eyelashLowerOuter = 0xD7F6,
    FACIAL_L_eyelidLowerInnerSDK = 0xF151,
    FACIAL_L_eyelidLowerInnerAnalog = 0x8242,
    FACIAL_L_eyelashLowerInner = 0x4CCF,
    FACIAL_L_eyelidUpper = 0x97C1,
    FACIAL_L_eyelidUpperOuterSDK = 0xAF15,
    FACIAL_L_eyelidUpperOuterAnalog = 0x67FA,
    FACIAL_L_eyelashUpperOuter = 0x27B7,
    FACIAL_L_eyelidUpperInnerSDK = 0xD341,
    FACIAL_L_eyelidUpperInnerAnalog = 0xF092,
    FACIAL_L_eyelashUpperInner = 0x9B1F,
    FACIAL_L_eyesackUpperOuterBulge = 0xA559,
    FACIAL_L_eyesackUpperInnerBulge = 0x2F2A,
    FACIAL_L_eyesackUpperOuterFurrow = 0xC597,
    FACIAL_L_eyesackUpperInnerFurrow = 0x52A7,
    FACIAL_forehead = 0x9218,
    FACIAL_L_foreheadInner = 0x843,
    FACIAL_L_foreheadInnerBulge = 0x767C,
    FACIAL_L_foreheadOuter = 0x8DCB,
    FACIAL_skull = 0x4221,
    FACIAL_foreheadUpper = 0xF7D6,
    FACIAL_L_foreheadUpperInner = 0xCF13,
    FACIAL_L_foreheadUpperOuter = 0x509B,
    FACIAL_R_foreheadUpperInner = 0xCEF3,
    FACIAL_R_foreheadUpperOuter = 0x507B,
    FACIAL_L_temple = 0xAF79,
    FACIAL_L_ear = 0x19DD,
    FACIAL_L_earLower = 0x6031,
    FACIAL_L_masseter = 0x2810,
    FACIAL_L_jawRecess = 0x9C7A,
    FACIAL_L_cheekOuterSkin = 0x14A5,
    FACIAL_R_cheekLower = 0xF367,
    FACIAL_R_cheekLowerBulge1 = 0x599B,
    FACIAL_R_cheekLowerBulge2 = 0x599C,
    FACIAL_R_masseter = 0x810,
    FACIAL_R_jawRecess = 0x93D4,
    FACIAL_R_ear = 0x1137,
    FACIAL_R_earLower = 0x8031,
    FACIAL_R_eyesackLower = 0x777B,
    FACIAL_R_nasolabialBulge = 0xD61E,
    FACIAL_R_cheekOuter = 0xD32,
    FACIAL_R_cheekInner = 0x737C,
    FACIAL_R_noseUpper = 0x1CD6,
    FACIAL_R_foreheadInner = 0xE43,
    FACIAL_R_foreheadInnerBulge = 0x769C,
    FACIAL_R_foreheadOuter = 0x8FCB,
    FACIAL_R_cheekOuterSkin = 0xB334,
    FACIAL_R_eyesackUpperInnerFurrow = 0x9FAE,
    FACIAL_R_eyesackUpperOuterFurrow = 0x140F,
    FACIAL_R_eyesackUpperInnerBulge = 0xA359,
    FACIAL_R_eyesackUpperOuterBulge = 0x1AF9,
    FACIAL_R_nasolabialFurrow = 0x2CAA,
    FACIAL_R_temple = 0xAF19,
    FACIAL_R_eyeball = 0x1944,
    FACIAL_R_eyelidUpper = 0x7E14,
    FACIAL_R_eyelidUpperOuterSDK = 0xB115,
    FACIAL_R_eyelidUpperOuterAnalog = 0xF25A,
    FACIAL_R_eyelashUpperOuter = 0xE0A,
    FACIAL_R_eyelidUpperInnerSDK = 0xD541,
    FACIAL_R_eyelidUpperInnerAnalog = 0x7C63,
    FACIAL_R_eyelashUpperInner = 0x8172,
    FACIAL_R_eyelidLower = 0x7FDF,
    FACIAL_R_eyelidLowerOuterSDK = 0x1BD,
    FACIAL_R_eyelidLowerOuterAnalog = 0x457B,
    FACIAL_R_eyelashLowerOuter = 0xBE49,
    FACIAL_R_eyelidLowerInnerSDK = 0xF351,
    FACIAL_R_eyelidLowerInnerAnalog = 0xE13,
    FACIAL_R_eyelashLowerInner = 0x3322,
    FACIAL_L_lipUpperSDK = 0x8F30,
    FACIAL_L_lipUpperAnalog = 0xB1CF,
    FACIAL_L_lipUpperThicknessH = 0x37CE,
    FACIAL_L_lipUpperThicknessV = 0x38BC,
    FACIAL_lipUpperSDK = 0x1774,
    FACIAL_lipUpperAnalog = 0xE064,
    FACIAL_lipUpperThicknessH = 0x7993,
    FACIAL_lipUpperThicknessV = 0x7981,
    FACIAL_L_lipCornerSDK = 0xB1C,
    FACIAL_L_lipCornerAnalog = 0xE568,
    FACIAL_L_lipCornerThicknessUpper = 0x7BC,
    FACIAL_L_lipCornerThicknessLower = 0xDD42,
    FACIAL_R_lipUpperSDK = 0x7583,
    FACIAL_R_lipUpperAnalog = 0x51CF,
    FACIAL_R_lipUpperThicknessH = 0x382E,
    FACIAL_R_lipUpperThicknessV = 0x385C,
    FACIAL_R_lipCornerSDK = 0xB3C,
    FACIAL_R_lipCornerAnalog = 0xEE0E,
    FACIAL_R_lipCornerThicknessUpper = 0x54C3,
    FACIAL_R_lipCornerThicknessLower = 0x2BBA,
    MH_MulletRoot = 0x3E73,
    MH_MulletScaler = 0xA1C2,
    MH_Hair_Scale = 0xC664,
    MH_Hair_Crown = 0x1675,
    SM_Torch = 0x8D6,
    FX_Light = 0x8959,
    FX_Light_Scale = 0x5038,
    FX_Light_Switch = 0xE18E,
    BagRoot = 0xAD09,
    BagPivotROOT = 0xB836,
    BagPivot = 0x4D11,
    BagBody = 0xAB6D,
    BagBone_R = 0x937,
    BagBone_L = 0x991,
    SM_LifeSaver_Front = 0x9420,
    SM_R_Pouches_ROOT = 0x2962,
    SM_R_Pouches = 0x4141,
    SM_L_Pouches_ROOT = 0x2A02,
    SM_L_Pouches = 0x4B41,
    SM_Suit_Back_Flapper = 0xDA2D,
    SPR_CopRadio = 0x8245,
    SM_LifeSaver_Back = 0x2127,
    MH_BlushSlider = 0xA0CE,
    SKEL_Tail_01 = 0x347,
    SKEL_Tail_02 = 0x348,
    MH_L_Concertina_B = 0xC988,
    MH_L_Concertina_A = 0xC987,
    MH_R_Concertina_B = 0xC8E8,
    MH_R_Concertina_A = 0xC8E7,
    MH_L_ShoulderBladeRoot = 0x8711,
    MH_L_ShoulderBlade = 0x4EAF,
    MH_R_ShoulderBladeRoot = 0x3A0A,
    MH_R_ShoulderBlade = 0x54AF,
    FB_R_Ear_000 = 0x6CDF,
    SPR_R_Ear = 0x63B6,
    FB_L_Ear_000 = 0x6439,
    SPR_L_Ear = 0x5B10,
    FB_TongueA_000 = 0x4206,
    FB_TongueB_000 = 0x4207,
    FB_TongueC_000 = 0x4208,
    SKEL_L_Toe1 = 0x1D6B,
    SKEL_R_Toe1 = 0xB23F,
    SKEL_Tail_03 = 0x349,
    SKEL_Tail_04 = 0x34A,
    SKEL_Tail_05 = 0x34B,
    SPR_Gonads_ROOT = 0xBFDE,
    SPR_Gonads = 0x1C00,
    FB_L_Brow_Out_001 = 0xE3DB,
    FB_L_Lid_Upper_001 = 0xB2B6,
    FB_L_Eye_001 = 0x62AC,
    FB_L_CheekBone_001 = 0x542E,
    FB_L_Lip_Corner_001 = 0x74AC,
    FB_R_Lid_Upper_001 = 0xAA10,
    FB_R_Eye_001 = 0x6B52,
    FB_R_CheekBone_001 = 0x4B88,
    FB_R_Brow_Out_001 = 0x54C,
    FB_R_Lip_Corner_001 = 0x2BA6,
    FB_Brow_Centre_001 = 0x9149,
    FB_UpperLipRoot_001 = 0x4ED2,
    FB_UpperLip_001 = 0xF18F,
    FB_L_Lip_Top_001 = 0x4F37,
    FB_R_Lip_Top_001 = 0x4537,
    FB_Jaw_001 = 0xB4A0,
    FB_LowerLipRoot_001 = 0x4324,
    FB_LowerLip_001 = 0x508F,
    FB_L_Lip_Bot_001 = 0xB93B,
    FB_R_Lip_Bot_001 = 0xC33B,
    FB_Tongue_001 = 0xB987
}; 
```

pub fn get_ped_bone_index_safe(ped: Ped, boneId: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_BONE_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_BONE_INDEX(ped, boneId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_BONE_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_bone_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_bone_index_raw(ped: i32, boneId: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_BONE_INDEX(ped, boneId)
}

/// ## Parameters
* **toggle**:

pub fn set_create_random_cops_not_on_scenarios_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CREATE_RANDOM_COPS_NOT_ON_SCENARIOS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CREATE_RANDOM_COPS_NOT_ON_SCENARIOS(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CREATE_RANDOM_COPS_NOT_ON_SCENARIOS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_create_random_cops_not_on_scenarios_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_create_random_cops_not_on_scenarios_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CREATE_RANDOM_COPS_NOT_ON_SCENARIOS(toggle)
}

/// ```
Min: 0.00  
Max: 10.00  
Can be used in combo with fast run cheat.  
When value is set to 10.00:  
Sprinting without fast run cheat: 66 m/s  
Sprinting with fast run cheat: 77 m/s  
Needs to be looped!  
Note: According to IDA for the Xbox360 xex, when they check bgt they seem to have the min to 0.0f, but the max set to 1.15f not 10.0f.  
```

pub fn set_ped_move_rate_override_safe(ped: Ped, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_MOVE_RATE_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_MOVE_RATE_OVERRIDE(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_MOVE_RATE_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_move_rate_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_move_rate_override_raw(ped: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_MOVE_RATE_OVERRIDE(ped, value)
}

/// ## Parameters
* **ped**:

pub fn get_ped_combat_movement_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_COMBAT_MOVEMENT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_COMBAT_MOVEMENT(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_COMBAT_MOVEMENT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_combat_movement_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_combat_movement_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_COMBAT_MOVEMENT(ped)
}

/// ## Parameters
* **p0**:

pub fn _0xea9960d07dadcf10_safe(p0: serde_json::Value) -> NativeResult<i64> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xEA9960D07DADCF10");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xEA9960D07DADCF10(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0xEA9960D07DADCF10
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xea9960d07dadcf10_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xea9960d07dadcf10_raw(p0: *mut std::os::raw::c_void) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xEA9960D07DADCF10(p0)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn set_ped_can_torso_react_ik_safe(ped: Ped, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_TORSO_REACT_IK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_TORSO_REACT_IK(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_TORSO_REACT_IK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_torso_react_ik_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_torso_react_ik_raw(ped: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_TORSO_REACT_IK(ped, p1)
}

/// ## Parameters
* **ped**:

pub fn get_ped_time_of_death_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_TIME_OF_DEATH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_TIME_OF_DEATH(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_TIME_OF_DEATH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_time_of_death_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_time_of_death_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_TIME_OF_DEATH(ped)
}

/// Input: Haircolor index, value between 0 and 63 (inclusive).
Output: RGB values for the haircolor specified in the input.

This is used with the hair color swatches scaleform.

Use [`_0x013E5CFC38CD5387`](#_0x013E5CFC38CD5387) to get the makeup colors.

pub fn _get_ped_hair_rgb_color_safe(hairColorIndex: i64, outR: *mut i64, outG: *mut i64, outB: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: _GET_PED_HAIR_RGB_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_PED_HAIR_RGB_COLOR(hairColorIndex, outR, outG, outB)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _GET_PED_HAIR_RGB_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_ped_hair_rgb_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_ped_hair_rgb_color_raw(hairColorIndex: i64, outR: *mut i64, outG: *mut i64, outB: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_PED_HAIR_RGB_COLOR(hairColorIndex, outR, outG, outB)
}

/// ## Parameters
* **ped**:

pub fn is_ped_tracked_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_TRACKED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_TRACKED(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_TRACKED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_tracked_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_tracked_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_TRACKED(ped)
}

/// ```
works with TASK::TASK_SET_BLOCKING_OF_NON_TEMPORARY_EVENTS to make a ped completely oblivious to all events going on around him
```

pub fn set_blocking_of_non_temporary_events_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_BLOCKING_OF_NON_TEMPORARY_EVENTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_BLOCKING_OF_NON_TEMPORARY_EVENTS(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_BLOCKING_OF_NON_TEMPORARY_EVENTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_blocking_of_non_temporary_events_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_blocking_of_non_temporary_events_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_BLOCKING_OF_NON_TEMPORARY_EVENTS(ped, toggle)
}

/// ## Parameters
* **p0**:

pub fn _0x9911f4a24485f653_safe(p0: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x9911F4A24485F653");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x9911F4A24485F653(p0)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x9911F4A24485F653
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x9911f4a24485f653_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x9911f4a24485f653_raw(p0: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x9911F4A24485F653(p0)
}

/// ```
state: https://alloc8or.re/gta5/doc/enums/eKnockOffVehicle.txt
```

pub fn set_ped_can_be_knocked_off_vehicle_safe(ped: Ped, state: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_BE_KNOCKED_OFF_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_BE_KNOCKED_OFF_VEHICLE(ped, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_BE_KNOCKED_OFF_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_be_knocked_off_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_be_knocked_off_vehicle_raw(ped: i32, state: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_BE_KNOCKED_OFF_VEHICLE(ped, state)
}

/// ## Parameters
* **ped**:

pub fn is_ped_running_ragdoll_task_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_RUNNING_RAGDOLL_TASK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_RUNNING_RAGDOLL_TASK(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_RUNNING_RAGDOLL_TASK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_running_ragdoll_task_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_running_ragdoll_task_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_RUNNING_RAGDOLL_TASK(ped)
}

/// ## Parameters
* **ped**:

pub fn get_vehicle_ped_is_trying_to_enter_safe(ped: Ped) -> NativeResult<Vehicle> {
    
    debug!("Calling native function: GET_VEHICLE_PED_IS_TRYING_TO_ENTER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_PED_IS_TRYING_TO_ENTER(ped)
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: GET_VEHICLE_PED_IS_TRYING_TO_ENTER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_ped_is_trying_to_enter_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_ped_is_trying_to_enter_raw(ped: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_PED_IS_TRYING_TO_ENTER(ped)
}

/// ## Parameters
* **sceneID**: 
* **rate**:

pub fn set_synchronized_scene_rate_safe(sceneID: i64, rate: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SYNCHRONIZED_SCENE_RATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SYNCHRONIZED_SCENE_RATE(sceneID, rate)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SYNCHRONIZED_SCENE_RATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_synchronized_scene_rate_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_synchronized_scene_rate_raw(sceneID: i64, rate: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SYNCHRONIZED_SCENE_RATE(sceneID, rate)
}

/// ## Parameters
* **ped**: 
* **targetPed**: 
* **p2**:

pub fn _clone_ped_to_target_ex_safe(ped: Ped, targetPed: Ped, p2: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p2_any_str = serde_json::to_string(&p2)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p2", e)))?;
    let p2_any_str_cstr = std::ffi::CString::new(p2_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p2", e)))?;
    
    debug!("Calling native function: _CLONE_PED_TO_TARGET_EX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_CLONE_PED_TO_TARGET_EX(ped, targetPed, crate::utils::any_to_c_void_ptr(p2))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _CLONE_PED_TO_TARGET_EX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_clone_ped_to_target_ex_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _clone_ped_to_target_ex_raw(ped: i32, targetPed: i32, p2: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_CLONE_PED_TO_TARGET_EX(ped, targetPed, p2)
}

/// ## Parameters
* **colorId**:

pub fn _is_ped_lipstick_color_valid_2_safe(colorId: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PED_LIPSTICK_COLOR_VALID_2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PED_LIPSTICK_COLOR_VALID_2(colorId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PED_LIPSTICK_COLOR_VALID_2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_ped_lipstick_color_valid_2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_ped_lipstick_color_valid_2_raw(colorId: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PED_LIPSTICK_COLOR_VALID_2(colorId)
}

/// ## Parameters
* **ped**:

pub fn get_peds_jacker_safe(ped: Ped) -> NativeResult<Ped> {
    
    debug!("Calling native function: GET_PEDS_JACKER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PEDS_JACKER(ped)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: GET_PEDS_JACKER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_peds_jacker_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_peds_jacker_raw(ped: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PEDS_JACKER(ped)
}

/// ## Parameters
* **x1**: 
* **y1**: 
* **z1**: 
* **x2**: 
* **y2**: 
* **z2**:

pub fn _does_scenario_blocking_area_exist_safe(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> NativeResult<bool> {
    
    debug!("Calling native function: _DOES_SCENARIO_BLOCKING_AREA_EXIST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_DOES_SCENARIO_BLOCKING_AREA_EXIST(x1, y1, z1, x2, y2, z2)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _DOES_SCENARIO_BLOCKING_AREA_EXIST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_does_scenario_blocking_area_exist_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _does_scenario_blocking_area_exist_raw(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_DOES_SCENARIO_BLOCKING_AREA_EXIST(x1, y1, z1, x2, y2, z2)
}

/// Fires a weapon at a coordinate using a ped.

pub fn set_ped_shoots_at_coord_safe(ped: Ped, x: f32, y: f32, z: f32, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_SHOOTS_AT_COORD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_SHOOTS_AT_COORD(ped, x, y, z, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_SHOOTS_AT_COORD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_shoots_at_coord_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_shoots_at_coord_raw(ped: i32, x: f32, y: f32, z: f32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_SHOOTS_AT_COORD(ped, x, y, z, toggle)
}

/// ## Parameters
* **ped**: 
* **animName**: 
* **animDict**:

pub fn set_facial_idle_anim_override_safe(ped: Ped, animName: String, animDict: String) -> NativeResult<()> {
    let animName_cstr = std::ffi::CString::new(animName.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animName", e)))?;
    let animDict_cstr = std::ffi::CString::new(animDict.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animDict", e)))?;
    
    debug!("Calling native function: SET_FACIAL_IDLE_ANIM_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_FACIAL_IDLE_ANIM_OVERRIDE(ped, crate::utils::rust_to_c_string(animName), crate::utils::rust_to_c_string(animDict))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_FACIAL_IDLE_ANIM_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_facial_idle_anim_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_facial_idle_anim_override_raw(ped: i32, animName: *const std::os::raw::c_char, animDict: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_FACIAL_IDLE_ANIM_OVERRIDE(ped, animName, animDict)
}

/// ```
Gets a random ped in the x/y/zRadius near the x/y/z coordinates passed.   
Ped Types:  
Any = -1  
Player = 1  
Male = 4   
Female = 5   
Cop = 6  
Human = 26  
SWAT = 27   
Animal = 28  
Army = 29  
```

pub fn get_random_ped_at_coord_safe(x: f32, y: f32, z: f32, xRadius: f32, yRadius: f32, zRadius: f32, pedType: i64) -> NativeResult<Ped> {
    
    debug!("Calling native function: GET_RANDOM_PED_AT_COORD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_RANDOM_PED_AT_COORD(x, y, z, xRadius, yRadius, zRadius, pedType)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: GET_RANDOM_PED_AT_COORD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_random_ped_at_coord_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_random_ped_at_coord_raw(x: f32, y: f32, z: f32, xRadius: f32, yRadius: f32, zRadius: f32, pedType: i64) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_RANDOM_PED_AT_COORD(x, y, z, xRadius, yRadius, zRadius, pedType)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **p2**: 
* **p3**:

pub fn set_ped_should_play_flee_scenario_exit_safe(ped: Ped, p1: serde_json::Value, p2: serde_json::Value, p3: serde_json::Value) -> NativeResult<serde_json::Value> {
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
    
    debug!("Calling native function: SET_PED_SHOULD_PLAY_FLEE_SCENARIO_EXIT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_SHOULD_PLAY_FLEE_SCENARIO_EXIT(ped, crate::utils::any_to_c_void_ptr(p1), crate::utils::any_to_c_void_ptr(p2), crate::utils::any_to_c_void_ptr(p3))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: SET_PED_SHOULD_PLAY_FLEE_SCENARIO_EXIT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_should_play_flee_scenario_exit_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_should_play_flee_scenario_exit_raw(ped: i32, p1: *mut std::os::raw::c_void, p2: *mut std::os::raw::c_void, p3: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_SHOULD_PLAY_FLEE_SCENARIO_EXIT(ped, p1, p2, p3)
}

/// ## Parameters
* **ped**:

pub fn is_ped_on_foot_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_ON_FOOT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_ON_FOOT(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_ON_FOOT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_on_foot_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_on_foot_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_ON_FOOT(ped)
}

/// Applies lethal damage (FLT_MAX) to the `SKEL_Head` bone of the specified ped using the weapon passed, leading to the
ped's untimely demise.

The naming of the native is a legacy leftover (formerly EXPLODE_CHAR_HEAD in GTA3) as in the early 3D GTA games, lethal
damage to a ped head would 'explode' it.

Do note that this native function does not work in multiplayer/network environment.

pub fn explode_ped_head_safe(ped: Ped, weaponHash: u32) -> NativeResult<()> {
    
    debug!("Calling native function: EXPLODE_PED_HEAD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::EXPLODE_PED_HEAD(ped, weaponHash)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: EXPLODE_PED_HEAD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `explode_ped_head_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn explode_ped_head_raw(ped: i32, weaponHash: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::EXPLODE_PED_HEAD(ped, weaponHash)
}

/// ```
It clears the wetness of the selected Ped/Player. Clothes have to be wet to notice the difference.  
```

pub fn clear_ped_wetness_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_WETNESS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_WETNESS(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_WETNESS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_wetness_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_wetness_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_WETNESS(ped)
}

/// ```
Gets the closest ped in a radius.  
Ped Types:  
Any ped = -1  
Player = 1  
Male = 4   
Female = 5   
Cop = 6  
Human = 26  
SWAT = 27   
Animal = 28  
Army = 29  
------------------  
P4 P5 P7 P8  
1  0  x  x  = return nearest walking Ped  
1  x  0  x  = return nearest walking Ped  
x  1  1  x  = return Ped you are using  
0  0  x  x  = no effect  
0  x  0  x  = no effect  
x = can be 1 or 0. Does not have any obvious changes.  
This function does not return ped who is:  
1. Standing still  
2. Driving  
3. Fleeing  
4. Attacking  
This function only work if the ped is:  
1. walking normally.  
2. waiting to cross a road.  
Note: PED::GET_PED_NEARBY_PEDS works for more peds.  
```

pub fn get_closest_ped_safe(x: f32, y: f32, z: f32, radius: f32, p4: bool, p5: bool, outPed: *mut i32, p7: bool, p8: bool, pedType: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_CLOSEST_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_CLOSEST_PED(x, y, z, radius, p4, p5, outPed, p7, p8, pedType)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_CLOSEST_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_closest_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_closest_ped_raw(x: f32, y: f32, z: f32, radius: f32, p4: bool, p5: bool, outPed: *mut i32, p7: bool, p8: bool, pedType: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_CLOSEST_PED(x, y, z, radius, p4, p5, outPed, p7, p8, pedType)
}

/// ## Parameters
* **ped**:

pub fn remove_ped_preferred_cover_set_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_PED_PREFERRED_COVER_SET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_PED_PREFERRED_COVER_SET(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_PED_PREFERRED_COVER_SET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_ped_preferred_cover_set_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_ped_preferred_cover_set_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_PED_PREFERRED_COVER_SET(ped)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn _0xfd325494792302d7_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xFD325494792302D7");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xFD325494792302D7(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xFD325494792302D7
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xfd325494792302d7_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xfd325494792302d7_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xFD325494792302D7(ped, toggle)
}

/// ```
Returns whether the entity is in stealth mode  
```

pub fn get_ped_stealth_movement_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_PED_STEALTH_MOVEMENT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_STEALTH_MOVEMENT(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_STEALTH_MOVEMENT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_stealth_movement_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_stealth_movement_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_STEALTH_MOVEMENT(ped)
}

/// ## Parameters
* **p0**:

pub fn _0xa52d5247a4227e14_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xA52D5247A4227E14");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xA52D5247A4227E14(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xA52D5247A4227E14
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xa52d5247a4227e14_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xa52d5247a4227e14_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xA52D5247A4227E14(p0)
}

/// ```
-1: no landing  
0: landing on both feet  
1: stumbling  
2: rolling  
3: ragdoll  
```

pub fn get_ped_parachute_landing_type_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_PARACHUTE_LANDING_TYPE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_PARACHUTE_LANDING_TYPE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_PARACHUTE_LANDING_TYPE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_parachute_landing_type_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_parachute_landing_type_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_PARACHUTE_LANDING_TYPE(ped)
}

/// ```
angle is ped's view cone  
```

pub fn is_ped_facing_ped_safe(ped: Ped, otherPed: Ped, angle: f32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_FACING_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_FACING_PED(ped, otherPed, angle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_FACING_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_facing_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_facing_ped_raw(ped: i32, otherPed: i32, angle: f32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_FACING_PED(ped, otherPed, angle)
}

/// ## Parameters
* **ped**:

pub fn is_ped_planting_bomb_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_PLANTING_BOMB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_PLANTING_BOMB(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_PLANTING_BOMB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_planting_bomb_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_planting_bomb_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_PLANTING_BOMB(ped)
}

/// INSTANTLY_FILL_PED_POPULATION native function

pub fn instantly_fill_ped_population_safe() -> NativeResult<()> {
    
    debug!("Calling native function: INSTANTLY_FILL_PED_POPULATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::INSTANTLY_FILL_PED_POPULATION()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: INSTANTLY_FILL_PED_POPULATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `instantly_fill_ped_population_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn instantly_fill_ped_population_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::INSTANTLY_FILL_PED_POPULATION()
}

/// ```
NativeDB Added Parameter 3: BOOL p2
```

pub fn set_ped_helmet_prop_index_safe(ped: Ped, propIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_HELMET_PROP_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_HELMET_PROP_INDEX(ped, propIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_HELMET_PROP_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_helmet_prop_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_helmet_prop_index_raw(ped: i32, propIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_HELMET_PROP_INDEX(ped, propIndex)
}

/// ```
Enables diving motion when underwater.  
```

pub fn set_enable_scuba_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_ENABLE_SCUBA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_ENABLE_SCUBA(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_ENABLE_SCUBA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_enable_scuba_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_enable_scuba_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_ENABLE_SCUBA(ped, toggle)
}

/// ## Parameters
* **colorID**:

pub fn _is_ped_hair_color_valid_safe(colorID: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PED_HAIR_COLOR_VALID");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PED_HAIR_COLOR_VALID(colorID)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PED_HAIR_COLOR_VALID
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_ped_hair_color_valid_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_ped_hair_color_valid_raw(colorID: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PED_HAIR_COLOR_VALID(colorID)
}

/// RESET_AI_WEAPON_DAMAGE_MODIFIER native function

pub fn reset_ai_weapon_damage_modifier_safe() -> NativeResult<()> {
    
    debug!("Calling native function: RESET_AI_WEAPON_DAMAGE_MODIFIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_AI_WEAPON_DAMAGE_MODIFIER()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_AI_WEAPON_DAMAGE_MODIFIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_ai_weapon_damage_modifier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_ai_weapon_damage_modifier_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_AI_WEAPON_DAMAGE_MODIFIER()
}

/// ```
Likely a char, if that overlay is not set, e.i. "None" option, returns 255;
This might be the once removed native GET_PED_HEAD_OVERLAY.
```

pub fn _get_ped_head_overlay_value_safe(ped: Ped, overlayID: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_PED_HEAD_OVERLAY_VALUE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_PED_HEAD_OVERLAY_VALUE(ped, overlayID)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_PED_HEAD_OVERLAY_VALUE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_ped_head_overlay_value_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_ped_head_overlay_value_raw(ped: i32, overlayID: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_PED_HEAD_OVERLAY_VALUE(ped, overlayID)
}

/// ```
range 0.0f - 1.0f  
```

pub fn set_driver_aggressiveness_safe(driver: Ped, aggressiveness: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_DRIVER_AGGRESSIVENESS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DRIVER_AGGRESSIVENESS(driver, aggressiveness)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DRIVER_AGGRESSIVENESS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_driver_aggressiveness_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_driver_aggressiveness_raw(driver: i32, aggressiveness: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DRIVER_AGGRESSIVENESS(driver, aggressiveness)
}

/// ## Parameters
* **ped**: 
* **value**:

pub fn set_ped_seeing_range_safe(ped: Ped, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_SEEING_RANGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_SEEING_RANGE(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_SEEING_RANGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_seeing_range_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_seeing_range_raw(ped: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_SEEING_RANGE(ped, value)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_allow_vehicles_override_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_ALLOW_VEHICLES_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_ALLOW_VEHICLES_OVERRIDE(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_ALLOW_VEHICLES_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_allow_vehicles_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_allow_vehicles_override_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_ALLOW_VEHICLES_OVERRIDE(ped, toggle)
}

/// ```
Despite this function's name, it simply returns whether the specified handle is a Ped.  
```

pub fn was_ped_skeleton_updated_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: WAS_PED_SKELETON_UPDATED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::WAS_PED_SKELETON_UPDATED(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: WAS_PED_SKELETON_UPDATED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `was_ped_skeleton_updated_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn was_ped_skeleton_updated_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::WAS_PED_SKELETON_UPDATED(ped)
}

/// ```
Gets the relationship between two peds. This should be called twice (once for each ped).  
Relationship types:  
0 = Companion  
1 = Respect  
2 = Like  
3 = Neutral  
4 = Dislike  
5 = Hate  
255 = Pedestrians  
(Credits: Inco)  
Example:  
PED::GET_RELATIONSHIP_BETWEEN_PEDS(2, l_1017, 0xA49E591C);  
PED::GET_RELATIONSHIP_BETWEEN_PEDS(2, 0xA49E591C, l_1017);  
```

pub fn get_relationship_between_peds_safe(ped1: Ped, ped2: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_RELATIONSHIP_BETWEEN_PEDS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_RELATIONSHIP_BETWEEN_PEDS(ped1, ped2)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_RELATIONSHIP_BETWEEN_PEDS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_relationship_between_peds_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_relationship_between_peds_raw(ped1: i32, ped2: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_RELATIONSHIP_BETWEEN_PEDS(ped1, ped2)
}

/// ```
stance:  
0 = idle  
1 = walk  
2 = running  
p5 = usually set to true  
```

[Animations list](https://alexguirre.github.io/animations-list/)

pub fn set_ped_alternate_movement_anim_safe(ped: Ped, stance: i64, animDictionary: String, animationName: String, p4: f32, p5: bool) -> NativeResult<()> {
    let animDictionary_cstr = std::ffi::CString::new(animDictionary.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animDictionary", e)))?;
    let animationName_cstr = std::ffi::CString::new(animationName.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animationName", e)))?;
    
    debug!("Calling native function: SET_PED_ALTERNATE_MOVEMENT_ANIM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_ALTERNATE_MOVEMENT_ANIM(ped, stance, crate::utils::rust_to_c_string(animDictionary), crate::utils::rust_to_c_string(animationName), p4, p5)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_ALTERNATE_MOVEMENT_ANIM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_alternate_movement_anim_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_alternate_movement_anim_raw(ped: i32, stance: i64, animDictionary: *const std::os::raw::c_char, animationName: *const std::os::raw::c_char, p4: f32, p5: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_ALTERNATE_MOVEMENT_ANIM(ped, stance, animDictionary, animationName, p4, p5)
}

/// ```
This native refers to the field of vision the ped has above them, starting at 0 degrees. 90f would let the ped see enemies directly above of them.  
```

pub fn set_ped_visual_field_max_elevation_angle_safe(ped: Ped, angle: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_VISUAL_FIELD_MAX_ELEVATION_ANGLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_VISUAL_FIELD_MAX_ELEVATION_ANGLE(ped, angle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_VISUAL_FIELD_MAX_ELEVATION_ANGLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_visual_field_max_elevation_angle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_visual_field_max_elevation_angle_raw(ped: i32, angle: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_VISUAL_FIELD_MAX_ELEVATION_ANGLE(ped, angle)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn spawnpoints_get_search_result_flags_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: SPAWNPOINTS_GET_SEARCH_RESULT_FLAGS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPAWNPOINTS_GET_SEARCH_RESULT_FLAGS(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPAWNPOINTS_GET_SEARCH_RESULT_FLAGS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `spawnpoints_get_search_result_flags_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn spawnpoints_get_search_result_flags_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPAWNPOINTS_GET_SEARCH_RESULT_FLAGS(p0, p1)
}

/// ## Parameters
* **ped**:

pub fn clear_ped_env_dirt_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_ENV_DIRT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_ENV_DIRT(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_ENV_DIRT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_env_dirt_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_env_dirt_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_ENV_DIRT(ped)
}

/// ```
p2 is usually -1 in the scripts. action is either 0 or "DEFAULT_ACTION".  
```

pub fn set_ped_using_action_mode_safe(ped: Ped, p1: bool, p2: i64, action: String) -> NativeResult<()> {
    let action_cstr = std::ffi::CString::new(action.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "action", e)))?;
    
    debug!("Calling native function: SET_PED_USING_ACTION_MODE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_USING_ACTION_MODE(ped, p1, p2, crate::utils::rust_to_c_string(action))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_USING_ACTION_MODE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_using_action_mode_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_using_action_mode_raw(ped: i32, p1: bool, p2: i64, action: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_USING_ACTION_MODE(ped, p1, p2, action)
}

/// Verifies whether a ped is firing within a specific area.

pub fn is_any_ped_shooting_in_area_safe(minX: f32, minY: f32, minZ: f32, maxX: f32, maxY: f32, maxZ: f32, bHighlightArea: bool, bDo3DCheck: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_ANY_PED_SHOOTING_IN_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_ANY_PED_SHOOTING_IN_AREA(minX, minY, minZ, maxX, maxY, maxZ, bHighlightArea, bDo3DCheck)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_ANY_PED_SHOOTING_IN_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_any_ped_shooting_in_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_any_ped_shooting_in_area_raw(minX: f32, minY: f32, minZ: f32, maxX: f32, maxY: f32, maxZ: f32, bHighlightArea: bool, bDo3DCheck: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_ANY_PED_SHOOTING_IN_AREA(minX, minY, minZ, maxX, maxY, maxZ, bHighlightArea, bDo3DCheck)
}

/// ```
In agency_heist3b.c4, its like this 90% of the time:  
PED::_110F526AB784111F(ped, 0.099);  
PED::SET_PED_ENVEFF_SCALE(ped, 1.0);  
PED::_D69411AA0CEBF9E9(ped, 87, 81, 68);  
PED::SET_ENABLE_PED_ENVEFF_SCALE(ped, 1);  
and its like this 10% of the time:  
PED::_110F526AB784111F(ped, 0.2);  
PED::SET_PED_ENVEFF_SCALE(ped, 0.65);  
PED::_D69411AA0CEBF9E9(ped, 74, 69, 60);  
PED::SET_ENABLE_PED_ENVEFF_SCALE(ped, 1);  
```

pub fn _0x110f526ab784111f_safe(ped: Ped, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0x110F526AB784111F");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x110F526AB784111F(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x110F526AB784111F
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x110f526ab784111f_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x110f526ab784111f_raw(ped: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x110F526AB784111F(ped, p1)
}

/// ```
OverlayID ranges from 0 to 12, index from 0 to _GET_NUM_OVERLAY_VALUES(overlayID)-1, and opacity from 0.0 to 1.0.   
overlayID       Part                  Index, to disable  
0               Blemishes             0 - 23, 255  
1               Facial Hair           0 - 28, 255  
2               Eyebrows              0 - 33, 255  
3               Ageing                0 - 14, 255  
4               Makeup                0 - 74, 255  
5               Blush                 0 - 6, 255  
6               Complexion            0 - 11, 255  
7               Sun Damage            0 - 10, 255  
8               Lipstick              0 - 9, 255  
9               Moles/Freckles        0 - 17, 255  
10              Chest Hair            0 - 16, 255  
11              Body Blemishes        0 - 11, 255  
12              Add Body Blemishes    0 - 1, 255  
```

pub fn set_ped_head_overlay_safe(ped: Ped, overlayID: i64, index: i64, opacity: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_HEAD_OVERLAY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_HEAD_OVERLAY(ped, overlayID, index, opacity)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_HEAD_OVERLAY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_head_overlay_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_head_overlay_raw(ped: i32, overlayID: i64, index: i64, opacity: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_HEAD_OVERLAY(ped, overlayID, index, opacity)
}

/// ```
Scripts use 0.2, 0.5 and 1.0. Value must be >= 0.0 && <= 1.0
```

pub fn set_driver_racing_modifier_safe(driver: Ped, modifier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_DRIVER_RACING_MODIFIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DRIVER_RACING_MODIFIER(driver, modifier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DRIVER_RACING_MODIFIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_driver_racing_modifier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_driver_racing_modifier_raw(driver: i32, modifier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DRIVER_RACING_MODIFIER(driver, modifier)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0xe906ec930f5fe7c8_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0xE906EC930F5FE7C8");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xE906EC930F5FE7C8(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xE906EC930F5FE7C8
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xe906ec930f5fe7c8_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xe906ec930f5fe7c8_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xE906EC930F5FE7C8(p0, p1)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn _0x2dfc81c9b9608549_safe(ped: Ped, p1: *mut i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _0x2DFC81C9B9608549");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x2DFC81C9B9608549(ped, p1)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0x2DFC81C9B9608549
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x2dfc81c9b9608549_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x2dfc81c9b9608549_raw(ped: i32, p1: *mut i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x2DFC81C9B9608549(ped, p1)
}

/// ## Return value

pub fn has_pedheadshot_img_upload_failed_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_PEDHEADSHOT_IMG_UPLOAD_FAILED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_PEDHEADSHOT_IMG_UPLOAD_FAILED()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_PEDHEADSHOT_IMG_UPLOAD_FAILED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_pedheadshot_img_upload_failed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_pedheadshot_img_upload_failed_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_PEDHEADSHOT_IMG_UPLOAD_FAILED()
}

/// ## Parameters
* **ped**:

pub fn has_ped_preload_prop_data_finished_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_PED_PRELOAD_PROP_DATA_FINISHED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_PED_PRELOAD_PROP_DATA_FINISHED(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_PED_PRELOAD_PROP_DATA_FINISHED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_ped_preload_prop_data_finished_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_ped_preload_prop_data_finished_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_PED_PRELOAD_PROP_DATA_FINISHED(ped)
}

/// ## Parameters
* **ped**:

pub fn is_ped_on_any_bike_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_ON_ANY_BIKE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_ON_ANY_BIKE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_ON_ANY_BIKE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_on_any_bike_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_on_any_bike_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_ON_ANY_BIKE(ped)
}

/// ```
Same function call as PED::GET_MOUNT, aka just returns 0  
```

pub fn is_ped_on_mount_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_ON_MOUNT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_ON_MOUNT(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_ON_MOUNT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_on_mount_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_on_mount_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_ON_MOUNT(ped)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **p2**:

pub fn create_parachute_bag_object_safe(ped: Ped, p1: bool, p2: bool) -> NativeResult<Object> {
    
    debug!("Calling native function: CREATE_PARACHUTE_BAG_OBJECT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CREATE_PARACHUTE_BAG_OBJECT(ped, p1, p2)
    };
    
    
    Ok(Ok(Object(result)))
}

/// Raw native function: CREATE_PARACHUTE_BAG_OBJECT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `create_parachute_bag_object_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn create_parachute_bag_object_raw(ped: i32, p1: bool, p2: bool) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CREATE_PARACHUTE_BAG_OBJECT(ped, p1, p2)
}

/// ## Parameters
* **asset**:

pub fn remove_action_mode_asset_safe(asset: String) -> NativeResult<()> {
    let asset_cstr = std::ffi::CString::new(asset.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "asset", e)))?;
    
    debug!("Calling native function: REMOVE_ACTION_MODE_ASSET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_ACTION_MODE_ASSET(crate::utils::rust_to_c_string(asset))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_ACTION_MODE_ASSET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_action_mode_asset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_action_mode_asset_raw(asset: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_ACTION_MODE_ASSET(asset)
}

/// ```
Type equals 0 for male non-dlc, 1 for female non-dlc, 2 for male dlc, and 3 for female dlc.
Used when calling SET_PED_HEAD_BLEND_DATA.
```

pub fn get_ped_head_blend_first_index_safe(type: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_HEAD_BLEND_FIRST_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_HEAD_BLEND_FIRST_INDEX(type)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_HEAD_BLEND_FIRST_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_head_blend_first_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_head_blend_first_index_raw(type: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_HEAD_BLEND_FIRST_INDEX(type)
}

/// Clears the blood on a ped.

pub fn clear_ped_blood_damage_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_BLOOD_DAMAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_BLOOD_DAMAGE(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_BLOOD_DAMAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_blood_damage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_blood_damage_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_BLOOD_DAMAGE(ped)
}

/// ## Parameters
* **ped**:

pub fn clear_ped_decorations_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_DECORATIONS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_DECORATIONS(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_DECORATIONS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_decorations_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_decorations_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_DECORATIONS(ped)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn set_ped_cloth_prone_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: SET_PED_CLOTH_PRONE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CLOTH_PRONE(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CLOTH_PRONE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_cloth_prone_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_cloth_prone_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CLOTH_PRONE(p0, p1)
}

/// ## Parameters
* **ped**: 
* **helmetFlag**:

pub fn set_ped_helmet_flag_safe(ped: Ped, helmetFlag: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_HELMET_FLAG");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_HELMET_FLAG(ped, helmetFlag)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_HELMET_FLAG
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_helmet_flag_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_helmet_flag_raw(ped: i32, helmetFlag: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_HELMET_FLAG(ped, helmetFlag)
}

/// ```
scar
blushing
cs_flush_anger
cs_flush_anger_face
bruise
bruise_large
herpes
ArmorBullet
basic_dirt_cloth
basic_dirt_skin
cs_trev1_dirt
```

pub fn apply_ped_damage_decal_safe(ped: Ped, damageZone: i64, xOffset: f32, yOffset: f32, heading: f32, scale: f32, alpha: f32, variation: i64, fadeIn: bool, decalName: String) -> NativeResult<()> {
    let decalName_cstr = std::ffi::CString::new(decalName.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "decalName", e)))?;
    
    debug!("Calling native function: APPLY_PED_DAMAGE_DECAL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::APPLY_PED_DAMAGE_DECAL(ped, damageZone, xOffset, yOffset, heading, scale, alpha, variation, fadeIn, crate::utils::rust_to_c_string(decalName))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: APPLY_PED_DAMAGE_DECAL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `apply_ped_damage_decal_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn apply_ped_damage_decal_raw(ped: i32, damageZone: i64, xOffset: f32, yOffset: f32, heading: f32, scale: f32, alpha: f32, variation: i64, fadeIn: bool, decalName: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::APPLY_PED_DAMAGE_DECAL(ped, damageZone, xOffset, yOffset, heading, scale, alpha, variation, fadeIn, decalName)
}

/// Sets the palette index of a ped's phone.

| Value | Color      |
| :---: | :-----:    |
|  `0`  | Light Blue |
|  `1`  | Green      |
|  `2`  | Red        |
|  `3`  | Orange     |
|  `4`  | Grey       |
|  `5`  | Purple     |
|  `6`  | Pink       |

```
NativeDB Introduced: v323
```

pub fn set_ped_phone_palette_idx_safe(ped: Ped, index: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_PHONE_PALETTE_IDX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_PHONE_PALETTE_IDX(ped, index)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_PHONE_PALETTE_IDX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_phone_palette_idx_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_phone_palette_idx_raw(ped: i32, index: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_PHONE_PALETTE_IDX(ped, index)
}

/// ```c
// Potential names and hash collisions included as comments
enum ePedConfigFlags {
	CPED_CONFIG_FLAG_CreatedByFactory = 0,
	CPED_CONFIG_FLAG_CanBeShotInVehicle = 1,
	CPED_CONFIG_FLAG_NoCriticalHits = 2,
	CPED_CONFIG_FLAG_DrownsInWater = 3,
	CPED_CONFIG_FLAG_DrownsInSinkingVehicle = 4,
	CPED_CONFIG_FLAG_DiesInstantlyWhenSwimming = 5,
	CPED_CONFIG_FLAG_HasBulletProofVest = 6,
	CPED_CONFIG_FLAG_UpperBodyDamageAnimsOnly = 7,
	CPED_CONFIG_FLAG_NeverFallOffSkis = 8,
	CPED_CONFIG_FLAG_NeverEverTargetThisPed = 9,
	CPED_CONFIG_FLAG_ThisPedIsATargetPriority = 10,
	CPED_CONFIG_FLAG_TargettableWithNoLos = 11,
	CPED_CONFIG_FLAG_DoesntListenToPlayerGroupCommands = 12,
	CPED_CONFIG_FLAG_NeverLeavesGroup = 13,
	CPED_CONFIG_FLAG_DoesntDropWeaponsWhenDead = 14,
	CPED_CONFIG_FLAG_SetDelayedWeaponAsCurrent = 15,
	CPED_CONFIG_FLAG_KeepTasksAfterCleanUp = 16,
	CPED_CONFIG_FLAG_BlockNonTemporaryEvents = 17,
	CPED_CONFIG_FLAG_HasAScriptBrain = 18,
	CPED_CONFIG_FLAG_WaitingForScriptBrainToLoad = 19,
	CPED_CONFIG_FLAG_AllowMedicsToReviveMe = 20,
	CPED_CONFIG_FLAG_MoneyHasBeenGivenByScript = 21,
	CPED_CONFIG_FLAG_NotAllowedToCrouch = 22,
	CPED_CONFIG_FLAG_DeathPickupsPersist = 23,
	CPED_CONFIG_FLAG_IgnoreSeenMelee = 24,
	CPED_CONFIG_FLAG_ForceDieIfInjured = 25,
	CPED_CONFIG_FLAG_DontDragMeOutCar = 26,
	CPED_CONFIG_FLAG_StayInCarOnJack = 27,
	CPED_CONFIG_FLAG_ForceDieInCar = 28,
	CPED_CONFIG_FLAG_GetOutUndriveableVehicle = 29,
	CPED_CONFIG_FLAG_WillRemainOnBoatAfterMissionEnds = 30,
	CPED_CONFIG_FLAG_DontStoreAsPersistent = 31,
	CPED_CONFIG_FLAG_WillFlyThroughWindscreen = 32,
	CPED_CONFIG_FLAG_DieWhenRagdoll = 33,
	CPED_CONFIG_FLAG_HasHelmet = 34,
	CPED_CONFIG_FLAG_UseHelmet = 35,
	CPED_CONFIG_FLAG_DontTakeOffHelmet = 36,
	CPED_CONFIG_FLAG_HideInCutscene = 37,
	CPED_CONFIG_FLAG_PedIsEnemyToPlayer = 38,
	CPED_CONFIG_FLAG_DisableEvasiveDives = 39,
	CPED_CONFIG_FLAG_PedGeneratesDeadBodyEvents = 40,
	CPED_CONFIG_FLAG_DontAttackPlayerWithoutWantedLevel = 41,
	CPED_CONFIG_FLAG_DontInfluenceWantedLevel = 42,
	CPED_CONFIG_FLAG_DisablePlayerLockon = 43,
	CPED_CONFIG_FLAG_DisableLockonToRandomPeds = 44,
	CPED_CONFIG_FLAG_AllowLockonToFriendlyPlayers = 45,
	_0xDB115BFA = 46,
	CPED_CONFIG_FLAG_PedBeingDeleted = 47,
	CPED_CONFIG_FLAG_BlockWeaponSwitching = 48,
	CPED_CONFIG_FLAG_BlockGroupPedAimedAtResponse = 49,
	CPED_CONFIG_FLAG_WillFollowLeaderAnyMeans = 50,
	CPED_CONFIG_FLAG_BlippedByScript = 51,
	CPED_CONFIG_FLAG_DrawRadarVisualField = 52,
	CPED_CONFIG_FLAG_StopWeaponFiringOnImpact = 53,
	CPED_CONFIG_FLAG_DissableAutoFallOffTests = 54,
	CPED_CONFIG_FLAG_SteerAroundDeadBodies = 55,
	CPED_CONFIG_FLAG_ConstrainToNavMesh = 56,
	CPED_CONFIG_FLAG_SyncingAnimatedProps = 57,
	CPED_CONFIG_FLAG_IsFiring = 58,
	CPED_CONFIG_FLAG_WasFiring = 59,
	CPED_CONFIG_FLAG_IsStanding = 60,
	CPED_CONFIG_FLAG_WasStanding = 61,
	CPED_CONFIG_FLAG_InVehicle = 62,
	CPED_CONFIG_FLAG_OnMount = 63,
	CPED_CONFIG_FLAG_AttachedToVehicle = 64,
	CPED_CONFIG_FLAG_IsSwimming = 65,
	CPED_CONFIG_FLAG_WasSwimming = 66,
	CPED_CONFIG_FLAG_IsSkiing = 67,
	CPED_CONFIG_FLAG_IsSitting = 68,
	CPED_CONFIG_FLAG_KilledByStealth = 69,
	CPED_CONFIG_FLAG_KilledByTakedown = 70,
	CPED_CONFIG_FLAG_Knockedout = 71,
	CPED_CONFIG_FLAG_ClearRadarBlipOnDeath = 72,
	CPED_CONFIG_FLAG_JustGotOffTrain = 73,
	CPED_CONFIG_FLAG_JustGotOnTrain = 74,
	CPED_CONFIG_FLAG_UsingCoverPoint = 75,
	CPED_CONFIG_FLAG_IsInTheAir = 76,
	CPED_CONFIG_FLAG_KnockedUpIntoAir = 77,
	CPED_CONFIG_FLAG_IsAimingGun = 78,
	CPED_CONFIG_FLAG_HasJustLeftCar = 79,
	CPED_CONFIG_FLAG_TargetWhenInjuredAllowed = 80,
	CPED_CONFIG_FLAG_CurrLeftFootCollNM = 81,
	CPED_CONFIG_FLAG_PrevLeftFootCollNM = 82,
	CPED_CONFIG_FLAG_CurrRightFootCollNM = 83,
	CPED_CONFIG_FLAG_PrevRightFootCollNM = 84,
	CPED_CONFIG_FLAG_HasBeenBumpedInCar = 85,
	CPED_CONFIG_FLAG_InWaterTaskQuitToClimbLadder = 86,
	CPED_CONFIG_FLAG_NMTwoHandedWeaponBothHandsConstrained = 87,
	CPED_CONFIG_FLAG_CreatedBloodPoolTimer = 88,
	CPED_CONFIG_FLAG_DontActivateRagdollFromAnyPedImpact = 89,
	CPED_CONFIG_FLAG_GroupPedFailedToEnterCover = 90,
	CPED_CONFIG_FLAG_AlreadyChattedOnPhone = 91,
	CPED_CONFIG_FLAG_AlreadyReactedToPedOnRoof = 92,
	CPED_CONFIG_FLAG_ForcePedLoadCover = 93,
	CPED_CONFIG_FLAG_BlockCoweringInCover = 94,
	CPED_CONFIG_FLAG_BlockPeekingInCover = 95,
	CPED_CONFIG_FLAG_JustLeftCarNotCheckedForDoors = 96,
	CPED_CONFIG_FLAG_VaultFromCover = 97,
	CPED_CONFIG_FLAG_AutoConversationLookAts = 98,
	CPED_CONFIG_FLAG_UsingCrouchedPedCapsule = 99,
	CPED_CONFIG_FLAG_HasDeadPedBeenReported = 100,
	CPED_CONFIG_FLAG_ForcedAim = 101,
	CPED_CONFIG_FLAG_SteersAroundPeds = 102,
	CPED_CONFIG_FLAG_SteersAroundObjects = 103,
	CPED_CONFIG_FLAG_OpenDoorArmIK = 104,
	CPED_CONFIG_FLAG_ForceReload = 105,
	CPED_CONFIG_FLAG_DontActivateRagdollFromVehicleImpact = 106,
	CPED_CONFIG_FLAG_DontActivateRagdollFromBulletImpact = 107,
	CPED_CONFIG_FLAG_DontActivateRagdollFromExplosions = 108,
	CPED_CONFIG_FLAG_DontActivateRagdollFromFire = 109,
	CPED_CONFIG_FLAG_DontActivateRagdollFromElectrocution = 110,
	CPED_CONFIG_FLAG_IsBeingDraggedToSafety = 111,
	CPED_CONFIG_FLAG_HasBeenDraggedToSafety = 112,
	CPED_CONFIG_FLAG_KeepWeaponHolsteredUnlessFired = 113,
	CPED_CONFIG_FLAG_ForceScriptControlledKnockout = 114,
	CPED_CONFIG_FLAG_FallOutOfVehicleWhenKilled = 115,
	CPED_CONFIG_FLAG_GetOutBurningVehicle = 116,
	CPED_CONFIG_FLAG_BumpedByPlayer = 117,
	CPED_CONFIG_FLAG_RunFromFiresAndExplosions = 118,
	CPED_CONFIG_FLAG_TreatAsPlayerDuringTargeting = 119,
	CPED_CONFIG_FLAG_IsHandCuffed = 120,
	CPED_CONFIG_FLAG_IsAnkleCuffed = 121,
	CPED_CONFIG_FLAG_DisableMelee = 122,
	CPED_CONFIG_FLAG_DisableUnarmedDrivebys = 123,
	CPED_CONFIG_FLAG_JustGetsPulledOutWhenElectrocuted = 124,
	CPED_CONFIG_FLAG_UNUSED_REPLACE_ME = 125,
	CPED_CONFIG_FLAG_WillNotHotwireLawEnforcementVehicle = 126,
	CPED_CONFIG_FLAG_WillCommandeerRatherThanJack = 127,
	CPED_CONFIG_FLAG_CanBeAgitated = 128,
	CPED_CONFIG_FLAG_ForcePedToFaceLeftInCover = 129,
	CPED_CONFIG_FLAG_ForcePedToFaceRightInCover = 130,
	CPED_CONFIG_FLAG_BlockPedFromTurningInCover = 131,
	CPED_CONFIG_FLAG_KeepRelationshipGroupAfterCleanUp = 132,
	CPED_CONFIG_FLAG_ForcePedToBeDragged = 133,
	CPED_CONFIG_FLAG_PreventPedFromReactingToBeingJacked = 134,
	CPED_CONFIG_FLAG_IsScuba = 135,
	CPED_CONFIG_FLAG_WillArrestRatherThanJack = 136,
	CPED_CONFIG_FLAG_RemoveDeadExtraFarAway = 137,
	CPED_CONFIG_FLAG_RidingTrain = 138,
	CPED_CONFIG_FLAG_ArrestResult = 139,
	CPED_CONFIG_FLAG_CanAttackFriendly = 140,
	CPED_CONFIG_FLAG_WillJackAnyPlayer = 141,
	CPED_CONFIG_FLAG_BumpedByPlayerVehicle = 142,
	CPED_CONFIG_FLAG_DodgedPlayerVehicle = 143,
	CPED_CONFIG_FLAG_WillJackWantedPlayersRatherThanStealCar = 144,
	CPED_CONFIG_FLAG_NoCopWantedAggro = 145,
	CPED_CONFIG_FLAG_DisableLadderClimbing = 146,
	CPED_CONFIG_FLAG_StairsDetected = 147,
	CPED_CONFIG_FLAG_SlopeDetected = 148,
	CPED_CONFIG_FLAG_HelmetHasBeenShot = 149,
	CPED_CONFIG_FLAG_CowerInsteadOfFlee = 150,
	CPED_CONFIG_FLAG_CanActivateRagdollWhenVehicleUpsideDown = 151,
	CPED_CONFIG_FLAG_AlwaysRespondToCriesForHelp = 152,
	CPED_CONFIG_FLAG_DisableBloodPoolCreation = 153,
	CPED_CONFIG_FLAG_ShouldFixIfNoCollision = 154,
	CPED_CONFIG_FLAG_CanPerformArrest = 155,
	CPED_CONFIG_FLAG_CanPerformUncuff = 156,
	CPED_CONFIG_FLAG_CanBeArrested = 157,
	CPED_CONFIG_FLAG_MoverConstrictedByOpposingCollisions = 158,
	CPED_CONFIG_FLAG_PlayerPreferFrontSeatMP = 159,
	CPED_CONFIG_FLAG_DontActivateRagdollFromImpactObject = 160,
	CPED_CONFIG_FLAG_DontActivateRagdollFromMelee = 161,
	CPED_CONFIG_FLAG_DontActivateRagdollFromWaterJet = 162,
	CPED_CONFIG_FLAG_DontActivateRagdollFromDrowning = 163,
	CPED_CONFIG_FLAG_DontActivateRagdollFromFalling = 164,
	CPED_CONFIG_FLAG_DontActivateRagdollFromRubberBullet = 165,
	CPED_CONFIG_FLAG_IsInjured = 166,
	CPED_CONFIG_FLAG_DontEnterVehiclesInPlayersGroup = 167,
	CPED_CONFIG_FLAG_SwimmingTasksRunning = 168,
	CPED_CONFIG_FLAG_PreventAllMeleeTaunts = 169,
	CPED_CONFIG_FLAG_ForceDirectEntry = 170,
	CPED_CONFIG_FLAG_AlwaysSeeApproachingVehicles = 171,
	CPED_CONFIG_FLAG_CanDiveAwayFromApproachingVehicles = 172,
	CPED_CONFIG_FLAG_AllowPlayerToInterruptVehicleEntryExit = 173,
	CPED_CONFIG_FLAG_OnlyAttackLawIfPlayerIsWanted = 174,
	CPED_CONFIG_FLAG_PlayerInContactWithKinematicPed = 175,
	CPED_CONFIG_FLAG_PlayerInContactWithSomethingOtherThanKinematicPed = 176,
	CPED_CONFIG_FLAG_PedsJackingMeDontGetIn = 177,
	CPED_CONFIG_FLAG_AdditionalRappellingPed = 178,
	CPED_CONFIG_FLAG_PedIgnoresAnimInterruptEvents = 179,
	CPED_CONFIG_FLAG_IsInCustody = 180,
	CPED_CONFIG_FLAG_ForceStandardBumpReactionThresholds = 181,
	CPED_CONFIG_FLAG_LawWillOnlyAttackIfPlayerIsWanted = 182,
	CPED_CONFIG_FLAG_IsAgitated = 183,
	CPED_CONFIG_FLAG_PreventAutoShuffleToDriversSeat = 184,
	CPED_CONFIG_FLAG_UseKinematicModeWhenStationary = 185,
	CPED_CONFIG_FLAG_EnableWeaponBlocking = 186,
	CPED_CONFIG_FLAG_HasHurtStarted = 187,
	CPED_CONFIG_FLAG_DisableHurt = 188,
	CPED_CONFIG_FLAG_PlayerIsWeird = 189,
	CPED_CONFIG_FLAG_PedHadPhoneConversation = 190,
	CPED_CONFIG_FLAG_BeganCrossingRoad = 191,
	CPED_CONFIG_FLAG_WarpIntoLeadersVehicle = 192,
	CPED_CONFIG_FLAG_DoNothingWhenOnFootByDefault = 193,
	CPED_CONFIG_FLAG_UsingScenario = 194,
	CPED_CONFIG_FLAG_VisibleOnScreen = 195,
	CPED_CONFIG_FLAG_DontCollideWithKinematic = 196,
	CPED_CONFIG_FLAG_ActivateOnSwitchFromLowPhysicsLod = 197,
	CPED_CONFIG_FLAG_DontActivateRagdollOnPedCollisionWhenDead = 198,
	CPED_CONFIG_FLAG_DontActivateRagdollOnVehicleCollisionWhenDead = 199,
	CPED_CONFIG_FLAG_HasBeenInArmedCombat = 200,
	CPED_CONFIG_FLAG_UseDiminishingAmmoRate = 201,
	CPED_CONFIG_FLAG_Avoidance_Ignore_All = 202,
	CPED_CONFIG_FLAG_Avoidance_Ignored_by_All = 203,
	CPED_CONFIG_FLAG_Avoidance_Ignore_Group1 = 204,
	CPED_CONFIG_FLAG_Avoidance_Member_of_Group1 = 205,
	CPED_CONFIG_FLAG_ForcedToUseSpecificGroupSeatIndex = 206,
	CPED_CONFIG_FLAG_LowPhysicsLodMayPlaceOnNavMesh = 207,
	CPED_CONFIG_FLAG_DisableExplosionReactions = 208,
	CPED_CONFIG_FLAG_DodgedPlayer = 209,
	CPED_CONFIG_FLAG_WaitingForPlayerControlInterrupt = 210,
	CPED_CONFIG_FLAG_ForcedToStayInCover = 211,
	CPED_CONFIG_FLAG_GeneratesSoundEvents = 212,
	CPED_CONFIG_FLAG_ListensToSoundEvents = 213,
	CPED_CONFIG_FLAG_AllowToBeTargetedInAVehicle = 214,
	CPED_CONFIG_FLAG_WaitForDirectEntryPointToBeFreeWhenExiting = 215,
	CPED_CONFIG_FLAG_OnlyRequireOnePressToExitVehicle = 216,
	CPED_CONFIG_FLAG_ForceExitToSkyDive = 217,
	CPED_CONFIG_FLAG_SteersAroundVehicles = 218,
	CPED_CONFIG_FLAG_AllowPedInVehiclesOverrideTaskFlags = 219,
	CPED_CONFIG_FLAG_DontEnterLeadersVehicle = 220,
	CPED_CONFIG_FLAG_DisableExitToSkyDive = 221,
	CPED_CONFIG_FLAG_ScriptHasDisabledCollision = 222,
	CPED_CONFIG_FLAG_UseAmbientModelScaling = 223,
	CPED_CONFIG_FLAG_DontWatchFirstOnNextHurryAway = 224,
	CPED_CONFIG_FLAG_DisablePotentialToBeWalkedIntoResponse = 225,
	CPED_CONFIG_FLAG_DisablePedAvoidance = 226,
	CPED_CONFIG_FLAG_ForceRagdollUponDeath = 227,
	CPED_CONFIG_FLAG_CanLosePropsOnDamage = 228,
	CPED_CONFIG_FLAG_DisablePanicInVehicle = 229,
	CPED_CONFIG_FLAG_AllowedToDetachTrailer = 230,
	CPED_CONFIG_FLAG_HasShotBeenReactedToFromFront = 231,
	CPED_CONFIG_FLAG_HasShotBeenReactedToFromBack = 232,
	CPED_CONFIG_FLAG_HasShotBeenReactedToFromLeft = 233,
	CPED_CONFIG_FLAG_HasShotBeenReactedToFromRight = 234,
	CPED_CONFIG_FLAG_AllowBlockDeadPedRagdollActivation = 235,
	CPED_CONFIG_FLAG_IsHoldingProp = 236,
	CPED_CONFIG_FLAG_BlocksPathingWhenDead = 237,
	CPED_CONFIG_FLAG_ForcePlayNormalScenarioExitOnNextScriptCommand = 238,
	CPED_CONFIG_FLAG_ForcePlayImmediateScenarioExitOnNextScriptCommand = 239,
	CPED_CONFIG_FLAG_ForceSkinCharacterCloth = 240,
	CPED_CONFIG_FLAG_LeaveEngineOnWhenExitingVehicles = 241,
	CPED_CONFIG_FLAG_PhoneDisableTextingAnimations = 242,
	CPED_CONFIG_FLAG_PhoneDisableTalkingAnimations = 243,
	CPED_CONFIG_FLAG_PhoneDisableCameraAnimations = 244,
	CPED_CONFIG_FLAG_DisableBlindFiringInShotReactions = 245,
	CPED_CONFIG_FLAG_AllowNearbyCoverUsage = 246,
	CPED_CONFIG_FLAG_InStrafeTransition = 247,
	CPED_CONFIG_FLAG_CanPlayInCarIdles = 248,
	CPED_CONFIG_FLAG_CanAttackNonWantedPlayerAsLaw = 249,
	CPED_CONFIG_FLAG_WillTakeDamageWhenVehicleCrashes = 250,
	CPED_CONFIG_FLAG_AICanDrivePlayerAsRearPassenger = 251,
	CPED_CONFIG_FLAG_PlayerCanJackFriendlyPlayers = 252,
	CPED_CONFIG_FLAG_OnStairs = 253,
	CPED_CONFIG_FLAG_SimulatingAiming = 254,
	CPED_CONFIG_FLAG_AIDriverAllowFriendlyPassengerSeatEntry = 255,
	CPED_CONFIG_FLAG_ParentCarIsBeingRemoved = 256,
	CPED_CONFIG_FLAG_AllowMissionPedToUseInjuredMovement = 257,
	CPED_CONFIG_FLAG_CanLoseHelmetOnDamage = 258,
	CPED_CONFIG_FLAG_NeverDoScenarioExitProbeChecks = 259,
	CPED_CONFIG_FLAG_SuppressLowLODRagdollSwitchWhenCorpseSettles = 260,
	CPED_CONFIG_FLAG_PreventUsingLowerPrioritySeats = 261,
	CPED_CONFIG_FLAG_JustLeftVehicleNeedsReset = 262,
	CPED_CONFIG_FLAG_TeleportIfCantReachPlayer = 263,
	CPED_CONFIG_FLAG_PedsInVehiclePositionNeedsReset = 264,
	CPED_CONFIG_FLAG_PedsFullyInSeat = 265,
	CPED_CONFIG_FLAG_AllowPlayerLockOnIfFriendly = 266,
	CPED_CONFIG_FLAG_UseCameraHeadingForDesiredDirectionLockOnTest = 267,
	CPED_CONFIG_FLAG_TeleportToLeaderVehicle = 268,
	CPED_CONFIG_FLAG_Avoidance_Ignore_WeirdPedBuffer = 269,
	CPED_CONFIG_FLAG_OnStairSlope = 270,
	CPED_CONFIG_FLAG_HasPlayedNMGetup = 271,
	CPED_CONFIG_FLAG_DontBlipCop = 272,
	CPED_CONFIG_FLAG_SpawnedAtExtendedRangeScenario = 273,
	CPED_CONFIG_FLAG_WalkAlongsideLeaderWhenClose = 274,
	CPED_CONFIG_FLAG_KillWhenTrapped = 275,
	CPED_CONFIG_FLAG_EdgeDetected = 276,
	CPED_CONFIG_FLAG_AlwaysWakeUpPhysicsOfIntersectedPeds = 277,
	CPED_CONFIG_FLAG_EquippedAmbientLoadOutWeapon = 278,
	CPED_CONFIG_FLAG_AvoidTearGas = 279,
	CPED_CONFIG_FLAG_StoppedSpeechUponFreezing = 280,
	CPED_CONFIG_FLAG_DisableGoToWritheWhenInjured = 281,
	CPED_CONFIG_FLAG_OnlyUseForcedSeatWhenEnteringHeliInGroup = 282,
	CPED_CONFIG_FLAG_ThrownFromVehicleDueToExhaustion = 283,
	CPED_CONFIG_FLAG_UpdateEnclosedSearchRegion = 284,
	CPED_CONFIG_FLAG_DisableWeirdPedEvents = 285,
	CPED_CONFIG_FLAG_ShouldChargeNow = 286,
	CPED_CONFIG_FLAG_RagdollingOnBoat = 287,
	CPED_CONFIG_FLAG_HasBrandishedWeapon = 288,
	CPED_CONFIG_FLAG_AllowMinorReactionsAsMissionPed = 289,
	CPED_CONFIG_FLAG_BlockDeadBodyShockingEventsWhenDead = 290,
	CPED_CONFIG_FLAG_PedHasBeenSeen = 291,
	CPED_CONFIG_FLAG_PedIsInReusePool = 292,
	CPED_CONFIG_FLAG_PedWasReused = 293,
	CPED_CONFIG_FLAG_DisableShockingEvents = 294,
	CPED_CONFIG_FLAG_MovedUsingLowLodPhysicsSinceLastActive = 295,
	CPED_CONFIG_FLAG_NeverReactToPedOnRoof = 296,
	CPED_CONFIG_FLAG_ForcePlayFleeScenarioExitOnNextScriptCommand = 297,
	CPED_CONFIG_FLAG_JustBumpedIntoVehicle = 298,
	CPED_CONFIG_FLAG_DisableShockingDrivingOnPavementEvents = 299,
	CPED_CONFIG_FLAG_ShouldThrowSmokeNow = 300,
	CPED_CONFIG_FLAG_DisablePedConstraints = 301,
	CPED_CONFIG_FLAG_ForceInitialPeekInCover = 302,
	CPED_CONFIG_FLAG_CreatedByDispatch = 303,
	CPED_CONFIG_FLAG_PointGunLeftHandSupporting = 304,
	CPED_CONFIG_FLAG_DisableJumpingFromVehiclesAfterLeader = 305,
	CPED_CONFIG_FLAG_DontActivateRagdollFromPlayerPedImpact = 306,
	CPED_CONFIG_FLAG_DontActivateRagdollFromAiRagdollImpact = 307,
	CPED_CONFIG_FLAG_DontActivateRagdollFromPlayerRagdollImpact = 308,
	CPED_CONFIG_FLAG_DisableQuadrupedSpring = 309,
	CPED_CONFIG_FLAG_IsInCluster = 310,
	CPED_CONFIG_FLAG_ShoutToGroupOnPlayerMelee = 311,
	CPED_CONFIG_FLAG_IgnoredByAutoOpenDoors = 312,
	CPED_CONFIG_FLAG_PreferInjuredGetup = 313,
	CPED_CONFIG_FLAG_ForceIgnoreMeleeActiveCombatant = 314,
	CPED_CONFIG_FLAG_CheckLoSForSoundEvents = 315,
	CPED_CONFIG_FLAG_JackedAbandonedCar = 316,
	CPED_CONFIG_FLAG_CanSayFollowedByPlayerAudio = 317,
	CPED_CONFIG_FLAG_ActivateRagdollFromMinorPlayerContact = 318,
	CPED_CONFIG_FLAG_HasPortablePickupAttached = 319,
	CPED_CONFIG_FLAG_ForcePoseCharacterCloth = 320,
	CPED_CONFIG_FLAG_HasClothCollisionBounds = 321,
	CPED_CONFIG_FLAG_HasHighHeels = 322,
	CPED_CONFIG_FLAG_TreatAsAmbientPedForDriverLockOn = 323,
	CPED_CONFIG_FLAG_DontBehaveLikeLaw = 324,
	CPED_CONFIG_FLAG_SpawnedAtScenario = 325,
	CPED_CONFIG_FLAG_DisablePoliceInvestigatingBody = 326,
	CPED_CONFIG_FLAG_DisableWritheShootFromGround = 327,
	CPED_CONFIG_FLAG_LowerPriorityOfWarpSeats = 328,
	CPED_CONFIG_FLAG_DisableTalkTo = 329,
	CPED_CONFIG_FLAG_DontBlip = 330,
	CPED_CONFIG_FLAG_IsSwitchingWeapon = 331,
	CPED_CONFIG_FLAG_IgnoreLegIkRestrictions = 332,
	CPED_CONFIG_FLAG_ScriptForceNoTimesliceIntelligenceUpdate = 333,
	CPED_CONFIG_FLAG_JackedOutOfMyVehicle = 334,
	CPED_CONFIG_FLAG_WentIntoCombatAfterBeingJacked = 335,
	CPED_CONFIG_FLAG_DontActivateRagdollForVehicleGrab = 336,
	CPED_CONFIG_FLAG_ForcePackageCharacterCloth = 337,
	CPED_CONFIG_FLAG_DontRemoveWithValidOrder = 338,
	CPED_CONFIG_FLAG_AllowTaskDoNothingTimeslicing = 339,
	CPED_CONFIG_FLAG_ForcedToStayInCoverDueToPlayerSwitch = 340,
	CPED_CONFIG_FLAG_ForceProneCharacterCloth = 341,
	CPED_CONFIG_FLAG_NotAllowedToJackAnyPlayers = 342,
	CPED_CONFIG_FLAG_InToStrafeTransition = 343,
	CPED_CONFIG_FLAG_KilledByStandardMelee = 344,
	CPED_CONFIG_FLAG_AlwaysLeaveTrainUponArrival = 345,
	CPED_CONFIG_FLAG_ForcePlayDirectedNormalScenarioExitOnNextScriptCommand = 346,
	CPED_CONFIG_FLAG_OnlyWritheFromWeaponDamage = 347,
	CPED_CONFIG_FLAG_UseSloMoBloodVfx = 348,
	CPED_CONFIG_FLAG_EquipJetpack = 349,
	CPED_CONFIG_FLAG_PreventDraggedOutOfCarThreatResponse = 350,
	CPED_CONFIG_FLAG_ScriptHasCompletelyDisabledCollision = 351,
	CPED_CONFIG_FLAG_NeverDoScenarioNavChecks = 352,
	CPED_CONFIG_FLAG_ForceSynchronousScenarioExitChecking = 353,
	CPED_CONFIG_FLAG_ThrowingGrenadeWhileAiming = 354,
	CPED_CONFIG_FLAG_HeadbobToRadioEnabled = 355,
	CPED_CONFIG_FLAG_ForceDeepSurfaceCheck = 356,
	CPED_CONFIG_FLAG_DisableDeepSurfaceAnims = 357,
	CPED_CONFIG_FLAG_DontBlipNotSynced = 358,
	CPED_CONFIG_FLAG_IsDuckingInVehicle = 359,
	CPED_CONFIG_FLAG_PreventAutoShuffleToTurretSeat = 360,
	CPED_CONFIG_FLAG_DisableEventInteriorStatusCheck = 361,
	CPED_CONFIG_FLAG_HasReserveParachute = 362,
	CPED_CONFIG_FLAG_UseReserveParachute = 363,
	CPED_CONFIG_FLAG_TreatDislikeAsHateWhenInCombat = 364,
	CPED_CONFIG_FLAG_OnlyUpdateTargetWantedIfSeen = 365,
	CPED_CONFIG_FLAG_AllowAutoShuffleToDriversSeat = 366,
	CPED_CONFIG_FLAG_DontActivateRagdollFromSmokeGrenade = 367,
	CPED_CONFIG_FLAG_LinkMBRToOwnerOnChain = 368,
	CPED_CONFIG_FLAG_AmbientFriendBumpedByPlayer = 369,
	CPED_CONFIG_FLAG_AmbientFriendBumpedByPlayerVehicle = 370,
	CPED_CONFIG_FLAG_InFPSUnholsterTransition = 371,
	CPED_CONFIG_FLAG_PreventReactingToSilencedCloneBullets = 372,
	CPED_CONFIG_FLAG_DisableInjuredCryForHelpEvents = 373,
	CPED_CONFIG_FLAG_NeverLeaveTrain = 374,
	CPED_CONFIG_FLAG_DontDropJetpackOnDeath = 375,
	CPED_CONFIG_FLAG_UseFPSUnholsterTransitionDuringCombatRoll = 376,
	CPED_CONFIG_FLAG_ExitingFPSCombatRoll = 377,
	CPED_CONFIG_FLAG_ScriptHasControlOfPlayer = 378,
	CPED_CONFIG_FLAG_PlayFPSIdleFidgetsForProjectile = 379,
	CPED_CONFIG_FLAG_DisableAutoEquipHelmetsInBikes = 380,
	CPED_CONFIG_FLAG_DisableAutoEquipHelmetsInAircraft = 381,
	CPED_CONFIG_FLAG_WasPlayingFPSGetup = 382,
	CPED_CONFIG_FLAG_WasPlayingFPSMeleeActionResult = 383,
	CPED_CONFIG_FLAG_PreferNoPriorityRemoval = 384,
	CPED_CONFIG_FLAG_FPSFidgetsAbortedOnFire = 385,
	CPED_CONFIG_FLAG_ForceFPSIKWithUpperBodyAnim = 386,
	CPED_CONFIG_FLAG_SwitchingCharactersInFirstPerson = 387,
	CPED_CONFIG_FLAG_IsClimbingLadder = 388,
	CPED_CONFIG_FLAG_HasBareFeet = 389,
	CPED_CONFIG_FLAG_UNUSED_REPLACE_ME_2 = 390,
	CPED_CONFIG_FLAG_GoOnWithoutVehicleIfItIsUnableToGetBackToRoad = 391,
	CPED_CONFIG_FLAG_BlockDroppingHealthSnacksOnDeath = 392,
	CPED_CONFIG_FLAG_ResetLastVehicleOnVehicleExit = 393,
	CPED_CONFIG_FLAG_ForceThreatResponseToNonFriendToFriendMeleeActions = 394,
	CPED_CONFIG_FLAG_DontRespondToRandomPedsDamage = 395,
	CPED_CONFIG_FLAG_AllowContinuousThreatResponseWantedLevelUpdates = 396,
	CPED_CONFIG_FLAG_KeepTargetLossResponseOnCleanup = 397,
	CPED_CONFIG_FLAG_PlayersDontDragMeOutOfCar = 398,
	CPED_CONFIG_FLAG_BroadcastRepondedToThreatWhenGoingToPointShooting = 399,
	CPED_CONFIG_FLAG_IgnorePedTypeForIsFriendlyWith = 400,
	CPED_CONFIG_FLAG_TreatNonFriendlyAsHateWhenInCombat = 401,
	CPED_CONFIG_FLAG_DontLeaveVehicleIfLeaderNotInVehicle = 402,
	CPED_CONFIG_FLAG_ChangeFromPermanentToAmbientPopTypeOnMigration = 403,
	CPED_CONFIG_FLAG_AllowMeleeReactionIfMeleeProofIsOn = 404,
	CPED_CONFIG_FLAG_UsingLowriderLeans = 405,
	CPED_CONFIG_FLAG_UsingAlternateLowriderLeans = 406,
	CPED_CONFIG_FLAG_UseNormalExplosionDamageWhenBlownUpInVehicle = 407,
	CPED_CONFIG_FLAG_DisableHomingMissileLockForVehiclePedInside = 408,
	CPED_CONFIG_FLAG_DisableTakeOffScubaGear = 409,
	CPED_CONFIG_FLAG_IgnoreMeleeFistWeaponDamageMult = 410,
	CPED_CONFIG_FLAG_LawPedsCanFleeFromNonWantedPlayer = 411,
	CPED_CONFIG_FLAG_ForceBlipSecurityPedsIfPlayerIsWanted = 412,
	CPED_CONFIG_FLAG_IsHolsteringWeapon = 413,
	CPED_CONFIG_FLAG_UseGoToPointForScenarioNavigation = 414,
	CPED_CONFIG_FLAG_DontClearLocalPassengersWantedLevel = 415,
	CPED_CONFIG_FLAG_BlockAutoSwapOnWeaponPickups = 416,
	CPED_CONFIG_FLAG_ThisPedIsATargetPriorityForAI = 417,
	CPED_CONFIG_FLAG_IsSwitchingHelmetVisor = 418,
	CPED_CONFIG_FLAG_ForceHelmetVisorSwitch = 419,
	CPED_CONFIG_FLAG_IsPerformingVehicleMelee = 420,
	CPED_CONFIG_FLAG_UseOverrideFootstepPtFx = 421,
	CPED_CONFIG_FLAG_DisableVehicleCombat = 422,
	CPED_CONFIG_FLAG_TreatAsFriendlyForTargetingAndDamage = 423,
	CPED_CONFIG_FLAG_AllowBikeAlternateAnimations = 424,
	CPED_CONFIG_FLAG_TreatAsFriendlyForTargetingAndDamageNonSynced = 425,
	CPED_CONFIG_FLAG_UseLockpickVehicleEntryAnimations = 426,
	CPED_CONFIG_FLAG_IgnoreInteriorCheckForSprinting = 427,
	CPED_CONFIG_FLAG_SwatHeliSpawnWithinLastSpottedLocation = 428,
	CPED_CONFIG_FLAG_DisableStartEngine = 429,
	CPED_CONFIG_FLAG_IgnoreBeingOnFire = 430,
	CPED_CONFIG_FLAG_DisableTurretOrRearSeatPreference = 431,
	CPED_CONFIG_FLAG_DisableWantedHelicopterSpawning = 432,
	CPED_CONFIG_FLAG_UseTargetPerceptionForCreatingAimedAtEvents = 433,
	CPED_CONFIG_FLAG_DisableHomingMissileLockon = 434,
	CPED_CONFIG_FLAG_ForceIgnoreMaxMeleeActiveSupportCombatants = 435,
	CPED_CONFIG_FLAG_StayInDefensiveAreaWhenInVehicle = 436,
	CPED_CONFIG_FLAG_DontShoutTargetPosition = 437,
	CPED_CONFIG_FLAG_DisableHelmetArmor = 438,
	CPED_CONFIG_FLAG_CreatedByConcealedPlayer = 439,
	CPED_CONFIG_FLAG_PermanentlyDisablePotentialToBeWalkedIntoResponse = 440,
	CPED_CONFIG_FLAG_PreventVehExitDueToInvalidWeapon = 441,
	CPED_CONFIG_FLAG_IgnoreNetSessionFriendlyFireCheckForAllowDamage = 442,
	CPED_CONFIG_FLAG_DontLeaveCombatIfTargetPlayerIsAttackedByPolice = 443,
	CPED_CONFIG_FLAG_CheckLockedBeforeWarp = 444,
	CPED_CONFIG_FLAG_DontShuffleInVehicleToMakeRoom = 445,
	CPED_CONFIG_FLAG_GiveWeaponOnGetup = 446,
	CPED_CONFIG_FLAG_DontHitVehicleWithProjectiles = 447,
	CPED_CONFIG_FLAG_DisableForcedEntryForOpenVehiclesFromTryLockedDoor = 448,
	CPED_CONFIG_FLAG_FiresDummyRockets = 449,
	CPED_CONFIG_FLAG_PedIsArresting = 450,
	CPED_CONFIG_FLAG_IsDecoyPed = 451,
	CPED_CONFIG_FLAG_HasEstablishedDecoy = 452,
	CPED_CONFIG_FLAG_BlockDispatchedHelicoptersFromLanding = 453,
	CPED_CONFIG_FLAG_DontCryForHelpOnStun = 454,
	CPED_CONFIG_FLAG_HitByTranqWeapon = 455,
	CPED_CONFIG_FLAG_CanBeIncapacitated = 456,
	CPED_CONFIG_FLAG_ForcedAimFromArrest = 457,
	CPED_CONFIG_FLAG_DontChangeTargetFromMelee = 458,
	_0x4376ABF2 = 459,
	CPED_CONFIG_FLAG_RagdollFloatsIndefinitely = 460,
	CPED_CONFIG_FLAG_BlockElectricWeaponDamage = 461,
	_0x262A3B8E = 462,
	_0x1AA79A25 = 463,
}
```

pub fn set_ped_config_flag_safe(ped: Ped, flagId: i64, value: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CONFIG_FLAG");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CONFIG_FLAG(ped, flagId, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CONFIG_FLAG
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_config_flag_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_config_flag_raw(ped: i32, flagId: i64, value: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CONFIG_FLAG(ped, flagId, value)
}

/// ```
Sends the message that was created by a call to CREATE_NM_MESSAGE to the specified Ped.  
If a message hasn't been created already, this function does nothing.  
If the Ped is not ragdolled with Euphoria enabled, this function does nothing.  
The following call can be used to ragdoll the Ped with Euphoria enabled: SET_PED_TO_RAGDOLL(ped, 4000, 5000, 1, 1, 1, 0);  
Call order:  
SET_PED_TO_RAGDOLL  
CREATE_NM_MESSAGE  
GIVE_PED_NM_MESSAGE  
Multiple messages can be chained. Eg. to make the ped stagger and swing his arms around, the following calls can be made:  
SET_PED_TO_RAGDOLL(ped, 4000, 5000, 1, 1, 1, 0);  
CREATE_NM_MESSAGE(true, 0); // stopAllBehaviours - Stop all other behaviours, in case the Ped is already doing some Euphoria stuff.  
GIVE_PED_NM_MESSAGE(ped); // Dispatch message to Ped.  
CREATE_NM_MESSAGE(true, 1151); // staggerFall - Attempt to walk while falling.  
GIVE_PED_NM_MESSAGE(ped); // Dispatch message to Ped.  
CREATE_NM_MESSAGE(true, 372); // armsWindmill - Swing arms around.  
GIVE_PED_NM_MESSAGE(ped); // Dispatch message to Ped.  
```

pub fn give_ped_nm_message_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: GIVE_PED_NM_MESSAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GIVE_PED_NM_MESSAGE(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GIVE_PED_NM_MESSAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `give_ped_nm_message_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn give_ped_nm_message_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GIVE_PED_NM_MESSAGE(ped)
}

/// ```
Something related to the environmental effects natives.
In the "agency_heist3b" script, p1 - p3 are always under 100 - usually they are {87, 81, 68}. If SET_PED_ENVEFF_SCALE is set to 0.65 (instead of the usual 1.0), they use {74, 69, 60}
```

pub fn set_ped_enveff_color_modulator_safe(ped: Ped, r: i64, g: i64, b: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_ENVEFF_COLOR_MODULATOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_ENVEFF_COLOR_MODULATOR(ped, r, g, b)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_ENVEFF_COLOR_MODULATOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_enveff_color_modulator_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_enveff_color_modulator_raw(ped: i32, r: i64, g: i64, b: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_ENVEFF_COLOR_MODULATOR(ped, r, g, b)
}

/// Removes the scubagear (for mp male: component id: 8, drawableId: 123, textureId: any) from peds. Does not play the 'remove scuba gear' animation, but instantly removes it.

pub fn clear_ped_scuba_gear_variation_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_SCUBA_GEAR_VARIATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_SCUBA_GEAR_VARIATION(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_SCUBA_GEAR_VARIATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_scuba_gear_variation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_scuba_gear_variation_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_SCUBA_GEAR_VARIATION(ped)
}

/// ## Parameters
* **asset**:

pub fn remove_stealth_mode_asset_safe(asset: String) -> NativeResult<()> {
    let asset_cstr = std::ffi::CString::new(asset.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "asset", e)))?;
    
    debug!("Calling native function: REMOVE_STEALTH_MODE_ASSET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_STEALTH_MODE_ASSET(crate::utils::rust_to_c_string(asset))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_STEALTH_MODE_ASSET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_stealth_mode_asset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_stealth_mode_asset_raw(asset: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_STEALTH_MODE_ASSET(asset)
}

/// ## Parameters
* **ped**:

pub fn is_ped_falling_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_FALLING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_FALLING(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_FALLING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_falling_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_falling_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_FALLING(ped)
}

/// ## Parameters
* **ped**:

pub fn release_ped_preload_prop_data_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: RELEASE_PED_PRELOAD_PROP_DATA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RELEASE_PED_PRELOAD_PROP_DATA(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RELEASE_PED_PRELOAD_PROP_DATA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `release_ped_preload_prop_data_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn release_ped_preload_prop_data_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RELEASE_PED_PRELOAD_PROP_DATA(ped)
}

/// ```
CLEAR_PED_*
```

pub fn _clear_ped_cover_clipset_override_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: _CLEAR_PED_COVER_CLIPSET_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_CLEAR_PED_COVER_CLIPSET_OVERRIDE(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _CLEAR_PED_COVER_CLIPSET_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_clear_ped_cover_clipset_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _clear_ped_cover_clipset_override_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_CLEAR_PED_COVER_CLIPSET_OVERRIDE(ped)
}

/// ## Parameters
* **ped**: 
* **tintIndex**:

pub fn set_ped_parachute_tint_index_safe(ped: Ped, tintIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_PARACHUTE_TINT_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_PARACHUTE_TINT_INDEX(ped, tintIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_PARACHUTE_TINT_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_parachute_tint_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_parachute_tint_index_raw(ped: i32, tintIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_PARACHUTE_TINT_INDEX(ped, tintIndex)
}

/// [Animations list](https://alexguirre.github.io/animations-list/)

pub fn get_anim_initial_offset_rotation_safe(animDict: String, animName: String, x: f32, y: f32, z: f32, xRot: f32, yRot: f32, zRot: f32, p8: f32, p9: i64) -> NativeResult<Vector3> {
    let animDict_cstr = std::ffi::CString::new(animDict.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animDict", e)))?;
    let animName_cstr = std::ffi::CString::new(animName.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animName", e)))?;
    
    debug!("Calling native function: GET_ANIM_INITIAL_OFFSET_ROTATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_ANIM_INITIAL_OFFSET_ROTATION(crate::utils::rust_to_c_string(animDict), crate::utils::rust_to_c_string(animName), x, y, z, xRot, yRot, zRot, p8, p9)
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: GET_ANIM_INITIAL_OFFSET_ROTATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_anim_initial_offset_rotation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_anim_initial_offset_rotation_raw(animDict: *const std::os::raw::c_char, animName: *const std::os::raw::c_char, x: f32, y: f32, z: f32, xRot: f32, yRot: f32, zRot: f32, p8: f32, p9: i64) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_ANIM_INITIAL_OFFSET_ROTATION(animDict, animName, x, y, z, xRot, yRot, zRot, p8, p9)
}

/// ## Parameters
* **ped**:

pub fn is_conversation_ped_dead_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_CONVERSATION_PED_DEAD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_CONVERSATION_PED_DEAD(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_CONVERSATION_PED_DEAD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_conversation_ped_dead_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_conversation_ped_dead_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_CONVERSATION_PED_DEAD(ped)
}

/// ## Parameters
* **model**: 
* **toggle**:

pub fn set_ped_model_is_suppressed_safe() -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_MODEL_IS_SUPPRESSED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_MODEL_IS_SUPPRESSED()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_MODEL_IS_SUPPRESSED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_model_is_suppressed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_model_is_suppressed_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_MODEL_IS_SUPPRESSED()
}

/// ## Parameters
* **ped**:

pub fn stop_ped_weapon_firing_when_dropped_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: STOP_PED_WEAPON_FIRING_WHEN_DROPPED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::STOP_PED_WEAPON_FIRING_WHEN_DROPPED(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: STOP_PED_WEAPON_FIRING_WHEN_DROPPED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `stop_ped_weapon_firing_when_dropped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn stop_ped_weapon_firing_when_dropped_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::STOP_PED_WEAPON_FIRING_WHEN_DROPPED(ped)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_use_auto_conversation_lookat_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_USE_AUTO_CONVERSATION_LOOKAT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_USE_AUTO_CONVERSATION_LOOKAT(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_USE_AUTO_CONVERSATION_LOOKAT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_use_auto_conversation_lookat_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_use_auto_conversation_lookat_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_USE_AUTO_CONVERSATION_LOOKAT(ped, toggle)
}

/// ```
Only appears in lamar1 script.  
```

pub fn _0x1a330d297aac6bc1_safe(ped: Ped, p1: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _0x1A330D297AAC6BC1");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x1A330D297AAC6BC1(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x1A330D297AAC6BC1
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x1a330d297aac6bc1_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x1a330d297aac6bc1_raw(ped: i32, p1: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x1A330D297AAC6BC1(ped, p1)
}

/// ## Parameters
* **ped**:

pub fn is_ped_in_group_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_GROUP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_GROUP(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_GROUP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_group_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_group_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_GROUP(ped)
}

/// Sets the IK target for a given IK part belonging to the ped.

pub fn set_ik_target_safe(ped: Ped, ikIndex: i64, entityLookAt: Entity, boneLookAt: i64, offsetX: f32, offsetY: f32, offsetZ: f32, ikTargetFlags: i64, blendInDuration: i64, blendOutDuration: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_IK_TARGET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_IK_TARGET(ped, ikIndex, entityLookAt, boneLookAt, offsetX, offsetY, offsetZ, ikTargetFlags, blendInDuration, blendOutDuration)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_IK_TARGET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ik_target_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ik_target_raw(ped: i32, ikIndex: i64, entityLookAt: i32, boneLookAt: i64, offsetX: f32, offsetY: f32, offsetZ: f32, ikTargetFlags: i64, blendInDuration: i64, blendOutDuration: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_IK_TARGET(ped, ikIndex, entityLookAt, boneLookAt, offsetX, offsetY, offsetZ, ikTargetFlags, blendInDuration, blendOutDuration)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_never_leaves_group_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_NEVER_LEAVES_GROUP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_NEVER_LEAVES_GROUP(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_NEVER_LEAVES_GROUP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_never_leaves_group_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_never_leaves_group_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_NEVER_LEAVES_GROUP(ped, toggle)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_evasive_dive_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_EVASIVE_DIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_EVASIVE_DIVE(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_EVASIVE_DIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_evasive_dive_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_evasive_dive_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_EVASIVE_DIVE(ped, toggle)
}

/// ## Parameters
* **vehicle**: 
* **pedType**: See [`CREATE_PED`](#_0xD49F9B0955C367DE)
* **modelHash**: 
* **seat**: 
* **isNetwork**: 
* **bScriptHostPed**:

pub fn create_ped_inside_vehicle_safe(vehicle: Vehicle, pedType: i64, modelHash: u32, seat: i64, isNetwork: bool, bScriptHostPed: bool) -> NativeResult<Ped> {
    
    debug!("Calling native function: CREATE_PED_INSIDE_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CREATE_PED_INSIDE_VEHICLE(vehicle, pedType, modelHash, seat, isNetwork, bScriptHostPed)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: CREATE_PED_INSIDE_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `create_ped_inside_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn create_ped_inside_vehicle_raw(vehicle: i32, pedType: i64, modelHash: u32, seat: i64, isNetwork: bool, bScriptHostPed: bool) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CREATE_PED_INSIDE_VEHICLE(vehicle, pedType, modelHash, seat, isNetwork, bScriptHostPed)
}

/// ## Parameters
* **ped**: 
* **vehicle**:

pub fn is_ped_on_specific_vehicle_safe(ped: Ped, vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_ON_SPECIFIC_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_ON_SPECIFIC_VEHICLE(ped, vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_ON_SPECIFIC_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_on_specific_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_on_specific_vehicle_raw(ped: i32, vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_ON_SPECIFIC_VEHICLE(ped, vehicle)
}

/// ## Parameters
* **ped**:

pub fn is_ped_hanging_on_to_vehicle_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_HANGING_ON_TO_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_HANGING_ON_TO_VEHICLE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_HANGING_ON_TO_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_hanging_on_to_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_hanging_on_to_vehicle_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_HANGING_ON_TO_VEHICLE(ped)
}

/// **Usage:** Call this native every frame
    
## Parameters
* **multiplier**: Adjust from 0.0 (minimum, indicating no pedestrians in the street) to 1.0 (maximum, representing a normal amount of pedestrians on the street).

pub fn set_ped_density_multiplier_this_frame_safe(multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_DENSITY_MULTIPLIER_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_DENSITY_MULTIPLIER_THIS_FRAME(multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_DENSITY_MULTIPLIER_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_density_multiplier_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_density_multiplier_this_frame_raw(multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_DENSITY_MULTIPLIER_THIS_FRAME(multiplier)
}

/// ```
Based on TASK_COMBAT_HATED_TARGETS_AROUND_PED, the parameters are likely similar (PedHandle, and area to attack in).  
```

pub fn register_hated_targets_around_ped_safe(ped: Ped, radius: f32) -> NativeResult<()> {
    
    debug!("Calling native function: REGISTER_HATED_TARGETS_AROUND_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REGISTER_HATED_TARGETS_AROUND_PED(ped, radius)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REGISTER_HATED_TARGETS_AROUND_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `register_hated_targets_around_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn register_hated_targets_around_ped_raw(ped: i32, radius: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REGISTER_HATED_TARGETS_AROUND_PED(ped, radius)
}

/// ## Parameters
* **ped**: The ped handle.
* **componentId**: The component that you want to set. Refer to [SET_PED_COMPONENT_VARIATION](#_0x262B14F48D29DE80).
* **drawableId**: The drawable id that is going to be set. Refer to [GET_NUMBER_OF_PED_PROP_DRAWABLE_VARIATIONS](#_0x5FAF9754E789FB47).
* **textureId**: The texture id of the drawable. Refer to [GET_NUMBER_OF_PED_PROP_TEXTURE_VARIATIONS](#_0xA6E7F1CEB523E171).

pub fn set_ped_preload_prop_data_safe(ped: Ped, componentId: i64, drawableId: i64, textureId: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: SET_PED_PRELOAD_PROP_DATA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_PRELOAD_PROP_DATA(ped, componentId, drawableId, textureId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: SET_PED_PRELOAD_PROP_DATA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_preload_prop_data_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_preload_prop_data_raw(ped: i32, componentId: i64, drawableId: i64, textureId: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_PRELOAD_PROP_DATA(ped, componentId, drawableId, textureId)
}

/// ## Parameters
* **x**: 
* **y**: 
* **z**:

pub fn set_scripted_conversion_coord_this_frame_safe(x: f32, y: f32, z: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SCRIPTED_CONVERSION_COORD_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SCRIPTED_CONVERSION_COORD_THIS_FRAME(x, y, z)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SCRIPTED_CONVERSION_COORD_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_scripted_conversion_coord_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_scripted_conversion_coord_this_frame_raw(x: f32, y: f32, z: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SCRIPTED_CONVERSION_COORD_THIS_FRAME(x, y, z)
}

/// ## Parameters
* **sceneID**:

pub fn get_synchronized_scene_rate_safe(sceneID: i64) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_SYNCHRONIZED_SCENE_RATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_SYNCHRONIZED_SCENE_RATE(sceneID)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_SYNCHRONIZED_SCENE_RATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_synchronized_scene_rate_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_synchronized_scene_rate_raw(sceneID: i64) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_SYNCHRONIZED_SCENE_RATE(sceneID)
}

/// ```
Values look to be between 0.0 and 1.0  
From decompiled scripts: 0.0, 0.6, 0.65, 0.8, 1.0  
You are correct, just looked in IDA it breaks from the function if it's less than 0.0f or greater than 1.0f.  
```

pub fn set_ped_enveff_scale_safe(ped: Ped, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_ENVEFF_SCALE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_ENVEFF_SCALE(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_ENVEFF_SCALE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_enveff_scale_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_enveff_scale_raw(ped: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_ENVEFF_SCALE(ped, value)
}

/// ## Parameters
* **ped**:

pub fn get_ped_money_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_MONEY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_MONEY(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_MONEY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_money_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_money_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_MONEY(ped)
}

/// STOP_ANY_PED_MODEL_BEING_SUPPRESSED native function

pub fn stop_any_ped_model_being_suppressed_safe() -> NativeResult<()> {
    
    debug!("Calling native function: STOP_ANY_PED_MODEL_BEING_SUPPRESSED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::STOP_ANY_PED_MODEL_BEING_SUPPRESSED()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: STOP_ANY_PED_MODEL_BEING_SUPPRESSED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `stop_any_ped_model_being_suppressed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn stop_any_ped_model_being_suppressed_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::STOP_ANY_PED_MODEL_BEING_SUPPRESSED()
}

/// ```
NativeDB Introduced: v2699
```

pub fn _get_ped_dies_in_water_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: _GET_PED_DIES_IN_WATER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_PED_DIES_IN_WATER(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_PED_DIES_IN_WATER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_ped_dies_in_water_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_ped_dies_in_water_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_PED_DIES_IN_WATER(ped)
}

/// ```
CLEAR_PED_*
```

pub fn _0x80054d7fcc70eec6_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: _0x80054D7FCC70EEC6");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x80054D7FCC70EEC6(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x80054D7FCC70EEC6
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x80054d7fcc70eec6_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x80054d7fcc70eec6_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x80054D7FCC70EEC6(ped)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_to_load_cover_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_TO_LOAD_COVER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_TO_LOAD_COVER(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_TO_LOAD_COVER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_to_load_cover_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_to_load_cover_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_TO_LOAD_COVER(ped, toggle)
}

/// ## Parameters
* **ped**: 
* **entity**:

pub fn is_ped_headtracking_entity_safe(ped: Ped, entity: Entity) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_HEADTRACKING_ENTITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_HEADTRACKING_ENTITY(ped, entity)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_HEADTRACKING_ENTITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_headtracking_entity_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_headtracking_entity_raw(ped: i32, entity: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_HEADTRACKING_ENTITY(ped, entity)
}

/// ```
Gets ID of vehicle player using. It means it can get ID at any interaction with vehicle. Enter\exit for example. And that means it is faster than GET_VEHICLE_PED_IS_IN but less safe.  
```

pub fn get_vehicle_ped_is_using_safe(ped: Ped) -> NativeResult<Vehicle> {
    
    debug!("Calling native function: GET_VEHICLE_PED_IS_USING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_PED_IS_USING(ped)
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: GET_VEHICLE_PED_IS_USING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_ped_is_using_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_ped_is_using_raw(ped: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_PED_IS_USING(ped)
}

/// ```
If the ped handle passed through the parenthesis is in a ragdoll state this will return true.  
```

pub fn is_ped_ragdoll_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_RAGDOLL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_RAGDOLL(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_RAGDOLL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_ragdoll_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_ragdoll_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_RAGDOLL(ped)
}

/// ```
Resets the value for the last vehicle driven by the Ped.  
```

pub fn reset_ped_last_vehicle_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: RESET_PED_LAST_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_PED_LAST_VEHICLE(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_PED_LAST_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_ped_last_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_ped_last_vehicle_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_PED_LAST_VEHICLE(ped)
}

/// ## Parameters
* **ped**:

pub fn set_ped_should_play_normal_scenario_exit_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_SHOULD_PLAY_NORMAL_SCENARIO_EXIT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_SHOULD_PLAY_NORMAL_SCENARIO_EXIT(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_SHOULD_PLAY_NORMAL_SCENARIO_EXIT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_should_play_normal_scenario_exit_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_should_play_normal_scenario_exit_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_SHOULD_PLAY_NORMAL_SCENARIO_EXIT(ped)
}

/// RESET_AI_MELEE_WEAPON_DAMAGE_MODIFIER native function

pub fn reset_ai_melee_weapon_damage_modifier_safe() -> NativeResult<()> {
    
    debug!("Calling native function: RESET_AI_MELEE_WEAPON_DAMAGE_MODIFIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_AI_MELEE_WEAPON_DAMAGE_MODIFIER()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_AI_MELEE_WEAPON_DAMAGE_MODIFIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_ai_melee_weapon_damage_modifier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_ai_melee_weapon_damage_modifier_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_AI_MELEE_WEAPON_DAMAGE_MODIFIER()
}

/// ```
Returns size of array, passed into the second variable.  
See below for usage information.  
This function actually requires a struct, where the first value is the maximum number of elements to return.  Here is a sample of how I was able to get it to work correctly, without yet knowing the struct format.  
//Setup the array  
	const int numElements = 10;  
	const int arrSize = numElements * 2 + 2;  
	Any veh[arrSize];  
	//0 index is the size of the array  
	veh[0] = numElements;  
	int count = PED::GET_PED_NEARBY_VEHICLES(PLAYER::PLAYER_PED_ID(), veh);  
	if (veh != NULL)  
	{  
//Simple loop to go through results  
for (int i = 0; i < count; i++)  
{  
	int offsettedID = i * 2 + 2;  
	//Make sure it exists  
	if (veh[offsettedID] != NULL && ENTITY::DOES_ENTITY_EXIST(veh[offsettedID]))  
	{  
//Do something  
	}  
}  
	}    
Here's the right way to do it (console and pc):  
pastebin.com/SsFej963  
```

pub fn get_ped_nearby_vehicles_safe(ped: Ped, sizeAndVehs: *mut i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_NEARBY_VEHICLES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_NEARBY_VEHICLES(ped, sizeAndVehs)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_NEARBY_VEHICLES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_nearby_vehicles_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_nearby_vehicles_raw(ped: i32, sizeAndVehs: *mut i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_NEARBY_VEHICLES(ped, sizeAndVehs)
}

/// ```
p1 is always 0 in R* scripts; and a quick disassembly seems to indicate that p1 is unused.  
```

pub fn set_ped_random_component_variation_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_RANDOM_COMPONENT_VARIATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_RANDOM_COMPONENT_VARIATION(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_RANDOM_COMPONENT_VARIATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_random_component_variation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_random_component_variation_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_RANDOM_COMPONENT_VARIATION(ped)
}

/// See [`SET_PED_HEAD_BLEND_DATA`](#_0x9414E18B9434C2FE)

pub fn update_ped_head_blend_data_safe(ped: Ped, shapeMix: f32, skinMix: f32, thirdMix: f32) -> NativeResult<()> {
    
    debug!("Calling native function: UPDATE_PED_HEAD_BLEND_DATA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::UPDATE_PED_HEAD_BLEND_DATA(ped, shapeMix, skinMix, thirdMix)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: UPDATE_PED_HEAD_BLEND_DATA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `update_ped_head_blend_data_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn update_ped_head_blend_data_raw(ped: i32, shapeMix: f32, skinMix: f32, thirdMix: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::UPDATE_PED_HEAD_BLEND_DATA(ped, shapeMix, skinMix, thirdMix)
}

/// ## Parameters
* **ped**: 
* **radius**: 
* **maxFriends**:

pub fn set_ped_to_inform_respected_friends_safe(ped: Ped, radius: f32, maxFriends: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_TO_INFORM_RESPECTED_FRIENDS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_TO_INFORM_RESPECTED_FRIENDS(ped, radius, maxFriends)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_TO_INFORM_RESPECTED_FRIENDS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_to_inform_respected_friends_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_to_inform_respected_friends_raw(ped: i32, radius: f32, maxFriends: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_TO_INFORM_RESPECTED_FRIENDS(ped, radius, maxFriends)
}

/// ```
teleports ped to coords along with the vehicle ped is in  
```

pub fn set_ped_coords_keep_vehicle_safe(ped: Ped, posX: f32, posY: f32, posZ: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_COORDS_KEEP_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_COORDS_KEEP_VEHICLE(ped, posX, posY, posZ)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_COORDS_KEEP_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_coords_keep_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_coords_keep_vehicle_raw(ped: i32, posX: f32, posY: f32, posZ: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_COORDS_KEEP_VEHICLE(ped, posX, posY, posZ)
}

/// Retrieves the vehicle the specified ped is currently in, or the last vehicle they were in.

pub fn get_vehicle_ped_is_in_safe(ped: Ped, lastVehicle: bool) -> NativeResult<Vehicle> {
    
    debug!("Calling native function: GET_VEHICLE_PED_IS_IN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_PED_IS_IN(ped, lastVehicle)
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: GET_VEHICLE_PED_IS_IN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_ped_is_in_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_ped_is_in_raw(ped: i32, lastVehicle: bool) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_PED_IS_IN(ped, lastVehicle)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x9e30e91fb03a2caf_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<bool> {
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
    
    debug!("Calling native function: _0x9E30E91FB03A2CAF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x9E30E91FB03A2CAF(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0x9E30E91FB03A2CAF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x9e30e91fb03a2caf_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x9e30e91fb03a2caf_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x9E30E91FB03A2CAF(p0, p1)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_generates_dead_body_events_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_GENERATES_DEAD_BODY_EVENTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_GENERATES_DEAD_BODY_EVENTS(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_GENERATES_DEAD_BODY_EVENTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_generates_dead_body_events_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_generates_dead_body_events_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_GENERATES_DEAD_BODY_EVENTS(ped, toggle)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_play_ambient_base_anims_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_PLAY_AMBIENT_BASE_ANIMS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_PLAY_AMBIENT_BASE_ANIMS(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_PLAY_AMBIENT_BASE_ANIMS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_play_ambient_base_anims_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_play_ambient_base_anims_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_PLAY_AMBIENT_BASE_ANIMS(ped, toggle)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x711794453cfd692b_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x711794453CFD692B");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x711794453CFD692B(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x711794453CFD692B
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x711794453cfd692b_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x711794453cfd692b_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x711794453CFD692B(p0, p1)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_leg_ik_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_LEG_IK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_LEG_IK(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_LEG_IK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_leg_ik_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_leg_ik_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_LEG_IK(ped, toggle)
}

/// ## Parameters
* **sceneID**:

pub fn is_synchronized_scene_looped_safe(sceneID: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_SYNCHRONIZED_SCENE_LOOPED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_SYNCHRONIZED_SCENE_LOOPED(sceneID)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_SYNCHRONIZED_SCENE_LOOPED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_synchronized_scene_looped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_synchronized_scene_looped_raw(sceneID: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_SYNCHRONIZED_SCENE_LOOPED(sceneID)
}

/// ```
name: "MP_FEMALE_ACTION" found multiple times in the b617d scripts.
```

pub fn set_movement_mode_override_safe(ped: Ped, name: String) -> NativeResult<()> {
    let name_cstr = std::ffi::CString::new(name.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "name", e)))?;
    
    debug!("Calling native function: SET_MOVEMENT_MODE_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_MOVEMENT_MODE_OVERRIDE(ped, crate::utils::rust_to_c_string(name))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_MOVEMENT_MODE_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_movement_mode_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_movement_mode_override_raw(ped: i32, name: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_MOVEMENT_MODE_OVERRIDE(ped, name)
}

/// ## Parameters
* **ped**: 
* **groupId**:

pub fn set_ped_as_group_member_safe(ped: Ped, groupId: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_AS_GROUP_MEMBER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_AS_GROUP_MEMBER(ped, groupId)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_AS_GROUP_MEMBER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_as_group_member_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_as_group_member_raw(ped: i32, groupId: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_AS_GROUP_MEMBER(ped, groupId)
}

/// ## Parameters
* **ped**: The ped handle.
* **componentId**: The component id you want to get the texture variations of. Refer to [SET_PED_COMPONENT_VARIATION](#_0x262B14F48D29DE80).
* **drawableId**: The drawable id of the component you want to get the texture variations of. Refer to [GET_NUMBER_OF_PED_PROP_DRAWABLE_VARIATIONS](#_0x5FAF9754E789FB47).

pub fn get_number_of_ped_texture_variations_safe(ped: Ped, componentId: i64, drawableId: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_NUMBER_OF_PED_TEXTURE_VARIATIONS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_NUMBER_OF_PED_TEXTURE_VARIATIONS(ped, componentId, drawableId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_NUMBER_OF_PED_TEXTURE_VARIATIONS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_number_of_ped_texture_variations_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_number_of_ped_texture_variations_raw(ped: i32, componentId: i64, drawableId: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_NUMBER_OF_PED_TEXTURE_VARIATIONS(ped, componentId, drawableId)
}

/// ## Parameters
* **p0**:

pub fn _0x412f1364fa066cfb_safe(p0: serde_json::Value) -> NativeResult<serde_json::Value> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x412F1364FA066CFB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x412F1364FA066CFB(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0x412F1364FA066CFB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x412f1364fa066cfb_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x412f1364fa066cfb_raw(p0: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x412F1364FA066CFB(p0)
}

/// ```
Overrides the ped's collision capsule radius for the current tick.  
Must be called every tick to be effective.  
Setting this to 0.001 will allow warping through some objects.  
```

pub fn set_ped_capsule_safe(ped: Ped, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAPSULE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAPSULE(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAPSULE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_capsule_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_capsule_raw(ped: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAPSULE(ped, value)
}

/// ## Parameters
* **ped**:

pub fn clear_ped_decorations_leave_scars_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_DECORATIONS_LEAVE_SCARS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_DECORATIONS_LEAVE_SCARS(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_DECORATIONS_LEAVE_SCARS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_decorations_leave_scars_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_decorations_leave_scars_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_DECORATIONS_LEAVE_SCARS(ped)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x3e9679c1dfcf422c_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x3E9679C1DFCF422C");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x3E9679C1DFCF422C(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x3E9679C1DFCF422C
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x3e9679c1dfcf422c_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x3e9679c1dfcf422c_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x3E9679C1DFCF422C(p0, p1)
}

/// ```
Detect if ped is in any vehicle  
[True/False]  
```

pub fn is_ped_sitting_in_any_vehicle_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_SITTING_IN_ANY_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_SITTING_IN_ANY_VEHICLE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_SITTING_IN_ANY_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_sitting_in_any_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_sitting_in_any_vehicle_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_SITTING_IN_ANY_VEHICLE(ped)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn get_ped_defensive_area_position_safe(ped: Ped, p1: bool) -> NativeResult<Vector3> {
    
    debug!("Calling native function: GET_PED_DEFENSIVE_AREA_POSITION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_DEFENSIVE_AREA_POSITION(ped, p1)
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: GET_PED_DEFENSIVE_AREA_POSITION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_defensive_area_position_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_defensive_area_position_raw(ped: i32, p1: bool) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_DEFENSIVE_AREA_POSITION(ped, p1)
}

/// Creates a ped (biped character, pedestrian, actor) with the specified model at the specified position and heading.
This ped will initially be owned by the creating script as a mission entity, and the model should be loaded already
(e.g. using REQUEST_MODEL).

pub fn create_ped_safe(pedType: i64, modelHash: u32, x: f32, y: f32, z: f32, heading: f32, isNetwork: bool, bScriptHostPed: bool) -> NativeResult<Ped> {
    
    debug!("Calling native function: CREATE_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CREATE_PED(pedType, modelHash, x, y, z, heading, isNetwork, bScriptHostPed)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: CREATE_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `create_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn create_ped_raw(pedType: i64, modelHash: u32, x: f32, y: f32, z: f32, heading: f32, isNetwork: bool, bScriptHostPed: bool) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CREATE_PED(pedType, modelHash, x, y, z, heading, isNetwork, bScriptHostPed)
}

/// _SET_BLOCK_AMBIENT_PEDS_FROM_DROPPING_WEAPONS_THIS_FRAME native function

pub fn _set_block_ambient_peds_from_dropping_weapons_this_frame_safe() -> NativeResult<()> {
    
    debug!("Calling native function: _SET_BLOCK_AMBIENT_PEDS_FROM_DROPPING_WEAPONS_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_BLOCK_AMBIENT_PEDS_FROM_DROPPING_WEAPONS_THIS_FRAME()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_BLOCK_AMBIENT_PEDS_FROM_DROPPING_WEAPONS_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_block_ambient_peds_from_dropping_weapons_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_block_ambient_peds_from_dropping_weapons_this_frame_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_BLOCK_AMBIENT_PEDS_FROM_DROPPING_WEAPONS_THIS_FRAME()
}

/// ```
gtaforums.com/topic/885580-ped-headshotmugshot-txd/  
```

pub fn is_pedheadshot_valid_safe(id: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PEDHEADSHOT_VALID");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PEDHEADSHOT_VALID(id)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PEDHEADSHOT_VALID
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_pedheadshot_valid_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_pedheadshot_valid_raw(id: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PEDHEADSHOT_VALID(id)
}

/// ## Parameters
* **ped**:

pub fn is_ped_performing_stealth_kill_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_PERFORMING_STEALTH_KILL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_PERFORMING_STEALTH_KILL(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_PERFORMING_STEALTH_KILL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_performing_stealth_kill_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_performing_stealth_kill_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_PERFORMING_STEALTH_KILL(ped)
}

/// Used with [SET_ENABLE_HANDCUFFS](#_0xDF1AF8B5D56542FA) in decompiled scripts. From my observations, I have noticed that while being ragdolled you are not able to get up but you can still run. Your legs can also bend.

pub fn set_enable_bound_ankles_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_ENABLE_BOUND_ANKLES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_ENABLE_BOUND_ANKLES(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_ENABLE_BOUND_ANKLES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_enable_bound_ankles_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_enable_bound_ankles_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_ENABLE_BOUND_ANKLES(ped, toggle)
}

/// ```
From the scripts:  
PED::SET_PED_GESTURE_GROUP(PLAYER::PLAYER_PED_ID(),  
"ANIM_GROUP_GESTURE_MISS_FRA0");  
PED::SET_PED_GESTURE_GROUP(PLAYER::PLAYER_PED_ID(),  
"ANIM_GROUP_GESTURE_MISS_DocksSetup1");  
```

pub fn set_ped_gesture_group_safe(ped: Ped, animGroupGesture: String) -> NativeResult<()> {
    let animGroupGesture_cstr = std::ffi::CString::new(animGroupGesture.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animGroupGesture", e)))?;
    
    debug!("Calling native function: SET_PED_GESTURE_GROUP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_GESTURE_GROUP(ped, crate::utils::rust_to_c_string(animGroupGesture))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_GESTURE_GROUP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_gesture_group_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_gesture_group_raw(ped: i32, animGroupGesture: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_GESTURE_GROUP(ped, animGroupGesture)
}

/// ```
gtaforums.com/topic/885580-ped-headshotmugshot-txd/  
```

pub fn register_pedheadshot_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: REGISTER_PEDHEADSHOT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REGISTER_PEDHEADSHOT(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: REGISTER_PEDHEADSHOT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `register_pedheadshot_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn register_pedheadshot_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REGISTER_PEDHEADSHOT(ped)
}

/// ## Parameters
* **x**: 
* **y**: 
* **z**: 
* **radius**: 
* **object**:

pub fn _create_synchronized_scene_2_safe(x: f32, y: f32, z: f32, radius: f32, object: u32) -> NativeResult<i64> {
    
    debug!("Calling native function: _CREATE_SYNCHRONIZED_SCENE_2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_CREATE_SYNCHRONIZED_SCENE_2(x, y, z, radius, object)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _CREATE_SYNCHRONIZED_SCENE_2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_create_synchronized_scene_2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _create_synchronized_scene_2_raw(x: f32, y: f32, z: f32, radius: f32, object: u32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_CREATE_SYNCHRONIZED_SCENE_2(x, y, z, radius, object)
}

/// ## Parameters
* **groupId**: 
* **p1**: 
* **p2**: 
* **p3**:

pub fn set_group_formation_spacing_safe(groupId: i64, p1: f32, p2: f32, p3: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_GROUP_FORMATION_SPACING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_GROUP_FORMATION_SPACING(groupId, p1, p2, p3)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_GROUP_FORMATION_SPACING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_group_formation_spacing_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_group_formation_spacing_raw(groupId: i64, p1: f32, p2: f32, p3: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_GROUP_FORMATION_SPACING(groupId, p1, p2, p3)
}

/// ```
p1 is always 0  
```

pub fn is_ped_being_stunned_safe(ped: Ped, p1: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_BEING_STUNNED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_BEING_STUNNED(ped, p1)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_BEING_STUNNED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_being_stunned_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_being_stunned_raw(ped: i32, p1: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_BEING_STUNNED(ped, p1)
}

/// ```
NativeDB Introduced: v2189
```

pub fn _get_ped_event_data_safe(ped: Ped, eventType: i64, outData: serde_json::Value) -> NativeResult<bool> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let outData_any_str = serde_json::to_string(&outData)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "outData", e)))?;
    let outData_any_str_cstr = std::ffi::CString::new(outData_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "outData", e)))?;
    
    debug!("Calling native function: _GET_PED_EVENT_DATA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_PED_EVENT_DATA(ped, eventType, crate::utils::any_to_c_void_ptr(outData))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_PED_EVENT_DATA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_ped_event_data_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_ped_event_data_raw(ped: i32, eventType: i64, outData: *mut std::os::raw::c_void) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_PED_EVENT_DATA(ped, eventType, outData)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**: 
* **p5**:

pub fn _0x06087579e7aa85a9_safe(p0: serde_json::Value, p1: serde_json::Value, p2: f32, p3: f32, p4: f32, p5: f32) -> NativeResult<bool> {
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
    
    debug!("Calling native function: _0x06087579E7AA85A9");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x06087579E7AA85A9(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1), p2, p3, p4, p5)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0x06087579E7AA85A9
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x06087579e7aa85a9_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x06087579e7aa85a9_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void, p2: f32, p3: f32, p4: f32, p5: f32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x06087579E7AA85A9(p0, p1, p2, p3, p4, p5)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_ragdoll_from_player_impact_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_RAGDOLL_FROM_PLAYER_IMPACT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_RAGDOLL_FROM_PLAYER_IMPACT(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_RAGDOLL_FROM_PLAYER_IMPACT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_ragdoll_from_player_impact_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_ragdoll_from_player_impact_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_RAGDOLL_FROM_PLAYER_IMPACT(ped, toggle)
}

/// ```
Returns true/false if the ped is/isn't male.  
```

pub fn is_ped_male_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_MALE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_MALE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_MALE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_male_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_male_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_MALE(ped)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**:

pub fn apply_ped_blood_damage_by_zone_safe(ped: Ped, p1: serde_json::Value, p2: f32, p3: f32, p4: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p1_any_str = serde_json::to_string(&p1)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p1", e)))?;
    let p1_any_str_cstr = std::ffi::CString::new(p1_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p1", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p4_any_str = serde_json::to_string(&p4)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p4", e)))?;
    let p4_any_str_cstr = std::ffi::CString::new(p4_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p4", e)))?;
    
    debug!("Calling native function: APPLY_PED_BLOOD_DAMAGE_BY_ZONE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::APPLY_PED_BLOOD_DAMAGE_BY_ZONE(ped, crate::utils::any_to_c_void_ptr(p1), p2, p3, crate::utils::any_to_c_void_ptr(p4))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: APPLY_PED_BLOOD_DAMAGE_BY_ZONE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `apply_ped_blood_damage_by_zone_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn apply_ped_blood_damage_by_zone_raw(ped: i32, p1: *mut std::os::raw::c_void, p2: f32, p3: f32, p4: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::APPLY_PED_BLOOD_DAMAGE_BY_ZONE(ped, p1, p2, p3, p4)
}

/// ```
combined with PED::SET_PED_WETNESS_HEIGHT(), this native makes the ped drenched in water up to the height specified in the other function  
```

pub fn set_ped_wetness_enabled_this_frame_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_WETNESS_ENABLED_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_WETNESS_ENABLED_THIS_FRAME(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_WETNESS_ENABLED_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_wetness_enabled_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_wetness_enabled_this_frame_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_WETNESS_ENABLED_THIS_FRAME(ped)
}

/// ```
NativeDB Introduced: v1493
```

pub fn _is_scuba_gear_light_enabled_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_SCUBA_GEAR_LIGHT_ENABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_SCUBA_GEAR_LIGHT_ENABLED(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_SCUBA_GEAR_LIGHT_ENABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_scuba_gear_light_enabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_scuba_gear_light_enabled_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_SCUBA_GEAR_LIGHT_ENABLED(ped)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**:

pub fn _0x336b3d200ab007cb_safe(p0: serde_json::Value, p1: f32, p2: f32, p3: f32, p4: f32) -> NativeResult<serde_json::Value> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x336B3D200AB007CB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x336B3D200AB007CB(crate::utils::any_to_c_void_ptr(p0), p1, p2, p3, p4)
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0x336B3D200AB007CB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x336b3d200ab007cb_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x336b3d200ab007cb_raw(p0: *mut std::os::raw::c_void, p1: f32, p2: f32, p3: f32, p4: f32) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x336B3D200AB007CB(p0, p1, p2, p3, p4)
}

/// ```
Gets the position of the specified bone of the specified ped.  
ped: The ped to get the position of a bone from.  
boneId: The ID of the bone to get the position from. This is NOT the index.  
offsetX: The X-component of the offset to add to the position relative to the bone's rotation.  
offsetY: The Y-component of the offset to add to the position relative to the bone's rotation.  
offsetZ: The Z-component of the offset to add to the position relative to the bone's rotation.  
```

pub fn get_ped_bone_coords_safe(ped: Ped, boneId: i64, offsetX: f32, offsetY: f32, offsetZ: f32) -> NativeResult<Vector3> {
    
    debug!("Calling native function: GET_PED_BONE_COORDS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_BONE_COORDS(ped, boneId, offsetX, offsetY, offsetZ)
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: GET_PED_BONE_COORDS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_bone_coords_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_bone_coords_raw(ped: i32, boneId: i64, offsetX: f32, offsetY: f32, offsetZ: f32) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_BONE_COORDS(ped, boneId, offsetX, offsetY, offsetZ)
}

/// ## Parameters
* **asset**:

pub fn has_stealth_mode_asset_loaded_safe(asset: String) -> NativeResult<bool> {
    let asset_cstr = std::ffi::CString::new(asset.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "asset", e)))?;
    
    debug!("Calling native function: HAS_STEALTH_MODE_ASSET_LOADED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_STEALTH_MODE_ASSET_LOADED(crate::utils::rust_to_c_string(asset))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_STEALTH_MODE_ASSET_LOADED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_stealth_mode_asset_loaded_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_stealth_mode_asset_loaded_raw(asset: *const std::os::raw::c_char) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_STEALTH_MODE_ASSET_LOADED(asset)
}

/// [Animations list](https://alexguirre.github.io/animations-list/)

pub fn set_ped_alternate_walk_anim_safe(ped: Ped, animDict: String, animName: String, p3: f32, p4: bool) -> NativeResult<()> {
    let animDict_cstr = std::ffi::CString::new(animDict.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animDict", e)))?;
    let animName_cstr = std::ffi::CString::new(animName.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animName", e)))?;
    
    debug!("Calling native function: SET_PED_ALTERNATE_WALK_ANIM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_ALTERNATE_WALK_ANIM(ped, crate::utils::rust_to_c_string(animDict), crate::utils::rust_to_c_string(animName), p3, p4)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_ALTERNATE_WALK_ANIM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_alternate_walk_anim_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_alternate_walk_anim_raw(ped: i32, animDict: *const std::os::raw::c_char, animName: *const std::os::raw::c_char, p3: f32, p4: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_ALTERNATE_WALK_ANIM(ped, animDict, animName, p3, p4)
}

/// ```
Judging purely from a quick disassembly, if the ped is in a vehicle, the ped will be deleted immediately. If not, it'll be marked as no longer needed. 
```

pub fn remove_ped_elegantly_safe(ped: *mut i32) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_PED_ELEGANTLY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_PED_ELEGANTLY(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_PED_ELEGANTLY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_ped_elegantly_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_ped_elegantly_raw(ped: *mut i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_PED_ELEGANTLY(ped)
}

/// [`SET_VEHICLE_STEER_BIAS`](#_0x42A8EC77D5150CBE) for peds, e.g., `_SET_PED_STEER_BIAS`.

pub fn _0x288df530c92dad6f_safe(ped: Ped, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0x288DF530C92DAD6F");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x288DF530C92DAD6F(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x288DF530C92DAD6F
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x288df530c92dad6f_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x288df530c92dad6f_raw(ped: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x288DF530C92DAD6F(ped, value)
}

/// ## Parameters
* **ped**:

pub fn reset_ped_in_vehicle_context_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: RESET_PED_IN_VEHICLE_CONTEXT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_PED_IN_VEHICLE_CONTEXT(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_PED_IN_VEHICLE_CONTEXT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_ped_in_vehicle_context_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_ped_in_vehicle_context_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_PED_IN_VEHICLE_CONTEXT(ped)
}

/// ## Parameters
* **ped**: 
* **target**:

pub fn can_ped_in_combat_see_target_safe(ped: Ped, target: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: CAN_PED_IN_COMBAT_SEE_TARGET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CAN_PED_IN_COMBAT_SEE_TARGET(ped, target)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CAN_PED_IN_COMBAT_SEE_TARGET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `can_ped_in_combat_see_target_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn can_ped_in_combat_see_target_raw(ped: i32, target: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CAN_PED_IN_COMBAT_SEE_TARGET(ped, target)
}

/// Kicks the ped from the current vehicle and keeps the rendering-focus on this ped (also disables its collision). If doing this for your player ped, you'll still be able to drive the vehicle.  
Only to be used in very specific situations where the ped needs to be inside the car still but not attached.

pub fn special_function_do_not_use_safe(ped: Ped, noCollisionUntilClear: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SPECIAL_FUNCTION_DO_NOT_USE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPECIAL_FUNCTION_DO_NOT_USE(ped, noCollisionUntilClear)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPECIAL_FUNCTION_DO_NOT_USE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `special_function_do_not_use_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn special_function_do_not_use_raw(ped: i32, noCollisionUntilClear: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPECIAL_FUNCTION_DO_NOT_USE(ped, noCollisionUntilClear)
}

/// ## Parameters
* **ped**:

pub fn reset_ped_strafe_clipset_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: RESET_PED_STRAFE_CLIPSET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_PED_STRAFE_CLIPSET(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_PED_STRAFE_CLIPSET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_ped_strafe_clipset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_ped_strafe_clipset_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_PED_STRAFE_CLIPSET(ped)
}

/// ## Return value

pub fn can_create_random_cops_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: CAN_CREATE_RANDOM_COPS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CAN_CREATE_RANDOM_COPS()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CAN_CREATE_RANDOM_COPS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `can_create_random_cops_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn can_create_random_cops_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CAN_CREATE_RANDOM_COPS()
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_allowed_to_duck_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_ALLOWED_TO_DUCK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_ALLOWED_TO_DUCK(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_ALLOWED_TO_DUCK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_allowed_to_duck_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_allowed_to_duck_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_ALLOWED_TO_DUCK(ped, toggle)
}

/// ```
REQUEST_*
```

pub fn _0xcd018c591f94cb43_safe(ped: Ped, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xCD018C591F94CB43");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xCD018C591F94CB43(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xCD018C591F94CB43
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xcd018c591f94cb43_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xcd018c591f94cb43_raw(ped: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xCD018C591F94CB43(ped, p1)
}

/// Used for freemode (online) characters.  

Indices:
  1. black
  2. very light blue/green
  3. dark blue
  4. brown
  5. darker brown
  6. light brown
  7. blue
  8. light blue
  9. pink
  10. yellow
  11. purple
  12. black
  13. dark green
  14. light brown
  15. yellow/black pattern
  16. light colored spiral pattern
  17. shiny red
  18. shiny half blue/half red
  19. half black/half light blue
  20. white/red perimter
  21. green snake
  22. red snake
  23. dark blue snake
  24. dark yellow
  25. bright yellow
  26. all black
  28. red small pupil
  29. devil blue/black
  30. white small pupil
  31. glossed over

pub fn _set_ped_eye_color_safe(ped: Ped, index: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PED_EYE_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PED_EYE_COLOR(ped, index)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PED_EYE_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_ped_eye_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_ped_eye_color_raw(ped: i32, index: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PED_EYE_COLOR(ped, index)
}

/// ## Parameters
* **ped**:

pub fn _0xc2ee020f5fb4db53_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: _0xC2EE020F5FB4DB53");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xC2EE020F5FB4DB53(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xC2EE020F5FB4DB53
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xc2ee020f5fb4db53_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xc2ee020f5fb4db53_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xC2EE020F5FB4DB53(ped)
}

/// ## Parameters
* **ped**: 
* **player**: 
* **toggle**:

pub fn set_ped_can_be_targetted_by_player_safe(ped: Ped, player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_BE_TARGETTED_BY_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_BE_TARGETTED_BY_PLAYER(ped, player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_BE_TARGETTED_BY_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_be_targetted_by_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_be_targetted_by_player_raw(ped: i32, player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_BE_TARGETTED_BY_PLAYER(ped, player, toggle)
}

/// ```
SET_PED_*  
Has most likely to do with some shooting attributes as it sets the float which is in the same range as shootRate.  
```

pub fn _0xec4b4b3b9908052a_safe(ped: Ped, unk: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0xEC4B4B3B9908052A");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xEC4B4B3B9908052A(ped, unk)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xEC4B4B3B9908052A
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xec4b4b3b9908052a_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xec4b4b3b9908052a_raw(ped: i32, unk: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xEC4B4B3B9908052A(ped, unk)
}

/// ## Return value

pub fn can_create_random_driver_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: CAN_CREATE_RANDOM_DRIVER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CAN_CREATE_RANDOM_DRIVER()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CAN_CREATE_RANDOM_DRIVER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `can_create_random_driver_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn can_create_random_driver_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CAN_CREATE_RANDOM_DRIVER()
}

/// SPAWNPOINTS_CANCEL_SEARCH native function

pub fn spawnpoints_cancel_search_safe() -> NativeResult<()> {
    
    debug!("Calling native function: SPAWNPOINTS_CANCEL_SEARCH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPAWNPOINTS_CANCEL_SEARCH()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPAWNPOINTS_CANCEL_SEARCH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `spawnpoints_cancel_search_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn spawnpoints_cancel_search_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPAWNPOINTS_CANCEL_SEARCH()
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_dies_when_injured_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_DIES_WHEN_INJURED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_DIES_WHEN_INJURED(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_DIES_WHEN_INJURED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_dies_when_injured_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_dies_when_injured_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_DIES_WHEN_INJURED(ped, toggle)
}

/// ```
Somehow related to changing ped's clothes.  
```

pub fn clear_ped_blood_damage_by_zone_safe(ped: Ped, p1: i64) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_BLOOD_DAMAGE_BY_ZONE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_BLOOD_DAMAGE_BY_ZONE(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_BLOOD_DAMAGE_BY_ZONE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_blood_damage_by_zone_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_blood_damage_by_zone_raw(ped: i32, p1: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_BLOOD_DAMAGE_BY_ZONE(ped, p1)
}

/// ## Parameters
* **ped**:

pub fn is_ped_in_any_boat_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_ANY_BOAT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_ANY_BOAT(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_ANY_BOAT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_any_boat_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_any_boat_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_ANY_BOAT(ped)
}

/// ## Parameters
* **ped**:

pub fn clear_ped_stored_hat_prop_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_STORED_HAT_PROP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_STORED_HAT_PROP(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_STORED_HAT_PROP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_stored_hat_prop_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_stored_hat_prop_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_STORED_HAT_PROP(ped)
}

/// ```
Gets a value indicating whether the specified ped is on top of any vehicle.  
Return 1 when ped is on vehicle.  
Return 0 when ped is not on a vehicle.  
```

pub fn is_ped_on_vehicle_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_ON_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_ON_VEHICLE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_ON_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_on_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_on_vehicle_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_ON_VEHICLE(ped)
}

/// ## Parameters
* **ped**: 
* **heading**:

pub fn set_ped_desired_heading_safe(ped: Ped, heading: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_DESIRED_HEADING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_DESIRED_HEADING(ped, heading)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_DESIRED_HEADING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_desired_heading_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_desired_heading_raw(ped: i32, heading: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_DESIRED_HEADING(ped, heading)
}

/// ## Return value

pub fn can_create_random_bike_rider_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: CAN_CREATE_RANDOM_BIKE_RIDER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CAN_CREATE_RANDOM_BIKE_RIDER()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CAN_CREATE_RANDOM_BIKE_RIDER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `can_create_random_bike_rider_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn can_create_random_bike_rider_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CAN_CREATE_RANDOM_BIKE_RIDER()
}

/// ```
Presumably returns the Entity that the Ped is currently diving out of the way of.
var num3;
    if (PED::IS_PED_EVASIVE_DIVING(A_0, &num3) != 0)
        if (ENTITY::IS_ENTITY_A_VEHICLE(num3) != 0)
```

pub fn is_ped_evasive_diving_safe(ped: Ped, evadingEntity: *mut i32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_EVASIVE_DIVING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_EVASIVE_DIVING(ped, evadingEntity)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_EVASIVE_DIVING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_evasive_diving_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_evasive_diving_raw(ped: i32, evadingEntity: *mut i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_EVASIVE_DIVING(ped, evadingEntity)
}

/// ## Parameters
* **ped**:

pub fn disable_head_blend_palette_color_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: DISABLE_HEAD_BLEND_PALETTE_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DISABLE_HEAD_BLEND_PALETTE_COLOR(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DISABLE_HEAD_BLEND_PALETTE_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `disable_head_blend_palette_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn disable_head_blend_palette_color_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DISABLE_HEAD_BLEND_PALETTE_COLOR(ped)
}

/// ```
Can't select void. This function returns nothing. The hash of the created relationship group is output in the second parameter.  
```

pub fn add_relationship_group_safe(name: String, groupHash: *mut u32) -> NativeResult<serde_json::Value> {
    let name_cstr = std::ffi::CString::new(name.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "name", e)))?;
    
    debug!("Calling native function: ADD_RELATIONSHIP_GROUP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ADD_RELATIONSHIP_GROUP(crate::utils::rust_to_c_string(name), groupHash)
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: ADD_RELATIONSHIP_GROUP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `add_relationship_group_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn add_relationship_group_raw(name: *const std::os::raw::c_char, groupHash: *mut u32) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ADD_RELATIONSHIP_GROUP(name, groupHash)
}

/// ## Parameters
* **ped**:

pub fn is_ped_going_into_cover_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_GOING_INTO_COVER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_GOING_INTO_COVER(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_GOING_INTO_COVER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_going_into_cover_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_going_into_cover_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_GOING_INTO_COVER(ped)
}

/// ## Parameters
* **ped**:

pub fn release_ped_preload_variation_data_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: RELEASE_PED_PRELOAD_VARIATION_DATA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RELEASE_PED_PRELOAD_VARIATION_DATA(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RELEASE_PED_PRELOAD_VARIATION_DATA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `release_ped_preload_variation_data_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn release_ped_preload_variation_data_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RELEASE_PED_PRELOAD_VARIATION_DATA(ped)
}

/// See [`TASK_START_SCENARIO_IN_PLACE`](#_0x142A02425FF02BD9) for a list of scenarios.

pub fn is_ped_using_scenario_safe(ped: Ped, scenario: String) -> NativeResult<bool> {
    let scenario_cstr = std::ffi::CString::new(scenario.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "scenario", e)))?;
    
    debug!("Calling native function: IS_PED_USING_SCENARIO");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_USING_SCENARIO(ped, crate::utils::rust_to_c_string(scenario))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_USING_SCENARIO
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_using_scenario_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_using_scenario_raw(ped: i32, scenario: *const std::os::raw::c_char) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_USING_SCENARIO(ped, scenario)
}

/// ```
The distance between these points, is the diagonal of a box (remember it's 3D).  
```

pub fn set_ped_non_creation_area_safe(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_NON_CREATION_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_NON_CREATION_AREA(x1, y1, z1, x2, y2, z2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_NON_CREATION_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_non_creation_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_non_creation_area_raw(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_NON_CREATION_AREA(x1, y1, z1, x2, y2, z2)
}

/// ## Parameters
* **ped**: 
* **value**:

pub fn set_ped_max_move_blend_ratio_safe(ped: Ped, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_MAX_MOVE_BLEND_RATIO");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_MAX_MOVE_BLEND_RATIO(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_MAX_MOVE_BLEND_RATIO
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_max_move_blend_ratio_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_max_move_blend_ratio_raw(ped: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_MAX_MOVE_BLEND_RATIO(ped, value)
}

/// Returns whether the specified ped is in any vehicle. If `atGetIn` is set to true, also returns true if the ped is
currently in the process of entering a vehicle (a specific stage check for `CTaskEnterVehicle`).

pub fn is_ped_in_any_vehicle_safe(ped: Ped, atGetIn: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_ANY_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_ANY_VEHICLE(ped, atGetIn)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_ANY_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_any_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_any_vehicle_raw(ped: i32, atGetIn: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_ANY_VEHICLE(ped, atGetIn)
}

/// ```
NativeDB Introduced: v3407
```

pub fn is_ped_in_sphere_area_of_any_enemy_peds_safe(ped: Ped, x: f32, y: f32, z: f32, range: f32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_SPHERE_AREA_OF_ANY_ENEMY_PEDS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_SPHERE_AREA_OF_ANY_ENEMY_PEDS(ped, x, y, z, range)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_SPHERE_AREA_OF_ANY_ENEMY_PEDS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_sphere_area_of_any_enemy_peds_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_sphere_area_of_any_enemy_peds_raw(ped: i32, x: f32, y: f32, z: f32, range: f32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_SPHERE_AREA_OF_ANY_ENEMY_PEDS(ped, x, y, z, range)
}

/// ## Parameters
* **sceneID**: 
* **phase**:

pub fn set_synchronized_scene_phase_safe(sceneID: i64, phase: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SYNCHRONIZED_SCENE_PHASE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SYNCHRONIZED_SCENE_PHASE(sceneID, phase)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SYNCHRONIZED_SCENE_PHASE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_synchronized_scene_phase_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_synchronized_scene_phase_raw(sceneID: i64, phase: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SYNCHRONIZED_SCENE_PHASE(sceneID, phase)
}

/// ```
0 - Stationary (Will just stand in place)  
1 - Defensive (Will try to find cover and very likely to blind fire)  
2 - Offensive (Will attempt to charge at enemy but take cover as well)  
3 - Suicidal Offensive (Will try to flank enemy in a suicidal attack)  
```

pub fn set_ped_combat_movement_safe(ped: Ped, combatMovement: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_COMBAT_MOVEMENT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_COMBAT_MOVEMENT(ped, combatMovement)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_COMBAT_MOVEMENT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_combat_movement_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_combat_movement_raw(ped: i32, combatMovement: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_COMBAT_MOVEMENT(ped, combatMovement)
}

/// ## Parameters
* **ped**:

pub fn is_ped_in_any_train_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_ANY_TRAIN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_ANY_TRAIN(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_ANY_TRAIN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_any_train_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_any_train_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_ANY_TRAIN(ped)
}

/// ```
GET_*
```

pub fn _0x511f1a683387c7e2_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: _0x511F1A683387C7E2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x511F1A683387C7E2(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0x511F1A683387C7E2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x511f1a683387c7e2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x511f1a683387c7e2_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x511F1A683387C7E2(ped)
}

/// ## Parameters
* **asset**:

pub fn has_action_mode_asset_loaded_safe(asset: String) -> NativeResult<bool> {
    let asset_cstr = std::ffi::CString::new(asset.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "asset", e)))?;
    
    debug!("Calling native function: HAS_ACTION_MODE_ASSET_LOADED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_ACTION_MODE_ASSET_LOADED(crate::utils::rust_to_c_string(asset))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_ACTION_MODE_ASSET_LOADED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_action_mode_asset_loaded_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_action_mode_asset_loaded_raw(asset: *const std::os::raw::c_char) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_ACTION_MODE_ASSET_LOADED(asset)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**: 
* **interiorFlags**: 
* **scale**: 
* **duration**:

pub fn spawnpoints_start_search_safe(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, interiorFlags: i64, scale: f32, duration: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SPAWNPOINTS_START_SEARCH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPAWNPOINTS_START_SEARCH(p0, p1, p2, p3, p4, interiorFlags, scale, duration)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SPAWNPOINTS_START_SEARCH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `spawnpoints_start_search_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn spawnpoints_start_search_raw(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, interiorFlags: i64, scale: f32, duration: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPAWNPOINTS_START_SEARCH(p0, p1, p2, p3, p4, interiorFlags, scale, duration)
}

/// ## Parameters
* **ped**:

pub fn get_ped_helmet_stored_hat_tex_index_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_HELMET_STORED_HAT_TEX_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_HELMET_STORED_HAT_TEX_INDEX(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_HELMET_STORED_HAT_TEX_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_helmet_stored_hat_tex_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_helmet_stored_hat_tex_index_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_HELMET_STORED_HAT_TEX_INDEX(ped)
}

/// ```
Works for both player and peds, but some flags don't seem to work for the player (1, for example)  
1 - Blocks ragdolling when shot.  
2 - Blocks ragdolling when hit by a vehicle. The ped still might play a falling animation.  
4 - Blocks ragdolling when set on fire.  
-----------------------------------------------------------------------  
There seem to be 26 flags  
```

pub fn set_ragdoll_blocking_flags_safe(ped: Ped, flags: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_RAGDOLL_BLOCKING_FLAGS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_RAGDOLL_BLOCKING_FLAGS(ped, flags)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_RAGDOLL_BLOCKING_FLAGS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ragdoll_blocking_flags_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ragdoll_blocking_flags_raw(ped: i32, flags: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_RAGDOLL_BLOCKING_FLAGS(ped, flags)
}

/// ## Parameters
* **ped**: 
* **hash**:

pub fn set_ped_relationship_group_hash_safe(ped: Ped, hash: u32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_RELATIONSHIP_GROUP_HASH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_RELATIONSHIP_GROUP_HASH(ped, hash)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_RELATIONSHIP_GROUP_HASH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_relationship_group_hash_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_relationship_group_hash_raw(ped: i32, hash: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_RELATIONSHIP_GROUP_HASH(ped, hash)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_force_footstep_update_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_FORCE_FOOTSTEP_UPDATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_FORCE_FOOTSTEP_UPDATE(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_FORCE_FOOTSTEP_UPDATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_force_footstep_update_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_force_footstep_update_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_FORCE_FOOTSTEP_UPDATE(ped, toggle)
}

/// ```
Sets a value indicating whether scenario peds should be returned by the next call to a command that returns peds. Eg. GET_CLOSEST_PED.  
```

pub fn set_scenario_peds_to_be_returned_by_next_command_safe(value: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SCENARIO_PEDS_TO_BE_RETURNED_BY_NEXT_COMMAND");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SCENARIO_PEDS_TO_BE_RETURNED_BY_NEXT_COMMAND(value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SCENARIO_PEDS_TO_BE_RETURNED_BY_NEXT_COMMAND
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_scenario_peds_to_be_returned_by_next_command_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_scenario_peds_to_be_returned_by_next_command_raw(value: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SCENARIO_PEDS_TO_BE_RETURNED_BY_NEXT_COMMAND(value)
}

/// ```
Returns true if the given ped has a valid pointer to CPlayerInfo in its CPed class. That's all.
```

pub fn is_ped_a_player_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_A_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_A_PLAYER(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_A_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_a_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_a_player_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_A_PLAYER(ped)
}

/// ```
p2 usually 0  
```

pub fn set_ped_can_play_viseme_anims_safe(ped: Ped, toggle: bool, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_PLAY_VISEME_ANIMS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_PLAY_VISEME_ANIMS(ped, toggle, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_PLAY_VISEME_ANIMS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_play_viseme_anims_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_play_viseme_anims_raw(ped: i32, toggle: bool, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_PLAY_VISEME_ANIMS(ped, toggle, p2)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_be_shot_in_vehicle_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_BE_SHOT_IN_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_BE_SHOT_IN_VEHICLE(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_BE_SHOT_IN_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_be_shot_in_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_be_shot_in_vehicle_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_BE_SHOT_IN_VEHICLE(ped, toggle)
}

/// ## Parameters
* **ped**:

pub fn has_ped_head_blend_finished_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_PED_HEAD_BLEND_FINISHED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_PED_HEAD_BLEND_FINISHED(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_PED_HEAD_BLEND_FINISHED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_ped_head_blend_finished_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_ped_head_blend_finished_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_PED_HEAD_BLEND_FINISHED(ped)
}

/// ```
Returns whether the specified ped is shooting.  
```

pub fn is_ped_shooting_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_SHOOTING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_SHOOTING(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_SHOOTING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_shooting_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_shooting_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_SHOOTING(ped)
}

/// SET_PED_MAX_TIME_UNDERWATER native function

pub fn set_ped_max_time_underwater_safe(ped: Ped, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_MAX_TIME_UNDERWATER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_MAX_TIME_UNDERWATER(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_MAX_TIME_UNDERWATER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_max_time_underwater_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_max_time_underwater_raw(ped: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_MAX_TIME_UNDERWATER(ped, value)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_arm_ik_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_ARM_IK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_ARM_IK(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_ARM_IK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_arm_ik_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_arm_ik_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_ARM_IK(ped, toggle)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_as_enemy_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_AS_ENEMY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_AS_ENEMY(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_AS_ENEMY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_as_enemy_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_as_enemy_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_AS_ENEMY(ped, toggle)
}

/// ## Parameters
* **ped**: 
* **attachPed**: 
* **p2**: 
* **p3**: 
* **p4**: 
* **p5**: 
* **p6**: 
* **p7**: 
* **p8**: 
* **p9**: 
* **p10**:

pub fn set_ped_defensive_area_attached_to_ped_safe(ped: Ped, attachPed: Ped, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: f32, p9: bool, p10: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_DEFENSIVE_AREA_ATTACHED_TO_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_DEFENSIVE_AREA_ATTACHED_TO_PED(ped, attachPed, p2, p3, p4, p5, p6, p7, p8, p9, p10)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_DEFENSIVE_AREA_ATTACHED_TO_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_defensive_area_attached_to_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_defensive_area_attached_to_ped_raw(ped: i32, attachPed: i32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: f32, p9: bool, p10: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_DEFENSIVE_AREA_ATTACHED_TO_PED(ped, attachPed, p2, p3, p4, p5, p6, p7, p8, p9, p10)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn _0x9a77dfd295e29b09_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x9A77DFD295E29B09");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x9A77DFD295E29B09(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x9A77DFD295E29B09
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x9a77dfd295e29b09_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x9a77dfd295e29b09_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x9A77DFD295E29B09(ped, toggle)
}

/// ## Parameters
* **group**: 
* **p1**:

pub fn _set_relationship_group_dont_affect_wanted_level_safe() -> NativeResult<()> {
    
    debug!("Calling native function: _SET_RELATIONSHIP_GROUP_DONT_AFFECT_WANTED_LEVEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_RELATIONSHIP_GROUP_DONT_AFFECT_WANTED_LEVEL()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_RELATIONSHIP_GROUP_DONT_AFFECT_WANTED_LEVEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_relationship_group_dont_affect_wanted_level_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_relationship_group_dont_affect_wanted_level_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_RELATIONSHIP_GROUP_DONT_AFFECT_WANTED_LEVEL()
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn _block_ped_dead_body_shocking_events_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _BLOCK_PED_DEAD_BODY_SHOCKING_EVENTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_BLOCK_PED_DEAD_BODY_SHOCKING_EVENTS(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _BLOCK_PED_DEAD_BODY_SHOCKING_EVENTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_block_ped_dead_body_shocking_events_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _block_ped_dead_body_shocking_events_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_BLOCK_PED_DEAD_BODY_SHOCKING_EVENTS(ped, toggle)
}

/// ## Parameters
* **ped**: 
* **combatType**: A value between 0-14, See [`GET_COMBAT_FLOAT`](#_0x52DFF8A10508090A) for a list of possible parameters.
* **p2**:

pub fn set_combat_float_safe(ped: Ped, combatType: i64, p2: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_COMBAT_FLOAT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_COMBAT_FLOAT(ped, combatType, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_COMBAT_FLOAT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_combat_float_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_combat_float_raw(ped: i32, combatType: i64, p2: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_COMBAT_FLOAT(ped, combatType, p2)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**:

pub fn _0x0f62619393661d6e_safe(p0: serde_json::Value, p1: serde_json::Value, p2: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x0F62619393661D6E");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x0F62619393661D6E(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1), crate::utils::any_to_c_void_ptr(p2))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x0F62619393661D6E
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x0f62619393661d6e_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x0f62619393661d6e_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void, p2: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x0F62619393661D6E(p0, p1, p2)
}

/// ## Parameters
* **ped**:

pub fn set_ped_move_anims_blend_out_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_MOVE_ANIMS_BLEND_OUT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_MOVE_ANIMS_BLEND_OUT(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_MOVE_ANIMS_BLEND_OUT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_move_anims_blend_out_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_move_anims_blend_out_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_MOVE_ANIMS_BLEND_OUT(ped)
}

/// If the ped is attempting to enter a locked vehicle.

pub fn is_ped_trying_to_enter_a_locked_vehicle_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_TRYING_TO_ENTER_A_LOCKED_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_TRYING_TO_ENTER_A_LOCKED_VEHICLE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_TRYING_TO_ENTER_A_LOCKED_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_trying_to_enter_a_locked_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_trying_to_enter_a_locked_vehicle_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_TRYING_TO_ENTER_A_LOCKED_VEHICLE(ped)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn _0x9c6a6c19b6c0c496_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: _0x9C6A6C19B6C0C496");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x9C6A6C19B6C0C496(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0x9C6A6C19B6C0C496
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x9c6a6c19b6c0c496_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x9c6a6c19b6c0c496_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x9C6A6C19B6C0C496(ped)
}

/// Sets the tint index for the hair on the specified ped.

```
NativeDB Introduced: v323
```

pub fn set_ped_hair_tint_safe(ped: Ped, colorID: i64, highlightColorID: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_HAIR_TINT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_HAIR_TINT(ped, colorID, highlightColorID)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_HAIR_TINT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_hair_tint_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_hair_tint_raw(ped: i32, colorID: i64, highlightColorID: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_HAIR_TINT(ped, colorID, highlightColorID)
}

/// ```
NativeDB Introduced: v1180
```

pub fn _0xdfe68c4b787e1bfb_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: _0xDFE68C4B787E1BFB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xDFE68C4B787E1BFB(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xDFE68C4B787E1BFB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xdfe68c4b787e1bfb_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xdfe68c4b787e1bfb_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xDFE68C4B787E1BFB(ped)
}

/// Sets the IsHandCuffed (120) config flag on the ped. This blocks the ped from switching weapons (with the exception of switching to `weapon_unarmed`), makes the ped ragdoll on getting punched and forces a different get-up animation after ragdolling. The ped can also not vault over or climb on top of objects.

Used in combination with [SET_ENABLE_BOUND_ANKLES](#_0xC52E0F855C58FC2E) in decompiled scripts.

pub fn set_enable_handcuffs_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_ENABLE_HANDCUFFS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_ENABLE_HANDCUFFS(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_ENABLE_HANDCUFFS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_enable_handcuffs_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_enable_handcuffs_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_ENABLE_HANDCUFFS(ped, toggle)
}

/// ## Parameters
* **ped**:

pub fn get_ped_decorations_state_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_DECORATIONS_STATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_DECORATIONS_STATE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_DECORATIONS_STATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_decorations_state_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_decorations_state_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_DECORATIONS_STATE(ped)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_enable_weapon_blocking_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_ENABLE_WEAPON_BLOCKING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_ENABLE_WEAPON_BLOCKING(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_ENABLE_WEAPON_BLOCKING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_enable_weapon_blocking_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_enable_weapon_blocking_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_ENABLE_WEAPON_BLOCKING(ped, toggle)
}

/// ```
List of movement clipsets:
Thanks to elsewhat for list.
 "ANIM_GROUP_MOVE_BALLISTIC"
 "ANIM_GROUP_MOVE_LEMAR_ALLEY"
 "clipset@move@trash_fast_turn"
 "FEMALE_FAST_RUNNER"
 "missfbi4prepp1_garbageman"
 "move_characters@franklin@fire"
 "move_characters@Jimmy@slow@"
 "move_characters@michael@fire"
 "move_f@flee@a"
 "move_f@scared"
 "move_f@sexy@a"
 "move_heist_lester"
 "move_injured_generic"
 "move_lester_CaneUp"
 "move_m@bag"
 "MOVE_M@BAIL_BOND_NOT_TAZERED"
 "MOVE_M@BAIL_BOND_TAZERED"
 "move_m@brave"
 "move_m@casual@d"
 "move_m@drunk@moderatedrunk"
 "MOVE_M@DRUNK@MODERATEDRUNK"
 "MOVE_M@DRUNK@MODERATEDRUNK_HEAD_UP"
 "MOVE_M@DRUNK@SLIGHTLYDRUNK"
 "MOVE_M@DRUNK@VERYDRUNK"
 "move_m@fire"
 "move_m@gangster@var_e"
 "move_m@gangster@var_f"
 "move_m@gangster@var_i"
 "move_m@JOG@"
 "MOVE_M@PRISON_GAURD"
 "MOVE_P_M_ONE"
 "MOVE_P_M_ONE_BRIEFCASE"
 "move_p_m_zero_janitor"
 "move_p_m_zero_slow"
 "move_ped_bucket"
 "move_ped_crouched"
 "move_ped_mop"
 "MOVE_M@FEMME@"
 "MOVE_F@FEMME@"
 "MOVE_M@GANGSTER@NG"
 "MOVE_F@GANGSTER@NG"
 "MOVE_M@POSH@"
 "MOVE_F@POSH@"
 "MOVE_M@TOUGH_GUY@"
 "MOVE_F@TOUGH_GUY@"
~ NotCrunchyTaco
```

pub fn set_ped_movement_clipset_safe(ped: Ped, clipSet: String, transitionSpeed: f32) -> NativeResult<()> {
    let clipSet_cstr = std::ffi::CString::new(clipSet.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "clipSet", e)))?;
    
    debug!("Calling native function: SET_PED_MOVEMENT_CLIPSET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_MOVEMENT_CLIPSET(ped, crate::utils::rust_to_c_string(clipSet), transitionSpeed)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_MOVEMENT_CLIPSET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_movement_clipset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_movement_clipset_raw(ped: i32, clipSet: *const std::os::raw::c_char, transitionSpeed: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_MOVEMENT_CLIPSET(ped, clipSet, transitionSpeed)
}

/// ```
Min and max are usually 100.0 and 200.0
```

pub fn set_pop_control_sphere_this_frame_safe(x: f32, y: f32, z: f32, min: f32, max: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_POP_CONTROL_SPHERE_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_POP_CONTROL_SPHERE_THIS_FRAME(x, y, z, min, max)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_POP_CONTROL_SPHERE_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_pop_control_sphere_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_pop_control_sphere_this_frame_raw(x: f32, y: f32, z: f32, min: f32, max: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_POP_CONTROL_SPHERE_THIS_FRAME(x, y, z, min, max)
}

/// ## Parameters
* **ped**: 
* **animName**: 
* **animDict**:

pub fn play_facial_anim_safe(ped: Ped, animName: String, animDict: String) -> NativeResult<()> {
    let animName_cstr = std::ffi::CString::new(animName.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animName", e)))?;
    let animDict_cstr = std::ffi::CString::new(animDict.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "animDict", e)))?;
    
    debug!("Calling native function: PLAY_FACIAL_ANIM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::PLAY_FACIAL_ANIM(ped, crate::utils::rust_to_c_string(animName), crate::utils::rust_to_c_string(animDict))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: PLAY_FACIAL_ANIM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `play_facial_anim_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn play_facial_anim_raw(ped: i32, animName: *const std::os::raw::c_char, animDict: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::PLAY_FACIAL_ANIM(ped, animName, animDict)
}

/// ## Parameters
* **ped**:

pub fn get_ped_accuracy_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_ACCURACY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_ACCURACY(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_ACCURACY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_accuracy_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_accuracy_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_ACCURACY(ped)
}

/// ## Parameters
* **groupId**:

pub fn remove_group_safe(groupId: i64) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_GROUP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_GROUP(groupId)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_GROUP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_group_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_group_raw(groupId: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_GROUP(groupId)
}

/// ## Parameters
* **ped**:

pub fn get_melee_target_for_ped_safe(ped: Ped) -> NativeResult<Ped> {
    
    debug!("Calling native function: GET_MELEE_TARGET_FOR_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_MELEE_TARGET_FOR_PED(ped)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: GET_MELEE_TARGET_FOR_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_melee_target_for_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_melee_target_for_ped_raw(ped: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_MELEE_TARGET_FOR_PED(ped)
}

/// ```
from fm_mission_controller.c4 (variable names changed for clarity):  
int groupID = PLAYER::GET_PLAYER_GROUP(PLAYER::PLAYER_ID());  
PED::GET_GROUP_SIZE(group, &unused, &groupSize);  
if (groupSize >= 1) {  
. . . . for (int memberNumber = 0; memberNumber < groupSize; memberNumber++) {  
. . . . . . . . Ped ped1 = PED::GET_PED_AS_GROUP_MEMBER(groupID, memberNumber);  
. . . . . . . . //and so on  
```

pub fn get_ped_as_group_member_safe(groupID: i64, memberNumber: i64) -> NativeResult<Ped> {
    
    debug!("Calling native function: GET_PED_AS_GROUP_MEMBER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_AS_GROUP_MEMBER(groupID, memberNumber)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: GET_PED_AS_GROUP_MEMBER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_as_group_member_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_as_group_member_raw(groupID: i64, memberNumber: i64) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_AS_GROUP_MEMBER(groupID, memberNumber)
}

/// ## Parameters
* **ped**:

pub fn _0x733c87d4ce22bea2_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: _0x733C87D4CE22BEA2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x733C87D4CE22BEA2(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x733C87D4CE22BEA2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x733c87d4ce22bea2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x733c87d4ce22bea2_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x733C87D4CE22BEA2(ped)
}

/// ## Parameters
* **x**: 
* **y**: 
* **z**: 
* **range**: 
* **p4**:

pub fn set_scenario_peds_spawn_in_sphere_area_safe(x: f32, y: f32, z: f32, range: f32, p4: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SCENARIO_PEDS_SPAWN_IN_SPHERE_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SCENARIO_PEDS_SPAWN_IN_SPHERE_AREA(x, y, z, range, p4)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SCENARIO_PEDS_SPAWN_IN_SPHERE_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_scenario_peds_spawn_in_sphere_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_scenario_peds_spawn_in_sphere_area_raw(x: f32, y: f32, z: f32, range: f32, p4: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SCENARIO_PEDS_SPAWN_IN_SPHERE_AREA(x, y, z, range, p4)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_peek_in_cover_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_PEEK_IN_COVER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_PEEK_IN_COVER(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_PEEK_IN_COVER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_peek_in_cover_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_peek_in_cover_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_PEEK_IN_COVER(ped, toggle)
}

/// ## Parameters
* **ped**:

pub fn is_ped_running_mobile_phone_task_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_RUNNING_MOBILE_PHONE_TASK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_RUNNING_MOBILE_PHONE_TASK(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_RUNNING_MOBILE_PHONE_TASK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_running_mobile_phone_task_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_running_mobile_phone_task_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_RUNNING_MOBILE_PHONE_TASK(ped)
}

/// ## Parameters
* **sceneID**:

pub fn is_synchronized_scene_hold_last_frame_safe(sceneID: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_SYNCHRONIZED_SCENE_HOLD_LAST_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_SYNCHRONIZED_SCENE_HOLD_LAST_FRAME(sceneID)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_SYNCHRONIZED_SCENE_HOLD_LAST_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_synchronized_scene_hold_last_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_synchronized_scene_hold_last_frame_raw(sceneID: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_SYNCHRONIZED_SCENE_HOLD_LAST_FRAME(sceneID)
}

/// ## Parameters
* **ped**:

pub fn is_ped_climbing_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_CLIMBING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_CLIMBING(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_CLIMBING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_climbing_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_climbing_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_CLIMBING(ped)
}

/// ## Parameters
* **ped**: 
* **target**: 
* **xOffset**: 
* **yOffset**: 
* **zOffset**: 
* **radius**: 
* **p6**:

pub fn set_ped_defensive_sphere_attached_to_ped_safe(ped: Ped, target: Ped, xOffset: f32, yOffset: f32, zOffset: f32, radius: f32, p6: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_DEFENSIVE_SPHERE_ATTACHED_TO_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_DEFENSIVE_SPHERE_ATTACHED_TO_PED(ped, target, xOffset, yOffset, zOffset, radius, p6)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_DEFENSIVE_SPHERE_ATTACHED_TO_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_defensive_sphere_attached_to_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_defensive_sphere_attached_to_ped_raw(ped: i32, target: i32, xOffset: f32, yOffset: f32, zOffset: f32, radius: f32, p6: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_DEFENSIVE_SPHERE_ATTACHED_TO_PED(ped, target, xOffset, yOffset, zOffset, radius, p6)
}

/// SET_PED_RAGDOLL_ON_COLLISION native function

pub fn set_ped_ragdoll_on_collision_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_RAGDOLL_ON_COLLISION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_RAGDOLL_ON_COLLISION(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_RAGDOLL_ON_COLLISION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_ragdoll_on_collision_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_ragdoll_on_collision_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_RAGDOLL_ON_COLLISION(ped, toggle)
}

/// ```
Detect if ped is sitting in the specified vehicle  
[True/False]  
```

pub fn is_ped_sitting_in_vehicle_safe(ped: Ped, vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_SITTING_IN_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_SITTING_IN_VEHICLE(ped, vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_SITTING_IN_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_sitting_in_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_sitting_in_vehicle_raw(ped: i32, vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_SITTING_IN_VEHICLE(ped, vehicle)
}

/// ```
It adds the wetness level to the player clothing/outfit. As if player just got out from water surface.  
```

pub fn set_ped_wetness_height_safe(ped: Ped, height: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_WETNESS_HEIGHT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_WETNESS_HEIGHT(ped, height)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_WETNESS_HEIGHT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_wetness_height_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_wetness_height_raw(ped: i32, height: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_WETNESS_HEIGHT(ped, height)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **p2**: 
* **p3**:

pub fn _set_ped_helmet_unk_safe(ped: Ped, p1: bool, p2: i64, p3: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PED_HELMET_UNK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PED_HELMET_UNK(ped, p1, p2, p3)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PED_HELMET_UNK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_ped_helmet_unk_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_ped_helmet_unk_raw(ped: i32, p1: bool, p2: i64, p3: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PED_HELMET_UNK(ped, p1, p2, p3)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_steers_around_objects_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_STEERS_AROUND_OBJECTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_STEERS_AROUND_OBJECTS(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_STEERS_AROUND_OBJECTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_steers_around_objects_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_steers_around_objects_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_STEERS_AROUND_OBJECTS(ped, toggle)
}

/// ## Parameters
* **ped**:

pub fn get_jack_target_safe(ped: Ped) -> NativeResult<Ped> {
    
    debug!("Calling native function: GET_JACK_TARGET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_JACK_TARGET(ped)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: GET_JACK_TARGET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_jack_target_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_jack_target_raw(ped: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_JACK_TARGET(ped)
}

/// ## Parameters
* **ped**:

pub fn is_ped_using_action_mode_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_USING_ACTION_MODE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_USING_ACTION_MODE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_USING_ACTION_MODE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_using_action_mode_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_using_action_mode_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_USING_ACTION_MODE(ped)
}

/// ```
NativeDB Introduced: v2372
```

pub fn _get_ped_task_combat_target_safe(ped: Ped, p1: serde_json::Value) -> NativeResult<Entity> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p1_any_str = serde_json::to_string(&p1)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p1", e)))?;
    let p1_any_str_cstr = std::ffi::CString::new(p1_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p1", e)))?;
    
    debug!("Calling native function: _GET_PED_TASK_COMBAT_TARGET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_PED_TASK_COMBAT_TARGET(ped, crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(Entity(result)))
}

/// Raw native function: _GET_PED_TASK_COMBAT_TARGET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_ped_task_combat_target_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_ped_task_combat_target_raw(ped: i32, p1: *mut std::os::raw::c_void) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_PED_TASK_COMBAT_TARGET(ped, p1)
}

/// ## Parameters
* **modelHash**: 
* **p1**: 
* **p2**:

pub fn _0xc56fbf2f228e1dac_safe(modelHash: u32, p1: serde_json::Value, p2: serde_json::Value) -> NativeResult<serde_json::Value> {
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
    
    debug!("Calling native function: _0xC56FBF2F228E1DAC");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xC56FBF2F228E1DAC(modelHash, crate::utils::any_to_c_void_ptr(p1), crate::utils::any_to_c_void_ptr(p2))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0xC56FBF2F228E1DAC
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xc56fbf2f228e1dac_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xc56fbf2f228e1dac_raw(modelHash: u32, p1: *mut std::os::raw::c_void, p2: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xC56FBF2F228E1DAC(modelHash, p1, p2)
}

/// ```
p1 may be a BOOL representing whether or not the group even exists  
```

pub fn get_group_size_safe(groupID: i64, unknown: serde_json::Value, sizeInMembers: *mut i64) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let unknown_any_str = serde_json::to_string(&unknown)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "unknown", e)))?;
    let unknown_any_str_cstr = std::ffi::CString::new(unknown_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "unknown", e)))?;
    
    debug!("Calling native function: GET_GROUP_SIZE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_GROUP_SIZE(groupID, crate::utils::any_to_c_void_ptr(unknown), sizeInMembers)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_GROUP_SIZE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_group_size_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_group_size_raw(groupID: i64, unknown: *mut std::os::raw::c_void, sizeInMembers: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_GROUP_SIZE(groupID, unknown, sizeInMembers)
}

/// ```
Copies ped's components and props to targetPed.
```

pub fn clone_ped_to_target_safe(ped: Ped, targetPed: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: CLONE_PED_TO_TARGET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLONE_PED_TO_TARGET(ped, targetPed)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLONE_PED_TO_TARGET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clone_ped_to_target_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clone_ped_to_target_raw(ped: i32, targetPed: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLONE_PED_TO_TARGET(ped, targetPed)
}

/// ## Parameters
* **ped**:

pub fn is_ped_in_cover_facing_left_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_COVER_FACING_LEFT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_COVER_FACING_LEFT(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_COVER_FACING_LEFT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_cover_facing_left_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_cover_facing_left_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_COVER_FACING_LEFT(ped)
}

/// ## Parameters
* **ped**: The ped handle.
* **componentId**: The component id to get the prop texture index from. Refer to [SET_PED_COMPONENT_VARIATION](#_0x262B14F48D29DE80).

pub fn get_ped_prop_texture_index_safe(ped: Ped, componentId: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_PROP_TEXTURE_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_PROP_TEXTURE_INDEX(ped, componentId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_PROP_TEXTURE_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_prop_texture_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_prop_texture_index_raw(ped: i32, componentId: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_PROP_TEXTURE_INDEX(ped, componentId)
}

/// ## Parameters
* **sceneID**: 
* **x**: 
* **y**: 
* **z**: 
* **roll**: 
* **pitch**: 
* **yaw**: 
* **p7**:

pub fn set_synchronized_scene_origin_safe(sceneID: i64, x: f32, y: f32, z: f32, roll: f32, pitch: f32, yaw: f32, p7: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SYNCHRONIZED_SCENE_ORIGIN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SYNCHRONIZED_SCENE_ORIGIN(sceneID, x, y, z, roll, pitch, yaw, p7)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SYNCHRONIZED_SCENE_ORIGIN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_synchronized_scene_origin_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_synchronized_scene_origin_raw(sceneID: i64, x: f32, y: f32, z: f32, roll: f32, pitch: f32, yaw: f32, p7: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SYNCHRONIZED_SCENE_ORIGIN(sceneID, x, y, z, roll, pitch, yaw, p7)
}

/// ## Parameters
* **ped**:

pub fn remove_ped_from_group_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_PED_FROM_GROUP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_PED_FROM_GROUP(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_PED_FROM_GROUP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_ped_from_group_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_ped_from_group_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_PED_FROM_GROUP(ped)
}

/// ```
Only 1 and 2 appear in the scripts. combatbehaviour.meta seems to only have TLR_SearchForTarget for all peds, but we don't know if that's 1 or 2.  
```

pub fn set_ped_target_loss_response_safe(ped: Ped, responseType: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_TARGET_LOSS_RESPONSE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_TARGET_LOSS_RESPONSE(ped, responseType)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_TARGET_LOSS_RESPONSE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_target_loss_response_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_target_loss_response_raw(ped: i32, responseType: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_TARGET_LOSS_RESPONSE(ped, responseType)
}

/// Ped types:

```cpp
enum ePedType
{
	PED_TYPE_PLAYER_0 = 0,
	PED_TYPE_PLAYER_1 = 1,
	PED_TYPE_NETWORK_PLAYER = 2,
	PED_TYPE_PLAYER_2 = 3,
	PED_TYPE_CIVMALE = 4,
	PED_TYPE_CIVFEMALE = 5,
	PED_TYPE_COP = 6,
	PED_TYPE_GANG_ALBANIAN = 7,
	PED_TYPE_GANG_BIKER_1 = 8,
	PED_TYPE_GANG_BIKER_2 = 9,
	PED_TYPE_GANG_ITALIAN = 10,
	PED_TYPE_GANG_RUSSIAN = 11,
	PED_TYPE_GANG_RUSSIAN_2 = 12,
	PED_TYPE_GANG_IRISH = 13,
	PED_TYPE_GANG_JAMAICAN = 14,
	PED_TYPE_GANG_AFRICAN_AMERICAN = 15,
	PED_TYPE_GANG_KOREAN = 16,
	PED_TYPE_GANG_CHINESE_JAPANESE = 17,
	PED_TYPE_GANG_PUERTO_RICAN = 18,
	PED_TYPE_DEALER = 19,
	PED_TYPE_MEDIC = 20,
	PED_TYPE_FIREMAN = 21,
	PED_TYPE_CRIMINAL = 22,
	PED_TYPE_BUM = 23,
	PED_TYPE_PROSTITUTE = 24,
	PED_TYPE_SPECIAL = 25,
	PED_TYPE_MISSION = 26,
	PED_TYPE_SWAT = 27,
	PED_TYPE_ANIMAL = 28,
	PED_TYPE_ARMY = 29
};
```

pub fn get_ped_type_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_TYPE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_TYPE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_TYPE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_type_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_type_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_TYPE(ped)
}

/// ## Parameters
* **colorID**:

pub fn _is_ped_blush_color_valid_safe(colorID: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PED_BLUSH_COLOR_VALID");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PED_BLUSH_COLOR_VALID(colorID)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PED_BLUSH_COLOR_VALID
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_ped_blush_color_valid_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_ped_blush_color_valid_raw(colorID: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PED_BLUSH_COLOR_VALID(colorID)
}

/// Indicates whether this ped's health is below its injured threshold.
The default threshold is 100, these are stored in the `pedhealth.meta` file located in `common:\data\`

pub fn is_ped_injured_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_INJURED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_INJURED(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_INJURED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_injured_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_injured_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_INJURED(ped)
}

/// ## Parameters
* **ped**:

pub fn clear_all_ped_vehicle_forced_seat_usage_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_ALL_PED_VEHICLE_FORCED_SEAT_USAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_ALL_PED_VEHICLE_FORCED_SEAT_USAGE(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_ALL_PED_VEHICLE_FORCED_SEAT_USAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_all_ped_vehicle_forced_seat_usage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_all_ped_vehicle_forced_seat_usage_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_ALL_PED_VEHICLE_FORCED_SEAT_USAGE(ped)
}

/// ```
Returns the ped's alertness (0-3).  
Values :   
0 : Neutral  
1 : Heard something (gun shot, hit, etc)  
2 : Knows (the origin of the event)  
3 : Fully alerted (is facing the event?)  
If the Ped does not exist, returns -1.  
```

pub fn get_ped_alertness_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_ALERTNESS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_ALERTNESS(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_ALERTNESS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_alertness_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_alertness_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_ALERTNESS(ped)
}

/// ```
Returns true if the ped doesn't do any movement. If the ped is being pushed forwards by using APPLY_FORCE_TO_ENTITY for example, the function returns false.  
```

pub fn is_ped_stopped_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_STOPPED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_STOPPED(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_STOPPED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_stopped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_stopped_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_STOPPED(ped)
}

/// ```
"IK" stands for "Inverse kinematics." I assume this has something to do with how the ped uses his legs to balance. In the scripts, the second parameter is always an int with a value of 2, 0, or sometimes 1  
```

pub fn set_ped_leg_ik_mode_safe(ped: Ped, mode: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_LEG_IK_MODE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_LEG_IK_MODE(ped, mode)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_LEG_IK_MODE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_leg_ik_mode_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_leg_ik_mode_raw(ped: i32, mode: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_LEG_IK_MODE(ped, mode)
}

/// ```
Returns whether the specified ped is hurt.  
```

pub fn is_ped_hurt_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_HURT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_HURT(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_HURT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_hurt_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_hurt_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_HURT(ped)
}

/// ## Parameters
* **ped**:

pub fn can_knock_ped_off_vehicle_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: CAN_KNOCK_PED_OFF_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CAN_KNOCK_PED_OFF_VEHICLE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CAN_KNOCK_PED_OFF_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `can_knock_ped_off_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn can_knock_ped_off_vehicle_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CAN_KNOCK_PED_OFF_VEHICLE(ped)
}

/// ## Parameters
* **ped**:

pub fn reset_ped_ragdoll_timer_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: RESET_PED_RAGDOLL_TIMER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_PED_RAGDOLL_TIMER(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_PED_RAGDOLL_TIMER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_ped_ragdoll_timer_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_ped_ragdoll_timer_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_PED_RAGDOLL_TIMER(ped)
}

/// ## Parameters
* **groupHash**:

pub fn _does_relationship_group_exist_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: _DOES_RELATIONSHIP_GROUP_EXIST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_DOES_RELATIONSHIP_GROUP_EXIST()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _DOES_RELATIONSHIP_GROUP_EXIST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_does_relationship_group_exist_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _does_relationship_group_exist_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_DOES_RELATIONSHIP_GROUP_EXIST()
}

/// ```
value ranges from 0 to 3.  
```

pub fn set_ped_alertness_safe(ped: Ped, value: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_ALERTNESS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_ALERTNESS(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_ALERTNESS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_alertness_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_alertness_raw(ped: i32, value: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_ALERTNESS(ped, value)
}

/// ```
Damage Packs:  
"SCR_TrevorTreeBang"  
"HOSPITAL_0"  
"HOSPITAL_1"  
"HOSPITAL_2"  
"HOSPITAL_3"  
"HOSPITAL_4"  
"HOSPITAL_5"  
"HOSPITAL_6"  
"HOSPITAL_7"  
"HOSPITAL_8"  
"HOSPITAL_9"  
"SCR_Dumpster"  
"BigHitByVehicle"  
"SCR_Finale_Michael_Face"  
"SCR_Franklin_finb"  
"SCR_Finale_Michael"  
"SCR_Franklin_finb2"  
"Explosion_Med"  
"SCR_Torture"  
"SCR_TracySplash"  
"Skin_Melee_0"  
Additional damage packs:  
gist.github.com/alexguirre/f3f47f75ddcf617f416f3c8a55ae2227  
```

pub fn apply_ped_damage_pack_safe(ped: Ped, damagePack: String, damage: f32, mult: f32) -> NativeResult<()> {
    let damagePack_cstr = std::ffi::CString::new(damagePack.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "damagePack", e)))?;
    
    debug!("Calling native function: APPLY_PED_DAMAGE_PACK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::APPLY_PED_DAMAGE_PACK(ped, crate::utils::rust_to_c_string(damagePack), damage, mult)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: APPLY_PED_DAMAGE_PACK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `apply_ped_damage_pack_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn apply_ped_damage_pack_raw(ped: i32, damagePack: *const std::os::raw::c_char, damage: f32, mult: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::APPLY_PED_DAMAGE_PACK(ped, damagePack, damage, mult)
}

/// ## Parameters
* **ped**: 
* **textureIndex**:

pub fn set_ped_helmet_texture_index_safe(ped: Ped, textureIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_HELMET_TEXTURE_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_HELMET_TEXTURE_INDEX(ped, textureIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_HELMET_TEXTURE_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_helmet_texture_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_helmet_texture_index_raw(ped: i32, textureIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_HELMET_TEXTURE_INDEX(ped, textureIndex)
}

/// ```
SET_PED_*
```

pub fn _0xafc976fd0580c7b3_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xAFC976FD0580C7B3");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xAFC976FD0580C7B3(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xAFC976FD0580C7B3
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xafc976fd0580c7b3_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xafc976fd0580c7b3_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xAFC976FD0580C7B3(ped, toggle)
}

/// ## Parameters
* **ped**: The ped handle.
* **propId**: The prop id you want to get the drawable variations of. Refer to [SET_PED_PROP_INDEX](#_0x93376B65A266EB5F)

pub fn get_number_of_ped_prop_drawable_variations_safe(ped: Ped, propId: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_NUMBER_OF_PED_PROP_DRAWABLE_VARIATIONS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_NUMBER_OF_PED_PROP_DRAWABLE_VARIATIONS(ped, propId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_NUMBER_OF_PED_PROP_DRAWABLE_VARIATIONS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_number_of_ped_prop_drawable_variations_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_number_of_ped_prop_drawable_variations_raw(ped: i32, propId: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_NUMBER_OF_PED_PROP_DRAWABLE_VARIATIONS(ped, propId)
}

/// ## Parameters
* **ped**:

pub fn _register_pedheadshot_3_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: _REGISTER_PEDHEADSHOT_3");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_REGISTER_PEDHEADSHOT_3(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _REGISTER_PEDHEADSHOT_3
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_register_pedheadshot_3_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _register_pedheadshot_3_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_REGISTER_PEDHEADSHOT_3(ped)
}

/// ```
Returns:  
-1: Normal  
0: Wearing parachute on back  
1: Parachute opening  
2: Parachute open  
3: Falling to doom (e.g. after exiting parachute)  
Normal means no parachute?  
```

pub fn get_ped_parachute_state_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_PARACHUTE_STATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_PARACHUTE_STATE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_PARACHUTE_STATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_parachute_state_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_parachute_state_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_PARACHUTE_STATE(ped)
}

/// ## Parameters
* **ped**: 
* **hash**:

pub fn set_ped_relationship_group_default_hash_safe(ped: Ped, hash: u32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_RELATIONSHIP_GROUP_DEFAULT_HASH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_RELATIONSHIP_GROUP_DEFAULT_HASH(ped, hash)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_RELATIONSHIP_GROUP_DEFAULT_HASH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_relationship_group_default_hash_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_relationship_group_default_hash_raw(ped: i32, hash: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_RELATIONSHIP_GROUP_DEFAULT_HASH(ped, hash)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_torso_ik_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_TORSO_IK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_TORSO_IK(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_TORSO_IK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_torso_ik_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_torso_ik_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_TORSO_IK(ped, toggle)
}

/// ## Parameters
* **ped**:

pub fn is_ped_in_any_police_vehicle_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_ANY_POLICE_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_ANY_POLICE_VEHICLE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_ANY_POLICE_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_any_police_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_any_police_vehicle_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_ANY_POLICE_VEHICLE(ped)
}

/// ```
Returns the group id of which the specified ped is a member of.  
```

pub fn get_ped_group_index_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_GROUP_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_GROUP_INDEX(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_GROUP_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_group_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_group_index_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_GROUP_INDEX(ped)
}

/// ## Parameters
* **colorID**:

pub fn _is_ped_lipstick_color_valid_safe(colorID: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PED_LIPSTICK_COLOR_VALID");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PED_LIPSTICK_COLOR_VALID(colorID)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PED_LIPSTICK_COLOR_VALID
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_ped_lipstick_color_valid_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_ped_lipstick_color_valid_raw(colorID: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PED_LIPSTICK_COLOR_VALID(colorID)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**: 
* **p3**:

pub fn _0x25361a96e0f7e419_safe(p0: serde_json::Value, p1: serde_json::Value, p2: serde_json::Value, p3: serde_json::Value) -> NativeResult<serde_json::Value> {
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
    
    debug!("Calling native function: _0x25361A96E0F7E419");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x25361A96E0F7E419(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1), crate::utils::any_to_c_void_ptr(p2), crate::utils::any_to_c_void_ptr(p3))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0x25361A96E0F7E419
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x25361a96e0f7e419_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x25361a96e0f7e419_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void, p2: *mut std::os::raw::c_void, p3: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x25361A96E0F7E419(p0, p1, p2, p3)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0xa660faf550eb37e5_safe(p0: serde_json::Value, p1: bool) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xA660FAF550EB37E5");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xA660FAF550EB37E5(crate::utils::any_to_c_void_ptr(p0), p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xA660FAF550EB37E5
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xa660faf550eb37e5_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xa660faf550eb37e5_raw(p0: *mut std::os::raw::c_void, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xA660FAF550EB37E5(p0, p1)
}

/// ## Parameters
* **colorId**:

pub fn _is_ped_blush_color_valid_2_safe(colorId: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PED_BLUSH_COLOR_VALID_2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PED_BLUSH_COLOR_VALID_2(colorId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PED_BLUSH_COLOR_VALID_2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_ped_blush_color_valid_2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_ped_blush_color_valid_2_raw(colorId: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PED_BLUSH_COLOR_VALID_2(colorId)
}

/// ## Parameters
* **ped**: 
* **angle**:

pub fn set_ped_visual_field_center_angle_safe(ped: Ped, angle: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_VISUAL_FIELD_CENTER_ANGLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_VISUAL_FIELD_CENTER_ANGLE(ped, angle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_VISUAL_FIELD_CENTER_ANGLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_visual_field_center_angle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_visual_field_center_angle_raw(ped: i32, angle: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_VISUAL_FIELD_CENTER_ANGLE(ped, angle)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn _0x2b694afcf64e6994_safe(ped: Ped, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x2B694AFCF64E6994");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x2B694AFCF64E6994(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x2B694AFCF64E6994
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x2b694afcf64e6994_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x2b694afcf64e6994_raw(ped: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x2B694AFCF64E6994(ped, p1)
}

/// ## Parameters
* **ped**:

pub fn is_ped_jacking_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_JACKING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_JACKING(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_JACKING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_jacking_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_jacking_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_JACKING(ped)
}

/// ```
SET_A*
```

pub fn _0x87ddeb611b329a9c_safe(multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0x87DDEB611B329A9C");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x87DDEB611B329A9C(multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x87DDEB611B329A9C
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x87ddeb611b329a9c_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x87ddeb611b329a9c_raw(multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x87DDEB611B329A9C(multiplier)
}

/// This native sets the glow intensity of illuminated clothing items.

This native does **NOT** need to be executed every tick.
This native is **NOT** synced with other connected players, you will have to set the opacity on the ped on all clients individually.


Glow intensity is a value between `0.0` and `1.0`.

In some older decompiled scripts this is known as `_SET_PED_REFLECTION_INTENSITY`.
Since there's no joaat hash for this, I find `_SET_PED_ILLUMINATED_CLOTHING_GLOW_INTENSITY` more descriptive than `_SET_PED_REFLECTION_INTENSITY`.

Use [`GetPedIlluminatedClothingGlowIntensity`](#_0x1461B28A06717D68) to get the illuminated clothing glow intensity of a specific ped.

Intensity: `1.0`:
![](https://www.vespura.com/hi/i/2018-11-13_17-03_c2e23_229.png)

Intensity: `0.0`:
![](https://www.vespura.com/hi/i/2018-11-13_17-03_35c33_230.png)

pub fn _set_ped_emissive_intensity_safe(ped: Ped, intensity: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PED_EMISSIVE_INTENSITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PED_EMISSIVE_INTENSITY(ped, intensity)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PED_EMISSIVE_INTENSITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_ped_emissive_intensity_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_ped_emissive_intensity_raw(ped: i32, intensity: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PED_EMISSIVE_INTENSITY(ped, intensity)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn is_ped_defensive_area_active_safe(ped: Ped, p1: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_DEFENSIVE_AREA_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_DEFENSIVE_AREA_ACTIVE(ped, p1)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_DEFENSIVE_AREA_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_defensive_area_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_defensive_area_active_raw(ped: i32, p1: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_DEFENSIVE_AREA_ACTIVE(ped, p1)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_keep_task_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_KEEP_TASK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_KEEP_TASK(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_KEEP_TASK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_keep_task_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_keep_task_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_KEEP_TASK(ped, toggle)
}

/// ## Parameters
* **ped**: The ped handle.

pub fn clear_all_ped_props_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_ALL_PED_PROPS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_ALL_PED_PROPS(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_ALL_PED_PROPS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_all_ped_props_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_all_ped_props_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_ALL_PED_PROPS(ped)
}

/// ## Parameters
* **ped**: 
* **x**: 
* **y**: 
* **z**: 
* **radius**:

pub fn is_any_hostile_ped_near_point_safe(ped: Ped, x: f32, y: f32, z: f32, radius: f32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_ANY_HOSTILE_PED_NEAR_POINT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_ANY_HOSTILE_PED_NEAR_POINT(ped, x, y, z, radius)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_ANY_HOSTILE_PED_NEAR_POINT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_any_hostile_ped_near_point_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_any_hostile_ped_near_point_raw(ped: i32, x: f32, y: f32, z: f32, radius: f32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_ANY_HOSTILE_PED_NEAR_POINT(ped, x, y, z, radius)
}

/// ## Parameters
* **ped**:

pub fn is_ped_jumping_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_JUMPING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_JUMPING(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_JUMPING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_jumping_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_jumping_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_JUMPING(ped)
}

/// ## Parameters
* **ped**:

pub fn is_ped_in_any_plane_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_ANY_PLANE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_ANY_PLANE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_ANY_PLANE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_any_plane_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_any_plane_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_ANY_PLANE(ped)
}

/// ## Parameters
* **ped**: 
* **value**:

pub fn set_ped_id_range_safe(ped: Ped, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_ID_RANGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_ID_RANGE(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_ID_RANGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_id_range_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_id_range_raw(ped: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_ID_RANGE(ped, value)
}

/// ## Parameters
* **ped**: 
* **x**: 
* **y**: 
* **z**: 
* **radius**: 
* **p5**: 
* **p6**:

pub fn set_ped_sphere_defensive_area_safe(ped: Ped, x: f32, y: f32, z: f32, radius: f32, p5: bool, p6: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_SPHERE_DEFENSIVE_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_SPHERE_DEFENSIVE_AREA(ped, x, y, z, radius, p5, p6)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_SPHERE_DEFENSIVE_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_sphere_defensive_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_sphere_defensive_area_raw(ped: i32, x: f32, y: f32, z: f32, radius: f32, p5: bool, p6: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_SPHERE_DEFENSIVE_AREA(ped, x, y, z, radius, p5, p6)
}

/// ```
accuracy = 0-100, 100 being perfectly accurate
```

pub fn set_ped_accuracy_safe(ped: Ped, accuracy: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_ACCURACY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_ACCURACY(ped, accuracy)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_ACCURACY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_accuracy_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_accuracy_raw(ped: i32, accuracy: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_ACCURACY(ped, accuracy)
}

/// ```
SET_PED_STE*
```

pub fn _0x2016c603d6b8987c_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x2016C603D6B8987C");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x2016C603D6B8987C(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x2016C603D6B8987C
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x2016c603d6b8987c_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x2016c603d6b8987c_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x2016C603D6B8987C(ped, toggle)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_be_dragged_out_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_BE_DRAGGED_OUT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_BE_DRAGGED_OUT(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_BE_DRAGGED_OUT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_be_dragged_out_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_be_dragged_out_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_BE_DRAGGED_OUT(ped, toggle)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x5b6010b3cbc29095_safe(p0: serde_json::Value, p1: bool) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x5B6010B3CBC29095");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x5B6010B3CBC29095(crate::utils::any_to_c_void_ptr(p0), p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x5B6010B3CBC29095
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x5b6010b3cbc29095_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x5b6010b3cbc29095_raw(p0: *mut std::os::raw::c_void, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x5B6010B3CBC29095(p0, p1)
}

/// ## Parameters
* **ped**: 
* **value**:

pub fn set_ped_max_time_in_water_safe(ped: Ped, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_MAX_TIME_IN_WATER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_MAX_TIME_IN_WATER(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_MAX_TIME_IN_WATER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_max_time_in_water_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_max_time_in_water_raw(ped: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_MAX_TIME_IN_WATER(ped, value)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_enable_ped_enveff_scale_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_ENABLE_PED_ENVEFF_SCALE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_ENABLE_PED_ENVEFF_SCALE(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_ENABLE_PED_ENVEFF_SCALE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_enable_ped_enveff_scale_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_enable_ped_enveff_scale_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_ENABLE_PED_ENVEFF_SCALE(ped, toggle)
}

/// ## Parameters
* **toggle**:

pub fn set_create_random_cops_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CREATE_RANDOM_COPS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CREATE_RANDOM_COPS(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CREATE_RANDOM_COPS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_create_random_cops_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_create_random_cops_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CREATE_RANDOM_COPS(toggle)
}

/// ## Parameters
* **ped**: The ped handle.
* **propId**: The prop id you want to clear from the ped. Refer to [SET_PED_PROP_INDEX](#_0x93376B65A266EB5F).

pub fn clear_ped_prop_safe(ped: Ped, propId: i64) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_PROP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_PROP(ped, propId)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_PROP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_prop_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_prop_raw(ped: i32, propId: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_PROP(ped, propId)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_head_ik_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_HEAD_IK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_HEAD_IK(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_HEAD_IK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_head_ik_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_head_ik_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_HEAD_IK(ped, toggle)
}

/// ## Parameters
* **ped**:

pub fn get_ped_combat_range_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_COMBAT_RANGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_COMBAT_RANGE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_COMBAT_RANGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_combat_range_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_combat_range_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_COMBAT_RANGE(ped)
}

/// ```
Found one occurence in re_crashrescue.c4  
PED::APPLY_PED_BLOOD(l_4B, 3, 0.0, 0.0, 0.0, "wound_sheet");  
```

pub fn apply_ped_blood_safe(ped: Ped, boneIndex: i64, xRot: f32, yRot: f32, zRot: f32, woundType: String) -> NativeResult<()> {
    let woundType_cstr = std::ffi::CString::new(woundType.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "woundType", e)))?;
    
    debug!("Calling native function: APPLY_PED_BLOOD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::APPLY_PED_BLOOD(ped, boneIndex, xRot, yRot, zRot, crate::utils::rust_to_c_string(woundType))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: APPLY_PED_BLOOD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `apply_ped_blood_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn apply_ped_blood_raw(ped: i32, boneIndex: i64, xRot: f32, yRot: f32, zRot: f32, woundType: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::APPLY_PED_BLOOD(ped, boneIndex, xRot, yRot, zRot, woundType)
}

/// ## Parameters
* **ped**: The ped handle.

pub fn set_ped_random_props_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_RANDOM_PROPS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_RANDOM_PROPS(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_RANDOM_PROPS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_random_props_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_random_props_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_RANDOM_PROPS(ped)
}

/// **This native does absolutely nothing, just a nullsub**

```
Points to the same function as for example GET_RANDOM_VEHICLE_MODEL_IN_MEMORY and it does absolutely nothing.  
```

pub fn set_ped_plays_head_on_horn_anim_when_dies_in_vehicle_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_PLAYS_HEAD_ON_HORN_ANIM_WHEN_DIES_IN_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_PLAYS_HEAD_ON_HORN_ANIM_WHEN_DIES_IN_VEHICLE(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_PLAYS_HEAD_ON_HORN_ANIM_WHEN_DIES_IN_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_plays_head_on_horn_anim_when_dies_in_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_plays_head_on_horn_anim_when_dies_in_vehicle_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_PLAYS_HEAD_ON_HORN_ANIM_WHEN_DIES_IN_VEHICLE(ped, toggle)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_be_targeted_without_los_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_BE_TARGETED_WITHOUT_LOS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_BE_TARGETED_WITHOUT_LOS(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_BE_TARGETED_WITHOUT_LOS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_be_targeted_without_los_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_be_targeted_without_los_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_BE_TARGETED_WITHOUT_LOS(ped, toggle)
}

/// ```
This function will simply bring the dead person back to life.
Try not to use it alone, since using this function alone, will make peds fall through ground in hell(well for the most of the times).
Instead, before calling this function, you may want to declare the position, where your Resurrected ped to be spawn at.(For instance, Around 2 floats of Player's current position.)
Also, disabling any assigned task immediately helped in the number of scenarios, where If you want peds to perform certain decided tasks.
```

pub fn resurrect_ped_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: RESURRECT_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESURRECT_PED(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESURRECT_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `resurrect_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn resurrect_ped_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESURRECT_PED(ped)
}

/// ```
p0: Ped Handle  
p1: int i | 0 <= i <= 27  
p1 probably refers to the attributes configured in combatbehavior.meta. There are 13. Example:  
<BlindFireChance value="0.1"/>  
<WeaponShootRateModifier value="1.0"/>  
<TimeBetweenBurstsInCover value="1.25"/>  
<BurstDurationInCover value="2.0"/>  
<TimeBetweenPeeks value="10.0"/>  
<WeaponAccuracy value="0.18"/>  
<FightProficiency value="0.8"/>  
<StrafeWhenMovingChance value="1.0"/>  
<WalkWhenStrafingChance value="0.0"/>  
<AttackWindowDistanceForCover value="55.0"/>  
<TimeToInvalidateInjuredTarget value="9.0"/>  
<TriggerChargeTime_Near value="4.0"/>  
<TriggerChargeTime_Far value="10.0"/>  
-------------Confirmed by editing combatbehavior.meta:  
p1:  
0=BlindFireChance  
1=BurstDurationInCover  
3=TimeBetweenBurstsInCover  
4=TimeBetweenPeeks  
5=StrafeWhenMovingChance  
8=WalkWhenStrafingChance  
11=AttackWindowDistanceForCover  
12=TimeToInvalidateInjuredTarget  
16=OptimalCoverDistance  
```

pub fn get_combat_float_safe(ped: Ped, p1: i64) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_COMBAT_FLOAT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_COMBAT_FLOAT(ped, p1)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_COMBAT_FLOAT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_combat_float_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_combat_float_raw(ped: i32, p1: i64) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_COMBAT_FLOAT(ped, p1)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **type**: 
* **p3**:

pub fn set_force_step_type_safe(ped: Ped, p1: bool, type: i64, p3: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_FORCE_STEP_TYPE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_FORCE_STEP_TYPE(ped, p1, type, p3)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_FORCE_STEP_TYPE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_force_step_type_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_force_step_type_raw(ped: i32, p1: bool, type: i64, p3: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_FORCE_STEP_TYPE(ped, p1, type, p3)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn set_ped_cloth_package_index_safe(ped: Ped, p1: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CLOTH_PACKAGE_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CLOTH_PACKAGE_INDEX(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CLOTH_PACKAGE_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_cloth_package_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_cloth_package_index_raw(ped: i32, p1: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CLOTH_PACKAGE_INDEX(ped, p1)
}

/// ```
Found in the b617d scripts:
PED::_9DBA107B4937F809(v_7, "trevor_heist_cover_2h");
SET_PED_MO*
```

pub fn _set_ped_cover_clipset_override_safe(ped: Ped, p1: String) -> NativeResult<()> {
    let p1_cstr = std::ffi::CString::new(p1.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "p1", e)))?;
    
    debug!("Calling native function: _SET_PED_COVER_CLIPSET_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PED_COVER_CLIPSET_OVERRIDE(ped, crate::utils::rust_to_c_string(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PED_COVER_CLIPSET_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_ped_cover_clipset_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_ped_cover_clipset_override_raw(ped: i32, p1: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PED_COVER_CLIPSET_OVERRIDE(ped, p1)
}

/// ```
NativeDB Introduced: v1493
```

pub fn _get_ped_visual_field_center_angle_safe(ped: Ped) -> NativeResult<f32> {
    
    debug!("Calling native function: _GET_PED_VISUAL_FIELD_CENTER_ANGLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_PED_VISUAL_FIELD_CENTER_ANGLE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_PED_VISUAL_FIELD_CENTER_ANGLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_ped_visual_field_center_angle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_ped_visual_field_center_angle_raw(ped: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_PED_VISUAL_FIELD_CENTER_ANGLE(ped)
}

/// ## Parameters
* **ped**:

pub fn is_ped_fleeing_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_FLEEING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_FLEEING(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_FLEEING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_fleeing_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_fleeing_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_FLEEING(ped)
}

/// ```
NativeDB Introduced: v1493
```

pub fn _set_enable_scuba_gear_light_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_ENABLE_SCUBA_GEAR_LIGHT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_ENABLE_SCUBA_GEAR_LIGHT(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_ENABLE_SCUBA_GEAR_LIGHT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_enable_scuba_gear_light_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_enable_scuba_gear_light_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_ENABLE_SCUBA_GEAR_LIGHT(ped, toggle)
}

/// ```
gtaforums.com/topic/885580-ped-headshotmugshot-txd/  
```

pub fn get_pedheadshot_txd_string_safe(id: i64) -> NativeResult<String> {
    
    debug!("Calling native function: GET_PEDHEADSHOT_TXD_STRING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PEDHEADSHOT_TXD_STRING(id)
    };
    
    
    Ok(crate::utils::c_string_to_rust_string(result))
}

/// Raw native function: GET_PEDHEADSHOT_TXD_STRING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_pedheadshot_txd_string_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_pedheadshot_txd_string_raw(id: i64) -> *const std::os::raw::c_char {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PEDHEADSHOT_TXD_STRING(id)
}

/// ```
Related to Peds dropping pickup_health_snack; p0 is a value between [0.0, 1.0] that corresponds to drop rate
```

pub fn _0xff4803bc019852d9_safe(p0: f32, p1: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p1_any_str = serde_json::to_string(&p1)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p1", e)))?;
    let p1_any_str_cstr = std::ffi::CString::new(p1_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p1", e)))?;
    
    debug!("Calling native function: _0xFF4803BC019852D9");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xFF4803BC019852D9(p0, crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xFF4803BC019852D9
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xff4803bc019852d9_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xff4803bc019852d9_raw(p0: f32, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xFF4803BC019852D9(p0, p1)
}

/// ## Parameters
* **ped**: 
* **p1**:

pub fn _0x2735233a786b1bef_safe(ped: Ped, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0x2735233A786B1BEF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x2735233A786B1BEF(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x2735233A786B1BEF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x2735233a786b1bef_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x2735233a786b1bef_raw(ped: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x2735233A786B1BEF(ped, p1)
}

/// ## Return value

pub fn spawnpoints_get_num_search_results_safe() -> NativeResult<i64> {
    
    debug!("Calling native function: SPAWNPOINTS_GET_NUM_SEARCH_RESULTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPAWNPOINTS_GET_NUM_SEARCH_RESULTS()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: SPAWNPOINTS_GET_NUM_SEARCH_RESULTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `spawnpoints_get_num_search_results_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn spawnpoints_get_num_search_results_raw() -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPAWNPOINTS_GET_NUM_SEARCH_RESULTS()
}

/// ## Parameters
* **ped**: 
* **value**:

pub fn set_ped_min_move_blend_ratio_safe(ped: Ped, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_MIN_MOVE_BLEND_RATIO");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_MIN_MOVE_BLEND_RATIO(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_MIN_MOVE_BLEND_RATIO
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_min_move_blend_ratio_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_min_move_blend_ratio_raw(ped: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_MIN_MOVE_BLEND_RATIO(ped, value)
}

/// ## Parameters
* **ped**: 
* **x**: 
* **y**: 
* **z**:

pub fn set_ped_panic_exit_scenario_safe(ped: Ped, x: f32, y: f32, z: f32) -> NativeResult<serde_json::Value> {
    
    debug!("Calling native function: SET_PED_PANIC_EXIT_SCENARIO");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_PANIC_EXIT_SCENARIO(ped, x, y, z)
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: SET_PED_PANIC_EXIT_SCENARIO
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_panic_exit_scenario_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_panic_exit_scenario_raw(ped: i32, x: f32, y: f32, z: f32) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_PANIC_EXIT_SCENARIO(ped, x, y, z)
}

/// ## Parameters
* **ped**: 
* **range**:

pub fn set_ped_visual_field_peripheral_range_safe(ped: Ped, range: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_VISUAL_FIELD_PERIPHERAL_RANGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_VISUAL_FIELD_PERIPHERAL_RANGE(ped, range)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_VISUAL_FIELD_PERIPHERAL_RANGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_visual_field_peripheral_range_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_visual_field_peripheral_range_raw(ped: i32, range: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_VISUAL_FIELD_PERIPHERAL_RANGE(ped, range)
}

/// ## Parameters
* **ped**:

pub fn is_ped_in_any_sub_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_ANY_SUB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_ANY_SUB(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_ANY_SUB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_any_sub_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_any_sub_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_ANY_SUB(ped)
}

/// SET_A*

```
NativeDB Introduced: v1734
```

pub fn _0xfab944d4d481accb_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xFAB944D4D481ACCB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xFAB944D4D481ACCB(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xFAB944D4D481ACCB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xfab944d4d481accb_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xfab944d4d481accb_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xFAB944D4D481ACCB(ped, toggle)
}

/// ```
Sweat is set to 100.0 or 0.0 in the decompiled scripts.  
```

pub fn set_ped_sweat_safe(ped: Ped, sweat: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_SWEAT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_SWEAT(ped, sweat)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_SWEAT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_sweat_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_sweat_raw(ped: i32, sweat: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_SWEAT(ped, sweat)
}

/// ```
p1 is nearly always 0 in the scripts.  
```

pub fn is_ped_in_cover_safe(ped: Ped, exceptUseWeapon: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_COVER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_COVER(ped, exceptUseWeapon)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_COVER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_cover_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_cover_raw(ped: i32, exceptUseWeapon: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_COVER(ped, exceptUseWeapon)
}

/// ## Parameters
* **Ped**:

pub fn _is_ped_swapping_weapon_safe(Ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PED_SWAPPING_WEAPON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PED_SWAPPING_WEAPON(Ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PED_SWAPPING_WEAPON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_ped_swapping_weapon_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_ped_swapping_weapon_raw(Ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PED_SWAPPING_WEAPON(Ped)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **p2**:

pub fn force_ped_ai_and_animation_update_safe(ped: Ped, p1: bool, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: FORCE_PED_AI_AND_ANIMATION_UPDATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::FORCE_PED_AI_AND_ANIMATION_UPDATE(ped, p1, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: FORCE_PED_AI_AND_ANIMATION_UPDATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `force_ped_ai_and_animation_update_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn force_ped_ai_and_animation_update_raw(ped: i32, p1: bool, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::FORCE_PED_AI_AND_ANIMATION_UPDATE(ped, p1, p2)
}

/// ## Parameters
* **ped**:

pub fn _is_ped_doing_beast_jump_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PED_DOING_BEAST_JUMP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PED_DOING_BEAST_JUMP(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PED_DOING_BEAST_JUMP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_ped_doing_beast_jump_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_ped_doing_beast_jump_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PED_DOING_BEAST_JUMP(ped)
}

/// ## Parameters
* **ped**:

pub fn is_ped_aiming_from_cover_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_AIMING_FROM_COVER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_AIMING_FROM_COVER(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_AIMING_FROM_COVER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_aiming_from_cover_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_aiming_from_cover_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_AIMING_FROM_COVER(ped)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_get_out_upside_down_vehicle_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_GET_OUT_UPSIDE_DOWN_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_GET_OUT_UPSIDE_DOWN_VEHICLE(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_GET_OUT_UPSIDE_DOWN_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_get_out_upside_down_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_get_out_upside_down_vehicle_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_GET_OUT_UPSIDE_DOWN_VEHICLE(ped, toggle)
}

/// ```
REQUEST_*
```

pub fn _0x75ba1cb3b7d40caf_safe(ped: Ped, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x75BA1CB3B7D40CAF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x75BA1CB3B7D40CAF(ped, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x75BA1CB3B7D40CAF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x75ba1cb3b7d40caf_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x75ba1cb3b7d40caf_raw(ped: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x75BA1CB3B7D40CAF(ped, p1)
}

/// ## Parameters
* **ped**:

pub fn is_ped_jumping_out_of_vehicle_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_JUMPING_OUT_OF_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_JUMPING_OUT_OF_VEHICLE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_JUMPING_OUT_OF_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_jumping_out_of_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_jumping_out_of_vehicle_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_JUMPING_OUT_OF_VEHICLE(ped)
}

/// ## Parameters
* **asset**:

pub fn request_action_mode_asset_safe(asset: String) -> NativeResult<()> {
    let asset_cstr = std::ffi::CString::new(asset.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "asset", e)))?;
    
    debug!("Calling native function: REQUEST_ACTION_MODE_ASSET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REQUEST_ACTION_MODE_ASSET(crate::utils::rust_to_c_string(asset))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REQUEST_ACTION_MODE_ASSET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `request_action_mode_asset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn request_action_mode_asset_raw(asset: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REQUEST_ACTION_MODE_ASSET(asset)
}

/// ## Parameters
* **ped**:

pub fn is_ped_prone_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_PRONE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_PRONE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_PRONE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_prone_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_prone_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_PRONE(ped)
}

/// ## Parameters
* **ped**:

pub fn is_ped_in_any_heli_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_ANY_HELI");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_ANY_HELI(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_ANY_HELI
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_any_heli_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_any_heli_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_ANY_HELI(ped)
}

/// ## Parameters
* **ped**:

pub fn _0xd33daa36272177c4_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: _0xD33DAA36272177C4");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xD33DAA36272177C4(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xD33DAA36272177C4
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xd33daa36272177c4_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xd33daa36272177c4_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xD33DAA36272177C4(ped)
}

/// ```
p6 always 2 (but it doesnt seem to matter...)  
roll and pitch 0  
yaw to Ped.rotation  
```

pub fn create_synchronized_scene_safe(x: f32, y: f32, z: f32, roll: f32, pitch: f32, yaw: f32, p6: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: CREATE_SYNCHRONIZED_SCENE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CREATE_SYNCHRONIZED_SCENE(x, y, z, roll, pitch, yaw, p6)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CREATE_SYNCHRONIZED_SCENE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `create_synchronized_scene_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn create_synchronized_scene_raw(x: f32, y: f32, z: f32, roll: f32, pitch: f32, yaw: f32, p6: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CREATE_SYNCHRONIZED_SCENE(x, y, z, roll, pitch, yaw, p6)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_be_targetted_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_BE_TARGETTED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_BE_TARGETTED(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_BE_TARGETTED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_be_targetted_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_be_targetted_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_BE_TARGETTED(ped, toggle)
}

/// ## Parameters
* **sceneID**:

pub fn detach_synchronized_scene_safe(sceneID: i64) -> NativeResult<()> {
    
    debug!("Calling native function: DETACH_SYNCHRONIZED_SCENE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DETACH_SYNCHRONIZED_SCENE(sceneID)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DETACH_SYNCHRONIZED_SCENE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `detach_synchronized_scene_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn detach_synchronized_scene_raw(sceneID: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DETACH_SYNCHRONIZED_SCENE(sceneID)
}

/// ## Parameters
* **ped**: 
* **father**: 
* **mother**: 
* **fathersSide**: 
* **mothersSide**:

pub fn set_ped_blend_from_parents_safe(ped: Ped, father: Ped, mother: Ped, fathersSide: f32, mothersSide: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_BLEND_FROM_PARENTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_BLEND_FROM_PARENTS(ped, father, mother, fathersSide, mothersSide)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_BLEND_FROM_PARENTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_blend_from_parents_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_blend_from_parents_raw(ped: i32, father: i32, mother: i32, fathersSide: f32, mothersSide: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_BLEND_FROM_PARENTS(ped, father, mother, fathersSide, mothersSide)
}

/// _0x5A7F62FDA59759BD native function

pub fn _0x5a7f62fda59759bd_safe() -> NativeResult<()> {
    
    debug!("Calling native function: _0x5A7F62FDA59759BD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x5A7F62FDA59759BD()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x5A7F62FDA59759BD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x5a7f62fda59759bd_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x5a7f62fda59759bd_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x5A7F62FDA59759BD()
}

/// ## Parameters
* **ped**:

pub fn is_ped_in_parachute_free_fall_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_PARACHUTE_FREE_FALL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_PARACHUTE_FREE_FALL(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_PARACHUTE_FREE_FALL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_parachute_free_fall_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_parachute_free_fall_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_PARACHUTE_FREE_FALL(ped)
}

/// ```
p1 is usually 0 in the scripts. action is either 0 or a pointer to "DEFAULT_ACTION".  
```

pub fn set_ped_stealth_movement_safe(ped: Ped, p1: bool, action: String) -> NativeResult<()> {
    let action_cstr = std::ffi::CString::new(action.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "action", e)))?;
    
    debug!("Calling native function: SET_PED_STEALTH_MOVEMENT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_STEALTH_MOVEMENT(ped, p1, crate::utils::rust_to_c_string(action))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_STEALTH_MOVEMENT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_stealth_movement_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_stealth_movement_raw(ped: i32, p1: bool, action: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_STEALTH_MOVEMENT(ped, p1, action)
}

/// ```
Creates a new ped group.  
Groups can contain up to 8 peds.  
The parameter is unused.  
Returns a handle to the created group, or 0 if a group couldn't be created.  
```

pub fn create_group_safe(unused: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: CREATE_GROUP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CREATE_GROUP(unused)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CREATE_GROUP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `create_group_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn create_group_raw(unused: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CREATE_GROUP(unused)
}

/// ## Parameters
* **ped**: 
* **outBone**:

pub fn get_ped_last_damage_bone_safe(ped: Ped, outBone: *mut i64) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_PED_LAST_DAMAGE_BONE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_LAST_DAMAGE_BONE(ped, outBone)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_LAST_DAMAGE_BONE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_last_damage_bone_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_last_damage_bone_raw(ped: i32, outBone: *mut i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_LAST_DAMAGE_BONE(ped, outBone)
}

/// ## Parameters
* **ped**: 
* **value**:

pub fn set_ped_hearing_range_safe(ped: Ped, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_HEARING_RANGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_HEARING_RANGE(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_HEARING_RANGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_hearing_range_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_hearing_range_raw(ped: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_HEARING_RANGE(ped, value)
}

/// ```
Returns the hash of the weapon/model/object that killed the ped.  
```

pub fn get_ped_cause_of_death_safe(ped: Ped) -> NativeResult<u32> {
    
    debug!("Calling native function: GET_PED_CAUSE_OF_DEATH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_CAUSE_OF_DEATH(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_CAUSE_OF_DEATH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_cause_of_death_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_cause_of_death_raw(ped: i32) -> u32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_CAUSE_OF_DEATH(ped)
}

/// ```
from extreme3.c4
PED::_39D55A620FCB6A3A(PLAYER::PLAYER_PED_ID(), 8, PED::GET_PED_DRAWABLE_VARIATION(PLAYER::PLAYER_PED_ID(), 8), PED::GET_PED_TEXTURE_VARIATION(PLAYER::PLAYER_PED_ID(), 8));
p1 is probably componentId
```

pub fn set_ped_preload_variation_data_safe(ped: Ped, slot: i64, drawableId: i64, textureId: i64) -> NativeResult<serde_json::Value> {
    
    debug!("Calling native function: SET_PED_PRELOAD_VARIATION_DATA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_PRELOAD_VARIATION_DATA(ped, slot, drawableId, textureId)
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: SET_PED_PRELOAD_VARIATION_DATA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_preload_variation_data_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_preload_variation_data_raw(ped: i32, slot: i64, drawableId: i64, textureId: i64) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_PRELOAD_VARIATION_DATA(ped, slot, drawableId, textureId)
}

/// ## Parameters
* **ped**:

pub fn set_ped_ragdoll_force_fall_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_RAGDOLL_FORCE_FALL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_RAGDOLL_FORCE_FALL(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_RAGDOLL_FORCE_FALL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_ragdoll_force_fall_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_ragdoll_force_fall_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_RAGDOLL_FORCE_FALL(ped)
}

/// Native to check whether [`_SET_PED_SCUBA_GEAR_VARIATION`](#_0x36C6984C3ED0C911) is enabled/actived.

pub fn _0xfec9a3b1820f3331_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: _0xFEC9A3B1820F3331");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xFEC9A3B1820F3331(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0xFEC9A3B1820F3331
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xfec9a3b1820f3331_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xfec9a3b1820f3331_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xFEC9A3B1820F3331(ped)
}

/// ## Parameters
* **ped**: 
* **stance**: 
* **p2**:

pub fn clear_ped_alternate_movement_anim_safe(ped: Ped, stance: i64, p2: f32) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_ALTERNATE_MOVEMENT_ANIM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_ALTERNATE_MOVEMENT_ANIM(ped, stance, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_ALTERNATE_MOVEMENT_ANIM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_alternate_movement_anim_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_alternate_movement_anim_raw(ped: i32, stance: i64, p2: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_ALTERNATE_MOVEMENT_ANIM(ped, stance, p2)
}

/// This native is used to set component variation on a ped. Components, drawables and textures IDs are related to the ped model.

pub fn set_ped_component_variation_safe(ped: Ped, componentId: i64, drawableId: i64, textureId: i64, paletteId: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_COMPONENT_VARIATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_COMPONENT_VARIATION(ped, componentId, drawableId, textureId, paletteId)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_COMPONENT_VARIATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_component_variation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_component_variation_raw(ped: i32, componentId: i64, drawableId: i64, textureId: i64, paletteId: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_COMPONENT_VARIATION(ped, componentId, drawableId, textureId, paletteId)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_play_ambient_anims_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_PLAY_AMBIENT_ANIMS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_PLAY_AMBIENT_ANIMS(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_PLAY_AMBIENT_ANIMS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_play_ambient_anims_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_play_ambient_anims_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_PLAY_AMBIENT_ANIMS(ped, toggle)
}

/// ## Parameters
* **scene**: Scene ID returned by [`CREATE_SYNCHRONIZED_SCENE`](#_0x8C18E0F9080ADD73)

pub fn take_ownership_of_synchronized_scene_safe(scene: i64) -> NativeResult<()> {
    
    debug!("Calling native function: TAKE_OWNERSHIP_OF_SYNCHRONIZED_SCENE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::TAKE_OWNERSHIP_OF_SYNCHRONIZED_SCENE(scene)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: TAKE_OWNERSHIP_OF_SYNCHRONIZED_SCENE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `take_ownership_of_synchronized_scene_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn take_ownership_of_synchronized_scene_raw(scene: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::TAKE_OWNERSHIP_OF_SYNCHRONIZED_SCENE(scene)
}

/// ## Parameters
* **ped**:

pub fn clear_facial_idle_anim_override_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_FACIAL_IDLE_ANIM_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_FACIAL_IDLE_ANIM_OVERRIDE(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_FACIAL_IDLE_ANIM_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_facial_idle_anim_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_facial_idle_anim_override_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_FACIAL_IDLE_ANIM_OVERRIDE(ped)
}

/// ## Parameters
* **p0**:

pub fn set_ambient_peds_drop_money_safe(p0: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_AMBIENT_PEDS_DROP_MONEY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_AMBIENT_PEDS_DROP_MONEY(p0)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_AMBIENT_PEDS_DROP_MONEY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ambient_peds_drop_money_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ambient_peds_drop_money_raw(p0: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_AMBIENT_PEDS_DROP_MONEY(p0)
}

/// ```
Gets the relationship between two groups. This should be called twice (once for each group).  
Relationship types:  
0 = Companion  
1 = Respect  
2 = Like  
3 = Neutral  
4 = Dislike  
5 = Hate  
255 = Pedestrians  
Example:  
PED::GET_RELATIONSHIP_BETWEEN_GROUPS(l_1017, 0xA49E591C);  
PED::GET_RELATIONSHIP_BETWEEN_GROUPS(0xA49E591C, l_1017);  
```

pub fn get_relationship_between_groups_safe(group1: u32, group2: u32) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_RELATIONSHIP_BETWEEN_GROUPS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_RELATIONSHIP_BETWEEN_GROUPS(group1, group2)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_RELATIONSHIP_BETWEEN_GROUPS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_relationship_between_groups_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_relationship_between_groups_raw(group1: u32, group2: u32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_RELATIONSHIP_BETWEEN_GROUPS(group1, group2)
}

/// p4/p5: Unusued in TU27

pub fn set_ped_to_ragdoll_safe(ped: Ped, minTime: i64, maxTime: i64, ragdollType: i64, bAbortIfInjured: bool, bAbortIfDead: bool, bForceScriptControl: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: SET_PED_TO_RAGDOLL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_TO_RAGDOLL(ped, minTime, maxTime, ragdollType, bAbortIfInjured, bAbortIfDead, bForceScriptControl)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: SET_PED_TO_RAGDOLL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_to_ragdoll_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_to_ragdoll_raw(ped: i32, minTime: i64, maxTime: i64, ragdollType: i64, bAbortIfInjured: bool, bAbortIfDead: bool, bForceScriptControl: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_TO_RAGDOLL(ped, minTime, maxTime, ragdollType, bAbortIfInjured, bAbortIfDead, bForceScriptControl)
}

/// ```
gtaforums.com/topic/885580-ped-headshotmugshot-txd/  
```

pub fn unregister_pedheadshot_safe(id: i64) -> NativeResult<()> {
    
    debug!("Calling native function: UNREGISTER_PEDHEADSHOT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::UNREGISTER_PEDHEADSHOT(id)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: UNREGISTER_PEDHEADSHOT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `unregister_pedheadshot_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn unregister_pedheadshot_raw(id: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::UNREGISTER_PEDHEADSHOT(id)
}

/// ## Parameters
* **ped**:

pub fn is_ped_performing_melee_action_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_PERFORMING_MELEE_ACTION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_PERFORMING_MELEE_ACTION(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_PERFORMING_MELEE_ACTION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_performing_melee_action_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_performing_melee_action_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_PERFORMING_MELEE_ACTION(ped)
}

/// ## Parameters
* **groupHash**:

pub fn remove_relationship_group_safe(groupHash: u32) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_RELATIONSHIP_GROUP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_RELATIONSHIP_GROUP(groupHash)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_RELATIONSHIP_GROUP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_relationship_group_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_relationship_group_raw(groupHash: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_RELATIONSHIP_GROUP(groupHash)
}

/// ```
Sets the armor of the specified ped.  
ped: The Ped to set the armor of.  
amount: A value between 0 and 100 indicating the value to set the Ped's armor to.  
```

pub fn set_ped_armour_safe(ped: Ped, amount: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_ARMOUR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_ARMOUR(ped, amount)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_ARMOUR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_armour_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_armour_raw(ped: i32, amount: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_ARMOUR(ped, amount)
}

/// Sets an area where scenarios are blocked

pub fn add_scenario_blocking_area_safe(posMinX: f32, posMinY: f32, posMinZ: f32, posMaxX: f32, posMaxY: f32, posMaxZ: f32, network: bool, cancelActive: bool, blockPeds: bool, blockVehicles: bool) -> NativeResult<i64> {
    
    debug!("Calling native function: ADD_SCENARIO_BLOCKING_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ADD_SCENARIO_BLOCKING_AREA(posMinX, posMinY, posMinZ, posMaxX, posMaxY, posMaxZ, network, cancelActive, blockPeds, blockVehicles)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: ADD_SCENARIO_BLOCKING_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `add_scenario_blocking_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn add_scenario_blocking_area_raw(posMinX: f32, posMinY: f32, posMinZ: f32, posMaxX: f32, posMaxY: f32, posMaxZ: f32, network: bool, cancelActive: bool, blockPeds: bool, blockVehicles: bool) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ADD_SCENARIO_BLOCKING_AREA(posMinX, posMinY, posMinZ, posMaxX, posMaxY, posMaxZ, network, cancelActive, blockPeds, blockVehicles)
}

/// ## Parameters
* **x**: 
* **y**: 
* **z**: 
* **radius**:

pub fn is_any_ped_near_point_safe(x: f32, y: f32, z: f32, radius: f32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_ANY_PED_NEAR_POINT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_ANY_PED_NEAR_POINT(x, y, z, radius)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_ANY_PED_NEAR_POINT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_any_ped_near_point_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_any_ped_near_point_raw(x: f32, y: f32, z: f32, radius: f32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_ANY_PED_NEAR_POINT(x, y, z, radius)
}

/// ```
GET_*
```

pub fn _0xf033419d1b81fae8_safe(p0: serde_json::Value) -> NativeResult<serde_json::Value> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xF033419D1B81FAE8");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xF033419D1B81FAE8(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0xF033419D1B81FAE8
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xf033419d1b81fae8_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xf033419d1b81fae8_raw(p0: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xF033419D1B81FAE8(p0)
}

/// ```
Gets the offset the specified ped has moved since the previous tick.  
If worldSpace is false, the returned offset is relative to the ped. That is, if the ped has moved 1 meter right and 5 meters forward, it'll return 1,5,0.  
If worldSpace is true, the returned offset is relative to the world. That is, if the ped has moved 1 meter on the X axis and 5 meters on the Y axis, it'll return 1,5,0.  
```

pub fn get_ped_extracted_displacement_safe(ped: Ped, worldSpace: bool) -> NativeResult<Vector3> {
    
    debug!("Calling native function: GET_PED_EXTRACTED_DISPLACEMENT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_EXTRACTED_DISPLACEMENT(ped, worldSpace)
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: GET_PED_EXTRACTED_DISPLACEMENT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_extracted_displacement_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_extracted_displacement_raw(ped: i32, worldSpace: bool) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_EXTRACTED_DISPLACEMENT(ped, worldSpace)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**: 
* **p5**: 
* **p6**: 
* **p7**: 
* **p8**: 
* **p9**:

pub fn set_ped_angled_defensive_area_safe(ped: Ped, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: bool, p9: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_ANGLED_DEFENSIVE_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_ANGLED_DEFENSIVE_AREA(ped, p1, p2, p3, p4, p5, p6, p7, p8, p9)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_ANGLED_DEFENSIVE_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_angled_defensive_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_angled_defensive_area_raw(ped: i32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32, p7: f32, p8: bool, p9: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_ANGLED_DEFENSIVE_AREA(ped, p1, p2, p3, p4, p5, p6, p7, p8, p9)
}

/// ## Parameters
* **ped**:

pub fn is_ped_swimming_under_water_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_SWIMMING_UNDER_WATER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_SWIMMING_UNDER_WATER(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_SWIMMING_UNDER_WATER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_swimming_under_water_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_swimming_under_water_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_SWIMMING_UNDER_WATER(ped)
}

/// ## Parameters
* **asset**:

pub fn request_stealth_mode_asset_safe(asset: String) -> NativeResult<()> {
    let asset_cstr = std::ffi::CString::new(asset.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "asset", e)))?;
    
    debug!("Calling native function: REQUEST_STEALTH_MODE_ASSET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REQUEST_STEALTH_MODE_ASSET(crate::utils::rust_to_c_string(asset))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REQUEST_STEALTH_MODE_ASSET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `request_stealth_mode_asset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn request_stealth_mode_asset_raw(asset: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REQUEST_STEALTH_MODE_ASSET(asset)
}

/// ```
Need to check behavior when drawableId = -1
```

pub fn get_number_of_ped_prop_texture_variations_safe(ped: Ped, propId: i64, drawableId: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_NUMBER_OF_PED_PROP_TEXTURE_VARIATIONS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_NUMBER_OF_PED_PROP_TEXTURE_VARIATIONS(ped, propId, drawableId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_NUMBER_OF_PED_PROP_TEXTURE_VARIATIONS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_number_of_ped_prop_texture_variations_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_number_of_ped_prop_texture_variations_raw(ped: i32, propId: i64, drawableId: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_NUMBER_OF_PED_PROP_TEXTURE_VARIATIONS(ped, propId, drawableId)
}

/// Use [`SetPedIlluminatedClothingGlowIntensity`](#_0x4E90D746056E273D) to set the illuminated clothing glow intensity for a specific ped.

pub fn _get_ped_emissive_intensity_safe(ped: Ped) -> NativeResult<f32> {
    
    debug!("Calling native function: _GET_PED_EMISSIVE_INTENSITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_PED_EMISSIVE_INTENSITY(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_PED_EMISSIVE_INTENSITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_ped_emissive_intensity_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_ped_emissive_intensity_raw(ped: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_PED_EMISSIVE_INTENSITY(ped)
}

/// ## Parameters
* **ped**:

pub fn was_ped_killed_by_takedown_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: WAS_PED_KILLED_BY_TAKEDOWN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::WAS_PED_KILLED_BY_TAKEDOWN(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: WAS_PED_KILLED_BY_TAKEDOWN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `was_ped_killed_by_takedown_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn was_ped_killed_by_takedown_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::WAS_PED_KILLED_BY_TAKEDOWN(ped)
}

/// ```
Notes: The function only returns true while the ped is:   
A.) Swinging a random melee attack (including pistol-whipping)  
B.) Reacting to being hit by a melee attack (including pistol-whipping)  
C.) Is locked-on to an enemy (arms up, strafing/skipping in the default fighting-stance, ready to dodge+counter).   
You don't have to be holding the melee-targetting button to be in this stance; you stay in it by default for a few seconds after swinging at someone. If you do a sprinting punch, it returns true for the duration of the punch animation and then returns false again, even if you've punched and made-angry many peds  
```

pub fn is_ped_in_melee_combat_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_MELEE_COMBAT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_MELEE_COMBAT(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_MELEE_COMBAT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_melee_combat_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_melee_combat_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_MELEE_COMBAT(ped)
}

/// ## Parameters
* **ped**:

pub fn force_ped_to_open_parachute_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: FORCE_PED_TO_OPEN_PARACHUTE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::FORCE_PED_TO_OPEN_PARACHUTE(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: FORCE_PED_TO_OPEN_PARACHUTE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `force_ped_to_open_parachute_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn force_ped_to_open_parachute_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::FORCE_PED_TO_OPEN_PARACHUTE(ped)
}

/// ## Parameters
* **ped**: 
* **clipset**:

pub fn set_ped_drive_by_clipset_override_safe(ped: Ped, clipset: String) -> NativeResult<()> {
    let clipset_cstr = std::ffi::CString::new(clipset.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "clipset", e)))?;
    
    debug!("Calling native function: SET_PED_DRIVE_BY_CLIPSET_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_DRIVE_BY_CLIPSET_OVERRIDE(ped, crate::utils::rust_to_c_string(clipset))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_DRIVE_BY_CLIPSET_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_drive_by_clipset_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_drive_by_clipset_override_raw(ped: i32, clipset: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_DRIVE_BY_CLIPSET_OVERRIDE(ped, clipset)
}

/// Set the number of scenario peds on the entire map

pub fn set_scenario_ped_density_multiplier_this_frame_safe(interiorMult: f32, exteriorMult: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SCENARIO_PED_DENSITY_MULTIPLIER_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SCENARIO_PED_DENSITY_MULTIPLIER_THIS_FRAME(interiorMult, exteriorMult)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SCENARIO_PED_DENSITY_MULTIPLIER_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_scenario_ped_density_multiplier_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_scenario_ped_density_multiplier_this_frame_raw(interiorMult: f32, exteriorMult: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SCENARIO_PED_DENSITY_MULTIPLIER_THIS_FRAME(interiorMult, exteriorMult)
}

/// ```
p1: from 0 to 5 in the b617d scripts.  
p2: "blushing" and "ALL" found in the b617d scripts.  
```

pub fn clear_ped_damage_decal_by_zone_safe(ped: Ped, p1: i64, p2: String) -> NativeResult<()> {
    let p2_cstr = std::ffi::CString::new(p2.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "p2", e)))?;
    
    debug!("Calling native function: CLEAR_PED_DAMAGE_DECAL_BY_ZONE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_DAMAGE_DECAL_BY_ZONE(ped, p1, crate::utils::rust_to_c_string(p2))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_DAMAGE_DECAL_BY_ZONE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_damage_decal_by_zone_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_damage_decal_by_zone_raw(ped: i32, p1: i64, p2: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_DAMAGE_DECAL_BY_ZONE(ped, p1, p2)
}

/// ```
GET_TIME_*
```

pub fn _get_time_of_last_ped_weapon_damage_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_TIME_OF_LAST_PED_WEAPON_DAMAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_TIME_OF_LAST_PED_WEAPON_DAMAGE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_TIME_OF_LAST_PED_WEAPON_DAMAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_time_of_last_ped_weapon_damage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_time_of_last_ped_weapon_damage_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_TIME_OF_LAST_PED_WEAPON_DAMAGE(ped)
}

/// ## Parameters
* **ped**:

pub fn _0xb8b52e498014f5b0_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: _0xB8B52E498014F5B0");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xB8B52E498014F5B0(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0xB8B52E498014F5B0
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xb8b52e498014f5b0_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xb8b52e498014f5b0_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xB8B52E498014F5B0(ped)
}

/// ```
Applies an Item from a PedDecorationCollection to a ped. These include tattoos and shirt decals.
collection - PedDecorationCollection filename hash
overlay - Item name hash
Example:
Entry inside "mpbeach_overlays.xml" -
<Item>
  <uvPos x="0.500000" y="0.500000" />
  <scale x="0.600000" y="0.500000" />
  <rotation value="0.000000" />
  <nameHash>FM_Hair_Fuzz</nameHash>
  <txdHash>mp_hair_fuzz</txdHash>
  <txtHash>mp_hair_fuzz</txtHash>
  <zone>ZONE_HEAD</zone>
  <type>TYPE_TATTOO</type>
  <faction>FM</faction>
  <garment>All</garment>
  <gender>GENDER_DONTCARE</gender>
  <award />
  <awardLevel />
</Item>
Code:
PED::_0x5F5D1665E352A839(PLAYER::PLAYER_PED_ID(), MISC::GET_HASH_KEY("mpbeach_overlays"), MISC::GET_HASH_KEY("fm_hair_fuzz"))
```

pub fn add_ped_decoration_from_hashes_safe(ped: Ped, collection: u32, overlay: u32) -> NativeResult<()> {
    
    debug!("Calling native function: ADD_PED_DECORATION_FROM_HASHES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ADD_PED_DECORATION_FROM_HASHES(ped, collection, overlay)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ADD_PED_DECORATION_FROM_HASHES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `add_ped_decoration_from_hashes_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn add_ped_decoration_from_hashes_raw(ped: i32, collection: u32, overlay: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ADD_PED_DECORATION_FROM_HASHES(ped, collection, overlay)
}

/// ```
Used for freemode (online) characters.  
```

pub fn _get_num_hair_colors_safe() -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_NUM_HAIR_COLORS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_NUM_HAIR_COLORS()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_NUM_HAIR_COLORS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_num_hair_colors_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_num_hair_colors_raw() -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_NUM_HAIR_COLORS()
}

/// ## Parameters
* **ped**: 
* **transitionSpeed**:

pub fn reset_ped_movement_clipset_safe(ped: Ped, transitionSpeed: f32) -> NativeResult<()> {
    
    debug!("Calling native function: RESET_PED_MOVEMENT_CLIPSET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_PED_MOVEMENT_CLIPSET(ped, transitionSpeed)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_PED_MOVEMENT_CLIPSET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_ped_movement_clipset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_ped_movement_clipset_raw(ped: i32, transitionSpeed: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_PED_MOVEMENT_CLIPSET(ped, transitionSpeed)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**: 
* **p5**: 
* **p6**: 
* **p7**: 
* **p8**:

pub fn _0x03ea03af85a85cb7_safe(ped: Ped, p1: bool, p2: bool, p3: bool, p4: bool, p5: bool, p6: bool, p7: bool, p8: serde_json::Value) -> NativeResult<bool> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p8_any_str = serde_json::to_string(&p8)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p8", e)))?;
    let p8_any_str_cstr = std::ffi::CString::new(p8_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p8", e)))?;
    
    debug!("Calling native function: _0x03EA03AF85A85CB7");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x03EA03AF85A85CB7(ped, p1, p2, p3, p4, p5, p6, p7, crate::utils::any_to_c_void_ptr(p8))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0x03EA03AF85A85CB7
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x03ea03af85a85cb7_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x03ea03af85a85cb7_raw(ped: i32, p1: bool, p2: bool, p3: bool, p4: bool, p5: bool, p6: bool, p7: bool, p8: *mut std::os::raw::c_void) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x03EA03AF85A85CB7(ped, p1, p2, p3, p4, p5, p6, p7, p8)
}

/// ## Parameters
* **unk**:

pub fn can_create_random_ped_safe(unk: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: CAN_CREATE_RANDOM_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CAN_CREATE_RANDOM_PED(unk)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CAN_CREATE_RANDOM_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `can_create_random_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn can_create_random_ped_raw(unk: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CAN_CREATE_RANDOM_PED(unk)
}

/// ## Parameters
* **ped**: 
* **value**:

pub fn set_ped_visual_field_min_angle_safe(ped: Ped, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_VISUAL_FIELD_MIN_ANGLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_VISUAL_FIELD_MIN_ANGLE(ped, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_VISUAL_FIELD_MIN_ANGLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_visual_field_min_angle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_visual_field_min_angle_raw(ped: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_VISUAL_FIELD_MIN_ANGLE(ped, value)
}

/// Define the scope within which the ped will engage in combat with the target.

```c
enum eCombatRange {
    CR_NEAR = 0, // keeps within 5-15m
    CR_MEDIUM = 1, // keeps within 7-30m
    CR_FAR = 2, // keeps within 15-40m
    CR_VERY_FAR = 3 // keeps within 22-45m
};
```

pub fn set_ped_combat_range_safe(ped: Ped, range: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_COMBAT_RANGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_COMBAT_RANGE(ped, range)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_COMBAT_RANGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_combat_range_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_combat_range_raw(ped: i32, range: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_COMBAT_RANGE(ped, range)
}

/// ## Parameters
* **ped**:

pub fn get_ped_relationship_group_hash_safe(ped: Ped) -> NativeResult<u32> {
    
    debug!("Calling native function: GET_PED_RELATIONSHIP_GROUP_HASH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_RELATIONSHIP_GROUP_HASH(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_RELATIONSHIP_GROUP_HASH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_relationship_group_hash_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_relationship_group_hash_raw(ped: i32) -> u32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_RELATIONSHIP_GROUP_HASH(ped)
}

/// Initial guess of native: `_IS_PED_WEARING_MOTORCYCLE_HELMET`.

pub fn _0xf2385935bffd4d92_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: _0xF2385935BFFD4D92");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xF2385935BFFD4D92(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0xF2385935BFFD4D92
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xf2385935bffd4d92_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xf2385935bffd4d92_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xF2385935BFFD4D92(ped)
}

/// ## Parameters
* **ped**:

pub fn knock_ped_off_vehicle_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: KNOCK_PED_OFF_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::KNOCK_PED_OFF_VEHICLE(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: KNOCK_PED_OFF_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `knock_ped_off_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn knock_ped_off_vehicle_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::KNOCK_PED_OFF_VEHICLE(ped)
}

/// ## Parameters
* **ped**:

pub fn is_ped_vaulting_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_VAULTING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_VAULTING(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_VAULTING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_vaulting_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_vaulting_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_VAULTING(ped)
}

/// ## Parameters
* **ped**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**:

pub fn apply_ped_blood_by_zone_safe(ped: Ped, p1: serde_json::Value, p2: f32, p3: f32, p4: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p1_any_str = serde_json::to_string(&p1)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p1", e)))?;
    let p1_any_str_cstr = std::ffi::CString::new(p1_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p1", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p4_any_str = serde_json::to_string(&p4)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p4", e)))?;
    let p4_any_str_cstr = std::ffi::CString::new(p4_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p4", e)))?;
    
    debug!("Calling native function: APPLY_PED_BLOOD_BY_ZONE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::APPLY_PED_BLOOD_BY_ZONE(ped, crate::utils::any_to_c_void_ptr(p1), p2, p3, crate::utils::any_to_c_void_ptr(p4))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: APPLY_PED_BLOOD_BY_ZONE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `apply_ped_blood_by_zone_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn apply_ped_blood_by_zone_raw(ped: i32, p1: *mut std::os::raw::c_void, p2: f32, p3: f32, p4: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::APPLY_PED_BLOOD_BY_ZONE(ped, p1, p2, p3, p4)
}

/// It makes the ped lose (or not lose) their props (like glasses or helmets/hat) when someone punches or pushes the ped.
This is probably what's being used in GTA:O to keep players from knocking other player's hats/glasses off when in combat.

pub fn set_ped_can_lose_props_on_damage_safe(ped: Ped, loseProps: bool, p2: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_LOSE_PROPS_ON_DAMAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_LOSE_PROPS_ON_DAMAGE(ped, loseProps, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_LOSE_PROPS_ON_DAMAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_lose_props_on_damage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_lose_props_on_damage_raw(ped: i32, loseProps: bool, p2: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_LOSE_PROPS_ON_DAMAGE(ped, loseProps, p2)
}

/// ## Parameters
* **ped**:

pub fn get_vehicle_ped_is_entering_safe(ped: Ped) -> NativeResult<Vehicle> {
    
    debug!("Calling native function: GET_VEHICLE_PED_IS_ENTERING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_PED_IS_ENTERING(ped)
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: GET_VEHICLE_PED_IS_ENTERING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_ped_is_entering_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_ped_is_entering_raw(ped: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_PED_IS_ENTERING(ped)
}

/// ## Parameters
* **ped**:

pub fn get_ped_relationship_group_default_hash_safe(ped: Ped) -> NativeResult<u32> {
    
    debug!("Calling native function: GET_PED_RELATIONSHIP_GROUP_DEFAULT_HASH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_RELATIONSHIP_GROUP_DEFAULT_HASH(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_RELATIONSHIP_GROUP_DEFAULT_HASH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_relationship_group_default_hash_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_relationship_group_default_hash_raw(ped: i32) -> u32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_RELATIONSHIP_GROUP_DEFAULT_HASH(ped)
}

/// ## Return value

pub fn _get_num_makeup_colors_safe() -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_NUM_MAKEUP_COLORS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_NUM_MAKEUP_COLORS()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_NUM_MAKEUP_COLORS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_num_makeup_colors_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_num_makeup_colors_raw() -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_NUM_MAKEUP_COLORS()
}

/// ## Parameters
* **ped**: 
* **index**:

pub fn set_ped_group_member_passenger_index_safe(ped: Ped, index: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_GROUP_MEMBER_PASSENGER_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_GROUP_MEMBER_PASSENGER_INDEX(ped, index)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_GROUP_MEMBER_PASSENGER_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_group_member_passenger_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_group_member_passenger_index_raw(ped: i32, index: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_GROUP_MEMBER_PASSENGER_INDEX(ped, index)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_highly_perceptive_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_HIGHLY_PERCEPTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_HIGHLY_PERCEPTIVE(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_HIGHLY_PERCEPTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_highly_perceptive_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_highly_perceptive_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_HIGHLY_PERCEPTIVE(ped, toggle)
}

/// See [`SET_PED_CONFIG_FLAG`](#_0x1913FE4CBF41C463).

pub fn get_ped_config_flag_safe(ped: Ped, flagId: i64, p2: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_PED_CONFIG_FLAG");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_CONFIG_FLAG(ped, flagId, p2)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_CONFIG_FLAG
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_config_flag_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_config_flag_raw(ped: i32, flagId: i64, p2: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_CONFIG_FLAG(ped, flagId, p2)
}

/// ## Parameters
* **sceneID**: 
* **toggle**:

pub fn set_synchronized_scene_hold_last_frame_safe(sceneID: i64, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SYNCHRONIZED_SCENE_HOLD_LAST_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SYNCHRONIZED_SCENE_HOLD_LAST_FRAME(sceneID, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SYNCHRONIZED_SCENE_HOLD_LAST_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_synchronized_scene_hold_last_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_synchronized_scene_hold_last_frame_raw(sceneID: i64, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SYNCHRONIZED_SCENE_HOLD_LAST_FRAME(sceneID, toggle)
}

/// ## Return value

pub fn spawnpoints_is_search_complete_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: SPAWNPOINTS_IS_SEARCH_COMPLETE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SPAWNPOINTS_IS_SEARCH_COMPLETE()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: SPAWNPOINTS_IS_SEARCH_COMPLETE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `spawnpoints_is_search_complete_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn spawnpoints_is_search_complete_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SPAWNPOINTS_IS_SEARCH_COMPLETE()
}

/// ```
Sets the relationship between two groups. This should be called twice (once for each group).  
Relationship types:  
0 = Companion  
1 = Respect  
2 = Like  
3 = Neutral  
4 = Dislike  
5 = Hate  
255 = Pedestrians  
Example:  
PED::SET_RELATIONSHIP_BETWEEN_GROUPS(2, l_1017, 0xA49E591C);  
PED::SET_RELATIONSHIP_BETWEEN_GROUPS(2, 0xA49E591C, l_1017);  
```

pub fn set_relationship_between_groups_safe(relationship: i64, group1: u32, group2: u32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_RELATIONSHIP_BETWEEN_GROUPS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_RELATIONSHIP_BETWEEN_GROUPS(relationship, group1, group2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_RELATIONSHIP_BETWEEN_GROUPS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_relationship_between_groups_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_relationship_between_groups_raw(relationship: i64, group1: u32, group2: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_RELATIONSHIP_BETWEEN_GROUPS(relationship, group1, group2)
}

/// ## Parameters
* **sceneID**: 
* **entity**: 
* **boneIndex**:

pub fn attach_synchronized_scene_to_entity_safe(sceneID: i64, entity: Entity, boneIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: ATTACH_SYNCHRONIZED_SCENE_TO_ENTITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ATTACH_SYNCHRONIZED_SCENE_TO_ENTITY(sceneID, entity, boneIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ATTACH_SYNCHRONIZED_SCENE_TO_ENTITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `attach_synchronized_scene_to_entity_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn attach_synchronized_scene_to_entity_raw(sceneID: i64, entity: i32, boneIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ATTACH_SYNCHRONIZED_SCENE_TO_ENTITY(sceneID, entity, boneIndex)
}

/// ```
Returns true if a synchronized scene is running  
```

pub fn is_synchronized_scene_running_safe(sceneId: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_SYNCHRONIZED_SCENE_RUNNING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_SYNCHRONIZED_SCENE_RUNNING(sceneId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_SYNCHRONIZED_SCENE_RUNNING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_synchronized_scene_running_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_synchronized_scene_running_raw(sceneId: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_SYNCHRONIZED_SCENE_RUNNING(sceneId)
}

/// A getter for [`_SET_PED_EYE_COLOR`](#_0x50B56988B170AFDF).

pub fn _get_ped_eye_color_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_PED_EYE_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_PED_EYE_COLOR(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_PED_EYE_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_ped_eye_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_ped_eye_color_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_PED_EYE_COLOR(ped)
}

/// ## Parameters
* **ped**: The ped handle.

pub fn is_ped_using_any_scenario_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_USING_ANY_SCENARIO");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_USING_ANY_SCENARIO(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_USING_ANY_SCENARIO
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_using_any_scenario_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_using_any_scenario_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_USING_ANY_SCENARIO(ped)
}

/// ## Parameters
* **ped**: 
* **eventId**:

pub fn has_ped_received_event_safe(ped: Ped, eventId: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_PED_RECEIVED_EVENT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_PED_RECEIVED_EVENT(ped, eventId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_PED_RECEIVED_EVENT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_ped_received_event_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_ped_received_event_raw(ped: i32, eventId: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_PED_RECEIVED_EVENT(ped, eventId)
}

/// ## Parameters
* **ped**:

pub fn finalize_head_blend_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: FINALIZE_HEAD_BLEND");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::FINALIZE_HEAD_BLEND(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: FINALIZE_HEAD_BLEND
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `finalize_head_blend_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn finalize_head_blend_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::FINALIZE_HEAD_BLEND(ped)
}

/// ## Parameters
* **ped**: 
* **toggle**:

pub fn set_ped_can_ragdoll_safe(ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_CAN_RAGDOLL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_CAN_RAGDOLL(ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_CAN_RAGDOLL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_can_ragdoll_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_can_ragdoll_raw(ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_CAN_RAGDOLL(ped, toggle)
}

/// ## Parameters
* **ped**:

pub fn clear_ped_last_damage_bone_safe(ped: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_PED_LAST_DAMAGE_BONE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_PED_LAST_DAMAGE_BONE(ped)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_PED_LAST_DAMAGE_BONE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_ped_last_damage_bone_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_ped_last_damage_bone_raw(ped: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_PED_LAST_DAMAGE_BONE(ped)
}

/// ## Parameters
* **ped**: 
* **bone**:

pub fn get_ped_ragdoll_bone_index_safe(ped: Ped, bone: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_RAGDOLL_BONE_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_RAGDOLL_BONE_INDEX(ped, bone)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_RAGDOLL_BONE_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_ragdoll_bone_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_ragdoll_bone_index_raw(ped: i32, bone: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_RAGDOLL_BONE_INDEX(ped, bone)
}

/// Overwrites the minimum time the ped will stay on the ground for after being stunned. Setting this while the ped is stunned will not alter the duration of the current stun but will still effect future stuns.

Passing -1 into the second parameter `minTimeInMs` will reset the modifier, making it use the weapons original `DamageTime` as the stun duration (see `update/update.rpf/common/data/ai/weapons.meta`)

pub fn set_ped_min_ground_time_for_stungun_safe(ped: Ped, minTimeInMs: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PED_MIN_GROUND_TIME_FOR_STUNGUN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PED_MIN_GROUND_TIME_FOR_STUNGUN(ped, minTimeInMs)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PED_MIN_GROUND_TIME_FOR_STUNGUN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ped_min_ground_time_for_stungun_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ped_min_ground_time_for_stungun_raw(ped: i32, minTimeInMs: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PED_MIN_GROUND_TIME_FOR_STUNGUN(ped, minTimeInMs)
}

/// ```
Returns true/false if the ped is/isn't humanoid.  
```

pub fn is_ped_human_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_HUMAN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_HUMAN(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_HUMAN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_human_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_human_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_HUMAN(ped)
}

/// ```
Type equals 0 for male non-dlc, 1 for female non-dlc, 2 for male dlc, and 3 for female dlc.  
```

pub fn get_ped_head_blend_num_heads_safe(type: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_PED_HEAD_BLEND_NUM_HEADS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_HEAD_BLEND_NUM_HEADS(type)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_PED_HEAD_BLEND_NUM_HEADS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_head_blend_num_heads_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_head_blend_num_heads_raw(type: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_HEAD_BLEND_NUM_HEADS(type)
}

/// ## Parameters
* **ped**:

pub fn is_ped_in_flying_vehicle_safe(ped: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_IN_FLYING_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_IN_FLYING_VEHICLE(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_IN_FLYING_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_in_flying_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_in_flying_vehicle_raw(ped: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_IN_FLYING_VEHICLE(ped)
}

/// ## Parameters
* **ped1**: 
* **ped2**:

pub fn is_ped_headtracking_ped_safe(ped1: Ped, ped2: Ped) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PED_HEADTRACKING_PED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PED_HEADTRACKING_PED(ped1, ped2)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PED_HEADTRACKING_PED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_ped_headtracking_ped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_ped_headtracking_ped_raw(ped1: i32, ped2: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PED_HEADTRACKING_PED(ped1, ped2)
}

/// ## Parameters
* **p0**:

pub fn _0xaaa6a3698a69e048_safe(p0: serde_json::Value) -> NativeResult<serde_json::Value> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xAAA6A3698A69E048");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xAAA6A3698A69E048(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0xAAA6A3698A69E048
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xaaa6a3698a69e048_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xaaa6a3698a69e048_raw(p0: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xAAA6A3698A69E048(p0)
}

/// Similar to REGISTER_PEDHEADSHOT but creates a transparent background instead of black.

pub fn register_pedheadshot_transparent_safe(ped: Ped) -> NativeResult<i64> {
    
    debug!("Calling native function: REGISTER_PEDHEADSHOT_TRANSPARENT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REGISTER_PEDHEADSHOT_TRANSPARENT(ped)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: REGISTER_PEDHEADSHOT_TRANSPARENT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `register_pedheadshot_transparent_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn register_pedheadshot_transparent_raw(ped: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REGISTER_PEDHEADSHOT_TRANSPARENT(ped)
}

