//! MISC native functions
//! 
//! Functions for the misc category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
This sets bit [offset] of [address] to off.
Example:
MISC::CLEAR_BIT(&bitAddress, 1);
To check if this bit has been enabled:
MISC::IS_BIT_SET(bitAddress, 1); // will return 0 afterwards
```



pub fn clear_bit_safe(
        
        
            address: 
        , 
        
        
            offset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE80492A9AC099A93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE80492A9AC099A93u64;
        
        let result = invoke_raw!(
            hash,
                address, 
                offset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_bit_raw(
        address: , 
        offset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE80492A9AC099A93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE80492A9AC099A93u64;

        invoke_raw_typed!(
            hash,
                address, 
                offset
        )
    }
}

/// ## Parameters
*



pub fn _get_num_dispatched_units_for_player_safe(
        
        
            dispatchService: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB4A0C2D56441717u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB4A0C2D56441717u64;
        
        let result = invoke_raw!(
            hash,
                dispatchService
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_num_dispatched_units_for_player_raw(
        dispatchService: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB4A0C2D56441717u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB4A0C2D56441717u64;

        invoke_raw_typed!(
            hash,
                dispatchService
        )
    }
}

/// To remove, see: [`REMOVE_DISPATCH_SPAWN_BLOCKING_AREA`](#_0x264AC28B01B353A5).

See [`IS_POINT_IN_ANGLED_AREA`](#_0x2A70BAE8883E4C81) for the definition of an angled area.



pub fn _add_dispatch_spawn_blocking_angled_area_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            width: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x918C7B2D2FF3928Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x918C7B2D2FF3928Bu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _add_dispatch_spawn_blocking_angled_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        width: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x918C7B2D2FF3928Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x918C7B2D2FF3928Bu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width
        )
    }
}

/// ## Parameters
*



pub fn pause_death_arrest_restart_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C2B3493FBF51C71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C2B3493FBF51C71u64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pause_death_arrest_restart_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C2B3493FBF51C71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C2B3493FBF51C71u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
MISC::_0x957838AAF91BD12D(x, y, z, radius, false, false, false, false); seem to make all objects go away, peds, vehicles etc. All booleans set to true doesn't seem to change anything.
```



pub fn clear_area_leave_vehicle_health_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x957838AAF91BD12Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x957838AAF91BD12Du64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                p4, 
                p5, 
                p6, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_area_leave_vehicle_health_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        p4: , 
        p5: , 
        p6: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x957838AAF91BD12Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x957838AAF91BD12Du64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                p4, 
                p5, 
                p6, 
                p7
        )
    }
}

/// ## Parameters
*



pub fn add_police_restart_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x452736765B31FC4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x452736765B31FC4Bu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_police_restart_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x452736765B31FC4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x452736765B31FC4Bu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn _copy_memory_safe(
        
        
            dst: 
        , 
        
        
            src: 
        , 
        
        
            size: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x213AEB2B90CBA7ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x213AEB2B90CBA7ACu64;
        
        let result = invoke_raw!(
            hash,
                dst, 
                src, 
                size
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _copy_memory_raw(
        dst: , 
        src: , 
        size: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x213AEB2B90CBA7ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x213AEB2B90CBA7ACu64;

        invoke_raw_typed!(
            hash,
                dst, 
                src, 
                size
        )
    }
}

/// Gets the number of the current frame being displayed.



pub fn get_frame_count_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC8202EFC642E6F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC8202EFC642E6F2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_frame_count_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC8202EFC642E6F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC8202EFC642E6F2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Refer to [`SET_WEATHER_TYPE_NOW`](#_0x29B487C359E19889) for weather types.



pub fn get_next_weather_type_hash_name_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x711327CD09C8F162u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x711327CD09C8F162u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_next_weather_type_hash_name_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x711327CD09C8F162u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x711327CD09C8F162u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value
Hard-coded to always return 0.



pub fn is_durango_version_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D982ADB1978442Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D982ADB1978442Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_durango_version_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D982ADB1978442Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D982ADB1978442Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns value of the '-benchmarkIterations' command line option.
```



pub fn _get_benchmark_iterations_from_command_line_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4750FC27570311ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4750FC27570311ECu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_benchmark_iterations_from_command_line_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4750FC27570311ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4750FC27570311ECu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```c
enum eGravityLevel
{
    GRAV_EARTH = 0, // earth gravity 9.8m/s2
    GRAV_MOON = 1, // moon gravity 2.4m/s2
    GRAV_LOW = 2, // very low gravity
    GRAV_ZERO = 3 // zero gravity
}
```



pub fn set_gravity_level_safe(
        
        
            level: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x740E14FAD5842351u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x740E14FAD5842351u64;
        
        let result = invoke_raw!(
            hash,
                level
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gravity_level_raw(
        level: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x740E14FAD5842351u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x740E14FAD5842351u64;

        invoke_raw_typed!(
            hash,
                level
        )
    }
}

/// ## Return value



pub fn queue_mission_repeat_load_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72DE52178C291CB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72DE52178C291CB5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn queue_mission_repeat_load_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72DE52178C291CB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72DE52178C291CB5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_replay_stat_count_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC9274A7EF6B2867u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC9274A7EF6B2867u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_replay_stat_count_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC9274A7EF6B2867u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC9274A7EF6B2867u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Clears an area of cops at the given coordinates and radius.



pub fn clear_area_of_cops_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04F8FC8FCF58F88Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04F8FC8FCF58F88Du64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_area_of_cops_raw(
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04F8FC8FCF58F88Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04F8FC8FCF58F88Du64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// ```
Gets the dimensions of a model.
Calculate (maximum - minimum) to get the size, in which case, Y will be how long the model is.
Example from the scripts: MISC::GET_MODEL_DIMENSIONS(ENTITY::GET_ENTITY_MODEL(PLAYER::PLAYER_PED_ID()), &v_1A, &v_17);
```



pub fn get_model_dimensions_safe(
        
        
            modelHash: 
        , 
        
        
            minimum: 
        , 
        
        
            maximum: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03E8D3D5F549087Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03E8D3D5F549087Au64;
        
        let result = invoke_raw!(
            hash,
                modelHash, 
                minimum, 
                maximum
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_model_dimensions_raw(
        modelHash: , 
        minimum: , 
        maximum: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03E8D3D5F549087Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03E8D3D5F549087Au64;

        invoke_raw_typed!(
            hash,
                modelHash, 
                minimum, 
                maximum
        )
    }
}

/// ## Parameters
*



pub fn block_dispatch_service_resource_creation_safe(
        
        
            dispatchService: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B2BD3773123EA2Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B2BD3773123EA2Fu64;
        
        let result = invoke_raw!(
            hash,
                dispatchService, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn block_dispatch_service_resource_creation_raw(
        dispatchService: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B2BD3773123EA2Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B2BD3773123EA2Fu64;

        invoke_raw_typed!(
            hash,
                dispatchService, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_incident_requested_units_safe(
        
        
            incidentId: 
        , 
        
        
            dispatchService: 
        , 
        
        
            numUnits: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB08B85D860E7BA3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB08B85D860E7BA3Cu64;
        
        let result = invoke_raw!(
            hash,
                incidentId, 
                dispatchService, 
                numUnits
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_incident_requested_units_raw(
        incidentId: , 
        dispatchService: , 
        numUnits: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB08B85D860E7BA3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB08B85D860E7BA3Cu64;

        invoke_raw_typed!(
            hash,
                incidentId, 
                dispatchService, 
                numUnits
        )
    }
}

/// ## Parameters
*



pub fn enable_stunt_jump_set_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE369A5783B866016u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE369A5783B866016u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn enable_stunt_jump_set_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE369A5783B866016u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE369A5783B866016u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
SET_PLAYER_*
```



pub fn _set_player_rockstar_editor_disabled_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D8D44ADBBA61EF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D8D44ADBBA61EF2u64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_player_rockstar_editor_disabled_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D8D44ADBBA61EF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D8D44ADBBA61EF2u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// DO_AUTO_SAVE native function



pub fn do_auto_save_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50EEAAD86232EE55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50EEAAD86232EE55u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn do_auto_save_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50EEAAD86232EE55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50EEAAD86232EE55u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn water_override_set_oceannoiseminamplitude_safe(
        
        
            minAmplitude: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31727907B2C43C55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31727907B2C43C55u64;
        
        let result = invoke_raw!(
            hash,
                minAmplitude
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn water_override_set_oceannoiseminamplitude_raw(
        minAmplitude: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31727907B2C43C55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31727907B2C43C55u64;

        invoke_raw_typed!(
            hash,
                minAmplitude
        )
    }
}

/// ```
Called 4 times in the b617d scripts:
MISC::_A74802FB8D0B7814("CONTRAILS", 0);
```



pub fn unload_cloud_hat_safe(
        
        
            name: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA74802FB8D0B7814u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA74802FB8D0B7814u64;
        
        let result = invoke_raw!(
            hash,
                name, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn unload_cloud_hat_raw(
        name: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA74802FB8D0B7814u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA74802FB8D0B7814u64;

        invoke_raw_typed!(
            hash,
                name, 
                p1
        )
    }
}

/// ## Return value



pub fn is_stunt_jump_in_progress_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A3F19700A4D0525u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A3F19700A4D0525u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_stunt_jump_in_progress_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A3F19700A4D0525u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A3F19700A4D0525u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_this_a_minigame_script_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B30F65D7B710098u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B30F65D7B710098u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_this_a_minigame_script_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B30F65D7B710098u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B30F65D7B710098u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_bits_in_range_safe(
        
        
            var: 
        , 
        
        
            rangeStart: 
        , 
        
        
            rangeEnd: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EF07E15701D61EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EF07E15701D61EDu64;
        
        let result = invoke_raw!(
            hash,
                var, 
                rangeStart, 
                rangeEnd, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_bits_in_range_raw(
        var: , 
        rangeStart: , 
        rangeEnd: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EF07E15701D61EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EF07E15701D61EDu64;

        invoke_raw_typed!(
            hash,
                var, 
                rangeStart, 
                rangeEnd, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn is_projectile_type_within_distance_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            projHash: 
        , 
        
        
            radius: 
        , 
        
        
            ownedByPlayer: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34318593248C8FB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34318593248C8FB2u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                projHash, 
                radius, 
                ownedByPlayer
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_projectile_type_within_distance_raw(
        x: , 
        y: , 
        z: , 
        projHash: , 
        radius: , 
        ownedByPlayer: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34318593248C8FB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34318593248C8FB2u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                projHash, 
                radius, 
                ownedByPlayer
        )
    }
}

/// ```
Determines whether there is a projectile of a specific type within the specified coordinates. The coordinates form a axis-aligned bounding box.  
```



pub fn is_projectile_type_in_area_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            type: 
        , 
        
        
            ownedByPlayer: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E0DC353342C4A6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E0DC353342C4A6Du64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                type, 
                ownedByPlayer
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_projectile_type_in_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        type: , 
        ownedByPlayer: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E0DC353342C4A6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E0DC353342C4A6Du64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                type, 
                ownedByPlayer
        )
    }
}

/// STOP_SAVE_STRUCT native function



pub fn stop_save_struct_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB1774DF12BB9F12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB1774DF12BB9F12u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_save_struct_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB1774DF12BB9F12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB1774DF12BB9F12u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// NETWORK_SET_SCRIPT_IS_SAFE_FOR_NETWORK_GAME native function



pub fn network_set_script_is_safe_for_network_game_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9243BAC96D64C050u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9243BAC96D64C050u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_set_script_is_safe_for_network_game_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9243BAC96D64C050u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9243BAC96D64C050u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _set_snow_level_safe(
        
        
            level: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F06937B0CDCBC1Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F06937B0CDCBC1Au64;
        
        let result = invoke_raw!(
            hash,
                level
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_snow_level_raw(
        level: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F06937B0CDCBC1Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F06937B0CDCBC1Au64;

        invoke_raw_typed!(
            hash,
                level
        )
    }
}

/// ## Return value



pub fn are_profile_settings_valid_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AA3BEFA29F03AD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AA3BEFA29F03AD4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn are_profile_settings_valid_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AA3BEFA29F03AD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AA3BEFA29F03AD4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
The following cloudhats are useable:
altostratus
Cirrus
cirrocumulus
Clear 01
Cloudy 01
Contrails
Horizon
horizonband1
horizonband2
horizonband3
horsey
Nimbus
Puffs
RAIN
Snowy 01
Stormy 01
stratoscumulus
Stripey
shower
Wispy
```



pub fn load_cloud_hat_safe(
        
        
            name: 
        , 
        
        
            transitionTime: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC4842A34657BFCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC4842A34657BFCBu64;
        
        let result = invoke_raw!(
            hash,
                name, 
                transitionTime
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn load_cloud_hat_raw(
        name: , 
        transitionTime: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC4842A34657BFCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC4842A34657BFCBu64;

        invoke_raw_typed!(
            hash,
                name, 
                transitionTime
        )
    }
}

/// ## Parameters
*



pub fn get_coords_of_projectile_type_in_area_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            projectileHash: 
        , 
        
        
            projectilePos: 
        , 
        
        
            ownedByPlayer: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D7A43EC6A5FEA45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D7A43EC6A5FEA45u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                projectileHash, 
                projectilePos, 
                ownedByPlayer
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_coords_of_projectile_type_in_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        projectileHash: , 
        projectilePos: , 
        ownedByPlayer: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D7A43EC6A5FEA45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D7A43EC6A5FEA45u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                projectileHash, 
                projectilePos, 
                ownedByPlayer
        )
    }
}

/// ```
Begins with START_*. Next character in the name is either D or E.
```



pub fn _start_benchmark_recording_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92790862E36C2ADAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92790862E36C2ADAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _start_benchmark_recording_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92790862E36C2ADAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92790862E36C2ADAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 19: Any p18
NativeDB Added Parameter 20: Any p19
NativeDB Added Parameter 21: Any p20
```



pub fn shoot_single_bullet_between_coords_ignore_entity_new_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            damage: 
        , 
        
        
            p7: 
        , 
        
        
            weaponHash: 
        , 
        
        
            ownerPed: 
        , 
        
        
            isAudible: 
        , 
        
        
            isInvisible: 
        , 
        
        
            speed: 
        , 
        
        
            entity: 
        , 
        
        
            p14: 
        , 
        
        
            p15: 
        , 
        
        
            p16: 
        , 
        
        
            p17: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFE5756E7407064Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFE5756E7407064Au64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                damage, 
                p7, 
                weaponHash, 
                ownerPed, 
                isAudible, 
                isInvisible, 
                speed, 
                entity, 
                p14, 
                p15, 
                p16, 
                p17
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn shoot_single_bullet_between_coords_ignore_entity_new_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        damage: , 
        p7: , 
        weaponHash: , 
        ownerPed: , 
        isAudible: , 
        isInvisible: , 
        speed: , 
        entity: , 
        p14: , 
        p15: , 
        p16: , 
        p17: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFE5756E7407064Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFE5756E7407064Au64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                damage, 
                p7, 
                weaponHash, 
                ownerPed, 
                isAudible, 
                isInvisible, 
                speed, 
                entity, 
                p14, 
                p15, 
                p16, 
                p17
        )
    }
}

/// ## Return value



pub fn _get_benchmark_time_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE599A503B3837E1Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE599A503B3837E1Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_benchmark_time_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE599A503B3837E1Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE599A503B3837E1Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Returns the current state of the text input box.

```c
enum eOSKStatus
{
  OSK_INVALID = -1,
  OSK_PENDING = 0,
  OSK_SUCCESS = 1,
  OSK_CANCELLED = 2,
  OSK_FAILED = 3
};
```



