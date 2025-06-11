//! VEHICLE native functions
//!
//! Functions for the vehicle category

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
On accelerating, spins the driven wheels with the others braked, so you don't go anywhere.  
```

pub fn set_vehicle_burnout_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_BURNOUT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_BURNOUT(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_BURNOUT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_burnout_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_burnout_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_BURNOUT(vehicle, toggle)
}

/// ```
NativeDB Added Parameter 1: Vehicle vehicle
NativeDB Added Parameter 2: Any p1
```

pub fn _0xdce97bdf8a0eabc8_safe() -> NativeResult<()> {
    
    debug!("Calling native function: _0xDCE97BDF8A0EABC8");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xDCE97BDF8A0EABC8()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xDCE97BDF8A0EABC8
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xdce97bdf8a0eabc8_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xdce97bdf8a0eabc8_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xDCE97BDF8A0EABC8()
}

/// ```
Sets a vehicle on the ground on all wheels.  Returns whether or not the operation was successful.  
```

```
NativeDB Added Parameter 2: float p1
```

pub fn set_vehicle_on_ground_properly_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: SET_VEHICLE_ON_GROUND_PROPERLY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_ON_GROUND_PROPERLY(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: SET_VEHICLE_ON_GROUND_PROPERLY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_on_ground_properly_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_on_ground_properly_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_ON_GROUND_PROPERLY(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn is_playback_using_ai_going_on_for_vehicle_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYBACK_USING_AI_GOING_ON_FOR_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYBACK_USING_AI_GOING_ON_FOR_VEHICLE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYBACK_USING_AI_GOING_ON_FOR_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_playback_using_ai_going_on_for_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_playback_using_ai_going_on_for_vehicle_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYBACK_USING_AI_GOING_ON_FOR_VEHICLE(vehicle)
}

/// ```c
enum eVehicleWheelType
{
    VWT_SPORT = 0,
    VWT_MUSCLE = 1,
    VWT_LOWRIDER = 2,
    VWT_SUV = 3,
    VWT_OFFROAD = 4,
    VWT_TUNER = 5,
    VWT_BIKE = 6,
    VWT_HIEND = 7,
    // Benny's Original
    VWT_SUPERMOD1 = 8,
    // Benny's Bespoke
    VWT_SUPERMOD2 = 9,
    // Open Wheel
    VWT_SUPERMOD3 = 10,
    // Street
    VWT_SUPERMOD4 = 11,
    // Track
    VWT_SUPERMOD5 = 12,
};
```

pub fn get_vehicle_wheel_type_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_WHEEL_TYPE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_WHEEL_TYPE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_WHEEL_TYPE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_wheel_type_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_wheel_type_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_WHEEL_TYPE(vehicle)
}

/// ## Parameters
* **multiplier**:

pub fn set_parked_vehicle_density_multiplier_this_frame_safe(multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PARKED_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PARKED_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME(multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PARKED_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_parked_vehicle_density_multiplier_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_parked_vehicle_density_multiplier_this_frame_raw(multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PARKED_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME(multiplier)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**: 
* **p5**: 
* **p6**:

pub fn add_vehicle_stuck_check_with_warp_safe(p0: serde_json::Value, p1: f32, p2: serde_json::Value, p3: bool, p4: bool, p5: bool, p6: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p2_any_str = serde_json::to_string(&p2)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p2", e)))?;
    let p2_any_str_cstr = std::ffi::CString::new(p2_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p2", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p6_any_str = serde_json::to_string(&p6)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p6", e)))?;
    let p6_any_str_cstr = std::ffi::CString::new(p6_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p6", e)))?;
    
    debug!("Calling native function: ADD_VEHICLE_STUCK_CHECK_WITH_WARP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ADD_VEHICLE_STUCK_CHECK_WITH_WARP(crate::utils::any_to_c_void_ptr(p0), p1, crate::utils::any_to_c_void_ptr(p2), p3, p4, p5, crate::utils::any_to_c_void_ptr(p6))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ADD_VEHICLE_STUCK_CHECK_WITH_WARP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `add_vehicle_stuck_check_with_warp_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn add_vehicle_stuck_check_with_warp_raw(p0: *mut std::os::raw::c_void, p1: f32, p2: *mut std::os::raw::c_void, p3: bool, p4: bool, p5: bool, p6: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ADD_VEHICLE_STUCK_CHECK_WITH_WARP(p0, p1, p2, p3, p4, p5, p6)
}

/// ## Parameters
* **vehicle**:

pub fn get_current_playback_for_vehicle_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_CURRENT_PLAYBACK_FOR_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_CURRENT_PLAYBACK_FOR_VEHICLE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_CURRENT_PLAYBACK_FOR_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_current_playback_for_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_current_playback_for_vehicle_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_CURRENT_PLAYBACK_FOR_VEHICLE(vehicle)
}

/// ```
From the driver's perspective, is the right headlight broken.  
```

pub fn get_is_right_vehicle_headlight_damaged_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_IS_RIGHT_VEHICLE_HEADLIGHT_DAMAGED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_IS_RIGHT_VEHICLE_HEADLIGHT_DAMAGED(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_IS_RIGHT_VEHICLE_HEADLIGHT_DAMAGED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_is_right_vehicle_headlight_damaged_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_is_right_vehicle_headlight_damaged_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_IS_RIGHT_VEHICLE_HEADLIGHT_DAMAGED(vehicle)
}

/// ## Parameters
* **recording**: 
* **script**:

pub fn get_total_duration_of_vehicle_recording_safe(recording: i64) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_TOTAL_DURATION_OF_VEHICLE_RECORDING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_TOTAL_DURATION_OF_VEHICLE_RECORDING(recording)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_TOTAL_DURATION_OF_VEHICLE_RECORDING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_total_duration_of_vehicle_recording_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_total_duration_of_vehicle_recording_raw(recording: i64) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_TOTAL_DURATION_OF_VEHICLE_RECORDING(recording)
}

/// ```
Returns true when in a vehicle, false whilst entering/exiting.  
```

pub fn get_is_vehicle_engine_running_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_IS_VEHICLE_ENGINE_RUNNING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_IS_VEHICLE_ENGINE_RUNNING(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_IS_VEHICLE_ENGINE_RUNNING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_is_vehicle_engine_running_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_is_vehicle_engine_running_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_IS_VEHICLE_ENGINE_RUNNING(vehicle)
}

/// Sets the specified door index open on the passed vehicle. See [`IS_VEHICLE_DOOR_FULLY_OPEN`](#_0x3E933CFF7B111C22).

pub fn set_vehicle_door_open_safe(vehicle: Vehicle, doorIndex: i64, loose: bool, openInstantly: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DOOR_OPEN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DOOR_OPEN(vehicle, doorIndex, loose, openInstantly)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DOOR_OPEN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_door_open_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_door_open_raw(vehicle: i32, doorIndex: i64, loose: bool, openInstantly: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DOOR_OPEN(vehicle, doorIndex, loose, openInstantly)
}

/// ```
Gets the trailer of a vehicle and puts it into the trailer parameter.  
```

pub fn get_vehicle_trailer_vehicle_safe(vehicle: Vehicle, trailer: *mut i32) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_VEHICLE_TRAILER_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_TRAILER_VEHICLE(vehicle, trailer)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_TRAILER_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_trailer_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_trailer_vehicle_raw(vehicle: i32, trailer: *mut i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_TRAILER_VEHICLE(vehicle, trailer)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _0xe851e480b814d4ba_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xE851E480B814D4BA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xE851E480B814D4BA(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xE851E480B814D4BA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xe851e480b814d4ba_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xe851e480b814d4ba_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xE851E480B814D4BA(vehicle, p1)
}

/// ## Parameters
* **vehicle**: 
* **color**:

pub fn _set_vehicle_dashboard_color_safe(vehicle: Vehicle, color: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_DASHBOARD_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_DASHBOARD_COLOR(vehicle, color)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_DASHBOARD_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_dashboard_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_dashboard_color_raw(vehicle: i32, color: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_DASHBOARD_COLOR(vehicle, color)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _0xab04325045427aae_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xAB04325045427AAE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xAB04325045427AAE(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xAB04325045427AAE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xab04325045427aae_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xab04325045427aae_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xAB04325045427AAE(vehicle, p1)
}

/// ## Parameters
* **vehicle**: 
* **speed**:

pub fn set_playback_speed_safe(vehicle: Vehicle, speed: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYBACK_SPEED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYBACK_SPEED(vehicle, speed)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYBACK_SPEED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_playback_speed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_playback_speed_raw(vehicle: i32, speed: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYBACK_SPEED(vehicle, speed)
}

/// ## Parameters
* **cargobob**: The cargobob
* **vehicle**: The vehicle that will be attached
* **vehicleBoneIndex**: A Vehicle bone the hook/magnet should attach to or -1 for none/default [GET_ENTITY_BONE_INDEX_BY_NAME](#_0xFB71170B7E76ACBA)
* **x**: x hook/magnet Offset 
* **y**: y hook/magnet Offset
* **z**: z hook/magnet Offset

pub fn attach_vehicle_to_cargobob_safe(cargobob: Vehicle, vehicle: Vehicle, vehicleBoneIndex: i64, x: f32, y: f32, z: f32) -> NativeResult<()> {
    
    debug!("Calling native function: ATTACH_VEHICLE_TO_CARGOBOB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ATTACH_VEHICLE_TO_CARGOBOB(cargobob, vehicle, vehicleBoneIndex, x, y, z)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ATTACH_VEHICLE_TO_CARGOBOB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `attach_vehicle_to_cargobob_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn attach_vehicle_to_cargobob_raw(cargobob: i32, vehicle: i32, vehicleBoneIndex: i64, x: f32, y: f32, z: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ATTACH_VEHICLE_TO_CARGOBOB(cargobob, vehicle, vehicleBoneIndex, x, y, z)
}

/// ## Parameters
* **vehicle**:

pub fn get_vehicle_layout_hash_safe(vehicle: Vehicle) -> NativeResult<u32> {
    
    debug!("Calling native function: GET_VEHICLE_LAYOUT_HASH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_LAYOUT_HASH(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_LAYOUT_HASH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_layout_hash_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_layout_hash_raw(vehicle: i32) -> u32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_LAYOUT_HASH(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **player**:

pub fn get_vehicle_doors_locked_for_player_safe(vehicle: Vehicle, player: Player) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_VEHICLE_DOORS_LOCKED_FOR_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_DOORS_LOCKED_FOR_PLAYER(vehicle, player)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_DOORS_LOCKED_FOR_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_doors_locked_for_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_doors_locked_for_player_raw(vehicle: i32, player: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_DOORS_LOCKED_FOR_PLAYER(vehicle, player)
}

/// ```
Can be used with GET_TOTAL_DURATION_OF_VEHICLE_RECORDING{_ID} to compute a percentage.
```

pub fn get_time_position_in_recording_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_TIME_POSITION_IN_RECORDING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_TIME_POSITION_IN_RECORDING(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_TIME_POSITION_IN_RECORDING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_time_position_in_recording_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_time_position_in_recording_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_TIME_POSITION_IN_RECORDING(vehicle)
}

/// ```
Sets the wanted state of this vehicle.  
```

pub fn set_vehicle_is_wanted_safe(vehicle: Vehicle, state: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_IS_WANTED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_IS_WANTED(vehicle, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_IS_WANTED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_is_wanted_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_is_wanted_raw(vehicle: i32, state: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_IS_WANTED(vehicle, state)
}

/// ## Parameters
* **id**: 
* **time**:

pub fn get_rotation_of_vehicle_recording_id_at_time_safe(id: i64, time: f32) -> NativeResult<Vector3> {
    
    debug!("Calling native function: GET_ROTATION_OF_VEHICLE_RECORDING_ID_AT_TIME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_ROTATION_OF_VEHICLE_RECORDING_ID_AT_TIME(id, time)
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: GET_ROTATION_OF_VEHICLE_RECORDING_ID_AT_TIME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_rotation_of_vehicle_recording_id_at_time_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_rotation_of_vehicle_recording_id_at_time_raw(id: i64, time: f32) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_ROTATION_OF_VEHICLE_RECORDING_ID_AT_TIME(id, time)
}

/// Does not work for vehicle of type: CBike, CBmx, CBoat, CTrain, CSubmarine.

pub fn _is_vehicle_parachute_active_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_VEHICLE_PARACHUTE_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_VEHICLE_PARACHUTE_ACTIVE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_VEHICLE_PARACHUTE_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_vehicle_parachute_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_vehicle_parachute_active_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_VEHICLE_PARACHUTE_ACTIVE(vehicle)
}

/// ```
Returns the text label of a mod type for a given vehicle  
Use _GET_LABEL_TEXT to get the part name in the game's language  
```

pub fn get_mod_text_label_safe(vehicle: Vehicle, modType: i64, modValue: i64) -> NativeResult<String> {
    
    debug!("Calling native function: GET_MOD_TEXT_LABEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_MOD_TEXT_LABEL(vehicle, modType, modValue)
    };
    
    
    Ok(crate::utils::c_string_to_rust_string(result))
}

/// Raw native function: GET_MOD_TEXT_LABEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_mod_text_label_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_mod_text_label_raw(vehicle: i32, modType: i64, modValue: i64) -> *const std::os::raw::c_char {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_MOD_TEXT_LABEL(vehicle, modType, modValue)
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)

pub fn is_vehicle_door_damaged_safe(veh: Vehicle, doorID: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_DOOR_DAMAGED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_DOOR_DAMAGED(veh, doorID)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_DOOR_DAMAGED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_door_damaged_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_door_damaged_raw(veh: i32, doorID: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_DOOR_DAMAGED(veh, doorID)
}

/// ```
Retracts the hook on the cargobob.  
Note: after you retract it the natives for dropping the hook no longer work  
```

pub fn remove_pick_up_rope_for_cargobob_safe(cargobob: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_PICK_UP_ROPE_FOR_CARGOBOB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_PICK_UP_ROPE_FOR_CARGOBOB(cargobob)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_PICK_UP_ROPE_FOR_CARGOBOB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_pick_up_rope_for_cargobob_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_pick_up_rope_for_cargobob_raw(cargobob: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_PICK_UP_ROPE_FOR_CARGOBOB(cargobob)
}

/// ```
SET_VEHICLE_D*
```

pub fn _set_vehicle_damage_modifier_safe(vehicle: Vehicle, p1: f32) -> NativeResult<serde_json::Value> {
    
    debug!("Calling native function: _SET_VEHICLE_DAMAGE_MODIFIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_DAMAGE_MODIFIER(vehicle, p1)
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _SET_VEHICLE_DAMAGE_MODIFIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_damage_modifier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_damage_modifier_raw(vehicle: i32, p1: f32) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_DAMAGE_MODIFIER(vehicle, p1)
}

/// ## Parameters
* **vehicle**:

pub fn get_vehicle_mod_kit_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_MOD_KIT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MOD_KIT(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MOD_KIT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_mod_kit_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_mod_kit_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MOD_KIT(vehicle)
}

/// ```
Sets the turn signal enabled for a vehicle.  
Set turnSignal to 1 for left light, 0 for right light.  
```

pub fn set_vehicle_indicator_lights_safe(vehicle: Vehicle, turnSignal: i64, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_INDICATOR_LIGHTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_INDICATOR_LIGHTS(vehicle, turnSignal, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_INDICATOR_LIGHTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_indicator_lights_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_indicator_lights_raw(vehicle: i32, turnSignal: i64, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_INDICATOR_LIGHTS(vehicle, turnSignal, toggle)
}

/// ```
From the driver's perspective, is the left headlight broken.  
```

pub fn get_is_left_vehicle_headlight_damaged_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_IS_LEFT_VEHICLE_HEADLIGHT_DAMAGED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_IS_LEFT_VEHICLE_HEADLIGHT_DAMAGED(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_IS_LEFT_VEHICLE_HEADLIGHT_DAMAGED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_is_left_vehicle_headlight_damaged_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_is_left_vehicle_headlight_damaged_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_IS_LEFT_VEHICLE_HEADLIGHT_DAMAGED(vehicle)
}

/// ## Parameters
* **x**: 
* **y**: 
* **z**: 
* **radius**:

pub fn is_any_vehicle_near_point_safe(x: f32, y: f32, z: f32, radius: f32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_ANY_VEHICLE_NEAR_POINT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_ANY_VEHICLE_NEAR_POINT(x, y, z, radius)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_ANY_VEHICLE_NEAR_POINT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_any_vehicle_near_point_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_any_vehicle_near_point_raw(x: f32, y: f32, z: f32, radius: f32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_ANY_VEHICLE_NEAR_POINT(x, y, z, radius)
}

/// ## Parameters
* **vehicle**: 
* **health**:

pub fn _set_heli_tail_rotor_health_safe(vehicle: Vehicle, health: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_HELI_TAIL_ROTOR_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_HELI_TAIL_ROTOR_HEALTH(vehicle, health)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_HELI_TAIL_ROTOR_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_heli_tail_rotor_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_heli_tail_rotor_health_raw(vehicle: i32, health: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_HELI_TAIL_ROTOR_HEALTH(vehicle, health)
}

/// ## Parameters
* **vehicle**: The vehicle to check.
* **seatIndex**: See eSeatPosition declared in [`IS_VEHICLE_SEAT_FREE`](#_0x22AC59A870E6A669).

pub fn is_seat_warp_only_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_SEAT_WARP_ONLY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_SEAT_WARP_ONLY(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_SEAT_WARP_ONLY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_seat_warp_only_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_seat_warp_only_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_SEAT_WARP_ONLY(vehicle)
}

/// ```
Only called once inside main_persitant with the parameters, 0  
```

pub fn set_train_track_spawn_frequency_safe(trackIndex: i64, frequency: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_TRAIN_TRACK_SPAWN_FREQUENCY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_TRAIN_TRACK_SPAWN_FREQUENCY(trackIndex, frequency)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_TRAIN_TRACK_SPAWN_FREQUENCY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_train_track_spawn_frequency_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_train_track_spawn_frequency_raw(trackIndex: i64, frequency: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_TRAIN_TRACK_SPAWN_FREQUENCY(trackIndex, frequency)
}

/// ```
Finds the vehicle that is carrying this entity with a handler frame.
The model of the entity must be prop_contr_03b_ld or the function will return 0.
```

pub fn _find_vehicle_carrying_this_entity_safe(entity: Entity) -> NativeResult<Vehicle> {
    
    debug!("Calling native function: _FIND_VEHICLE_CARRYING_THIS_ENTITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_FIND_VEHICLE_CARRYING_THIS_ENTITY(entity)
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: _FIND_VEHICLE_CARRYING_THIS_ENTITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_find_vehicle_carrying_this_entity_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _find_vehicle_carrying_this_entity_raw(entity: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_FIND_VEHICLE_CARRYING_THIS_ENTITY(entity)
}

/// Queries whether the control panels of a plane are intact. This native is used to determine the operational status of a plane's cockpit controls, which can affect the plane's flyability.

pub fn are_plane_control_panels_intact_safe(vehicle: Vehicle, checkForZeroHealth: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: ARE_PLANE_CONTROL_PANELS_INTACT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ARE_PLANE_CONTROL_PANELS_INTACT(vehicle, checkForZeroHealth)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: ARE_PLANE_CONTROL_PANELS_INTACT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `are_plane_control_panels_intact_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn are_plane_control_panels_intact_raw(vehicle: i32, checkForZeroHealth: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ARE_PLANE_CONTROL_PANELS_INTACT(vehicle, checkForZeroHealth)
}

/// ## Parameters
* **vehicle**:

pub fn set_car_boot_open_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CAR_BOOT_OPEN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CAR_BOOT_OPEN(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CAR_BOOT_OPEN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_car_boot_open_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_car_boot_open_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CAR_BOOT_OPEN(vehicle)
}

/// ```
It switch to highbeam when p1 is set to true.  
```

pub fn set_vehicle_fullbeam_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_FULLBEAM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_FULLBEAM(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_FULLBEAM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_fullbeam_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_fullbeam_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_FULLBEAM(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_disable_vehicle_petrol_tank_fires_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_DISABLE_VEHICLE_PETROL_TANK_FIRES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DISABLE_VEHICLE_PETROL_TANK_FIRES(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DISABLE_VEHICLE_PETROL_TANK_FIRES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_disable_vehicle_petrol_tank_fires_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_disable_vehicle_petrol_tank_fires_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DISABLE_VEHICLE_PETROL_TANK_FIRES(vehicle, toggle)
}

/// ```
Returns an int  
Vehicle Classes:  
0: Compacts  
1: Sedans  
2: SUVs  
3: Coupes  
4: Muscle  
5: Sports Classics  
6: Sports  
7: Super  
8: Motorcycles  
9: Off-road  
10: Industrial  
11: Utility  
12: Vans  
13: Cycles  
14: Boats  
15: Helicopters  
16: Planes  
17: Service  
18: Emergency  
19: Military  
20: Commercial  
21: Trains  
22: Open Wheel
char buffer[128];  
std::sprintf(buffer, "VEH_CLASS_%i", VEHICLE::GET_VEHICLE_CLASS(vehicle));  
char* className = UI::_GET_LABEL_TEXT(buffer);  
```

pub fn get_vehicle_class_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_CLASS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_CLASS(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_CLASS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_class_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_class_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_CLASS(vehicle)
}

/// Usex in decompiled scripts in combination with [`_GET_VEHICLE_SUSPENSION_BOUNDS`](#_0xDF7E3EEB29642C38).

NativeDB Introduced: v1180

```
// Example from fm_bj_race_controller.c
if (!VEHICLE::_0x51F30DB60626A20E(uParam0->f_26, uParam0->f_12.f_3, uParam0->f_12, 2, 1) && !func_282(uParam0->f_6))
{
    VEHICLE::_GET_VEHICLE_SUSPENSION_BOUNDS(*uParam0, &vVar15, &uVar16);
    VEHICLE::_GET_VEHICLE_SUSPENSION_BOUNDS(uParam0->f_26, &vVar17, &uVar18);
    fVar19 = SYSTEM::VDIST2(0f, 0f, vVar15.z, 0f, 0f, vVar17.z);
    uParam0->f_12.f_3.f_2 = (uParam0->f_12.f_3.f_2 + fVar19);
    if (!VEHICLE::_0x51F30DB60626A20E(uParam0->f_26, uParam0->f_12.f_3, uParam0->f_12, 2, 1))
    {
        uParam0->f_12.f_3 = { uParam0->f_6 };
        uParam0->f_12 = { uParam0->f_9 };
    }
}
```

pub fn _0x51f30db60626a20e_safe(vehicle: Vehicle, x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32, p7: i64, p8: serde_json::Value) -> NativeResult<bool> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p8_any_str = serde_json::to_string(&p8)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p8", e)))?;
    let p8_any_str_cstr = std::ffi::CString::new(p8_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p8", e)))?;
    
    debug!("Calling native function: _0x51F30DB60626A20E");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x51F30DB60626A20E(vehicle, x, y, z, rotX, rotY, rotZ, p7, crate::utils::any_to_c_void_ptr(p8))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0x51F30DB60626A20E
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x51f30db60626a20e_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x51f30db60626a20e_raw(vehicle: i32, x: f32, y: f32, z: f32, rotX: f32, rotY: f32, rotZ: f32, p7: i64, p8: *mut std::os::raw::c_void) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x51F30DB60626A20E(vehicle, x, y, z, rotX, rotY, rotZ, p7, p8)
}

/// ## Parameters
* **vehicle**: 
* **p1**: 
* **p2**:

pub fn _0x9bddc73cc6a115d4_safe(vehicle: Vehicle, p1: bool, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x9BDDC73CC6A115D4");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x9BDDC73CC6A115D4(vehicle, p1, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x9BDDC73CC6A115D4
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x9bddc73cc6a115d4_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x9bddc73cc6a115d4_raw(vehicle: i32, p1: bool, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x9BDDC73CC6A115D4(vehicle, p1, p2)
}

/// ## Parameters
* **vehicle**:

pub fn is_any_entity_attached_to_handler_frame_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_ANY_ENTITY_ATTACHED_TO_HANDLER_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_ANY_ENTITY_ATTACHED_TO_HANDLER_FRAME(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_ANY_ENTITY_ATTACHED_TO_HANDLER_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_any_entity_attached_to_handler_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_any_entity_attached_to_handler_frame_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_ANY_ENTITY_ATTACHED_TO_HANDLER_FRAME(vehicle)
}

/// ```
returns a string which is the codename of the vehicle's currently selected primary color  
p1 is always 0  
```

pub fn get_vehicle_mod_color_1_name_safe(vehicle: Vehicle, p1: bool) -> NativeResult<String> {
    
    debug!("Calling native function: GET_VEHICLE_MOD_COLOR_1_NAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MOD_COLOR_1_NAME(vehicle, p1)
    };
    
    
    Ok(crate::utils::c_string_to_rust_string(result))
}

/// Raw native function: GET_VEHICLE_MOD_COLOR_1_NAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_mod_color_1_name_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_mod_color_1_name_raw(vehicle: i32, p1: bool) -> *const std::os::raw::c_char {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MOD_COLOR_1_NAME(vehicle, p1)
}

/// Last named native above this one is `TRACK_VEHICLE_VISIBILITY` and first named native below is `UNCUFF_PED`. 
Unknown what it does, couldn't find good examples in the decompiled scripts.

pub fn _0x725012a415dba050_safe(p0: serde_json::Value, p1: serde_json::Value, p2: serde_json::Value) -> NativeResult<serde_json::Value> {
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
    
    debug!("Calling native function: _0x725012A415DBA050");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x725012A415DBA050(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1), crate::utils::any_to_c_void_ptr(p2))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0x725012A415DBA050
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x725012a415dba050_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x725012a415dba050_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void, p2: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x725012A415DBA050(p0, p1, p2)
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)

pub fn set_vehicle_door_latched_safe(vehicle: Vehicle, doorIndex: i64, forceClose: bool, lock: bool, p4: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DOOR_LATCHED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DOOR_LATCHED(vehicle, doorIndex, forceClose, lock, p4)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DOOR_LATCHED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_door_latched_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_door_latched_raw(vehicle: i32, doorIndex: i64, forceClose: bool, lock: bool, p4: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DOOR_LATCHED(vehicle, doorIndex, forceClose, lock, p4)
}

/// ## Parameters
* **vehicle**: The vehicle to set the plate for
* **plateText**: The text to set the plate to, 8 chars maximum

pub fn set_vehicle_number_plate_text_safe(vehicle: Vehicle, plateText: String) -> NativeResult<()> {
    let plateText_cstr = std::ffi::CString::new(plateText.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "plateText", e)))?;
    
    debug!("Calling native function: SET_VEHICLE_NUMBER_PLATE_TEXT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_NUMBER_PLATE_TEXT(vehicle, crate::utils::rust_to_c_string(plateText))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_NUMBER_PLATE_TEXT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_number_plate_text_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_number_plate_text_raw(vehicle: i32, plateText: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_NUMBER_PLATE_TEXT(vehicle, plateText)
}

/// ## Parameters
* **vehicle**: 
* **front**:

pub fn is_vehicle_bumper_broken_off_safe(vehicle: Vehicle, front: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_BUMPER_BROKEN_OFF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_BUMPER_BROKEN_OFF(vehicle, front)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_BUMPER_BROKEN_OFF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_bumper_broken_off_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_bumper_broken_off_raw(vehicle: i32, front: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_BUMPER_BROKEN_OFF(vehicle, front)
}

/// ## Parameters
* **vehicle**: 
* **angle**:

pub fn set_vehicle_flight_nozzle_position_immediate_safe(vehicle: Vehicle, angle: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_FLIGHT_NOZZLE_POSITION_IMMEDIATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_FLIGHT_NOZZLE_POSITION_IMMEDIATE(vehicle, angle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_FLIGHT_NOZZLE_POSITION_IMMEDIATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_flight_nozzle_position_immediate_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_flight_nozzle_position_immediate_raw(vehicle: i32, angle: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_FLIGHT_NOZZLE_POSITION_IMMEDIATE(vehicle, angle)
}

/// See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).

pub fn roll_up_window_safe(vehicle: Vehicle, windowIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: ROLL_UP_WINDOW");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ROLL_UP_WINDOW(vehicle, windowIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ROLL_UP_WINDOW
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `roll_up_window_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn roll_up_window_raw(vehicle: i32, windowIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ROLL_UP_WINDOW(vehicle, windowIndex)
}

/// _CLEAR_VEHICLE_PHONE_EXPLOSIVE_DEVICE native function

pub fn _clear_vehicle_phone_explosive_device_safe() -> NativeResult<()> {
    
    debug!("Calling native function: _CLEAR_VEHICLE_PHONE_EXPLOSIVE_DEVICE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_CLEAR_VEHICLE_PHONE_EXPLOSIVE_DEVICE()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _CLEAR_VEHICLE_PHONE_EXPLOSIVE_DEVICE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_clear_vehicle_phone_explosive_device_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _clear_vehicle_phone_explosive_device_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_CLEAR_VEHICLE_PHONE_EXPLOSIVE_DEVICE()
}

/// ## Parameters
* **toggle**:

pub fn set_random_boats_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_RANDOM_BOATS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_RANDOM_BOATS(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_RANDOM_BOATS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_random_boats_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_random_boats_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_RANDOM_BOATS(toggle)
}

/// ## Parameters
* **vehicle**: 
* **active**:

pub fn _set_vehicle_rocket_boost_active_safe(vehicle: Vehicle, active: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_ROCKET_BOOST_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_ROCKET_BOOST_ACTIVE(vehicle, active)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_ROCKET_BOOST_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_rocket_boost_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_rocket_boost_active_raw(vehicle: i32, active: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_ROCKET_BOOST_ACTIVE(vehicle, active)
}

/// ```
NativeDB Introduced: v1290
```

pub fn _0xb68cfaf83a02768d_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xB68CFAF83A02768D");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xB68CFAF83A02768D(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xB68CFAF83A02768D
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xb68cfaf83a02768d_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xb68cfaf83a02768d_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xB68CFAF83A02768D(vehicle, toggle)
}

/// ```
Deletes a vehicle.  
The vehicle must be a mission entity to delete, so call this before deleting: SET_ENTITY_AS_MISSION_ENTITY(vehicle, true, true);  
eg how to use:  
SET_ENTITY_AS_MISSION_ENTITY(vehicle, true, true);  
DELETE_VEHICLE(&vehicle);  
Deletes the specified vehicle, then sets the handle pointed to by the pointer to NULL.  
```

pub fn delete_vehicle_safe(vehicle: *mut i32) -> NativeResult<()> {
    
    debug!("Calling native function: DELETE_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DELETE_VEHICLE(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DELETE_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `delete_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn delete_vehicle_raw(vehicle: *mut i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DELETE_VEHICLE(vehicle)
}

/// ```
GET_VEHICLE_MODEL_*
9.8 * thrust if air vehicle, else 0.38 + drive force?
```

pub fn _get_vehicle_model_estimated_agility_safe(modelHash: u32) -> NativeResult<f32> {
    
    debug!("Calling native function: _GET_VEHICLE_MODEL_ESTIMATED_AGILITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_MODEL_ESTIMATED_AGILITY(modelHash)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_VEHICLE_MODEL_ESTIMATED_AGILITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_model_estimated_agility_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_model_estimated_agility_raw(modelHash: u32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_MODEL_ESTIMATED_AGILITY(modelHash)
}

/// SET_VEHICLE_HAS_UNBREAKABLE_LIGHTS native function

pub fn set_vehicle_has_unbreakable_lights_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_HAS_UNBREAKABLE_LIGHTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_HAS_UNBREAKABLE_LIGHTS(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_HAS_UNBREAKABLE_LIGHTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_has_unbreakable_lights_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_has_unbreakable_lights_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_HAS_UNBREAKABLE_LIGHTS(vehicle, toggle)
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)

pub fn set_vehicle_door_broken_safe(vehicle: Vehicle, doorIndex: i64, deleteDoor: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DOOR_BROKEN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DOOR_BROKEN(vehicle, doorIndex, deleteDoor)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DOOR_BROKEN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_door_broken_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_door_broken_raw(vehicle: i32, doorIndex: i64, deleteDoor: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DOOR_BROKEN(vehicle, doorIndex, deleteDoor)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn _set_hydraulic_raised_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_HYDRAULIC_RAISED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_HYDRAULIC_RAISED(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_HYDRAULIC_RAISED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_hydraulic_raised_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_hydraulic_raised_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_HYDRAULIC_RAISED(vehicle, toggle)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x1312ddd8385aee4e_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x1312DDD8385AEE4E");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x1312DDD8385AEE4E(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x1312DDD8385AEE4E
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x1312ddd8385aee4e_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x1312ddd8385aee4e_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x1312DDD8385AEE4E(p0, p1)
}

/// ## Parameters
* **vehicle**:

pub fn clear_vehicle_route_history_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_VEHICLE_ROUTE_HISTORY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_VEHICLE_ROUTE_HISTORY(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_VEHICLE_ROUTE_HISTORY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_vehicle_route_history_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_vehicle_route_history_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_VEHICLE_ROUTE_HISTORY(vehicle)
}

/// ## Parameters
* **cargobob**: 
* **vehicle**:

pub fn detach_vehicle_from_cargobob_safe(cargobob: Vehicle, vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: DETACH_VEHICLE_FROM_CARGOBOB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DETACH_VEHICLE_FROM_CARGOBOB(cargobob, vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DETACH_VEHICLE_FROM_CARGOBOB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `detach_vehicle_from_cargobob_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn detach_vehicle_from_cargobob_raw(cargobob: i32, vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DETACH_VEHICLE_FROM_CARGOBOB(cargobob, vehicle)
}

/// ```
p1 is always 0  
```

pub fn set_mission_train_as_no_longer_needed_safe(train: *mut i32, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_MISSION_TRAIN_AS_NO_LONGER_NEEDED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_MISSION_TRAIN_AS_NO_LONGER_NEEDED(train, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_MISSION_TRAIN_AS_NO_LONGER_NEEDED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_mission_train_as_no_longer_needed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_mission_train_as_no_longer_needed_raw(train: *mut i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_MISSION_TRAIN_AS_NO_LONGER_NEEDED(train, p1)
}

/// ```
Max value is 32767
```

pub fn set_vehicle_extended_removal_range_safe(vehicle: Vehicle, range: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_EXTENDED_REMOVAL_RANGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_EXTENDED_REMOVAL_RANGE(vehicle, range)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_EXTENDED_REMOVAL_RANGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_extended_removal_range_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_extended_removal_range_raw(vehicle: i32, range: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_EXTENDED_REMOVAL_RANGE(vehicle, range)
}

/// ```
Setting this to false, makes the specified vehicle to where if you press Y your character doesn't even attempt the animation to enter the vehicle. Hence it's not considered aka ignored.  
```

pub fn set_vehicle_is_considered_by_player_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_IS_CONSIDERED_BY_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_IS_CONSIDERED_BY_PLAYER(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_IS_CONSIDERED_BY_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_is_considered_by_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_is_considered_by_player_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_IS_CONSIDERED_BY_PLAYER(vehicle, toggle)
}

/// Enables spawning random trains on the preset tracks. 

Requires [`SWITCH_TRAIN_TRACK`](#_0xFD813BB7DB977F20) and [`SET_TRAIN_TRACK_SPAWN_FREQUENCY`](#_0x21973BBF8D17EDFA) to be set.

pub fn set_random_trains_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_RANDOM_TRAINS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_RANDOM_TRAINS(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_RANDOM_TRAINS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_random_trains_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_random_trains_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_RANDOM_TRAINS(toggle)
}

/// ## Parameters
* **vehicle**:

pub fn get_vehicle_max_number_of_passengers_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_MAX_NUMBER_OF_PASSENGERS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MAX_NUMBER_OF_PASSENGERS(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MAX_NUMBER_OF_PASSENGERS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_max_number_of_passengers_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_max_number_of_passengers_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MAX_NUMBER_OF_PASSENGERS(vehicle)
}

/// ```
For a full enum, see here : pastebin.com/i2GGAjY0
char buffer[128];
std::sprintf(buffer, "VEH_CLASS_%i", VEHICLE::GET_VEHICLE_CLASS_FROM_NAME (hash));
const char* className = HUD::_GET_LABEL_TEXT(buffer);
```

pub fn get_vehicle_class_from_name_safe(modelHash: u32) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_CLASS_FROM_NAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_CLASS_FROM_NAME(modelHash)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_CLASS_FROM_NAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_class_from_name_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_class_from_name_raw(modelHash: u32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_CLASS_FROM_NAME(modelHash)
}

/// ## Parameters
* **vehicle**:

pub fn _get_is_wheels_lowered_state_active_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _GET_IS_WHEELS_LOWERED_STATE_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_IS_WHEELS_LOWERED_STATE_ACTIVE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_IS_WHEELS_LOWERED_STATE_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_is_wheels_lowered_state_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_is_wheels_lowered_state_active_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_IS_WHEELS_LOWERED_STATE_ACTIVE(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn remove_vehicle_upsidedown_check_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_VEHICLE_UPSIDEDOWN_CHECK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_VEHICLE_UPSIDEDOWN_CHECK(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_VEHICLE_UPSIDEDOWN_CHECK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_vehicle_upsidedown_check_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_vehicle_upsidedown_check_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_VEHICLE_UPSIDEDOWN_CHECK(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn get_vehicle_window_tint_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_WINDOW_TINT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_WINDOW_TINT(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_WINDOW_TINT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_window_tint_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_window_tint_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_WINDOW_TINT(vehicle)
}

/// SET_HELI_TAIL_BOOM_CAN_BREAK_OFF native function

pub fn set_heli_tail_boom_can_break_off_safe(heli: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_HELI_TAIL_BOOM_CAN_BREAK_OFF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_HELI_TAIL_BOOM_CAN_BREAK_OFF(heli, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_HELI_TAIL_BOOM_CAN_BREAK_OFF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_heli_tail_boom_can_break_off_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_heli_tail_boom_can_break_off_raw(heli: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_HELI_TAIL_BOOM_CAN_BREAK_OFF(heli, toggle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_undriveable_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_UNDRIVEABLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_UNDRIVEABLE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_UNDRIVEABLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_undriveable_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_undriveable_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_UNDRIVEABLE(vehicle, toggle)
}

/// Despite its name, this works on Helicopters and Planes.

Sets the speed of the helicopter blades to full speed.

This is equivalent to calling `SetHeliBladesSpeed(vehicleHandle, 1.0);`

pub fn set_heli_blades_full_speed_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_HELI_BLADES_FULL_SPEED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_HELI_BLADES_FULL_SPEED(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_HELI_BLADES_FULL_SPEED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_heli_blades_full_speed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_heli_blades_full_speed_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_HELI_BLADES_FULL_SPEED(vehicle)
}

/// Transforms the `stormberg` to its "water vehicle" variant. If the vehicle is already in that state then the vehicle transformation audio will still play, but the vehicle won't change at all.

pub fn transform_to_submarine_safe(vehicle: Vehicle, instantly: bool) -> NativeResult<()> {
    
    debug!("Calling native function: TRANSFORM_TO_SUBMARINE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::TRANSFORM_TO_SUBMARINE(vehicle, instantly)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: TRANSFORM_TO_SUBMARINE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `transform_to_submarine_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn transform_to_submarine_raw(vehicle: i32, instantly: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::TRANSFORM_TO_SUBMARINE(vehicle, instantly)
}

/// ## Parameters
* **vehicle**: 
* **state**:

pub fn set_cargobob_pickup_rope_type_safe(vehicle: Vehicle, state: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CARGOBOB_PICKUP_ROPE_TYPE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CARGOBOB_PICKUP_ROPE_TYPE(vehicle, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CARGOBOB_PICKUP_ROPE_TYPE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_cargobob_pickup_rope_type_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_cargobob_pickup_rope_type_raw(vehicle: i32, state: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CARGOBOB_PICKUP_ROPE_TYPE(vehicle, state)
}

/// ```
Adds some kind of shadow to the vehicle.
-1 disables the effect.
DISABLE_*
```

pub fn _set_vehicle_shadow_effect_safe(vehicle: Vehicle, p1: i64, p2: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_SHADOW_EFFECT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_SHADOW_EFFECT(vehicle, p1, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_SHADOW_EFFECT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_shadow_effect_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_shadow_effect_raw(vehicle: i32, p1: i64, p2: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_SHADOW_EFFECT(vehicle, p1, p2)
}

/// Set a specific offset for helis chasing target in combat

```
NativeDB Introduced: v1180
```

pub fn set_heli_combat_offset_safe(vehicle: Vehicle, x: f32, y: f32, z: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_HELI_COMBAT_OFFSET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_HELI_COMBAT_OFFSET(vehicle, x, y, z)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_HELI_COMBAT_OFFSET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_heli_combat_offset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_heli_combat_offset_raw(vehicle: i32, x: f32, y: f32, z: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_HELI_COMBAT_OFFSET(vehicle, x, y, z)
}

/// ## Parameters
* **vehicle**: 
* **r**: 
* **g**: 
* **b**:

pub fn get_vehicle_custom_primary_colour_safe(vehicle: Vehicle, r: *mut i64, g: *mut i64, b: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_VEHICLE_CUSTOM_PRIMARY_COLOUR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_CUSTOM_PRIMARY_COLOUR(vehicle, r, g, b)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_VEHICLE_CUSTOM_PRIMARY_COLOUR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_custom_primary_colour_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_custom_primary_colour_raw(vehicle: i32, r: *mut i64, g: *mut i64, b: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_CUSTOM_PRIMARY_COLOUR(vehicle, r, g, b)
}

/// Use [_SET_VEHICLE_HEADLIGHTS_COLOUR](#_0xE41033B25D003A07) to set the headlights color for the vehicle.

You must enable xenon headlights for this native to work properly.

```c
enum eHeadlightColors {
    Default = 255,
    White = 0,
    Blue = 1,
    ElectricBlue = 2,
    MintGreen = 3,
    LimeGreen = 4,
    Yellow = 5,
    GoldenShower = 6,
    Orange = 7,
    Red = 8,
    PonyPink = 9,
    HotPink = 10,
    Purple = 11,
    Blacklight = 12
}
```

pub fn _get_vehicle_xenon_lights_color_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_VEHICLE_XENON_LIGHTS_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_XENON_LIGHTS_COLOR(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_VEHICLE_XENON_LIGHTS_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_xenon_lights_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_xenon_lights_color_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_XENON_LIGHTS_COLOR(vehicle)
}

/// ```
If set to true, vehicle will not take crash damage, but is still susceptible to damage from bullets and explosives  
```

pub fn set_vehicle_strong_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_STRONG");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_STRONG(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_STRONG
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_strong_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_strong_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_STRONG(vehicle, toggle)
}

/// Enables individual propeller on a propeller plane. This native is the inverse of [`DISABLE_INDIVIDUAL_PLANE_PROPELLER`](#_0x500873A45724C863).

```
NativeDB Introduced: v3407
```

pub fn _enable_individual_plane_propeller_safe(plane: Vehicle, propeller: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _ENABLE_INDIVIDUAL_PLANE_PROPELLER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_ENABLE_INDIVIDUAL_PLANE_PROPELLER(plane, propeller)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _ENABLE_INDIVIDUAL_PLANE_PROPELLER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_enable_individual_plane_propeller_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _enable_individual_plane_propeller_raw(plane: i32, propeller: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_ENABLE_INDIVIDUAL_PLANE_PROPELLER(plane, propeller)
}

/// ```
in the decompiled scripts, seems to be always called on the vehicle right after being attached to a trailer.
```

pub fn set_trailer_legs_raised_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_TRAILER_LEGS_RAISED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_TRAILER_LEGS_RAISED(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_TRAILER_LEGS_RAISED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_trailer_legs_raised_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_trailer_legs_raised_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_TRAILER_LEGS_RAISED(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_has_been_driven_flag_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_HAS_BEEN_DRIVEN_FLAG");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_HAS_BEEN_DRIVEN_FLAG(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_HAS_BEEN_DRIVEN_FLAG
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_has_been_driven_flag_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_has_been_driven_flag_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_HAS_BEEN_DRIVEN_FLAG(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_gravity_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_GRAVITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_GRAVITY(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_GRAVITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_gravity_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_gravity_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_GRAVITY(vehicle, toggle)
}

/// ```c
// eVehicleModType values modified to conform to script native reorganization (see 0x140D25327 in 1604).
enum eVehicleModType
{
	VMT_SPOILER = 0,
	VMT_BUMPER_F = 1,
	VMT_BUMPER_R = 2,
	VMT_SKIRT = 3,
	VMT_EXHAUST = 4,
	VMT_CHASSIS = 5,
	VMT_GRILL = 6,
	VMT_BONNET = 7,
	VMT_WING_L = 8,
	VMT_WING_R = 9,
	VMT_ROOF = 10,
	VMT_ENGINE = 11,
	VMT_BRAKES = 12,
	VMT_GEARBOX = 13,
	VMT_HORN = 14,
	VMT_SUSPENSION = 15,
	VMT_ARMOUR = 16,
	VMT_NITROUS = 17,
	VMT_TURBO = 18,
	VMT_SUBWOOFER = 19,
	VMT_TYRE_SMOKE = 20,
	VMT_HYDRAULICS = 21,
	VMT_XENON_LIGHTS = 22,
	VMT_WHEELS = 23,
	VMT_WHEELS_REAR_OR_HYDRAULICS = 24,
	VMT_PLTHOLDER = 25,
	VMT_PLTVANITY = 26,
	VMT_INTERIOR1 = 27,
	VMT_INTERIOR2 = 28,
	VMT_INTERIOR3 = 29,
	VMT_INTERIOR4 = 30,
	VMT_INTERIOR5 = 31,
	VMT_SEATS = 32,
	VMT_STEERING = 33,
	VMT_KNOB = 34,
	VMT_PLAQUE = 35,
	VMT_ICE = 36,
	VMT_TRUNK = 37,
	VMT_HYDRO = 38,
	VMT_ENGINEBAY1 = 39,
	VMT_ENGINEBAY2 = 40,
	VMT_ENGINEBAY3 = 41,
	VMT_CHASSIS2 = 42,
	VMT_CHASSIS3 = 43,
	VMT_CHASSIS4 = 44,
	VMT_CHASSIS5 = 45,
	VMT_DOOR_L = 46,
	VMT_DOOR_R = 47,
	VMT_LIVERY_MOD = 48,
	VMT_LIGHTBAR = 49,
};
```

pub fn set_vehicle_mod_safe(vehicle: Vehicle, modType: i64, modIndex: i64, customTires: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_MOD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_MOD(vehicle, modType, modIndex, customTires)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_MOD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_mod_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_mod_raw(vehicle: i32, modType: i64, modIndex: i64, customTires: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_MOD(vehicle, modType, modIndex, customTires)
}

/// STOP_ALL_GARAGE_ACTIVITY native function

pub fn stop_all_garage_activity_safe() -> NativeResult<()> {
    
    debug!("Calling native function: STOP_ALL_GARAGE_ACTIVITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::STOP_ALL_GARAGE_ACTIVITY()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: STOP_ALL_GARAGE_ACTIVITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `stop_all_garage_activity_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn stop_all_garage_activity_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::STOP_ALL_GARAGE_ACTIVITY()
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn explode_vehicle_in_cutscene_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: EXPLODE_VEHICLE_IN_CUTSCENE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::EXPLODE_VEHICLE_IN_CUTSCENE(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: EXPLODE_VEHICLE_IN_CUTSCENE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `explode_vehicle_in_cutscene_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn explode_vehicle_in_cutscene_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::EXPLODE_VEHICLE_IN_CUTSCENE(vehicle, p1)
}

/// Apply damage to vehicle at a location. Location is relative to vehicle model (not world).
Radius of effect damage applied in a sphere at impact location
When `focusOnModel` set to `true`, the damage sphere will travel towards the vehicle from the given point, thus guaranteeing an impact

pub fn set_vehicle_damage_safe(vehicle: Vehicle, xOffset: f32, yOffset: f32, zOffset: f32, damage: f32, radius: f32, focusOnModel: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DAMAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DAMAGE(vehicle, xOffset, yOffset, zOffset, damage, radius, focusOnModel)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DAMAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_damage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_damage_raw(vehicle: i32, xOffset: f32, yOffset: f32, zOffset: f32, damage: f32, radius: f32, focusOnModel: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DAMAGE(vehicle, xOffset, yOffset, zOffset, damage, radius, focusOnModel)
}

/// ```
To check if the model is an amphibious car, see gtaforums.com/topic/717612-v-scriptnative-documentation-and-research/page-33#entry1069317363 (for build 944 and above only!)  
```

pub fn is_this_model_a_car_safe(model: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_THIS_MODEL_A_CAR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_THIS_MODEL_A_CAR(model)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_THIS_MODEL_A_CAR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_this_model_a_car_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_this_model_a_car_raw(model: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_THIS_MODEL_A_CAR(model)
}

/// SET_BOAT_ANCHOR native function

pub fn set_boat_anchor_safe(boat: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_BOAT_ANCHOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_BOAT_ANCHOR(boat, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_BOAT_ANCHOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_boat_anchor_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_boat_anchor_raw(boat: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_BOAT_ANCHOR(boat, toggle)
}

/// ```
Also includes some "turnOffBones" when vehicle mods are installed.
```

pub fn _get_vehicle_number_of_broken_off_bones_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_VEHICLE_NUMBER_OF_BROKEN_OFF_BONES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_NUMBER_OF_BROKEN_OFF_BONES(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_VEHICLE_NUMBER_OF_BROKEN_OFF_BONES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_number_of_broken_off_bones_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_number_of_broken_off_bones_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_NUMBER_OF_BROKEN_OFF_BONES(vehicle)
}

/// ```
NativeDB Introduced: v1604
```

pub fn _set_disable_vehicle_unk_2_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_DISABLE_VEHICLE_UNK_2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_DISABLE_VEHICLE_UNK_2(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_DISABLE_VEHICLE_UNK_2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_disable_vehicle_unk_2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_disable_vehicle_unk_2_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_DISABLE_VEHICLE_UNK_2(toggle)
}

/// ## Parameters
* **vehicle**: 
* **ped**: 
* **toggle**:

pub fn set_vehicle_timed_explosion_safe(vehicle: Vehicle, ped: Ped, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_TIMED_EXPLOSION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_TIMED_EXPLOSION(vehicle, ped, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_TIMED_EXPLOSION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_timed_explosion_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_timed_explosion_raw(vehicle: i32, ped: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_TIMED_EXPLOSION(vehicle, ped, toggle)
}

/// Gets hash related to task happening with seat index
Native name: GET_I*

pub fn _0xa01bc64dd4bfbbac_safe(vehicle: Vehicle, seatIndex: i64) -> NativeResult<u32> {
    
    debug!("Calling native function: _0xA01BC64DD4BFBBAC");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xA01BC64DD4BFBBAC(vehicle, seatIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0xA01BC64DD4BFBBAC
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xa01bc64dd4bfbbac_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xa01bc64dd4bfbbac_raw(vehicle: i32, seatIndex: i64) -> u32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xA01BC64DD4BFBBAC(vehicle, seatIndex)
}

/// ## Parameters
* **p0**: 
* **modType**: Refer to eVehicleModType in [`SET_VEHICLE_MOD`](#_0x6AF0636DDEDCB6DD).
* **p2**:

pub fn preload_vehicle_mod_safe(p0: serde_json::Value, modType: i64, p2: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p2_any_str = serde_json::to_string(&p2)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p2", e)))?;
    let p2_any_str_cstr = std::ffi::CString::new(p2_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p2", e)))?;
    
    debug!("Calling native function: PRELOAD_VEHICLE_MOD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::PRELOAD_VEHICLE_MOD(crate::utils::any_to_c_void_ptr(p0), modType, crate::utils::any_to_c_void_ptr(p2))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: PRELOAD_VEHICLE_MOD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `preload_vehicle_mod_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn preload_vehicle_mod_raw(p0: *mut std::os::raw::c_void, modType: i64, p2: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::PRELOAD_VEHICLE_MOD(p0, modType, p2)
}

/// ```
NativeDB Introduced: v323
```

pub fn disable_individual_plane_propeller_safe(vehicle: Vehicle, propeller: i64) -> NativeResult<()> {
    
    debug!("Calling native function: DISABLE_INDIVIDUAL_PLANE_PROPELLER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DISABLE_INDIVIDUAL_PLANE_PROPELLER(vehicle, propeller)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DISABLE_INDIVIDUAL_PLANE_PROPELLER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `disable_individual_plane_propeller_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn disable_individual_plane_propeller_raw(vehicle: i32, propeller: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DISABLE_INDIVIDUAL_PLANE_PROPELLER(vehicle, propeller)
}

/// ## Parameters
* **value**:

pub fn set_number_of_parked_vehicles_safe(value: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_NUMBER_OF_PARKED_VEHICLES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_NUMBER_OF_PARKED_VEHICLES(value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_NUMBER_OF_PARKED_VEHICLES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_number_of_parked_vehicles_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_number_of_parked_vehicles_raw(value: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_NUMBER_OF_PARKED_VEHICLES(value)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**:

pub fn _0xc0ed6438e6d39ba8_safe(p0: serde_json::Value, p1: serde_json::Value, p2: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0xC0ED6438E6D39BA8");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xC0ED6438E6D39BA8(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1), crate::utils::any_to_c_void_ptr(p2))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xC0ED6438E6D39BA8
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xc0ed6438e6d39ba8_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xc0ed6438e6d39ba8_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void, p2: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xC0ED6438E6D39BA8(p0, p1, p2)
}

/// Checks whether the specified boat vehicle is capsized, meaning it has overturned or is upside down in the water.

pub fn get_is_boat_capsized_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_IS_BOAT_CAPSIZED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_IS_BOAT_CAPSIZED(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_IS_BOAT_CAPSIZED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_is_boat_capsized_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_is_boat_capsized_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_IS_BOAT_CAPSIZED(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **color**:

pub fn _get_vehicle_dashboard_color_safe(vehicle: Vehicle, color: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: _GET_VEHICLE_DASHBOARD_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_DASHBOARD_COLOR(vehicle, color)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _GET_VEHICLE_DASHBOARD_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_dashboard_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_dashboard_color_raw(vehicle: i32, color: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_DASHBOARD_COLOR(vehicle, color)
}

/// ## Parameters
* **id**: 
* **time**:

pub fn get_position_of_vehicle_recording_id_at_time_safe(id: i64, time: f32) -> NativeResult<Vector3> {
    
    debug!("Calling native function: GET_POSITION_OF_VEHICLE_RECORDING_ID_AT_TIME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_POSITION_OF_VEHICLE_RECORDING_ID_AT_TIME(id, time)
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: GET_POSITION_OF_VEHICLE_RECORDING_ID_AT_TIME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_position_of_vehicle_recording_id_at_time_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_position_of_vehicle_recording_id_at_time_raw(id: i64, time: f32) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_POSITION_OF_VEHICLE_RECORDING_ID_AT_TIME(id, time)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_can_save_in_garage_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_CAN_SAVE_IN_GARAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_CAN_SAVE_IN_GARAGE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_CAN_SAVE_IN_GARAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_can_save_in_garage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_can_save_in_garage_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_CAN_SAVE_IN_GARAGE(vehicle, toggle)
}

/// ## Parameters
* **vehicleClass**:

pub fn get_vehicle_class_max_acceleration_safe(vehicleClass: i64) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_CLASS_MAX_ACCELERATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_CLASS_MAX_ACCELERATION(vehicleClass)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_CLASS_MAX_ACCELERATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_class_max_acceleration_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_class_max_acceleration_raw(vehicleClass: i64) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_CLASS_MAX_ACCELERATION(vehicleClass)
}

/// Returns the display name/text label (`gameName` in `vehicles.meta`) for the specified vehicle model.

pub fn get_display_name_from_vehicle_model_safe(modelHash: u32) -> NativeResult<String> {
    
    debug!("Calling native function: GET_DISPLAY_NAME_FROM_VEHICLE_MODEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_DISPLAY_NAME_FROM_VEHICLE_MODEL(modelHash)
    };
    
    
    Ok(crate::utils::c_string_to_rust_string(result))
}

/// Raw native function: GET_DISPLAY_NAME_FROM_VEHICLE_MODEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_display_name_from_vehicle_model_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_display_name_from_vehicle_model_raw(modelHash: u32) -> *const std::os::raw::c_char {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_DISPLAY_NAME_FROM_VEHICLE_MODEL(modelHash)
}

/// ```
makes the train all jumbled up and derailed as it moves on the tracks (though that wont stop it from its normal operations)  
```

pub fn set_render_train_as_derailed_safe(train: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_RENDER_TRAIN_AS_DERAILED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_RENDER_TRAIN_AS_DERAILED(train, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_RENDER_TRAIN_AS_DERAILED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_render_train_as_derailed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_render_train_as_derailed_raw(train: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_RENDER_TRAIN_AS_DERAILED(train, toggle)
}

/// SET_VEHICLE_WHEEL_TYPE native function

pub fn set_vehicle_wheel_type_safe(vehicle: Vehicle, wheelType: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_WHEEL_TYPE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_WHEEL_TYPE(vehicle, wheelType)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_WHEEL_TYPE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_wheel_type_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_wheel_type_raw(vehicle: i32, wheelType: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_WHEEL_TYPE(vehicle, wheelType)
}

/// ```
NativeDB Introduced: v1365
```

pub fn _set_vehicle_doors_locked_for_unk_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_DOORS_LOCKED_FOR_UNK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_DOORS_LOCKED_FOR_UNK(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_DOORS_LOCKED_FOR_UNK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_doors_locked_for_unk_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_doors_locked_for_unk_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_DOORS_LOCKED_FOR_UNK(vehicle, toggle)
}

/// ## Parameters
* **vehicle**:

pub fn is_vehicle_attached_to_trailer_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_ATTACHED_TO_TRAILER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_ATTACHED_TO_TRAILER(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_ATTACHED_TO_TRAILER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_attached_to_trailer_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_attached_to_trailer_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_ATTACHED_TO_TRAILER(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_force_hd_vehicle_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_FORCE_HD_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_FORCE_HD_VEHICLE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_FORCE_HD_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_force_hd_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_force_hd_vehicle_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_FORCE_HD_VEHICLE(vehicle, toggle)
}

/// ```
Locks the vehicle's steering to the desired angle, explained below.  
Requires to be called onTick. Steering is unlocked the moment the function stops being called on the vehicle.  
Steer bias:  
-1.0 = full right  
0.0 = centered steering  
1.0 = full left  
```

pub fn set_vehicle_steer_bias_safe(vehicle: Vehicle, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_STEER_BIAS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_STEER_BIAS(vehicle, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_STEER_BIAS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_steer_bias_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_steer_bias_raw(vehicle: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_STEER_BIAS(vehicle, value)
}

/// ## Parameters
* **vehicle**: 
* **percentage**:

pub fn _set_vehicle_rocket_boost_percentage_safe(vehicle: Vehicle, percentage: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_ROCKET_BOOST_PERCENTAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_ROCKET_BOOST_PERCENTAGE(vehicle, percentage)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_ROCKET_BOOST_PERCENTAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_rocket_boost_percentage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_rocket_boost_percentage_raw(vehicle: i32, percentage: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_ROCKET_BOOST_PERCENTAGE(vehicle, percentage)
}

/// ```
SET_VEHICLE_AL*
```

pub fn _set_vehicle_can_be_locked_on_safe(vehicle: Vehicle, canBeLockedOn: bool, unk: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_CAN_BE_LOCKED_ON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_CAN_BE_LOCKED_ON(vehicle, canBeLockedOn, unk)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_CAN_BE_LOCKED_ON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_can_be_locked_on_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_can_be_locked_on_raw(vehicle: i32, canBeLockedOn: bool, unk: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_CAN_BE_LOCKED_ON(vehicle, canBeLockedOn, unk)
}

/// ```
Inverts vehicle's controls. So INPUT_VEH_ACCELERATE will be INPUT_VEH_BRAKE and vise versa (same for A/D controls)
Doesn't work for planes/helis.
```

pub fn _set_vehicle_controls_inverted_safe(vehicle: Vehicle, state: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_CONTROLS_INVERTED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_CONTROLS_INVERTED(vehicle, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_CONTROLS_INVERTED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_controls_inverted_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_controls_inverted_raw(vehicle: i32, state: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_CONTROLS_INVERTED(vehicle, state)
}

/// ```
Only used for wheels(ModType = 23/24) Returns true if the wheels are custom wheels
```

pub fn get_vehicle_mod_variation_safe(vehicle: Vehicle, modType: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_VEHICLE_MOD_VARIATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MOD_VARIATION(vehicle, modType)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MOD_VARIATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_mod_variation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_mod_variation_raw(vehicle: i32, modType: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MOD_VARIATION(vehicle, modType)
}

/// ## Parameters
* **vehicle**:

pub fn add_vehicle_phone_explosive_device_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: ADD_VEHICLE_PHONE_EXPLOSIVE_DEVICE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ADD_VEHICLE_PHONE_EXPLOSIVE_DEVICE(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ADD_VEHICLE_PHONE_EXPLOSIVE_DEVICE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `add_vehicle_phone_explosive_device_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn add_vehicle_phone_explosive_device_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ADD_VEHICLE_PHONE_EXPLOSIVE_DEVICE(vehicle)
}

/// ```
0.0 = Lowest 1.0 = Highest. This is best to be used if you wanna pick-up a car since un-realistically on GTA V forklifts can't pick up much of anything due to vehicle mass. If you put this under a car then set it above 0.0 to a 'lifted-value' it will raise the car with no issue lol
```

pub fn set_forklift_fork_height_safe(vehicle: Vehicle, height: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_FORKLIFT_FORK_HEIGHT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_FORKLIFT_FORK_HEIGHT(vehicle, height)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_FORKLIFT_FORK_HEIGHT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_forklift_fork_height_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_forklift_fork_height_raw(vehicle: i32, height: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_FORKLIFT_FORK_HEIGHT(vehicle, height)
}

/// ## Parameters
* **model**:

pub fn _is_this_model_an_amphibious_car_safe(model: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_THIS_MODEL_AN_AMPHIBIOUS_CAR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_THIS_MODEL_AN_AMPHIBIOUS_CAR(model)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_THIS_MODEL_AN_AMPHIBIOUS_CAR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_this_model_an_amphibious_car_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_this_model_an_amphibious_car_raw(model: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_THIS_MODEL_AN_AMPHIBIOUS_CAR(model)
}

/// ## Parameters
* **vehicle**:

pub fn _0xae3fee8709b39dcb_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _0xAE3FEE8709B39DCB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xAE3FEE8709B39DCB(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0xAE3FEE8709B39DCB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xae3fee8709b39dcb_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xae3fee8709b39dcb_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xAE3FEE8709B39DCB(vehicle)
}

/// ```
<1.0 - Decreased torque
=1.0 - Default torque
>1.0 - Increased torque
Negative values will cause the vehicle to go backwards instead of forwards while accelerating.
value - is between 0.2 and 1.8 in the decompiled scripts.
This needs to be called every frame to take effect.
```

pub fn set_vehicle_cheat_power_increase_safe(vehicle: Vehicle, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_CHEAT_POWER_INCREASE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_CHEAT_POWER_INCREASE(vehicle, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_CHEAT_POWER_INCREASE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_cheat_power_increase_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_cheat_power_increase_raw(vehicle: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_CHEAT_POWER_INCREASE(vehicle, value)
}

/// ```
Returns max speed (without mods) of the specified vehicle model in m/s.
```

pub fn get_vehicle_model_estimated_max_speed_safe(modelHash: u32) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_MODEL_ESTIMATED_MAX_SPEED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MODEL_ESTIMATED_MAX_SPEED(modelHash)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MODEL_ESTIMATED_MAX_SPEED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_model_estimated_max_speed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_model_estimated_max_speed_raw(modelHash: u32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MODEL_ESTIMATED_MAX_SPEED(modelHash)
}

/// ## Parameters
* **plane**:

pub fn _are_plane_wings_intact_safe(plane: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _ARE_PLANE_WINGS_INTACT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_ARE_PLANE_WINGS_INTACT(plane)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _ARE_PLANE_WINGS_INTACT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_are_plane_wings_intact_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _are_plane_wings_intact_raw(plane: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_ARE_PLANE_WINGS_INTACT(plane)
}

/// ```
SET_TIME_POSITION_IN_RECORDING can be emulated by: desired_time - GET_TIME_POSITION_IN_RECORDING(vehicle)
```

pub fn skip_time_in_playback_recorded_vehicle_safe(vehicle: Vehicle, time: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SKIP_TIME_IN_PLAYBACK_RECORDED_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SKIP_TIME_IN_PLAYBACK_RECORDED_VEHICLE(vehicle, time)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SKIP_TIME_IN_PLAYBACK_RECORDED_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `skip_time_in_playback_recorded_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn skip_time_in_playback_recorded_vehicle_raw(vehicle: i32, time: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SKIP_TIME_IN_PLAYBACK_RECORDED_VEHICLE(vehicle, time)
}

/// ```
what does this do?  
```

pub fn _0xcfd778e7904c255e_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _0xCFD778E7904C255E");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xCFD778E7904C255E(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xCFD778E7904C255E
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xcfd778e7904c255e_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xcfd778e7904c255e_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xCFD778E7904C255E(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _0x9becd4b9fef3f8a6_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x9BECD4B9FEF3F8A6");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x9BECD4B9FEF3F8A6(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x9BECD4B9FEF3F8A6
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x9becd4b9fef3f8a6_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x9becd4b9fef3f8a6_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x9BECD4B9FEF3F8A6(vehicle, p1)
}

/// ```
Tested on the player's current vehicle. Unless you kill the driver, the vehicle doesn't loose control, however, if enabled, explodeOnImpact is still active. The moment you crash, boom.  
```

pub fn set_vehicle_out_of_control_safe(vehicle: Vehicle, killDriver: bool, explodeOnImpact: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_OUT_OF_CONTROL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_OUT_OF_CONTROL(vehicle, killDriver, explodeOnImpact)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_OUT_OF_CONTROL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_out_of_control_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_out_of_control_raw(vehicle: i32, killDriver: bool, explodeOnImpact: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_OUT_OF_CONTROL(vehicle, killDriver, explodeOnImpact)
}

/// Returns the plates a vehicle has.

```c
enum eVehiclePlateType
{
	VPT_FRONT_AND_BACK_PLATES = 0,
	VPT_FRONT_PLATES = 1,
	VPT_BACK_PLATES = 2,
	VPT_NONE = 3,
};
```

Motorcycles with no visible plates will sometimes return a 2 for unknown reasons.

pub fn get_vehicle_plate_type_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_PLATE_TYPE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_PLATE_TYPE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_PLATE_TYPE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_plate_type_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_plate_type_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_PLATE_TYPE(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn is_playback_going_on_for_vehicle_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLAYBACK_GOING_ON_FOR_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLAYBACK_GOING_ON_FOR_VEHICLE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLAYBACK_GOING_ON_FOR_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_playback_going_on_for_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_playback_going_on_for_vehicle_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLAYBACK_GOING_ON_FOR_VEHICLE(vehicle)
}

/// Calling this native will keep a vehicle's engine running after exiting.

pub fn set_vehicle_keep_engine_on_when_abandoned_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_KEEP_ENGINE_ON_WHEN_ABANDONED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_KEEP_ENGINE_ON_WHEN_ABANDONED(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_KEEP_ENGINE_ON_WHEN_ABANDONED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_keep_engine_on_when_abandoned_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_keep_engine_on_when_abandoned_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_KEEP_ENGINE_ON_WHEN_ABANDONED(vehicle, toggle)
}

/// ## Parameters
* **vehicle**:

pub fn set_last_driven_vehicle_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_LAST_DRIVEN_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_LAST_DRIVEN_VEHICLE(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_LAST_DRIVEN_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_last_driven_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_last_driven_vehicle_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_LAST_DRIVEN_VEHICLE(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn _get_number_of_vehicle_doors_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_NUMBER_OF_VEHICLE_DOORS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_NUMBER_OF_VEHICLE_DOORS(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_NUMBER_OF_VEHICLE_DOORS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_number_of_vehicle_doors_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_number_of_vehicle_doors_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_NUMBER_OF_VEHICLE_DOORS(vehicle)
}

/// ## Parameters
* **modelHash**:

pub fn get_vehicle_model_max_braking_max_mods_safe(modelHash: u32) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_MODEL_MAX_BRAKING_MAX_MODS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MODEL_MAX_BRAKING_MAX_MODS(modelHash)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MODEL_MAX_BRAKING_MAX_MODS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_model_max_braking_max_mods_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_model_max_braking_max_mods_raw(modelHash: u32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MODEL_MAX_BRAKING_MAX_MODS(modelHash)
}

/// This native checks if the given vehicle is stopped at a red or amber traffic light junction, provided the driver's personality is set to not run amber lights.

pub fn is_vehicle_stopped_at_traffic_lights_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_STOPPED_AT_TRAFFIC_LIGHTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_STOPPED_AT_TRAFFIC_LIGHTS(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_STOPPED_AT_TRAFFIC_LIGHTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_stopped_at_traffic_lights_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_stopped_at_traffic_lights_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_STOPPED_AT_TRAFFIC_LIGHTS(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn set_heli_turbulence_scalar_safe(vehicle: Vehicle, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_HELI_TURBULENCE_SCALAR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_HELI_TURBULENCE_SCALAR(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_HELI_TURBULENCE_SCALAR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_heli_turbulence_scalar_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_heli_turbulence_scalar_raw(vehicle: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_HELI_TURBULENCE_SCALAR(vehicle, p1)
}

/// DELETE_MISSION_TRAIN native function

pub fn delete_mission_train_safe(train: *mut i32) -> NativeResult<()> {
    
    debug!("Calling native function: DELETE_MISSION_TRAIN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DELETE_MISSION_TRAIN(train)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DELETE_MISSION_TRAIN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `delete_mission_train_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn delete_mission_train_raw(train: *mut i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DELETE_MISSION_TRAIN(train)
}

/// ## Parameters
* **vehicle**: The vehicle to get the cause of destruction of.

pub fn get_vehicle_cause_of_destruction_safe(vehicle: Vehicle) -> NativeResult<u32> {
    
    debug!("Calling native function: GET_VEHICLE_CAUSE_OF_DESTRUCTION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_CAUSE_OF_DESTRUCTION(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_CAUSE_OF_DESTRUCTION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_cause_of_destruction_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_cause_of_destruction_raw(vehicle: i32) -> u32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_CAUSE_OF_DESTRUCTION(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn clear_vehicle_custom_primary_colour_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_VEHICLE_CUSTOM_PRIMARY_COLOUR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_VEHICLE_CUSTOM_PRIMARY_COLOUR(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_VEHICLE_CUSTOM_PRIMARY_COLOUR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_vehicle_custom_primary_colour_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_vehicle_custom_primary_colour_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_VEHICLE_CUSTOM_PRIMARY_COLOUR(vehicle)
}

/// ```
Stops CTaskBringVehicleToHalt
```

pub fn _stop_bring_vehicle_to_halt_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _STOP_BRING_VEHICLE_TO_HALT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_STOP_BRING_VEHICLE_TO_HALT(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _STOP_BRING_VEHICLE_TO_HALT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_stop_bring_vehicle_to_halt_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _stop_bring_vehicle_to_halt_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_STOP_BRING_VEHICLE_TO_HALT(vehicle)
}

/// ```
NativeDB Introduced: v2372
```

pub fn _set_drift_tyres_enabled_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_DRIFT_TYRES_ENABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_DRIFT_TYRES_ENABLED(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_DRIFT_TYRES_ENABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_drift_tyres_enabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_drift_tyres_enabled_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_DRIFT_TYRES_ENABLED(vehicle, toggle)
}

/// ```
NativeDB Introduced: v2372
```

pub fn _is_mission_train_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_MISSION_TRAIN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_MISSION_TRAIN(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_MISSION_TRAIN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_mission_train_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_mission_train_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_MISSION_TRAIN(vehicle)
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)

pub fn set_vehicle_door_control_safe(vehicle: Vehicle, doorIndex: i64, speed: i64, angle: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DOOR_CONTROL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DOOR_CONTROL(vehicle, doorIndex, speed, angle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DOOR_CONTROL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_door_control_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_door_control_raw(vehicle: i32, doorIndex: i64, speed: i64, angle: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DOOR_CONTROL(vehicle, doorIndex, speed, angle)
}

/// ```
NativeDB Introduced: v1290
```

pub fn _0x107a473d7a6647a9_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _0x107A473D7A6647A9");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x107A473D7A6647A9(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x107A473D7A6647A9
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x107a473d7a6647a9_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x107a473d7a6647a9_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x107A473D7A6647A9(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn is_heli_landing_area_blocked_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_HELI_LANDING_AREA_BLOCKED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_HELI_LANDING_AREA_BLOCKED(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_HELI_LANDING_AREA_BLOCKED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_heli_landing_area_blocked_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_heli_landing_area_blocked_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_HELI_LANDING_AREA_BLOCKED(vehicle)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**: 
* **p3**:

pub fn _0x796a877e459b99ea_safe(p0: serde_json::Value, p1: f32, p2: f32, p3: f32) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x796A877E459B99EA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x796A877E459B99EA(crate::utils::any_to_c_void_ptr(p0), p1, p2, p3)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x796A877E459B99EA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x796a877e459b99ea_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x796a877e459b99ea_raw(p0: *mut std::os::raw::c_void, p1: f32, p2: f32, p3: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x796A877E459B99EA(p0, p1, p2, p3)
}

/// ```
Allows creation of CEventShockingPlaneFlyby, CEventShockingHelicopterOverhead, and other(?) Shocking events
```

pub fn set_vehicle_generates_engine_shocking_events_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_GENERATES_ENGINE_SHOCKING_EVENTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_GENERATES_ENGINE_SHOCKING_EVENTS(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_GENERATES_ENGINE_SHOCKING_EVENTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_generates_engine_shocking_events_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_generates_engine_shocking_events_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_GENERATES_ENGINE_SHOCKING_EVENTS(vehicle, toggle)
}

/// ```
NativeDB Introduced: v1604
```

pub fn _hide_vehicle_tombstone_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _HIDE_VEHICLE_TOMBSTONE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_HIDE_VEHICLE_TOMBSTONE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _HIDE_VEHICLE_TOMBSTONE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_hide_vehicle_tombstone_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _hide_vehicle_tombstone_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_HIDE_VEHICLE_TOMBSTONE(vehicle, toggle)
}

/// Returns true when the bomb bay doors of this plane are open. False if they're closed.

pub fn _are_bomb_bay_doors_open_safe(aircraft: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _ARE_BOMB_BAY_DOORS_OPEN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_ARE_BOMB_BAY_DOORS_OPEN(aircraft)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _ARE_BOMB_BAY_DOORS_OPEN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_are_bomb_bay_doors_open_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _are_bomb_bay_doors_open_raw(aircraft: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_ARE_BOMB_BAY_DOORS_OPEN(aircraft)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0xbb2333bb87ddd87f_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0xBB2333BB87DDD87F");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xBB2333BB87DDD87F(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xBB2333BB87DDD87F
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xbb2333bb87ddd87f_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xbb2333bb87ddd87f_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xBB2333BB87DDD87F(p0, p1)
}

/// ## Parameters
* **ped**: 
* **vehicle**: 
* **outIndex**:

pub fn _is_ped_exclusive_driver_of_vehicle_safe(ped: Ped, vehicle: Vehicle, outIndex: *mut i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_PED_EXCLUSIVE_DRIVER_OF_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_PED_EXCLUSIVE_DRIVER_OF_VEHICLE(ped, vehicle, outIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_PED_EXCLUSIVE_DRIVER_OF_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_ped_exclusive_driver_of_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_ped_exclusive_driver_of_vehicle_raw(ped: i32, vehicle: i32, outIndex: *mut i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_PED_EXCLUSIVE_DRIVER_OF_VEHICLE(ped, vehicle, outIndex)
}

/// Enables or disables the opening of a vehicle's rear doors in the event of a sticky bomb explosion. This native is effective for armored vehicles, such as the Stockade (Brinks vehicle), allowing the rear doors to be opened through controlled explosions, which might otherwise remain locked due to the vehicle nature.

pub fn set_open_rear_doors_on_explosion_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_OPEN_REAR_DOORS_ON_EXPLOSION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_OPEN_REAR_DOORS_ON_EXPLOSION(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_OPEN_REAR_DOORS_ON_EXPLOSION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_open_rear_doors_on_explosion_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_open_rear_doors_on_explosion_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_OPEN_REAR_DOORS_ON_EXPLOSION(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_plane_resist_to_explosion_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLANE_RESIST_TO_EXPLOSION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLANE_RESIST_TO_EXPLOSION(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLANE_RESIST_TO_EXPLOSION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_plane_resist_to_explosion_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_plane_resist_to_explosion_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLANE_RESIST_TO_EXPLOSION(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: The vehicle for which to obtain the acceleration.

pub fn get_vehicle_acceleration_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_ACCELERATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_ACCELERATION(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_ACCELERATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_acceleration_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_acceleration_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_ACCELERATION(vehicle)
}

/// Returns a number of available rooftop liveries, or -1 if vehicle has no rooftop liveries available.

pub fn _get_vehicle_roof_livery_count_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_VEHICLE_ROOF_LIVERY_COUNT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_ROOF_LIVERY_COUNT(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_VEHICLE_ROOF_LIVERY_COUNT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_roof_livery_count_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_roof_livery_count_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_ROOF_LIVERY_COUNT(vehicle)
}

/// ```
Seems related to vehicle health, like the one in IV.  
Max 1000, min 0.  
Vehicle does not necessarily explode or become undrivable at 0.  
```

pub fn get_vehicle_body_health_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_BODY_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_BODY_HEALTH(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_BODY_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_body_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_body_health_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_BODY_HEALTH(vehicle)
}

/// ```
Related to monster trucks in native scripts.
```

```
NativeDB Introduced: v1604
```

pub fn _set_vehicle_wheels_deal_damage_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_WHEELS_DEAL_DAMAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_WHEELS_DEAL_DAMAGE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_WHEELS_DEAL_DAMAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_wheels_deal_damage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_wheels_deal_damage_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_WHEELS_DEAL_DAMAGE(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _set_vehicle_ramp_launch_modifier_safe(vehicle: Vehicle, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_RAMP_LAUNCH_MODIFIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_RAMP_LAUNCH_MODIFIER(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_RAMP_LAUNCH_MODIFIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_ramp_launch_modifier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_ramp_launch_modifier_raw(vehicle: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_RAMP_LAUNCH_MODIFIER(vehicle, p1)
}

/// ## Parameters
* **vehicle**: 
* **weaponSlot**:

pub fn _set_vehicle_weapons_disabled_safe(vehicle: Vehicle, weaponSlot: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_WEAPONS_DISABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_WEAPONS_DISABLED(vehicle, weaponSlot)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_WEAPONS_DISABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_weapons_disabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_weapons_disabled_raw(vehicle: i32, weaponSlot: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_WEAPONS_DISABLED(vehicle, weaponSlot)
}

/// ## Parameters
* **vehicleClass**:

pub fn get_vehicle_class_max_traction_safe(vehicleClass: i64) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_CLASS_MAX_TRACTION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_CLASS_MAX_TRACTION(vehicleClass)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_CLASS_MAX_TRACTION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_class_max_traction_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_class_max_traction_raw(vehicleClass: i64) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_CLASS_MAX_TRACTION(vehicleClass)
}

/// ## Parameters
* **model**:

pub fn is_this_model_a_heli_safe(model: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_THIS_MODEL_A_HELI");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_THIS_MODEL_A_HELI(model)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_THIS_MODEL_A_HELI
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_this_model_a_heli_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_this_model_a_heli_raw(model: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_THIS_MODEL_A_HELI(model)
}

/// Similar to [`_SET_AIRCRAFT_BOMB_COUNT`](#_0xF4B2ED59DEB5D774), this sets the amount of countermeasures that are present on this vehicle.

Use [`_GET_AIRCRAFT_COUNTERMEASURE_COUNT`](#_0xF846AA63DF56B804) to get the current amount.

pub fn _set_vehicle_countermeasure_count_safe(aircraft: Vehicle, count: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_COUNTERMEASURE_COUNT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_COUNTERMEASURE_COUNT(aircraft, count)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_COUNTERMEASURE_COUNT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_countermeasure_count_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_countermeasure_count_raw(aircraft: i32, count: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_COUNTERMEASURE_COUNT(aircraft, count)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x6501129c9e0ffa05_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x6501129C9E0FFA05");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x6501129C9E0FFA05(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x6501129C9E0FFA05
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x6501129c9e0ffa05_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x6501129c9e0ffa05_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x6501129C9E0FFA05(p0, p1)
}

/// ## Parameters
* **vehicle**:

pub fn remove_vehicle_stuck_check_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_VEHICLE_STUCK_CHECK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_VEHICLE_STUCK_CHECK(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_VEHICLE_STUCK_CHECK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_vehicle_stuck_check_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_vehicle_stuck_check_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_VEHICLE_STUCK_CHECK(vehicle)
}

/// ```
1000 is max health  
Begins leaking gas at around 650 health  
```

pub fn set_vehicle_petrol_tank_health_safe(vehicle: Vehicle, health: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_PETROL_TANK_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_PETROL_TANK_HEALTH(vehicle, health)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_PETROL_TANK_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_petrol_tank_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_petrol_tank_health_raw(vehicle: i32, health: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_PETROL_TANK_HEALTH(vehicle, health)
}

/// ## Parameters
* **vehicle**: 
* **scalar**:

pub fn set_vehicle_steering_bias_scalar_safe(vehicle: Vehicle, scalar: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_STEERING_BIAS_SCALAR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_STEERING_BIAS_SCALAR(vehicle, scalar)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_STEERING_BIAS_SCALAR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_steering_bias_scalar_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_steering_bias_scalar_raw(vehicle: i32, scalar: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_STEERING_BIAS_SCALAR(vehicle, scalar)
}

/// Only used with the "akula" in the decompiled native scripts.

```
NativeDB Introduced: v1290
```

pub fn _are_heli_stub_wings_deployed_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _ARE_HELI_STUB_WINGS_DEPLOYED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_ARE_HELI_STUB_WINGS_DEPLOYED(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _ARE_HELI_STUB_WINGS_DEPLOYED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_are_heli_stub_wings_deployed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _are_heli_stub_wings_deployed_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_ARE_HELI_STUB_WINGS_DEPLOYED(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn is_vehicle_high_detail_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_HIGH_DETAIL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_HIGH_DETAIL(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_HIGH_DETAIL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_high_detail_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_high_detail_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_HIGH_DETAIL(vehicle)
}

/// ```
Appears to return true if the vehicle has any damage, including cosmetically.
GET_*
```

pub fn _is_vehicle_damaged_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_VEHICLE_DAMAGED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_VEHICLE_DAMAGED(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_VEHICLE_DAMAGED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_vehicle_damaged_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_vehicle_damaged_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_VEHICLE_DAMAGED(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn is_big_vehicle_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_BIG_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_BIG_VEHICLE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_BIG_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_big_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_big_vehicle_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_BIG_VEHICLE(vehicle)
}

/// Seat indices range from -1 to [`GET_VEHICLE_MAX_NUMBER_OF_PASSENGERS`](#_0xA7C4F2C6E744A550) minus one.

```c
// CTaskExitVehicleSeat::eSeatPosition - 1
enum eSeatPosition
{
    SF_FrontDriverSide = -1,
    SF_FrontPassengerSide = 0,
    SF_BackDriverSide = 1,
    SF_BackPassengerSide = 2,
    SF_AltFrontDriverSide = 3,
    SF_AltFrontPassengerSide = 4,
    SF_AltBackDriverSide = 5,
    SF_AltBackPassengerSide = 6,
};
```

```
NativeDB Added Parameter 3: BOOL isTaskRunning

isTaskRunning = on true the function returns already false while a task on the target seat is running (TASK_ENTER_VEHICLE/TASK_SHUFFLE_TO_NEXT_VEHICLE_SEAT) - on false only when a ped is finally sitting in the seat.
```

pub fn is_vehicle_seat_free_safe(vehicle: Vehicle, seatIndex: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_SEAT_FREE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_SEAT_FREE(vehicle, seatIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_SEAT_FREE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_seat_free_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_seat_free_raw(vehicle: i32, seatIndex: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_SEAT_FREE(vehicle, seatIndex)
}

/// _GET_VEHICLE_DRIVETRAIN_TYPE native function

pub fn _get_vehicle_drivetrain_type_safe(vehicleModel: u32) -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_VEHICLE_DRIVETRAIN_TYPE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_DRIVETRAIN_TYPE(vehicleModel)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_VEHICLE_DRIVETRAIN_TYPE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_drivetrain_type_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_drivetrain_type_raw(vehicleModel: u32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_DRIVETRAIN_TYPE(vehicleModel)
}

/// Starts or stops the engine on the specified vehicle.
From what I've tested when I do this to a helicopter the propellers turn off after the engine has started.

pub fn set_vehicle_engine_on_safe(vehicle: Vehicle, value: bool, instantly: bool, disableAutoStart: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_ENGINE_ON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_ENGINE_ON(vehicle, value, instantly, disableAutoStart)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_ENGINE_ON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_engine_on_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_engine_on_raw(vehicle: i32, value: bool, instantly: bool, disableAutoStart: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_ENGINE_ON(vehicle, value, instantly, disableAutoStart)
}

/// ```
NativeDB Introduced: v1604
```

pub fn _0x6a973569ba094650_safe(vehicle: Vehicle, p1: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p1_any_str = serde_json::to_string(&p1)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p1", e)))?;
    let p1_any_str_cstr = std::ffi::CString::new(p1_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p1", e)))?;
    
    debug!("Calling native function: _0x6A973569BA094650");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x6A973569BA094650(vehicle, crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x6A973569BA094650
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x6a973569ba094650_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x6a973569ba094650_raw(vehicle: i32, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x6A973569BA094650(vehicle, p1)
}

/// ## Parameters
* **vehicle**: 
* **active**:

pub fn _set_vehicle_parachute_active_safe(vehicle: Vehicle, active: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_PARACHUTE_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_PARACHUTE_ACTIVE(vehicle, active)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_PARACHUTE_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_parachute_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_parachute_active_raw(vehicle: i32, active: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_PARACHUTE_ACTIVE(vehicle, active)
}

/// ```
Usually used alongside other vehicle door natives.
```

pub fn _0x3b458ddb57038f08_safe(vehicle: Vehicle, doorIndex: i64, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x3B458DDB57038F08");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x3B458DDB57038F08(vehicle, doorIndex, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x3B458DDB57038F08
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x3b458ddb57038f08_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x3b458ddb57038f08_raw(vehicle: i32, doorIndex: i64, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x3B458DDB57038F08(vehicle, doorIndex, toggle)
}

/// ```
Reduces grip significantly so it's hard to go anywhere.  
```

pub fn set_vehicle_reduce_grip_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_REDUCE_GRIP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_REDUCE_GRIP(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_REDUCE_GRIP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_reduce_grip_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_reduce_grip_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_REDUCE_GRIP(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _0xe5810ac70602f2f5_safe(vehicle: Vehicle, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0xE5810AC70602F2F5");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xE5810AC70602F2F5(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xE5810AC70602F2F5
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xe5810ac70602f2f5_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xe5810ac70602f2f5_raw(vehicle: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xE5810AC70602F2F5(vehicle, p1)
}

/// ```
parachuteModel = 230075693  
```

pub fn _set_vehicle_parachute_model_safe(vehicle: Vehicle, modelHash: u32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_PARACHUTE_MODEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_PARACHUTE_MODEL(vehicle, modelHash)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_PARACHUTE_MODEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_parachute_model_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_parachute_model_raw(vehicle: i32, modelHash: u32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_PARACHUTE_MODEL(vehicle, modelHash)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0xab31ef4de6800ce9_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0xAB31EF4DE6800CE9");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xAB31EF4DE6800CE9(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xAB31EF4DE6800CE9
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xab31ef4de6800ce9_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xab31ef4de6800ce9_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xAB31EF4DE6800CE9(p0, p1)
}

/// ```
NativeDB Introduced: v1868
```

pub fn _get_tyre_health_safe(vehicle: Vehicle, wheelIndex: i64) -> NativeResult<f32> {
    
    debug!("Calling native function: _GET_TYRE_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_TYRE_HEALTH(vehicle, wheelIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_TYRE_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_tyre_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_tyre_health_raw(vehicle: i32, wheelIndex: i64) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_TYRE_HEALTH(vehicle, wheelIndex)
}

/// ```
Hardcoded to not work in multiplayer.  
```

pub fn set_can_respray_vehicle_safe(vehicle: Vehicle, state: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CAN_RESPRAY_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CAN_RESPRAY_VEHICLE(vehicle, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CAN_RESPRAY_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_can_respray_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_can_respray_vehicle_raw(vehicle: i32, state: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CAN_RESPRAY_VEHICLE(vehicle, state)
}

/// ```
Returns max number of passengers (including the driver) for the specified vehicle model.
```

pub fn get_vehicle_model_number_of_seats_safe(modelHash: u32) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_MODEL_NUMBER_OF_SEATS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MODEL_NUMBER_OF_SEATS(modelHash)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MODEL_NUMBER_OF_SEATS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_model_number_of_seats_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_model_number_of_seats_raw(modelHash: u32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MODEL_NUMBER_OF_SEATS(modelHash)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_provides_cover_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_PROVIDES_COVER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_PROVIDES_COVER(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_PROVIDES_COVER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_provides_cover_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_provides_cover_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_PROVIDES_COVER(vehicle, toggle)
}

/// This native sets whether a specific vehicle influences the player's wanted level when it is involved in an incident that typically triggers a wanted response, such as being marked as a "victim" vehicle.

This is particularly useful when utilizing the wanted system from GTA, and you want to prevent a vehicle from affecting the wanted level when it is stolen. In the decompiled scripts this native is only used to disable the influence of the vehicle on the wanted level.

pub fn set_vehicle_influences_wanted_level_safe(vehicle: Vehicle, influenceWantedLevel: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_INFLUENCES_WANTED_LEVEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_INFLUENCES_WANTED_LEVEL(vehicle, influenceWantedLevel)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_INFLUENCES_WANTED_LEVEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_influences_wanted_level_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_influences_wanted_level_raw(vehicle: i32, influenceWantedLevel: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_INFLUENCES_WANTED_LEVEL(vehicle, influenceWantedLevel)
}

/// Gets the amount of bombs that this vehicle has. As far as I know, this does _not_ impact vehicle weapons or the ammo of those weapons in any way, it is just a way to keep track of the amount of bombs in a specific plane. 

In decompiled scripts this is used to check if the vehicle has enough bombs before a bomb can be dropped (bombs are dropped by using [`_SHOOT_SINGLE_BULLET_BETWEEN_COORDS_WITH_EXTRA_PARAMS`](#_0xBFE5756E7407064A)). 

Use [`_SET_AIRCRAFT_BOMB_COUNT`](#_0xF4B2ED59DEB5D774) to set the amount of bombs on that vehicle.

pub fn _get_vehicle_bomb_count_safe(aircraft: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_VEHICLE_BOMB_COUNT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_BOMB_COUNT(aircraft)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_VEHICLE_BOMB_COUNT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_bomb_count_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_bomb_count_raw(aircraft: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_BOMB_COUNT(aircraft)
}

/// ```
NativeDB Introduced: v1493
```

pub fn _0x59c3757b3b7408e8_safe(vehicle: Vehicle, toggle: bool, p2: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0x59C3757B3B7408E8");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x59C3757B3B7408E8(vehicle, toggle, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x59C3757B3B7408E8
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x59c3757b3b7408e8_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x59c3757b3b7408e8_raw(vehicle: i32, toggle: bool, p2: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x59C3757B3B7408E8(vehicle, toggle, p2)
}

/// ## Parameters
* **vehicle**: 
* **frontBumper**:

pub fn is_vehicle_bumper_bouncing_safe(vehicle: Vehicle, frontBumper: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_BUMPER_BOUNCING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_BUMPER_BOUNCING(vehicle, frontBumper)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_BUMPER_BOUNCING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_bumper_bouncing_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_bumper_bouncing_raw(vehicle: i32, frontBumper: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_BUMPER_BOUNCING(vehicle, frontBumper)
}

/// Gets the color of the neon lights of the specified vehicle.  

See [`_SET_VEHICLE_NEON_LIGHTS_COLOUR`](#_0x8E0A582209A62695) for more information

pub fn _get_vehicle_neon_lights_colour_safe(vehicle: Vehicle, r: *mut i64, g: *mut i64, b: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: _GET_VEHICLE_NEON_LIGHTS_COLOUR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_NEON_LIGHTS_COLOUR(vehicle, r, g, b)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _GET_VEHICLE_NEON_LIGHTS_COLOUR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_neon_lights_colour_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_neon_lights_colour_raw(vehicle: i32, r: *mut i64, g: *mut i64, b: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_NEON_LIGHTS_COLOUR(vehicle, r, g, b)
}

/// ## Parameters
* **vehicleGenerator**:

pub fn does_script_vehicle_generator_exist_safe(vehicleGenerator: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: DOES_SCRIPT_VEHICLE_GENERATOR_EXIST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DOES_SCRIPT_VEHICLE_GENERATOR_EXIST(vehicleGenerator)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: DOES_SCRIPT_VEHICLE_GENERATOR_EXIST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `does_script_vehicle_generator_exist_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn does_script_vehicle_generator_exist_raw(vehicleGenerator: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DOES_SCRIPT_VEHICLE_GENERATOR_EXIST(vehicleGenerator)
}

/// ## Parameters
* **vehicle**:

pub fn stop_playback_recorded_vehicle_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: STOP_PLAYBACK_RECORDED_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::STOP_PLAYBACK_RECORDED_VEHICLE(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: STOP_PLAYBACK_RECORDED_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `stop_playback_recorded_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn stop_playback_recorded_vehicle_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::STOP_PLAYBACK_RECORDED_VEHICLE(vehicle)
}

/// ```
SET_VEHICLE_AL*
```

pub fn _0x7d6f9a3ef26136a0_safe(vehicle: Vehicle, toggle: bool, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x7D6F9A3EF26136A0");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x7D6F9A3EF26136A0(vehicle, toggle, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x7D6F9A3EF26136A0
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x7d6f9a3ef26136a0_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x7d6f9a3ef26136a0_raw(vehicle: i32, toggle: bool, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x7D6F9A3EF26136A0(vehicle, toggle, p2)
}

/// ```
Corrected p1. it's basically the 'carriage/trailer number'. So if the train has 3 trailers you'd call the native once with a var or 3 times with 1, 2, 3.  
```

pub fn get_train_carriage_safe(train: Vehicle, trailerNumber: i64) -> NativeResult<Entity> {
    
    debug!("Calling native function: GET_TRAIN_CARRIAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_TRAIN_CARRIAGE(train, trailerNumber)
    };
    
    
    Ok(Ok(Entity(result)))
}

/// Raw native function: GET_TRAIN_CARRIAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_train_carriage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_train_carriage_raw(train: i32, trailerNumber: i64) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_TRAIN_CARRIAGE(train, trailerNumber)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn _disable_vehicle_neon_lights_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _DISABLE_VEHICLE_NEON_LIGHTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_DISABLE_VEHICLE_NEON_LIGHTS(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _DISABLE_VEHICLE_NEON_LIGHTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_disable_vehicle_neon_lights_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _disable_vehicle_neon_lights_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_DISABLE_VEHICLE_NEON_LIGHTS(vehicle, toggle)
}

/// ```
Returns true if the vehicle has the FLAG_JUMPING_CAR flag set.
```

pub fn _get_can_vehicle_jump_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _GET_CAN_VEHICLE_JUMP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_CAN_VEHICLE_JUMP(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_CAN_VEHICLE_JUMP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_can_vehicle_jump_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_can_vehicle_jump_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_CAN_VEHICLE_JUMP(vehicle)
}

/// CAN_ANCHOR_BOAT_HERE native function

pub fn can_anchor_boat_here_safe(boat: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: CAN_ANCHOR_BOAT_HERE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CAN_ANCHOR_BOAT_HERE(boat)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CAN_ANCHOR_BOAT_HERE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `can_anchor_boat_here_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn can_anchor_boat_here_raw(boat: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CAN_ANCHOR_BOAT_HERE(boat)
}

/// ```
Example usage  
VEHICLE::GET_CLOSEST_VEHICLE(x, y, z, radius, hash, unknown leave at 70)   
x, y, z: Position to get closest vehicle to.  
radius: Max radius to get a vehicle.  
modelHash: Limit to vehicles with this model. 0 for any.  
flags: The bitwise flags altering the function's behaviour.  
Does not return police cars or helicopters.  
It seems to return police cars for me, does not seem to return helicopters, planes or boats for some reason  
Only returns non police cars and motorbikes with the flag set to 70 and modelHash to 0. ModelHash seems to always be 0 when not a modelHash in the scripts, as stated above.   
These flags were found in the b617d scripts: 0,2,4,6,7,23,127,260,2146,2175,12294,16384,16386,20503,32768,67590,67711,98309,100359.  
Converted to binary, each bit probably represents a flag as explained regarding another native here: gtaforums.com/topic/822314-guide-driving-styles  
Conversion of found flags to binary: pastebin.com/kghNFkRi  
At exactly 16384 which is 0100000000000000 in binary and 4000 in hexadecimal only planes are returned.   
It's probably more convenient to use worldGetAllVehicles(int *arr, int arrSize) and check the shortest distance yourself and sort if you want by checking the vehicle type with for example VEHICLE::IS_THIS_MODEL_A_BOAT  
-------------------------------------------------------------------------  
Conclusion: This native is not worth trying to use. Use something like this instead: pastebin.com/xiFdXa7h
Use flag 127 to return police cars
```

pub fn get_closest_vehicle_safe(x: f32, y: f32, z: f32, radius: f32, modelHash: u32, flags: i64) -> NativeResult<Vehicle> {
    
    debug!("Calling native function: GET_CLOSEST_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_CLOSEST_VEHICLE(x, y, z, radius, modelHash, flags)
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: GET_CLOSEST_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_closest_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_closest_vehicle_raw(x: f32, y: f32, z: f32, radius: f32, modelHash: u32, flags: i64) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_CLOSEST_VEHICLE(x, y, z, radius, modelHash, flags)
}

/// ```
NativeDB Introduced: v3407
```

Prevents the plane from exploding when taking body damage if the inflictor is an AI-controlled vehicle. Only works for planes.

pub fn _set_disable_explode_from_body_damage_received_by_ai_vehicle_safe(plane: Vehicle, disable: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_DISABLE_EXPLODE_FROM_BODY_DAMAGE_RECEIVED_BY_AI_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_DISABLE_EXPLODE_FROM_BODY_DAMAGE_RECEIVED_BY_AI_VEHICLE(plane, disable)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_DISABLE_EXPLODE_FROM_BODY_DAMAGE_RECEIVED_BY_AI_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_disable_explode_from_body_damage_received_by_ai_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_disable_explode_from_body_damage_received_by_ai_vehicle_raw(plane: i32, disable: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_DISABLE_EXPLODE_FROM_BODY_DAMAGE_RECEIVED_BY_AI_VEHICLE(plane, disable)
}

/// ## Parameters
* **vehicle**:

pub fn _get_vehicle_number_of_broken_bones_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_VEHICLE_NUMBER_OF_BROKEN_BONES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_NUMBER_OF_BROKEN_BONES(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_VEHICLE_NUMBER_OF_BROKEN_BONES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_number_of_broken_bones_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_number_of_broken_bones_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_NUMBER_OF_BROKEN_BONES(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **paintType**: 
* **color**:

pub fn get_vehicle_mod_color_2_safe(vehicle: Vehicle, paintType: *mut i64, color: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_VEHICLE_MOD_COLOR_2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MOD_COLOR_2(vehicle, paintType, color)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_VEHICLE_MOD_COLOR_2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_mod_color_2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_mod_color_2_raw(vehicle: i32, paintType: *mut i64, color: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MOD_COLOR_2(vehicle, paintType, color)
}

/// This only works for planes.

Prevents a vehicle from exploding upon sustaining body damage from physical collisions. 

For helicopters, you might want to check [`SET_DISABLE_HELI_EXPLODE_FROM_BODY_DAMAGE`](#_0xEDBC8405B3895CC9) instead.

```
NativeDB Introduced: v1290
```

pub fn set_disable_explode_from_body_damage_on_collision_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_DISABLE_EXPLODE_FROM_BODY_DAMAGE_ON_COLLISION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DISABLE_EXPLODE_FROM_BODY_DAMAGE_ON_COLLISION(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DISABLE_EXPLODE_FROM_BODY_DAMAGE_ON_COLLISION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_disable_explode_from_body_damage_on_collision_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_disable_explode_from_body_damage_on_collision_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DISABLE_EXPLODE_FROM_BODY_DAMAGE_ON_COLLISION(vehicle)
}

/// Only ever used once in decompiled scripts: **am_pi_menu**:
Returns true if the engine is on fire, or if the vehicle engine health is < 0 and it **has been** on fire.

It sometimes doesn't return true when the vehicle engine has been on fire, and has since been fixed. I'm not really sure what the exact conditions are.

This usually returns true even if there are no visible flames yet (engine health > 0). However if you monitor engine health you'll see that it starts decreasing as soon as this returns true.

pub fn _is_vehicle_engine_on_fire_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_VEHICLE_ENGINE_ON_FIRE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_VEHICLE_ENGINE_ON_FIRE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_VEHICLE_ENGINE_ON_FIRE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_vehicle_engine_on_fire_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_vehicle_engine_on_fire_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_VEHICLE_ENGINE_ON_FIRE(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **color**:

pub fn _set_vehicle_interior_color_safe(vehicle: Vehicle, color: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_INTERIOR_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_INTERIOR_COLOR(vehicle, color)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_INTERIOR_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_interior_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_interior_color_raw(vehicle: i32, color: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_INTERIOR_COLOR(vehicle, color)
}

/// ## Parameters
* **distance**:

pub fn set_lights_cutoff_distance_tweak_safe(distance: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_LIGHTS_CUTOFF_DISTANCE_TWEAK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_LIGHTS_CUTOFF_DISTANCE_TWEAK(distance)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_LIGHTS_CUTOFF_DISTANCE_TWEAK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_lights_cutoff_distance_tweak_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_lights_cutoff_distance_tweak_raw(distance: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_LIGHTS_CUTOFF_DISTANCE_TWEAK(distance)
}

/// Identical to SET_PLAYBACK_TO_USE_AI_TRY_TO_REVERT_BACK_LATER with 0 as arguments for p1 and p3.

pub fn set_playback_to_use_ai_safe(vehicle: Vehicle, drivingStyle: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYBACK_TO_USE_AI");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYBACK_TO_USE_AI(vehicle, drivingStyle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYBACK_TO_USE_AI
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_playback_to_use_ai_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_playback_to_use_ai_raw(vehicle: i32, drivingStyle: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYBACK_TO_USE_AI(vehicle, drivingStyle)
}

/// ## Parameters
* **vehicle**: 
* **position**: 
* **p2**:

pub fn set_vehicle_tank_turret_position_safe(vehicle: Vehicle, position: f32, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_TANK_TURRET_POSITION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_TANK_TURRET_POSITION(vehicle, position, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_TANK_TURRET_POSITION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_tank_turret_position_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_tank_turret_position_raw(vehicle: i32, position: f32, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_TANK_TURRET_POSITION(vehicle, position, p2)
}

/// ## Parameters
* **vehicle**: 
* **extraId**:

pub fn is_vehicle_extra_turned_on_safe(vehicle: Vehicle, extraId: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_EXTRA_TURNED_ON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_EXTRA_TURNED_ON(vehicle, extraId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_EXTRA_TURNED_ON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_extra_turned_on_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_extra_turned_on_raw(vehicle: i32, extraId: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_EXTRA_TURNED_ON(vehicle, extraId)
}

/// ## Parameters
* **vehicle**: 
* **pearlescentColor**: 
* **wheelColor**:

pub fn get_vehicle_extra_colours_safe(vehicle: Vehicle, pearlescentColor: *mut i64, wheelColor: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_VEHICLE_EXTRA_COLOURS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_EXTRA_COLOURS(vehicle, pearlescentColor, wheelColor)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_VEHICLE_EXTRA_COLOURS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_extra_colours_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_extra_colours_raw(vehicle: i32, pearlescentColor: *mut i64, wheelColor: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_EXTRA_COLOURS(vehicle, pearlescentColor, wheelColor)
}

/// See [REQUEST_VEHICLE_RECORDING](#_0xAF514CABE74CBF15)

pub fn remove_vehicle_recording_safe(recording: i64, script: String) -> NativeResult<()> {
    let script_cstr = std::ffi::CString::new(script.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "script", e)))?;
    
    debug!("Calling native function: REMOVE_VEHICLE_RECORDING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_VEHICLE_RECORDING(recording, crate::utils::rust_to_c_string(script))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_VEHICLE_RECORDING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_vehicle_recording_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_vehicle_recording_raw(recording: i64, script: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_VEHICLE_RECORDING(recording, script)
}

/// ## Parameters
* **vehicle**:

pub fn is_vehicle_stolen_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_STOLEN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_STOLEN(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_STOLEN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_stolen_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_stolen_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_STOLEN(vehicle)
}

/// ## Parameters
* **p0**:

pub fn _0x65b080555ea48149_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x65B080555EA48149");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x65B080555EA48149(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x65B080555EA48149
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x65b080555ea48149_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x65b080555ea48149_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x65B080555EA48149(p0)
}

/// Detaches the specified entity currently being carried by a Cargobob.

pub fn detach_entity_from_cargobob_safe(vehicle: Vehicle, entity: Entity) -> NativeResult<serde_json::Value> {
    
    debug!("Calling native function: DETACH_ENTITY_FROM_CARGOBOB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DETACH_ENTITY_FROM_CARGOBOB(vehicle, entity)
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: DETACH_ENTITY_FROM_CARGOBOB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `detach_entity_from_cargobob_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn detach_entity_from_cargobob_raw(vehicle: i32, entity: i32) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DETACH_ENTITY_FROM_CARGOBOB(vehicle, entity)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0xed5ede9e676643c9_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0xED5EDE9E676643C9");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xED5EDE9E676643C9(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xED5EDE9E676643C9
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xed5ede9e676643c9_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xed5ede9e676643c9_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xED5EDE9E676643C9(p0, p1)
}

/// Sets the arm position of a bulldozer. Position must be a value between 0.0 and 1.0. Ignored when `p2` is set to false, instead incrementing arm position by 0.1 (or 10%).

pub fn set_vehicle_bulldozer_arm_position_safe(vehicle: Vehicle, position: f32, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_BULLDOZER_ARM_POSITION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_BULLDOZER_ARM_POSITION(vehicle, position, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_BULLDOZER_ARM_POSITION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_bulldozer_arm_position_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_bulldozer_arm_position_raw(vehicle: i32, position: f32, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_BULLDOZER_ARM_POSITION(vehicle, position, p2)
}

/// ## Parameters
* **vehicle**:

pub fn open_bomb_bay_doors_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: OPEN_BOMB_BAY_DOORS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::OPEN_BOMB_BAY_DOORS(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: OPEN_BOMB_BAY_DOORS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `open_bomb_bay_doors_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn open_bomb_bay_doors_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::OPEN_BOMB_BAY_DOORS(vehicle)
}

/// ## Parameters
* **cargobob**: 
* **p1**:

pub fn set_cargobob_pickup_magnet_pull_strength_safe(cargobob: Vehicle, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CARGOBOB_PICKUP_MAGNET_PULL_STRENGTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CARGOBOB_PICKUP_MAGNET_PULL_STRENGTH(cargobob, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CARGOBOB_PICKUP_MAGNET_PULL_STRENGTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_cargobob_pickup_magnet_pull_strength_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_cargobob_pickup_magnet_pull_strength_raw(cargobob: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CARGOBOB_PICKUP_MAGNET_PULL_STRENGTH(cargobob, p1)
}

/// ## Parameters
* **p0**:

pub fn _0x41290b40fa63e6da_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x41290B40FA63E6DA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x41290B40FA63E6DA(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x41290B40FA63E6DA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x41290b40fa63e6da_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x41290b40fa63e6da_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x41290B40FA63E6DA(p0)
}

/// ```
Returns `nMonetaryValue` from handling.meta for specific model, which is the vehicle's monetary value.
```

pub fn get_vehicle_model_value_safe() -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_MODEL_VALUE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MODEL_VALUE()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MODEL_VALUE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_model_value_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_model_value_raw() -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MODEL_VALUE()
}

/// ```
NativeDB Introduced: v2372
```

pub fn _get_hydraulic_wheel_value_safe(vehicle: Vehicle, wheelId: i64) -> NativeResult<f32> {
    
    debug!("Calling native function: _GET_HYDRAULIC_WHEEL_VALUE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_HYDRAULIC_WHEEL_VALUE(vehicle, wheelId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_HYDRAULIC_WHEEL_VALUE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_hydraulic_wheel_value_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_hydraulic_wheel_value_raw(vehicle: i32, wheelId: i64) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_HYDRAULIC_WHEEL_VALUE(vehicle, wheelId)
}

/// See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).

pub fn remove_vehicle_window_safe(vehicle: Vehicle, windowIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_VEHICLE_WINDOW");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_VEHICLE_WINDOW(vehicle, windowIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_VEHICLE_WINDOW
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_vehicle_window_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_vehicle_window_raw(vehicle: i32, windowIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_VEHICLE_WINDOW(vehicle, windowIndex)
}

/// ```
NativeDB Introduced: v2060
```

pub fn _get_tyre_wear_multiplier_safe(vehicle: Vehicle, wheelIndex: i64) -> NativeResult<f32> {
    
    debug!("Calling native function: _GET_TYRE_WEAR_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_TYRE_WEAR_MULTIPLIER(vehicle, wheelIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_TYRE_WEAR_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_tyre_wear_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_tyre_wear_multiplier_raw(vehicle: i32, wheelIndex: i64) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_TYRE_WEAR_MULTIPLIER(vehicle, wheelIndex)
}

/// ## Parameters
* **vehicle**:

pub fn does_vehicle_have_weapons_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: DOES_VEHICLE_HAVE_WEAPONS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DOES_VEHICLE_HAVE_WEAPONS(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: DOES_VEHICLE_HAVE_WEAPONS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `does_vehicle_have_weapons_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn does_vehicle_have_weapons_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DOES_VEHICLE_HAVE_WEAPONS(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **entity**:

pub fn get_vehicle_lock_on_target_safe(vehicle: Vehicle, entity: *mut i32) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_VEHICLE_LOCK_ON_TARGET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_LOCK_ON_TARGET(vehicle, entity)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_LOCK_ON_TARGET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_lock_on_target_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_lock_on_target_raw(vehicle: i32, entity: *mut i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_LOCK_ON_TARGET(vehicle, entity)
}

/// ## Parameters
* **p0**:

pub fn _0xa247f9ef01d8082e_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xA247F9EF01D8082E");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xA247F9EF01D8082E(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xA247F9EF01D8082E
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xa247f9ef01d8082e_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xa247f9ef01d8082e_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xA247F9EF01D8082E(p0)
}

/// ## Parameters
* **vehicle**:

pub fn get_boat_boom_position_ratio_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_BOAT_BOOM_POSITION_RATIO");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_BOAT_BOOM_POSITION_RATIO(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_BOAT_BOOM_POSITION_RATIO
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_boat_boom_position_ratio_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_boat_boom_position_ratio_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_BOAT_BOOM_POSITION_RATIO(vehicle)
}

/// This native it's a debug native. Won't do anything.

pub fn allow_ambient_vehicles_to_avoid_adverse_conditions_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: ALLOW_AMBIENT_VEHICLES_TO_AVOID_ADVERSE_CONDITIONS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ALLOW_AMBIENT_VEHICLES_TO_AVOID_ADVERSE_CONDITIONS(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ALLOW_AMBIENT_VEHICLES_TO_AVOID_ADVERSE_CONDITIONS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `allow_ambient_vehicles_to_avoid_adverse_conditions_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn allow_ambient_vehicles_to_avoid_adverse_conditions_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ALLOW_AMBIENT_VEHICLES_TO_AVOID_ADVERSE_CONDITIONS(vehicle)
}

/// ## Parameters
* **p0**:

pub fn _0xd3301660a57c9272_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xD3301660A57C9272");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xD3301660A57C9272(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xD3301660A57C9272
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xd3301660a57c9272_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xd3301660a57c9272_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xD3301660A57C9272(p0)
}

/// ```
Sets some health value. Looks like it's used for helis.
```

pub fn _0x5ee5632f47ae9695_safe(vehicle: Vehicle, health: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0x5EE5632F47AE9695");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x5EE5632F47AE9695(vehicle, health)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x5EE5632F47AE9695
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x5ee5632f47ae9695_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x5ee5632f47ae9695_raw(vehicle: i32, health: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x5EE5632F47AE9695(vehicle, health)
}

/// IS_VEHICLE_SIREN_ON native function

pub fn is_vehicle_siren_on_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_SIREN_ON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_SIREN_ON(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_SIREN_ON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_siren_on_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_siren_on_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_SIREN_ON(vehicle)
}

/// ```c
enum eVehicleWheels
{
	WHEEL_LF = 0, // Vehicle Left front
	WHEEL_RF = 1, // Vehicle Right front
	WHEEL_LM = 2, // Vehicle Left middle
	WHEEL_RM = 3, // Vehicle Right middle
	WHEEL_LR = 4, // Vehicle Left rear
	WHEEL_RR = 5, // Vehicle Right rear
	WHEEL_BF = 6, // Bike front
	WHEEL_BR = 7, // Bike rear
	MAX_WHEELS = 8
};
```

pub fn is_vehicle_tyre_burst_safe(vehicle: Vehicle, wheelID: i64, isBurstToRim: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_TYRE_BURST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_TYRE_BURST(vehicle, wheelID, isBurstToRim)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_TYRE_BURST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_tyre_burst_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_tyre_burst_raw(vehicle: i32, wheelID: i64, isBurstToRim: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_TYRE_BURST(vehicle, wheelID, isBurstToRim)
}

/// ## Parameters
* **x1**: 
* **y1**: 
* **z1**: 
* **x2**: 
* **y2**: 
* **z2**: 
* **unk**:

pub fn remove_vehicles_from_generators_in_area_safe(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, unk: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let unk_any_str = serde_json::to_string(&unk)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "unk", e)))?;
    let unk_any_str_cstr = std::ffi::CString::new(unk_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "unk", e)))?;
    
    debug!("Calling native function: REMOVE_VEHICLES_FROM_GENERATORS_IN_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_VEHICLES_FROM_GENERATORS_IN_AREA(x1, y1, z1, x2, y2, z2, crate::utils::any_to_c_void_ptr(unk))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_VEHICLES_FROM_GENERATORS_IN_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_vehicles_from_generators_in_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_vehicles_from_generators_in_area_raw(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, unk: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_VEHICLES_FROM_GENERATORS_IN_AREA(x1, y1, z1, x2, y2, z2, unk)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_interiorlight_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_INTERIORLIGHT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_INTERIORLIGHT(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_INTERIORLIGHT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_interiorlight_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_interiorlight_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_INTERIORLIGHT(vehicle, toggle)
}

/// Specifies an area of interest where cargens will focus on spawning vehicles

You can clear the area of interest with [`CLEAR_VEHICLE_GENERATOR_AREA_OF_INTEREST`](#_0x0A436B8643716D14)

pub fn set_vehicle_generator_area_of_interest_safe(x: f32, y: f32, z: f32, radius: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_GENERATOR_AREA_OF_INTEREST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_GENERATOR_AREA_OF_INTEREST(x, y, z, radius)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_GENERATOR_AREA_OF_INTEREST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_generator_area_of_interest_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_generator_area_of_interest_raw(x: f32, y: f32, z: f32, radius: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_GENERATOR_AREA_OF_INTEREST(x, y, z, radius)
}

/// ## Parameters
* **vehicle**:

pub fn get_vehicle_door_lock_status_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_DOOR_LOCK_STATUS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_DOOR_LOCK_STATUS(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_DOOR_LOCK_STATUS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_door_lock_status_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_door_lock_status_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_DOOR_LOCK_STATUS(vehicle)
}

/// ```
"To burst tyres VEHICLE::SET_VEHICLE_TYRE_BURST(vehicle, 0, true, 1000.0)  
to burst all tyres type it 8 times where p1 = 0 to 7.  
p3 seems to be how much damage it has taken. 0 doesn't deflate them, 1000 completely deflates them.  
'0 = wheel_lf / bike, plane or jet front  
'1 = wheel_rf  
'2 = wheel_lm / in 6 wheels trailer, plane or jet is first one on left  
'3 = wheel_rm / in 6 wheels trailer, plane or jet is first one on right  
'4 = wheel_lr / bike rear / in 6 wheels trailer, plane or jet is last one on left  
'5 = wheel_rr / in 6 wheels trailer, plane or jet is last one on right  
'45 = 6 wheels trailer mid wheel left  
'47 = 6 wheels trailer mid wheel right  
```

pub fn set_vehicle_tyre_burst_safe(vehicle: Vehicle, index: i64, onRim: bool, p3: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_TYRE_BURST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_TYRE_BURST(vehicle, index, onRim, p3)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_TYRE_BURST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_tyre_burst_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_tyre_burst_raw(vehicle: i32, index: i64, onRim: bool, p3: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_TYRE_BURST(vehicle, index, onRim, p3)
}

/// Detaches the vehicle's windscreen.

pub fn pop_out_vehicle_windscreen_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: POP_OUT_VEHICLE_WINDSCREEN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::POP_OUT_VEHICLE_WINDSCREEN(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: POP_OUT_VEHICLE_WINDSCREEN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `pop_out_vehicle_windscreen_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn pop_out_vehicle_windscreen_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::POP_OUT_VEHICLE_WINDSCREEN(vehicle)
}

/// ```
NativeDB Introduced: 3095
```

Activates or deactivates the nitrous system in the specified vehicle, based on the boolean value provided.
You can clear the nitrous with [`CLEAR_NITROUS`](#_0xC889AE921400E1ED), if you want to have more control on the nitrous and those settings, use [`SET_OVERRIDE_NITROUS_LEVEL`](#_0xC8E9B6B71B8E660D)

pub fn set_nitrous_is_active_safe(vehicle: Vehicle, isActive: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_NITROUS_IS_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_NITROUS_IS_ACTIVE(vehicle, isActive)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_NITROUS_IS_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_nitrous_is_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_nitrous_is_active_raw(vehicle: i32, isActive: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_NITROUS_IS_ACTIVE(vehicle, isActive)
}

/// ```
Gets a random vehicle in a sphere at the specified position, of the specified radius.  
x: The X-component of the position of the sphere.  
y: The Y-component of the position of the sphere.  
z: The Z-component of the position of the sphere.  
radius: The radius of the sphere. Max is 9999.9004.  
modelHash: The vehicle model to limit the selection to. Pass 0 for any model.  
flags: The bitwise flags that modifies the behaviour of this function.  
```

pub fn get_random_vehicle_in_sphere_safe(x: f32, y: f32, z: f32, radius: f32, modelHash: u32, flags: i64) -> NativeResult<Vehicle> {
    
    debug!("Calling native function: GET_RANDOM_VEHICLE_IN_SPHERE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_RANDOM_VEHICLE_IN_SPHERE(x, y, z, radius, modelHash, flags)
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: GET_RANDOM_VEHICLE_IN_SPHERE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_random_vehicle_in_sphere_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_random_vehicle_in_sphere_raw(x: f32, y: f32, z: f32, radius: f32, modelHash: u32, flags: i64) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_RANDOM_VEHICLE_IN_SPHERE(x, y, z, radius, modelHash, flags)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_doors_locked_for_all_players_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DOORS_LOCKED_FOR_ALL_PLAYERS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DOORS_LOCKED_FOR_ALL_PLAYERS(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DOORS_LOCKED_FOR_ALL_PLAYERS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_doors_locked_for_all_players_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_doors_locked_for_all_players_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DOORS_LOCKED_FOR_ALL_PLAYERS(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_kers_allowed_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_KERS_ALLOWED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_KERS_ALLOWED(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_KERS_ALLOWED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_kers_allowed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_kers_allowed_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_KERS_ALLOWED(vehicle, toggle)
}

/// ```
NativeDB Introduced: v1290
```

pub fn _0xaa653ae61924b0a0_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xAA653AE61924B0A0");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xAA653AE61924B0A0(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xAA653AE61924B0A0
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xaa653ae61924b0a0_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xaa653ae61924b0a0_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xAA653AE61924B0A0(vehicle, toggle)
}

/// Allows locking the hover/non-hover mode of a vehicle, such as the flying mode of the `Deluxo`. In the decompiled scripts, this native is used on `oppressor2` but couldn't get it to work on it.

pub fn set_special_flight_mode_allowed_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SPECIAL_FLIGHT_MODE_ALLOWED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SPECIAL_FLIGHT_MODE_ALLOWED(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SPECIAL_FLIGHT_MODE_ALLOWED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_special_flight_mode_allowed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_special_flight_mode_allowed_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SPECIAL_FLIGHT_MODE_ALLOWED(vehicle, toggle)
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)

pub fn _get_is_door_valid_safe(vehicle: Vehicle, doorIndex: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _GET_IS_DOOR_VALID");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_IS_DOOR_VALID(vehicle, doorIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_IS_DOOR_VALID
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_is_door_valid_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_is_door_valid_raw(vehicle: i32, doorIndex: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_IS_DOOR_VALID(vehicle, doorIndex)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_brake_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_BRAKE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_BRAKE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_BRAKE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_brake_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_brake_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_BRAKE(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_can_be_used_by_fleeing_peds_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_CAN_BE_USED_BY_FLEEING_PEDS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_CAN_BE_USED_BY_FLEEING_PEDS(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_CAN_BE_USED_BY_FLEEING_PEDS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_can_be_used_by_fleeing_peds_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_can_be_used_by_fleeing_peds_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_CAN_BE_USED_BY_FLEEING_PEDS(vehicle, toggle)
}

/// ## Parameters
* **plane**:

pub fn is_plane_landing_gear_intact_safe(plane: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_PLANE_LANDING_GEAR_INTACT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_PLANE_LANDING_GEAR_INTACT(plane)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_PLANE_LANDING_GEAR_INTACT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_plane_landing_gear_intact_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_plane_landing_gear_intact_raw(plane: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_PLANE_LANDING_GEAR_INTACT(plane)
}

/// Enable/Disables global slipstream physics

pub fn set_enable_vehicle_slipstreaming_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_ENABLE_VEHICLE_SLIPSTREAMING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_ENABLE_VEHICLE_SLIPSTREAMING(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_ENABLE_VEHICLE_SLIPSTREAMING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_enable_vehicle_slipstreaming_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_enable_vehicle_slipstreaming_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_ENABLE_VEHICLE_SLIPSTREAMING(toggle)
}

/// Retrieves the manufacturer's name for a specified vehicle.

```
NativeDB Introduced: v1868
```

pub fn get_make_name_from_vehicle_model_safe(modelHash: u32) -> NativeResult<String> {
    
    debug!("Calling native function: GET_MAKE_NAME_FROM_VEHICLE_MODEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_MAKE_NAME_FROM_VEHICLE_MODEL(modelHash)
    };
    
    
    Ok(crate::utils::c_string_to_rust_string(result))
}

/// Raw native function: GET_MAKE_NAME_FROM_VEHICLE_MODEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_make_name_from_vehicle_model_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_make_name_from_vehicle_model_raw(modelHash: u32) -> *const std::os::raw::c_char {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_MAKE_NAME_FROM_VEHICLE_MODEL(modelHash)
}

/// ## Parameters
* **model**:

pub fn is_this_model_a_bike_safe(model: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_THIS_MODEL_A_BIKE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_THIS_MODEL_A_BIKE(model)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_THIS_MODEL_A_BIKE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_this_model_a_bike_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_this_model_a_bike_raw(model: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_THIS_MODEL_A_BIKE(model)
}

/// Gets the ped in the specified seat of the passed vehicle.

If there is no ped in the seat, and the game considers the vehicle as ambient population, this will create a random occupant ped in the seat, which may be cleaned up by the game fairly soon if not marked as script-owned mission entity.

pub fn get_ped_in_vehicle_seat_safe(vehicle: Vehicle, seatIndex: i64) -> NativeResult<Ped> {
    
    debug!("Calling native function: GET_PED_IN_VEHICLE_SEAT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_IN_VEHICLE_SEAT(vehicle, seatIndex)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: GET_PED_IN_VEHICLE_SEAT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_in_vehicle_seat_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_in_vehicle_seat_raw(vehicle: i32, seatIndex: i64) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_IN_VEHICLE_SEAT(vehicle, seatIndex)
}

/// ## Parameters
* **x1**: 
* **y1**: 
* **z1**: 
* **x2**: 
* **y2**: 
* **z2**: 
* **p6**: 
* **p7**:

pub fn set_all_vehicle_generators_active_in_area_safe(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, p6: bool, p7: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_ALL_VEHICLE_GENERATORS_ACTIVE_IN_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_ALL_VEHICLE_GENERATORS_ACTIVE_IN_AREA(x1, y1, z1, x2, y2, z2, p6, p7)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_ALL_VEHICLE_GENERATORS_ACTIVE_IN_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_all_vehicle_generators_active_in_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_all_vehicle_generators_active_in_area_raw(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, p6: bool, p7: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_ALL_VEHICLE_GENERATORS_ACTIVE_IN_AREA(x1, y1, z1, x2, y2, z2, p6, p7)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**: 
* **p5**: 
* **p6**:

pub fn get_random_vehicle_back_bumper_in_sphere_safe(p0: f32, p1: f32, p2: f32, p3: f32, p4: i64, p5: i64, p6: i64) -> NativeResult<Vehicle> {
    
    debug!("Calling native function: GET_RANDOM_VEHICLE_BACK_BUMPER_IN_SPHERE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_RANDOM_VEHICLE_BACK_BUMPER_IN_SPHERE(p0, p1, p2, p3, p4, p5, p6)
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: GET_RANDOM_VEHICLE_BACK_BUMPER_IN_SPHERE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_random_vehicle_back_bumper_in_sphere_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_random_vehicle_back_bumper_in_sphere_raw(p0: f32, p1: f32, p2: f32, p3: f32, p4: i64, p5: i64, p6: i64) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_RANDOM_VEHICLE_BACK_BUMPER_IN_SPHERE(p0, p1, p2, p3, p4, p5, p6)
}

/// Checks if a Submarine has any air leaks, when there is more than 4 the player will drown.

```
NativeDB Introduced: v2189
```

pub fn get_submarine_number_of_air_leaks_safe(submarine: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_SUBMARINE_NUMBER_OF_AIR_LEAKS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_SUBMARINE_NUMBER_OF_AIR_LEAKS(submarine)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_SUBMARINE_NUMBER_OF_AIR_LEAKS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_submarine_number_of_air_leaks_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_submarine_number_of_air_leaks_raw(submarine: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_SUBMARINE_NUMBER_OF_AIR_LEAKS(submarine)
}

/// ```
in script hook .net   
Vehicle v = ...;  
Function.Call(Hash.TRACK_VEHICLE_VISIBILITY, v.Handle);  
```

pub fn track_vehicle_visibility_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: TRACK_VEHICLE_VISIBILITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::TRACK_VEHICLE_VISIBILITY(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: TRACK_VEHICLE_VISIBILITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `track_vehicle_visibility_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn track_vehicle_visibility_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::TRACK_VEHICLE_VISIBILITY(vehicle)
}

/// ```
REQUEST_VEHICLE_ASSET(GET_HASH_KEY(cargobob3), 3);  
vehicle found that have asset's:  
cargobob3  
submersible  
blazer  
```

pub fn request_vehicle_asset_safe(vehicleHash: u32, vehicleAsset: i64) -> NativeResult<()> {
    
    debug!("Calling native function: REQUEST_VEHICLE_ASSET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REQUEST_VEHICLE_ASSET(vehicleHash, vehicleAsset)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REQUEST_VEHICLE_ASSET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `request_vehicle_asset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn request_vehicle_asset_raw(vehicleHash: u32, vehicleAsset: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REQUEST_VEHICLE_ASSET(vehicleHash, vehicleAsset)
}

/// ```c
enum eVehiclePlateIndicies {
	SanAndreasCursive = 0,
	SanAndreasBlack = 1,
	SanAndreasBlue = 2,
	SanAndreasPlain = 3,
	SRExcept = 4,
	NorthYankton = 5,
	// All indicies below this require b3095
	ECola = 6,
	LasVenturas = 7,
	LiberyCity = 8,
	LSCarMeet = 9,
	LSPanic = 10,
	LSPounders = 11,
	Sprunk = 12,
}
```

pub fn get_vehicle_number_plate_text_index_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_NUMBER_PLATE_TEXT_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_NUMBER_PLATE_TEXT_INDEX(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_NUMBER_PLATE_TEXT_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_number_plate_text_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_number_plate_text_index_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_NUMBER_PLATE_TEXT_INDEX(vehicle)
}

/// ```
Actually number of color combinations  
```

pub fn get_number_of_vehicle_colours_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_NUMBER_OF_VEHICLE_COLOURS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_NUMBER_OF_VEHICLE_COLOURS(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_NUMBER_OF_VEHICLE_COLOURS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_number_of_vehicle_colours_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_number_of_vehicle_colours_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_NUMBER_OF_VEHICLE_COLOURS(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn detach_vehicle_from_any_cargobob_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: DETACH_VEHICLE_FROM_ANY_CARGOBOB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DETACH_VEHICLE_FROM_ANY_CARGOBOB(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: DETACH_VEHICLE_FROM_ANY_CARGOBOB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `detach_vehicle_from_any_cargobob_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn detach_vehicle_from_any_cargobob_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DETACH_VEHICLE_FROM_ANY_CARGOBOB(vehicle)
}

/// ```
Makes the vehicle accept no passengers.  
```

pub fn set_vehicle_allow_no_passengers_lockon_safe(veh: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_ALLOW_NO_PASSENGERS_LOCKON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_ALLOW_NO_PASSENGERS_LOCKON(veh, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_ALLOW_NO_PASSENGERS_LOCKON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_allow_no_passengers_lockon_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_allow_no_passengers_lockon_raw(veh: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_ALLOW_NO_PASSENGERS_LOCKON(veh, toggle)
}

/// ## Parameters
* **vehicle**:

pub fn _get_entity_attached_to_cargobob_safe(vehicle: Vehicle) -> NativeResult<Entity> {
    
    debug!("Calling native function: _GET_ENTITY_ATTACHED_TO_CARGOBOB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_ENTITY_ATTACHED_TO_CARGOBOB(vehicle)
    };
    
    
    Ok(Ok(Entity(result)))
}

/// Raw native function: _GET_ENTITY_ATTACHED_TO_CARGOBOB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_entity_attached_to_cargobob_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_entity_attached_to_cargobob_raw(vehicle: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_ENTITY_ATTACHED_TO_CARGOBOB(vehicle)
}

/// ```
Returns max traction of the specified vehicle model.
```

pub fn get_vehicle_model_max_traction_safe(modelHash: u32) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_MODEL_MAX_TRACTION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MODEL_MAX_TRACTION(modelHash)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MODEL_MAX_TRACTION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_model_max_traction_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_model_max_traction_raw(modelHash: u32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MODEL_MAX_TRACTION(modelHash)
}

/// Disables wings for `Deluxo` and `Oppressor MK II`. For the Deluxo, it retracts the wings immediately, preventing flight. For the Oppressor Mk II, the wings retract after landing and take-off is not possible, though it can still glide if launched into the air.

pub fn set_disable_hover_mode_flight_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_DISABLE_HOVER_MODE_FLIGHT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DISABLE_HOVER_MODE_FLIGHT(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DISABLE_HOVER_MODE_FLIGHT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_disable_hover_mode_flight_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_disable_hover_mode_flight_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DISABLE_HOVER_MODE_FLIGHT(vehicle, toggle)
}

/// R* used it to "remove" vehicle windows when "nightshark" had some mod, which adding some kind of armored windows. When enabled, you can't break vehicles glass. All your bullets wiil shoot through glass. You also will not able to break the glass with any other way (hitting and etc)

pub fn _set_disable_vehicle_window_collisions_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_DISABLE_VEHICLE_WINDOW_COLLISIONS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_DISABLE_VEHICLE_WINDOW_COLLISIONS(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_DISABLE_VEHICLE_WINDOW_COLLISIONS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_disable_vehicle_window_collisions_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_disable_vehicle_window_collisions_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_DISABLE_VEHICLE_WINDOW_COLLISIONS(vehicle, toggle)
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)

pub fn get_ped_using_vehicle_door_safe(vehicle: Vehicle, doorIndex: i64) -> NativeResult<Ped> {
    
    debug!("Calling native function: GET_PED_USING_VEHICLE_DOOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_PED_USING_VEHICLE_DOOR(vehicle, doorIndex)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: GET_PED_USING_VEHICLE_DOOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_ped_using_vehicle_door_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_ped_using_vehicle_door_raw(vehicle: i32, doorIndex: i64) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_PED_USING_VEHICLE_DOOR(vehicle, doorIndex)
}

/// ## Parameters
* **vehicle**:

pub fn _get_vehicle_has_parachute_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _GET_VEHICLE_HAS_PARACHUTE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_HAS_PARACHUTE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_VEHICLE_HAS_PARACHUTE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_has_parachute_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_has_parachute_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_HAS_PARACHUTE(vehicle)
}

/// For FiveM, use [`GET_GAME_POOL`](#_0x2B9D4F50).

pub fn get_all_vehicles_safe() -> NativeResult<i64> {
    
    debug!("Calling native function: GET_ALL_VEHICLES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_ALL_VEHICLES()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_ALL_VEHICLES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_all_vehicles_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_all_vehicles_raw() -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_ALL_VEHICLES()
}

/// ```c
enum eWindowId {
	VEH_EXT_WINDOW_LF = 0,
	VEH_EXT_WINDOW_RF = 1,
	VEH_EXT_WINDOW_LR = 2,
	VEH_EXT_WINDOW_RR = 3,
	VEH_EXT_WINDOW_LM = 4,
	VEH_EXT_WINDOW_RM = 5,
	VEH_EXT_WINDSCREEN = 6,
	VEH_EXT_WINDSCREEN_R = 7,
}
```

pub fn is_vehicle_window_intact_safe(vehicle: Vehicle, windowIndex: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_WINDOW_INTACT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_WINDOW_INTACT(vehicle, windowIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_WINDOW_INTACT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_window_intact_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_window_intact_raw(vehicle: i32, windowIndex: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_WINDOW_INTACT(vehicle, windowIndex)
}

/// Despite its name, this works on Helicopters and Planes.

Sets the speed of the helicopter blades in percentage of the full speed.

pub fn set_heli_blades_speed_safe(vehicle: Vehicle, speed: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_HELI_BLADES_SPEED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_HELI_BLADES_SPEED(vehicle, speed)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_HELI_BLADES_SPEED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_heli_blades_speed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_heli_blades_speed_raw(vehicle: i32, speed: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_HELI_BLADES_SPEED(vehicle, speed)
}

/// Often used in conjunction with: [SET_VEHICLE_REDUCE_GRIP](#_0x222FF6A823D122E2).

```
NativeDB Introduced: v1604
```

pub fn _set_vehicle_reduce_traction_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_REDUCE_TRACTION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_REDUCE_TRACTION(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_REDUCE_TRACTION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_reduce_traction_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_reduce_traction_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_REDUCE_TRACTION(vehicle)
}

/// ```
Returns 1000.0 if the function is unable to get the address of the specified vehicle or if it's not a vehicle.  
Minimum: -4000  
Maximum: 1000  
-4000: Engine is destroyed  
0 and below: Engine catches fire and health rapidly declines  
300: Engine is smoking and losing functionality  
1000: Engine is perfect  
```

pub fn get_vehicle_engine_health_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_ENGINE_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_ENGINE_HEALTH(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_ENGINE_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_engine_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_engine_health_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_ENGINE_HEALTH(vehicle)
}

/// Returns index of the current vehicle's rooftop livery.
A getter for [_SET_VEHICLE_ROOF_LIVERY](#_0xA6D3A8750DC73270).

pub fn _get_vehicle_roof_livery_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_VEHICLE_ROOF_LIVERY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_ROOF_LIVERY(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_VEHICLE_ROOF_LIVERY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_roof_livery_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_roof_livery_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_ROOF_LIVERY(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **state**:

pub fn set_vehicle_can_be_visibly_damaged_safe(vehicle: Vehicle, state: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_CAN_BE_VISIBLY_DAMAGED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_CAN_BE_VISIBLY_DAMAGED(vehicle, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_CAN_BE_VISIBLY_DAMAGED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_can_be_visibly_damaged_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_can_be_visibly_damaged_raw(vehicle: i32, state: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_CAN_BE_VISIBLY_DAMAGED(vehicle, state)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn _set_vehicle_ramp_upwards_launch_motion_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_RAMP_UPWARDS_LAUNCH_MOTION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_RAMP_UPWARDS_LAUNCH_MOTION(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_RAMP_UPWARDS_LAUNCH_MOTION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_ramp_upwards_launch_motion_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_ramp_upwards_launch_motion_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_RAMP_UPWARDS_LAUNCH_MOTION(vehicle, toggle)
}

/// ```
Gets the height of the vehicle's suspension.  
The higher the value the lower the suspension. Each 0.002 corresponds with one more level lowered.  
0.000 is the stock suspension.  
0.008 is Ultra Suspension.  
```

pub fn _get_vehicle_suspension_height_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: _GET_VEHICLE_SUSPENSION_HEIGHT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_SUSPENSION_HEIGHT(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_VEHICLE_SUSPENSION_HEIGHT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_suspension_height_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_suspension_height_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_SUSPENSION_HEIGHT(vehicle)
}

/// ```
Activate siren on vehicle (Only works if the vehicle has a siren).  
```

pub fn set_vehicle_siren_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_SIREN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_SIREN(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_SIREN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_siren_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_siren_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_SIREN(vehicle, toggle)
}

/// Raises the roof on a convertible vehicle, utilizing any available animations for the action. This native is particularly useful for creating a realistic interaction with convertible vehicles by animating the process of raising the roof.

You can check if the vehicle has an convertible roof using [`IS_VEHICLE_A_CONVERTIBLE`](#_0x52F357A30698BCCE).

To lower the convertible roof, you can use [`LOWER_CONVERTIBLE_ROOF`](#_0xDED51F703D0FA83D).

```
NativeDB Introduced: v323
```

pub fn raise_convertible_roof_safe(vehicle: Vehicle, instantlyRaise: bool) -> NativeResult<()> {
    
    debug!("Calling native function: RAISE_CONVERTIBLE_ROOF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RAISE_CONVERTIBLE_ROOF(vehicle, instantlyRaise)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RAISE_CONVERTIBLE_ROOF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `raise_convertible_roof_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn raise_convertible_roof_raw(vehicle: i32, instantlyRaise: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RAISE_CONVERTIBLE_ROOF(vehicle, instantlyRaise)
}

/// ```
Inverse of 0x95CF53B3D687F9FA
```

```
NativeDB Added Parameter 1: Vehicle vehicle
```

pub fn _set_trailer_legs_lowered_safe() -> NativeResult<()> {
    
    debug!("Calling native function: _SET_TRAILER_LEGS_LOWERED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_TRAILER_LEGS_LOWERED()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_TRAILER_LEGS_LOWERED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_trailer_legs_lowered_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_trailer_legs_lowered_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_TRAILER_LEGS_LOWERED()
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_can_leak_petrol_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_CAN_LEAK_PETROL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_CAN_LEAK_PETROL(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_CAN_LEAK_PETROL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_can_leak_petrol_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_can_leak_petrol_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_CAN_LEAK_PETROL(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _0x88bc673ca9e0ae99_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x88BC673CA9E0AE99");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x88BC673CA9E0AE99(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x88BC673CA9E0AE99
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x88bc673ca9e0ae99_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x88bc673ca9e0ae99_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x88BC673CA9E0AE99(vehicle, p1)
}

/// ## Parameters
* **p0**:

pub fn _0x6eaaefc76acc311f_safe(p0: serde_json::Value) -> NativeResult<serde_json::Value> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x6EAAEFC76ACC311F");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x6EAAEFC76ACC311F(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0x6EAAEFC76ACC311F
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x6eaaefc76acc311f_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x6eaaefc76acc311f_raw(p0: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x6EAAEFC76ACC311F(p0)
}

/// ```
Returns attached vehicle (Vehicle in parameter must be cargobob)  
```

pub fn get_vehicle_attached_to_cargobob_safe(cargobob: Vehicle) -> NativeResult<Vehicle> {
    
    debug!("Calling native function: GET_VEHICLE_ATTACHED_TO_CARGOBOB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_ATTACHED_TO_CARGOBOB(cargobob)
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: GET_VEHICLE_ATTACHED_TO_CARGOBOB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_attached_to_cargobob_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_attached_to_cargobob_raw(cargobob: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_ATTACHED_TO_CARGOBOB(cargobob)
}

/// ## Parameters
* **vehicle**: 
* **color**:

pub fn _get_vehicle_interior_color_safe(vehicle: Vehicle, color: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: _GET_VEHICLE_INTERIOR_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_INTERIOR_COLOR(vehicle, color)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _GET_VEHICLE_INTERIOR_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_interior_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_interior_color_raw(vehicle: i32, color: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_INTERIOR_COLOR(vehicle, color)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_disable_towing_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DISABLE_TOWING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DISABLE_TOWING(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DISABLE_TOWING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_disable_towing_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_disable_towing_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DISABLE_TOWING(vehicle, toggle)
}

/// ```
NativeDB Introduced: v1604
```

pub fn _0x8f0d5ba1c2cc91d7_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x8F0D5BA1C2CC91D7");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x8F0D5BA1C2CC91D7(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x8F0D5BA1C2CC91D7
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x8f0d5ba1c2cc91d7_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x8f0d5ba1c2cc91d7_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x8F0D5BA1C2CC91D7(toggle)
}

/// Only used with the "akula" and "annihilator2" in the decompiled native scripts.

```
NativeDB Introduced: v1290
```

pub fn _set_deploy_heli_stub_wings_safe(vehicle: Vehicle, deploy: bool, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_DEPLOY_HELI_STUB_WINGS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_DEPLOY_HELI_STUB_WINGS(vehicle, deploy, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_DEPLOY_HELI_STUB_WINGS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_deploy_heli_stub_wings_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_deploy_heli_stub_wings_raw(vehicle: i32, deploy: bool, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_DEPLOY_HELI_STUB_WINGS(vehicle, deploy, p2)
}

/// ## Parameters
* **vehicle**: 
* **isStolen**:

pub fn set_vehicle_is_stolen_safe(vehicle: Vehicle, isStolen: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_IS_STOLEN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_IS_STOLEN(vehicle, isStolen)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_IS_STOLEN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_is_stolen_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_is_stolen_raw(vehicle: i32, isStolen: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_IS_STOLEN(vehicle, isStolen)
}

/// ```
REQUEST_VEHICLE_*  
```

pub fn _request_vehicle_dashboard_scaleform_movie_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _REQUEST_VEHICLE_DASHBOARD_SCALEFORM_MOVIE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_REQUEST_VEHICLE_DASHBOARD_SCALEFORM_MOVIE(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _REQUEST_VEHICLE_DASHBOARD_SCALEFORM_MOVIE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_request_vehicle_dashboard_scaleform_movie_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _request_vehicle_dashboard_scaleform_movie_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_REQUEST_VEHICLE_DASHBOARD_SCALEFORM_MOVIE(vehicle)
}

/// GET_VEHICLE_DOOR_ANGLE_RATIO native function

pub fn get_vehicle_door_angle_ratio_safe(vehicle: Vehicle, doorIndex: i64) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_DOOR_ANGLE_RATIO");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_DOOR_ANGLE_RATIO(vehicle, doorIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_DOOR_ANGLE_RATIO
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_door_angle_ratio_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_door_angle_ratio_raw(vehicle: i32, doorIndex: i64) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_DOOR_ANGLE_RATIO(vehicle, doorIndex)
}

/// SET_VEHICLE_LIGHTS native function

pub fn set_vehicle_lights_safe(vehicle: Vehicle, state: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_LIGHTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_LIGHTS(vehicle, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_LIGHTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_lights_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_lights_raw(vehicle: i32, state: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_LIGHTS(vehicle, state)
}

/// ## Parameters
* **vehicle**: 
* **modType**: Refer to eVehicleModType in [`SET_VEHICLE_MOD`](#_0x6AF0636DDEDCB6DD).

pub fn get_mod_slot_name_safe(vehicle: Vehicle, modType: i64) -> NativeResult<String> {
    
    debug!("Calling native function: GET_MOD_SLOT_NAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_MOD_SLOT_NAME(vehicle, modType)
    };
    
    
    Ok(crate::utils::c_string_to_rust_string(result))
}

/// Raw native function: GET_MOD_SLOT_NAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_mod_slot_name_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_mod_slot_name_raw(vehicle: i32, modType: i64) -> *const std::os::raw::c_char {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_MOD_SLOT_NAME(vehicle, modType)
}

/// ```
paintType:  
0: Normal  
1: Metallic  
2: Pearl  
3: Matte  
4: Metal  
5: Chrome  
color: number of the color.  
p3 seems to always be 0.  
```

pub fn set_vehicle_mod_color_1_safe(vehicle: Vehicle, paintType: i64, color: i64, pearlescentColor: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_MOD_COLOR_1");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_MOD_COLOR_1(vehicle, paintType, color, pearlescentColor)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_MOD_COLOR_1
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_mod_color_1_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_mod_color_1_raw(vehicle: i32, paintType: i64, color: i64, pearlescentColor: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_MOD_COLOR_1(vehicle, paintType, color, pearlescentColor)
}

/// ```
p1 is always 0 in the scripts.  
p1 = check if vehicle is on fire  
```

pub fn is_vehicle_driveable_safe(vehicle: Vehicle, isOnFireCheck: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_DRIVEABLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_DRIVEABLE(vehicle, isOnFireCheck)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_DRIVEABLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_driveable_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_driveable_raw(vehicle: i32, isOnFireCheck: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_DRIVEABLE(vehicle, isOnFireCheck)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn _0x80e3357fdef45c21_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x80E3357FDEF45C21");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x80E3357FDEF45C21(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x80E3357FDEF45C21
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x80e3357fdef45c21_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x80e3357fdef45c21_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x80E3357FDEF45C21(vehicle, toggle)
}

/// ## Parameters
* **decorator**:

pub fn does_vehicle_exist_with_decorator_safe(decorator: String) -> NativeResult<bool> {
    let decorator_cstr = std::ffi::CString::new(decorator.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "decorator", e)))?;
    
    debug!("Calling native function: DOES_VEHICLE_EXIST_WITH_DECORATOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DOES_VEHICLE_EXIST_WITH_DECORATOR(crate::utils::rust_to_c_string(decorator))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: DOES_VEHICLE_EXIST_WITH_DECORATOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `does_vehicle_exist_with_decorator_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn does_vehicle_exist_with_decorator_raw(decorator: *const std::os::raw::c_char) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DOES_VEHICLE_EXIST_WITH_DECORATOR(decorator)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_disable_pretend_occupants_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_DISABLE_PRETEND_OCCUPANTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DISABLE_PRETEND_OCCUPANTS(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DISABLE_PRETEND_OCCUPANTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_disable_pretend_occupants_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_disable_pretend_occupants_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DISABLE_PRETEND_OCCUPANTS(vehicle, toggle)
}

/// ```
HAS_*
```

pub fn _has_filled_vehicle_population_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: _HAS_FILLED_VEHICLE_POPULATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_HAS_FILLED_VEHICLE_POPULATION()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _HAS_FILLED_VEHICLE_POPULATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_has_filled_vehicle_population_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _has_filled_vehicle_population_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_HAS_FILLED_VEHICLE_POPULATION()
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_has_muted_sirens_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_HAS_MUTED_SIRENS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_HAS_MUTED_SIRENS(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_HAS_MUTED_SIRENS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_has_muted_sirens_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_has_muted_sirens_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_HAS_MUTED_SIRENS(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _0x3441cad2f2231923_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x3441CAD2F2231923");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x3441CAD2F2231923(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x3441CAD2F2231923
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x3441cad2f2231923_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x3441cad2f2231923_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x3441CAD2F2231923(vehicle, p1)
}

/// ## Parameters
* **vehicle**: 
* **p1**: 
* **p2**:

pub fn set_vehicle_automatically_attaches_safe(vehicle: Vehicle, p1: bool, p2: serde_json::Value) -> NativeResult<serde_json::Value> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p2_any_str = serde_json::to_string(&p2)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p2", e)))?;
    let p2_any_str_cstr = std::ffi::CString::new(p2_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p2", e)))?;
    
    debug!("Calling native function: SET_VEHICLE_AUTOMATICALLY_ATTACHES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_AUTOMATICALLY_ATTACHES(vehicle, p1, crate::utils::any_to_c_void_ptr(p2))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: SET_VEHICLE_AUTOMATICALLY_ATTACHES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_automatically_attaches_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_automatically_attaches_raw(vehicle: i32, p1: bool, p2: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_AUTOMATICALLY_ATTACHES(vehicle, p1, p2)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_boat_sinks_when_wrecked_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_BOAT_SINKS_WHEN_WRECKED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_BOAT_SINKS_WHEN_WRECKED(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_BOAT_SINKS_WHEN_WRECKED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_boat_sinks_when_wrecked_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_boat_sinks_when_wrecked_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_BOAT_SINKS_WHEN_WRECKED(vehicle)
}

/// ```
Has something to do with trains. Always precedes SET_MISSION_TRAIN_AS_NO_LONGER_NEEDED.  
============================================  
May be true that it can be used with trains not sure, but not specifically for trains. Go find Xbox360 decompiled scripts and search for 'func_1333' in freemode.c it isn't used just for trains. Thanks for the info tho.  
Btw, func_1333 ends up calling this func which uses this native,  
void func_1338(int iParam0)//Position   
{  
	ENTITY::FREEZE_ENTITY_POSITION(iParam0, true);  
	ENTITY::SET_ENTITY_COLLISION(iParam0, false, 0);  
	ENTITY::SET_ENTITY_INVINCIBLE(iParam0, true);  
	VEHICLE::_0xDF594D8D(iParam0, true);  
}  
```

pub fn _set_vehicle_st_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_ST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_ST(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_ST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_st_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_st_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_ST(vehicle, toggle)
}

/// Sets the amount of bombs that this vehicle has. As far as I know, this does _not_ impact vehicle weapons or the ammo of those weapons in any way, it is just a way to keep track of the amount of bombs in a specific plane. 

In decompiled scripts this is used to deduct from or add to the count whenever bombs are dropped or purchased/restocked. 

Use [`_GET_AIRCRAFT_BOMB_COUNT`](#_0xEA12BD130D7569A1) to get the amount of bombs on that vehicle.

pub fn _set_vehicle_bomb_count_safe(aircraft: Vehicle, bombCount: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_BOMB_COUNT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_BOMB_COUNT(aircraft, bombCount)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_BOMB_COUNT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_bomb_count_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_bomb_count_raw(aircraft: i32, bombCount: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_BOMB_COUNT(aircraft, bombCount)
}

/// ## Parameters
* **cargobob**: 
* **strength**:

pub fn set_cargobob_pickup_magnet_strength_safe(cargobob: Vehicle, strength: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CARGOBOB_PICKUP_MAGNET_STRENGTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CARGOBOB_PICKUP_MAGNET_STRENGTH(cargobob, strength)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CARGOBOB_PICKUP_MAGNET_STRENGTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_cargobob_pickup_magnet_strength_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_cargobob_pickup_magnet_strength_raw(cargobob: i32, strength: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CARGOBOB_PICKUP_MAGNET_STRENGTH(cargobob, strength)
}

/// ## Parameters
* **recording**: 
* **time**: 
* **script**:

pub fn get_rotation_of_vehicle_recording_at_time_safe(recording: i64, time: f32) -> NativeResult<Vector3> {
    
    debug!("Calling native function: GET_ROTATION_OF_VEHICLE_RECORDING_AT_TIME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_ROTATION_OF_VEHICLE_RECORDING_AT_TIME(recording, time)
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: GET_ROTATION_OF_VEHICLE_RECORDING_AT_TIME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_rotation_of_vehicle_recording_at_time_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_rotation_of_vehicle_recording_at_time_raw(recording: i64, time: f32) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_ROTATION_OF_VEHICLE_RECORDING_AT_TIME(recording, time)
}

/// ```
-1 = no livery  
```

pub fn get_vehicle_livery_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_LIVERY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_LIVERY(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_LIVERY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_livery_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_livery_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_LIVERY(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn pause_playback_recorded_vehicle_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: PAUSE_PLAYBACK_RECORDED_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::PAUSE_PLAYBACK_RECORDED_VEHICLE(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: PAUSE_PLAYBACK_RECORDED_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `pause_playback_recorded_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn pause_playback_recorded_vehicle_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::PAUSE_PLAYBACK_RECORDED_VEHICLE(vehicle)
}

/// ## Parameters
* **cargobob**: 
* **p1**:

pub fn set_cargobob_pickup_rope_damping_multiplier_safe(cargobob: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CARGOBOB_PICKUP_ROPE_DAMPING_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CARGOBOB_PICKUP_ROPE_DAMPING_MULTIPLIER(cargobob)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CARGOBOB_PICKUP_ROPE_DAMPING_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_cargobob_pickup_rope_damping_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_cargobob_pickup_rope_damping_multiplier_raw(cargobob: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CARGOBOB_PICKUP_ROPE_DAMPING_MULTIPLIER(cargobob)
}

/// ## Parameters
* **vehicle**: The vehicle handle
* **weaponIndex**: The weapon index we're getting ammo for (see [SET_VEHICLE_WEAPON_RESTRICTED_AMMO](#_0x44CD1F493DB2A0A6) for information on how to access these slots).

pub fn get_vehicle_weapon_restricted_ammo_safe(vehicle: Vehicle, weaponIndex: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_WEAPON_RESTRICTED_AMMO");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_WEAPON_RESTRICTED_AMMO(vehicle, weaponIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_WEAPON_RESTRICTED_AMMO
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_weapon_restricted_ammo_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_weapon_restricted_ammo_raw(vehicle: i32, weaponIndex: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_WEAPON_RESTRICTED_AMMO(vehicle, weaponIndex)
}

/// Returns the acceleration of the specified model.

pub fn get_vehicle_model_acceleration_safe(modelHash: u32) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_MODEL_ACCELERATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MODEL_ACCELERATION(modelHash)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MODEL_ACCELERATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_model_acceleration_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_model_acceleration_raw(modelHash: u32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MODEL_ACCELERATION(modelHash)
}

/// ```
NativeDB Introduced: v3095
```

Enables or disables the use of the vehicle's horn button for activating the nitrous system.

pub fn _set_vehicle_use_horn_button_for_nitrous_safe(vehicle: Vehicle, bToggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_USE_HORN_BUTTON_FOR_NITROUS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_USE_HORN_BUTTON_FOR_NITROUS(vehicle, bToggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_USE_HORN_BUTTON_FOR_NITROUS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_use_horn_button_for_nitrous_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_use_horn_button_for_nitrous_raw(vehicle: i32, bToggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_USE_HORN_BUTTON_FOR_NITROUS(vehicle, bToggle)
}

/// ```
Controls how fast the tires wear out.
Default values from Rockstar's Open Wheel Race JSON's:
"owrtss" (Soft): 2.2
"owrtsm" (Medium): 1.7
"owrtsh" (Hard): 1.2
Usable wheels:
0: wheel_lf
1: wheel_rf
2: wheel_lm1
3: wheel_rm1
4: wheel_lr
5: wheel_rr
```

```
NativeDB Introduced: v2060
```

pub fn _set_tyre_softness_multiplier_safe(vehicle: Vehicle, wheelIndex: i64, multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_TYRE_SOFTNESS_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_TYRE_SOFTNESS_MULTIPLIER(vehicle, wheelIndex, multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_TYRE_SOFTNESS_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_tyre_softness_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_tyre_softness_multiplier_raw(vehicle: i32, wheelIndex: i64, multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_TYRE_SOFTNESS_MULTIPLIER(vehicle, wheelIndex, multiplier)
}

/// ```
vehicle must be a plane
```

pub fn set_vehicle_uses_large_rear_ramp_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_USES_LARGE_REAR_RAMP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_USES_LARGE_REAR_RAMP(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_USES_LARGE_REAR_RAMP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_uses_large_rear_ramp_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_uses_large_rear_ramp_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_USES_LARGE_REAR_RAMP(vehicle, toggle)
}

/// Used to control train speed, can be used to start and stop its movement as well.

pub fn set_train_cruise_speed_safe(train: Vehicle, speed: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_TRAIN_CRUISE_SPEED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_TRAIN_CRUISE_SPEED(train, speed)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_TRAIN_CRUISE_SPEED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_train_cruise_speed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_train_cruise_speed_raw(train: i32, speed: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_TRAIN_CRUISE_SPEED(train, speed)
}

/// Remove the weird shadow applied by [_SET_VEHICLE_SHADOW_EFFECT](#_0x2A70BAE8883E4C81)

pub fn _remove_vehicle_shadow_effect_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _REMOVE_VEHICLE_SHADOW_EFFECT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_REMOVE_VEHICLE_SHADOW_EFFECT(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _REMOVE_VEHICLE_SHADOW_EFFECT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_remove_vehicle_shadow_effect_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _remove_vehicle_shadow_effect_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_REMOVE_VEHICLE_SHADOW_EFFECT(vehicle)
}

/// ```
Returns whether this vehicle is currently disabled by an EMP mine.

NativeDB Introduced: v1604
```

pub fn _get_is_vehicle_emp_disabled_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _GET_IS_VEHICLE_EMP_DISABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_IS_VEHICLE_EMP_DISABLED(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_IS_VEHICLE_EMP_DISABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_is_vehicle_emp_disabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_is_vehicle_emp_disabled_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_IS_VEHICLE_EMP_DISABLED(vehicle)
}

/// ## Parameters
* **model**:

pub fn is_this_model_a_boat_safe(model: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_THIS_MODEL_A_BOAT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_THIS_MODEL_A_BOAT(model)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_THIS_MODEL_A_BOAT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_this_model_a_boat_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_this_model_a_boat_raw(model: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_THIS_MODEL_A_BOAT(model)
}

/// Sets the boat boom position for the `TR3` trailer.

Ratio value is between `0.0` and `1.0`, where `0.0` is 90 degrees to the left of the boat, and `1.0` is just slightly to the right/back of the boat.

To get the current boom position ratio, use [GET_BOAT_BOOM_POSITION_RATIO](#_0x6636C535F6CC2725).

pub fn _set_boat_boom_position_ratio_safe(vehicle: Vehicle, ratio: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_BOAT_BOOM_POSITION_RATIO");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_BOAT_BOOM_POSITION_RATIO(vehicle, ratio)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_BOAT_BOOM_POSITION_RATIO
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_boat_boom_position_ratio_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_boat_boom_position_ratio_raw(vehicle: i32, ratio: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_BOAT_BOOM_POSITION_RATIO(vehicle, ratio)
}

/// ```
indices:  
0 = Left  
1 = Right  
2 = Front  
3 = Back  
```

pub fn _is_vehicle_neon_light_enabled_safe(vehicle: Vehicle, index: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_VEHICLE_NEON_LIGHT_ENABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_VEHICLE_NEON_LIGHT_ENABLED(vehicle, index)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_VEHICLE_NEON_LIGHT_ENABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_vehicle_neon_light_enabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_vehicle_neon_light_enabled_raw(vehicle: i32, index: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_VEHICLE_NEON_LIGHT_ENABLED(vehicle, index)
}

/// ## Parameters
* **recording**: 
* **script**:

pub fn has_vehicle_recording_been_loaded_safe(recording: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_VEHICLE_RECORDING_BEEN_LOADED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_VEHICLE_RECORDING_BEEN_LOADED(recording)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_VEHICLE_RECORDING_BEEN_LOADED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_vehicle_recording_been_loaded_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_vehicle_recording_been_loaded_raw(recording: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_VEHICLE_RECORDING_BEEN_LOADED(recording)
}

/// SET_VEHICLE_HEADLIGHT_SHADOWS native function

pub fn set_vehicle_headlight_shadows_safe(vehicle: Vehicle, flag: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_HEADLIGHT_SHADOWS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_HEADLIGHT_SHADOWS(vehicle, flag)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_HEADLIGHT_SHADOWS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_headlight_shadows_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_headlight_shadows_raw(vehicle: i32, flag: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_HEADLIGHT_SHADOWS(vehicle, flag)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_active_for_ped_navigation_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_ACTIVE_FOR_PED_NAVIGATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_ACTIVE_FOR_PED_NAVIGATION(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_ACTIVE_FOR_PED_NAVIGATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_active_for_ped_navigation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_active_for_ped_navigation_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_ACTIVE_FOR_PED_NAVIGATION(vehicle, toggle)
}

/// ## Parameters
* **vehicleClass**:

pub fn get_vehicle_class_max_braking_safe(vehicleClass: i64) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_CLASS_MAX_BRAKING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_CLASS_MAX_BRAKING(vehicleClass)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_CLASS_MAX_BRAKING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_class_max_braking_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_class_max_braking_raw(vehicleClass: i64) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_CLASS_MAX_BRAKING(vehicleClass)
}

/// ```
Public Function isVehicleOnAllWheels(vh As Vehicle) As Boolean  
Return Native.Function.Call(Of Boolean)(Hash.IS_VEHICLE_ON_ALL_WHEELS, vh)  
		    End Function  
```

pub fn is_vehicle_on_all_wheels_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_ON_ALL_WHEELS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_ON_ALL_WHEELS(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_ON_ALL_WHEELS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_on_all_wheels_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_on_all_wheels_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_ON_ALL_WHEELS(vehicle)
}

/// This native does no interpolation between pathpoints. The same position will be returned for all times up to the next pathpoint in the recording.

See [`REQUEST_VEHICLE_RECORDING`](#_0xAF514CABE74CBF15).

pub fn get_position_of_vehicle_recording_at_time_safe(recording: i64, time: f32, script: String) -> NativeResult<Vector3> {
    let script_cstr = std::ffi::CString::new(script.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "script", e)))?;
    
    debug!("Calling native function: GET_POSITION_OF_VEHICLE_RECORDING_AT_TIME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_POSITION_OF_VEHICLE_RECORDING_AT_TIME(recording, time, crate::utils::rust_to_c_string(script))
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: GET_POSITION_OF_VEHICLE_RECORDING_AT_TIME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_position_of_vehicle_recording_at_time_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_position_of_vehicle_recording_at_time_raw(recording: i64, time: f32, script: *const std::os::raw::c_char) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_POSITION_OF_VEHICLE_RECORDING_AT_TIME(recording, time, script)
}

/// ```
Usage:  
public bool isCopInRange(Vector3 Location, float Range)  
        {  
            return Function.Call<bool>(Hash.IS_COP_PED_IN_AREA_3D, Location.X - Range, Location.Y - Range, Location.Z - Range, Location.X + Range, Location.Y + Range, Location.Z + Range);  
        }  
```

pub fn is_cop_vehicle_in_area_3d_safe(x1: f32, x2: f32, y1: f32, y2: f32, z1: f32, z2: f32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_COP_VEHICLE_IN_AREA_3D");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_COP_VEHICLE_IN_AREA_3D(x1, x2, y1, y2, z1, z2)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_COP_VEHICLE_IN_AREA_3D
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_cop_vehicle_in_area_3d_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_cop_vehicle_in_area_3d_raw(x1: f32, x2: f32, y1: f32, y2: f32, z1: f32, z2: f32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_COP_VEHICLE_IN_AREA_3D(x1, x2, y1, y2, z1, z2)
}

/// ## Parameters
* **plane**: 
* **height**:

pub fn set_task_vehicle_goto_plane_min_height_above_terrain_safe(plane: Vehicle, height: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_TASK_VEHICLE_GOTO_PLANE_MIN_HEIGHT_ABOVE_TERRAIN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_TASK_VEHICLE_GOTO_PLANE_MIN_HEIGHT_ABOVE_TERRAIN(plane, height)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_TASK_VEHICLE_GOTO_PLANE_MIN_HEIGHT_ABOVE_TERRAIN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_task_vehicle_goto_plane_min_height_above_terrain_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_task_vehicle_goto_plane_min_height_above_terrain_raw(plane: i32, height: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_TASK_VEHICLE_GOTO_PLANE_MIN_HEIGHT_ABOVE_TERRAIN(plane, height)
}

/// ## Parameters
* **train**: 
* **speed**:

pub fn set_train_speed_safe(train: Vehicle, speed: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_TRAIN_SPEED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_TRAIN_SPEED(train, speed)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_TRAIN_SPEED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_train_speed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_train_speed_raw(train: i32, speed: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_TRAIN_SPEED(train, speed)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_use_player_light_settings_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_USE_PLAYER_LIGHT_SETTINGS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_USE_PLAYER_LIGHT_SETTINGS(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_USE_PLAYER_LIGHT_SETTINGS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_use_player_light_settings_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_use_player_light_settings_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_USE_PLAYER_LIGHT_SETTINGS(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn _set_cambered_wheels_disabled_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_CAMBERED_WHEELS_DISABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_CAMBERED_WHEELS_DISABLED(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_CAMBERED_WHEELS_DISABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_cambered_wheels_disabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_cambered_wheels_disabled_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_CAMBERED_WHEELS_DISABLED(vehicle, toggle)
}

/// ```
Appears to return false if any window is broken.  
```

pub fn are_all_vehicle_windows_intact_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: ARE_ALL_VEHICLE_WINDOWS_INTACT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ARE_ALL_VEHICLE_WINDOWS_INTACT(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: ARE_ALL_VEHICLE_WINDOWS_INTACT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `are_all_vehicle_windows_intact_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn are_all_vehicle_windows_intact_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ARE_ALL_VEHICLE_WINDOWS_INTACT(vehicle)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0xd3e51c0ab8c26eee_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<serde_json::Value> {
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
    
    debug!("Calling native function: _0xD3E51C0AB8C26EEE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xD3E51C0AB8C26EEE(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0xD3E51C0AB8C26EEE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xd3e51c0ab8c26eee_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xd3e51c0ab8c26eee_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xD3E51C0AB8C26EEE(p0, p1)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x35bb21de06784373_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x35BB21DE06784373");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x35BB21DE06784373(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x35BB21DE06784373
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x35bb21de06784373_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x35bb21de06784373_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x35BB21DE06784373(p0, p1)
}

/// ```
Only used in R* Script fm_content_cargo
```

```
NativeDB Introduced: v2699
```

pub fn _0xef9d388f8d377f44_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xEF9D388F8D377F44");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xEF9D388F8D377F44(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xEF9D388F8D377F44
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xef9d388f8d377f44_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xef9d388f8d377f44_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xEF9D388F8D377F44(vehicle, p1)
}

/// ```
Only works on bikes, both X and Y work in the -1 - 1 range.
X forces the bike to turn left or right (-1, 1)
Y forces the bike to lean to the left or to the right (-1, 1)
Example with X -1/Y 1
http://i.imgur.com/TgIuAPJ.jpg
```

pub fn set_bike_on_stand_safe(vehicle: Vehicle, x: f32, y: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_BIKE_ON_STAND");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_BIKE_ON_STAND(vehicle, x, y)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_BIKE_ON_STAND
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_bike_on_stand_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_bike_on_stand_raw(vehicle: i32, x: f32, y: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_BIKE_ON_STAND(vehicle, x, y)
}

/// ## Parameters
* **vehicle**: The vehicle handle.

pub fn get_vehicle_has_kers_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_VEHICLE_HAS_KERS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_HAS_KERS(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_HAS_KERS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_has_kers_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_has_kers_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_HAS_KERS(vehicle)
}

/// ```
Money pickups are created around cars when they explode. Only works when the vehicle model is a car. A single pickup is between 1 and 18 dollars in size. All car models seem to give the same amount of money.
youtu.be/3arlUxzHl5Y
i.imgur.com/WrNpYFs.jpg
```

pub fn set_vehicle_drops_money_when_blown_up_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DROPS_MONEY_WHEN_BLOWN_UP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DROPS_MONEY_WHEN_BLOWN_UP(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DROPS_MONEY_WHEN_BLOWN_UP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_drops_money_when_blown_up_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_drops_money_when_blown_up_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DROPS_MONEY_WHEN_BLOWN_UP(vehicle, toggle)
}

/// Toggles whether ambient trains can spawn on the specified track or not.

| trackId | File | Description |
| --- | --- | --- |
| 0 | `trains1.dat` | Main track around SA |
| 1 | `trains2.dat` | Davis Quartz Quarry branch |
| 2 | `trains3.dat` | Second track alongside live track along Roy Lewenstein Blv. |
| 3 | `trains4.dat` | Metro track circuit |
| 4 | `trains5.dat` | Branch in Mirror Park Railyard |
| 5 | `trains6.dat` | Branch in Mirror Park Railyard |
| 6 | `trains7.dat` | LS branch to Mirror Park Railyard |
| 7 | `trains8.dat` | Overground part of metro track along Forum Dr. |
| 8 | `trains9.dat` | Branch to Mirror Park Railyard |
| 9 | `trains10.dat` | Yankton train |
| 10 | `trains11.dat` | Part of metro track near mission row |
| 11 | `trains12.dat` | Yankton prologue mission train |

Tracks IDs `0` and `3` are the main tracks you find trains on during normal gameplay, all the others are used during missions and are not complete tracks.

pub fn switch_train_track_safe(trackId: i64, state: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SWITCH_TRAIN_TRACK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SWITCH_TRAIN_TRACK(trackId, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SWITCH_TRAIN_TRACK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `switch_train_track_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn switch_train_track_raw(trackId: i64, state: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SWITCH_TRAIN_TRACK(trackId, state)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**: 
* **p3**:

pub fn set_vehicle_use_cutscene_wheel_compression_safe(p0: Vehicle, p1: bool, p2: bool, p3: bool) -> NativeResult<serde_json::Value> {
    
    debug!("Calling native function: SET_VEHICLE_USE_CUTSCENE_WHEEL_COMPRESSION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_USE_CUTSCENE_WHEEL_COMPRESSION(p0, p1, p2, p3)
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: SET_VEHICLE_USE_CUTSCENE_WHEEL_COMPRESSION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_use_cutscene_wheel_compression_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_use_cutscene_wheel_compression_raw(p0: i32, p1: bool, p2: bool, p3: bool) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_USE_CUTSCENE_WHEEL_COMPRESSION(p0, p1, p2, p3)
}

/// ```
GET_VEHICLE_MODEL_*
Function pertains only to aviation vehicles.
```

pub fn _get_vehicle_model_max_knots_safe(modelHash: u32) -> NativeResult<f32> {
    
    debug!("Calling native function: _GET_VEHICLE_MODEL_MAX_KNOTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_MODEL_MAX_KNOTS(modelHash)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_VEHICLE_MODEL_MAX_KNOTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_model_max_knots_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_model_max_knots_raw(modelHash: u32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_MODEL_MAX_KNOTS(modelHash)
}

/// ## Parameters
* **vehicle**:

pub fn is_taxi_light_on_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_TAXI_LIGHT_ON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_TAXI_LIGHT_ON(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_TAXI_LIGHT_ON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_taxi_light_on_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_taxi_light_on_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_TAXI_LIGHT_ON(vehicle)
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)

pub fn _get_entry_position_of_door_safe(vehicle: Vehicle, doorIndex: i64) -> NativeResult<Vector3> {
    
    debug!("Calling native function: _GET_ENTRY_POSITION_OF_DOOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_ENTRY_POSITION_OF_DOOR(vehicle, doorIndex)
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: _GET_ENTRY_POSITION_OF_DOOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_entry_position_of_door_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_entry_position_of_door_raw(vehicle: i32, doorIndex: i64) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_ENTRY_POSITION_OF_DOOR(vehicle, doorIndex)
}

/// ## Parameters
* **vehicle**:

pub fn _disable_vehicle_turret_movement_this_frame_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _DISABLE_VEHICLE_TURRET_MOVEMENT_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_DISABLE_VEHICLE_TURRET_MOVEMENT_THIS_FRAME(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _DISABLE_VEHICLE_TURRET_MOVEMENT_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_disable_vehicle_turret_movement_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _disable_vehicle_turret_movement_this_frame_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_DISABLE_VEHICLE_TURRET_MOVEMENT_THIS_FRAME(vehicle)
}

/// ```
seems to make the vehicle stop spawning naturally in traffic. Here's an essential example:  
VEHICLE::SET_VEHICLE_MODEL_IS_SUPPRESSED(GAMEPLAY::GET_HASH_KEY("taco"), true);  
```

pub fn set_vehicle_model_is_suppressed_safe(model: u32, suppressed: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_MODEL_IS_SUPPRESSED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_MODEL_IS_SUPPRESSED(model, suppressed)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_MODEL_IS_SUPPRESSED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_model_is_suppressed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_model_is_suppressed_raw(model: u32, suppressed: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_MODEL_IS_SUPPRESSED(model, suppressed)
}

/// ## Parameters
* **cargobob**: 
* **vehicle**:

pub fn set_cargobob_pickup_magnet_reduced_strength_safe(cargobob: Vehicle, vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CARGOBOB_PICKUP_MAGNET_REDUCED_STRENGTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CARGOBOB_PICKUP_MAGNET_REDUCED_STRENGTH(cargobob, vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CARGOBOB_PICKUP_MAGNET_REDUCED_STRENGTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_cargobob_pickup_magnet_reduced_strength_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_cargobob_pickup_magnet_reduced_strength_raw(cargobob: i32, vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CARGOBOB_PICKUP_MAGNET_REDUCED_STRENGTH(cargobob, vehicle)
}

/// ```
Returns the number of *types* of licence plates, enumerated below in SET_VEHICLE_NUMBER_PLATE_TEXT_INDEX.  
```

pub fn get_number_of_vehicle_number_plates_safe() -> NativeResult<i64> {
    
    debug!("Calling native function: GET_NUMBER_OF_VEHICLE_NUMBER_PLATES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_NUMBER_OF_VEHICLE_NUMBER_PLATES()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_NUMBER_OF_VEHICLE_NUMBER_PLATES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_number_of_vehicle_number_plates_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_number_of_vehicle_number_plates_raw() -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_NUMBER_OF_VEHICLE_NUMBER_PLATES()
}

/// ## Parameters
* **vehicle**: 
* **health**:

pub fn _set_heli_main_rotor_health_safe(vehicle: Vehicle, health: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_HELI_MAIN_ROTOR_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_HELI_MAIN_ROTOR_HEALTH(vehicle, health)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_HELI_MAIN_ROTOR_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_heli_main_rotor_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_heli_main_rotor_health_raw(vehicle: i32, health: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_HELI_MAIN_ROTOR_HEALTH(vehicle, health)
}

/// ```
p1 can be anywhere from 0 to 3 in the scripts. p2 is generally somewhere in the 1000 to 10000 range.  
```

pub fn is_vehicle_stuck_timer_up_safe(vehicle: Vehicle, p1: i64, p2: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_STUCK_TIMER_UP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_STUCK_TIMER_UP(vehicle, p1, p2)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_STUCK_TIMER_UP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_stuck_timer_up_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_stuck_timer_up_raw(vehicle: i32, p1: i64, p2: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_STUCK_TIMER_UP(vehicle, p1, p2)
}

/// See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).

This function is coded to not work on vehicles of type: `CBike`, `Bmx`, `CBoat`, `CTrain`, and `CSubmarine`.

pub fn fix_vehicle_window_safe(vehicle: Vehicle, windowIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: FIX_VEHICLE_WINDOW");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::FIX_VEHICLE_WINDOW(vehicle, windowIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: FIX_VEHICLE_WINDOW
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `fix_vehicle_window_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn fix_vehicle_window_raw(vehicle: i32, windowIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::FIX_VEHICLE_WINDOW(vehicle, windowIndex)
}

/// ```c
enum eColourBitField {
    HAS_BODY_COLOUR1 = 1,
    HAS_BODY_COLOUR2 = 2,
    HAS_BODY_COLOUR3 = 4,
    HAS_BODY_COLOUR4 = 8,
    HAS_BODY_COLOUR5 = 16
}
```

pub fn get_vehicle_colours_which_can_be_set_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_COLOURS_WHICH_CAN_BE_SET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_COLOURS_WHICH_CAN_BE_SET(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_COLOURS_WHICH_CAN_BE_SET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_colours_which_can_be_set_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_colours_which_can_be_set_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_COLOURS_WHICH_CAN_BE_SET(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **multiplier**:

pub fn _set_vehicle_unk_damage_multiplier_safe(vehicle: Vehicle, multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_UNK_DAMAGE_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_UNK_DAMAGE_MULTIPLIER(vehicle, multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_UNK_DAMAGE_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_unk_damage_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_unk_damage_multiplier_raw(vehicle: i32, multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_UNK_DAMAGE_MULTIPLIER(vehicle, multiplier)
}

/// ## Parameters
* **vehicle**:

pub fn close_bomb_bay_doors_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: CLOSE_BOMB_BAY_DOORS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLOSE_BOMB_BAY_DOORS(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLOSE_BOMB_BAY_DOORS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `close_bomb_bay_doors_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn close_bomb_bay_doors_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLOSE_BOMB_BAY_DOORS(vehicle)
}

/// ## Parameters
* **vehicleAsset**:

pub fn has_vehicle_asset_loaded_safe(vehicleAsset: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_VEHICLE_ASSET_LOADED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_VEHICLE_ASSET_LOADED(vehicleAsset)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_VEHICLE_ASSET_LOADED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_vehicle_asset_loaded_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_vehicle_asset_loaded_raw(vehicleAsset: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_VEHICLE_ASSET_LOADED(vehicleAsset)
}

/// ## Parameters
* **cargobob**: 
* **vehicleAttached**:

pub fn is_vehicle_attached_to_cargobob_safe(cargobob: Vehicle, vehicleAttached: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_ATTACHED_TO_CARGOBOB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_ATTACHED_TO_CARGOBOB(cargobob, vehicleAttached)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_ATTACHED_TO_CARGOBOB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_attached_to_cargobob_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_attached_to_cargobob_raw(cargobob: i32, vehicleAttached: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_ATTACHED_TO_CARGOBOB(cargobob, vehicleAttached)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_inactive_during_playback_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_INACTIVE_DURING_PLAYBACK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_INACTIVE_DURING_PLAYBACK(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_INACTIVE_DURING_PLAYBACK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_inactive_during_playback_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_inactive_during_playback_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_INACTIVE_DURING_PLAYBACK(vehicle, toggle)
}

/// ```
Returns true if vehicle is halted by BRING_VEHICLE_TO_HALT
_IS_VEHICLE_*
```

```
NativeDB Introduced: v1493
```

pub fn _is_vehicle_being_halted_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_VEHICLE_BEING_HALTED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_VEHICLE_BEING_HALTED(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_VEHICLE_BEING_HALTED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_vehicle_being_halted_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_vehicle_being_halted_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_VEHICLE_BEING_HALTED(vehicle)
}

/// Examples with a besra:

- [fade value `0.0`](https://i.imgur.com/DXNk63e.jpg)
- [fade value `0.5`](https://i.imgur.com/2Vb35fq.jpg)
- [fade value `1.0`](https://i.imgur.com/aa8cxaD.jpg)

The parameter fade is a value from 0-1, where 0 is fresh paint.

pub fn set_vehicle_enveff_scale_safe(vehicle: Vehicle, fade: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_ENVEFF_SCALE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_ENVEFF_SCALE(vehicle, fade)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_ENVEFF_SCALE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_enveff_scale_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_enveff_scale_raw(vehicle: i32, fade: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_ENVEFF_SCALE(vehicle, fade)
}

/// ## Parameters
* **aircraft**: The vehicle to check the hover mode on.

pub fn get_vehicle_flight_nozzle_position_safe(aircraft: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_FLIGHT_NOZZLE_POSITION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_FLIGHT_NOZZLE_POSITION(aircraft)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_FLIGHT_NOZZLE_POSITION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_flight_nozzle_position_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_flight_nozzle_position_raw(aircraft: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_FLIGHT_NOZZLE_POSITION(aircraft)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_doors_locked_for_non_script_players_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DOORS_LOCKED_FOR_NON_SCRIPT_PLAYERS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DOORS_LOCKED_FOR_NON_SCRIPT_PLAYERS(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DOORS_LOCKED_FOR_NON_SCRIPT_PLAYERS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_doors_locked_for_non_script_players_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_doors_locked_for_non_script_players_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DOORS_LOCKED_FOR_NON_SCRIPT_PLAYERS(vehicle, toggle)
}

/// ## Parameters
* **model**:

pub fn _is_this_model_an_amphibious_quadbike_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_THIS_MODEL_AN_AMPHIBIOUS_QUADBIKE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_THIS_MODEL_AN_AMPHIBIOUS_QUADBIKE()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_THIS_MODEL_AN_AMPHIBIOUS_QUADBIKE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_this_model_an_amphibious_quadbike_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_this_model_an_amphibious_quadbike_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_THIS_MODEL_AN_AMPHIBIOUS_QUADBIKE()
}

/// Disables collision for this vehicle (maybe it also supports other entities, not sure).
Only world/building/fixed world objects will have their collisions disabled, props, peds, or any other entity still collides with the vehicle.

[Example video](https://streamable.com/6n45d5)

Not sure if there is a native (and if so, which one) that resets the collisions.

pub fn _disable_vehicle_world_collision_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _DISABLE_VEHICLE_WORLD_COLLISION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_DISABLE_VEHICLE_WORLD_COLLISION(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _DISABLE_VEHICLE_WORLD_COLLISION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_disable_vehicle_world_collision_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _disable_vehicle_world_collision_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_DISABLE_VEHICLE_WORLD_COLLISION(vehicle)
}

/// ## Parameters
* **range**: Most likely a value between 0.0 and 1.0.

pub fn set_ambient_vehicle_range_multiplier_this_frame_safe(range: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_AMBIENT_VEHICLE_RANGE_MULTIPLIER_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_AMBIENT_VEHICLE_RANGE_MULTIPLIER_THIS_FRAME(range)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_AMBIENT_VEHICLE_RANGE_MULTIPLIER_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_ambient_vehicle_range_multiplier_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_ambient_vehicle_range_multiplier_this_frame_raw(range: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_AMBIENT_VEHICLE_RANGE_MULTIPLIER_THIS_FRAME(range)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn _set_vehicle_receives_ramp_damage_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_RECEIVES_RAMP_DAMAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_RECEIVES_RAMP_DAMAGE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_RECEIVES_RAMP_DAMAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_receives_ramp_damage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_receives_ramp_damage_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_RECEIVES_RAMP_DAMAGE(vehicle, toggle)
}

/// ## Parameters
* **vehicle**:

pub fn clear_vehicle_custom_secondary_colour_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_VEHICLE_CUSTOM_SECONDARY_COLOUR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_VEHICLE_CUSTOM_SECONDARY_COLOUR(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_VEHICLE_CUSTOM_SECONDARY_COLOUR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_vehicle_custom_secondary_colour_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_vehicle_custom_secondary_colour_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_VEHICLE_CUSTOM_SECONDARY_COLOUR(vehicle)
}

/// Transforms the `stormberg` to its "road vehicle" variant. If the vehicle is already in that state then the vehicle transformation audio will still play, but the vehicle won't change at all.

pub fn transform_to_car_safe(vehicle: Vehicle, instantly: bool) -> NativeResult<()> {
    
    debug!("Calling native function: TRANSFORM_TO_CAR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::TRANSFORM_TO_CAR(vehicle, instantly)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: TRANSFORM_TO_CAR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `transform_to_car_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn transform_to_car_raw(vehicle: i32, instantly: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::TRANSFORM_TO_CAR(vehicle, instantly)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_rudder_broken_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_RUDDER_BROKEN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_RUDDER_BROKEN(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_RUDDER_BROKEN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_rudder_broken_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_rudder_broken_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_RUDDER_BROKEN(vehicle, toggle)
}

/// ```
Implemented only for Trains.
```

```
NativeDB Introduced: v2372
```

pub fn _network_use_high_precision_vehicle_blending_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _NETWORK_USE_HIGH_PRECISION_VEHICLE_BLENDING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_NETWORK_USE_HIGH_PRECISION_VEHICLE_BLENDING(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _NETWORK_USE_HIGH_PRECISION_VEHICLE_BLENDING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_network_use_high_precision_vehicle_blending_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _network_use_high_precision_vehicle_blending_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_NETWORK_USE_HIGH_PRECISION_VEHICLE_BLENDING(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **speed**:

pub fn set_vehicle_turret_speed_this_frame_safe(vehicle: Vehicle, speed: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_TURRET_SPEED_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_TURRET_SPEED_THIS_FRAME(vehicle, speed)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_TURRET_SPEED_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_turret_speed_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_turret_speed_this_frame_raw(vehicle: i32, speed: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_TURRET_SPEED_THIS_FRAME(vehicle, speed)
}

/// ```
Returns last vehicle that was rammed by the given vehicle using the shunt boost.

NativeDB Introduced: v1604
```

pub fn _get_last_rammed_vehicle_safe(vehicle: Vehicle) -> NativeResult<Vehicle> {
    
    debug!("Calling native function: _GET_LAST_RAMMED_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_LAST_RAMMED_VEHICLE(vehicle)
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: _GET_LAST_RAMMED_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_last_rammed_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_last_rammed_vehicle_raw(vehicle: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_LAST_RAMMED_VEHICLE(vehicle)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**: 
* **p5**: 
* **p6**:

pub fn get_random_vehicle_front_bumper_in_sphere_safe(p0: f32, p1: f32, p2: f32, p3: f32, p4: i64, p5: i64, p6: i64) -> NativeResult<Vehicle> {
    
    debug!("Calling native function: GET_RANDOM_VEHICLE_FRONT_BUMPER_IN_SPHERE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_RANDOM_VEHICLE_FRONT_BUMPER_IN_SPHERE(p0, p1, p2, p3, p4, p5, p6)
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: GET_RANDOM_VEHICLE_FRONT_BUMPER_IN_SPHERE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_random_vehicle_front_bumper_in_sphere_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_random_vehicle_front_bumper_in_sphere_raw(p0: f32, p1: f32, p2: f32, p3: f32, p4: i64, p5: i64, p6: i64) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_RANDOM_VEHICLE_FRONT_BUMPER_IN_SPHERE(p0, p1, p2, p3, p4, p5, p6)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn _set_vehicle_can_engine_operate_on_fire_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_CAN_ENGINE_OPERATE_ON_FIRE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_CAN_ENGINE_OPERATE_ON_FIRE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_CAN_ENGINE_OPERATE_ON_FIRE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_can_engine_operate_on_fire_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_can_engine_operate_on_fire_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_CAN_ENGINE_OPERATE_ON_FIRE(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: The vehicle for which to obtain the estimated max speed.

pub fn get_vehicle_estimated_max_speed_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_ESTIMATED_MAX_SPEED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_ESTIMATED_MAX_SPEED(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_ESTIMATED_MAX_SPEED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_estimated_max_speed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_estimated_max_speed_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_ESTIMATED_MAX_SPEED(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_needs_to_be_hotwired_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_NEEDS_TO_BE_HOTWIRED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_NEEDS_TO_BE_HOTWIRED(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_NEEDS_TO_BE_HOTWIRED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_needs_to_be_hotwired_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_needs_to_be_hotwired_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_NEEDS_TO_BE_HOTWIRED(vehicle, toggle)
}

/// ## Parameters
* **vehicleClass**:

pub fn get_vehicle_class_estimated_max_speed_safe(vehicleClass: i64) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_CLASS_ESTIMATED_MAX_SPEED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_CLASS_ESTIMATED_MAX_SPEED(vehicleClass)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_CLASS_ESTIMATED_MAX_SPEED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_class_estimated_max_speed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_class_estimated_max_speed_raw(vehicleClass: i64) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_CLASS_ESTIMATED_MAX_SPEED(vehicleClass)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn set_cargobob_pickup_magnet_effect_radius_safe(vehicle: Vehicle, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CARGOBOB_PICKUP_MAGNET_EFFECT_RADIUS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CARGOBOB_PICKUP_MAGNET_EFFECT_RADIUS(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CARGOBOB_PICKUP_MAGNET_EFFECT_RADIUS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_cargobob_pickup_magnet_effect_radius_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_cargobob_pickup_magnet_effect_radius_raw(vehicle: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CARGOBOB_PICKUP_MAGNET_EFFECT_RADIUS(vehicle, p1)
}

/// ## Parameters
* **vehicle**: 
* **modType**: Refer to eVehicleModType in [`SET_VEHICLE_MOD`](#_0x6AF0636DDEDCB6DD).

pub fn is_toggle_mod_on_safe(vehicle: Vehicle, modType: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_TOGGLE_MOD_ON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_TOGGLE_MOD_ON(vehicle, modType)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_TOGGLE_MOD_ON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_toggle_mod_on_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_toggle_mod_on_raw(vehicle: i32, modType: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_TOGGLE_MOD_ON(vehicle, modType)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn reset_vehicle_wheels_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: RESET_VEHICLE_WHEELS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_VEHICLE_WHEELS(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_VEHICLE_WHEELS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_vehicle_wheels_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_vehicle_wheels_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_VEHICLE_WHEELS(vehicle, toggle)
}

/// ```
Controls how much traction the wheel loses.
Default values from Rockstar's Open Wheel Race JSON's:
"owrtds" (Soft): 0.05
"owrtdm" (Medium): 0.45
"owrtdh" (Hard): 0.8
Usable wheels:
0: wheel_lf
1: wheel_rf
2: wheel_lm1
3: wheel_rm1
4: wheel_lr
5: wheel_rr
```

```
NativeDB Introduced: v2060
```

pub fn _set_tyre_traction_loss_multiplier_safe(vehicle: Vehicle, wheelIndex: i64, multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_TYRE_TRACTION_LOSS_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_TYRE_TRACTION_LOSS_MULTIPLIER(vehicle, wheelIndex, multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_TYRE_TRACTION_LOSS_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_tyre_traction_loss_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_tyre_traction_loss_multiplier_raw(vehicle: i32, wheelIndex: i64, multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_TYRE_TRACTION_LOSS_MULTIPLIER(vehicle, wheelIndex, multiplier)
}

/// ## Parameters
* **p0**:

pub fn _0x8533cafde1f0f336_safe(p0: serde_json::Value) -> NativeResult<serde_json::Value> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x8533CAFDE1F0F336");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x8533CAFDE1F0F336(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0x8533CAFDE1F0F336
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x8533cafde1f0f336_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x8533cafde1f0f336_raw(p0: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x8533CAFDE1F0F336(p0)
}

/// Stops cargobob from being able to detach the attached vehicle.

pub fn set_cargobob_force_dont_detach_vehicle_safe(cargobob: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CARGOBOB_FORCE_DONT_DETACH_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CARGOBOB_FORCE_DONT_DETACH_VEHICLE(cargobob, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CARGOBOB_FORCE_DONT_DETACH_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_cargobob_force_dont_detach_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_cargobob_force_dont_detach_vehicle_raw(cargobob: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CARGOBOB_FORCE_DONT_DETACH_VEHICLE(cargobob, toggle)
}

/// ## Parameters
* **x**: 
* **y**: 
* **z**: 
* **radius**: 
* **speed**: 
* **p5**:

pub fn add_road_node_speed_zone_safe(x: f32, y: f32, z: f32, radius: f32, speed: f32, p5: bool) -> NativeResult<i64> {
    
    debug!("Calling native function: ADD_ROAD_NODE_SPEED_ZONE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ADD_ROAD_NODE_SPEED_ZONE(x, y, z, radius, speed, p5)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: ADD_ROAD_NODE_SPEED_ZONE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `add_road_node_speed_zone_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn add_road_node_speed_zone_raw(x: f32, y: f32, z: f32, radius: f32, speed: f32, p5: bool) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ADD_ROAD_NODE_SPEED_ZONE(x, y, z, radius, speed, p5)
}

/// ## Parameters
* **vehicle**:

pub fn get_vehicle_mod_kit_type_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_MOD_KIT_TYPE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MOD_KIT_TYPE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MOD_KIT_TYPE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_mod_kit_type_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_mod_kit_type_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MOD_KIT_TYPE(vehicle)
}

/// ```
p1, p2, p3 are RGB values for color (255,0,0 for Red, ect)  
```

pub fn set_vehicle_custom_secondary_colour_safe(vehicle: Vehicle, r: i64, g: i64, b: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_CUSTOM_SECONDARY_COLOUR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_CUSTOM_SECONDARY_COLOUR(vehicle, r, g, b)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_CUSTOM_SECONDARY_COLOUR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_custom_secondary_colour_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_custom_secondary_colour_raw(vehicle: i32, r: i64, g: i64, b: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_CUSTOM_SECONDARY_COLOUR(vehicle, r, g, b)
}

/// ```
NativeDB Introduced: v1604
```

pub fn _0xa4a9a4c40e615885_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xA4A9A4C40E615885");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xA4A9A4C40E615885(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xA4A9A4C40E615885
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xa4a9a4c40e615885_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xa4a9a4c40e615885_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xA4A9A4C40E615885(p0)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x5ba68a0840d546ac_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<serde_json::Value> {
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
    
    debug!("Calling native function: _0x5BA68A0840D546AC");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x5BA68A0840D546AC(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0x5BA68A0840D546AC
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x5ba68a0840d546ac_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x5ba68a0840d546ac_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x5BA68A0840D546AC(p0, p1)
}

/// ```
Changes the secondary paint type and color  
paintType:  
0: Normal  
1: Metallic  
2: Pearl  
3: Matte  
4: Metal  
5: Chrome  
color: number of the color  
```

pub fn set_vehicle_mod_color_2_safe(vehicle: Vehicle, paintType: i64, color: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_MOD_COLOR_2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_MOD_COLOR_2(vehicle, paintType, color)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_MOD_COLOR_2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_mod_color_2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_mod_color_2_raw(vehicle: i32, paintType: i64, color: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_MOD_COLOR_2(vehicle, paintType, color)
}

/// ```
min: 1.9f, max: 100.0f
```

pub fn set_pickup_rope_length_for_cargobob_safe(cargobob: Vehicle, length1: f32, length2: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PICKUP_ROPE_LENGTH_FOR_CARGOBOB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PICKUP_ROPE_LENGTH_FOR_CARGOBOB(cargobob, length1, length2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PICKUP_ROPE_LENGTH_FOR_CARGOBOB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_pickup_rope_length_for_cargobob_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_pickup_rope_length_for_cargobob_raw(cargobob: i32, length1: f32, length2: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PICKUP_ROPE_LENGTH_FOR_CARGOBOB(cargobob, length1, length2)
}

/// ## Parameters
* **vehicle**:

pub fn release_preload_mods_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: RELEASE_PRELOAD_MODS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RELEASE_PRELOAD_MODS(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RELEASE_PRELOAD_MODS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `release_preload_mods_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn release_preload_mods_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RELEASE_PRELOAD_MODS(vehicle)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0xd4196117af7bb974_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<serde_json::Value> {
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
    
    debug!("Calling native function: _0xD4196117AF7BB974");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xD4196117AF7BB974(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0xD4196117AF7BB974
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xd4196117af7bb974_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xd4196117af7bb974_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xD4196117AF7BB974(p0, p1)
}

/// ## Parameters
* **vehicle**:

pub fn detach_vehicle_from_trailer_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: DETACH_VEHICLE_FROM_TRAILER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DETACH_VEHICLE_FROM_TRAILER(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DETACH_VEHICLE_FROM_TRAILER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `detach_vehicle_from_trailer_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn detach_vehicle_from_trailer_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DETACH_VEHICLE_FROM_TRAILER(vehicle)
}

/// ## Parameters
* **toggle**:

pub fn _0x51db102f4a3ba5e0_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x51DB102F4A3BA5E0");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x51DB102F4A3BA5E0(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x51DB102F4A3BA5E0
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x51db102f4a3ba5e0_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x51db102f4a3ba5e0_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x51DB102F4A3BA5E0(toggle)
}

/// Will disable a plane or a helicopter's need to swerve around object in its heightmap when using TASK_PLANE_MISSION or other AI / Pilot behavior.  Will ensure plane flys directly to it's destination or die trying! This native does NOT need to be called every frame, but instead, just called once on the vehicle (NOT THE PED) you're trying to disable avoidance for!

pub fn _enable_aircraft_obstacle_avoidance_safe(vehicle: Vehicle, avoidObstacles: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _ENABLE_AIRCRAFT_OBSTACLE_AVOIDANCE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_ENABLE_AIRCRAFT_OBSTACLE_AVOIDANCE(vehicle, avoidObstacles)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _ENABLE_AIRCRAFT_OBSTACLE_AVOIDANCE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_enable_aircraft_obstacle_avoidance_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _enable_aircraft_obstacle_avoidance_raw(vehicle: i32, avoidObstacles: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_ENABLE_AIRCRAFT_OBSTACLE_AVOIDANCE(vehicle, avoidObstacles)
}

/// ```
Returns true if the vehicle's current speed is less than, or equal to 0.0025f.
For some vehicles it returns true if the current speed is <= 0.00039999999.
```

pub fn is_vehicle_stopped_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_STOPPED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_STOPPED(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_STOPPED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_stopped_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_stopped_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_STOPPED(vehicle)
}

/// ```
Only works during nighttime.
```

pub fn set_vehicle_searchlight_safe(heli: Vehicle, toggle: bool, canBeUsedByAI: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_SEARCHLIGHT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_SEARCHLIGHT(heli, toggle, canBeUsedByAI)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_SEARCHLIGHT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_searchlight_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_searchlight_raw(heli: i32, toggle: bool, canBeUsedByAI: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_SEARCHLIGHT(heli, toggle, canBeUsedByAI)
}

/// ```
True stops vtols from switching modes. Doesn't stop the sound though.
```

```
NativeDB Introduced: v1290
```

pub fn _set_disable_vehicle_flight_nozzle_position_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_DISABLE_VEHICLE_FLIGHT_NOZZLE_POSITION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_DISABLE_VEHICLE_FLIGHT_NOZZLE_POSITION(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_DISABLE_VEHICLE_FLIGHT_NOZZLE_POSITION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_disable_vehicle_flight_nozzle_position_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_disable_vehicle_flight_nozzle_position_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_DISABLE_VEHICLE_FLIGHT_NOZZLE_POSITION(vehicle)
}

/// ## Parameters
* **vehicle**: The vehicle to check.
* **seatIndex**: See eSeatPosition declared in [`IS_VEHICLE_SEAT_FREE`](#_0x22AC59A870E6A669).

pub fn is_turret_seat_safe(vehicle: Vehicle, seatIndex: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_TURRET_SEAT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_TURRET_SEAT(vehicle, seatIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_TURRET_SEAT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_turret_seat_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_turret_seat_raw(vehicle: i32, seatIndex: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_TURRET_SEAT(vehicle, seatIndex)
}

/// CAN_ANCHOR_BOAT_HERE_IGNORE_PLAYERS native function

pub fn can_anchor_boat_here_ignore_players_safe(boat: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: CAN_ANCHOR_BOAT_HERE_IGNORE_PLAYERS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CAN_ANCHOR_BOAT_HERE_IGNORE_PLAYERS(boat)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CAN_ANCHOR_BOAT_HERE_IGNORE_PLAYERS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `can_anchor_boat_here_ignore_players_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn can_anchor_boat_here_ignore_players_raw(boat: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CAN_ANCHOR_BOAT_HERE_IGNORE_PLAYERS(boat)
}

/// Determines whether the specified vehicle is equipped with a searchlight.

```
NativeDB Introduced: v2189
```

pub fn does_vehicle_have_searchlight_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: DOES_VEHICLE_HAVE_SEARCHLIGHT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DOES_VEHICLE_HAVE_SEARCHLIGHT(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: DOES_VEHICLE_HAVE_SEARCHLIGHT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `does_vehicle_have_searchlight_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn does_vehicle_have_searchlight_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DOES_VEHICLE_HAVE_SEARCHLIGHT(vehicle)
}

/// Allows vehicles with the FLAG_JUMPING_CAR flag to jump higher (i.e. Ruiner 2000).

pub fn _set_use_higher_vehicle_jump_force_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_USE_HIGHER_VEHICLE_JUMP_FORCE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_USE_HIGHER_VEHICLE_JUMP_FORCE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_USE_HIGHER_VEHICLE_JUMP_FORCE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_use_higher_vehicle_jump_force_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_use_higher_vehicle_jump_force_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_USE_HIGHER_VEHICLE_JUMP_FORCE(vehicle, toggle)
}

/// ## Parameters
* **vehicle**:

pub fn get_num_mod_kits_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_NUM_MOD_KITS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_NUM_MOD_KITS(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_NUM_MOD_KITS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_num_mod_kits_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_num_mod_kits_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_NUM_MOD_KITS(vehicle)
}

/// ```
This fixes the deformation of a vehicle but the vehicle health doesn't improve  
```

pub fn set_vehicle_deformation_fixed_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DEFORMATION_FIXED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DEFORMATION_FIXED(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DEFORMATION_FIXED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_deformation_fixed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_deformation_fixed_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DEFORMATION_FIXED(vehicle)
}

/// This multiplier has no limit, by default the game has this set to `1.0`.

pub fn set_vehicle_light_multiplier_safe(vehicle: Vehicle, multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_LIGHT_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_LIGHT_MULTIPLIER(vehicle, multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_LIGHT_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_light_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_light_multiplier_raw(vehicle: i32, multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_LIGHT_MULTIPLIER(vehicle, multiplier)
}

/// ```
NativeDB Introduced: v3407
```

pub fn _set_plane_avoids_others_safe(plane: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PLANE_AVOIDS_OTHERS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PLANE_AVOIDS_OTHERS(plane, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PLANE_AVOIDS_OTHERS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_plane_avoids_others_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_plane_avoids_others_raw(plane: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PLANE_AVOIDS_OTHERS(plane, toggle)
}

/// ```
NativeDB Introduced: v3095
```

Recharges the nitrous system of the specified vehicle to its maximum capacity. This action sets the nitrous charge duration to the maximum limit defined by previous settings applied through [`SET_OVERRIDE_NITROUS_LEVEL`](#_0xC8E9B6B71B8E660D).

pub fn fully_charge_nitrous_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: FULLY_CHARGE_NITROUS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::FULLY_CHARGE_NITROUS(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: FULLY_CHARGE_NITROUS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `fully_charge_nitrous_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn fully_charge_nitrous_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::FULLY_CHARGE_NITROUS(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **player**: 
* **toggle**:

pub fn set_vehicle_doors_locked_for_player_safe(vehicle: Vehicle, player: Player, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DOORS_LOCKED_FOR_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DOORS_LOCKED_FOR_PLAYER(vehicle, player, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DOORS_LOCKED_FOR_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_doors_locked_for_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_doors_locked_for_player_raw(vehicle: i32, player: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DOORS_LOCKED_FOR_PLAYER(vehicle, player, toggle)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x0419b167ee128f33_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<serde_json::Value> {
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
    
    debug!("Calling native function: _0x0419B167EE128F33");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x0419B167EE128F33(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0x0419B167EE128F33
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x0419b167ee128f33_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x0419b167ee128f33_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x0419B167EE128F33(p0, p1)
}

/// ```
Returns a float value between 0.0 and 3.0 related to its slipstream draft (boost/speedup).
GET_VEHICLE_*
```

pub fn _get_vehicle_current_slipstream_draft_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: _GET_VEHICLE_CURRENT_SLIPSTREAM_DRAFT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_CURRENT_SLIPSTREAM_DRAFT(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_VEHICLE_CURRENT_SLIPSTREAM_DRAFT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_current_slipstream_draft_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_current_slipstream_draft_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_CURRENT_SLIPSTREAM_DRAFT(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_engine_can_degrade_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_ENGINE_CAN_DEGRADE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_ENGINE_CAN_DEGRADE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_ENGINE_CAN_DEGRADE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_engine_can_degrade_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_engine_can_degrade_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_ENGINE_CAN_DEGRADE(vehicle, toggle)
}

/// colorPrimary & colorSecondary are the paint indexes for the vehicle.  

For a list of valid paint indexes, view: pastebin.com/pwHci0xK

pub fn set_vehicle_colours_safe(vehicle: Vehicle, colorPrimary: i64, colorSecondary: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_COLOURS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_COLOURS(vehicle, colorPrimary, colorSecondary)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_COLOURS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_colours_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_colours_raw(vehicle: i32, colorPrimary: i64, colorSecondary: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_COLOURS(vehicle, colorPrimary, colorSecondary)
}

/// Set state to true to extend the wings, false to retract them.

pub fn _set_oppressor_transform_state_safe(vehicle: Vehicle, extend: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_OPPRESSOR_TRANSFORM_STATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_OPPRESSOR_TRANSFORM_STATE(vehicle, extend)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_OPPRESSOR_TRANSFORM_STATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_oppressor_transform_state_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_oppressor_transform_state_raw(vehicle: i32, extend: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_OPPRESSOR_TRANSFORM_STATE(vehicle, extend)
}

/// ```
Returns false if the vehicle has the FLAG_NO_RESPRAY flag set.
```

pub fn is_vehicle_sprayable_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_SPRAYABLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_SPRAYABLE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_SPRAYABLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_sprayable_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_sprayable_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_SPRAYABLE(vehicle)
}

/// ```
If set to TRUE, it seems to suppress door noises and doesn't allow the horn to be continuous.  
```

pub fn _set_vehicle_silent_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_SILENT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_SILENT(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_SILENT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_silent_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_silent_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_SILENT(vehicle, toggle)
}

/// A getter for [`SET_VEHICLE_DIRT_LEVEL`](#_0x79D3B596FE44EE8B).

pub fn get_vehicle_dirt_level_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_DIRT_LEVEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_DIRT_LEVEL(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_DIRT_LEVEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_dirt_level_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_dirt_level_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_DIRT_LEVEL(vehicle)
}

/// ## Parameters
* **p0**:

pub fn _0xc4b3347bd68bd609_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xC4B3347BD68BD609");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xC4B3347BD68BD609(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xC4B3347BD68BD609
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xc4b3347bd68bd609_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xc4b3347bd68bd609_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xC4B3347BD68BD609(p0)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _0x6ebfb22d646ffc18_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x6EBFB22D646FFC18");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x6EBFB22D646FFC18(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x6EBFB22D646FFC18
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x6ebfb22d646ffc18_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x6ebfb22d646ffc18_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x6EBFB22D646FFC18(vehicle, p1)
}

/// ```
NativeDB Introduced: v1604
```

pub fn _0x5bbcf35bf6e456f7_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x5BBCF35BF6E456F7");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x5BBCF35BF6E456F7(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x5BBCF35BF6E456F7
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x5bbcf35bf6e456f7_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x5bbcf35bf6e456f7_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x5BBCF35BF6E456F7(toggle)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0xe05dd0e9707003a3_safe(p0: serde_json::Value, p1: bool) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xE05DD0E9707003A3");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xE05DD0E9707003A3(crate::utils::any_to_c_void_ptr(p0), p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xE05DD0E9707003A3
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xe05dd0e9707003a3_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xe05dd0e9707003a3_raw(p0: *mut std::os::raw::c_void, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xE05DD0E9707003A3(p0, p1)
}

/// ```
NativeDB Introduced: v1604
```

pub fn _get_does_vehicle_have_tombstone_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _GET_DOES_VEHICLE_HAVE_TOMBSTONE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_DOES_VEHICLE_HAVE_TOMBSTONE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_DOES_VEHICLE_HAVE_TOMBSTONE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_does_vehicle_have_tombstone_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_does_vehicle_have_tombstone_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_DOES_VEHICLE_HAVE_TOMBSTONE(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**: 
* **depth1**: 
* **depth2**: 
* **depth3**:

pub fn set_submarine_crush_depths_safe(vehicle: Vehicle, toggle: bool, depth1: f32, depth2: f32, depth3: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SUBMARINE_CRUSH_DEPTHS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SUBMARINE_CRUSH_DEPTHS(vehicle, toggle, depth1, depth2, depth3)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SUBMARINE_CRUSH_DEPTHS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_submarine_crush_depths_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_submarine_crush_depths_raw(vehicle: i32, toggle: bool, depth1: f32, depth2: f32, depth3: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SUBMARINE_CRUSH_DEPTHS(vehicle, toggle, depth1, depth2, depth3)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x73561d4425a021a2_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x73561D4425A021A2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x73561D4425A021A2(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x73561D4425A021A2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x73561d4425a021a2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x73561d4425a021a2_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x73561D4425A021A2(p0, p1)
}

/// ```
AI abides by the provided driving style (e.g., stopping at red lights or waiting behind traffic) while executing the specificed vehicle recording.

0x1F2E4E06DEA8992B is a related native that deals with the AI physics for such recordings.
```

pub fn start_playback_recorded_vehicle_using_ai_safe(vehicle: Vehicle, recording: i64) -> NativeResult<()> {
    
    debug!("Calling native function: START_PLAYBACK_RECORDED_VEHICLE_USING_AI");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::START_PLAYBACK_RECORDED_VEHICLE_USING_AI(vehicle, recording)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: START_PLAYBACK_RECORDED_VEHICLE_USING_AI
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `start_playback_recorded_vehicle_using_ai_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn start_playback_recorded_vehicle_using_ai_raw(vehicle: i32, recording: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::START_PLAYBACK_RECORDED_VEHICLE_USING_AI(vehicle, recording)
}

/// ```
Seems to be related to the metal parts, not tyres (like i was expecting lol)  
Must be called every tick.  
```

pub fn set_vehicle_friction_override_safe(vehicle: Vehicle, friction: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_FRICTION_OVERRIDE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_FRICTION_OVERRIDE(vehicle, friction)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_FRICTION_OVERRIDE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_friction_override_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_friction_override_raw(vehicle: i32, friction: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_FRICTION_OVERRIDE(vehicle, friction)
}

/// ## Parameters
* **vehicle**: 
* **lightsOn**: 
* **highbeamsOn**:

pub fn get_vehicle_lights_state_safe(vehicle: Vehicle, lightsOn: *mut bool, highbeamsOn: *mut bool) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_VEHICLE_LIGHTS_STATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_LIGHTS_STATE(vehicle, lightsOn, highbeamsOn)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_LIGHTS_STATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_lights_state_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_lights_state_raw(vehicle: i32, lightsOn: *mut bool, highbeamsOn: *mut bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_LIGHTS_STATE(vehicle, lightsOn, highbeamsOn)
}

/// ## Parameters
* **p0**:

pub fn remove_vehicle_combat_avoidance_area_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: REMOVE_VEHICLE_COMBAT_AVOIDANCE_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_VEHICLE_COMBAT_AVOIDANCE_AREA(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_VEHICLE_COMBAT_AVOIDANCE_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_vehicle_combat_avoidance_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_vehicle_combat_avoidance_area_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_VEHICLE_COMBAT_AVOIDANCE_AREA(p0)
}

/// ## Parameters
* **toggle**:

pub fn set_far_draw_vehicles_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_FAR_DRAW_VEHICLES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_FAR_DRAW_VEHICLES(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_FAR_DRAW_VEHICLES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_far_draw_vehicles_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_far_draw_vehicles_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_FAR_DRAW_VEHICLES(toggle)
}

/// ## Parameters
* **vehicle**:

pub fn is_vehicle_visible_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_VISIBLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_VISIBLE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_VISIBLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_visible_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_visible_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_VISIBLE(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_force_afterburner_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_FORCE_AFTERBURNER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_FORCE_AFTERBURNER(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_FORCE_AFTERBURNER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_force_afterburner_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_force_afterburner_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_FORCE_AFTERBURNER(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **entity**:

pub fn is_entity_attached_to_handler_frame_safe(vehicle: Vehicle, entity: Entity) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_ENTITY_ATTACHED_TO_HANDLER_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_ENTITY_ATTACHED_TO_HANDLER_FRAME(vehicle, entity)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_ENTITY_ATTACHED_TO_HANDLER_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_entity_attached_to_handler_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_entity_attached_to_handler_frame_raw(vehicle: i32, entity: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_ENTITY_ATTACHED_TO_HANDLER_FRAME(vehicle, entity)
}

/// GET_HELI_TAIL_ROTOR_HEALTH native function

pub fn get_heli_tail_rotor_health_safe(heli: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_HELI_TAIL_ROTOR_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_HELI_TAIL_ROTOR_HEALTH(heli)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_HELI_TAIL_ROTOR_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_heli_tail_rotor_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_heli_tail_rotor_health_raw(heli: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_HELI_TAIL_ROTOR_HEALTH(heli)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_can_deform_wheels_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_CAN_DEFORM_WHEELS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_CAN_DEFORM_WHEELS(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_CAN_DEFORM_WHEELS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_can_deform_wheels_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_can_deform_wheels_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_CAN_DEFORM_WHEELS(vehicle, toggle)
}

/// #### Vehicles with both roofed and roofless versions (others may exist; this list is compiled from decompiled scripts).

* chino
* voltic
* buccaneer
* buccaneer2
* chino2
* faction
* faction2
* mamba

pub fn set_convertible_roof_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CONVERTIBLE_ROOF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CONVERTIBLE_ROOF(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CONVERTIBLE_ROOF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_convertible_roof_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_convertible_roof_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CONVERTIBLE_ROOF(vehicle, toggle)
}

/// ```
Related to locking the vehicle or something similar.  
In the decompiled scripts, its always called after  
VEHICLE::_SET_EXCLUSIVE_DRIVER(a_0, 0, 0);  
VEHICLE::SET_VEHICLE_DOORS_LOCKED_FOR_ALL_PLAYERS(a_0, 1);  
VEHICLE::SET_VEHICLE_DOORS_LOCKED_FOR_PLAYER(a_0, PLAYER::PLAYER_ID(), 0);  
```

pub fn _0xdbc631f109350b8c_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xDBC631F109350B8C");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xDBC631F109350B8C(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xDBC631F109350B8C
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xdbc631f109350b8c_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xdbc631f109350b8c_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xDBC631F109350B8C(vehicle, p1)
}

/// ```
Returns true if the vehicle has the FLAG_ALLOWS_RAPPEL flag set.
```

pub fn _does_vehicle_allow_rappel_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _DOES_VEHICLE_ALLOW_RAPPEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_DOES_VEHICLE_ALLOW_RAPPEL(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _DOES_VEHICLE_ALLOW_RAPPEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_does_vehicle_allow_rappel_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _does_vehicle_allow_rappel_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_DOES_VEHICLE_ALLOW_RAPPEL(vehicle)
}

/// ```
Set modKit to 0 if you plan to call SET_VEHICLE_MOD. That's what the game does. Most body modifications through SET_VEHICLE_MOD will not take effect until this is set to 0.
```

pub fn set_vehicle_mod_kit_safe(vehicle: Vehicle, modKit: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_MOD_KIT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_MOD_KIT(vehicle, modKit)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_MOD_KIT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_mod_kit_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_mod_kit_raw(vehicle: i32, modKit: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_MOD_KIT(vehicle, modKit)
}

/// ## Parameters
* **p0**:

pub fn _0x7bbe7ff626a591fe_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x7BBE7FF626A591FE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x7BBE7FF626A591FE(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x7BBE7FF626A591FE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x7bbe7ff626a591fe_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x7bbe7ff626a591fe_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x7BBE7FF626A591FE(p0)
}

/// ## Parameters
* **vehicle**:

pub fn get_vehicle_colour_combination_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_COLOUR_COMBINATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_COLOUR_COMBINATION(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_COLOUR_COMBINATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_colour_combination_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_colour_combination_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_COLOUR_COMBINATION(vehicle)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0xb9562064627ff9db_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0xB9562064627FF9DB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xB9562064627FF9DB(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xB9562064627FF9DB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xb9562064627ff9db_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xb9562064627ff9db_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xB9562064627FF9DB(p0, p1)
}

/// ## Parameters
* **vehicle**:

pub fn get_vehicle_max_traction_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_MAX_TRACTION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MAX_TRACTION(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MAX_TRACTION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_max_traction_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_max_traction_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MAX_TRACTION(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn _get_has_retractable_wheels_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _GET_HAS_RETRACTABLE_WHEELS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_HAS_RETRACTABLE_WHEELS(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_HAS_RETRACTABLE_WHEELS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_has_retractable_wheels_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_has_retractable_wheels_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_HAS_RETRACTABLE_WHEELS(vehicle)
}

/// Determines whether the specified Cargobob can pick up a given entity.

pub fn can_cargobob_pick_up_entity_safe(cargobob: Vehicle, entity: Entity) -> NativeResult<bool> {
    
    debug!("Calling native function: CAN_CARGOBOB_PICK_UP_ENTITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CAN_CARGOBOB_PICK_UP_ENTITY(cargobob, entity)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CAN_CARGOBOB_PICK_UP_ENTITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `can_cargobob_pick_up_entity_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn can_cargobob_pick_up_entity_raw(cargobob: i32, entity: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CAN_CARGOBOB_PICK_UP_ENTITY(cargobob, entity)
}

/// ```
Won't attract or magnetize to any helicopters or planes of course, but that's common sense.  
```

pub fn set_cargobob_pickup_magnet_active_safe(cargobob: Vehicle, isActive: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CARGOBOB_PICKUP_MAGNET_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CARGOBOB_PICKUP_MAGNET_ACTIVE(cargobob, isActive)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CARGOBOB_PICKUP_MAGNET_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_cargobob_pickup_magnet_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_cargobob_pickup_magnet_active_raw(cargobob: i32, isActive: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CARGOBOB_PICKUP_MAGNET_ACTIVE(cargobob, isActive)
}

/// ```
Closes all doors of a vehicle:  
```

pub fn set_vehicle_doors_shut_safe(vehicle: Vehicle, closeInstantly: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DOORS_SHUT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DOORS_SHUT(vehicle, closeInstantly)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DOORS_SHUT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_doors_shut_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_doors_shut_raw(vehicle: i32, closeInstantly: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DOORS_SHUT(vehicle, closeInstantly)
}

/// See [`SET_VEHICLE_CUSTOM_PRIMARY_COLOUR`](#_0x7141766F91D15BEA) and [`SET_VEHICLE_CUSTOM_SECONDARY_COLOUR`](#_0x36CED73BFED89754).

pub fn get_vehicle_color_safe(vehicle: Vehicle, r: *mut i64, g: *mut i64, b: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_VEHICLE_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_COLOR(vehicle, r, g, b)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_VEHICLE_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_color_raw(vehicle: i32, r: *mut i64, g: *mut i64, b: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_COLOR(vehicle, r, g, b)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _0x9f3f689b814f2599_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x9F3F689B814F2599");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x9F3F689B814F2599(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x9F3F689B814F2599
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x9f3f689b814f2599_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x9f3f689b814f2599_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x9F3F689B814F2599(vehicle, p1)
}

/// ## Parameters
* **vehicle**:

pub fn is_vehicle_alarm_activated_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_ALARM_ACTIVATED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_ALARM_ACTIVATED(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_ALARM_ACTIVATED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_alarm_activated_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_alarm_activated_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_ALARM_ACTIVATED(vehicle)
}

/// Disables the additional physics forces applied to BMX bikes that enable them to perform tricks.

```
NativeDB Introduced: v463
```

pub fn set_disable_bmx_extra_trick_forces_safe() -> NativeResult<()> {
    
    debug!("Calling native function: SET_DISABLE_BMX_EXTRA_TRICK_FORCES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DISABLE_BMX_EXTRA_TRICK_FORCES()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DISABLE_BMX_EXTRA_TRICK_FORCES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_disable_bmx_extra_trick_forces_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_disable_bmx_extra_trick_forces_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DISABLE_BMX_EXTRA_TRICK_FORCES()
}

/// ## Parameters
* **train**: 
* **x**: 
* **y**: 
* **z**:

pub fn set_mission_train_coords_safe(train: Vehicle, x: f32, y: f32, z: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_MISSION_TRAIN_COORDS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_MISSION_TRAIN_COORDS(train, x, y, z)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_MISSION_TRAIN_COORDS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_mission_train_coords_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_mission_train_coords_raw(train: i32, x: f32, y: f32, z: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_MISSION_TRAIN_COORDS(train, x, y, z)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_use_alternate_handling_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_USE_ALTERNATE_HANDLING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_USE_ALTERNATE_HANDLING(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_USE_ALTERNATE_HANDLING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_use_alternate_handling_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_use_alternate_handling_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_USE_ALTERNATE_HANDLING(vehicle, toggle)
}

/// ## Parameters
* **towTruck**:

pub fn get_entity_attached_to_tow_truck_safe(towTruck: Vehicle) -> NativeResult<Entity> {
    
    debug!("Calling native function: GET_ENTITY_ATTACHED_TO_TOW_TRUCK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_ENTITY_ATTACHED_TO_TOW_TRUCK(towTruck)
    };
    
    
    Ok(Ok(Entity(result)))
}

/// Raw native function: GET_ENTITY_ATTACHED_TO_TOW_TRUCK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_entity_attached_to_tow_truck_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_entity_attached_to_tow_truck_raw(towTruck: i32) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_ENTITY_ATTACHED_TO_TOW_TRUCK(towTruck)
}

/// ## Parameters
* **vehicle**:

pub fn _set_boat_is_sinking_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_BOAT_IS_SINKING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_BOAT_IS_SINKING(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_BOAT_IS_SINKING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_boat_is_sinking_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_boat_is_sinking_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_BOAT_IS_SINKING(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn detach_vehicle_from_any_tow_truck_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: DETACH_VEHICLE_FROM_ANY_TOW_TRUCK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DETACH_VEHICLE_FROM_ANY_TOW_TRUCK(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: DETACH_VEHICLE_FROM_ANY_TOW_TRUCK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `detach_vehicle_from_any_tow_truck_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn detach_vehicle_from_any_tow_truck_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DETACH_VEHICLE_FROM_ANY_TOW_TRUCK(vehicle)
}

/// ```
This has not yet been tested - it's just an assumption of what the types could be.  
```

pub fn set_vehicle_can_be_targetted_safe(vehicle: Vehicle, state: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_CAN_BE_TARGETTED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_CAN_BE_TARGETTED(vehicle, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_CAN_BE_TARGETTED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_can_be_targetted_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_can_be_targetted_raw(vehicle: i32, state: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_CAN_BE_TARGETTED(vehicle, state)
}

/// ## Parameters
* **vehicle**: 
* **model**:

pub fn is_vehicle_model_safe(vehicle: Vehicle, model: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_MODEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_MODEL(vehicle, model)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_MODEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_model_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_model_raw(vehicle: i32, model: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_MODEL(vehicle, model)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _0xa7dcdf4ded40a8f4_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xA7DCDF4DED40A8F4");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xA7DCDF4DED40A8F4(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xA7DCDF4DED40A8F4
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xa7dcdf4ded40a8f4_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xa7dcdf4ded40a8f4_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xA7DCDF4DED40A8F4(vehicle, p1)
}

/// ```
Does nothing. It's a nullsub.

NativeDB Introduced: v1604
```

pub fn _0x36de109527a2c0c4_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x36DE109527A2C0C4");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x36DE109527A2C0C4(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x36DE109527A2C0C4
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x36de109527a2c0c4_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x36de109527a2c0c4_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x36DE109527A2C0C4(toggle)
}

/// ## Parameters
* **vehicle**:

pub fn _is_vehicle_slipstream_leader_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_VEHICLE_SLIPSTREAM_LEADER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_VEHICLE_SLIPSTREAM_LEADER(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_VEHICLE_SLIPSTREAM_LEADER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_vehicle_slipstream_leader_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_vehicle_slipstream_leader_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_VEHICLE_SLIPSTREAM_LEADER(vehicle)
}

/// ```
tyreIndex = 0 to 4 on normal vehicles  
'0 = wheel_lf / bike, plane or jet front  
'1 = wheel_rf  
'2 = wheel_lm / in 6 wheels trailer, plane or jet is first one on left  
'3 = wheel_rm / in 6 wheels trailer, plane or jet is first one on right  
'4 = wheel_lr / bike rear / in 6 wheels trailer, plane or jet is last one on left  
'5 = wheel_rr / in 6 wheels trailer, plane or jet is last one on right  
'45 = 6 wheels trailer mid wheel left  
'47 = 6 wheels trailer mid wheel right  
```

pub fn set_vehicle_tyre_fixed_safe(vehicle: Vehicle, tyreIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_TYRE_FIXED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_TYRE_FIXED(vehicle, tyreIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_TYRE_FIXED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_tyre_fixed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_tyre_fixed_raw(vehicle: i32, tyreIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_TYRE_FIXED(vehicle, tyreIndex)
}

/// ## Parameters
* **vehicle**:

pub fn _raise_retractable_wheels_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _RAISE_RETRACTABLE_WHEELS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_RAISE_RETRACTABLE_WHEELS(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _RAISE_RETRACTABLE_WHEELS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_raise_retractable_wheels_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _raise_retractable_wheels_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_RAISE_RETRACTABLE_WHEELS(vehicle)
}

/// ## Parameters
* **vehicle**: Vehicle to get the state of

pub fn get_convertible_roof_state_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_CONVERTIBLE_ROOF_STATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_CONVERTIBLE_ROOF_STATE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_CONVERTIBLE_ROOF_STATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_convertible_roof_state_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_convertible_roof_state_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_CONVERTIBLE_ROOF_STATE(vehicle)
}

/// _GET_IS_VEHICLE_ELECTRIC native function

pub fn _get_is_vehicle_electric_safe(vehicleModel: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: _GET_IS_VEHICLE_ELECTRIC");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_IS_VEHICLE_ELECTRIC(vehicleModel)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_IS_VEHICLE_ELECTRIC
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_is_vehicle_electric_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_is_vehicle_electric_raw(vehicleModel: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_IS_VEHICLE_ELECTRIC(vehicleModel)
}

/// ## Parameters
* **vehicle**:

pub fn is_vehicle_stuck_on_roof_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_STUCK_ON_ROOF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_STUCK_ON_ROOF(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_STUCK_ON_ROOF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_stuck_on_roof_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_stuck_on_roof_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_STUCK_ON_ROOF(vehicle)
}

/// SET_BOAT_LOW_LOD_ANCHOR_DISTANCE native function

pub fn set_boat_low_lod_anchor_distance_safe(boat: Vehicle, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_BOAT_LOW_LOD_ANCHOR_DISTANCE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_BOAT_LOW_LOD_ANCHOR_DISTANCE(boat, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_BOAT_LOW_LOD_ANCHOR_DISTANCE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_boat_low_lod_anchor_distance_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_boat_low_lod_anchor_distance_raw(boat: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_BOAT_LOW_LOD_ANCHOR_DISTANCE(boat, value)
}

/// ## Parameters
* **vehicle**: 
* **x**: 
* **y**: 
* **z**: 
* **p4**:

pub fn _0x5845066d8a1ea7f7_safe(vehicle: Vehicle, x: f32, y: f32, z: f32, p4: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p4_any_str = serde_json::to_string(&p4)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p4", e)))?;
    let p4_any_str_cstr = std::ffi::CString::new(p4_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p4", e)))?;
    
    debug!("Calling native function: _0x5845066D8A1EA7F7");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x5845066D8A1EA7F7(vehicle, x, y, z, crate::utils::any_to_c_void_ptr(p4))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x5845066D8A1EA7F7
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x5845066d8a1ea7f7_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x5845066d8a1ea7f7_raw(vehicle: i32, x: f32, y: f32, z: f32, p4: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x5845066D8A1EA7F7(vehicle, x, y, z, p4)
}

/// Lowers the vehicle's stance. Only works for vehicles that support this feature.

```
NativeDB Introduced: v2372
```

pub fn _set_reduce_drift_vehicle_suspension_safe(vehicle: Vehicle, enable: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_REDUCE_DRIFT_VEHICLE_SUSPENSION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_REDUCE_DRIFT_VEHICLE_SUSPENSION(vehicle, enable)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_REDUCE_DRIFT_VEHICLE_SUSPENSION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_reduce_drift_vehicle_suspension_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_reduce_drift_vehicle_suspension_raw(vehicle: i32, enable: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_REDUCE_DRIFT_VEHICLE_SUSPENSION(vehicle, enable)
}

/// Sets the dirt level of the passed vehicle.

pub fn set_vehicle_dirt_level_safe(vehicle: Vehicle, dirtLevel: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DIRT_LEVEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DIRT_LEVEL(vehicle, dirtLevel)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DIRT_LEVEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_dirt_level_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_dirt_level_raw(vehicle: i32, dirtLevel: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DIRT_LEVEL(vehicle, dirtLevel)
}

/// Drops the Hook/Magnet on a cargobob  

```c
enum eCargobobHook  
{  
	CARGOBOB_HOOK = 0,  
	CARGOBOB_MAGNET = 1,  
};  
```

pub fn create_pick_up_rope_for_cargobob_safe(cargobob: Vehicle, state: i64) -> NativeResult<()> {
    
    debug!("Calling native function: CREATE_PICK_UP_ROPE_FOR_CARGOBOB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CREATE_PICK_UP_ROPE_FOR_CARGOBOB(cargobob, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CREATE_PICK_UP_ROPE_FOR_CARGOBOB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `create_pick_up_rope_for_cargobob_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn create_pick_up_rope_for_cargobob_raw(cargobob: i32, state: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CREATE_PICK_UP_ROPE_FOR_CARGOBOB(cargobob, state)
}

/// ```
how does this work?  
```

pub fn disable_vehicle_weapon_safe(disabled: bool, weaponHash: u32, vehicle: Vehicle, owner: Ped) -> NativeResult<()> {
    
    debug!("Calling native function: DISABLE_VEHICLE_WEAPON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DISABLE_VEHICLE_WEAPON(disabled, weaponHash, vehicle, owner)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DISABLE_VEHICLE_WEAPON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `disable_vehicle_weapon_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn disable_vehicle_weapon_raw(disabled: bool, weaponHash: u32, vehicle: i32, owner: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DISABLE_VEHICLE_WEAPON(disabled, weaponHash, vehicle, owner)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _0x56eb5e94318d3fb6_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x56EB5E94318D3FB6");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x56EB5E94318D3FB6(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x56EB5E94318D3FB6
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x56eb5e94318d3fb6_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x56eb5e94318d3fb6_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x56EB5E94318D3FB6(vehicle, p1)
}

/// Forces a submarine to maintain neutral buoyancy for a specified duration, preventing it from rising when unoccupied or without a driver.

```
NativeDB Introduced: v2189
```

pub fn force_submarine_neurtal_buoyancy_safe(submarine: Vehicle, time: i64) -> NativeResult<()> {
    
    debug!("Calling native function: FORCE_SUBMARINE_NEURTAL_BUOYANCY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::FORCE_SUBMARINE_NEURTAL_BUOYANCY(submarine, time)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: FORCE_SUBMARINE_NEURTAL_BUOYANCY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `force_submarine_neurtal_buoyancy_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn force_submarine_neurtal_buoyancy_raw(submarine: i32, time: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::FORCE_SUBMARINE_NEURTAL_BUOYANCY(submarine, time)
}

/// ```
Returns the license plate text from a vehicle.  8 chars maximum.  
```

pub fn get_vehicle_number_plate_text_safe(vehicle: Vehicle) -> NativeResult<String> {
    
    debug!("Calling native function: GET_VEHICLE_NUMBER_PLATE_TEXT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_NUMBER_PLATE_TEXT(vehicle)
    };
    
    
    Ok(crate::utils::c_string_to_rust_string(result))
}

/// Raw native function: GET_VEHICLE_NUMBER_PLATE_TEXT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_number_plate_text_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_number_plate_text_raw(vehicle: i32) -> *const std::os::raw::c_char {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_NUMBER_PLATE_TEXT(vehicle)
}

/// ```
Returns true only when the hook is active, will return false if the magnet is active  
```

pub fn does_cargobob_have_pick_up_rope_safe(cargobob: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: DOES_CARGOBOB_HAVE_PICK_UP_ROPE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DOES_CARGOBOB_HAVE_PICK_UP_ROPE(cargobob)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: DOES_CARGOBOB_HAVE_PICK_UP_ROPE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `does_cargobob_have_pick_up_rope_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn does_cargobob_have_pick_up_rope_raw(cargobob: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DOES_CARGOBOB_HAVE_PICK_UP_ROPE(cargobob)
}

/// ```
SET_VEHICLE_LI*
```

pub fn _0xc50ce861b55eab8b_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xC50CE861B55EAB8B");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xC50CE861B55EAB8B(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xC50CE861B55EAB8B
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xc50ce861b55eab8b_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xc50ce861b55eab8b_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xC50CE861B55EAB8B(vehicle, p1)
}

/// Locks the doors of a specified vehicle to a defined lock state, affecting how players and NPCs can interact with the vehicle.

```
NativeDB Introduced: v323
```

```c
enum eVehicleLockState {
    // No specific lock state, vehicle behaves according to the game's default settings.
    VEHICLELOCK_NONE = 0,
    // Vehicle is fully unlocked, allowing free entry by players and NPCs.
    VEHICLELOCK_UNLOCKED = 1,
    // Vehicle is locked, preventing entry by players and NPCs.
    VEHICLELOCK_LOCKED = 2,
    // Vehicle locks out only players, allowing NPCs to enter.
    VEHICLELOCK_LOCKOUT_PLAYER_ONLY = 3,
    // Vehicle is locked once a player enters, preventing others from entering.
    VEHICLELOCK_LOCKED_PLAYER_INSIDE = 4,
    // Vehicle starts in a locked state, but may be unlocked through game events.
    VEHICLELOCK_LOCKED_INITIALLY = 5,
    // Forces the vehicle's doors to shut and lock.
    VEHICLELOCK_FORCE_SHUT_DOORS = 6,
    // Vehicle is locked but can still be damaged.
    VEHICLELOCK_LOCKED_BUT_CAN_BE_DAMAGED = 7,
    // Vehicle is locked, but its trunk/boot remains unlocked.
    VEHICLELOCK_LOCKED_BUT_BOOT_UNLOCKED = 8,
    // Vehicle is locked and does not allow passengers, except for the driver.
    VEHICLELOCK_LOCKED_NO_PASSENGERS = 9,
    // Vehicle is completely locked, preventing entry entirely, even if previously inside.
    VEHICLELOCK_CANNOT_ENTER = 10 
};

```

pub fn set_vehicle_doors_locked_safe(vehicle: Vehicle, doorLockStatus: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DOORS_LOCKED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DOORS_LOCKED(vehicle, doorLockStatus)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DOORS_LOCKED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_doors_locked_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_doors_locked_raw(vehicle: i32, doorLockStatus: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DOORS_LOCKED(vehicle, doorLockStatus)
}

/// Disables turret movement when called in a loop. You can still fire and aim. You cannot shoot backwards though.

```
NativeDB Introduced: v1365
```

pub fn _set_disable_turret_movement_this_frame_safe(vehicle: Vehicle, turretIdx: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_DISABLE_TURRET_MOVEMENT_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_DISABLE_TURRET_MOVEMENT_THIS_FRAME(vehicle, turretIdx)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_DISABLE_TURRET_MOVEMENT_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_disable_turret_movement_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_disable_turret_movement_this_frame_raw(vehicle: i32, turretIdx: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_DISABLE_TURRET_MOVEMENT_THIS_FRAME(vehicle, turretIdx)
}

/// ```
colorIndex = 0 - 7
```

pub fn _set_vehicle_parachute_texture_variation_safe(vehicle: Vehicle, textureVariation: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_PARACHUTE_TEXTURE_VARIATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_PARACHUTE_TEXTURE_VARIATION(vehicle, textureVariation)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_PARACHUTE_TEXTURE_VARIATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_parachute_texture_variation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_parachute_texture_variation_raw(vehicle: i32, textureVariation: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_PARACHUTE_TEXTURE_VARIATION(vehicle, textureVariation)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_brake_lights_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_BRAKE_LIGHTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_BRAKE_LIGHTS(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_BRAKE_LIGHTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_brake_lights_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_brake_lights_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_BRAKE_LIGHTS(vehicle, toggle)
}

/// ## Parameters
* **model**:

pub fn is_this_model_a_quadbike_safe(model: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_THIS_MODEL_A_QUADBIKE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_THIS_MODEL_A_QUADBIKE(model)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_THIS_MODEL_A_QUADBIKE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_this_model_a_quadbike_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_this_model_a_quadbike_raw(model: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_THIS_MODEL_A_QUADBIKE(model)
}

/// Gets the position of the cargobob hook, in world coords.

pub fn _get_cargobob_hook_position_safe(cargobob: Vehicle) -> NativeResult<Vector3> {
    
    debug!("Calling native function: _GET_CARGOBOB_HOOK_POSITION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_CARGOBOB_HOOK_POSITION(cargobob)
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: _GET_CARGOBOB_HOOK_POSITION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_cargobob_hook_position_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_cargobob_hook_position_raw(cargobob: i32) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_CARGOBOB_HOOK_POSITION(cargobob)
}

/// ## Parameters
* **vehicle**: 
* **entity**: 
* **p2**:

pub fn stabilise_entity_attached_to_heli_safe(vehicle: Vehicle, entity: Entity, p2: f32) -> NativeResult<()> {
    
    debug!("Calling native function: STABILISE_ENTITY_ATTACHED_TO_HELI");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::STABILISE_ENTITY_ATTACHED_TO_HELI(vehicle, entity, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: STABILISE_ENTITY_ATTACHED_TO_HELI
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `stabilise_entity_attached_to_heli_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn stabilise_entity_attached_to_heli_raw(vehicle: i32, entity: i32, p2: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::STABILISE_ENTITY_ATTACHED_TO_HELI(vehicle, entity, p2)
}

/// ```
Check if Vehicle Secondary is avaliable for customize  
```

pub fn get_is_vehicle_secondary_colour_custom_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_IS_VEHICLE_SECONDARY_COLOUR_CUSTOM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_IS_VEHICLE_SECONDARY_COLOUR_CUSTOM(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_IS_VEHICLE_SECONDARY_COLOUR_CUSTOM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_is_vehicle_secondary_colour_custom_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_is_vehicle_secondary_colour_custom_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_IS_VEHICLE_SECONDARY_COLOUR_CUSTOM(vehicle)
}

/// Check if a entry point for a certain seat is clear, useful for checking if a vehicle seat is accesible.
If you park your vehicle near a wall and the ped cannot enter/exit this side, the return value toggles from true (not blocked) to false (blocked).

Keep in mind, with checkSide set to true, that only certain vehicles have entry points on both sides for the same seat, like motorcycles, most normal vehicles don't have this and if the native doesn't find a entry point with the given parameters it will always return false. So for most normal usecases leaving checkSide set to false would result in the expected behavior.

pub fn is_entry_point_for_seat_clear_safe(ped: Ped, vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_ENTRY_POINT_FOR_SEAT_CLEAR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_ENTRY_POINT_FOR_SEAT_CLEAR(ped, vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_ENTRY_POINT_FOR_SEAT_CLEAR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_entry_point_for_seat_clear_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_entry_point_for_seat_clear_raw(ped: i32, vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_ENTRY_POINT_FOR_SEAT_CLEAR(ped, vehicle)
}

/// ## Parameters
* **vehicle**: The vehicle to get the ped for.
* **seatIndex**: See eSeatPosition declared in [`IS_VEHICLE_SEAT_FREE`](#_0x22AC59A870E6A669).

pub fn get_last_ped_in_vehicle_seat_safe(vehicle: Vehicle, seatIndex: i64) -> NativeResult<Ped> {
    
    debug!("Calling native function: GET_LAST_PED_IN_VEHICLE_SEAT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_LAST_PED_IN_VEHICLE_SEAT(vehicle, seatIndex)
    };
    
    
    Ok(Ok(Ped(result)))
}

/// Raw native function: GET_LAST_PED_IN_VEHICLE_SEAT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_last_ped_in_vehicle_seat_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_last_ped_in_vehicle_seat_raw(vehicle: i32, seatIndex: i64) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_LAST_PED_IN_VEHICLE_SEAT(vehicle, seatIndex)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0xd565f438137f0e10_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0xD565F438137F0E10");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xD565F438137F0E10(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xD565F438137F0E10
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xd565f438137f0e10_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xd565f438137f0e10_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xD565F438137F0E10(p0, p1)
}

/// ## Parameters
* **vehicleAsset**:

pub fn remove_vehicle_asset_safe(vehicleAsset: i64) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_VEHICLE_ASSET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_VEHICLE_ASSET(vehicleAsset)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_VEHICLE_ASSET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_vehicle_asset_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_vehicle_asset_raw(vehicleAsset: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_VEHICLE_ASSET(vehicleAsset)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_disable_vehicle_engine_fires_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_DISABLE_VEHICLE_ENGINE_FIRES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DISABLE_VEHICLE_ENGINE_FIRES(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DISABLE_VEHICLE_ENGINE_FIRES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_disable_vehicle_engine_fires_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_disable_vehicle_engine_fires_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DISABLE_VEHICLE_ENGINE_FIRES(vehicle, toggle)
}

/// See [`REQUEST_VEHICLE_RECORDING`](#_0xAF514CABE74CBF15).

pub fn get_vehicle_recording_id_safe(recording: i64, script: String) -> NativeResult<i64> {
    let script_cstr = std::ffi::CString::new(script.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "script", e)))?;
    
    debug!("Calling native function: GET_VEHICLE_RECORDING_ID");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_RECORDING_ID(recording, crate::utils::rust_to_c_string(script))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_RECORDING_ID
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_recording_id_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_recording_id_raw(recording: i64, script: *const std::os::raw::c_char) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_RECORDING_ID(recording, script)
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)

pub fn _set_vehicle_door_can_break_safe(vehicle: Vehicle, doorIndex: i64, isBreakable: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_DOOR_CAN_BREAK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_DOOR_CAN_BREAK(vehicle, doorIndex, isBreakable)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_DOOR_CAN_BREAK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_door_can_break_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_door_can_break_raw(vehicle: i32, doorIndex: i64, isBreakable: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_DOOR_CAN_BREAK(vehicle, doorIndex, isBreakable)
}

/// ```
NativeDB Introduced: v1493
```

pub fn _set_vehicle_neon_lights_color_2_safe(vehicle: Vehicle, color: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_NEON_LIGHTS_COLOR_2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_NEON_LIGHTS_COLOR_2(vehicle, color)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_NEON_LIGHTS_COLOR_2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_neon_lights_color_2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_neon_lights_color_2_raw(vehicle: i32, color: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_NEON_LIGHTS_COLOR_2(vehicle, color)
}

/// ```
Returns false if every seat is occupied.  
```

pub fn are_any_vehicle_seats_free_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: ARE_ANY_VEHICLE_SEATS_FREE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ARE_ANY_VEHICLE_SEATS_FREE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: ARE_ANY_VEHICLE_SEATS_FREE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `are_any_vehicle_seats_free_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn are_any_vehicle_seats_free_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ARE_ANY_VEHICLE_SEATS_FREE(vehicle)
}

/// Lowers the roof on a convertible vehicle, utilizing any available animations for the action. This native is particularly useful for creating a realistic interaction with convertible vehicles by animating the process of lowering the roof.

You can check if the vehicle has an convertible roof using [`IS_VEHICLE_A_CONVERTIBLE`](#_0x52F357A30698BCCE).

```
NativeDB Introduced: v323
```

pub fn lower_convertible_roof_safe(vehicle: Vehicle, instantlyLower: bool) -> NativeResult<()> {
    
    debug!("Calling native function: LOWER_CONVERTIBLE_ROOF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::LOWER_CONVERTIBLE_ROOF(vehicle, instantlyLower)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: LOWER_CONVERTIBLE_ROOF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `lower_convertible_roof_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn lower_convertible_roof_raw(vehicle: i32, instantlyLower: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::LOWER_CONVERTIBLE_ROOF(vehicle, instantlyLower)
}

/// This native is used to simulate a high-speed impact for a vehicle when it collides with a breakable object (frag). It's particularly useful in scripted sequences where a vehicle is required to break through a barrier but might not actually be moving at a sufficient speed to do so realistically. Note that this setting is temporary and will reset after one frame, so it needs to be called every frame for a lasting effect.

pub fn set_vehicle_act_as_if_high_speed_for_frag_smashing_safe(vehicle: Vehicle, actHighSpeed: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_ACT_AS_IF_HIGH_SPEED_FOR_FRAG_SMASHING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_ACT_AS_IF_HIGH_SPEED_FOR_FRAG_SMASHING(vehicle, actHighSpeed)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_ACT_AS_IF_HIGH_SPEED_FOR_FRAG_SMASHING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_act_as_if_high_speed_for_frag_smashing_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_act_as_if_high_speed_for_frag_smashing_raw(vehicle: i32, actHighSpeed: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_ACT_AS_IF_HIGH_SPEED_FOR_FRAG_SMASHING(vehicle, actHighSpeed)
}

/// ## Parameters
* **vehicle**: 
* **paintType**: 
* **color**: 
* **pearlescentColor**:

pub fn get_vehicle_mod_color_1_safe(vehicle: Vehicle, paintType: *mut i64, color: *mut i64, pearlescentColor: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_VEHICLE_MOD_COLOR_1");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MOD_COLOR_1(vehicle, paintType, color, pearlescentColor)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_VEHICLE_MOD_COLOR_1
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_mod_color_1_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_mod_color_1_raw(vehicle: i32, paintType: *mut i64, color: *mut i64, pearlescentColor: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MOD_COLOR_1(vehicle, paintType, color, pearlescentColor)
}

/// ```
p1, p2, p3 are RGB values for color (255,0,0 for Red, ect)  
```

pub fn set_vehicle_custom_primary_colour_safe(vehicle: Vehicle, r: i64, g: i64, b: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_CUSTOM_PRIMARY_COLOUR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_CUSTOM_PRIMARY_COLOUR(vehicle, r, g, b)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_CUSTOM_PRIMARY_COLOUR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_custom_primary_colour_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_custom_primary_colour_raw(vehicle: i32, r: i64, g: i64, b: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_CUSTOM_PRIMARY_COLOUR(vehicle, r, g, b)
}

/// Sets the specified door index shut on the passed vehicle.

```c
enum eDoorId
{
	VEH_EXT_DOOR_DSIDE_F = 0,
	VEH_EXT_DOOR_DSIDE_R = 1,
	VEH_EXT_DOOR_PSIDE_F = 2,
	VEH_EXT_DOOR_PSIDE_R = 3,
	VEH_EXT_BONNET = 4,
	VEH_EXT_BOOT = 5,
	// 0x872E72B8 = 0xFFFFFFFF,
}
```

pub fn set_vehicle_door_shut_safe(vehicle: Vehicle, doorIndex: i64, closeInstantly: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DOOR_SHUT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DOOR_SHUT(vehicle, doorIndex, closeInstantly)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DOOR_SHUT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_door_shut_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_door_shut_raw(vehicle: i32, doorIndex: i64, closeInstantly: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DOOR_SHUT(vehicle, doorIndex, closeInstantly)
}

/// ## Parameters
* **vehicle**: 
* **team**: 
* **toggle**:

pub fn set_vehicle_doors_locked_for_team_safe(vehicle: Vehicle, team: i64, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DOORS_LOCKED_FOR_TEAM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DOORS_LOCKED_FOR_TEAM(vehicle, team, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DOORS_LOCKED_FOR_TEAM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_doors_locked_for_team_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_doors_locked_for_team_raw(vehicle: i32, team: i64, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DOORS_LOCKED_FOR_TEAM(vehicle, team, toggle)
}

/// ## Parameters
* **cargobob**: 
* **p1**:

pub fn set_cargobob_pickup_magnet_pull_rope_length_safe(cargobob: Vehicle, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CARGOBOB_PICKUP_MAGNET_PULL_ROPE_LENGTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CARGOBOB_PICKUP_MAGNET_PULL_ROPE_LENGTH(cargobob, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CARGOBOB_PICKUP_MAGNET_PULL_ROPE_LENGTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_cargobob_pickup_magnet_pull_rope_length_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_cargobob_pickup_magnet_pull_rope_length_raw(cargobob: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CARGOBOB_PICKUP_MAGNET_PULL_ROPE_LENGTH(cargobob, p1)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0xaf60e6a2936f982a_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0xAF60E6A2936F982A");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xAF60E6A2936F982A(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xAF60E6A2936F982A
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xaf60e6a2936f982a_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xaf60e6a2936f982a_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xAF60E6A2936F982A(p0, p1)
}

/// ## Parameters
* **model**:

pub fn is_this_model_a_train_safe(model: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_THIS_MODEL_A_TRAIN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_THIS_MODEL_A_TRAIN(model)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_THIS_MODEL_A_TRAIN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_this_model_a_train_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_this_model_a_train_raw(model: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_THIS_MODEL_A_TRAIN(model)
}

/// Used to set the tornado custom (convertible) rooftop livery.

Livery value that works for tornado custom is between 0 and 9 from what i can tell. Maybe 0-8 even.

Might work on other custom vehicles but im not sure what those might be, only confirmed it working with the tornado custom.

pub fn _set_vehicle_roof_livery_safe(vehicle: Vehicle, livery: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_ROOF_LIVERY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_ROOF_LIVERY(vehicle, livery)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_ROOF_LIVERY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_roof_livery_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_roof_livery_raw(vehicle: i32, livery: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_ROOF_LIVERY(vehicle, livery)
}

/// This native makes the vehicle stop immediately, as it happens when we enter a multiplayer garage.

pub fn bring_vehicle_to_halt_safe(vehicle: Vehicle, distance: f32, duration: i64, bControlVerticalVelocity: bool) -> NativeResult<()> {
    
    debug!("Calling native function: BRING_VEHICLE_TO_HALT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::BRING_VEHICLE_TO_HALT(vehicle, distance, duration, bControlVerticalVelocity)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: BRING_VEHICLE_TO_HALT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `bring_vehicle_to_halt_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn bring_vehicle_to_halt_raw(vehicle: i32, distance: f32, duration: i64, bControlVerticalVelocity: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::BRING_VEHICLE_TO_HALT(vehicle, distance, duration, bControlVerticalVelocity)
}

/// ```
NativeDB Introduced: v1493
```

pub fn _0x407dc5e97db1a4d3_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x407DC5E97DB1A4D3");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x407DC5E97DB1A4D3(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x407DC5E97DB1A4D3
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x407dc5e97db1a4d3_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x407dc5e97db1a4d3_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x407DC5E97DB1A4D3(p0, p1)
}

/// ## Parameters
* **model**:

pub fn is_this_model_a_bicycle_safe(model: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_THIS_MODEL_A_BICYCLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_THIS_MODEL_A_BICYCLE(model)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_THIS_MODEL_A_BICYCLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_this_model_a_bicycle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_this_model_a_bicycle_raw(model: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_THIS_MODEL_A_BICYCLE(model)
}

/// ```
NativeDB Introduced: v2060
```

pub fn _0xf8b49f5ba7f850e7_safe(vehicle: Vehicle, p1: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _0xF8B49F5BA7F850E7");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xF8B49F5BA7F850E7(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xF8B49F5BA7F850E7
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xf8b49f5ba7f850e7_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xf8b49f5ba7f850e7_raw(vehicle: i32, p1: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xF8B49F5BA7F850E7(vehicle, p1)
}

/// ```
flags requires further research, e.g., 0x4/0x8 are related to the AI driving task and 0x20 is internally set and interacts with dynamic entity components.
time, often zero and capped at 500, is related to SET_PLAYBACK_TO_USE_AI_TRY_TO_REVERT_BACK_LATER
```

pub fn start_playback_recorded_vehicle_with_flags_safe(vehicle: Vehicle, recording: i64, script: String, flags: i64, time: i64, drivingStyle: i64) -> NativeResult<()> {
    let script_cstr = std::ffi::CString::new(script.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "script", e)))?;
    
    debug!("Calling native function: START_PLAYBACK_RECORDED_VEHICLE_WITH_FLAGS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::START_PLAYBACK_RECORDED_VEHICLE_WITH_FLAGS(vehicle, recording, crate::utils::rust_to_c_string(script), flags, time, drivingStyle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: START_PLAYBACK_RECORDED_VEHICLE_WITH_FLAGS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `start_playback_recorded_vehicle_with_flags_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn start_playback_recorded_vehicle_with_flags_raw(vehicle: i32, recording: i64, script: *const std::os::raw::c_char, flags: i64, time: i64, drivingStyle: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::START_PLAYBACK_RECORDED_VEHICLE_WITH_FLAGS(vehicle, recording, script, flags, time, drivingStyle)
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)

pub fn get_vehicle_individual_door_lock_status_safe(vehicle: Vehicle, doorIndex: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_INDIVIDUAL_DOOR_LOCK_STATUS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_INDIVIDUAL_DOOR_LOCK_STATUS(vehicle, doorIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_INDIVIDUAL_DOOR_LOCK_STATUS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_individual_door_lock_status_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_individual_door_lock_status_raw(vehicle: i32, doorIndex: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_INDIVIDUAL_DOOR_LOCK_STATUS(vehicle, doorIndex)
}

/// ## Parameters
* **vehicle**: 
* **hash**:

pub fn _set_vehicle_handling_hash_for_ai_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_HANDLING_HASH_FOR_AI");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_HANDLING_HASH_FOR_AI(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_HANDLING_HASH_FOR_AI
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_handling_hash_for_ai_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_handling_hash_for_ai_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_HANDLING_HASH_FOR_AI(vehicle)
}

/// Sets the color of the neon lights on the specified vehicle.

RGB values and colour names taken from the decompiled scripts:

| Colour         |  R  |  G  |  B  |
|---------------|:---:|:---:|:---:|
| White         | 222 | 222 | 255 |
| Blue          | 2   | 21  | 255 |
| Electric Blue | 3   | 83  | 255 |
| Mint Green    | 0   | 255 | 140 |
| Lime Green    | 94  | 255 | 1   |
| Yellow        | 255 | 255 | 0   |
| Golden Shower | 255 | 150 | 0   |
| Orange        | 255 | 62  | 0   |
| Red           | 255 | 1   | 1   |
| Pony Pink     | 255 | 50  | 100 |
| Hot Pink      | 255 | 5   | 190 |
| Purple        | 35  | 1   | 255 |
| Blacklight    | 15  | 3   | 255 |

pub fn _set_vehicle_neon_lights_colour_safe(vehicle: Vehicle, r: i64, g: i64, b: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_NEON_LIGHTS_COLOUR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_NEON_LIGHTS_COLOUR(vehicle, r, g, b)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_NEON_LIGHTS_COLOUR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_neon_lights_colour_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_neon_lights_colour_raw(vehicle: i32, r: i64, g: i64, b: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_NEON_LIGHTS_COLOUR(vehicle, r, g, b)
}

/// ```
NativeDB Introduced: v3407
```

pub fn set_plane_control_sections_should_break_off_from_explosions_safe(plane: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLANE_CONTROL_SECTIONS_SHOULD_BREAK_OFF_FROM_EXPLOSIONS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLANE_CONTROL_SECTIONS_SHOULD_BREAK_OFF_FROM_EXPLOSIONS(plane, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLANE_CONTROL_SECTIONS_SHOULD_BREAK_OFF_FROM_EXPLOSIONS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_plane_control_sections_should_break_off_from_explosions_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_plane_control_sections_should_break_off_from_explosions_raw(plane: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLANE_CONTROL_SECTIONS_SHOULD_BREAK_OFF_FROM_EXPLOSIONS(plane, toggle)
}

/// SET_BOAT_REMAINS_ANCHORED_WHILE_PLAYER_IS_DRIVER native function

pub fn set_boat_remains_anchored_while_player_is_driver_safe(boat: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_BOAT_REMAINS_ANCHORED_WHILE_PLAYER_IS_DRIVER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_BOAT_REMAINS_ANCHORED_WHILE_PLAYER_IS_DRIVER(boat, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_BOAT_REMAINS_ANCHORED_WHILE_PLAYER_IS_DRIVER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_boat_remains_anchored_while_player_is_driver_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_boat_remains_anchored_while_player_is_driver_raw(boat: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_BOAT_REMAINS_ANCHORED_WHILE_PLAYER_IS_DRIVER(boat, toggle)
}

/// ```
GET_H*

NativeDB Introduced: v1604
```

pub fn _0xe8718faf591fd224_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _0xE8718FAF591FD224");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xE8718FAF591FD224(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0xE8718FAF591FD224
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xe8718faf591fd224_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xe8718faf591fd224_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xE8718FAF591FD224(vehicle)
}

/// ## Parameters
* **handler**: 
* **container**:

pub fn _attach_container_to_handler_frame_safe(handler: Vehicle, container: Entity) -> NativeResult<()> {
    
    debug!("Calling native function: _ATTACH_CONTAINER_TO_HANDLER_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_ATTACH_CONTAINER_TO_HANDLER_FRAME(handler, container)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _ATTACH_CONTAINER_TO_HANDLER_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_attach_container_to_handler_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _attach_container_to_handler_frame_raw(handler: i32, container: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_ATTACH_CONTAINER_TO_HANDLER_FRAME(handler, container)
}

/// ```
SCALE: Setting the speed to 30 would result in a speed of roughly 60mph, according to speedometer.  
Speed is in meters per second  
You can convert meters/s to mph here:  
http://www.calculateme.com/Speed/MetersperSecond/ToMilesperHour.htm  
```

pub fn set_vehicle_forward_speed_safe(vehicle: Vehicle, speed: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_FORWARD_SPEED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_FORWARD_SPEED(vehicle, speed)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_FORWARD_SPEED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_forward_speed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_forward_speed_raw(vehicle: i32, speed: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_FORWARD_SPEED(vehicle, speed)
}

/// ```
Sets vehicle wheel hydraulic states transition. Known states:
0 - reset
1 - raise wheel (uses value arg, works just like _SET_VEHICLE_HYDRAULIC_WHEEL_VALUE)
2 - jump using wheel
```

pub fn _set_hydraulic_wheel_state_transition_safe(vehicle: Vehicle, wheelId: i64, state: i64, value: f32, p4: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_HYDRAULIC_WHEEL_STATE_TRANSITION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_HYDRAULIC_WHEEL_STATE_TRANSITION(vehicle, wheelId, state, value, p4)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_HYDRAULIC_WHEEL_STATE_TRANSITION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_hydraulic_wheel_state_transition_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_hydraulic_wheel_state_transition_raw(vehicle: i32, wheelId: i64, state: i64, value: f32, p4: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_HYDRAULIC_WHEEL_STATE_TRANSITION(vehicle, wheelId, state, value, p4)
}

/// ## Parameters
* **plane**:

pub fn are_plane_propellers_intact_safe(plane: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: ARE_PLANE_PROPELLERS_INTACT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ARE_PLANE_PROPELLERS_INTACT(plane)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: ARE_PLANE_PROPELLERS_INTACT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `are_plane_propellers_intact_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn are_plane_propellers_intact_raw(plane: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ARE_PLANE_PROPELLERS_INTACT(plane)
}

/// This native is used to latch or unlatch the convertible roof of a vehicle. It allows for direct control over the roof's mechanism without actually opening or closing the roof. This can be useful for scenarios where you need to prepare a vehicle's roof to be opened or closed by another action or to ensure it remains fixed in its current state regardless of other interactions.

```
NativeDB Introduced: v323
```

pub fn set_convertible_roof_latch_state_safe(vehicle: Vehicle, bLatched: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CONVERTIBLE_ROOF_LATCH_STATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CONVERTIBLE_ROOF_LATCH_STATE(vehicle, bLatched)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CONVERTIBLE_ROOF_LATCH_STATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_convertible_roof_latch_state_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_convertible_roof_latch_state_raw(vehicle: i32, bLatched: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CONVERTIBLE_ROOF_LATCH_STATE(vehicle, bLatched)
}

/// ```
NativeDB Introduced: v1868
```

pub fn _set_tyre_health_safe(vehicle: Vehicle, wheelIndex: i64, health: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_TYRE_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_TYRE_HEALTH(vehicle, wheelIndex, health)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_TYRE_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_tyre_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_tyre_health_raw(vehicle: i32, wheelIndex: i64, health: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_TYRE_HEALTH(vehicle, wheelIndex, health)
}

/// ## Parameters
* **vehicle**: 
* **colorPrimary**: 
* **colorSecondary**:

pub fn get_vehicle_colours_safe(vehicle: Vehicle, colorPrimary: *mut i64, colorSecondary: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_VEHICLE_COLOURS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_COLOURS(vehicle, colorPrimary, colorSecondary)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_VEHICLE_COLOURS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_colours_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_colours_raw(vehicle: i32, colorPrimary: *mut i64, colorSecondary: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_COLOURS(vehicle, colorPrimary, colorSecondary)
}

/// ```
NativeDB Introduced: v3407
```

pub fn _is_vehicle_on_boost_pad_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_VEHICLE_ON_BOOST_PAD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_VEHICLE_ON_BOOST_PAD(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_VEHICLE_ON_BOOST_PAD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_vehicle_on_boost_pad_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_vehicle_on_boost_pad_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_VEHICLE_ON_BOOST_PAD(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn _get_has_rocket_boost_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _GET_HAS_ROCKET_BOOST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_HAS_ROCKET_BOOST(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_HAS_ROCKET_BOOST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_has_rocket_boost_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_has_rocket_boost_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_HAS_ROCKET_BOOST(vehicle)
}

/// ```
Time is number of milliseconds before reverting, zero for indefinitely.
```

pub fn set_playback_to_use_ai_try_to_revert_back_later_safe(vehicle: Vehicle, time: i64, drivingStyle: i64, p3: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYBACK_TO_USE_AI_TRY_TO_REVERT_BACK_LATER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYBACK_TO_USE_AI_TRY_TO_REVERT_BACK_LATER(vehicle, time, drivingStyle, p3)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYBACK_TO_USE_AI_TRY_TO_REVERT_BACK_LATER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_playback_to_use_ai_try_to_revert_back_later_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_playback_to_use_ai_try_to_revert_back_later_raw(vehicle: i32, time: i64, drivingStyle: i64, p3: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYBACK_TO_USE_AI_TRY_TO_REVERT_BACK_LATER(vehicle, time, drivingStyle, p3)
}

/// Determines if the submarine is operating below its designated crush depth.

```
NativeDB Introduced: v2189
```

pub fn get_submarine_is_under_design_depth_safe(submarine: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_SUBMARINE_IS_UNDER_DESIGN_DEPTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_SUBMARINE_IS_UNDER_DESIGN_DEPTH(submarine)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_SUBMARINE_IS_UNDER_DESIGN_DEPTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_submarine_is_under_design_depth_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_submarine_is_under_design_depth_raw(submarine: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_SUBMARINE_IS_UNDER_DESIGN_DEPTH(submarine)
}

/// ```
Note: only some vehicle have extras  
extra ids are from 1 - 9 depending on the vehicle  
-------------------------------------------------  
^ not sure if outdated or simply wrong. Max extra ID for b944 is 14  
-------------------------------------------------  
p2 is not a on/off toggle. mostly 0 means on and 1 means off.  
not sure if it really should be a BOOL.  
-------------------------------------------------  
Confirmed p2 does not work as a bool. Changed to int. [0=on, 1=off]  
```

pub fn set_vehicle_extra_safe(vehicle: Vehicle, extraId: i64, disable: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_EXTRA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_EXTRA(vehicle, extraId, disable)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_EXTRA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_extra_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_extra_raw(vehicle: i32, extraId: i64, disable: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_EXTRA(vehicle, extraId, disable)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn set_trailer_inverse_mass_scale_safe(vehicle: Vehicle, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_TRAILER_INVERSE_MASS_SCALE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_TRAILER_INVERSE_MASS_SCALE(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_TRAILER_INVERSE_MASS_SCALE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_trailer_inverse_mass_scale_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_trailer_inverse_mass_scale_raw(vehicle: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_TRAILER_INVERSE_MASS_SCALE(vehicle, p1)
}

/// Incorrectly named `SET_VEHICLE_EXCLUSIVE_DRIVER`; likely `SET_VEHICLE_ALLOW_*`.

Toggles a flag related to `SET_VEHICLE_EXCLUSIVE_DRIVER`, however, doesn't enable that feature (or trigger script events related to it).

See [`_SET_VEHICLE_EXCLUSIVE_DRIVER_2`](#_0xB5C51B5502E85E83).

```
NativeDB Removed Parameter 2: int index
```

pub fn set_vehicle_exclusive_driver_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_EXCLUSIVE_DRIVER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_EXCLUSIVE_DRIVER(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_EXCLUSIVE_DRIVER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_exclusive_driver_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_exclusive_driver_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_EXCLUSIVE_DRIVER(vehicle)
}

/// ```
Toggles specific flag on turret
```

```
NativeDB Introduced: v1290
```

pub fn _set_vehicle_turret_unk_safe(vehicle: Vehicle, index: i64, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_TURRET_UNK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_TURRET_UNK(vehicle, index, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_TURRET_UNK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_turret_unk_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_turret_unk_raw(vehicle: i32, index: i64, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_TURRET_UNK(vehicle, index, toggle)
}

/// ## Parameters
* **p0**:

pub fn has_preload_mods_finished_safe(p0: serde_json::Value) -> NativeResult<bool> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: HAS_PRELOAD_MODS_FINISHED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_PRELOAD_MODS_FINISHED(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_PRELOAD_MODS_FINISHED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_preload_mods_finished_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_preload_mods_finished_raw(p0: *mut std::os::raw::c_void) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_PRELOAD_MODS_FINISHED(p0)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn _0x737e398138550fff_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x737E398138550FFF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x737E398138550FFF(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x737E398138550FFF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x737e398138550fff_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x737e398138550fff_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x737E398138550FFF(vehicle, toggle)
}

/// Similar to [`_GET_AIRCRAFT_BOMB_COUNT`](#_0xEA12BD130D7569A1), this gets the amount of countermeasures that are present on this vehicle.

Use [`_SET_AIRCRAFT_COUNTERMEASURE_COUNT`](#_0x9BDA23BF666F0855) to set the current amount.

pub fn _get_vehicle_countermeasure_count_safe(aircraft: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: _GET_VEHICLE_COUNTERMEASURE_COUNT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_COUNTERMEASURE_COUNT(aircraft)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_VEHICLE_COUNTERMEASURE_COUNT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_countermeasure_count_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_countermeasure_count_raw(aircraft: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_COUNTERMEASURE_COUNT(aircraft)
}

/// _0xF25E02CB9C5818F8 native function

pub fn _0xf25e02cb9c5818f8_safe() -> NativeResult<()> {
    
    debug!("Calling native function: _0xF25E02CB9C5818F8");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xF25E02CB9C5818F8()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xF25E02CB9C5818F8
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xf25e02cb9c5818f8_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xf25e02cb9c5818f8_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xF25E02CB9C5818F8()
}

/// ```
if true, axles won't bend.  
```

pub fn set_vehicle_has_strong_axles_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_HAS_STRONG_AXLES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_HAS_STRONG_AXLES(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_HAS_STRONG_AXLES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_has_strong_axles_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_has_strong_axles_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_HAS_STRONG_AXLES(vehicle, toggle)
}

/// ```
NativeDB Introduced: v1180
```

pub fn _0x430a7631a84c9be7_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x430A7631A84C9BE7");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x430A7631A84C9BE7(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x430A7631A84C9BE7
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x430a7631a84c9be7_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x430a7631a84c9be7_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x430A7631A84C9BE7(p0)
}

/// SET_ALL_VEHICLE_GENERATORS_ACTIVE native function

pub fn set_all_vehicle_generators_active_safe() -> NativeResult<()> {
    
    debug!("Calling native function: SET_ALL_VEHICLE_GENERATORS_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_ALL_VEHICLE_GENERATORS_ACTIVE()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_ALL_VEHICLE_GENERATORS_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_all_vehicle_generators_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_all_vehicle_generators_active_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_ALL_VEHICLE_GENERATORS_ACTIVE()
}

/// ```
The only example I can find of this function in the scripts, is this:  
struct _s = VEHICLE::GET_VEHICLE_DEFORMATION_AT_POS(rPtr((A_0) + 4), 1.21f, 6.15f, 0.3f);  
-----------------------------------------------------------------------------------------------------------------------------------------  
PC scripts:  
v_5/*{3}*/ = VEHICLE::GET_VEHICLE_DEFORMATION_AT_POS(a_0._f1, 1.21, 6.15, 0.3);  
```

pub fn get_vehicle_deformation_at_pos_safe(vehicle: Vehicle, offsetX: f32, offsetY: f32, offsetZ: f32) -> NativeResult<Vector3> {
    
    debug!("Calling native function: GET_VEHICLE_DEFORMATION_AT_POS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_DEFORMATION_AT_POS(vehicle, offsetX, offsetY, offsetZ)
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: GET_VEHICLE_DEFORMATION_AT_POS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_deformation_at_pos_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_deformation_at_pos_raw(vehicle: i32, offsetX: f32, offsetY: f32, offsetZ: f32) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_DEFORMATION_AT_POS(vehicle, offsetX, offsetY, offsetZ)
}

/// DETONATE_VEHICLE_PHONE_EXPLOSIVE_DEVICE native function

pub fn detonate_vehicle_phone_explosive_device_safe() -> NativeResult<()> {
    
    debug!("Calling native function: DETONATE_VEHICLE_PHONE_EXPLOSIVE_DEVICE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DETONATE_VEHICLE_PHONE_EXPLOSIVE_DEVICE()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DETONATE_VEHICLE_PHONE_EXPLOSIVE_DEVICE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `detonate_vehicle_phone_explosive_device_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn detonate_vehicle_phone_explosive_device_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DETONATE_VEHICLE_PHONE_EXPLOSIVE_DEVICE()
}

/// ```
NativeDB Introduced: v1604
```

pub fn _0x9849de24fcf23ccc_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x9849DE24FCF23CCC");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x9849DE24FCF23CCC(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x9849DE24FCF23CCC
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x9849de24fcf23ccc_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x9849de24fcf23ccc_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x9849DE24FCF23CCC(vehicle, toggle)
}

/// ```
returns a string which is the codename of the vehicle's currently selected secondary color  
```

pub fn get_vehicle_mod_color_2_name_safe(vehicle: Vehicle) -> NativeResult<String> {
    
    debug!("Calling native function: GET_VEHICLE_MOD_COLOR_2_NAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MOD_COLOR_2_NAME(vehicle)
    };
    
    
    Ok(crate::utils::c_string_to_rust_string(result))
}

/// Raw native function: GET_VEHICLE_MOD_COLOR_2_NAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_mod_color_2_name_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_mod_color_2_name_raw(vehicle: i32) -> *const std::os::raw::c_char {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MOD_COLOR_2_NAME(vehicle)
}

/// Returns whether the outrigger legs are deployed for the vehicle.
The Chernobog is one of the few vehicles with outrigger legs.

```
NativeDB Introduced: v1290
```

pub fn _are_outrigger_legs_deployed_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _ARE_OUTRIGGER_LEGS_DEPLOYED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_ARE_OUTRIGGER_LEGS_DEPLOYED(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _ARE_OUTRIGGER_LEGS_DEPLOYED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_are_outrigger_legs_deployed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _are_outrigger_legs_deployed_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_ARE_OUTRIGGER_LEGS_DEPLOYED(vehicle)
}

/// ```
NativeDB Introduced: 3095
```

Determines if the nitrous is currently activated in the specified vehicle.

pub fn is_nitrous_active_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_NITROUS_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_NITROUS_ACTIVE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_NITROUS_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_nitrous_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_nitrous_active_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_NITROUS_ACTIVE(vehicle)
}

/// ```
Sets the neon lights of the specified vehicle on/off.  
Indices:  
0 = Left  
1 = Right  
2 = Front  
3 = Back  
```

pub fn _set_vehicle_neon_light_enabled_safe(vehicle: Vehicle, index: i64, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_NEON_LIGHT_ENABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_NEON_LIGHT_ENABLED(vehicle, index, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_NEON_LIGHT_ENABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_neon_light_enabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_neon_light_enabled_raw(vehicle: i32, index: i64, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_NEON_LIGHT_ENABLED(vehicle, index, toggle)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x065d03a9d6b2c6b5_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x065D03A9D6B2C6B5");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x065D03A9D6B2C6B5(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x065D03A9D6B2C6B5
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x065d03a9d6b2c6b5_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x065d03a9d6b2c6b5_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x065D03A9D6B2C6B5(p0, p1)
}

/// ```
Returns true only when the magnet is active, will return false if the hook is active  
```

pub fn does_cargobob_have_pickup_magnet_safe(cargobob: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: DOES_CARGOBOB_HAVE_PICKUP_MAGNET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DOES_CARGOBOB_HAVE_PICKUP_MAGNET(cargobob)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: DOES_CARGOBOB_HAVE_PICKUP_MAGNET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `does_cargobob_have_pickup_magnet_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn does_cargobob_have_pickup_magnet_raw(cargobob: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DOES_CARGOBOB_HAVE_PICKUP_MAGNET(cargobob)
}

/// ```
Scripts verify that towTruck is the first parameter, not the second.  
```

pub fn is_vehicle_attached_to_tow_truck_safe(towTruck: Vehicle, vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_ATTACHED_TO_TOW_TRUCK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_ATTACHED_TO_TOW_TRUCK(towTruck, vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_ATTACHED_TO_TOW_TRUCK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_attached_to_tow_truck_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_attached_to_tow_truck_raw(towTruck: i32, vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_ATTACHED_TO_TOW_TRUCK(towTruck, vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn set_players_last_vehicle_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLAYERS_LAST_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLAYERS_LAST_VEHICLE(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLAYERS_LAST_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_players_last_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_players_last_vehicle_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLAYERS_LAST_VEHICLE(vehicle)
}

/// ```
The inner function has a switch on the second parameter. It's the stuck timer index.  
Here's some pseudo code I wrote for the inner function:  
void __fastcall NATIVE_RESET_VEHICLE_STUCK_TIMER_INNER(CUnknown* unknownClassInVehicle, int timerIndex)  
{  
	switch (timerIndex)  
	{  
	case 0:  
unknownClassInVehicle->FirstStuckTimer = (WORD)0u;  
	case 1:  
unknownClassInVehicle->SecondStuckTimer = (WORD)0u;  
	case 2:  
unknownClassInVehicle->ThirdStuckTimer = (WORD)0u;  
	case 3:  
unknownClassInVehicle->FourthStuckTimer = (WORD)0u;  
	case 4:  
unknownClassInVehicle->FirstStuckTimer = (WORD)0u;  
unknownClassInVehicle->SecondStuckTimer = (WORD)0u;  
unknownClassInVehicle->ThirdStuckTimer = (WORD)0u;  
unknownClassInVehicle->FourthStuckTimer = (WORD)0u;  
break;  
	};  
}  
```

pub fn reset_vehicle_stuck_timer_safe(vehicle: Vehicle, nullAttributes: i64) -> NativeResult<()> {
    
    debug!("Calling native function: RESET_VEHICLE_STUCK_TIMER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::RESET_VEHICLE_STUCK_TIMER(vehicle, nullAttributes)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: RESET_VEHICLE_STUCK_TIMER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `reset_vehicle_stuck_timer_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn reset_vehicle_stuck_timer_raw(vehicle: i32, nullAttributes: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::RESET_VEHICLE_STUCK_TIMER(vehicle, nullAttributes)
}

/// ## Parameters
* **p0**:

pub fn _0xf051d9bfb6ba39c0_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xF051D9BFB6BA39C0");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xF051D9BFB6BA39C0(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xF051D9BFB6BA39C0
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xf051d9bfb6ba39c0_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xf051d9bfb6ba39c0_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xF051D9BFB6BA39C0(p0)
}

/// ## Parameters
* **vehicle**: 
* **trailer**: 
* **radius**:

pub fn attach_vehicle_to_trailer_safe(vehicle: Vehicle, trailer: Vehicle, radius: f32) -> NativeResult<()> {
    
    debug!("Calling native function: ATTACH_VEHICLE_TO_TRAILER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ATTACH_VEHICLE_TO_TRAILER(vehicle, trailer, radius)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ATTACH_VEHICLE_TO_TRAILER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `attach_vehicle_to_trailer_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn attach_vehicle_to_trailer_raw(vehicle: i32, trailer: i32, radius: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ATTACH_VEHICLE_TO_TRAILER(vehicle, trailer, radius)
}

/// Sets whether the trailer can attach to vehicles

pub fn set_trailer_attachment_enabled_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_TRAILER_ATTACHMENT_ENABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_TRAILER_ATTACHMENT_ENABLED(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_TRAILER_ATTACHMENT_ENABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_trailer_attachment_enabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_trailer_attachment_enabled_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_TRAILER_ATTACHMENT_ENABLED(vehicle)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**:

pub fn _0x66e3aaface2d1eb8_safe(p0: serde_json::Value, p1: serde_json::Value, p2: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x66E3AAFACE2D1EB8");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x66E3AAFACE2D1EB8(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1), crate::utils::any_to_c_void_ptr(p2))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x66E3AAFACE2D1EB8
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x66e3aaface2d1eb8_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x66e3aaface2d1eb8_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void, p2: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x66E3AAFACE2D1EB8(p0, p1, p2)
}

/// This native sets the turbulence multiplier. It only works for planes.
0.0 = no turbulence at all.
1.0 = heavy turbulence.

Works by just calling it once, does not need to be called every tick.

pub fn set_plane_turbulence_multiplier_safe(vehicle: Vehicle, multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLANE_TURBULENCE_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLANE_TURBULENCE_MULTIPLIER(vehicle, multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLANE_TURBULENCE_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_plane_turbulence_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_plane_turbulence_multiplier_raw(vehicle: i32, multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLANE_TURBULENCE_MULTIPLIER(vehicle, multiplier)
}

/// Changes the key used to transform a vehicle into submarine mode. When set to true, the transformation key switches from the default raise/lower convertible roof key (usually 'H') to the special vehicle transformation key (usually 'X').

```
NativeDB Introduced: v1365
```

pub fn set_transform_to_submarine_uses_alternate_input_safe(vehicle: Vehicle, useAlternateInput: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_TRANSFORM_TO_SUBMARINE_USES_ALTERNATE_INPUT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_TRANSFORM_TO_SUBMARINE_USES_ALTERNATE_INPUT(vehicle, useAlternateInput)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_TRANSFORM_TO_SUBMARINE_USES_ALTERNATE_INPUT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_transform_to_submarine_uses_alternate_input_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_transform_to_submarine_uses_alternate_input_raw(vehicle: i32, useAlternateInput: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_TRANSFORM_TO_SUBMARINE_USES_ALTERNATE_INPUT(vehicle, useAlternateInput)
}

/// ## Parameters
* **vehicle**:

pub fn have_vehicle_mods_streamed_in_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: HAVE_VEHICLE_MODS_STREAMED_IN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAVE_VEHICLE_MODS_STREAMED_IN(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAVE_VEHICLE_MODS_STREAMED_IN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `have_vehicle_mods_streamed_in_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn have_vehicle_mods_streamed_in_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAVE_VEHICLE_MODS_STREAMED_IN(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn detach_container_from_handler_frame_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: DETACH_CONTAINER_FROM_HANDLER_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DETACH_CONTAINER_FROM_HANDLER_FRAME(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DETACH_CONTAINER_FROM_HANDLER_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `detach_container_from_handler_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn detach_container_from_handler_frame_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DETACH_CONTAINER_FROM_HANDLER_FRAME(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **modType**: Refer to eVehicleModType in [`SET_VEHICLE_MOD`](#_0x6AF0636DDEDCB6DD).

pub fn remove_vehicle_mod_safe(vehicle: Vehicle, modType: i64) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_VEHICLE_MOD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_VEHICLE_MOD(vehicle, modType)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_VEHICLE_MOD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_vehicle_mod_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_vehicle_mod_raw(vehicle: i32, modType: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_VEHICLE_MOD(vehicle, modType)
}

/// ```
Outputs 2 Vector3's.
Scripts check if out2.x - out1.x > something.x
Could be suspension related, as in max suspension height and min suspension height, considering the natives location.
```

pub fn _get_vehicle_suspension_bounds_safe(vehicle: Vehicle, out1: *mut *mut Vector3_raw, out2: *mut *mut Vector3_raw) -> NativeResult<()> {
    
    debug!("Calling native function: _GET_VEHICLE_SUSPENSION_BOUNDS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_SUSPENSION_BOUNDS(vehicle, out1, out2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _GET_VEHICLE_SUSPENSION_BOUNDS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_suspension_bounds_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_suspension_bounds_raw(vehicle: i32, out1: *mut *mut Vector3_raw, out2: *mut *mut Vector3_raw) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_SUSPENSION_BOUNDS(vehicle, out1, out2)
}

/// See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).

pub fn roll_down_window_safe(vehicle: Vehicle, windowIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: ROLL_DOWN_WINDOW");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ROLL_DOWN_WINDOW(vehicle, windowIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ROLL_DOWN_WINDOW
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `roll_down_window_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn roll_down_window_raw(vehicle: i32, windowIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ROLL_DOWN_WINDOW(vehicle, windowIndex)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn _set_vehicle_ramp_sideways_launch_motion_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_RAMP_SIDEWAYS_LAUNCH_MOTION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_RAMP_SIDEWAYS_LAUNCH_MOTION(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_RAMP_SIDEWAYS_LAUNCH_MOTION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_ramp_sideways_launch_motion_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_ramp_sideways_launch_motion_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_RAMP_SIDEWAYS_LAUNCH_MOTION(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_can_leak_oil_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_CAN_LEAK_OIL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_CAN_LEAK_OIL(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_CAN_LEAK_OIL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_can_leak_oil_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_can_leak_oil_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_CAN_LEAK_OIL(vehicle, toggle)
}

/// ```
formerly known as _GET_VEHICLE_PAINT_FADE
The result is a value from 0-1, where 0 is fresh paint.
```

pub fn get_vehicle_enveff_scale_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_ENVEFF_SCALE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_ENVEFF_SCALE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_ENVEFF_SCALE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_enveff_scale_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_enveff_scale_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_ENVEFF_SCALE(vehicle)
}

/// ```
NativeDB Introduced: v2189
```

pub fn _0x8664170ef165c4a6_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x8664170EF165C4A6");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x8664170EF165C4A6(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x8664170EF165C4A6
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x8664170ef165c4a6_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x8664170ef165c4a6_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x8664170EF165C4A6(p0, p1)
}

/// ## Parameters
* **vehicle**: 
* **p1**: 
* **p2**:

pub fn disable_plane_aileron_safe(vehicle: Vehicle, p1: bool, p2: bool) -> NativeResult<()> {
    
    debug!("Calling native function: DISABLE_PLANE_AILERON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DISABLE_PLANE_AILERON(vehicle, p1, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DISABLE_PLANE_AILERON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `disable_plane_aileron_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn disable_plane_aileron_raw(vehicle: i32, p1: bool, p2: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DISABLE_PLANE_AILERON(vehicle, p1, p2)
}

/// ```
NativeDB Introduced: v2372
```

pub fn _0x2310a8f9421ebf43_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x2310A8F9421EBF43");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x2310A8F9421EBF43(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x2310A8F9421EBF43
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x2310a8f9421ebf43_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x2310a8f9421ebf43_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x2310A8F9421EBF43(p0)
}

/// ```
NativeDB Introduced: v3407
```

pub fn _set_vehicle_explosive_damage_scale_safe(vehicle: Vehicle, scale: f32) -> NativeResult<serde_json::Value> {
    
    debug!("Calling native function: _SET_VEHICLE_EXPLOSIVE_DAMAGE_SCALE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_EXPLOSIVE_DAMAGE_SCALE(vehicle, scale)
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _SET_VEHICLE_EXPLOSIVE_DAMAGE_SCALE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_explosive_damage_scale_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_explosive_damage_scale_raw(vehicle: i32, scale: f32) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_EXPLOSIVE_DAMAGE_SCALE(vehicle, scale)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_can_break_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_CAN_BREAK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_CAN_BREAK(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_CAN_BREAK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_can_break_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_can_break_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_CAN_BREAK(vehicle, toggle)
}

/// ```
Stops the cargobob from being able to attach any vehicle
```

```
NativeDB Introduced: v1180
```

pub fn _set_cargobob_hook_can_attach_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_CARGOBOB_HOOK_CAN_ATTACH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_CARGOBOB_HOOK_CAN_ATTACH(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_CARGOBOB_HOOK_CAN_ATTACH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_cargobob_hook_can_attach_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_cargobob_hook_can_attach_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_CARGOBOB_HOOK_CAN_ATTACH(vehicle, toggle)
}

/// ## Parameters
* **vehicle**:

pub fn add_vehicle_upsidedown_check_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: ADD_VEHICLE_UPSIDEDOWN_CHECK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ADD_VEHICLE_UPSIDEDOWN_CHECK(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ADD_VEHICLE_UPSIDEDOWN_CHECK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `add_vehicle_upsidedown_check_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn add_vehicle_upsidedown_check_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ADD_VEHICLE_UPSIDEDOWN_CHECK(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn is_vehicle_siren_audio_on_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_SIREN_AUDIO_ON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_SIREN_AUDIO_ON(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_SIREN_AUDIO_ON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_siren_audio_on_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_siren_audio_on_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_SIREN_AUDIO_ON(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn force_submarine_surface_mode_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: FORCE_SUBMARINE_SURFACE_MODE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::FORCE_SUBMARINE_SURFACE_MODE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: FORCE_SUBMARINE_SURFACE_MODE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `force_submarine_surface_mode_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn force_submarine_surface_mode_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::FORCE_SUBMARINE_SURFACE_MODE(vehicle, toggle)
}

/// ## Parameters
* **multiplier**:

pub fn set_random_vehicle_density_multiplier_this_frame_safe(multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_RANDOM_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_RANDOM_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME(multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_RANDOM_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_random_vehicle_density_multiplier_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_random_vehicle_density_multiplier_this_frame_raw(multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_RANDOM_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME(multiplier)
}

/// Vehicle must be a plane.
Native name is between SET_VEHICLE_BRAKE_LIGHTS and SET_VEHICLE_BULLDOZER_ARM_POSITION alphabetically.

pub fn _0xc361aa040d6637a8_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xC361AA040D6637A8");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xC361AA040D6637A8(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xC361AA040D6637A8
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xc361aa040d6637a8_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xc361aa040d6637a8_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xC361AA040D6637A8(vehicle, p1)
}

/// ## Parameters
* **vehicle**: 
* **enabled**:

pub fn set_vehicle_wheels_can_break_safe(vehicle: Vehicle, enabled: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_WHEELS_CAN_BREAK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_WHEELS_CAN_BREAK(vehicle, enabled)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_WHEELS_CAN_BREAK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_wheels_can_break_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_wheels_can_break_raw(vehicle: i32, enabled: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_WHEELS_CAN_BREAK(vehicle, enabled)
}

/// ## Parameters
* **vehicle**: The vehicle handle
* **modType**: The mod type (see [`SET_VEHICLE_MOD`](#_0x6AF0636DDEDCB6DD))
* **modIndex**: The mod index

pub fn is_vehicle_mod_gen9_exclusive_safe(vehicle: Vehicle, modType: i64, modIndex: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_MOD_GEN9_EXCLUSIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_MOD_GEN9_EXCLUSIVE(vehicle, modType, modIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_MOD_GEN9_EXCLUSIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_mod_gen9_exclusive_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_mod_gen9_exclusive_raw(vehicle: i32, modType: i64, modIndex: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_MOD_GEN9_EXCLUSIVE(vehicle, modType, modIndex)
}

/// ```
NativeDB Introduced: v1290
```

pub fn _0x8821196d91fa2de5_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x8821196D91FA2DE5");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x8821196D91FA2DE5(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x8821196D91FA2DE5
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x8821196d91fa2de5_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x8821196d91fa2de5_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x8821196D91FA2DE5(vehicle, toggle)
}

/// ```
SET_VEHICLE_*
```

pub fn _0x76d26a22750e849e_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _0x76D26A22750E849E");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x76D26A22750E849E(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x76D26A22750E849E
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x76d26a22750e849e_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x76d26a22750e849e_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x76D26A22750E849E(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **entity**: 
* **p2**: 
* **x**: 
* **y**: 
* **z**:

pub fn attach_entity_to_cargobob_safe(vehicle: Vehicle, entity: Entity, p2: i64, x: f32, y: f32, z: f32) -> NativeResult<()> {
    
    debug!("Calling native function: ATTACH_ENTITY_TO_CARGOBOB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ATTACH_ENTITY_TO_CARGOBOB(vehicle, entity, p2, x, y, z)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ATTACH_ENTITY_TO_CARGOBOB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `attach_entity_to_cargobob_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn attach_entity_to_cargobob_raw(vehicle: i32, entity: i32, p2: i64, x: f32, y: f32, z: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ATTACH_ENTITY_TO_CARGOBOB(vehicle, entity, p2, x, y, z)
}

/// ```
Not present in the retail version! It's just a nullsub.  
p0 always true (except in one case)  
p1 a random vehicle hash loaded in memory  
successIndicator: 0 if success, -1 if failed
```

pub fn get_random_vehicle_model_in_memory_safe(p0: bool, modelHash: *mut u32, successIndicator: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_RANDOM_VEHICLE_MODEL_IN_MEMORY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_RANDOM_VEHICLE_MODEL_IN_MEMORY(p0, modelHash, successIndicator)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_RANDOM_VEHICLE_MODEL_IN_MEMORY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_random_vehicle_model_in_memory_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_random_vehicle_model_in_memory_raw(p0: bool, modelHash: *mut u32, successIndicator: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_RANDOM_VEHICLE_MODEL_IN_MEMORY(p0, modelHash, successIndicator)
}

/// Paint index goes from 0 to 12.

You can find the list of colors and ids here: [_GET_VEHICLE_HEADLIGHTS_COLOUR](#_0x3DFF319A831E0CDB)

pub fn _set_vehicle_xenon_lights_color_safe(vehicle: Vehicle, color: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_XENON_LIGHTS_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_XENON_LIGHTS_COLOR(vehicle, color)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_XENON_LIGHTS_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_xenon_lights_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_xenon_lights_color_raw(vehicle: i32, color: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_XENON_LIGHTS_COLOR(vehicle, color)
}

/// ## Parameters
* **active**:

pub fn set_all_low_priority_vehicle_generators_active_safe(active: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_ALL_LOW_PRIORITY_VEHICLE_GENERATORS_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_ALL_LOW_PRIORITY_VEHICLE_GENERATORS_ACTIVE(active)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_ALL_LOW_PRIORITY_VEHICLE_GENERATORS_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_all_low_priority_vehicle_generators_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_all_low_priority_vehicle_generators_active_raw(active: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_ALL_LOW_PRIORITY_VEHICLE_GENERATORS_ACTIVE(active)
}

/// ```
Request the vehicle recording defined by the lowercase format string "%s%03d.yvr". For example, REQUEST_VEHICLE_RECORDING(1, "FBIs1UBER") corresponds to fbis1uber001.yvr.
For all vehicle recording/playback natives, "script" is a common prefix that usually corresponds to the script/mission the recording is used in, "recording" is its int suffix, and "id" (e.g., in native GET_TOTAL_DURATION_OF_VEHICLE_RECORDING_ID) corresponds to a unique identifier within the recording streaming module.
Note that only 24 recordings (hardcoded in multiple places) can ever active at a given time before clobbering begins.
```

pub fn request_vehicle_recording_safe(recording: i64, script: String) -> NativeResult<()> {
    let script_cstr = std::ffi::CString::new(script.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "script", e)))?;
    
    debug!("Calling native function: REQUEST_VEHICLE_RECORDING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REQUEST_VEHICLE_RECORDING(recording, crate::utils::rust_to_c_string(script))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REQUEST_VEHICLE_RECORDING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `request_vehicle_recording_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn request_vehicle_recording_raw(recording: i64, script: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REQUEST_VEHICLE_RECORDING(recording, script)
}

/// This method is utilized solely for debugging purposes and is functional only in debug builds of the game. Please note that its functionality may not be available in the retail version.

pub fn set_vehicle_name_debug_safe(vehicle: Vehicle, name: String) -> NativeResult<()> {
    let name_cstr = std::ffi::CString::new(name.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "name", e)))?;
    
    debug!("Calling native function: SET_VEHICLE_NAME_DEBUG");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_NAME_DEBUG(vehicle, crate::utils::rust_to_c_string(name))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_NAME_DEBUG
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_name_debug_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_name_debug_raw(vehicle: i32, name: *const std::os::raw::c_char) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_NAME_DEBUG(vehicle, name)
}

/// ```
NativeDB Introduced: v1604
```

pub fn _set_disable_vehicle_unk_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_DISABLE_VEHICLE_UNK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_DISABLE_VEHICLE_UNK(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_DISABLE_VEHICLE_UNK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_disable_vehicle_unk_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_disable_vehicle_unk_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_DISABLE_VEHICLE_UNK(toggle)
}

/// ```
Possibly: Returns whether the searchlight (found on police vehicles) is toggled on.  
```

pub fn is_vehicle_searchlight_on_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_SEARCHLIGHT_ON");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_SEARCHLIGHT_ON(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_SEARCHLIGHT_ON
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_searchlight_on_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_searchlight_on_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_SEARCHLIGHT_ON(vehicle)
}

/// ```
They use the same color indexs as SET_VEHICLE_COLOURS.  
```

pub fn set_vehicle_extra_colours_safe(vehicle: Vehicle, pearlescentColor: i64, wheelColor: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_EXTRA_COLOURS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_EXTRA_COLOURS(vehicle, pearlescentColor, wheelColor)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_EXTRA_COLOURS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_extra_colours_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_extra_colours_raw(vehicle: i32, pearlescentColor: i64, wheelColor: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_EXTRA_COLOURS(vehicle, pearlescentColor, wheelColor)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**: 
* **p5**: 
* **p6**:

pub fn add_vehicle_combat_angled_avoidance_area_safe(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32) -> NativeResult<serde_json::Value> {
    
    debug!("Calling native function: ADD_VEHICLE_COMBAT_ANGLED_AVOIDANCE_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ADD_VEHICLE_COMBAT_ANGLED_AVOIDANCE_AREA(p0, p1, p2, p3, p4, p5, p6)
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: ADD_VEHICLE_COMBAT_ANGLED_AVOIDANCE_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `add_vehicle_combat_angled_avoidance_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn add_vehicle_combat_angled_avoidance_area_raw(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: f32) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ADD_VEHICLE_COMBAT_ANGLED_AVOIDANCE_AREA(p0, p1, p2, p3, p4, p5, p6)
}

/// Train models must be [requested](#_0x963D27A58DF860AC) before use. See trains.xml (located in `Grand Theft Auto V\update\update.rpf\common\data\levels\gta5\trains.xml`) for freight and metro variations.

Model names to request can be found by searching `model_name` in the file.

The `Lua` usage example provided down below has been provided in such way so users can test each and every train variation.

pub fn create_mission_train_safe(variation: i64, x: f32, y: f32, z: f32, direction: bool) -> NativeResult<Vehicle> {
    
    debug!("Calling native function: CREATE_MISSION_TRAIN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CREATE_MISSION_TRAIN(variation, x, y, z, direction)
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: CREATE_MISSION_TRAIN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `create_mission_train_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn create_mission_train_raw(variation: i64, x: f32, y: f32, z: f32, direction: bool) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CREATE_MISSION_TRAIN(variation, x, y, z, direction)
}

/// ```
Creates a script vehicle generator at the given coordinates. Most parameters after the model hash are unknown.  
Parameters:  
a/w/s - Generator position  
heading - Generator heading  
p4 - Unknown (always 5.0)  
p5 - Unknown (always 3.0)  
modelHash - Vehicle model hash  
p7/8/9/10 - Unknown (always -1)  
p11 - Unknown (usually TRUE, only one instance of FALSE)  
p12/13 - Unknown (always FALSE)  
p14 - Unknown (usally FALSE, only two instances of TRUE)  
p15 - Unknown (always TRUE)  
p16 - Unknown (always -1)  
Vector3 coords = GET_ENTITY_COORDS(PLAYER_PED_ID(), 0);	CREATE_SCRIPT_VEHICLE_GENERATOR(coords.x, coords.y, coords.z, 1.0f, 5.0f, 3.0f, GET_HASH_KEY("adder"), -1. -1, -1, -1, -1, true, false, false, false, true, -1);  
```

pub fn create_script_vehicle_generator_safe(x: f32, y: f32, z: f32, heading: f32, p4: f32, p5: f32, modelHash: u32, p7: i64, p8: i64, p9: i64, p10: i64, p11: bool, p12: bool, p13: bool, p14: bool, p15: bool, p16: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: CREATE_SCRIPT_VEHICLE_GENERATOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CREATE_SCRIPT_VEHICLE_GENERATOR(x, y, z, heading, p4, p5, modelHash, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CREATE_SCRIPT_VEHICLE_GENERATOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `create_script_vehicle_generator_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn create_script_vehicle_generator_raw(x: f32, y: f32, z: f32, heading: f32, p4: f32, p5: f32, modelHash: u32, p7: i64, p8: i64, p9: i64, p10: i64, p11: bool, p12: bool, p13: bool, p14: bool, p15: bool, p16: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CREATE_SCRIPT_VEHICLE_GENERATOR(x, y, z, heading, p4, p5, modelHash, p7, p8, p9, p10, p11, p12, p13, p14, p15, p16)
}

/// ## Parameters
* **vehicle**:

pub fn _is_vehicle_rocket_boost_active_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_VEHICLE_ROCKET_BOOST_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_VEHICLE_ROCKET_BOOST_ACTIVE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_VEHICLE_ROCKET_BOOST_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_vehicle_rocket_boost_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_vehicle_rocket_boost_active_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_VEHICLE_ROCKET_BOOST_ACTIVE(vehicle)
}

/// Affects the playback speed of the submarine car conversion animations. Does not affect hardcoded animations such as the wheels being retracted. In decompiled scripts the only value used for transformRate is 2.5.

pub fn set_transform_rate_for_animation_safe(vehicle: Vehicle, transformRate: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_TRANSFORM_RATE_FOR_ANIMATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_TRANSFORM_RATE_FOR_ANIMATION(vehicle, transformRate)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_TRANSFORM_RATE_FOR_ANIMATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_transform_rate_for_animation_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_transform_rate_for_animation_raw(vehicle: i32, transformRate: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_TRANSFORM_RATE_FOR_ANIMATION(vehicle, transformRate)
}

/// ```
Allows you to toggle bulletproof tires.  
```

pub fn set_vehicle_tyres_can_burst_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_TYRES_CAN_BURST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_TYRES_CAN_BURST(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_TYRES_CAN_BURST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_tyres_can_burst_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_tyres_can_burst_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_TYRES_CAN_BURST(vehicle, toggle)
}

/// This native allows opening or closing the wings of the Deluxo/Oppressor. For the Deluxo, wing deployment depends on sufficient altitude.

pub fn set_hover_mode_wing_ratio_safe(vehicle: Vehicle, ratio: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_HOVER_MODE_WING_RATIO");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_HOVER_MODE_WING_RATIO(vehicle, ratio)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_HOVER_MODE_WING_RATIO
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_hover_mode_wing_ratio_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_hover_mode_wing_ratio_raw(vehicle: i32, ratio: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_HOVER_MODE_WING_RATIO(vehicle, ratio)
}

/// ```
HookOffset defines where the hook is attached. leave at 0 for default attachment.
```

pub fn attach_vehicle_to_tow_truck_safe(towTruck: Vehicle, vehicle: Vehicle, rear: bool, hookOffsetX: f32, hookOffsetY: f32, hookOffsetZ: f32) -> NativeResult<()> {
    
    debug!("Calling native function: ATTACH_VEHICLE_TO_TOW_TRUCK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ATTACH_VEHICLE_TO_TOW_TRUCK(towTruck, vehicle, rear, hookOffsetX, hookOffsetY, hookOffsetZ)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ATTACH_VEHICLE_TO_TOW_TRUCK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `attach_vehicle_to_tow_truck_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn attach_vehicle_to_tow_truck_raw(towTruck: i32, vehicle: i32, rear: bool, hookOffsetX: f32, hookOffsetY: f32, hookOffsetZ: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ATTACH_VEHICLE_TO_TOW_TRUCK(towTruck, vehicle, rear, hookOffsetX, hookOffsetY, hookOffsetZ)
}

/// ```
Often called after START_PLAYBACK_RECORDED_VEHICLE and SKIP_TIME_IN_PLAYBACK_RECORDED_VEHICLE; similar in use to FORCE_ENTITY_AI_AND_ANIMATION_UPDATE.
```

pub fn force_playback_recorded_vehicle_update_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: FORCE_PLAYBACK_RECORDED_VEHICLE_UPDATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::FORCE_PLAYBACK_RECORDED_VEHICLE_UPDATE(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: FORCE_PLAYBACK_RECORDED_VEHICLE_UPDATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `force_playback_recorded_vehicle_update_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn force_playback_recorded_vehicle_update_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::FORCE_PLAYBACK_RECORDED_VEHICLE_UPDATE(vehicle, p1)
}

/// ```
NativeDB Introduced: v1868
```

pub fn _0x4ad280eb48b2d8e6_safe(vehicle: Vehicle, togle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x4AD280EB48B2D8E6");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x4AD280EB48B2D8E6(vehicle, togle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x4AD280EB48B2D8E6
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x4ad280eb48b2d8e6_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x4ad280eb48b2d8e6_raw(vehicle: i32, togle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x4AD280EB48B2D8E6(vehicle, togle)
}

/// ## Parameters
* **vehicle**: The vehicle to attach to the trailer
* **trailer**: The trailer to attach the vehicle to
* **offsetX**: The x offset of the vehicle
* **offsetY**: The y offset of the vehicle
* **offsetZ**: The z offset of the vehicle
* **coordsX**: The x coords of where you want the vehicle placed (must be an offset from the trailer itself)
* **coordsY**: The y coords of where you want the vehicle placed (must be an offset from the trailer itself)
* **coordsZ**: The z coords of where you want the vehicle placed (must be an offset from the trailer itself)
* **rotationX**: The x rotation of where you want the vehicle placed
* **rotationY**: The y rotation of where you want the vehicle placed
* **rotationZ**: The z rotation of where you want the vehicle placed
* **disableColls**: Should actually be a boolean, this will disable the collision between the vehicle you're attaching and the trailer

This is the proper way of attaching vehicles to the car carrier, it's what Rockstar uses. [Video Demo](https://www.youtube.com/watch?v=2lVEIzf7bgo)

pub fn attach_vehicle_on_to_trailer_safe(vehicle: Vehicle, trailer: Vehicle, offsetX: f32, offsetY: f32, offsetZ: f32, coordsX: f32, coordsY: f32, coordsZ: f32, rotationX: f32, rotationY: f32, rotationZ: f32, disableColls: f32) -> NativeResult<()> {
    
    debug!("Calling native function: ATTACH_VEHICLE_ON_TO_TRAILER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ATTACH_VEHICLE_ON_TO_TRAILER(vehicle, trailer, offsetX, offsetY, offsetZ, coordsX, coordsY, coordsZ, rotationX, rotationY, rotationZ, disableColls)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ATTACH_VEHICLE_ON_TO_TRAILER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `attach_vehicle_on_to_trailer_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn attach_vehicle_on_to_trailer_raw(vehicle: i32, trailer: i32, offsetX: f32, offsetY: f32, offsetZ: f32, coordsX: f32, coordsY: f32, coordsZ: f32, rotationX: f32, rotationY: f32, rotationZ: f32, disableColls: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ATTACH_VEHICLE_ON_TO_TRAILER(vehicle, trailer, offsetX, offsetY, offsetZ, coordsX, coordsY, coordsZ, rotationX, rotationY, rotationZ, disableColls)
}

/// INSTANTLY_FILL_VEHICLE_POPULATION native function

pub fn instantly_fill_vehicle_population_safe() -> NativeResult<()> {
    
    debug!("Calling native function: INSTANTLY_FILL_VEHICLE_POPULATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::INSTANTLY_FILL_VEHICLE_POPULATION()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: INSTANTLY_FILL_VEHICLE_POPULATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `instantly_fill_vehicle_population_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn instantly_fill_vehicle_population_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::INSTANTLY_FILL_VEHICLE_POPULATION()
}

/// ## Parameters
* **vehicle**: 
* **x**: 
* **y**: 
* **z**:

pub fn _eject_jb700_roof_safe(vehicle: Vehicle, x: f32, y: f32, z: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _EJECT_JB700_ROOF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_EJECT_JB700_ROOF(vehicle, x, y, z)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _EJECT_JB700_ROOF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_eject_jb700_roof_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _eject_jb700_roof_raw(vehicle: i32, x: f32, y: f32, z: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_EJECT_JB700_ROOF(vehicle, x, y, z)
}

/// ## Parameters
* **vehicle**: 
* **time**:

pub fn _set_vehicle_rocket_boost_refill_time_safe(vehicle: Vehicle, time: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_ROCKET_BOOST_REFILL_TIME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_ROCKET_BOOST_REFILL_TIME(vehicle, time)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_ROCKET_BOOST_REFILL_TIME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_rocket_boost_refill_time_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_rocket_boost_refill_time_raw(vehicle: i32, time: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_ROCKET_BOOST_REFILL_TIME(vehicle, time)
}

/// ```
Max 1000.
At -100 both helicopter rotors will stall.
```

pub fn get_heli_tail_boom_health_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_HELI_TAIL_BOOM_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_HELI_TAIL_BOOM_HEALTH(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_HELI_TAIL_BOOM_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_heli_tail_boom_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_heli_tail_boom_health_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_HELI_TAIL_BOOM_HEALTH(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn is_any_ped_rappelling_from_heli_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_ANY_PED_RAPPELLING_FROM_HELI");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_ANY_PED_RAPPELLING_FROM_HELI(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_ANY_PED_RAPPELLING_FROM_HELI
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_any_ped_rappelling_from_heli_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_any_ped_rappelling_from_heli_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_ANY_PED_RAPPELLING_FROM_HELI(vehicle)
}

/// Removes the cargen area of interest and resumes normal cargen spawning.

You can set the area of interest with [`SET_VEHICLE_GENERATOR_AREA_OF_INTEREST`](#_0x9A75585FB2E54FAD)

pub fn clear_vehicle_generator_area_of_interest_safe() -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_VEHICLE_GENERATOR_AREA_OF_INTEREST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_VEHICLE_GENERATOR_AREA_OF_INTEREST()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_VEHICLE_GENERATOR_AREA_OF_INTEREST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_vehicle_generator_area_of_interest_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_vehicle_generator_area_of_interest_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_VEHICLE_GENERATOR_AREA_OF_INTEREST()
}

/// ## Parameters
* **vehicle**:

pub fn get_is_vehicle_primary_colour_custom_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_IS_VEHICLE_PRIMARY_COLOUR_CUSTOM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_IS_VEHICLE_PRIMARY_COLOUR_CUSTOM(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_IS_VEHICLE_PRIMARY_COLOUR_CUSTOM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_is_vehicle_primary_colour_custom_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_is_vehicle_primary_colour_custom_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_IS_VEHICLE_PRIMARY_COLOUR_CUSTOM(vehicle)
}

/// ```
Sets the tire smoke's color of this vehicle.  
vehicle: The vehicle that is the target of this method.  
r: The red level in the RGB color code.  
g: The green level in the RGB color code.  
b: The blue level in the RGB color code.  
Note:  
setting r,g,b to 0 will give the car independance day tyre smoke  
```

pub fn set_vehicle_tyre_smoke_color_safe(vehicle: Vehicle, r: i64, g: i64, b: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_TYRE_SMOKE_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_TYRE_SMOKE_COLOR(vehicle, r, g, b)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_TYRE_SMOKE_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_tyre_smoke_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_tyre_smoke_color_raw(vehicle: i32, r: i64, g: i64, b: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_TYRE_SMOKE_COLOR(vehicle, r, g, b)
}

/// ## Parameters
* **outVec**: 
* **p1**: 
* **outVec1**: 
* **p3**: 
* **p4**: 
* **p5**: 
* **p6**: 
* **p7**: 
* **p8**:

pub fn _0xa4822f1cf23f4810_safe(outVec: *mut *mut Vector3_raw, p1: *mut *mut Vector3_raw, outVec1: *mut *mut Vector3_raw, p3: serde_json::Value, p4: serde_json::Value, p5: serde_json::Value, p6: serde_json::Value, p7: serde_json::Value, p8: serde_json::Value) -> NativeResult<bool> {
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
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p6_any_str = serde_json::to_string(&p6)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p6", e)))?;
    let p6_any_str_cstr = std::ffi::CString::new(p6_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p6", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p7_any_str = serde_json::to_string(&p7)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p7", e)))?;
    let p7_any_str_cstr = std::ffi::CString::new(p7_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p7", e)))?;
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p8_any_str = serde_json::to_string(&p8)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p8", e)))?;
    let p8_any_str_cstr = std::ffi::CString::new(p8_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p8", e)))?;
    
    debug!("Calling native function: _0xA4822F1CF23F4810");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xA4822F1CF23F4810(outVec, p1, outVec1, crate::utils::any_to_c_void_ptr(p3), crate::utils::any_to_c_void_ptr(p4), crate::utils::any_to_c_void_ptr(p5), crate::utils::any_to_c_void_ptr(p6), crate::utils::any_to_c_void_ptr(p7), crate::utils::any_to_c_void_ptr(p8))
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _0xA4822F1CF23F4810
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xa4822f1cf23f4810_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xa4822f1cf23f4810_raw(outVec: *mut *mut Vector3_raw, p1: *mut *mut Vector3_raw, outVec1: *mut *mut Vector3_raw, p3: *mut std::os::raw::c_void, p4: *mut std::os::raw::c_void, p5: *mut std::os::raw::c_void, p6: *mut std::os::raw::c_void, p7: *mut std::os::raw::c_void, p8: *mut std::os::raw::c_void) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xA4822F1CF23F4810(outVec, p1, outVec1, p3, p4, p5, p6, p7, p8)
}

/// ```
NativeDB Introduced: v1180
```

pub fn _0x97841634ef7df1d6_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x97841634EF7DF1D6");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x97841634EF7DF1D6(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x97841634EF7DF1D6
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x97841634ef7df1d6_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x97841634ef7df1d6_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x97841634EF7DF1D6(vehicle, toggle)
}

/// ```
Toggles to render distant vehicles. They may not be vehicles but images to look like vehicles.  
```

pub fn set_distant_cars_enabled_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_DISTANT_CARS_ENABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DISTANT_CARS_ENABLED(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DISTANT_CARS_ENABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_distant_cars_enabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_distant_cars_enabled_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DISTANT_CARS_ENABLED(toggle)
}

/// ```c
enum WindowTints  
{  
	WINDOWTINT_NONE = 0,
	WINDOWTINT_PURE_BLACK = 1,
	WINDOWTINT_DARKSMOKE = 2,
	WINDOWTINT_LIGHTSMOKE = 3,
	WINDOWTINT_STOCK = 4,
	WINDOWTINT_LIMO = 5,
	WINDOWTINT_GREEN = 6
};  
```

pub fn set_vehicle_window_tint_safe(vehicle: Vehicle, tint: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_WINDOW_TINT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_WINDOW_TINT(vehicle, tint)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_WINDOW_TINT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_window_tint_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_window_tint_raw(vehicle: i32, tint: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_WINDOW_TINT(vehicle, tint)
}

/// Smashes a vehicles window. See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).

pub fn smash_vehicle_window_safe(vehicle: Vehicle, windowIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SMASH_VEHICLE_WINDOW");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SMASH_VEHICLE_WINDOW(vehicle, windowIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SMASH_VEHICLE_WINDOW
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `smash_vehicle_window_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn smash_vehicle_window_raw(vehicle: i32, windowIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SMASH_VEHICLE_WINDOW(vehicle, windowIndex)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0xf3b0e0aed097a3f5_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<serde_json::Value> {
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
    
    debug!("Calling native function: _0xF3B0E0AED097A3F5");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xF3B0E0AED097A3F5(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(unsafe { crate::utils::c_void_ptr_to_any(result) }))
}

/// Raw native function: _0xF3B0E0AED097A3F5
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xf3b0e0aed097a3f5_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xf3b0e0aed097a3f5_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xF3B0E0AED097A3F5(p0, p1)
}

/// ## Parameters
* **vehicle**: The vehicle to get the mod of.
* **modType**: Refer to eVehicleModType in [`SET_VEHICLE_MOD`](#_0x6AF0636DDEDCB6DD).

pub fn get_vehicle_mod_safe(vehicle: Vehicle, modType: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_MOD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MOD(vehicle, modType)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MOD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_mod_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_mod_raw(vehicle: i32, modType: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MOD(vehicle, modType)
}

/// ```
Works only on vehicles that support hydraulic.
```

pub fn _set_hydraulic_wheel_value_safe(vehicle: Vehicle, wheelId: i64, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_HYDRAULIC_WHEEL_VALUE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_HYDRAULIC_WHEEL_VALUE(vehicle, wheelId, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_HYDRAULIC_WHEEL_VALUE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_hydraulic_wheel_value_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_hydraulic_wheel_value_raw(vehicle: i32, wheelId: i64, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_HYDRAULIC_WHEEL_VALUE(vehicle, wheelId, value)
}

/// ## Parameters
* **vehicle**:

pub fn skip_to_end_and_stop_playback_recorded_vehicle_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SKIP_TO_END_AND_STOP_PLAYBACK_RECORDED_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SKIP_TO_END_AND_STOP_PLAYBACK_RECORDED_VEHICLE(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SKIP_TO_END_AND_STOP_PLAYBACK_RECORDED_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `skip_to_end_and_stop_playback_recorded_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn skip_to_end_and_stop_playback_recorded_vehicle_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SKIP_TO_END_AND_STOP_PLAYBACK_RECORDED_VEHICLE(vehicle)
}

/// ## Parameters
* **toggle**:

pub fn set_disable_random_trains_this_frame_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_DISABLE_RANDOM_TRAINS_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DISABLE_RANDOM_TRAINS_THIS_FRAME(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DISABLE_RANDOM_TRAINS_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_disable_random_trains_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_disable_random_trains_this_frame_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DISABLE_RANDOM_TRAINS_THIS_FRAME(toggle)
}

/// ```
NativeDB Introduced: 3095
```

Retrieves the remaining duration of nitrous boost available for the specified vehicle.

pub fn _get_remaining_nitrous_duration_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: _GET_REMAINING_NITROUS_DURATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_REMAINING_NITROUS_DURATION(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_REMAINING_NITROUS_DURATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_remaining_nitrous_duration_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_remaining_nitrous_duration_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_REMAINING_NITROUS_DURATION(vehicle)
}

/// ```
SET_*
```

pub fn _0x428ad3e26c8d9eb0_safe(vehicle: Vehicle, x: f32, y: f32, z: f32, p4: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0x428AD3E26C8D9EB0");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x428AD3E26C8D9EB0(vehicle, x, y, z, p4)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x428AD3E26C8D9EB0
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x428ad3e26c8d9eb0_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x428ad3e26c8d9eb0_raw(vehicle: i32, x: f32, y: f32, z: f32, p4: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x428AD3E26C8D9EB0(vehicle, x, y, z, p4)
}

/// ```
NativeDB Introduced: v1290
```

pub fn is_vehicle_in_submarine_mode_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_IN_SUBMARINE_MODE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_IN_SUBMARINE_MODE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_IN_SUBMARINE_MODE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_in_submarine_mode_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_in_submarine_mode_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_IN_SUBMARINE_MODE(vehicle)
}

/// ```
1000 is max health  
Begins leaking gas at around 650 health  
```

pub fn get_vehicle_petrol_tank_health_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_PETROL_TANK_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_PETROL_TANK_HEALTH(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_PETROL_TANK_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_petrol_tank_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_petrol_tank_health_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_PETROL_TANK_HEALTH(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn _0x5e569ec46ec21cae_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x5E569EC46EC21CAE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x5E569EC46EC21CAE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x5E569EC46EC21CAE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x5e569ec46ec21cae_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x5e569ec46ec21cae_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x5E569EC46EC21CAE(vehicle, toggle)
}

/// ```
NativeDB Added Parameter 2: float maxEngineHealth
NativeDB Added Parameter 3: float maxPetrolTankHealth
NativeDB Added Parameter 4: float maxBodyHealth
NativeDB Added Parameter 5: float maxMainRotorHealth
NativeDB Added Parameter 6: float maxTailRotorHealth
NativeDB Added Parameter 7: float maxUnkHealth
```

pub fn get_vehicle_health_percentage_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_HEALTH_PERCENTAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_HEALTH_PERCENTAGE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_HEALTH_PERCENTAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_health_percentage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_health_percentage_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_HEALTH_PERCENTAGE(vehicle)
}

/// Sounds the horn for the specified vehicle. Note that if a player is in the vehicle, it will only sound briefly.

pub fn start_vehicle_horn_safe(vehicle: Vehicle, duration: i64, mode: u32, forever: bool) -> NativeResult<()> {
    
    debug!("Calling native function: START_VEHICLE_HORN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::START_VEHICLE_HORN(vehicle, duration, mode, forever)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: START_VEHICLE_HORN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `start_vehicle_horn_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn start_vehicle_horn_raw(vehicle: i32, duration: i64, mode: u32, forever: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::START_VEHICLE_HORN(vehicle, duration, mode, forever)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_handbrake_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_HANDBRAKE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_HANDBRAKE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_HANDBRAKE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_handbrake_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_handbrake_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_HANDBRAKE(vehicle, toggle)
}

/// ```
SET_VEHICLE_W* (next character is either H or I)
```

pub fn _0x2c4a1590abf43e8b_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x2C4A1590ABF43E8B");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x2C4A1590ABF43E8B(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x2C4A1590ABF43E8B
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x2c4a1590abf43e8b_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x2c4a1590abf43e8b_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x2C4A1590ABF43E8B(vehicle, p1)
}

/// ```
Maximum amount of vehicles with vehicle stuck check appears to be 16.  
```

pub fn does_vehicle_have_stuck_vehicle_check_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: DOES_VEHICLE_HAVE_STUCK_VEHICLE_CHECK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DOES_VEHICLE_HAVE_STUCK_VEHICLE_CHECK(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: DOES_VEHICLE_HAVE_STUCK_VEHICLE_CHECK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `does_vehicle_have_stuck_vehicle_check_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn does_vehicle_have_stuck_vehicle_check_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DOES_VEHICLE_HAVE_STUCK_VEHICLE_CHECK(vehicle)
}

/// Adjusts the scale of damage applied to a specified section of a plane.
In the decompiled scripts the `damageScale` is always set to `0f` (maybe to disable damages on the specified section)

```c
enum ePlaneDamageSection {
    WING_L = 0,
    WING_R = 1,
    TAIL = 2,
    ENGINE_L = 3,
    ENGINE_R = 4,
    ELEVATOR_L = 5,
    ELEVATOR_R = 6,
    AILERON_L = 7,
    AILERON_R = 8,
    RUDDER = 9,
    RUDDER_2 = 10,
    AIRBRAKE_L = 11,
    AIRBRAKE_R = 12
}
```

```
NativeDB Introduced: v1290
```

pub fn set_plane_section_damage_scale_safe(vehicle: Vehicle, damageSection: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_PLANE_SECTION_DAMAGE_SCALE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_PLANE_SECTION_DAMAGE_SCALE(vehicle, damageSection)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_PLANE_SECTION_DAMAGE_SCALE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_plane_section_damage_scale_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_plane_section_damage_scale_raw(vehicle: i32, damageSection: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_PLANE_SECTION_DAMAGE_SCALE(vehicle, damageSection)
}

/// ## Parameters
* **vehicle**: 
* **doorIndex**: See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)
* **doorLockStatus**: See eCarLock declared in [`SET_VEHICLE_DOORS_LOCKED`](#_0xB664292EAECF7FA6)

pub fn set_vehicle_individual_doors_locked_safe(vehicle: Vehicle, doorIndex: i64, doorLockStatus: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_INDIVIDUAL_DOORS_LOCKED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_INDIVIDUAL_DOORS_LOCKED(vehicle, doorIndex, doorLockStatus)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_INDIVIDUAL_DOORS_LOCKED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_individual_doors_locked_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_individual_doors_locked_raw(vehicle: i32, doorIndex: i64, doorLockStatus: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_INDIVIDUAL_DOORS_LOCKED(vehicle, doorIndex, doorLockStatus)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _get_boat_boom_position_ratio_3_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _GET_BOAT_BOOM_POSITION_RATIO_3");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_BOAT_BOOM_POSITION_RATIO_3(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _GET_BOAT_BOOM_POSITION_RATIO_3
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_boat_boom_position_ratio_3_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_boat_boom_position_ratio_3_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_BOAT_BOOM_POSITION_RATIO_3(vehicle, p1)
}

/// Please refer to [`GET_VEHICLE_NUMBER_PLATE_TEXT_INDEX`](#_0xF11BC2DD9A3E7195) for plate indicies.

pub fn set_vehicle_number_plate_text_index_safe(vehicle: Vehicle, plateIndex: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_NUMBER_PLATE_TEXT_INDEX");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_NUMBER_PLATE_TEXT_INDEX(vehicle, plateIndex)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_NUMBER_PLATE_TEXT_INDEX
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_number_plate_text_index_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_number_plate_text_index_raw(vehicle: i32, plateIndex: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_NUMBER_PLATE_TEXT_INDEX(vehicle, plateIndex)
}

/// ## Parameters
* **vehicle**: 
* **p1**: 
* **p2**: 
* **p3**:

pub fn is_heli_part_broken_safe(vehicle: Vehicle, p1: bool, p2: bool, p3: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_HELI_PART_BROKEN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_HELI_PART_BROKEN(vehicle, p1, p2, p3)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_HELI_PART_BROKEN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_heli_part_broken_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_heli_part_broken_raw(vehicle: i32, p1: bool, p2: bool, p3: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_HELI_PART_BROKEN(vehicle, p1, p2, p3)
}

/// Prevents a specified entity from being detached from a Cargobob, even in the event of collisions.

pub fn set_cargobob_exclude_from_pickup_entity_safe(cargobob: Vehicle, entity: Entity) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CARGOBOB_EXCLUDE_FROM_PICKUP_ENTITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CARGOBOB_EXCLUDE_FROM_PICKUP_ENTITY(cargobob, entity)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CARGOBOB_EXCLUDE_FROM_PICKUP_ENTITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_cargobob_exclude_from_pickup_entity_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_cargobob_exclude_from_pickup_entity_raw(cargobob: i32, entity: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CARGOBOB_EXCLUDE_FROM_PICKUP_ENTITY(cargobob, entity)
}

/// ```
NativeDB Introduced: v1180
```

pub fn _does_vehicle_have_landing_gear_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _DOES_VEHICLE_HAVE_LANDING_GEAR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_DOES_VEHICLE_HAVE_LANDING_GEAR(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _DOES_VEHICLE_HAVE_LANDING_GEAR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_does_vehicle_have_landing_gear_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _does_vehicle_have_landing_gear_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_DOES_VEHICLE_HAVE_LANDING_GEAR(vehicle)
}

/// ```
Checks via CVehicleModelInfo  
```

pub fn does_extra_exist_safe(vehicle: Vehicle, extraId: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: DOES_EXTRA_EXIST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DOES_EXTRA_EXIST(vehicle, extraId)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: DOES_EXTRA_EXIST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `does_extra_exist_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn does_extra_exist_raw(vehicle: i32, extraId: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DOES_EXTRA_EXIST(vehicle, extraId)
}

/// ```
Something to do with "high speed bump severity"?  
if (!sub_87a46("SET_CAR_HIGH_SPEED_BUMP_SEVERITY_MULTIPLIER")) {  
    VEHICLE::_84FD40F56075E816(0.0);  
    sub_8795b("SET_CAR_HIGH_SPEED_BUMP_SEVERITY_MULTIPLIER", 1);  
}  
```

pub fn _set_car_high_speed_bump_severity_multiplier_safe(multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_CAR_HIGH_SPEED_BUMP_SEVERITY_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_CAR_HIGH_SPEED_BUMP_SEVERITY_MULTIPLIER(multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_CAR_HIGH_SPEED_BUMP_SEVERITY_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_car_high_speed_bump_severity_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_car_high_speed_bump_severity_multiplier_raw(multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_CAR_HIGH_SPEED_BUMP_SEVERITY_MULTIPLIER(multiplier)
}

/// ```
First two parameters swapped. Scripts verify that towTruck is the first parameter, not the second.  
```

pub fn detach_vehicle_from_tow_truck_safe(towTruck: Vehicle, vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: DETACH_VEHICLE_FROM_TOW_TRUCK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DETACH_VEHICLE_FROM_TOW_TRUCK(towTruck, vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DETACH_VEHICLE_FROM_TOW_TRUCK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `detach_vehicle_from_tow_truck_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn detach_vehicle_from_tow_truck_raw(towTruck: i32, vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DETACH_VEHICLE_FROM_TOW_TRUCK(towTruck, vehicle)
}

/// p3 is some flag related to 'trailers' (invokes CVehicle::GetTrailer).

See [`REQUEST_VEHICLE_RECORDING`](#_0xAF514CABE74CBF15).

pub fn start_playback_recorded_vehicle_safe(vehicle: Vehicle, recording: i64, script: String, p3: bool) -> NativeResult<()> {
    let script_cstr = std::ffi::CString::new(script.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "script", e)))?;
    
    debug!("Calling native function: START_PLAYBACK_RECORDED_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::START_PLAYBACK_RECORDED_VEHICLE(vehicle, recording, crate::utils::rust_to_c_string(script), p3)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: START_PLAYBACK_RECORDED_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `start_playback_recorded_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn start_playback_recorded_vehicle_raw(vehicle: i32, recording: i64, script: *const std::os::raw::c_char, p3: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::START_PLAYBACK_RECORDED_VEHICLE(vehicle, recording, script, p3)
}

/// ```
1000 is max health
Begins leaking gas at around 650 health
-999.90002441406 appears to be minimum health, although nothing special occurs <- false statement
-------------------------
Minimum: -4000
Maximum: 1000
-4000: Engine is destroyed
0 and below: Engine catches fire and health rapidly declines
300: Engine is smoking and losing functionality
1000: Engine is perfect
```

pub fn set_vehicle_engine_health_safe(vehicle: Vehicle, health: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_ENGINE_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_ENGINE_HEALTH(vehicle, health)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_ENGINE_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_engine_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_engine_health_raw(vehicle: i32, health: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_ENGINE_HEALTH(vehicle, health)
}

/// ## Parameters
* **id**:

pub fn get_total_duration_of_vehicle_recording_id_safe(id: i64) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_TOTAL_DURATION_OF_VEHICLE_RECORDING_ID");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_TOTAL_DURATION_OF_VEHICLE_RECORDING_ID(id)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_TOTAL_DURATION_OF_VEHICLE_RECORDING_ID
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_total_duration_of_vehicle_recording_id_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_total_duration_of_vehicle_recording_id_raw(id: i64) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_TOTAL_DURATION_OF_VEHICLE_RECORDING_ID(id)
}

/// ```
Checks if model is a boat, then checks for FLAG_IS_JETSKI.
```

pub fn _is_this_model_a_jetski_safe(model: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_THIS_MODEL_A_JETSKI");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_THIS_MODEL_A_JETSKI(model)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_THIS_MODEL_A_JETSKI
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_this_model_a_jetski_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_this_model_a_jetski_raw(model: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_THIS_MODEL_A_JETSKI(model)
}

/// ```
A vehicle recording playback flag only used in jewelry_heist
```

pub fn _0x063ae2b2cc273588_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x063AE2B2CC273588");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x063AE2B2CC273588(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x063AE2B2CC273588
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x063ae2b2cc273588_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x063ae2b2cc273588_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x063AE2B2CC273588(vehicle, p1)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_active_during_playback_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_ACTIVE_DURING_PLAYBACK");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_ACTIVE_DURING_PLAYBACK(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_ACTIVE_DURING_PLAYBACK
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_active_during_playback_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_active_during_playback_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_ACTIVE_DURING_PLAYBACK(vehicle, toggle)
}

/// ```
NativeDB Introduced: v1180
```

pub fn _0x8235f1bead557629_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x8235F1BEAD557629");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x8235F1BEAD557629(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x8235F1BEAD557629
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x8235f1bead557629_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x8235f1bead557629_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x8235F1BEAD557629(vehicle, toggle)
}

/// ```
NativeDB Introduced: v1604
```

pub fn _get_is_vehicle_shunt_boost_active_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _GET_IS_VEHICLE_SHUNT_BOOST_ACTIVE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_IS_VEHICLE_SHUNT_BOOST_ACTIVE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_IS_VEHICLE_SHUNT_BOOST_ACTIVE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_is_vehicle_shunt_boost_active_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_is_vehicle_shunt_boost_active_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_IS_VEHICLE_SHUNT_BOOST_ACTIVE(vehicle)
}

/// ```
Distance traveled in the vehicles current recording.
```

pub fn get_position_in_recording_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_POSITION_IN_RECORDING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_POSITION_IN_RECORDING(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_POSITION_IN_RECORDING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_position_in_recording_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_position_in_recording_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_POSITION_IN_RECORDING(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **state**:

pub fn _set_hydraulic_wheel_state_safe(vehicle: Vehicle, state: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_HYDRAULIC_WHEEL_STATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_HYDRAULIC_WHEEL_STATE(vehicle, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_HYDRAULIC_WHEEL_STATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_hydraulic_wheel_state_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_hydraulic_wheel_state_raw(vehicle: i32, state: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_HYDRAULIC_WHEEL_STATE(vehicle, state)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_vehicle_wheels_can_break_off_when_blow_up_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_WHEELS_CAN_BREAK_OFF_WHEN_BLOW_UP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_WHEELS_CAN_BREAK_OFF_WHEN_BLOW_UP(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_WHEELS_CAN_BREAK_OFF_WHEN_BLOW_UP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_wheels_can_break_off_when_blow_up_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_wheels_can_break_off_when_blow_up_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_WHEELS_CAN_BREAK_OFF_WHEN_BLOW_UP(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn set_cargobob_pickup_magnet_falloff_safe(vehicle: Vehicle, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CARGOBOB_PICKUP_MAGNET_FALLOFF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CARGOBOB_PICKUP_MAGNET_FALLOFF(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CARGOBOB_PICKUP_MAGNET_FALLOFF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_cargobob_pickup_magnet_falloff_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_cargobob_pickup_magnet_falloff_raw(vehicle: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CARGOBOB_PICKUP_MAGNET_FALLOFF(vehicle, p1)
}

/// ## Parameters
* **vehicle**: 
* **state**:

pub fn set_vehicle_alarm_safe(vehicle: Vehicle, state: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_ALARM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_ALARM(vehicle, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_ALARM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_alarm_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_alarm_raw(vehicle: i32, state: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_ALARM(vehicle, state)
}

/// This works on helicopters and planes.

Prevents a helicopter from exploding due to relatively minor body damage. 

```
NativeDB Introduced: v1103
```

pub fn set_disable_heli_explode_from_body_damage_safe(helicopter: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_DISABLE_HELI_EXPLODE_FROM_BODY_DAMAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DISABLE_HELI_EXPLODE_FROM_BODY_DAMAGE(helicopter)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DISABLE_HELI_EXPLODE_FROM_BODY_DAMAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_disable_heli_explode_from_body_damage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_disable_heli_explode_from_body_damage_raw(helicopter: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DISABLE_HELI_EXPLODE_FROM_BODY_DAMAGE(helicopter)
}

/// ```
CLEAR_VEHICLE_*
```

pub fn _0x4419966c9936071a_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _0x4419966C9936071A");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x4419966C9936071A(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x4419966C9936071A
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x4419966c9936071a_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x4419966c9936071a_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x4419966C9936071A(vehicle)
}

/// ## Parameters
* **vehicle**:

pub fn get_vehicle_tyres_can_burst_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_VEHICLE_TYRES_CAN_BURST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_TYRES_CAN_BURST(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_TYRES_CAN_BURST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_tyres_can_burst_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_tyres_can_burst_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_TYRES_CAN_BURST(vehicle)
}

/// ```
Sets a vehicle to be strongly resistant to explosions. p0 is the vehicle; set p1 to false to toggle the effect on/off.  
```

pub fn set_vehicle_explodes_on_high_explosion_damage_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_EXPLODES_ON_HIGH_EXPLOSION_DAMAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_EXPLODES_ON_HIGH_EXPLOSION_DAMAGE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_EXPLODES_ON_HIGH_EXPLOSION_DAMAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_explodes_on_high_explosion_damage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_explodes_on_high_explosion_damage_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_EXPLODES_ON_HIGH_EXPLOSION_DAMAGE(vehicle, toggle)
}

/// According to decompiled scripts this should work with the `deluxo` and `oppressor2` vehicles.
Does nothing when used on `oppressor2`.

For the deluxo:
- Set `state` to `0.0`: Fully transform to a 'road' vehicle (non-hover mode).
- Set `state` to `1.0`: Fully transform to a 'flying' vehicle (hover mode).

If you set it to something like 0.5, then something [weird happens](https://streamable.com/p6wmr), you end up in some 50% hover mode, 50% not hover mode.

This doesn't need to be called every tick, just once and the vehicle will transform to that state at the usual transform speed. It'll just stop transforming when it reaches the state you provided.

Once this native is used then players will just be able to hit the vehicle transform key to toggle the transformation cycle; it won't block users from using the key.

pub fn set_special_flight_mode_target_ratio_safe(vehicle: Vehicle, state: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SPECIAL_FLIGHT_MODE_TARGET_RATIO");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SPECIAL_FLIGHT_MODE_TARGET_RATIO(vehicle, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SPECIAL_FLIGHT_MODE_TARGET_RATIO
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_special_flight_mode_target_ratio_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_special_flight_mode_target_ratio_raw(vehicle: i32, state: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SPECIAL_FLIGHT_MODE_TARGET_RATIO(vehicle, state)
}

/// ## Parameters
* **vehicle**: 
* **p1**: 
* **p2**:

pub fn _0x99cad8e7afdb60fa_safe(vehicle: Vehicle, p1: f32, p2: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0x99CAD8E7AFDB60FA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x99CAD8E7AFDB60FA(vehicle, p1, p2)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x99CAD8E7AFDB60FA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x99cad8e7afdb60fa_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x99cad8e7afdb60fa_raw(vehicle: i32, p1: f32, p2: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x99CAD8E7AFDB60FA(vehicle, p1, p2)
}

/// Copies sourceVehicle's damage (broken bumpers, broken lights, etc.) to targetVehicle.

pub fn copy_vehicle_damages_safe(sourceVehicle: Vehicle, targetVehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: COPY_VEHICLE_DAMAGES");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::COPY_VEHICLE_DAMAGES(sourceVehicle, targetVehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: COPY_VEHICLE_DAMAGES
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `copy_vehicle_damages_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn copy_vehicle_damages_raw(sourceVehicle: i32, targetVehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::COPY_VEHICLE_DAMAGES(sourceVehicle, targetVehicle)
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)

pub fn is_vehicle_door_fully_open_safe(vehicle: Vehicle, doorIndex: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_DOOR_FULLY_OPEN");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_DOOR_FULLY_OPEN(vehicle, doorIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_DOOR_FULLY_OPEN
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_door_fully_open_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_door_fully_open_raw(vehicle: i32, doorIndex: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_DOOR_FULLY_OPEN(vehicle, doorIndex)
}

/// ```
NativeDB Introduced: v2372
```

pub fn _set_random_boats_in_mp_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_RANDOM_BOATS_IN_MP");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_RANDOM_BOATS_IN_MP(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_RANDOM_BOATS_IN_MP
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_random_boats_in_mp_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_random_boats_in_mp_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_RANDOM_BOATS_IN_MP(toggle)
}

/// ## Parameters
* **vehicle**: 
* **modType**: Refer to eVehicleModType in [`SET_VEHICLE_MOD`](#_0x6AF0636DDEDCB6DD).
* **modIndex**:

pub fn get_vehicle_mod_modifier_value_safe(vehicle: Vehicle, modType: i64, modIndex: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_MOD_MODIFIER_VALUE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MOD_MODIFIER_VALUE(vehicle, modType, modIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MOD_MODIFIER_VALUE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_mod_modifier_value_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_mod_modifier_value_raw(vehicle: i32, modType: i64, modIndex: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MOD_MODIFIER_VALUE(vehicle, modType, modIndex)
}

/// Used alongside [`SET_SPECIAL_FLIGHT_MODE_TARGET_RATIO`](#_0x438B3D7CA026FE91), this function initiates hover transformation for vehicles with a hover mode, like the `Deluxo`, based on a specified ratio (0.0 to 1.0). Incorrect values can glitch the vehicle. Without pairing, vehicles revert to car mode. Ineffective on the `oppressor2`

pub fn set_special_flight_mode_ratio_safe(vehicle: Vehicle, ratio: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SPECIAL_FLIGHT_MODE_RATIO");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SPECIAL_FLIGHT_MODE_RATIO(vehicle, ratio)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SPECIAL_FLIGHT_MODE_RATIO
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_special_flight_mode_ratio_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_special_flight_mode_ratio_raw(vehicle: i32, ratio: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SPECIAL_FLIGHT_MODE_RATIO(vehicle, ratio)
}

/// ```
p1 (toggle) was always 1 (true) except in one case in the b678 scripts.  
```

pub fn set_vehicle_is_racing_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_IS_RACING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_IS_RACING(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_IS_RACING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_is_racing_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_is_racing_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_IS_RACING(vehicle, toggle)
}

/// ## Parameters
* **vehicle**:

pub fn _lower_retractable_wheels_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _LOWER_RETRACTABLE_WHEELS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_LOWER_RETRACTABLE_WHEELS(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _LOWER_RETRACTABLE_WHEELS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_lower_retractable_wheels_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _lower_retractable_wheels_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_LOWER_RETRACTABLE_WHEELS(vehicle)
}

/// Native is significantly more complicated than simply generating a random vector & length.

The 'point' is either 400.0 or 250.0 units away from the Ped's current coordinates; and paths into functions like rage::grcViewport___IsSphereVisible.

```
NativeDB Introduced: v1290
```

pub fn _find_random_point_in_space_safe(ped: Ped) -> NativeResult<Vector3> {
    
    debug!("Calling native function: _FIND_RANDOM_POINT_IN_SPACE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_FIND_RANDOM_POINT_IN_SPACE(ped)
    };
    
    
    Ok(Ok(Vector3::from_raw(result)))
}

/// Raw native function: _FIND_RANDOM_POINT_IN_SPACE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_find_random_point_in_space_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _find_random_point_in_space_raw(ped: i32) -> *mut Vector3_raw {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_FIND_RANDOM_POINT_IN_SPACE(ped)
}

/// ```
Previously named GET_VEHICLE_DEFORMATION_GET_TREE (hash collision)
from Decrypted Scripts I found
VEHICLE::SET_VEHICLE_CEILING_HEIGHT(l_BD9[2/*2*/], 420.0);
```

pub fn set_vehicle_ceiling_height_safe(vehicle: Vehicle, height: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_CEILING_HEIGHT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_CEILING_HEIGHT(vehicle, height)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_CEILING_HEIGHT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_ceiling_height_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_ceiling_height_raw(vehicle: i32, height: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_CEILING_HEIGHT(vehicle, height)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**:

pub fn _0x870b8b7a766615c8_safe(p0: serde_json::Value, p1: serde_json::Value, p2: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x870B8B7A766615C8");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x870B8B7A766615C8(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1), crate::utils::any_to_c_void_ptr(p2))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x870B8B7A766615C8
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x870b8b7a766615c8_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x870b8b7a766615c8_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void, p2: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x870B8B7A766615C8(p0, p1, p2)
}

/// ```
Returns how many possible mods a vehicle has for a given mod type  
```

pub fn get_num_vehicle_mods_safe(vehicle: Vehicle, modType: i64) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_NUM_VEHICLE_MODS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_NUM_VEHICLE_MODS(vehicle, modType)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_NUM_VEHICLE_MODS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_num_vehicle_mods_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_num_vehicle_mods_raw(vehicle: i32, modType: i64) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_NUM_VEHICLE_MODS(vehicle, modType)
}

/// CLEAR_LAST_DRIVEN_VEHICLE native function

pub fn clear_last_driven_vehicle_safe() -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_LAST_DRIVEN_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_LAST_DRIVEN_VEHICLE()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_LAST_DRIVEN_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_last_driven_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_last_driven_vehicle_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_LAST_DRIVEN_VEHICLE()
}

/// ```
Works for vehicles with a retractable landing gear  
landing gear states:  
0: Deployed  
1: Closing  
2: Opening  
3: Retracted  
```

pub fn control_landing_gear_safe(vehicle: Vehicle, state: i64) -> NativeResult<()> {
    
    debug!("Calling native function: CONTROL_LANDING_GEAR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CONTROL_LANDING_GEAR(vehicle, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CONTROL_LANDING_GEAR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `control_landing_gear_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn control_landing_gear_raw(vehicle: i32, state: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CONTROL_LANDING_GEAR(vehicle, state)
}

/// ## Parameters
* **vehicle**: The vehicle to check.
* **seatIndex**: See eSeatPosition declared in [`IS_VEHICLE_SEAT_FREE`](#_0x22AC59A870E6A669).

pub fn can_shuffle_seat_safe(vehicle: Vehicle, seatIndex: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: CAN_SHUFFLE_SEAT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CAN_SHUFFLE_SEAT(vehicle, seatIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: CAN_SHUFFLE_SEAT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `can_shuffle_seat_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn can_shuffle_seat_raw(vehicle: i32, seatIndex: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CAN_SHUFFLE_SEAT(vehicle, seatIndex)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _0x182f266c2d9e2beb_safe(vehicle: Vehicle, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0x182F266C2D9E2BEB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x182F266C2D9E2BEB(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x182F266C2D9E2BEB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x182f266c2d9e2beb_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x182f266c2d9e2beb_raw(vehicle: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x182F266C2D9E2BEB(vehicle, p1)
}

/// ```
Returns max braking of the specified vehicle model.
```

pub fn get_vehicle_model_max_braking_safe(modelHash: u32) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_MODEL_MAX_BRAKING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MODEL_MAX_BRAKING(modelHash)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MODEL_MAX_BRAKING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_model_max_braking_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_model_max_braking_raw(modelHash: u32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MODEL_MAX_BRAKING(modelHash)
}

/// ```
NativeDB Introduced: v1365
```

pub fn _0x887fa38787de8c72_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: _0x887FA38787DE8C72");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x887FA38787DE8C72(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x887FA38787DE8C72
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x887fa38787de8c72_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x887fa38787de8c72_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x887FA38787DE8C72(vehicle)
}

/// Determines if a vehicle is a convertible with an animatable roof. This native checks if the specified vehicle model features a convertible roof that can be lowered or raised through an animation.

```
NativeDB Introduced: v323
```

pub fn is_vehicle_a_convertible_safe(vehicle: Vehicle, checkRoofExtras: bool) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_A_CONVERTIBLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_A_CONVERTIBLE(vehicle, checkRoofExtras)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_A_CONVERTIBLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_a_convertible_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_a_convertible_raw(vehicle: i32, checkRoofExtras: bool) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_A_CONVERTIBLE(vehicle, checkRoofExtras)
}

/// Determines whether a specific vehicle is equipped with a roof.

```
NativeDB Introduced: v323
```

pub fn does_vehicle_have_roof_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: DOES_VEHICLE_HAVE_ROOF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DOES_VEHICLE_HAVE_ROOF(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: DOES_VEHICLE_HAVE_ROOF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `does_vehicle_have_roof_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn does_vehicle_have_roof_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DOES_VEHICLE_HAVE_ROOF(vehicle)
}

/// ## Parameters
* **weaponHash**: 
* **vehicle**: 
* **owner**:

pub fn _is_vehicle_weapon_disabled_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_VEHICLE_WEAPON_DISABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_VEHICLE_WEAPON_DISABLED()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_VEHICLE_WEAPON_DISABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_vehicle_weapon_disabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_vehicle_weapon_disabled_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_VEHICLE_WEAPON_DISABLED()
}

/// ## Parameters
* **p0**:

pub fn _0x35e0654f4bad7971_safe(p0: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x35E0654F4BAD7971");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x35E0654F4BAD7971(p0)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x35E0654F4BAD7971
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x35e0654f4bad7971_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x35e0654f4bad7971_raw(p0: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x35E0654F4BAD7971(p0)
}

/// ```
Does nothing. It's a nullsub.

NativeDB Introduced: v1604
```

pub fn _0x99a05839c46ce316_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x99A05839C46CE316");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x99A05839C46CE316(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x99A05839C46CE316
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x99a05839c46ce316_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x99a05839c46ce316_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x99A05839C46CE316(toggle)
}

/// ```
Roll down all the windows of the vehicle passed through the first parameter.  
```

pub fn roll_down_windows_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: ROLL_DOWN_WINDOWS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::ROLL_DOWN_WINDOWS(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: ROLL_DOWN_WINDOWS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `roll_down_windows_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn roll_down_windows_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::ROLL_DOWN_WINDOWS(vehicle)
}

/// ```
Same call as VEHICLE::_0x0F3B4D4E43177236
```

pub fn _get_boat_boom_position_ratio_2_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _GET_BOAT_BOOM_POSITION_RATIO_2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_BOAT_BOOM_POSITION_RATIO_2(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _GET_BOAT_BOOM_POSITION_RATIO_2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_boat_boom_position_ratio_2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_boat_boom_position_ratio_2_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_BOAT_BOOM_POSITION_RATIO_2(vehicle, p1)
}

/// ```
NativeDB Introduced: v1290
```

pub fn _0x9640e30a7f395e4b_safe(vehicle: Vehicle, p1: serde_json::Value, p2: serde_json::Value, p3: serde_json::Value, p4: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x9640E30A7F395E4B");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x9640E30A7F395E4B(vehicle, crate::utils::any_to_c_void_ptr(p1), crate::utils::any_to_c_void_ptr(p2), crate::utils::any_to_c_void_ptr(p3), crate::utils::any_to_c_void_ptr(p4))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x9640E30A7F395E4B
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x9640e30a7f395e4b_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x9640e30a7f395e4b_raw(vehicle: i32, p1: *mut std::os::raw::c_void, p2: *mut std::os::raw::c_void, p3: *mut std::os::raw::c_void, p4: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x9640E30A7F395E4B(vehicle, p1, p2, p3, p4)
}

/// Sets flag on vehicle that changes behaviour in relation to when player gets wanted level

pub fn set_police_focus_will_track_vehicle_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_POLICE_FOCUS_WILL_TRACK_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_POLICE_FOCUS_WILL_TRACK_VEHICLE(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_POLICE_FOCUS_WILL_TRACK_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_police_focus_will_track_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_police_focus_will_track_vehicle_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_POLICE_FOCUS_WILL_TRACK_VEHICLE(vehicle, p1)
}

/// ```
Commands the driver of an armed vehicle (p0) to shoot its weapon at a target (p1). p3, p4 and p5 are the coordinates of the target. Example:  
WEAPON::SET_CURRENT_PED_VEHICLE_WEAPON(pilot,GAMEPLAY::GET_HASH_KEY("VEHICLE_WEAPON_PLANE_ROCKET"));VEHICLE::SET_VEHICLE_SHOOT_AT_TARGET(pilot, target, targPos.x, targPos.y, targPos.z);  
```

pub fn set_vehicle_shoot_at_target_safe(driver: Ped, entity: Entity, xTarget: f32, yTarget: f32, zTarget: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_SHOOT_AT_TARGET");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_SHOOT_AT_TARGET(driver, entity, xTarget, yTarget, zTarget)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_SHOOT_AT_TARGET
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_shoot_at_target_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_shoot_at_target_raw(driver: i32, entity: i32, xTarget: f32, yTarget: f32, zTarget: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_SHOOT_AT_TARGET(driver, entity, xTarget, yTarget, zTarget)
}

/// ```
Landing gear states:  
0: Deployed  
1: Closing (Retracting)
3: Opening (Deploying)
4: Retracted  
5: Broken
```

Landing gear state 2 is never used.

pub fn get_landing_gear_state_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_LANDING_GEAR_STATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_LANDING_GEAR_STATE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_LANDING_GEAR_STATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_landing_gear_state_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_landing_gear_state_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_LANDING_GEAR_STATE(vehicle)
}

/// ```
NativeDB Introduced: v1290
```

pub fn _0x0205f5365292d2eb_safe(vehicle: Vehicle, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _0x0205F5365292D2EB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x0205F5365292D2EB(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x0205F5365292D2EB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x0205f5365292d2eb_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x0205f5365292d2eb_raw(vehicle: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x0205F5365292D2EB(vehicle, p1)
}

/// IS_BOAT_ANCHORED native function

pub fn is_boat_anchored_safe(boat: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_BOAT_ANCHORED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_BOAT_ANCHORED(boat)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_BOAT_ANCHORED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_boat_anchored_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_boat_anchored_raw(boat: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_BOAT_ANCHORED(boat)
}

/// ```
Gets the number of passengers, NOT including the driver. Use IS_VEHICLE_SEAT_FREE(Vehicle, -1) to also check for the driver  
```

pub fn get_vehicle_number_of_passengers_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_NUMBER_OF_PASSENGERS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_NUMBER_OF_PASSENGERS(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_NUMBER_OF_PASSENGERS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_number_of_passengers_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_number_of_passengers_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_NUMBER_OF_PASSENGERS(vehicle)
}

/// ```
Works just like SET_VEHICLE_ENGINE_HEALTH, didn't saw any difference. But this native works only for planes.
```

pub fn _set_plane_engine_health_safe(vehicle: Vehicle, health: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PLANE_ENGINE_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PLANE_ENGINE_HEALTH(vehicle, health)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PLANE_ENGINE_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_plane_engine_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_plane_engine_health_raw(vehicle: i32, health: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PLANE_ENGINE_HEALTH(vehicle, health)
}

/// ## Parameters
* **vehicle**:

pub fn request_vehicle_high_detail_model_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: REQUEST_VEHICLE_HIGH_DETAIL_MODEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REQUEST_VEHICLE_HIGH_DETAIL_MODEL(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REQUEST_VEHICLE_HIGH_DETAIL_MODEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `request_vehicle_high_detail_model_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn request_vehicle_high_detail_model_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REQUEST_VEHICLE_HIGH_DETAIL_MODEL(vehicle)
}

/// ## Return value

pub fn get_num_vehicle_window_tints_safe() -> NativeResult<i64> {
    
    debug!("Calling native function: GET_NUM_VEHICLE_WINDOW_TINTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_NUM_VEHICLE_WINDOW_TINTS()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_NUM_VEHICLE_WINDOW_TINTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_num_vehicle_window_tints_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_num_vehicle_window_tints_raw() -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_NUM_VEHICLE_WINDOW_TINTS()
}

/// ## Parameters
* **vehicle**: 
* **value**:

pub fn modify_vehicle_top_speed_safe(vehicle: Vehicle, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: MODIFY_VEHICLE_TOP_SPEED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::MODIFY_VEHICLE_TOP_SPEED(vehicle, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: MODIFY_VEHICLE_TOP_SPEED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `modify_vehicle_top_speed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn modify_vehicle_top_speed_raw(vehicle: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::MODIFY_VEHICLE_TOP_SPEED(vehicle, value)
}

/// ```
NativeDB Introduced: v1604
NativeDB Added Parameter 2 (2060): float durationMod : A multiplier applied to the default nitrous duration (default is 3 seconds). 
NativeDB Added Parameter 3 (2060): float power : A multiplier applied to the default nitrous power multiplier (default is 3).
NativeDB Added Parameter 4 (2060): float rechargeTime : A multiplier applied to the default nitrous recharge rate.
NativeDB Added Parameter 5 (2060): BOOL disableSound : A boolean to disable the default nitrous sound when the nitrous is active.
```

Overrides the default settings of a vehicle's nitrous system, allowing custom control over its performance characteristics.

pub fn set_override_nitrous_level_safe(vehicle: Vehicle, override: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_OVERRIDE_NITROUS_LEVEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_OVERRIDE_NITROUS_LEVEL(vehicle, override)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_OVERRIDE_NITROUS_LEVEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_override_nitrous_level_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_override_nitrous_level_raw(vehicle: i32, override: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_OVERRIDE_NITROUS_LEVEL(vehicle, override)
}

/// ```
value between 0.0 and 1.0  
```

pub fn _set_helicopter_roll_pitch_yaw_mult_safe(helicopter: Vehicle, multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_HELICOPTER_ROLL_PITCH_YAW_MULT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_HELICOPTER_ROLL_PITCH_YAW_MULT(helicopter, multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_HELICOPTER_ROLL_PITCH_YAW_MULT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_helicopter_roll_pitch_yaw_mult_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_helicopter_roll_pitch_yaw_mult_raw(helicopter: i32, multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_HELICOPTER_ROLL_PITCH_YAW_MULT(helicopter, multiplier)
}

/// ```
NativeDB Introduced: v1868
```

pub fn _set_tyre_wear_multiplier_safe(vehicle: Vehicle, wheelIndex: i64, multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_TYRE_WEAR_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_TYRE_WEAR_MULTIPLIER(vehicle, wheelIndex, multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_TYRE_WEAR_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_tyre_wear_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_tyre_wear_multiplier_raw(vehicle: i32, wheelIndex: i64, multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_TYRE_WEAR_MULTIPLIER(vehicle, wheelIndex, multiplier)
}

/// ## Parameters
* **vehicleClass**:

pub fn get_vehicle_class_max_agility_safe(vehicleClass: i64) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_CLASS_MAX_AGILITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_CLASS_MAX_AGILITY(vehicleClass)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_CLASS_MAX_AGILITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_class_max_agility_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_class_max_agility_raw(vehicleClass: i64) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_CLASS_MAX_AGILITY(vehicleClass)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x78ceee41f49f421f_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x78CEEE41F49F421F");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x78CEEE41F49F421F(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x78CEEE41F49F421F
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x78ceee41f49f421f_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x78ceee41f49f421f_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x78CEEE41F49F421F(p0, p1)
}

/// ## Parameters
* **vehicle**: 
* **multiplier**:

pub fn set_vehicle_lod_multiplier_safe(vehicle: Vehicle, multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_LOD_MULTIPLIER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_LOD_MULTIPLIER(vehicle, multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_LOD_MULTIPLIER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_lod_multiplier_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_lod_multiplier_raw(vehicle: i32, multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_LOD_MULTIPLIER(vehicle, multiplier)
}

/// ## Parameters
* **vehicle**: The vehicle handle
* **weaponIndex**: The weapon index we're setting ammo for (between `0` and `3`), see description for more.
* **ammoCount**: When set positive, will count down with every fire and prevent firing at `0`. Set `-1` to disable restricted ammo.

pub fn set_vehicle_weapon_restricted_ammo_safe(vehicle: Vehicle, weaponIndex: i64, ammoCount: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_WEAPON_RESTRICTED_AMMO");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_WEAPON_RESTRICTED_AMMO(vehicle, weaponIndex, ammoCount)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_WEAPON_RESTRICTED_AMMO
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_weapon_restricted_ammo_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_weapon_restricted_ammo_raw(vehicle: i32, weaponIndex: i64, ammoCount: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_WEAPON_RESTRICTED_AMMO(vehicle, weaponIndex, ammoCount)
}

/// ```
paintType:
0: Normal
1: Metallic
2: Pearl
3: Matte
4: Metal
5: Chrome
```

pub fn get_num_mod_colors_safe(paintType: i64, p1: bool) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_NUM_MOD_COLORS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_NUM_MOD_COLORS(paintType, p1)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_NUM_MOD_COLORS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_num_mod_colors_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_num_mod_colors_raw(paintType: i64, p1: bool) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_NUM_MOD_COLORS(paintType, p1)
}

/// ```
Checks if vehicle tyre at index exists. Also returns false if tyre was removed.
```

```
NativeDB Introduced: v1493
```

pub fn _does_vehicle_tyre_exist_safe(vehicle: Vehicle, tyreIndex: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: _DOES_VEHICLE_TYRE_EXIST");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_DOES_VEHICLE_TYRE_EXIST(vehicle, tyreIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _DOES_VEHICLE_TYRE_EXIST
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_does_vehicle_tyre_exist_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _does_vehicle_tyre_exist_raw(vehicle: i32, tyreIndex: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_DOES_VEHICLE_TYRE_EXIST(vehicle, tyreIndex)
}

/// ```
Returns a value depending on the lock-on state of vehicle weapons.
0: not locked on
1: locking on
2: locked on
```

pub fn get_vehicle_homing_lockon_state_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_HOMING_LOCKON_STATE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_HOMING_LOCKON_STATE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_HOMING_LOCKON_STATE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_homing_lockon_state_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_homing_lockon_state_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_HOMING_LOCKON_STATE(vehicle)
}

/// DELETE_ALL_TRAINS native function

pub fn delete_all_trains_safe() -> NativeResult<()> {
    
    debug!("Calling native function: DELETE_ALL_TRAINS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DELETE_ALL_TRAINS()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DELETE_ALL_TRAINS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `delete_all_trains_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn delete_all_trains_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DELETE_ALL_TRAINS()
}

/// ## Parameters
* **vehicle**: 
* **r**: 
* **g**: 
* **b**:

pub fn get_vehicle_custom_secondary_colour_safe(vehicle: Vehicle, r: *mut i64, g: *mut i64, b: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_VEHICLE_CUSTOM_SECONDARY_COLOUR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_CUSTOM_SECONDARY_COLOUR(vehicle, r, g, b)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_VEHICLE_CUSTOM_SECONDARY_COLOUR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_custom_secondary_colour_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_custom_secondary_colour_raw(vehicle: i32, r: *mut i64, g: *mut i64, b: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_CUSTOM_SECONDARY_COLOUR(vehicle, r, g, b)
}

/// ## Parameters
* **vehicle**:

pub fn remove_vehicle_high_detail_model_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: REMOVE_VEHICLE_HIGH_DETAIL_MODEL");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_VEHICLE_HIGH_DETAIL_MODEL(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: REMOVE_VEHICLE_HIGH_DETAIL_MODEL
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_vehicle_high_detail_model_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_vehicle_high_detail_model_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_VEHICLE_HIGH_DETAIL_MODEL(vehicle)
}

/// Makes a helicopter resistant to multiple explosions. When enabled, helicopters can survive two or more explosions.

```
NativeDB Introduced: 2545
```

pub fn set_heli_resist_to_explosion_safe(helicopter: Vehicle, bResistToExplosion: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_HELI_RESIST_TO_EXPLOSION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_HELI_RESIST_TO_EXPLOSION(helicopter, bResistToExplosion)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_HELI_RESIST_TO_EXPLOSION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_heli_resist_to_explosion_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_heli_resist_to_explosion_raw(helicopter: i32, bResistToExplosion: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_HELI_RESIST_TO_EXPLOSION(helicopter, bResistToExplosion)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x4d9d109f63fee1d4_safe(p0: serde_json::Value, p1: bool) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x4D9D109F63FEE1D4");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x4D9D109F63FEE1D4(crate::utils::any_to_c_void_ptr(p0), p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x4D9D109F63FEE1D4
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x4d9d109f63fee1d4_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x4d9d109f63fee1d4_raw(p0: *mut std::os::raw::c_void, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x4D9D109F63FEE1D4(p0, p1)
}

/// ```
Returns whether the specified vehicle is currently in a burnout.  
vb.net  
Public Function isVehicleInBurnout(vh As Vehicle) As Boolean  
        Return Native.Function.Call(Of Boolean)(Hash.IS_VEHICLE_IN_BURNOUT, vh)  
    End Function  
```

pub fn is_vehicle_in_burnout_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_VEHICLE_IN_BURNOUT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_IN_BURNOUT(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_IN_BURNOUT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_in_burnout_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_in_burnout_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_IN_BURNOUT(vehicle)
}

/// ```
Returns -1 if the vehicle has no livery  
```

pub fn get_vehicle_livery_count_safe(vehicle: Vehicle) -> NativeResult<i64> {
    
    debug!("Calling native function: GET_VEHICLE_LIVERY_COUNT");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_LIVERY_COUNT(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_LIVERY_COUNT
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_livery_count_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_livery_count_raw(vehicle: i32) -> i64 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_LIVERY_COUNT(vehicle)
}

/// ```
Second Param = LiveryIndex  
example   
int count = VEHICLE::GET_VEHICLE_LIVERY_COUNT(veh);  
for (int i = 0; i < count; i++)    
	{  
char* LiveryName = VEHICLE::GET_LIVERY_NAME(veh, i);  
	}  
this example will work fine to fetch all names   
for example for Sanchez we get   
SANC_LV1  
SANC_LV2  
SANC_LV3  
SANC_LV4  
SANC_LV5  
Use _GET_LABEL_TEXT, to get the localized livery name.  
```

NOTE: You may need to set the vehicle's modKit to 0 by using this function [SET_VEHICLE_MOD_KIT](#_0x1F2AA07F00B3217A) before getting the name, otherwise this native may return NULL.

pub fn get_livery_name_safe(vehicle: Vehicle, liveryIndex: i64) -> NativeResult<String> {
    
    debug!("Calling native function: GET_LIVERY_NAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_LIVERY_NAME(vehicle, liveryIndex)
    };
    
    
    Ok(crate::utils::c_string_to_rust_string(result))
}

/// Raw native function: GET_LIVERY_NAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_livery_name_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_livery_name_raw(vehicle: i32, liveryIndex: i64) -> *const std::os::raw::c_char {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_LIVERY_NAME(vehicle, liveryIndex)
}

/// ## Parameters
* **vehicle**:

pub fn start_vehicle_alarm_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: START_VEHICLE_ALARM");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::START_VEHICLE_ALARM(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: START_VEHICLE_ALARM
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `start_vehicle_alarm_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn start_vehicle_alarm_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::START_VEHICLE_ALARM(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **r**: 
* **g**: 
* **b**:

pub fn get_vehicle_tyre_smoke_color_safe(vehicle: Vehicle, r: *mut i64, g: *mut i64, b: *mut i64) -> NativeResult<()> {
    
    debug!("Calling native function: GET_VEHICLE_TYRE_SMOKE_COLOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_TYRE_SMOKE_COLOR(vehicle, r, g, b)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: GET_VEHICLE_TYRE_SMOKE_COLOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_tyre_smoke_color_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_tyre_smoke_color_raw(vehicle: i32, r: *mut i64, g: *mut i64, b: *mut i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_TYRE_SMOKE_COLOR(vehicle, r, g, b)
}

/// **Usage:**

- Use this native inside a looped function.
- Values:
  - `0.0` = no vehicles on streets
  - `1.0` = normal vehicles on streets

`1.0` Seems to be the maximum.

pub fn set_vehicle_density_multiplier_this_frame_safe(multiplier: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME(multiplier)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_density_multiplier_this_frame_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_density_multiplier_this_frame_raw(multiplier: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_DENSITY_MULTIPLIER_THIS_FRAME(multiplier)
}

/// ## Parameters
* **vehicle**:

pub fn unpause_playback_recorded_vehicle_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: UNPAUSE_PLAYBACK_RECORDED_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::UNPAUSE_PLAYBACK_RECORDED_VEHICLE(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: UNPAUSE_PLAYBACK_RECORDED_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `unpause_playback_recorded_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn unpause_playback_recorded_vehicle_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::UNPAUSE_PLAYBACK_RECORDED_VEHICLE(vehicle)
}

/// Enables or disables a vehicle mod by index (`modType`) for a given vehicle.  

`eVehicleModType` enum, used for `modType` index can be found under [`SET_VEHICLE_MOD`](#_0x6AF0636DDEDCB6DD).

pub fn toggle_vehicle_mod_safe(vehicle: Vehicle, modType: i64, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: TOGGLE_VEHICLE_MOD");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::TOGGLE_VEHICLE_MOD(vehicle, modType, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: TOGGLE_VEHICLE_MOD
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `toggle_vehicle_mod_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn toggle_vehicle_mod_raw(vehicle: i32, modType: i64, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::TOGGLE_VEHICLE_MOD(vehicle, modType, toggle)
}

/// To reset the max speed, set the `speed` value to `0.0` or lower.

pub fn _set_vehicle_max_speed_safe(vehicle: Vehicle, speed: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_MAX_SPEED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_MAX_SPEED(vehicle, speed)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_MAX_SPEED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_max_speed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_max_speed_raw(vehicle: i32, speed: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_MAX_SPEED(vehicle, speed)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _0x2311dd7159f00582_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x2311DD7159F00582");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x2311DD7159F00582(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x2311DD7159F00582
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x2311dd7159f00582_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x2311dd7159f00582_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x2311DD7159F00582(vehicle, p1)
}

/// ```
This is not tested - it's just an assumption.  
Doesn't seem to work.  I'll try with an int instead. --JT  
Read the scripts, im dumpass.   
Doesn't work at all, wether with an bool neither an int  
                            if (!VEHICLE::IS_TAXI_LIGHT_ON(l_115)) {  
                                VEHICLE::SET_TAXI_LIGHTS(l_115, 1);  
                            }  
```

pub fn set_taxi_lights_safe(vehicle: Vehicle, state: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_TAXI_LIGHTS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_TAXI_LIGHTS(vehicle, state)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_TAXI_LIGHTS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_taxi_lights_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_taxi_lights_raw(vehicle: i32, state: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_TAXI_LIGHTS(vehicle, state)
}

/// ```js
// Get the player ped
const playerPed = PlayerPedId();

// Retrieve the vehicle the player is in
const vehicle = GetVehiclePedIsIn(playerPed, false);

// If the player is not in a vehicle, return
if (vehicle === 0) return;

// Fix the vehicle
SetVehicleFixed(vehicle);
```

```cs
using static CitizenFX.Core.Native.API;
// ...

// Get the player ped
int playerPed = PlayerPedId();

// Retrieve the vehicle the player is in
int vehicle = GetVehiclePedIsIn(playerPed, false);

// If the player is not in a vehicle, return
if (vehicle == 0) return; 

// Fix the vehicle
SetVehicleFixed(vehicle);
```

pub fn set_vehicle_fixed_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_FIXED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_FIXED(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_FIXED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_fixed_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_fixed_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_FIXED(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **livery**:

pub fn set_vehicle_livery_safe(vehicle: Vehicle, livery: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_LIVERY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_LIVERY(vehicle, livery)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_LIVERY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_livery_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_livery_raw(vehicle: i32, livery: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_LIVERY(vehicle, livery)
}

/// ```
Explodes a selected vehicle.  
Vehicle vehicle = Vehicle you want to explode.  
BOOL isAudible = If explosion makes a sound.  
BOOL isInvisible = If the explosion is invisible or not.  
First BOOL does not give any visual explosion, the vehicle just falls apart completely but slowly and starts to burn.  
```

pub fn explode_vehicle_safe(vehicle: Vehicle, isAudible: bool, isInvisible: bool) -> NativeResult<()> {
    
    debug!("Calling native function: EXPLODE_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::EXPLODE_VEHICLE(vehicle, isAudible, isInvisible)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: EXPLODE_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `explode_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn explode_vehicle_raw(vehicle: i32, isAudible: bool, isInvisible: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::EXPLODE_VEHICLE(vehicle, isAudible, isInvisible)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**: 
* **p3**:

pub fn _0xfaf2a78061fd9ef4_safe(p0: serde_json::Value, p1: f32, p2: f32, p3: f32) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xFAF2A78061FD9EF4");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xFAF2A78061FD9EF4(crate::utils::any_to_c_void_ptr(p0), p1, p2, p3)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xFAF2A78061FD9EF4
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xfaf2a78061fd9ef4_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xfaf2a78061fd9ef4_raw(p0: *mut std::os::raw::c_void, p1: f32, p2: f32, p3: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xFAF2A78061FD9EF4(p0, p1, p2, p3)
}

/// ## Parameters
* **model**:

pub fn is_this_model_a_plane_safe(model: u32) -> NativeResult<bool> {
    
    debug!("Calling native function: IS_THIS_MODEL_A_PLANE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_THIS_MODEL_A_PLANE(model)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_THIS_MODEL_A_PLANE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_this_model_a_plane_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_this_model_a_plane_raw(model: u32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_THIS_MODEL_A_PLANE(model)
}

/// ## Parameters
* **toggle**:

pub fn set_garbage_trucks_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_GARBAGE_TRUCKS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_GARBAGE_TRUCKS(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_GARBAGE_TRUCKS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_garbage_trucks_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_garbage_trucks_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_GARBAGE_TRUCKS(toggle)
}

/// ```
p2 often set to 1000.0 in the decompiled scripts.  
```

pub fn set_vehicle_body_health_safe(vehicle: Vehicle, value: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_BODY_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_BODY_HEALTH(vehicle, value)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_BODY_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_body_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_body_health_raw(vehicle: i32, value: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_BODY_HEALTH(vehicle, value)
}

/// ## Return value

pub fn has_vehicle_phone_explosive_device_safe() -> NativeResult<bool> {
    
    debug!("Calling native function: HAS_VEHICLE_PHONE_EXPLOSIVE_DEVICE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::HAS_VEHICLE_PHONE_EXPLOSIVE_DEVICE()
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: HAS_VEHICLE_PHONE_EXPLOSIVE_DEVICE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `has_vehicle_phone_explosive_device_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn has_vehicle_phone_explosive_device_raw() -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::HAS_VEHICLE_PHONE_EXPLOSIVE_DEVICE()
}

/// ## Parameters
* **handler**: 
* **container**:

pub fn _is_handler_frame_above_container_safe(handler: Vehicle, container: Entity) -> NativeResult<bool> {
    
    debug!("Calling native function: _IS_HANDLER_FRAME_ABOVE_CONTAINER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_IS_HANDLER_FRAME_ABOVE_CONTAINER(handler, container)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _IS_HANDLER_FRAME_ABOVE_CONTAINER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_is_handler_frame_above_container_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _is_handler_frame_above_container_raw(handler: i32, container: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_IS_HANDLER_FRAME_ABOVE_CONTAINER(handler, container)
}

/// ## Parameters
* **cargobob**: 
* **p1**:

pub fn set_cargobob_pickup_magnet_reduced_falloff_safe(cargobob: Vehicle, p1: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_CARGOBOB_PICKUP_MAGNET_REDUCED_FALLOFF");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_CARGOBOB_PICKUP_MAGNET_REDUCED_FALLOFF(cargobob, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_CARGOBOB_PICKUP_MAGNET_REDUCED_FALLOFF
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_cargobob_pickup_magnet_reduced_falloff_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_cargobob_pickup_magnet_reduced_falloff_raw(cargobob: i32, p1: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_CARGOBOB_PICKUP_MAGNET_REDUCED_FALLOFF(cargobob, p1)
}

/// ## Parameters
* **speedzone**:

pub fn remove_road_node_speed_zone_safe(speedzone: i64) -> NativeResult<bool> {
    
    debug!("Calling native function: REMOVE_ROAD_NODE_SPEED_ZONE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::REMOVE_ROAD_NODE_SPEED_ZONE(speedzone)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: REMOVE_ROAD_NODE_SPEED_ZONE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `remove_road_node_speed_zone_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn remove_road_node_speed_zone_raw(speedzone: i64) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::REMOVE_ROAD_NODE_SPEED_ZONE(speedzone)
}

/// Creates a vehicle with the specified model at the specified position. This vehicle will initially be owned by the creating
script as a mission entity, and the model should be loaded already (e.g. using REQUEST_MODEL).

```
NativeDB Added Parameter 8: BOOL p7
```

pub fn create_vehicle_safe(modelHash: u32, x: f32, y: f32, z: f32, heading: f32, isNetwork: bool, netMissionEntity: bool) -> NativeResult<Vehicle> {
    
    debug!("Calling native function: CREATE_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CREATE_VEHICLE(modelHash, x, y, z, heading, isNetwork, netMissionEntity)
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: CREATE_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `create_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn create_vehicle_raw(modelHash: u32, x: f32, y: f32, z: f32, heading: f32, isNetwork: bool, netMissionEntity: bool) -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CREATE_VEHICLE(modelHash, x, y, z, heading, isNetwork, netMissionEntity)
}

/// ```
Only called once in the decompiled scripts. Presumably activates the specified generator.  
```

pub fn set_script_vehicle_generator_safe(vehicleGenerator: i64, enabled: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_SCRIPT_VEHICLE_GENERATOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_SCRIPT_VEHICLE_GENERATOR(vehicleGenerator, enabled)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_SCRIPT_VEHICLE_GENERATOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_script_vehicle_generator_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_script_vehicle_generator_raw(vehicleGenerator: i64, enabled: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_SCRIPT_VEHICLE_GENERATOR(vehicleGenerator, enabled)
}

/// ## Parameters
* **p0**:

pub fn _0x9d30687c57baa0bb_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0x9D30687C57BAA0BB");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x9D30687C57BAA0BB(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x9D30687C57BAA0BB
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x9d30687c57baa0bb_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x9d30687c57baa0bb_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x9D30687C57BAA0BB(p0)
}

/// ```
Can be used for IS_DLC_VEHICLE_MOD and _0xC098810437312FFF
```

pub fn get_vehicle_mod_identifier_hash_safe(vehicle: Vehicle, modType: i64, modIndex: i64) -> NativeResult<u32> {
    
    debug!("Calling native function: GET_VEHICLE_MOD_IDENTIFIER_HASH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MOD_IDENTIFIER_HASH(vehicle, modType, modIndex)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MOD_IDENTIFIER_HASH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_mod_identifier_hash_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_mod_identifier_hash_raw(vehicle: i32, modType: i64, modIndex: i64) -> u32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MOD_IDENTIFIER_HASH(vehicle, modType, modIndex)
}

/// ```
Sets how much the crane on the tow truck is raised, where 0.0 is fully lowered and 1.0 is fully raised.  
```

pub fn set_vehicle_tow_truck_arm_position_safe(vehicle: Vehicle, position: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_TOW_TRUCK_ARM_POSITION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_TOW_TRUCK_ARM_POSITION(vehicle, position)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_TOW_TRUCK_ARM_POSITION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_tow_truck_arm_position_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_tow_truck_arm_position_raw(vehicle: i32, position: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_TOW_TRUCK_ARM_POSITION(vehicle, position)
}

/// ```
RESET_*

Resets the effect of 0x428AD3E26C8D9EB0
```

pub fn _0xe2f53f172b45ede1_safe() -> NativeResult<()> {
    
    debug!("Calling native function: _0xE2F53F172B45EDE1");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xE2F53F172B45EDE1()
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xE2F53F172B45EDE1
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xe2f53f172b45ede1_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xe2f53f172b45ede1_raw() -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xE2F53F172B45EDE1()
}

/// SET_FORCE_LOW_LOD_ANCHOR_MODE native function

pub fn set_force_low_lod_anchor_mode_safe(boat: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_FORCE_LOW_LOD_ANCHOR_MODE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_FORCE_LOW_LOD_ANCHOR_MODE(boat, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_FORCE_LOW_LOD_ANCHOR_MODE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_force_low_lod_anchor_mode_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_force_low_lod_anchor_mode_raw(boat: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_FORCE_LOW_LOD_ANCHOR_MODE(boat, toggle)
}

/// ## Parameters
* **vehicle**:

pub fn get_vehicle_max_braking_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_VEHICLE_MAX_BRAKING");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_MAX_BRAKING(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_MAX_BRAKING
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_max_braking_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_max_braking_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_MAX_BRAKING(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn set_disable_vehicle_petrol_tank_damage_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_DISABLE_VEHICLE_PETROL_TANK_DAMAGE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_DISABLE_VEHICLE_PETROL_TANK_DAMAGE(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_DISABLE_VEHICLE_PETROL_TANK_DAMAGE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_disable_vehicle_petrol_tank_damage_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_disable_vehicle_petrol_tank_damage_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_DISABLE_VEHICLE_PETROL_TANK_DAMAGE(vehicle, toggle)
}

/// Sets the selected vehicle's colors to their default value (specific variant specified using the colorCombination parameter).

Range of possible values for colorCombination is currently unknown, I couldn't find where these values are stored either (Disquse's guess was vehicles.meta but I haven't seen it in there.)

pub fn set_vehicle_colour_combination_safe(vehicle: Vehicle, colorCombination: i64) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_COLOUR_COMBINATION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_COLOUR_COMBINATION(vehicle, colorCombination)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_COLOUR_COMBINATION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_colour_combination_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_colour_combination_raw(vehicle: i32, colorCombination: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_COLOUR_COMBINATION(vehicle, colorCombination)
}

/// Removes a scripted vehicle generator.

pub fn delete_script_vehicle_generator_safe(vehicleGenerator: i64) -> NativeResult<()> {
    
    debug!("Calling native function: DELETE_SCRIPT_VEHICLE_GENERATOR");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::DELETE_SCRIPT_VEHICLE_GENERATOR(vehicleGenerator)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: DELETE_SCRIPT_VEHICLE_GENERATOR
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `delete_script_vehicle_generator_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn delete_script_vehicle_generator_raw(vehicleGenerator: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::DELETE_SCRIPT_VEHICLE_GENERATOR(vehicleGenerator)
}

/// ## Parameters
* **p0**: 
* **p1**:

pub fn _0x72beccf4b829522e_safe(p0: serde_json::Value, p1: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x72BECCF4B829522E");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x72BECCF4B829522E(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x72BECCF4B829522E
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x72beccf4b829522e_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x72beccf4b829522e_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x72BECCF4B829522E(p0, p1)
}

/// ## Parameters
* **vehicle**: 
* **owned**:

pub fn set_vehicle_has_been_owned_by_player_safe(vehicle: Vehicle, owned: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_HAS_BEEN_OWNED_BY_PLAYER");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_HAS_BEEN_OWNED_BY_PLAYER(vehicle, owned)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_HAS_BEEN_OWNED_BY_PLAYER
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_has_been_owned_by_player_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_has_been_owned_by_player_raw(vehicle: i32, owned: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_HAS_BEEN_OWNED_BY_PLAYER(vehicle, owned)
}

/// ## Parameters
* **p0**:

pub fn _0xcf9159024555488c_safe(p0: serde_json::Value) -> NativeResult<()> {
    // Экспериментальная обработка Any: сериализуем в JSON строку и передаем как CString
    let p0_any_str = serde_json::to_string(&p0)
        .map_err(|e| NativeError::CallFailed(format!("Failed to serialize Any parameter '{}' to JSON string: {}", "p0", e)))?;
    let p0_any_str_cstr = std::ffi::CString::new(p0_any_str)
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert Any JSON string for '{}' to CString: {}", "p0", e)))?;
    
    debug!("Calling native function: _0xCF9159024555488C");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xCF9159024555488C(crate::utils::any_to_c_void_ptr(p0))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xCF9159024555488C
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xcf9159024555488c_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xcf9159024555488c_raw(p0: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xCF9159024555488C(p0)
}

/// ```
NativeDB Introduced: 3095
```

Resets or clears the nitrous system for a specified vehicle. You can check if a vehicle has nitrous with [`IS_NITROUS_ACTIVE`](#_0x491E822B2C464FE4)

pub fn clear_nitrous_safe(vehicle: Vehicle) -> NativeResult<()> {
    
    debug!("Calling native function: CLEAR_NITROUS");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::CLEAR_NITROUS(vehicle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: CLEAR_NITROUS
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `clear_nitrous_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn clear_nitrous_raw(vehicle: i32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::CLEAR_NITROUS(vehicle)
}

/// Returns whether the specified vehicle is designated as a mercenary vehicle

pub fn get_vehicle_is_mercenary_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: GET_VEHICLE_IS_MERCENARY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_VEHICLE_IS_MERCENARY(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_VEHICLE_IS_MERCENARY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_vehicle_is_mercenary_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_vehicle_is_mercenary_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_VEHICLE_IS_MERCENARY(vehicle)
}

/// ## Parameters
* **vehicle**: 
* **toggle**:

pub fn _0xbe5c1255a1830ff5_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xBE5C1255A1830FF5");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xBE5C1255A1830FF5(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xBE5C1255A1830FF5
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xbe5c1255a1830ff5_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xbe5c1255a1830ff5_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xBE5C1255A1830FF5(vehicle, toggle)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn set_boat_disable_avoidance_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: SET_BOAT_DISABLE_AVOIDANCE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_BOAT_DISABLE_AVOIDANCE(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_BOAT_DISABLE_AVOIDANCE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_boat_disable_avoidance_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_boat_disable_avoidance_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_BOAT_DISABLE_AVOIDANCE(vehicle, p1)
}

/// ## Parameters
* **p0**: 
* **p1**: 
* **p2**: 
* **p3**: 
* **p4**: 
* **p5**:

pub fn _0x0581730ab9380412_safe(p0: serde_json::Value, p1: serde_json::Value, p2: serde_json::Value, p3: serde_json::Value, p4: serde_json::Value, p5: serde_json::Value) -> NativeResult<()> {
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
    
    debug!("Calling native function: _0x0581730AB9380412");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x0581730AB9380412(crate::utils::any_to_c_void_ptr(p0), crate::utils::any_to_c_void_ptr(p1), crate::utils::any_to_c_void_ptr(p2), crate::utils::any_to_c_void_ptr(p3), crate::utils::any_to_c_void_ptr(p4), crate::utils::any_to_c_void_ptr(p5))
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x0581730AB9380412
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x0581730ab9380412_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x0581730ab9380412_raw(p0: *mut std::os::raw::c_void, p1: *mut std::os::raw::c_void, p2: *mut std::os::raw::c_void, p3: *mut std::os::raw::c_void, p4: *mut std::os::raw::c_void, p5: *mut std::os::raw::c_void) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x0581730AB9380412(p0, p1, p2, p3, p4, p5)
}

/// ## Parameters
* **plane**: 
* **health**:

pub fn _set_plane_propellers_health_safe(plane: Vehicle, health: f32) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_PLANE_PROPELLERS_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_PLANE_PROPELLERS_HEALTH(plane, health)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_PLANE_PROPELLERS_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_plane_propellers_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_plane_propellers_health_raw(plane: i32, health: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_PLANE_PROPELLERS_HEALTH(plane, health)
}

/// ```
Does nothing. It's a nullsub.

NativeDB Introduced: v1604
```

pub fn _0x82e0ac411e41a5b4_safe(toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0x82E0AC411E41A5B4");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0x82E0AC411E41A5B4(toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0x82E0AC411E41A5B4
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0x82e0ac411e41a5b4_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0x82e0ac411e41a5b4_raw(toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0x82E0AC411E41A5B4(toggle)
}

/// ## Return value

pub fn get_last_driven_vehicle_safe() -> NativeResult<Vehicle> {
    
    debug!("Calling native function: GET_LAST_DRIVEN_VEHICLE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_LAST_DRIVEN_VEHICLE()
    };
    
    
    Ok(Ok(Vehicle(result)))
}

/// Raw native function: GET_LAST_DRIVEN_VEHICLE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_last_driven_vehicle_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_last_driven_vehicle_raw() -> i32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_LAST_DRIVEN_VEHICLE()
}

/// The **actual** [`SET_VEHICLE_EXCLUSIVE_DRIVER`](#_0x41062318F23ED854) native.

pub fn _set_vehicle_exclusive_driver_2_safe(vehicle: Vehicle, ped: Ped, index: i64) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_VEHICLE_EXCLUSIVE_DRIVER_2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_VEHICLE_EXCLUSIVE_DRIVER_2(vehicle, ped, index)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_VEHICLE_EXCLUSIVE_DRIVER_2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_vehicle_exclusive_driver_2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_vehicle_exclusive_driver_2_raw(vehicle: i32, ped: i32, index: i64) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_VEHICLE_EXCLUSIVE_DRIVER_2(vehicle, ped, index)
}

/// Retrieves the agility for a specific boat model, including any vehicle mods. Unlike other vehicles where Rockstar Games typically assess performance based on traction, boats use agility as a measure. This static value is distinct from the traction metrics used for other vehicle types.

```
NativeDB Introduced: v323
```

pub fn get_boat_vehicle_model_agility_safe(modelHash: u32) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_BOAT_VEHICLE_MODEL_AGILITY");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_BOAT_VEHICLE_MODEL_AGILITY(modelHash)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_BOAT_VEHICLE_MODEL_AGILITY
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_boat_vehicle_model_agility_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_boat_vehicle_model_agility_raw(modelHash: u32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_BOAT_VEHICLE_MODEL_AGILITY(modelHash)
}

/// ## Parameters
* **vehicle**: 
* **p1**:

pub fn _set_disable_superdummy_mode_safe(vehicle: Vehicle, p1: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _SET_DISABLE_SUPERDUMMY_MODE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_SET_DISABLE_SUPERDUMMY_MODE(vehicle, p1)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _SET_DISABLE_SUPERDUMMY_MODE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_set_disable_superdummy_mode_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _set_disable_superdummy_mode_raw(vehicle: i32, p1: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_SET_DISABLE_SUPERDUMMY_MODE(vehicle, p1)
}

/// ## Parameters
* **vehicle**:

pub fn _get_vehicle_can_activate_parachute_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _GET_VEHICLE_CAN_ACTIVATE_PARACHUTE");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_VEHICLE_CAN_ACTIVATE_PARACHUTE(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_VEHICLE_CAN_ACTIVATE_PARACHUTE
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_vehicle_can_activate_parachute_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_vehicle_can_activate_parachute_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_VEHICLE_CAN_ACTIVATE_PARACHUTE(vehicle)
}

/// ```
Max 1000.  
At 0 the main rotor will stall.  
```

pub fn get_heli_main_rotor_health_safe(vehicle: Vehicle) -> NativeResult<f32> {
    
    debug!("Calling native function: GET_HELI_MAIN_ROTOR_HEALTH");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::GET_HELI_MAIN_ROTOR_HEALTH(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: GET_HELI_MAIN_ROTOR_HEALTH
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `get_heli_main_rotor_health_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn get_heli_main_rotor_health_raw(vehicle: i32) -> f32 {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::GET_HELI_MAIN_ROTOR_HEALTH(vehicle)
}

/// ```
SET_C*
```

pub fn _0xb2e0c0d6922d31f2_safe(vehicle: Vehicle, toggle: bool) -> NativeResult<()> {
    
    debug!("Calling native function: _0xB2E0C0D6922D31F2");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_0xB2E0C0D6922D31F2(vehicle, toggle)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: _0xB2E0C0D6922D31F2
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_0xb2e0c0d6922d31f2_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _0xb2e0c0d6922d31f2_raw(vehicle: i32, toggle: bool) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_0xB2E0C0D6922D31F2(vehicle, toggle)
}

/// ```
garageName example "Michael - Beverly Hills"
```

pub fn is_vehicle_in_garage_area_safe(garageName: String, vehicle: Vehicle) -> NativeResult<bool> {
    let garageName_cstr = std::ffi::CString::new(garageName.as_str())
        .map_err(|e| NativeError::CallFailed(format!("Failed to convert string parameter '{}' to CString: {}", "garageName", e)))?;
    
    debug!("Calling native function: IS_VEHICLE_IN_GARAGE_AREA");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::IS_VEHICLE_IN_GARAGE_AREA(crate::utils::rust_to_c_string(garageName), vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: IS_VEHICLE_IN_GARAGE_AREA
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `is_vehicle_in_garage_area_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn is_vehicle_in_garage_area_raw(garageName: *const std::os::raw::c_char, vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::IS_VEHICLE_IN_GARAGE_AREA(garageName, vehicle)
}

/// ## Parameters
* **vehicle**: 
* **angleRatio**:

pub fn set_vehicle_flight_nozzle_position_safe(vehicle: Vehicle, angleRatio: f32) -> NativeResult<()> {
    
    debug!("Calling native function: SET_VEHICLE_FLIGHT_NOZZLE_POSITION");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::SET_VEHICLE_FLIGHT_NOZZLE_POSITION(vehicle, angleRatio)
    };
    
    
    Ok(Ok(()))
}

/// Raw native function: SET_VEHICLE_FLIGHT_NOZZLE_POSITION
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `set_vehicle_flight_nozzle_position_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn set_vehicle_flight_nozzle_position_raw(vehicle: i32, angleRatio: f32) -> std::ffi::c_void {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::SET_VEHICLE_FLIGHT_NOZZLE_POSITION(vehicle, angleRatio)
}

/// ```
NativeDB Introduced: v2372
```

pub fn _get_drift_tyres_enabled_safe(vehicle: Vehicle) -> NativeResult<bool> {
    
    debug!("Calling native function: _GET_DRIFT_TYRES_ENABLED");

    // SAFETY: Parameter validation should happen above. FFI calls are inherently unsafe.
    let result = unsafe {
        crate::raw::_GET_DRIFT_TYRES_ENABLED(vehicle)
    };
    
    
    Ok(Ok(result))
}

/// Raw native function: _GET_DRIFT_TYRES_ENABLED
/// 
/// # Safety
/// This function calls directly into the game's native function without validation.
/// Use the safe wrapper `_get_drift_tyres_enabled_safe` instead.
#[allow(clippy::too_many_arguments)] // Natives can have many arguments
pub unsafe fn _get_drift_tyres_enabled_raw(vehicle: i32) -> bool {
    // For raw strings, expect *const c_char, do not create CString here.
    // For Any parameters, the raw function expects *mut c_void. The safe wrapper (if Any is stringified)
    // would have created a CString. If direct Any pointer manipulation is needed, it must be done by the caller of _raw.
    crate::raw::_GET_DRIFT_TYRES_ENABLED(vehicle)
}

