//! WATER native functions
//! 
//! Functions for the water category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn _0x547237aa71ab44de_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x547237AA71AB44DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x547237AA71AB44DEu64;
        
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
pub fn _0x547237aa71ab44de_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x547237AA71AB44DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x547237AA71AB44DEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Sets the water height for a given position and radius.  
```



pub fn modify_water_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            height: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC443FD757C3BA637u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC443FD757C3BA637u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                height, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn modify_water_raw(
        x: , 
        y: , 
        height: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC443FD757C3BA637u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC443FD757C3BA637u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                height, 
                radius
        )
    }
}

/// ```
Gets the aggressiveness factor of the ocean waves.  
```



pub fn get_deep_ocean_scaler_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B2A2CC86778B619u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B2A2CC86778B619u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_deep_ocean_scaler_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B2A2CC86778B619u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B2A2CC86778B619u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Sets a value that determines how aggressive the ocean waves will be. Values of 2.0 or more make for very aggressive waves like you see during a thunderstorm.  
Works only ~200 meters around the player.  
```



pub fn set_deep_ocean_scaler_safe(
        
        
            intensity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB96B00E976BE977Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB96B00E976BE977Fu64;
        
        let result = invoke_raw!(
            hash,
                intensity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_deep_ocean_scaler_raw(
        intensity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB96B00E976BE977Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB96B00E976BE977Fu64;

        invoke_raw_typed!(
            hash,
                intensity
        )
    }
}

/// Only 8 current rises can exist. If rises need to be changed, use REMOVE_EXTRA_CALMING_QUAD and then ADD_EXTRA_CALMING_QUAD again.
After removing a rise, you will be able to add a rise again.



pub fn add_extra_calming_quad_safe(
        
        
            xLow: 
        , 
        
        
            yLow: 
        , 
        
        
            xHigh: 
        , 
        
        
            yHigh: 
        , 
        
        
            height: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDBF4CDBC07E1706u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDBF4CDBC07E1706u64;
        
        let result = invoke_raw!(
            hash,
                xLow, 
                yLow, 
                xHigh, 
                yHigh, 
                height
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_extra_calming_quad_raw(
        xLow: , 
        yLow: , 
        xHigh: , 
        yHigh: , 
        height: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDBF4CDBC07E1706u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDBF4CDBC07E1706u64;

        invoke_raw_typed!(
            hash,
                xLow, 
                yLow, 
                xHigh, 
                yHigh, 
                height
        )
    }
}

/// Retrieves the depth of the water beneath the specified position, disregarding wave effects.



pub fn get_water_height_no_waves_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            height: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EE6B53CE13A9794u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EE6B53CE13A9794u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                height
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_water_height_no_waves_raw(
        x: , 
        y: , 
        z: , 
        height: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EE6B53CE13A9794u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EE6B53CE13A9794u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                height
        )
    }
}

/// ## Parameters
*



pub fn test_probe_against_water_safe(
        
        
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
        
        
            result: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFFA5D878809819DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFFA5D878809819DBu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                result
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn test_probe_against_water_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        result: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFFA5D878809819DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFFA5D878809819DBu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                result
        )
    }
}

/// ## Parameters
*



pub fn test_vertical_probe_against_all_water_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            flag: 
        , 
        
        
            height: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B3451FA1E3142E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B3451FA1E3142E2u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                flag, 
                height
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn test_vertical_probe_against_all_water_raw(
        x: , 
        y: , 
        z: , 
        flag: , 
        height: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B3451FA1E3142E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B3451FA1E3142E2u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                flag, 
                height
        )
    }
}

/// Retrieves the depth of the water beneath the specified position, accounting for the waves.



pub fn get_water_height_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            height: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6829842C06AE524u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6829842C06AE524u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                height
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_water_height_raw(
        x: , 
        y: , 
        z: , 
        height: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6829842C06AE524u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6829842C06AE524u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                height
        )
    }
}

/// ```
Sets the waves intensity back to original (1.0 in most cases). 
```



pub fn reset_deep_ocean_scaler_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E5E99285AE812DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E5E99285AE812DBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_deep_ocean_scaler_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E5E99285AE812DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E5E99285AE812DBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p0 is the handle returned from _0xFDBF4CDBC07E1706  
```



pub fn _remove_current_rise_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1252E3E59A82AAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1252E3E59A82AAFu64;
        
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
pub fn _remove_current_rise_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1252E3E59A82AAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1252E3E59A82AAFu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Flags are identical to START_SHAPE_TEST*, however, 128 is automatically set.



pub fn test_probe_against_all_water_safe(
        
        
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
        
        
            flag: 
        , 
        
        
            result: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8974647ED222EA5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8974647ED222EA5Fu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                flag, 
                result
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn test_probe_against_all_water_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        flag: , 
        result: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8974647ED222EA5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8974647ED222EA5Fu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                flag, 
                result
        )
    }
}