pub fn update_onscreen_keyboard_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CF2B696BBF945AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CF2B696BBF945AEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn update_onscreen_keyboard_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CF2B696BBF945AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CF2B696BBF945AEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x5b1f2e327b6b6fe1_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B1F2E327B6B6FE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B1F2E327B6B6FE1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5b1f2e327b6b6fe1_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B1F2E327B6B6FE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B1F2E327B6B6FE1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn absf_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73D57CFFDD12C355u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73D57CFFDD12C355u64;
        
        let result = invoke_raw!(
            hash,
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn absf_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73D57CFFDD12C355u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73D57CFFDD12C355u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ```
=======================================================  
Correction, I have change this to int, instead of int*  
as it doesn't use a pointer to the createdIncident.  
If you try it you will crash (or) freeze.  
=======================================================  
```



pub fn is_incident_valid_safe(
        
        
            incidentId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8BC6461E629BEAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8BC6461E629BEAAu64;
        
        let result = invoke_raw!(
            hash,
                incidentId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_incident_valid_raw(
        incidentId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8BC6461E629BEAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8BC6461E629BEAAu64;

        invoke_raw_typed!(
            hash,
                incidentId
        )
    }
}

/// Returns true if the game is using the metric measurement system (profile setting 227), false if imperial is used.



pub fn should_use_metric_measurements_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3D15555431AB793u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3D15555431AB793u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn should_use_metric_measurements_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3D15555431AB793u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3D15555431AB793u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
The game by default has 5 hospital respawn points. Disabling them all will cause the player to respawn at the last position they were.
```



pub fn disable_hospital_restart_safe(
        
        
            hospitalIndex: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8535819C450EBA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8535819C450EBA8u64;
        
        let result = invoke_raw!(
            hash,
                hospitalIndex, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_hospital_restart_raw(
        hospitalIndex: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8535819C450EBA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8535819C450EBA8u64;

        invoke_raw_typed!(
            hash,
                hospitalIndex, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_random_int_in_range_safe(
        
        
            startRange: 
        , 
        
        
            endRange: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD53343AA4FB7DD28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD53343AA4FB7DD28u64;
        
        let result = invoke_raw!(
            hash,
                startRange, 
                endRange
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_random_int_in_range_raw(
        startRange: , 
        endRange: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD53343AA4FB7DD28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD53343AA4FB7DD28u64;

        invoke_raw_typed!(
            hash,
                startRange, 
                endRange
        )
    }
}

/// ```
Appears to remove stealth kill action from memory
```



pub fn _remove_stealth_kill_safe(
        
        
            hash: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6A12939F16D85BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6A12939F16D85BEu64;
        
        let result = invoke_raw!(
            hash,
                hash, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _remove_stealth_kill_raw(
        hash: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6A12939F16D85BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6A12939F16D85BEu64;

        invoke_raw_typed!(
            hash,
                hash, 
                p1
        )
    }
}

/// Displays a text input box.

```c
enum eKeyboardType
{
  ONSCREEN_KEYBOARD_ENGLISH = 0,
  ONSCREEN_KEYBOARD_LOCALISED = 1,
  ONSCREEN_KEYBOARD_PASSWORD = 2,
  ONSCREEN_KEYBOARD_GAMERTAG = 3,
  ONSCREEN_KEYBOARD_EMAIL = 4,
  ONSCREEN_KEYBOARD_BASIC_ENGLISH = 5,
  ONSCREEN_KEYBOARD_FILENAME = 6
};
```



pub fn display_onscreen_keyboard_safe(
        
        
            keyboardType: 
        , 
        
        
            windowTitle: 
        , 
        
        
            description: 
        , 
        
        
            defaultText: 
        , 
        
        
            defaultConcat1: 
        , 
        
        
            defaultConcat2: 
        , 
        
        
            defaultConcat3: 
        , 
        
        
            maxInputLength: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00DC833F2568DBF6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00DC833F2568DBF6u64;
        
        let result = invoke_raw!(
            hash,
                keyboardType, 
                windowTitle, 
                description, 
                defaultText, 
                defaultConcat1, 
                defaultConcat2, 
                defaultConcat3, 
                maxInputLength
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn display_onscreen_keyboard_raw(
        keyboardType: , 
        windowTitle: , 
        description: , 
        defaultText: , 
        defaultConcat1: , 
        defaultConcat2: , 
        defaultConcat3: , 
        maxInputLength: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00DC833F2568DBF6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00DC833F2568DBF6u64;

        invoke_raw_typed!(
            hash,
                keyboardType, 
                windowTitle, 
                description, 
                defaultText, 
                defaultConcat1, 
                defaultConcat2, 
                defaultConcat3, 
                maxInputLength
        )
    }
}

/// STOP_SAVE_DATA native function



pub fn stop_save_data_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74E20C9145FB66FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74E20C9145FB66FDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_save_data_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74E20C9145FB66FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74E20C9145FB66FDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// In singleplayer it does exactly what the name implies. In FiveM / GTA:Online it shows the `Disconnecting from GTA Online` warning screen message and quits the game.
After quitting, the game process is started again (as the name implies).



pub fn restart_game_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE574A662ACAEFBB1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE574A662ACAEFBB1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn restart_game_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE574A662ACAEFBB1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE574A662ACAEFBB1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Creates a new stunt jump. 

The radius1 and radius2 might actually not be a radius at all, but that's what it seems to me testing them in-game. But they may be 'angle' floats instead, considering this native is named ADD\_STUNT\_JUMP\_**ANGLED**.

Info about the specific 'parameter sections':



pub fn add_stunt_jump_angled_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            radius1: 
        , 
        
        
            x3: 
        , 
        
        
            y3: 
        , 
        
        
            z3: 
        , 
        
        
            x4: 
        , 
        
        
            y4: 
        , 
        
        
            z4: 
        , 
        
        
            radius2: 
        , 
        
        
            camX: 
        , 
        
        
            camY: 
        , 
        
        
            camZ: 
        , 
        
        
            unk1: 
        , 
        
        
            unk2: 
        , 
        
        
            unk3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBE5D803A5360CBFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBE5D803A5360CBFu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                radius1, 
                x3, 
                y3, 
                z3, 
                x4, 
                y4, 
                z4, 
                radius2, 
                camX, 
                camY, 
                camZ, 
                unk1, 
                unk2, 
                unk3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_stunt_jump_angled_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        radius1: , 
        x3: , 
        y3: , 
        z3: , 
        x4: , 
        y4: , 
        z4: , 
        radius2: , 
        camX: , 
        camY: , 
        camZ: , 
        unk1: , 
        unk2: , 
        unk3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBE5D803A5360CBFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBE5D803A5360CBFu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                radius1, 
                x3, 
                y3, 
                z3, 
                x4, 
                y4, 
                z4, 
                radius2, 
                camX, 
                camY, 
                camZ, 
                unk1, 
                unk2, 
                unk3
        )
    }
}

/// ## Parameters
*



pub fn play_tennis_dive_anim_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8FA9C42FC5D7C64Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8FA9C42FC5D7C64Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_tennis_dive_anim_raw(
        ped: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8FA9C42FC5D7C64Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8FA9C42FC5D7C64Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn is_string_null_or_empty_safe(
        
        
            string: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA042B6957743895u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA042B6957743895u64;
        
        let result = invoke_raw!(
            hash,
                string
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_string_null_or_empty_raw(
        string: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA042B6957743895u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA042B6957743895u64;

        invoke_raw_typed!(
            hash,
                string
        )
    }
}

/// ## Parameters
*



pub fn set_fade_in_after_load_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3D78F59DFE18D79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3D78F59DFE18D79u64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_fade_in_after_load_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3D78F59DFE18D79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3D78F59DFE18D79u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_explosive_ammo_this_frame_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA66C71C98D5F2CFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA66C71C98D5F2CFBu64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_explosive_ammo_this_frame_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA66C71C98D5F2CFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA66C71C98D5F2CFBu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// CLEAR_WEATHER_TYPE_PERSIST native function



pub fn clear_weather_type_persist_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCC39339BEF76CF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCC39339BEF76CF5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_weather_type_persist_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCC39339BEF76CF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCC39339BEF76CF5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// See [`IS_POINT_IN_ANGLED_AREA`](#_0x2A70BAE8883E4C81) for the definition of an angled area.

For projectiles, see: [`IS_PROJECTILE_TYPE_IN_ANGLED_AREA`](#_0xF0BC12401061DEA0)



pub fn is_bullet_in_angled_area_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            width: 
        , 
        
        
            ownedByPlayer: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A8B5F3C01E2B477u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A8B5F3C01E2B477u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width, 
                ownedByPlayer
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_bullet_in_angled_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        width: , 
        ownedByPlayer: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A8B5F3C01E2B477u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A8B5F3C01E2B477u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width, 
                ownedByPlayer
        )
    }
}

/// Determines whether a line segment intersects a plane and, if so, returns the parameter value at which this intersection occurs.

```
NativeDB Introduced: v323
```



pub fn get_line_plane_intersection_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            planeX: 
        , 
        
        
            planeY: 
        , 
        
        
            planeZ: 
        , 
        
        
            planeNormalX: 
        , 
        
        
            planeNormalY: 
        , 
        
        
            planeNormalZ: 
        , 
        
        
            intersectionParameter: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF56DFB7B61BE7276u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF56DFB7B61BE7276u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                planeX, 
                planeY, 
                planeZ, 
                planeNormalX, 
                planeNormalY, 
                planeNormalZ, 
                intersectionParameter
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_line_plane_intersection_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        planeX: , 
        planeY: , 
        planeZ: , 
        planeNormalX: , 
        planeNormalY: , 
        planeNormalZ: , 
        intersectionParameter: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF56DFB7B61BE7276u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF56DFB7B61BE7276u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                planeX, 
                planeY, 
                planeZ, 
                planeNormalX, 
                planeNormalY, 
                planeNormalZ, 
                intersectionParameter
        )
    }
}

/// Sets the wind direction. The wind direction will stay persistent until it is reset (see examples).



pub fn set_wind_direction_safe(
        
        
            direction: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB0F4468467B4528u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB0F4468467B4528u64;
        
        let result = invoke_raw!(
            hash,
                direction
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_wind_direction_raw(
        direction: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB0F4468467B4528u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB0F4468467B4528u64;

        invoke_raw_typed!(
            hash,
                direction
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _is_pop_multiplier_area_unk_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1312F4B242609CE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1312F4B242609CE3u64;
        
        let result = invoke_raw!(
            hash,
                id
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_pop_multiplier_area_unk_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1312F4B242609CE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1312F4B242609CE3u64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// Clears the active weather type after a specific amount of time determined by `transitionTimeInMs`.



pub fn clear_weather_type_now_persist_network_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CF97F497FE7D048u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CF97F497FE7D048u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_weather_type_now_persist_network_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CF97F497FE7D048u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CF97F497FE7D048u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Overrides the cloud settings, which are normally controlled by the weather, with the specified named version. This native allows for custom cloud formations and behaviors that deviate from the default settings associated with the game's current weather conditions.

```
NativeDB Introduced: v323
```



pub fn set_cloud_settings_override_safe(
        
        
            overrideSettingsName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02DEAAC8F8EA7FE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02DEAAC8F8EA7FE7u64;
        
        let result = invoke_raw!(
            hash,
                overrideSettingsName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cloud_settings_override_raw(
        overrideSettingsName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02DEAAC8F8EA7FE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02DEAAC8F8EA7FE7u64;

        invoke_raw_typed!(
            hash,
                overrideSettingsName
        )
    }
}

/// Returns whether the In-Game Pause Menu Launched the Benchmark Tests.



pub fn ui_started_end_user_benchmark_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA2F2061875EED90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA2F2061875EED90u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ui_started_end_user_benchmark_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA2F2061875EED90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA2F2061875EED90u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn delete_stunt_jump_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC518000E39DAE1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC518000E39DAE1Fu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn delete_stunt_jump_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC518000E39DAE1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC518000E39DAE1Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
dx = x1 - x2
dy = y1 - y2
```



pub fn get_heading_from_vector_2d_safe(
        
        
            dx: 
        , 
        
        
            dy: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FFB6B224F4B2926u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FFB6B224F4B2926u64;
        
        let result = invoke_raw!(
            hash,
                dx, 
                dy
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_heading_from_vector_2d_raw(
        dx: , 
        dy: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FFB6B224F4B2926u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FFB6B224F4B2926u64;

        invoke_raw_typed!(
            hash,
                dx, 
                dy
        )
    }
}

/// ```
GET_SAVE_*

GET_SAVE_UNLESS_CUSTOM_DOT ?
```



pub fn _0xa4a0065e39c9f25c_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            fadeInAfterLoad: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4A0065E39C9F25Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4A0065E39C9F25Cu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                fadeInAfterLoad, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa4a0065e39c9f25c_raw(
        p0: , 
        p1: , 
        fadeInAfterLoad: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4A0065E39C9F25Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4A0065E39C9F25Cu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                fadeInAfterLoad, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn remove_pop_multiplier_area_safe(
        
        
            id: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB129E447A2EDA4BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB129E447A2EDA4BFu64;
        
        let result = invoke_raw!(
            hash,
                id, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_pop_multiplier_area_raw(
        id: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB129E447A2EDA4BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB129E447A2EDA4BFu64;

        invoke_raw_typed!(
            hash,
                id, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn get_coords_of_projectile_type_within_distance_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            distance: 
        , 
        
        
            outCoords: 
        , 
        
        
            ownedByPlayer: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFB4138EEFED7B81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFB4138EEFED7B81u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                distance, 
                outCoords, 
                ownedByPlayer
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_coords_of_projectile_type_within_distance_raw(
        ped: , 
        weaponHash: , 
        distance: , 
        outCoords: , 
        ownedByPlayer: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFB4138EEFED7B81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFB4138EEFED7B81u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                distance, 
                outCoords, 
                ownedByPlayer
        )
    }
}

/// ## Parameters
*



pub fn start_save_array_with_size_safe(
        
        
            p0: 
        , 
        
        
            size: 
        , 
        
        
            arrayName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60FE567DF1B1AF9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60FE567DF1B1AF9Du64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                size, 
                arrayName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_save_array_with_size_raw(
        p0: , 
        size: , 
        arrayName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60FE567DF1B1AF9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60FE567DF1B1AF9Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                size, 
                arrayName
        )
    }
}

/// ## Parameters
*



pub fn set_minigame_in_progress_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19E00D7322C6F85Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19E00D7322C6F85Bu64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_minigame_in_progress_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19E00D7322C6F85Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19E00D7322C6F85Bu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Compares two strings up to a specified number of characters.
Parameters:
str1 - String to be compared.
str2 - String to be compared.
matchCase - Comparison will be case-sensitive.
maxLength - Maximum number of characters to compare. A value of -1 indicates an infinite length.
Returns:
A value indicating the relationship between the strings:
<0 - The first non-matching character in 'str1' is less than the one in 'str2'. (e.g. 'A' < 'B', so result = -1)
0 - The contents of both strings are equal.
>0 - The first non-matching character in 'str1' is less than the one in 'str2'. (e.g. 'B' > 'A', so result = 1)
Examples:
MISC::COMPARE_STRINGS("STRING", "string", false, -1); // 0; equal
MISC::COMPARE_STRINGS("TESTING", "test", false, 4); // 0; equal
MISC::COMPARE_STRINGS("R2D2", "R2xx", false, 2); // 0; equal
MISC::COMPARE_STRINGS("foo", "bar", false, -1); // 4; 'f' > 'b'
MISC::COMPARE_STRINGS("A", "A", true, 1); // 0; equal
When comparing case-sensitive strings, lower-case characters are greater than upper-case characters:
MISC::COMPARE_STRINGS("A", "a", true, 1); // -1; 'A' < 'a'
MISC::COMPARE_STRINGS("a", "A", true, 1); // 1; 'a' > 'A'
```



pub fn compare_strings_safe(
        
        
            str1: 
        , 
        
        
            str2: 
        , 
        
        
            matchCase: 
        , 
        
        
            maxLength: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E34710ECD4AB0EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E34710ECD4AB0EBu64;
        
        let result = invoke_raw!(
            hash,
                str1, 
                str2, 
                matchCase, 
                maxLength
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn compare_strings_raw(
        str1: , 
        str2: , 
        matchCase: , 
        maxLength: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E34710ECD4AB0EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E34710ECD4AB0EBu64;

        invoke_raw_typed!(
            hash,
                str1, 
                str2, 
                matchCase, 
                maxLength
        )
    }
}

/// ```
Example: CLEAR_AREA(0, 0, 0, 30, true, false, false, false);  
```



pub fn clear_area_safe(
        
        
            X: 
        , 
        
        
            Y: 
        , 
        
        
            Z: 
        , 
        
        
            radius: 
        , 
        
        
            p4: 
        , 
        
        
            ignoreCopCars: 
        , 
        
        
            ignoreObjects: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA56F01F3765B93A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA56F01F3765B93A0u64;
        
        let result = invoke_raw!(
            hash,
                X, 
                Y, 
                Z, 
                radius, 
                p4, 
                ignoreCopCars, 
                ignoreObjects, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_area_raw(
        X: , 
        Y: , 
        Z: , 
        radius: , 
        p4: , 
        ignoreCopCars: , 
        ignoreObjects: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA56F01F3765B93A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA56F01F3765B93A0u64;

        invoke_raw_typed!(
            hash,
                X, 
                Y, 
                Z, 
                radius, 
                p4, 
                ignoreCopCars, 
                ignoreObjects, 
                p7
        )
    }
}

/// Returns the distance between two three-dimensional points, optionally ignoring the Z values.
If useZ is false, only the 2D plane (X-Y) will be considered for calculating the distance.
Consider using this faster native instead: SYSTEM::VDIST - DVIST always takes in consideration the 3D coordinates.



pub fn get_distance_between_coords_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            useZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1B760881820C952u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1B760881820C952u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                useZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_distance_between_coords_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        useZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1B760881820C952u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1B760881820C952u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                useZ
        )
    }
}

/// ```
RESET_*
```



pub fn _0xd9f692d349249528_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9F692D349249528u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9F692D349249528u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xd9f692d349249528_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9F692D349249528u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9F692D349249528u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x97e7e2c04245115b_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97E7E2C04245115Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97E7E2C04245115Bu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x97e7e2c04245115b_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97E7E2C04245115Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97E7E2C04245115Bu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Sets an unknown flag used by CScene in determining which entities from CMapData scene nodes to draw, similar to SET_INSTANCE_PRIORITY_MODE.
```



pub fn set_instance_priority_hint_safe(
        
        
            flag: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5F0A8EBD3F361CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5F0A8EBD3F361CEu64;
        
        let result = invoke_raw!(
            hash,
                flag
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_instance_priority_hint_raw(
        flag: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5F0A8EBD3F361CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5F0A8EBD3F361CEu64;

        invoke_raw_typed!(
            hash,
                flag
        )
    }
}

/// STOP_SAVE_ARRAY native function



pub fn stop_save_array_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04456F95153C6BE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04456F95153C6BE4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_save_array_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04456F95153C6BE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04456F95153C6BE4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Enable/disable optional stunt camera.

```
NativeDB Introduced: v757
```



pub fn toggle_show_optional_stunt_jump_camera_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB80AB299D2EE1BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB80AB299D2EE1BDu64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn toggle_show_optional_stunt_jump_camera_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB80AB299D2EE1BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB80AB299D2EE1BDu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_random_float_in_range_safe(
        
        
            startRange: 
        , 
        
        
            endRange: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x313CE5879CEB6FCDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x313CE5879CEB6FCDu64;
        
        let result = invoke_raw!(
            hash,
                startRange, 
                endRange
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_random_float_in_range_raw(
        startRange: , 
        endRange: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x313CE5879CEB6FCDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x313CE5879CEB6FCDu64;

        invoke_raw_typed!(
            hash,
                startRange, 
                endRange
        )
    }
}

/// ```
GET_TENNIS_*; references 0xFBFEC0E9 = interruptswing
```



pub fn _0x19bfed045c647c49_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19BFED045C647C49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19BFED045C647C49u64;
        
        let result = invoke_raw!(
            hash,
                ped
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x19bfed045c647c49_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19BFED045C647C49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19BFED045C647C49u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// _CLEANUP_ASYNC_INSTALL native function



pub fn _cleanup_async_install_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC79AE21974B01FB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC79AE21974B01FB2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _cleanup_async_install_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC79AE21974B01FB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC79AE21974B01FB2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn add_replay_stat_value_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69FE6DC87BD2A5E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69FE6DC87BD2A5E9u64;
        
        let result = invoke_raw!(
            hash,
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_replay_stat_value_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69FE6DC87BD2A5E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69FE6DC87BD2A5E9u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Parameters
*



pub fn acos_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D08B970013C34B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D08B970013C34B6u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn acos_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D08B970013C34B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D08B970013C34B6u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Added Parameter 10: BOOL p9
```



pub fn add_pop_multiplier_area_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67F6413D3220E18Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67F6413D3220E18Du64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                p6, 
                p7, 
                p8
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_pop_multiplier_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        p6: , 
        p7: , 
        p8: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67F6413D3220E18Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67F6413D3220E18Du64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                p6, 
                p7, 
                p8
        )
    }
}

/// See description of [`ADD_STUNT_JUMP_ANGLED`](#_0xBBE5D803A5360CBF) for detailed info. The only difference really is this one does not have the radius (or angle, not sure) floats parameters for entry and landing zones.



pub fn add_stunt_jump_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            x3: 
        , 
        
        
            y3: 
        , 
        
        
            z3: 
        , 
        
        
            x4: 
        , 
        
        
            y4: 
        , 
        
        
            z4: 
        , 
        
        
            camX: 
        , 
        
        
            camY: 
        , 
        
        
            camZ: 
        , 
        
        
            unk1: 
        , 
        
        
            unk2: 
        , 
        
        
            unk3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A992DA297A4630Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A992DA297A4630Cu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                x3, 
                y3, 
                z3, 
                x4, 
                y4, 
                z4, 
                camX, 
                camY, 
                camZ, 
                unk1, 
                unk2, 
                unk3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_stunt_jump_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        x3: , 
        y3: , 
        z3: , 
        x4: , 
        y4: , 
        z4: , 
        camX: , 
        camY: , 
        camZ: , 
        unk1: , 
        unk2: , 
        unk3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A992DA297A4630Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A992DA297A4630Cu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                x3, 
                y3, 
                z3, 
                x4, 
                y4, 
                z4, 
                camX, 
                camY, 
                camZ, 
                unk1, 
                unk2, 
                unk3
        )
    }
}

