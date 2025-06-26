//! RECORDING native functions
//! 
//! Functions for the recording category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// Stops recording and saves the recorded clip.



pub fn _stop_recording_and_save_clip_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x071A5197D6AFC8B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x071A5197D6AFC8B3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _stop_recording_and_save_clip_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x071A5197D6AFC8B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x071A5197D6AFC8B3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Stops recording and discards the recorded clip.



pub fn _stop_recording_and_discard_clip_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88BB3507ED41A240u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88BB3507ED41A240u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _stop_recording_and_discard_clip_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88BB3507ED41A240u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88BB3507ED41A240u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0xF854439EFBB3B583 native function



pub fn _0xf854439efbb3b583_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF854439EFBB3B583u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF854439EFBB3B583u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf854439efbb3b583_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF854439EFBB3B583u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF854439EFBB3B583u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
-This function appears to be deprecated/ unused. Tracing the call internally leads to a _nullsub -  
first one seems to be a string of a mission name, second one seems to be a bool/toggle  
p1 was always 0.  
```



pub fn _0x208784099002bc30_safe(
        
        
            missionNameLabel: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x208784099002BC30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x208784099002BC30u64;
        
        let result = invoke_raw!(
            hash,
                missionNameLabel, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x208784099002bc30_raw(
        missionNameLabel: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x208784099002BC30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x208784099002BC30u64;

        invoke_raw_typed!(
            hash,
                missionNameLabel, 
                p1
        )
    }
}

/// Checks if you're recording.



pub fn _is_recording_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1897CA71995A90B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1897CA71995A90B4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_recording_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1897CA71995A90B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1897CA71995A90B4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x293220da1b46cebc_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x293220DA1B46CEBCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x293220DA1B46CEBCu64;
        
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
pub fn _0x293220da1b46cebc_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x293220DA1B46CEBCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x293220DA1B46CEBCu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x66972397e0757e7a_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66972397E0757E7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66972397E0757E7Au64;
        
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
pub fn _0x66972397e0757e7a_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66972397E0757E7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66972397E0757E7Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn _0x48621c9fca3ebd28_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48621C9FCA3EBD28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48621C9FCA3EBD28u64;
        
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
pub fn _0x48621c9fca3ebd28_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48621C9FCA3EBD28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48621C9FCA3EBD28u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _0xdf4b952f7d381b95_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF4B952F7D381B95u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF4B952F7D381B95u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xdf4b952f7d381b95_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF4B952F7D381B95u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF4B952F7D381B95u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _save_recording_clip_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x644546EC5287471Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x644546EC5287471Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _save_recording_clip_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x644546EC5287471Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x644546EC5287471Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x81CBAE94390F9F89 native function



pub fn _0x81cbae94390f9f89_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x81CBAE94390F9F89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x81CBAE94390F9F89u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x81cbae94390f9f89_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x81CBAE94390F9F89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x81CBAE94390F9F89u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Disable recording for this frame only.



pub fn _stop_recording_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB2D525B57F42B40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB2D525B57F42B40u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _stop_recording_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB2D525B57F42B40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB2D525B57F42B40u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// This will disable the ability to make camera changes in R* Editor.



pub fn _disable_rockstar_editor_camera_changes_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF66DCEE6609B148u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF66DCEE6609B148u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _disable_rockstar_editor_camera_changes_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF66DCEE6609B148u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF66DCEE6609B148u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x4282e08174868be3_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4282E08174868BE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4282E08174868BE3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4282e08174868be3_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4282E08174868BE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4282E08174868BE3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x13B350B8AD0EEE10 native function



pub fn _0x13b350b8ad0eee10_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13B350B8AD0EEE10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13B350B8AD0EEE10u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x13b350b8ad0eee10_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13B350B8AD0EEE10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13B350B8AD0EEE10u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Starts recording a replay.  
If already recording a replay, does nothing.



pub fn _start_recording_safe(
        
        
            mode: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3AC2FFF9612AC81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3AC2FFF9612AC81u64;
        
        let result = invoke_raw!(
            hash,
                mode
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _start_recording_raw(
        mode: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3AC2FFF9612AC81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3AC2FFF9612AC81u64;

        invoke_raw_typed!(
            hash,
                mode
        )
    }
}

/// ## Parameters
*



pub fn _0x33d47e85b476abcd_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33D47E85B476ABCDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33D47E85B476ABCDu64;
        
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
pub fn _0x33d47e85b476abcd_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33D47E85B476ABCDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33D47E85B476ABCDu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