/// ## Parameters
*



pub fn atan_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9D1795CD5043663u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9D1795CD5043663u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn atan_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9D1795CD5043663u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9D1795CD5043663u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
This native is adding a zone, where you can change density settings. For example, you can add a zone on 0.0, 0.0, 0.0 with radius 900.0 and vehicleMultiplier 0.0, and you will not see any new population vehicle spawned in a radius of 900.0 from 0.0, 0.0, 0.0. Returns the id. You can have only 15 zones at the same time. You can remove zone using REMOVE_POP_MULTIPLIER_SPHERE
```



pub fn add_pop_multiplier_sphere_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            pedMultiplier: 
        , 
        
        
            vehicleMultiplier: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32C7A7E8C43A1F80u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32C7A7E8C43A1F80u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                pedMultiplier, 
                vehicleMultiplier, 
                p6, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_pop_multiplier_sphere_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        pedMultiplier: , 
        vehicleMultiplier: , 
        p6: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32C7A7E8C43A1F80u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32C7A7E8C43A1F80u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                pedMultiplier, 
                vehicleMultiplier, 
                p6, 
                p7
        )
    }
}

/// ## Parameters
*



pub fn are_strings_equal_safe(
        
        
            string1: 
        , 
        
        
            string2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C515FAB3FF9EA92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C515FAB3FF9EA92u64;
        
        let result = invoke_raw!(
            hash,
                string1, 
                string2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn are_strings_equal_raw(
        string1: , 
        string2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C515FAB3FF9EA92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C515FAB3FF9EA92u64;

        invoke_raw_typed!(
            hash,
                string1, 
                string2
        )
    }
}

/// ## Parameters
*



pub fn allow_mission_creator_warp_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEA36202FC3382DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEA36202FC3382DFu64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn allow_mission_creator_warp_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEA36202FC3382DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEA36202FC3382DFu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Returns the index of the newly created hospital spawn point.  
p3 might be radius?  
```



pub fn add_hospital_restart_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F464EF988465A81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F464EF988465A81u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_hospital_restart_raw(
        x: , 
        y: , 
        z: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F464EF988465A81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F464EF988465A81u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                p3, 
                p4
        )
    }
}

/// ```
This function is hard-coded to always return 0.  
```



pub fn is_frontend_fading_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EA2B6AF97ECA6EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EA2B6AF97ECA6EDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_frontend_fading_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EA2B6AF97ECA6EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EA2B6AF97ECA6EDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn script_race_get_player_split_time_safe(
        
        
            player: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EF5573A1F801A5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EF5573A1F801A5Cu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn script_race_get_player_split_time_raw(
        player: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EF5573A1F801A5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EF5573A1F801A5Cu64;

        invoke_raw_typed!(
            hash,
                player, 
                p1, 
                p2
        )
    }
}

/// ## Return value



pub fn have_credits_reached_end_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x075F1D57402C93BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x075F1D57402C93BAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn have_credits_reached_end_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x075F1D57402C93BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x075F1D57402C93BAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Only found 3 times in decompiled scripts. Not a whole lot to go off of.
MISC::_48F069265A0E4BEC(a_0, "Movie_Name_For_This_Player");
MISC::_48F069265A0E4BEC(&a_0._fB, "Ringtone_For_This_Player");
MISC::_48F069265A0E4BEC(&a_0._f1EC4._f12[v_A/*6*/], &v_13); // where v_13 is "MPATMLOGSCRS0" thru "MPATMLOGSCRS15"
```



pub fn _0x48f069265a0e4bec_safe(
        
        
            p0: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48F069265A0E4BECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48F069265A0E4BECu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x48f069265a0e4bec_raw(
        p0: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48F069265A0E4BECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48F069265A0E4BECu64;

        invoke_raw_typed!(
            hash,
                p0, 
                name
        )
    }
}

/// ```
NativeDB Added Parameter 13: Any p12
```



pub fn clear_angled_area_of_vehicles_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            width: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        , 
        
        
            p10: 
        , 
        
        
            p11: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11DB3500F042A8AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11DB3500F042A8AAu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width, 
                p7, 
                p8, 
                p9, 
                p10, 
                p11
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_angled_area_of_vehicles_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        width: , 
        p7: , 
        p8: , 
        p9: , 
        p10: , 
        p11: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11DB3500F042A8AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11DB3500F042A8AAu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width, 
                p7, 
                p8, 
                p9, 
                p10, 
                p11
        )
    }
}

/// This native always come right before SET_ENTITY_QUATERNION where its final 4 parameters are SLERP_NEAR_QUATERNION p9 to p12



pub fn slerp_near_quaternion_safe(
        
        
            t: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            w: 
        , 
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            w1: 
        , 
        
        
            outX: 
        , 
        
        
            outY: 
        , 
        
        
            outZ: 
        , 
        
        
            outW: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2F6A2FA49278625u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2F6A2FA49278625u64;
        
        let result = invoke_raw!(
            hash,
                t, 
                x, 
                y, 
                z, 
                w, 
                x1, 
                y1, 
                z1, 
                w1, 
                outX, 
                outY, 
                outZ, 
                outW
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn slerp_near_quaternion_raw(
        t: , 
        x: , 
        y: , 
        z: , 
        w: , 
        x1: , 
        y1: , 
        z1: , 
        w1: , 
        outX: , 
        outY: , 
        outZ: , 
        outW: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2F6A2FA49278625u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2F6A2FA49278625u64;

        invoke_raw_typed!(
            hash,
                t, 
                x, 
                y, 
                z, 
                w, 
                x1, 
                y1, 
                z1, 
                w1, 
                outX, 
                outY, 
                outZ, 
                outW
        )
    }
}

/// ## Return value
The wind speed in meters per second



pub fn get_wind_speed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8CF1CC0AFCD3F12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8CF1CC0AFCD3F12u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_wind_speed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8CF1CC0AFCD3F12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8CF1CC0AFCD3F12u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p3 - possibly radius?  
```



pub fn has_bullet_impacted_in_area_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9870ACFB89A90995u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9870ACFB89A90995u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                p3, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_bullet_impacted_in_area_raw(
        x: , 
        y: , 
        z: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9870ACFB89A90995u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9870ACFB89A90995u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                p3, 
                p4, 
                p5
        )
    }
}

/// Refer to [`SET_WEATHER_TYPE_NOW`](#_0x29B487C359E19889) for weather types.

```
Mixes two weather types. If percentWeather2 is set to 0.0f, then the weather will be entirely of weatherType1, if it is set to 1.0f it will be entirely of weatherType2. If it's set somewhere in between, there will be a mixture of weather behaviors. To test, try this in the RPH console, and change the float to different values between 0 and 1:  
execute "NativeFunction.Natives.x578C752848ECFA0C(Game.GetHashKey(""RAIN""), Game.GetHashKey(""SMOG""), 0.50f);  
```



pub fn _set_weather_type_transition_safe(
        
        
            weatherType1: 
        , 
        
        
            weatherType2: 
        , 
        
        
            percentWeather2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x578C752848ECFA0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x578C752848ECFA0Cu64;
        
        let result = invoke_raw!(
            hash,
                weatherType1, 
                weatherType2, 
                percentWeather2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_weather_type_transition_raw(
        weatherType1: , 
        weatherType2: , 
        percentWeather2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x578C752848ECFA0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x578C752848ECFA0Cu64;

        invoke_raw_typed!(
            hash,
                weatherType1, 
                weatherType2, 
                percentWeather2
        )
    }
}

/// ## Parameters
*



pub fn register_bool_to_save_safe(
        
        
            p0: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8F4131414C835A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8F4131414C835A1u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn register_bool_to_save_raw(
        p0: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8F4131414C835A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8F4131414C835A1u64;

        invoke_raw_typed!(
            hash,
                p0, 
                name
        )
    }
}

/// ## Parameters
*



pub fn set_dispatch_spawn_location_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD10F442036302D50u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD10F442036302D50u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_dispatch_spawn_location_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD10F442036302D50u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD10F442036302D50u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ```
Returns true if command line option '-benchmark' is set.
```



pub fn _is_command_line_benchmark_value_set_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA049A5BE0F04F2F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA049A5BE0F04F2F8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_command_line_benchmark_value_set_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA049A5BE0F04F2F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA049A5BE0F04F2F8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
I*
```



pub fn _0x8d74e26f54b4e5c3_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D74E26F54B4E5C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D74E26F54B4E5C3u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8d74e26f54b4e5c3_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D74E26F54B4E5C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D74E26F54B4E5C3u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn get_rain_level_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96695E368AD855F3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96695E368AD855F3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_rain_level_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96695E368AD855F3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96695E368AD855F3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Attempts to identify the highest ground Z-coordinate and determine the corresponding surface normal directly beneath a specified 3D coordinate.

```
NativeDB Introduced: v323
```



pub fn get_ground_z_and_normal_for_3d_coord_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            groundZ: 
        , 
        
        
            normal: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BDC7BFC57A81E76u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BDC7BFC57A81E76u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                groundZ, 
                normal
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ground_z_and_normal_for_3d_coord_raw(
        x: , 
        y: , 
        z: , 
        groundZ: , 
        normal: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BDC7BFC57A81E76u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BDC7BFC57A81E76u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                groundZ, 
                normal
        )
    }
}

/// ```
Saves the benchmark recording to %USERPROFILE%\Documents\Rockstar Games\GTA V\Benchmarks and submits some metrics.
```



pub fn _save_benchmark_recording_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37DEB0AA183FB6D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37DEB0AA183FB6D8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _save_benchmark_recording_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37DEB0AA183FB6D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37DEB0AA183FB6D8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```c
enum eFontBitField
{
  FONT_BIT_STANDARD = 1,
  FONT_BIT_CURSIVE = 2,
  FONT_BIT_ROCKSTAR_TAG = 4
  FONT_BIT_LEADERBOARD = 8
  FONT_BIT_CONDENSED = 16
  FONT_BIT_FIXED_WIDTH_NUMBERS = 32
  FONT_BIT_CONDENSED_NOT_GAMERNAME = 64
  FONT_BIT_PRICEDOWN = 128
};
```



pub fn next_onscreen_keyboard_result_will_display_using_these_fonts_safe(
        
        
            fontBitField: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3ED1438C1F5C6612u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3ED1438C1F5C6612u64;
        
        let result = invoke_raw!(
            hash,
                fontBitField
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn next_onscreen_keyboard_result_will_display_using_these_fonts_raw(
        fontBitField: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3ED1438C1F5C6612u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3ED1438C1F5C6612u64;

        invoke_raw_typed!(
            hash,
                fontBitField
        )
    }
}

/// ## Parameters
*



pub fn water_override_set_oceanwavemaxamplitude_safe(
        
        
            maxAmplitude: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3E6360DDE733E82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3E6360DDE733E82u64;
        
        let result = invoke_raw!(
            hash,
                maxAmplitude
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn water_override_set_oceanwavemaxamplitude_raw(
        maxAmplitude: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3E6360DDE733E82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3E6360DDE733E82u64;

        invoke_raw_typed!(
            hash,
                maxAmplitude
        )
    }
}

/// ## Parameters
*



pub fn register_save_house_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0714D0A7EEECA54u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0714D0A7EEECA54u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn register_save_house_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0714D0A7EEECA54u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0714D0A7EEECA54u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )
    }
}

/// _CLEAR_CLOUD_HAT native function



pub fn _clear_cloud_hat_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x957E790EA1727B64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x957E790EA1727B64u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _clear_cloud_hat_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x957E790EA1727B64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x957E790EA1727B64u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Sets the localplayer playerinfo state back to playing (State 0)  
States are:  
-1: "Invalid"  
0: "Playing"  
1: "Died"  
2: "Arrested"  
3: "Failed Mission"  
4: "Left Game"  
5: "Respawn"  
6: "In MP Cutscene"  
```



pub fn force_game_state_playing_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0AA53F866B3134Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0AA53F866B3134Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_game_state_playing_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0AA53F866B3134Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0AA53F866B3134Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value
Hard-coded to always return 0.



pub fn is_orbis_version_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA72BC0B675B1519Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA72BC0B675B1519Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_orbis_version_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA72BC0B675B1519Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA72BC0B675B1519Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0xFB00CA71DA386228 native function



pub fn _0xfb00ca71da386228_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB00CA71DA386228u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB00CA71DA386228u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xfb00ca71da386228_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB00CA71DA386228u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB00CA71DA386228u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_credits_active_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB938B7E6D3C0620Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB938B7E6D3C0620Cu64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_credits_active_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB938B7E6D3C0620Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB938B7E6D3C0620Cu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Get inputted "Cheat code", for example:
while (TRUE)
{
    if (MISC::_557E43C447E700A8(${fugitive}))
    {
       // Do something.
    }
    SYSTEM::WAIT(0);
}
Calling this will also set the last saved string hash to zero.
```



pub fn _has_cheat_string_just_been_entered_safe(
        
        
            hash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x557E43C447E700A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x557E43C447E700A8u64;
        
        let result = invoke_raw!(
            hash,
                hash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _has_cheat_string_just_been_entered_raw(
        hash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x557E43C447E700A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x557E43C447E700A8u64;

        invoke_raw_typed!(
            hash,
                hash
        )
    }
}

/// ```
NativeDB Introduced: v2060
```



pub fn _0xfa3ffb0eebc288a3_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA3FFB0EEBC288A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA3FFB0EEBC288A3u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xfa3ffb0eebc288a3_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA3FFB0EEBC288A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA3FFB0EEBC288A3u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// CANCEL_STUNT_JUMP native function



pub fn cancel_stunt_jump_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6B7B0ACD4E4B75Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6B7B0ACD4E4B75Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn cancel_stunt_jump_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6B7B0ACD4E4B75Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6B7B0ACD4E4B75Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Maximum value is 1.  
At a value of 0 the game will still run at a minimum time scale.  
Slow Motion 1: 0.6  
Slow Motion 2: 0.4  
Slow Motion 3: 0.2  
```



pub fn set_time_scale_safe(
        
        
            timeScale: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D408577D440E81Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D408577D440E81Eu64;
        
        let result = invoke_raw!(
            hash,
                timeScale
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_time_scale_raw(
        timeScale: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D408577D440E81Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D408577D440E81Eu64;

        invoke_raw_typed!(
            hash,
                timeScale
        )
    }
}

/// ## Parameters
*



pub fn water_override_set_oceanwaveamplitude_safe(
        
        
            amplitude: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x405591EC8FD9096Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x405591EC8FD9096Du64;
        
        let result = invoke_raw!(
            hash,
                amplitude
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn water_override_set_oceanwaveamplitude_raw(
        amplitude: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x405591EC8FD9096Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x405591EC8FD9096Du64;

        invoke_raw_typed!(
            hash,
                amplitude
        )
    }
}

/// ```
I looked through the PC scripts that this site provides you with a link to find. It shows the last param mainly uses, (0, 2, 6, 16, and 17) so I am going to assume it is a type of flag.  
```



pub fn clear_area_of_objects_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD9B9B385AAC7F5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD9B9B385AAC7F5Bu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_area_of_objects_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD9B9B385AAC7F5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD9B9B385AAC7F5Bu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                flags
        )
    }
}

/// ## Parameters
*



pub fn set_explosive_melee_this_frame_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF1BED81BFDC0FE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF1BED81BFDC0FE0u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_explosive_melee_this_frame_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF1BED81BFDC0FE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF1BED81BFDC0FE0u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn _0xb51b9ab9ef81868c_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB51B9AB9EF81868Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB51B9AB9EF81868Cu64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb51b9ab9ef81868c_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB51B9AB9EF81868Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB51B9AB9EF81868Cu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value



pub fn _0x397baa01068baa96_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x397BAA01068BAA96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x397BAA01068BAA96u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x397baa01068baa96_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x397BAA01068BAA96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x397BAA01068BAA96u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
SET_INCIDENT_*
```



pub fn _set_incident_unk_safe(
        
        
            incidentId: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD261BA3E7E998072u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD261BA3E7E998072u64;
        
        let result = invoke_raw!(
            hash,
                incidentId, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_incident_unk_raw(
        incidentId: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD261BA3E7E998072u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD261BA3E7E998072u64;

        invoke_raw_typed!(
            hash,
                incidentId, 
                p1
        )
    }
}

/// Shoots a bullet from the first vector to the second vector. The weapon used as weaponHash should already be loaded via REQUEST_WEAPON_ASSET, otherwise the bullet may fail to materialise.



pub fn shoot_single_bullet_between_coords_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            damage: 
        , 
        
        
            pureAccuracy: 
        , 
        
        
            weaponHash: 
        , 
        
        
            ownerPed: 
        , 
        
        
            isAudible: 
        , 
        
        
            isInvisible: 
        , 
        
        
            speed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x867654CBC7606F2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x867654CBC7606F2Cu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                damage, 
                pureAccuracy, 
                weaponHash, 
                ownerPed, 
                isAudible, 
                isInvisible, 
                speed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn shoot_single_bullet_between_coords_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        damage: , 
        pureAccuracy: , 
        weaponHash: , 
        ownerPed: , 
        isAudible: , 
        isInvisible: , 
        speed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x867654CBC7606F2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x867654CBC7606F2Cu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                damage, 
                pureAccuracy, 
                weaponHash, 
                ownerPed, 
                isAudible, 
                isInvisible, 
                speed
        )
    }
}

/// ## Parameters
*



pub fn _get_projectile_near_ped_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            distance: 
        , 
        
        
            outCoords: 
        , 
        
        
            outProjectile: 
        , 
        
        
            ownedByPlayer: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82FDE6A57EE4EE44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82FDE6A57EE4EE44u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                distance, 
                outCoords, 
                outProjectile, 
                ownedByPlayer
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_projectile_near_ped_raw(
        ped: , 
        weaponHash: , 
        distance: , 
        outCoords: , 
        outProjectile: , 
        ownedByPlayer: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82FDE6A57EE4EE44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82FDE6A57EE4EE44u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                distance, 
                outCoords, 
                outProjectile, 
                ownedByPlayer
        )
    }
}

/// Refer to [`SET_WEATHER_TYPE_NOW`](#_0x29B487C359E19889) for weather types.



pub fn is_next_weather_type_safe(
        
        
            weatherType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FAA3A30BEC0F25Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FAA3A30BEC0F25Du64;
        
        let result = invoke_raw!(
            hash,
                weatherType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_next_weather_type_raw(
        weatherType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FAA3A30BEC0F25Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FAA3A30BEC0F25Du64;

        invoke_raw_typed!(
            hash,
                weatherType
        )
    }
}

/// ```
Returns true if the current frontend menu is FE_MENU_VERSION_LANDING_MENU
```



pub fn _landing_menu_is_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BBBD13E5041A79Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BBBD13E5041A79Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _landing_menu_is_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BBBD13E5041A79Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BBBD13E5041A79Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x8951eb9c6906d3c8_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8951EB9C6906D3C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8951EB9C6906D3C8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8951eb9c6906d3c8_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8951EB9C6906D3C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8951EB9C6906D3C8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn reset_dispatch_time_between_spawn_attempts_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB2DB0CAD13154B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB2DB0CAD13154B3u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_dispatch_time_between_spawn_attempts_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB2DB0CAD13154B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB2DB0CAD13154B3u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn get_total_successful_stunt_jumps_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6856EC3D35C81EA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6856EC3D35C81EA4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_total_successful_stunt_jumps_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6856EC3D35C81EA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6856EC3D35C81EA4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn register_text_label_to_save_safe(
        
        
            p0: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDB1232C5BEAE62Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDB1232C5BEAE62Fu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn register_text_label_to_save_raw(
        p0: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDB1232C5BEAE62Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDB1232C5BEAE62Fu64;

        invoke_raw_typed!(
            hash,
                p0, 
                name
        )
    }
}

/// ## Return value



pub fn get_replay_stat_mission_type_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B626A0150E4D449u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B626A0150E4D449u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_replay_stat_mission_type_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B626A0150E4D449u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B626A0150E4D449u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Refer to [`SET_WEATHER_TYPE_NOW`](#_0x29B487C359E19889) for weather types.



pub fn set_override_weather_safe(
        
        
            weatherType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA43D5C6FE51ADBEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA43D5C6FE51ADBEFu64;
        
        let result = invoke_raw!(
            hash,
                weatherType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_override_weather_raw(
        weatherType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA43D5C6FE51ADBEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA43D5C6FE51ADBEFu64;

        invoke_raw_typed!(
            hash,
                weatherType
        )
    }
}

/// ## Return value



pub fn get_status_of_mission_repeat_save_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B5E102E4A42F2BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B5E102E4A42F2BFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_status_of_mission_repeat_save_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B5E102E4A42F2BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B5E102E4A42F2BFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
spawns a few distant/out-of-sight peds, vehicles, animals etc each time it is called  
```



pub fn populate_now_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7472BB270D7B4F3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7472BB270D7B4F3Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn populate_now_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7472BB270D7B4F3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7472BB270D7B4F3Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_dispatch_time_between_spawn_attempts_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44F7CBC1BEB3327Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44F7CBC1BEB3327Du64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_dispatch_time_between_spawn_attempts_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44F7CBC1BEB3327Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44F7CBC1BEB3327Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_this_script_can_be_paused_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA391C728106F7AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA391C728106F7AFu64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_this_script_can_be_paused_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA391C728106F7AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA391C728106F7AFu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
if (MISC::IS_AUSSIE_VERSION()) {
    sub_127a9(&l_31, 1024); // l_31 |= 1024
    l_129 = 3;
    sub_129d2("AUSSIE VERSION IS TRUE!?!?!"); // DEBUG
}
Used to block some of the prostitute stuff due to laws in Australia.
```



pub fn is_aussie_version_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F1935CA1F724008u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F1935CA1F724008u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_aussie_version_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F1935CA1F724008u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F1935CA1F724008u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// See [`IS_POINT_IN_ANGLED_AREA`](#_0x2A70BAE8883E4C81) for the definition of an angled area.

```
NativeDB Removed Parameter 7: float p7
```



pub fn is_projectile_type_in_angled_area_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            width: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0BC12401061DEA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0BC12401061DEA0u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_projectile_type_in_angled_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        width: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0BC12401061DEA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0BC12401061DEA0u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width, 
                p7
        )
    }
}

/// Refer to [`SET_WEATHER_TYPE_NOW`](#_0x29B487C359E19889) for weather types.



pub fn _get_weather_type_transition_safe(
        
        
            weatherType1: 
        , 
        
        
            weatherType2: 
        , 
        
        
            percentWeather2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3BBE884A14BB413u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3BBE884A14BB413u64;
        
        let result = invoke_raw!(
            hash,
                weatherType1, 
                weatherType2, 
                percentWeather2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_weather_type_transition_raw(
        weatherType1: , 
        weatherType2: , 
        percentWeather2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3BBE884A14BB413u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3BBE884A14BB413u64;

        invoke_raw_typed!(
            hash,
                weatherType1, 
                weatherType2, 
                percentWeather2
        )
    }
}

/// ## Parameters
*



pub fn absi_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0D31AD191A74F87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0D31AD191A74F87u64;
        
        let result = invoke_raw!(
            hash,
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn absi_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0D31AD191A74F87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0D31AD191A74F87u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Parameters
*



pub fn water_override_set_ripplemaxbumpiness_safe(
        
        
            maxBumpiness: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F5E6BB6B34540DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F5E6BB6B34540DAu64;
        
        let result = invoke_raw!(
            hash,
                maxBumpiness
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn water_override_set_ripplemaxbumpiness_raw(
        maxBumpiness: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F5E6BB6B34540DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F5E6BB6B34540DAu64;

        invoke_raw_typed!(
            hash,
                maxBumpiness
        )
    }
}

/// ## Parameters
*



pub fn water_override_set_shorewavemaxamplitude_safe(
        
        
            maxAmplitude: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7A1127490312C36u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7A1127490312C36u64;
        
        let result = invoke_raw!(
            hash,
                maxAmplitude
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn water_override_set_shorewavemaxamplitude_raw(
        maxAmplitude: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7A1127490312C36u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7A1127490312C36u64;

        invoke_raw_typed!(
            hash,
                maxAmplitude
        )
    }
}

/// ## Parameters
*



pub fn get_bits_in_range_safe(
        
        
            var: 
        , 
        
        
            rangeStart: 
        , 
        
        
            rangeEnd: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53158863FCC0893Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53158863FCC0893Au64;
        
        let result = invoke_raw!(
            hash,
                var, 
                rangeStart, 
                rangeEnd
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_bits_in_range_raw(
        var: , 
        rangeStart: , 
        rangeEnd: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53158863FCC0893Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53158863FCC0893Au64;

        invoke_raw_typed!(
            hash,
                var, 
                rangeStart, 
                rangeEnd
        )
    }
}

/// RESET_DISPATCH_SPAWN_BLOCKING_AREAS native function



pub fn reset_dispatch_spawn_blocking_areas_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC7BFD5C1D83EA75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC7BFD5C1D83EA75u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_dispatch_spawn_blocking_areas_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC7BFD5C1D83EA75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC7BFD5C1D83EA75u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Related to tennis mode. Checks for `0x0FCED5ADF = swung`



pub fn _0xe95b0c7d5ba3b96b_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE95B0C7D5BA3B96Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE95B0C7D5BA3B96Bu64;
        
        let result = invoke_raw!(
            hash,
                ped
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xe95b0c7d5ba3b96b_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE95B0C7D5BA3B96Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE95B0C7D5BA3B96Bu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Sets the maximum prop density and changes a loading screen flag from 'loading story mode' to 'loading GTA Online'. It causes a loading screen to show as it reloads map data.



pub fn set_instance_priority_mode_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BAE5AD2508DF078u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BAE5AD2508DF078u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_instance_priority_mode_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BAE5AD2508DF078u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BAE5AD2508DF078u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_auto_save_in_progress_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69240733738C19A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69240733738C19A0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_auto_save_in_progress_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x69240733738C19A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x69240733738C19A0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value
Hard-coded to always return 1.



pub fn is_pc_version_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48AF36444B965238u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48AF36444B965238u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_pc_version_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48AF36444B965238u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48AF36444B965238u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// END_REPLAY_STATS native function



pub fn end_replay_stats_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA23E821FBDF8A5F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA23E821FBDF8A5F2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_replay_stats_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA23E821FBDF8A5F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA23E821FBDF8A5F2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Sets the the raw wind speed value. The wind speed will stay persistent until it is reset (see examples).



pub fn set_wind_safe(
        
        
            speed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC3A74E8384A9919u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC3A74E8384A9919u64;
        
        let result = invoke_raw!(
            hash,
                speed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_wind_raw(
        speed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC3A74E8384A9919u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC3A74E8384A9919u64;

        invoke_raw_typed!(
            hash,
                speed
        )
    }
}

/// ```
Sets bit 3 in GtaThread+0x150

SET_T*
```



pub fn _0x6f2135b6129620c1_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F2135B6129620C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F2135B6129620C1u64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6f2135b6129620c1_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F2135B6129620C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F2135B6129620C1u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn register_float_to_save_safe(
        
        
            p0: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CAEC29ECB5DFEBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CAEC29ECB5DFEBBu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn register_float_to_save_raw(
        p0: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CAEC29ECB5DFEBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CAEC29ECB5DFEBBu64;

        invoke_raw_typed!(
            hash,
                p0, 
                name
        )
    }
}

/// ## Parameters
*



pub fn get_tennis_swing_anim_complete_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17DF68D720AA77F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17DF68D720AA77F8u64;
        
        let result = invoke_raw!(
            hash,
                ped
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_tennis_swing_anim_complete_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17DF68D720AA77F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17DF68D720AA77F8u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Sets GtaThread+0x14A

SET_S*
```



pub fn _0x65d2ebb47e1cec21_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65D2EBB47E1CEC21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65D2EBB47E1CEC21u64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x65d2ebb47e1cec21_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65D2EBB47E1CEC21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65D2EBB47E1CEC21u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _set_force_player_to_jump_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1183BCFEE0F93D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1183BCFEE0F93D1u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_force_player_to_jump_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1183BCFEE0F93D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1183BCFEE0F93D1u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
NativeDB Added Parameter 6: Any p5
NativeDB Added Parameter 7: Any p6
```



pub fn create_incident_with_entity_safe(
        
        
            dispatchService: 
        , 
        
        
            ped: 
        , 
        
        
            numUnits: 
        , 
        
        
            radius: 
        , 
        
        
            outIncidentID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x05983472F0494E60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x05983472F0494E60u64;
        
        let result = invoke_raw!(
            hash,
                dispatchService, 
                ped, 
                numUnits, 
                radius, 
                outIncidentID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_incident_with_entity_raw(
        dispatchService: , 
        ped: , 
        numUnits: , 
        radius: , 
        outIncidentID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x05983472F0494E60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x05983472F0494E60u64;

        invoke_raw_typed!(
            hash,
                dispatchService, 
                ped, 
                numUnits, 
                radius, 
                outIncidentID
        )
    }
}

/// ```
Adds a point related to CTacticalAnalysis
```



pub fn _add_tactical_analysis_point_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8721407EE9C3FF6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8721407EE9C3FF6u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _add_tactical_analysis_point_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8721407EE9C3FF6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8721407EE9C3FF6u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ## Return value



pub fn get_mission_flag_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA33CDCCDA663159Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA33CDCCDA663159Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_mission_flag_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA33CDCCDA663159Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA33CDCCDA663159Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
HAS_*
```



pub fn _0x2107a3773771186d_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2107A3773771186Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2107A3773771186Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2107a3773771186d_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2107A3773771186Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2107A3773771186Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value
Hard-coded to always return 0.



pub fn is_ps3_version_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCA1072C29D096C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCA1072C29D096C2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ps3_version_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCA1072C29D096C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCA1072C29D096C2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x39455bf4f4f55186_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        , 
        
        
            p10: 
        , 
        
        
            p11: 
        , 
        
        
            p12: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39455BF4F4F55186u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39455BF4F4F55186u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9, 
                p10, 
                p11, 
                p12
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x39455bf4f4f55186_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: , 
        p10: , 
        p11: , 
        p12: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39455BF4F4F55186u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39455BF4F4F55186u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9, 
                p10, 
                p11, 
                p12
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x31125fd509d9043f_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31125FD509D9043Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31125FD509D9043Fu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x31125fd509d9043f_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31125FD509D9043Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31125FD509D9043Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Closes the onscreen keyboard on console versions of the game.



pub fn cancel_onscreen_keyboard_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58A39BE597CE99CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58A39BE597CE99CDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn cancel_onscreen_keyboard_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58A39BE597CE99CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58A39BE597CE99CDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Although we don't have a jenkins hash for this one, the name is 100% confirmed.
```



pub fn _get_is_player_in_animal_form_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9689123E3F213AA5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9689123E3F213AA5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_is_player_in_animal_form_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9689123E3F213AA5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9689123E3F213AA5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
entity - entity to ignore  
```

```
NativeDB Added Parameter 15: Any p14
```



pub fn shoot_single_bullet_between_coords_ignore_entity_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            damage: 
        , 
        
        
            p7: 
        , 
        
        
            weaponHash: 
        , 
        
        
            ownerPed: 
        , 
        
        
            isAudible: 
        , 
        
        
            isInvisible: 
        , 
        
        
            speed: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3A7742E0B7A2F8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3A7742E0B7A2F8Bu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                damage, 
                p7, 
                weaponHash, 
                ownerPed, 
                isAudible, 
                isInvisible, 
                speed, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn shoot_single_bullet_between_coords_ignore_entity_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        damage: , 
        p7: , 
        weaponHash: , 
        ownerPed: , 
        isAudible: , 
        isInvisible: , 
        speed: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3A7742E0B7A2F8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3A7742E0B7A2F8Bu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                damage, 
                p7, 
                weaponHash, 
                ownerPed, 
                isAudible, 
                isInvisible, 
                speed, 
                entity
        )
    }
}

/// _CLEAR_RESTART_CUSTOM_POSITION native function



pub fn _clear_restart_custom_position_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2716D40842EAF79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2716D40842EAF79u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _clear_restart_custom_position_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2716D40842EAF79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2716D40842EAF79u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Clears an area of projectiles at the given coordinates and radius.



pub fn clear_area_of_projectiles_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A1CB9094635D1A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A1CB9094635D1A6u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_area_of_projectiles_raw(
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A1CB9094635D1A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A1CB9094635D1A6u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// ```
Returns false if it's a null or empty string or if the string is too long. outInteger will be set to -999 in that case.  
If all checks have passed successfully, the return value will be set to whatever strtol(string, 0i64, 10); returns.  
```



pub fn string_to_int_safe(
        
        
            string: 
        , 
        
        
            outInteger: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A5F40FE637EB584u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A5F40FE637EB584u64;
        
        let result = invoke_raw!(
            hash,
                string, 
                outInteger
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn string_to_int_raw(
        string: , 
        outInteger: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A5F40FE637EB584u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A5F40FE637EB584u64;

        invoke_raw_typed!(
            hash,
                string, 
                outInteger
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0xebd3205a207939ed_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBD3205A207939EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBD3205A207939EDu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xebd3205a207939ed_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBD3205A207939EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBD3205A207939EDu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
If the parameter is true, sets the random event flag to true, if the parameter is false, the function does nothing at all.  
Does nothing if the mission flag is set.  
```



pub fn set_random_event_flag_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x971927086CFD2158u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x971927086CFD2158u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_random_event_flag_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x971927086CFD2158u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x971927086CFD2158u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
GET_C*
```



pub fn _0x21c235bc64831e5a_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21C235BC64831E5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21C235BC64831E5Au64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x21c235bc64831e5a_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21C235BC64831E5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21C235BC64831E5Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9
        )
    }
}

/// ```
Finds a position ahead of the player by predicting the players next actions.  
The positions match path finding node positions.  
When roads diverge, the position may rapidly change between two or more positions. This is due to the engine not being certain of which path the player will take.  
```



pub fn find_spawn_point_in_direction_safe(
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            dirX: 
        , 
        
        
            dirY: 
        , 
        
        
            dirZ: 
        , 
        
        
            distance: 
        , 
        
        
            spawnPoint: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6874E2190B0C1972u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6874E2190B0C1972u64;
        
        let result = invoke_raw!(
            hash,
                posX, 
                posY, 
                posZ, 
                dirX, 
                dirY, 
                dirZ, 
                distance, 
                spawnPoint
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn find_spawn_point_in_direction_raw(
        posX: , 
        posY: , 
        posZ: , 
        dirX: , 
        dirY: , 
        dirZ: , 
        distance: , 
        spawnPoint: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6874E2190B0C1972u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6874E2190B0C1972u64;

        invoke_raw_typed!(
            hash,
                posX, 
                posY, 
                posZ, 
                dirX, 
                dirY, 
                dirZ, 
                distance, 
                spawnPoint
        )
    }
}

/// ## Parameters
*



pub fn get_size_of_save_data_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA09F896CE912481Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA09F896CE912481Fu64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_size_of_save_data_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA09F896CE912481Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA09F896CE912481Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Returns duration of how long the game has been in power-saving mode (aka "constrained") in milliseconds.
```



pub fn _get_power_saving_mode_duration_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xABB2FA71C83A1B72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xABB2FA71C83A1B72u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_power_saving_mode_duration_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xABB2FA71C83A1B72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xABB2FA71C83A1B72u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Displays the text input box with support for input with 500 characters.



pub fn display_onscreen_keyboard_with_longer_initial_string_safe(
        
        
            keyboardType: 
        , 
        
        
            windowTitle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA78CFA0366592FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA78CFA0366592FEu64;
        
        let result = invoke_raw!(
            hash,
                keyboardType, 
                windowTitle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn display_onscreen_keyboard_with_longer_initial_string_raw(
        keyboardType: , 
        windowTitle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA78CFA0366592FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA78CFA0366592FEu64;

        invoke_raw_typed!(
            hash,
                keyboardType, 
                windowTitle
        )
    }
}

/// This native gets the ground level (ground elevation) and returns the Z coordinate that represents it.
Note: This native can only calculate the elevation when the coordinates are within the render distance of the client.

```
NativeDB Added Parameter 6: BOOL p5
```



pub fn get_ground_z_for_3d_coord_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            groundZ: 
        , 
        
        
            includeWater: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC906A7DAB05C8D2Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC906A7DAB05C8D2Bu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                groundZ, 
                includeWater
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ground_z_for_3d_coord_raw(
        x: , 
        y: , 
        z: , 
        groundZ: , 
        includeWater: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC906A7DAB05C8D2Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC906A7DAB05C8D2Bu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                groundZ, 
                includeWater
        )
    }
}

/// ## Parameters
*



pub fn register_enum_to_save_safe(
        
        
            p0: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10C2FA78D0E128A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10C2FA78D0E128A1u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn register_enum_to_save_raw(
        p0: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10C2FA78D0E128A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10C2FA78D0E128A1u64;

        invoke_raw_typed!(
            hash,
                p0, 
                name
        )
    }
}

/// Creates and opens a new activity feed post to start filling in.



pub fn activity_feed_create_safe(
        
        
            captionString: 
        , 
        
        
            condensedCaptionString: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4DCDF92BF64236CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4DCDF92BF64236CDu64;
        
        let result = invoke_raw!(
            hash,
                captionString, 
                condensedCaptionString
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn activity_feed_create_raw(
        captionString: , 
        condensedCaptionString: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4DCDF92BF64236CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4DCDF92BF64236CDu64;

        invoke_raw_typed!(
            hash,
                captionString, 
                condensedCaptionString
        )
    }
}

/// ## Parameters
*



pub fn register_int_to_save_safe(
        
        
            p0: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34C9EE5986258415u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34C9EE5986258415u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn register_int_to_save_raw(
        p0: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34C9EE5986258415u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34C9EE5986258415u64;

        invoke_raw_typed!(
            hash,
                p0, 
                name
        )
    }
}

/// ```
This sets bit [offset] of [address] to on.
The offsets used are different bits to be toggled on and off, typically there is only one address used in a script.
Example:
MISC::SET_BIT(&bitAddress, 1);
To check if this bit has been enabled:
MISC::IS_BIT_SET(bitAddress, 1); // will return 1 afterwards
Please note, this method may assign a value to [address] when used.
```



pub fn set_bit_safe(
        
        
            address: 
        , 
        
        
            offset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x933D6A9EEC1BACD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x933D6A9EEC1BACD0u64;
        
        let result = invoke_raw!(
            hash,
                address, 
                offset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_bit_raw(
        address: , 
        offset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x933D6A9EEC1BACD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x933D6A9EEC1BACD0u64;

        invoke_raw_typed!(
            hash,
                address, 
                offset
        )
    }
}

/// ## Return value



pub fn get_num_successful_stunt_jumps_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x996DD1E1E02F1008u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x996DD1E1E02F1008u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_num_successful_stunt_jumps_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x996DD1E1E02F1008u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x996DD1E1E02F1008u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn start_save_data_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9575F812C6A7997u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9575F812C6A7997u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_save_data_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9575F812C6A7997u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9575F812C6A7997u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
Returns true if profile setting 208 is equal to 0.
```



pub fn get_is_auto_save_off_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E04F06094C87047u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E04F06094C87047u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_is_auto_save_off_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E04F06094C87047u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E04F06094C87047u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
CLEAR_*
```



pub fn _0x06462a961e94b67c_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06462A961E94B67Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06462A961E94B67Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x06462a961e94b67c_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06462A961E94B67Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06462A961E94B67Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn remove_pop_multiplier_sphere_safe(
        
        
            id: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6869BECDD8F2403u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6869BECDD8F2403u64;
        
        let result = invoke_raw!(
            hash,
                id, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_pop_multiplier_sphere_raw(
        id: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6869BECDD8F2403u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6869BECDD8F2403u64;

        invoke_raw_typed!(
            hash,
                id, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn water_override_set_shorewaveamplitude_safe(
        
        
            amplitude: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8F87EAD7533B176u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8F87EAD7533B176u64;
        
        let result = invoke_raw!(
            hash,
                amplitude
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn water_override_set_shorewaveamplitude_raw(
        amplitude: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8F87EAD7533B176u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8F87EAD7533B176u64;

        invoke_raw_typed!(
            hash,
                amplitude
        )
    }
}

/// ## Parameters
*



pub fn water_override_set_ripplebumpiness_safe(
        
        
            bumpiness: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C9C0B1EEB1F9072u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C9C0B1EEB1F9072u64;
        
        let result = invoke_raw!(
            hash,
                bumpiness
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn water_override_set_ripplebumpiness_raw(
        bumpiness: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C9C0B1EEB1F9072u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C9C0B1EEB1F9072u64;

        invoke_raw_typed!(
            hash,
                bumpiness
        )
    }
}

/// Sets the current weather type to persist indefinitely until changed.



pub fn set_weather_type_persist_safe(
        
        
            weatherType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x704983DF373B198Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x704983DF373B198Fu64;
        
        let result = invoke_raw!(
            hash,
                weatherType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_weather_type_persist_raw(
        weatherType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x704983DF373B198Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x704983DF373B198Fu64;

        invoke_raw_typed!(
            hash,
                weatherType
        )
    }
}

/// ```
Hardcoded to always return true.
```



pub fn _has_async_install_finished_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14832BF2ABA53FC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14832BF2ABA53FC5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _has_async_install_finished_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14832BF2ABA53FC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14832BF2ABA53FC5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0xd642319c54aadeb6_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD642319C54AADEB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD642319C54AADEB6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xd642319c54aadeb6_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD642319C54AADEB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD642319C54AADEB6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Possibly used to clear scenario points.

CLEAR_*
```



pub fn _0x7ec6f9a478a6a512_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EC6F9A478A6A512u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EC6F9A478A6A512u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7ec6f9a478a6a512_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EC6F9A478A6A512u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EC6F9A478A6A512u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
This native appears on the cheat_controller script and tracks a combination of buttons, which may be used to toggle cheats in-game. Credits to ThreeSocks for the info. The hash contains the combination, while the "amount" represents the amount of buttons used in a combination. The following page can be used to make a button combination: gta5offset.com/ts/hash/
INT_SCORES_SCORTED was a hash collision
```



pub fn _has_button_combination_just_been_entered_safe(
        
        
            hash: 
        , 
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x071E2A839DE82D90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x071E2A839DE82D90u64;
        
        let result = invoke_raw!(
            hash,
                hash, 
                amount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _has_button_combination_just_been_entered_raw(
        hash: , 
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x071E2A839DE82D90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x071E2A839DE82D90u64;

        invoke_raw_typed!(
            hash,
                hash, 
                amount
        )
    }
}

/// ## Parameters
*



pub fn get_replay_stat_at_index_safe(
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8098C8D6597AAE18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8098C8D6597AAE18u64;
        
        let result = invoke_raw!(
            hash,
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_replay_stat_at_index_raw(
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8098C8D6597AAE18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8098C8D6597AAE18u64;

        invoke_raw_typed!(
            hash,
                index
        )
    }
}

/// For a full list, see [here](https://gist.github.com/4mmonium/f76f3ecef649ed275b260b433ea84494).



pub fn terminate_all_scripts_with_this_name_safe(
        
        
            scriptName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DC711BC69C548DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DC711BC69C548DFu64;
        
        let result = invoke_raw!(
            hash,
                scriptName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn terminate_all_scripts_with_this_name_raw(
        scriptName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DC711BC69C548DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DC711BC69C548DFu64;

        invoke_raw_typed!(
            hash,
                scriptName
        )
    }
}

/// This native converts its past string to hash. It is hashed using jenkins one at a time method.



pub fn get_hash_key_safe(
        
        
            string: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD24D37CC275948CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD24D37CC275948CCu64;
        
        let result = invoke_raw!(
            hash,
                string
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_hash_key_raw(
        string: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD24D37CC275948CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD24D37CC275948CCu64;

        invoke_raw_typed!(
            hash,
                string
        )
    }
}

/// _0x693478ACBD7F18E7 native function



pub fn _0x693478acbd7f18e7_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x693478ACBD7F18E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x693478ACBD7F18E7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x693478acbd7f18e7_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x693478ACBD7F18E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x693478ACBD7F18E7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Clears an area of peds at the given coordinates and radius.



pub fn clear_area_of_peds_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE31FD6CE464AC59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE31FD6CE464AC59u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_area_of_peds_raw(
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE31FD6CE464AC59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE31FD6CE464AC59u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// Refer to [`SET_WEATHER_TYPE_NOW`](#_0x29B487C359E19889) for weather types.



pub fn set_weather_type_overtime_persist_safe(
        
        
            weatherType: 
        , 
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB5045B7C42B75BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB5045B7C42B75BFu64;
        
        let result = invoke_raw!(
            hash,
                weatherType, 
                time
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_weather_type_overtime_persist_raw(
        weatherType: , 
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB5045B7C42B75BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB5045B7C42B75BFu64;

        invoke_raw_typed!(
            hash,
                weatherType, 
                time
        )
    }
}

/// ## Parameters
*



pub fn set_save_house_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F548CABEAE553BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F548CABEAE553BCu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_save_house_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F548CABEAE553BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F548CABEAE553BCu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// Controls rain, rain sounds and the creation of puddles.

With an `level` higher than `0.5f`, only the creation of puddles gets faster, rain and rain sound won't increase after that.

With an `level` of `0.0f` rain and rain sounds are disabled and there won't be any new puddles.

To use the rain level of the current weather, call this native with `-1f` as `level`.



pub fn _set_rain_level_safe(
        
        
            level: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x643E26EA6E024D92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x643E26EA6E024D92u64;
        
        let result = invoke_raw!(
            hash,
                level
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_rain_level_raw(
        level: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x643E26EA6E024D92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x643E26EA6E024D92u64;

        invoke_raw_typed!(
            hash,
                level
        )
    }
}

/// ## Parameters
*



pub fn _get_base_element_metadata_safe(
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB335F761606DB47Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB335F761606DB47Cu64;
        
        let result = invoke_raw!(
            hash,
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_base_element_metadata_raw(
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB335F761606DB47Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB335F761606DB47Cu64;

        invoke_raw_typed!(
            hash,
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ```
NativeDB Introduced: 2545
```



pub fn is_japanese_version_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8C0BB75D8A77DB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8C0BB75D8A77DB3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_japanese_version_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8C0BB75D8A77DB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8C0BB75D8A77DB3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Using this native will clamp the wind speed value to a range of 0.0 - 12.0. The wind speed will stay persistent until it is reset (see examples).



pub fn set_wind_speed_safe(
        
        
            speed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE09ECEDBABE47FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE09ECEDBABE47FCu64;
        
        let result = invoke_raw!(
            hash,
                speed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_wind_speed_raw(
        speed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE09ECEDBABE47FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE09ECEDBABE47FCu64;

        invoke_raw_typed!(
            hash,
                speed
        )
    }
}

/// ## Return value



pub fn get_random_event_flag_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2D57F1D764117B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2D57F1D764117B1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_random_event_flag_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2D57F1D764117B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2D57F1D764117B1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
If toggle is true, the ped's head is shown in the pause menu
If toggle is false, the ped's head is not shown in the pause menu
```



pub fn _set_player_is_in_animal_form_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4EBB7E87AA0DBED4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4EBB7E87AA0DBED4u64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_player_is_in_animal_form_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4EBB7E87AA0DBED4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4EBB7E87AA0DBED4u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
HAS_*
Probably something like "has game been started for the first time".
```



pub fn _0x6fddf453c0c756ec_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FDDF453C0C756ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FDDF453C0C756ECu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6fddf453c0c756ec_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FDDF453C0C756ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FDDF453C0C756ECu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Seems to have the same functionality as REGISTER_TEXT_LABEL_TO_SAVE?
MISC::_6F7794F28C6B2535(&a_0._f1, "tlPlateText");
MISC::_6F7794F28C6B2535(&a_0._f1C, "tlPlateText_pending");
MISC::_6F7794F28C6B2535(&a_0._f10B, "tlCarAppPlateText");
"tl" prefix sounds like "Text Label"
```



pub fn _register_text_label_to_save_2_safe(
        
        
            p0: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F7794F28C6B2535u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F7794F28C6B2535u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _register_text_label_to_save_2_raw(
        p0: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F7794F28C6B2535u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F7794F28C6B2535u64;

        invoke_raw_typed!(
            hash,
                p0, 
                name
        )
    }
}

/// ```
Found in the scripts:
MISC::_11B56FBBF7224868("CONTRAILS");
```



pub fn preload_cloud_hat_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11B56FBBF7224868u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11B56FBBF7224868u64;
        
        let result = invoke_raw!(
            hash,
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn preload_cloud_hat_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11B56FBBF7224868u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11B56FBBF7224868u64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ## Parameters
*



pub fn play_tennis_swing_anim_safe(
        
        
            ped: 
        , 
        
        
            animDict: 
        , 
        
        
            animName: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE266ED23311F24D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE266ED23311F24D4u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                animDict, 
                animName, 
                p3, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_tennis_swing_anim_raw(
        ped: , 
        animDict: , 
        animName: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE266ED23311F24D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE266ED23311F24D4u64;

        invoke_raw_typed!(
            hash,
                ped, 
                animDict, 
                animName, 
                p3, 
                p4, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn is_point_obscured_by_a_mission_entity_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE54E209C35FFA18Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE54E209C35FFA18Du64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_point_obscured_by_a_mission_entity_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE54E209C35FFA18Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE54E209C35FFA18Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )
    }
}

/// Enables or disables the specified 'dispatch service' type. 'Dispatch services' are used for spawning AI response peds/vehicles for events such as a fire in the street (type 3 - DT_FireDepartment), or gunfire in a gang area (type 11 - DT_Gangs).

List of dispatch services:

```c
enum eDispatchType
{
	DT_Invalid = 0,
	DT_PoliceAutomobile = 1,
	DT_PoliceHelicopter = 2,
	DT_FireDepartment = 3,
	DT_SwatAutomobile = 4,
	DT_AmbulanceDepartment = 5,
	DT_PoliceRiders = 6,
	DT_PoliceVehicleRequest = 7,
	DT_PoliceRoadBlock = 8,
	DT_PoliceAutomobileWaitPulledOver = 9,
	DT_PoliceAutomobileWaitCruising = 10,
	DT_Gangs = 11,
	DT_SwatHelicopter = 13,
	DT_PoliceBoat = 14,
	DT_ArmyVehicle = 15,
	DT_BikerBackup = 15
};
```

Note that 'dispatch service' has nothing to do with the police scanner (audio), to toggle that, use [SET_AUDIO_FLAG](#_0xB9EFD5C25018725A) with `'PoliceScannerDisabled'`.



pub fn enable_dispatch_service_safe(
        
        
            dispatchService: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC0F817884CDD856u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC0F817884CDD856u64;
        
        let result = invoke_raw!(
            hash,
                dispatchService, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn enable_dispatch_service_raw(
        dispatchService: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC0F817884CDD856u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC0F817884CDD856u64;

        invoke_raw_typed!(
            hash,
                dispatchService, 
                toggle
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0xeb078ca2b5e82add_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB078CA2B5E82ADDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB078CA2B5E82ADDu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xeb078ca2b5e82add_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB078CA2B5E82ADDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB078CA2B5E82ADDu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn water_override_set_rippledisturb_safe(
        
        
            disturb: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9854DFDE0D833D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9854DFDE0D833D6u64;
        
        let result = invoke_raw!(
            hash,
                disturb
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn water_override_set_rippledisturb_raw(
        disturb: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9854DFDE0D833D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9854DFDE0D833D6u64;

        invoke_raw_typed!(
            hash,
                disturb
        )
    }
}

/// ## Parameters
*



pub fn _set_restart_custom_position_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            heading: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x706B5EDCAA7FA663u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x706B5EDCAA7FA663u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                heading
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_restart_custom_position_raw(
        x: , 
        y: , 
        z: , 
        heading: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x706B5EDCAA7FA663u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x706B5EDCAA7FA663u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                heading
        )
    }
}

/// Allows modification of the cloud opacity. It can also be used in other contexts, such as when the player is in a switch state [`IS_PLAYER_SWITCH_IN_PROGRESS`](#_0xD9D2CFFF49FAB35F).



pub fn set_clouds_alpha_safe(
        
        
            opacity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF36199225D6D8C86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF36199225D6D8C86u64;
        
        let result = invoke_raw!(
            hash,
                opacity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_clouds_alpha_raw(
        opacity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF36199225D6D8C86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF36199225D6D8C86u64;

        invoke_raw_typed!(
            hash,
                opacity
        )
    }
}

/// ```
From the scripts:
MISC::_54F157E0336A3822(sub_aa49(a_0), "ForcedStopDirection", v_E);
Related to tennis mode.
SET_*
```



pub fn _0x54f157e0336a3822_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54F157E0336A3822u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54F157E0336A3822u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x54f157e0336a3822_raw(
        ped: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54F157E0336A3822u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54F157E0336A3822u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn is_bullet_in_box_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            ownedByPlayer: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE0F6D7450D37351u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE0F6D7450D37351u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                ownedByPlayer
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_bullet_in_box_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        ownedByPlayer: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE0F6D7450D37351u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE0F6D7450D37351u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                ownedByPlayer
        )
    }
}

/// ```
aka "constrained"
```



pub fn _is_in_power_saving_mode_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x684A41975F077262u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x684A41975F077262u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_in_power_saving_mode_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x684A41975F077262u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x684A41975F077262u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn water_override_fade_out_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3C221ADDDE31A11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3C221ADDDE31A11u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn water_override_fade_out_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3C221ADDDE31A11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3C221ADDDE31A11u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Refer to [`SET_WEATHER_TYPE_NOW`](#_0x29B487C359E19889) for weather types.



pub fn get_prev_weather_type_hash_name_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x564B884A05EC45A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x564B884A05EC45A3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_prev_weather_type_hash_name_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x564B884A05EC45A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x564B884A05EC45A3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Used for hunting in singleplayer and for golfing in both sp and online. The [`GET_HEADING_FROM_VECTOR_2D`](#_0x2FFB6B224F4B2926) native can be used to get the wind heading from the direction.



pub fn get_wind_direction_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F400FEF721170DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F400FEF721170DAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_wind_direction_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F400FEF721170DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F400FEF721170DAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
This seems to edit the water wave, intensity around your current location.  
0.0f = Normal  
1.0f = So Calm and Smooth, a boat will stay still.  
3.0f = Really Intense.  
```



pub fn water_override_set_strength_safe(
        
        
            strength: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC54A08C85AE4D410u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC54A08C85AE4D410u64;
        
        let result = invoke_raw!(
            hash,
                strength
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn water_override_set_strength_raw(
        strength: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC54A08C85AE4D410u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC54A08C85AE4D410u64;

        invoke_raw_typed!(
            hash,
                strength
        )
    }
}

/// ## Parameters
*



pub fn water_override_fade_in_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8434F1DFF41D6E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8434F1DFF41D6E7u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn water_override_fade_in_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8434F1DFF41D6E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8434F1DFF41D6E7u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Activates riot mode. All NPCs are being hostile to each other (including player). Also the game will give weapons (pistols, smgs) to random NPCs.



pub fn set_riot_mode_enabled_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2587A48BC88DFADFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2587A48BC88DFADFu64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_riot_mode_enabled_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2587A48BC88DFADFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2587A48BC88DFADFu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Does nothing (it's a nullsub). Seems to be PS4 specific.

```
NativeDB Introduced: v2060
```



pub fn _0x916ca67d26fd1e37_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x916CA67D26FD1E37u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x916CA67D26FD1E37u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x916ca67d26fd1e37_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x916CA67D26FD1E37u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x916CA67D26FD1E37u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Determines whether there is a projectile within the specified coordinates. The coordinates form a rectangle.  
ownedByPlayer = only projectiles fired by the player will be detected.  
```



pub fn is_projectile_in_area_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            ownedByPlayer: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5270A8FBC098C3F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5270A8FBC098C3F8u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                ownedByPlayer
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_projectile_in_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        ownedByPlayer: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5270A8FBC098C3F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5270A8FBC098C3F8u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                ownedByPlayer
        )
    }
}

/// ## Return value



pub fn _get_cloud_hat_opacity_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20AC25E781AE4A84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20AC25E781AE4A84u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_cloud_hat_opacity_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20AC25E781AE4A84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20AC25E781AE4A84u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0xd10282b6e3751ba0_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD10282B6E3751BA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD10282B6E3751BA0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xd10282b6e3751ba0_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD10282B6E3751BA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD10282B6E3751BA0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
creates single lightning+thunder at random position  
```



pub fn force_lightning_flash_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6062E089251C898u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6062E089251C898u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_lightning_flash_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6062E089251C898u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6062E089251C898u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Determines the highest ground Z-coordinate directly below a specified 3D coordinate, excluding any objects at that point. Optionally, water can be considered as ground when determining the highest point.

```
NativeDB Added Parameter 6: BOOL ignoreDistToWaterLevelCheck - If set to true, the distance to the water level will be ignored when checking for water as ground. 
```

```
NativeDB Introduced: v505
```



pub fn get_ground_z_excluding_objects_for_3d_coord_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            groundZ: 
        , 
        
        
            waterAsGround: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E82F0F362881B29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E82F0F362881B29u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                groundZ, 
                waterAsGround
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ground_z_excluding_objects_for_3d_coord_raw(
        x: , 
        y: , 
        z: , 
        groundZ: , 
        waterAsGround: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E82F0F362881B29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E82F0F362881B29u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                groundZ, 
                waterAsGround
        )
    }
}

/// ```
NativeDB Introduced: 2545
```



pub fn is_steam_version_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A27B2B6282F7169u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A27B2B6282F7169u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_steam_version_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A27B2B6282F7169u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A27B2B6282F7169u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
ignoreVehicle - bypasses vehicle check of the local player (it will not open if you are in a vehicle and this is set to false)
```



pub fn set_save_menu_active_safe(
        
        
            ignoreVehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9BF75D28165FF77u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9BF75D28165FF77u64;
        
        let result = invoke_raw!(
            hash,
                ignoreVehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_save_menu_active_raw(
        ignoreVehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9BF75D28165FF77u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9BF75D28165FF77u64;

        invoke_raw_typed!(
            hash,
                ignoreVehicle
        )
    }
}

/// ## Parameters
*



pub fn is_bullet_in_area_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            ownedByPlayer: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F2023999AD51C1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F2023999AD51C1Fu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                ownedByPlayer
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_bullet_in_area_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        ownedByPlayer: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F2023999AD51C1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F2023999AD51C1Fu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                ownedByPlayer
        )
    }
}

/// SET_RANDOM_WEATHER_TYPE native function



pub fn set_random_weather_type_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B05F884CF7E8020u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B05F884CF7E8020u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_random_weather_type_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B05F884CF7E8020u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B05F884CF7E8020u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn start_save_struct_with_size_safe(
        
        
            p0: 
        , 
        
        
            size: 
        , 
        
        
            structName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF737600CDDBEADDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF737600CDDBEADDu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                size, 
                structName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_save_struct_with_size_raw(
        p0: , 
        size: , 
        structName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF737600CDDBEADDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF737600CDDBEADDu64;

        invoke_raw_typed!(
            hash,
                p0, 
                size, 
                structName
        )
    }
}

/// ```
Example: 		CLEAR_AREA_OF_VEHICLES(0, 0, 0, 10000, false, false, false, false, false);  
```

```
NativeDB Added Parameter 10: BOOL p9
```



pub fn clear_area_of_vehicles_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01C7B9B38428AEB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01C7B9B38428AEB6u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_area_of_vehicles_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01C7B9B38428AEB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01C7B9B38428AEB6u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8
        )
    }
}

/// ```
Begins with RESET_*. Next character in the name is either D or E.
```



pub fn _reset_benchmark_recording_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x437138B6A830166Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x437138B6A830166Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _reset_benchmark_recording_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x437138B6A830166Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x437138B6A830166Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_fire_ammo_this_frame_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11879CDD803D30F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11879CDD803D30F4u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_fire_ammo_this_frame_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11879CDD803D30F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11879CDD803D30F4u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0xa0ad167e4b39d9a2_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        , 
        
        
            p10: 
        , 
        
        
            p11: 
        , 
        
        
            p12: 
        , 
        
        
            p13: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0AD167E4B39D9A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0AD167E4B39D9A2u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9, 
                p10, 
                p11, 
                p12, 
                p13
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa0ad167e4b39d9a2_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: , 
        p10: , 
        p11: , 
        p12: , 
        p13: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0AD167E4B39D9A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0AD167E4B39D9A2u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9, 
                p10, 
                p11, 
                p12, 
                p13
        )
    }
}

/// The BOOL parameters that are documented have not been confirmed. They are just documented from what I've found during testing. They may not work as expected in all cases.



pub fn is_position_occupied_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            range: 
        , 
        
        
            p4: 
        , 
        
        
            checkVehicles: 
        , 
        
        
            checkPeds: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            ignoreEntity: 
        , 
        
        
            p10: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADCDE75E1C60F32Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADCDE75E1C60F32Du64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                range, 
                p4, 
                checkVehicles, 
                checkPeds, 
                p7, 
                p8, 
                ignoreEntity, 
                p10
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_position_occupied_raw(
        x: , 
        y: , 
        z: , 
        range: , 
        p4: , 
        checkVehicles: , 
        checkPeds: , 
        p7: , 
        p8: , 
        ignoreEntity: , 
        p10: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADCDE75E1C60F32Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADCDE75E1C60F32Du64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                range, 
                p4, 
                checkVehicles, 
                checkPeds, 
                p7, 
                p8, 
                ignoreEntity, 
                p10
        )
    }
}

/// ## Parameters
*



pub fn get_profile_setting_safe(
        
        
            profileSetting: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC488FF2356EA7791u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC488FF2356EA7791u64;
        
        let result = invoke_raw!(
            hash,
                profileSetting
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_profile_setting_raw(
        profileSetting: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC488FF2356EA7791u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC488FF2356EA7791u64;

        invoke_raw_typed!(
            hash,
                profileSetting
        )
    }
}

/// ## Parameters
*



pub fn using_mission_creator_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF14878FC50BEC6EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF14878FC50BEC6EEu64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn using_mission_creator_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF14878FC50BEC6EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF14878FC50BEC6EEu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x23227df0b2115469_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23227DF0B2115469u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23227DF0B2115469u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x23227df0b2115469_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23227DF0B2115469u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23227DF0B2115469u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Quits the game.



pub fn quit_game_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB6891F03362FB12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB6891F03362FB12u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn quit_game_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB6891F03362FB12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB6891F03362FB12u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
sets something to 1  
```



pub fn _0xe3d969d2785ffb5e_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3D969D2785FFB5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3D969D2785FFB5Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xe3d969d2785ffb5e_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3D969D2785FFB5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3D969D2785FFB5Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_index_of_current_level_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCBAD6729F7B1F4FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCBAD6729F7B1F4FCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_index_of_current_level_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCBAD6729F7B1F4FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCBAD6729F7B1F4FCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_dispatch_ideal_spawn_distance_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FE601A64180D423u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FE601A64180D423u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_dispatch_ideal_spawn_distance_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FE601A64180D423u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FE601A64180D423u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Disables the spawn point at the police house on the specified index.
policeIndex: The police house index.
toggle: true to enable the spawn point, false to disable.
- Nacorpio
```



pub fn disable_police_restart_safe(
        
        
            policeIndex: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23285DED6EBD7EA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23285DED6EBD7EA3u64;
        
        let result = invoke_raw!(
            hash,
                policeIndex, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_police_restart_raw(
        policeIndex: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23285DED6EBD7EA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23285DED6EBD7EA3u64;

        invoke_raw_typed!(
            hash,
                policeIndex, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn begin_replay_stats_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0E500246FF73D66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0E500246FF73D66u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_replay_stats_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0E500246FF73D66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0E500246FF73D66u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Determines whether there is a sniper bullet within the specified coordinates. The coordinates form an axis-aligned bounding box.  
```



pub fn is_sniper_bullet_in_area_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEFCF11B01287125u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEFCF11B01287125u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_sniper_bullet_in_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEFCF11B01287125u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEFCF11B01287125u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2
        )
    }
}

/// SCRIPT_RACE_SHUTDOWN native function



pub fn script_race_shutdown_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1FF6BF9A63E5757Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1FF6BF9A63E5757Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn script_race_shutdown_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1FF6BF9A63E5757Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1FF6BF9A63E5757Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
This function is hard-coded to always return 0.  
```



pub fn is_sniper_inverted_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61A23B7EDA9BDA24u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61A23B7EDA9BDA24u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_sniper_inverted_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61A23B7EDA9BDA24u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61A23B7EDA9BDA24u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Suppresses or enables a specific type of random event for the current frame.

```c
enum eEventType {
    RC_PED_STEAL_VEHICLE = 0,
    RC_PED_JAY_WALK_LIGHT = 1,
    RC_COP_PURSUE = 2,
    RC_COP_PURSUE_VEHICLE_FLEE_SPAWNED = 3,
    RC_COP_VEHICLE_DRIVING_FAST = 4,
    RC_COP_VEHICLE_DRIVING_SLOW = 5,
    RC_DRIVER_RECKLESS = 6,
    RC_DRIVER_PRO = 7,
    RC_PED_PURSUE_WHEN_HIT_BY_CAR = 8
}
```



pub fn supress_random_event_this_frame_safe(
        
        
            eventType: 
        , 
        
        
            enable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1EAE0A6E978894A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1EAE0A6E978894A2u64;
        
        let result = invoke_raw!(
            hash,
                eventType, 
                enable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn supress_random_event_this_frame_raw(
        eventType: , 
        enable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1EAE0A6E978894A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1EAE0A6E978894A2u64;

        invoke_raw_typed!(
            hash,
                eventType, 
                enable
        )
    }
}

/// ## Parameters
*



pub fn get_number_of_free_stacks_of_this_size_safe(
        
        
            stackSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEAD16FC8F9DFC0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEAD16FC8F9DFC0Fu64;
        
        let result = invoke_raw!(
            hash,
                stackSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_number_of_free_stacks_of_this_size_raw(
        stackSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEAD16FC8F9DFC0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEAD16FC8F9DFC0Fu64;

        invoke_raw_typed!(
            hash,
                stackSize
        )
    }
}

/// ```
Returns pointer to an empty string.
GET_C*
```



pub fn _get_global_char_buffer_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24DA7D7667FD7B09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24DA7D7667FD7B09u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_global_char_buffer_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24DA7D7667FD7B09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24DA7D7667FD7B09u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn does_pop_multiplier_area_exist_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1327E2FE9746BAEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1327E2FE9746BAEEu64;
        
        let result = invoke_raw!(
            hash,
                id
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_pop_multiplier_area_exist_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1327E2FE9746BAEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1327E2FE9746BAEEu64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _get_random_int_in_range_2_safe(
        
        
            startRange: 
        , 
        
        
            endRange: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2D49816A804D134u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2D49816A804D134u64;
        
        let result = invoke_raw!(
            hash,
                startRange, 
                endRange
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_random_int_in_range_2_raw(
        startRange: , 
        endRange: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2D49816A804D134u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2D49816A804D134u64;

        invoke_raw_typed!(
            hash,
                startRange, 
                endRange
        )
    }
}

/// ## Parameters
*



pub fn _set_beast_mode_active_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x438822C279B73B93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x438822C279B73B93u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_beast_mode_active_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x438822C279B73B93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x438822C279B73B93u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Return value
Hard-coded to always return 0.



pub fn is_xbox360_version_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6201B4DAF662A9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6201B4DAF662A9Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_xbox360_version_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6201B4DAF662A9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6201B4DAF662A9Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Normally, blips can only be removed by the script or resource that created them. However, this native function allows a script to bypass this logic and remove blips from any script.



pub fn set_this_script_can_remove_blips_created_by_any_script_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB98236CAAECEF897u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB98236CAAECEF897u64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_this_script_can_remove_blips_created_by_any_script_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB98236CAAECEF897u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB98236CAAECEF897u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0xba4b8d83bdc75551_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA4B8D83BDC75551u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA4B8D83BDC75551u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xba4b8d83bdc75551_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA4B8D83BDC75551u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA4B8D83BDC75551u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Another unknown label type...
MISC::_FAA457EF263E8763(a_0, "Thumb_label");
MISC::_FAA457EF263E8763(&a_0._f10, "Photo_label");
MISC::_FAA457EF263E8763(a_0, "GXTlabel");
MISC::_FAA457EF263E8763(&a_0._f21, "StringComp");
MISC::_FAA457EF263E8763(&a_0._f43, "SecondStringComp");
MISC::_FAA457EF263E8763(&a_0._f53, "ThirdStringComp");
MISC::_FAA457EF263E8763(&a_0._f32, "SenderStringComp");
MISC::_FAA457EF263E8763(&a_0._f726[v_1A/*16*/], &v_20); // where v_20 is "LastJobTL_0_1" thru "LastJobTL_2_1", gets saved in a struct called "LAST_JobGamer_TL"
MISC::_FAA457EF263E8763(&a_0._f4B, "PAID_PLAYER");
MISC::_FAA457EF263E8763(&a_0._f5B, "RADIO_STATION");
```



pub fn _0xfaa457ef263e8763_safe(
        
        
            p0: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAA457EF263E8763u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAA457EF263E8763u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xfaa457ef263e8763_raw(
        p0: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAA457EF263E8763u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAA457EF263E8763u64;

        invoke_raw_typed!(
            hash,
                p0, 
                name
        )
    }
}

/// ## Return value



pub fn get_allocated_stack_size_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B3CA62B1EF19B62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B3CA62B1EF19B62u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_allocated_stack_size_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B3CA62B1EF19B62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B3CA62B1EF19B62u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Sets a visually fake wanted level on the user interface. Used by Rockstar's scripts to "override" regular wanted levels and make custom ones while the real wanted level and multipliers are still in effect.
Max is 6, anything above this makes it just 6. Also the mini-map gets the red & blue flashing effect.
```



pub fn set_fake_wanted_level_safe(
        
        
            fakeWantedLevel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1454F2448DE30163u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1454F2448DE30163u64;
        
        let result = invoke_raw!(
            hash,
                fakeWantedLevel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_fake_wanted_level_raw(
        fakeWantedLevel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1454F2448DE30163u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1454F2448DE30163u64;

        invoke_raw_typed!(
            hash,
                fakeWantedLevel
        )
    }
}

/// ## Parameters
*



pub fn is_tennis_mode_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D5479D115290C3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D5479D115290C3Fu64;
        
        let result = invoke_raw!(
            hash,
                ped
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_tennis_mode_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D5479D115290C3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D5479D115290C3Fu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Make sure to call this from the correct thread if you're using multiple threads because all other threads except the one which is calling SET_GAME_PAUSED will be paused which means you will lose control and the game remains in paused mode until you exit GTA5.exe  
```



pub fn set_game_paused_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x577D1284D6873711u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x577D1284D6873711u64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_game_paused_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x577D1284D6873711u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x577D1284D6873711u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_stunt_jumps_can_trigger_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD79185689F8FD5DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD79185689F8FD5DFu64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_stunt_jumps_can_trigger_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD79185689F8FD5DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD79185689F8FD5DFu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn remove_dispatch_spawn_blocking_area_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x264AC28B01B353A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x264AC28B01B353A5u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_dispatch_spawn_blocking_area_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x264AC28B01B353A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x264AC28B01B353A5u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _add_dispatch_spawn_blocking_area_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D4259F1FEB81DA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D4259F1FEB81DA9u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                x2, 
                y2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _add_dispatch_spawn_blocking_area_raw(
        x1: , 
        y1: , 
        x2: , 
        y2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D4259F1FEB81DA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D4259F1FEB81DA9u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                x2, 
                y2
        )
    }
}

/// Records that a player has successfully passed a checkpoint during a scripted race in GTA:Online. This native should be used after initializing the race with [`SCRIPT_RACE_INIT`](#_0x0A60017F841A54F2).

```
NativeDB Introduced: v323
```



pub fn script_race_player_hit_checkpoint_safe(
        
        
            ped: 
        , 
        
        
            checkpoint: 
        , 
        
        
            lap: 
        , 
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1BB299305C3E8C13u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1BB299305C3E8C13u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                checkpoint, 
                lap, 
                time
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn script_race_player_hit_checkpoint_raw(
        ped: , 
        checkpoint: , 
        lap: , 
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1BB299305C3E8C13u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1BB299305C3E8C13u64;

        invoke_raw_typed!(
            hash,
                ped, 
                checkpoint, 
                lap, 
                time
        )
    }
}

/// ## Return value



pub fn get_game_timer_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CD27B0045628463u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CD27B0045628463u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_game_timer_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CD27B0045628463u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CD27B0045628463u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0xeb2104e905c6f2e9_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB2104E905C6F2E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB2104E905C6F2E9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xeb2104e905c6f2e9_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB2104E905C6F2E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB2104E905C6F2E9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn disable_stunt_jump_set_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5272EBEDD4747F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5272EBEDD4747F6u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_stunt_jump_set_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5272EBEDD4747F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5272EBEDD4747F6u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn water_override_set_oceanwaveminamplitude_safe(
        
        
            minAmplitude: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF751B16FB32ABC1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF751B16FB32ABC1Du64;
        
        let result = invoke_raw!(
            hash,
                minAmplitude
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn water_override_set_oceanwaveminamplitude_raw(
        minAmplitude: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF751B16FB32ABC1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF751B16FB32ABC1Du64;

        invoke_raw_typed!(
            hash,
                minAmplitude
        )
    }
}

/// RESET_DISPATCH_IDEAL_SPAWN_DISTANCE native function



pub fn reset_dispatch_ideal_spawn_distance_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77A84429DD9F0A15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77A84429DD9F0A15u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_dispatch_ideal_spawn_distance_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77A84429DD9F0A15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77A84429DD9F0A15u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn ignore_next_restart_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21FFB63D8C615361u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21FFB63D8C615361u64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ignore_next_restart_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21FFB63D8C615361u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21FFB63D8C615361u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Shows the screen which is visible before you redo a mission? The game will make a restoration point where you will cameback when the mission is over.



pub fn queue_mission_repeat_save_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44A0BDC559B35F6Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44A0BDC559B35F6Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn queue_mission_repeat_save_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44A0BDC559B35F6Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44A0BDC559B35F6Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn is_area_occupied_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        , 
        
        
            p10: 
        , 
        
        
            p11: 
        , 
        
        
            p12: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA61B4DF533DCB56Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA61B4DF533DCB56Eu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9, 
                p10, 
                p11, 
                p12
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_area_occupied_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: , 
        p10: , 
        p11: , 
        p12: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA61B4DF533DCB56Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA61B4DF533DCB56Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9, 
                p10, 
                p11, 
                p12
        )
    }
}

/// ```
Sets whether the game should fade out after the player is arrested.  
```



pub fn set_fade_out_after_arrest_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E0B4DC0D990A4E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E0B4DC0D990A4E7u64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_fade_out_after_arrest_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E0B4DC0D990A4E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E0B4DC0D990A4E7u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value



pub fn get_snow_level_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5868A966E5BE3AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5868A966E5BE3AEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_snow_level_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5868A966E5BE3AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5868A966E5BE3AEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 8: Any p7
NativeDB Added Parameter 9: Any p8
```



pub fn create_incident_safe(
        
        
            dispatchService: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            numUnits: 
        , 
        
        
            radius: 
        , 
        
        
            outIncidentID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F892CAF67444AE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F892CAF67444AE7u64;
        
        let result = invoke_raw!(
            hash,
                dispatchService, 
                x, 
                y, 
                z, 
                numUnits, 
                radius, 
                outIncidentID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_incident_raw(
        dispatchService: , 
        x: , 
        y: , 
        z: , 
        numUnits: , 
        radius: , 
        outIncidentID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F892CAF67444AE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F892CAF67444AE7u64;

        invoke_raw_typed!(
            hash,
                dispatchService, 
                x, 
                y, 
                z, 
                numUnits, 
                radius, 
                outIncidentID
        )
    }
}

/// ## Return value



pub fn is_minigame_in_progress_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B4A15E44DE0F478u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B4A15E44DE0F478u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_minigame_in_progress_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B4A15E44DE0F478u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B4A15E44DE0F478u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_stunt_jump_message_showing_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2272B0A1343129F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2272B0A1343129F4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_stunt_jump_message_showing_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2272B0A1343129F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2272B0A1343129F4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_memory_card_in_use_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A75CE2956274ADDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A75CE2956274ADDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_memory_card_in_use_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A75CE2956274ADDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A75CE2956274ADDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
SET_*
```



pub fn _0xe532ec1a63231b4f_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE532EC1A63231B4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE532EC1A63231B4Fu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xe532ec1a63231b4f_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE532EC1A63231B4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE532EC1A63231B4Fu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Immediately changes the game's weather to the specified type, which will then persist for one cycle before the game resumes its natural weather progression.



pub fn set_weather_type_now_safe(
        
        
            weatherType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29B487C359E19889u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29B487C359E19889u64;
        
        let result = invoke_raw!(
            hash,
                weatherType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_weather_type_now_raw(
        weatherType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29B487C359E19889u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29B487C359E19889u64;

        invoke_raw_typed!(
            hash,
                weatherType
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0x1178e104409fe58c_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1178E104409FE58Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1178E104409FE58Cu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x1178e104409fe58c_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1178E104409FE58Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1178E104409FE58Cu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Delete an incident with a given id.  
=======================================================  
Correction, I have change this to int, instead of int*  
as it doesn't use a pointer to the createdIncident.  
If you try it you will crash (or) freeze.  
=======================================================  
```



pub fn delete_incident_safe(
        
        
            incidentId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x556C1AA270D5A207u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x556C1AA270D5A207u64;
        
        let result = invoke_raw!(
            hash,
                incidentId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn delete_incident_raw(
        incidentId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x556C1AA270D5A207u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x556C1AA270D5A207u64;

        invoke_raw_typed!(
            hash,
                incidentId
        )
    }
}

/// Initializes a script race in GTA:Online and sets up the helper split time system.

```
NativeDB Introduced: v323
```



pub fn script_race_init_safe(
        
        
            numCheckpoints: 
        , 
        
        
            numLaps: 
        , 
        
        
            numPlayers: 
        , 
        
        
            localPlayer: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A60017F841A54F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A60017F841A54F2u64;
        
        let result = invoke_raw!(
            hash,
                numCheckpoints, 
                numLaps, 
                numPlayers, 
                localPlayer
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn script_race_init_raw(
        numCheckpoints: , 
        numLaps: , 
        numPlayers: , 
        localPlayer: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A60017F841A54F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A60017F841A54F2u64;

        invoke_raw_typed!(
            hash,
                numCheckpoints, 
                numLaps, 
                numPlayers, 
                localPlayer
        )
    }
}

/// ## Parameters
*



pub fn set_random_seed_safe(
        
        
            seed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x444D98F98C11F3ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x444D98F98C11F3ECu64;
        
        let result = invoke_raw!(
            hash,
                seed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_random_seed_raw(
        seed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x444D98F98C11F3ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x444D98F98C11F3ECu64;

        invoke_raw_typed!(
            hash,
                seed
        )
    }
}

/// ## Parameters
*



pub fn asin_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC843060B5765DCE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC843060B5765DCE7u64;
        
        let result = invoke_raw!(
            hash,
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn asin_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC843060B5765DCE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC843060B5765DCE7u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ```
Returns value of the '-benchmarkPass' command line option.
```



pub fn _get_benchmark_pass_from_command_line_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B2366C3F2A5C8DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B2366C3F2A5C8DFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_benchmark_pass_from_command_line_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B2366C3F2A5C8DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B2366C3F2A5C8DFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn override_save_house_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1162EA8AE9D24EEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1162EA8AE9D24EEAu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn override_save_house_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1162EA8AE9D24EEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1162EA8AE9D24EEAu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7
        )
    }
}

/// ## Parameters
*



pub fn _0x7f8f6405f4777af6_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F8F6405F4777AF6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F8F6405F4777AF6u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7f8f6405f4777af6_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F8F6405F4777AF6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F8F6405F4777AF6u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9
        )
    }
}

/// ```
Returns NULL unless UPDATE_ONSCREEN_KEYBOARD() returns 1 in the same tick.  
```



pub fn get_onscreen_keyboard_result_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8362B09B91893647u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8362B09B91893647u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_onscreen_keyboard_result_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8362B09B91893647u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8362B09B91893647u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _CLEAR_TACTICAL_ANALYSIS_POINTS native function



pub fn _clear_tactical_analysis_points_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3CD58CCA6CDA852u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3CD58CCA6CDA852u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _clear_tactical_analysis_points_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3CD58CCA6CDA852u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3CD58CCA6CDA852u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Refer to [`SET_WEATHER_TYPE_NOW`](#_0x29B487C359E19889) for weather types.



pub fn is_prev_weather_type_safe(
        
        
            weatherType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44F28F86433B10A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44F28F86433B10A9u64;
        
        let result = invoke_raw!(
            hash,
                weatherType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_prev_weather_type_raw(
        weatherType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44F28F86433B10A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44F28F86433B10A9u64;

        invoke_raw_typed!(
            hash,
                weatherType
        )
    }
}

/// ## Parameters
*



pub fn has_bullet_impacted_in_box_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC8C5D7CFEAB8394u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC8C5D7CFEAB8394u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_bullet_impacted_in_box_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC8C5D7CFEAB8394u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC8C5D7CFEAB8394u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7
        )
    }
}

/// ```
If true, the player can't save the game.   
If the parameter is true, sets the mission flag to true, if the parameter is false, the function does nothing at all.  
^ also, if the mission flag is already set, the function does nothing at all  
```



pub fn set_mission_flag_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4301E5121A0ED73u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4301E5121A0ED73u64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mission_flag_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4301E5121A0ED73u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4301E5121A0ED73u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x703cc7f60cbb2b57_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x703CC7F60CBB2B57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x703CC7F60CBB2B57u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x703cc7f60cbb2b57_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x703CC7F60CBB2B57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x703CC7F60CBB2B57u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_dispatch_time_between_spawn_attempts_multiplier_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48838ED9937A15D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48838ED9937A15D1u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_dispatch_time_between_spawn_attempts_multiplier_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48838ED9937A15D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48838ED9937A15D1u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Makes the ped jump around like they're in a tennis match  
```



pub fn enable_tennis_mode_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28A04B411933F8A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28A04B411933F8A6u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                toggle, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn enable_tennis_mode_raw(
        ped: , 
        toggle: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28A04B411933F8A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28A04B411933F8A6u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle, 
                p2
        )
    }
}

/// Gets the high precision frame time of the last frame in seconds.

_note: the example above is way less precise._



pub fn get_frame_time_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15C40837039FFAF7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15C40837039FFAF7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_frame_time_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15C40837039FFAF7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15C40837039FFAF7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Hardcoded to return false.
```



pub fn _has_resumed_from_suspend_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8B9C0EC9E183F35u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8B9C0EC9E183F35u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _has_resumed_from_suspend_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8B9C0EC9E183F35u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8B9C0EC9E183F35u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _register_int64_to_save_safe(
        
        
            p0: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA735353C77334EA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA735353C77334EA0u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _register_int64_to_save_raw(
        p0: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA735353C77334EA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA735353C77334EA0u64;

        invoke_raw_typed!(
            hash,
                p0, 
                name
        )
    }
}

/// ```
Returns bit's boolean state from [offset] of [address].
Example:
MISC::IS_BIT_SET(bitAddress, 1);
To enable and disable bits, see:
MISC::SET_BIT(&bitAddress, 1);   // enable
MISC::CLEAR_BIT(&bitAddress, 1); // disable
```



pub fn is_bit_set_safe(
        
        
            address: 
        , 
        
        
            offset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA921AA820C25702Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA921AA820C25702Fu64;
        
        let result = invoke_raw!(
            hash,
                address, 
                offset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_bit_set_raw(
        address: , 
        offset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA921AA820C25702Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA921AA820C25702Fu64;

        invoke_raw_typed!(
            hash,
                address, 
                offset
        )
    }
}

/// Refer to [`SET_WEATHER_TYPE_NOW`](#_0x29B487C359E19889) for weather types.



pub fn set_weather_type_now_persist_safe(
        
        
            weatherType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED712CA327900C8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED712CA327900C8Au64;
        
        let result = invoke_raw!(
            hash,
                weatherType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_weather_type_now_persist_raw(
        weatherType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED712CA327900C8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED712CA327900C8Au64;

        invoke_raw_typed!(
            hash,
                weatherType
        )
    }
}

/// ## Parameters
*



pub fn is_string_null_safe(
        
        
            string: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF22B6C47C6EAB066u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF22B6C47C6EAB066u64;
        
        let result = invoke_raw!(
            hash,
                string
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_string_null_raw(
        string: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF22B6C47C6EAB066u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF22B6C47C6EAB066u64;

        invoke_raw_typed!(
            hash,
                string
        )
    }
}

/// ## Parameters
*



pub fn get_angle_between_2d_vectors_safe(
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x186FC4BE848E1C92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x186FC4BE848E1C92u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                x2, 
                y2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_angle_between_2d_vectors_raw(
        x1: , 
        y1: , 
        x2: , 
        y2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x186FC4BE848E1C92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x186FC4BE848E1C92u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                x2, 
                y2
        )
    }
}

/// CLEAR_REPLAY_STATS native function



pub fn clear_replay_stats_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B1AB132A16FDA55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B1AB132A16FDA55u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_replay_stats_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B1AB132A16FDA55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B1AB132A16FDA55u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _reset_dispatch_spawn_location_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5896F2BD5683A4E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5896F2BD5683A4E1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _reset_dispatch_spawn_location_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5896F2BD5683A4E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5896F2BD5683A4E1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn tan_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x632106CC96E82E91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x632106CC96E82E91u64;
        
        let result = invoke_raw!(
            hash,
                p0
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn tan_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x632106CC96E82E91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x632106CC96E82E91u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Only found 2 times in decompiled scripts. Not a whole lot to go off of.
MISC::_8269816F6CFD40F8(&a_0._f1F5A._f6[0/*8*/], "TEMPSTAT_LABEL"); // gets saved in a struct called "g_SaveData_STRING_ScriptSaves"
MISC::_8269816F6CFD40F8(&a_0._f4B4[v_1A/*8*/], &v_5); // where v_5 is "Name0" thru "Name9", gets saved in a struct called "OUTFIT_Name"
```



pub fn _0x8269816f6cfd40f8_safe(
        
        
            p0: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8269816F6CFD40F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8269816F6CFD40F8u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8269816f6cfd40f8_raw(
        p0: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8269816F6CFD40F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8269816F6CFD40F8u64;

        invoke_raw_typed!(
            hash,
                p0, 
                name
        )
    }
}

/// ## Parameters
*



pub fn does_pop_multiplier_sphere_exist_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x171BAFB3C60389F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x171BAFB3C60389F4u64;
        
        let result = invoke_raw!(
            hash,
                id
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_pop_multiplier_sphere_exist_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x171BAFB3C60389F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x171BAFB3C60389F4u64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// CLEAR_OVERRIDE_WEATHER native function



pub fn clear_override_weather_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x338D2E3477711050u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x338D2E3477711050u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_override_weather_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x338D2E3477711050u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x338D2E3477711050u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Allows the player to perform super jumps. This function must be called every frame for it to work.
It basically OR's a flag for a single frame, allowing the ped to perform a super jump only when the flag is set.



pub fn set_super_jump_this_frame_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57FFF03E423A4C0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57FFF03E423A4C0Bu64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_super_jump_this_frame_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57FFF03E423A4C0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57FFF03E423A4C0Bu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn atan2_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8927CBF9D22261A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8927CBF9D22261A4u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn atan2_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8927CBF9D22261A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8927CBF9D22261A4u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Sets whether the game should fade in after the player dies or is arrested.  
```



pub fn set_fade_in_after_death_arrest_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA66D2796BA33F12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA66D2796BA33F12u64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_fade_in_after_death_arrest_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA66D2796BA33F12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA66D2796BA33F12u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Begins with STOP_*. Next character in the name is either D or E.
```



pub fn _stop_benchmark_recording_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7DB36C24634F52Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7DB36C24634F52Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _stop_benchmark_recording_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7DB36C24634F52Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7DB36C24634F52Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn water_override_set_rippleminbumpiness_safe(
        
        
            minBumpiness: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6216B116083A7CB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6216B116083A7CB4u64;
        
        let result = invoke_raw!(
            hash,
                minBumpiness
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn water_override_set_rippleminbumpiness_raw(
        minBumpiness: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6216B116083A7CB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6216B116083A7CB4u64;

        invoke_raw_typed!(
            hash,
                minBumpiness
        )
    }
}

/// ## Return value



pub fn get_fake_wanted_level_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C9296CBCD1B971Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C9296CBCD1B971Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_fake_wanted_level_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C9296CBCD1B971Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C9296CBCD1B971Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn water_override_set_shorewaveminamplitude_safe(
        
        
            minAmplitude: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3EAD29AB273ECE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3EAD29AB273ECE8u64;
        
        let result = invoke_raw!(
            hash,
                minAmplitude
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn water_override_set_shorewaveminamplitude_raw(
        minAmplitude: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3EAD29AB273ECE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3EAD29AB273ECE8u64;

        invoke_raw_typed!(
            hash,
                minAmplitude
        )
    }
}

/// ```
Sets whether the game should fade out after the player dies.  
```



pub fn set_fade_out_after_death_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A18E01DF2C87B86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A18E01DF2C87B86u64;
        
        let result = invoke_raw!(
            hash,
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_fade_out_after_death_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A18E01DF2C87B86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A18E01DF2C87B86u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

