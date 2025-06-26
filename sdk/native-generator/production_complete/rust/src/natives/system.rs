//! SYSTEM native functions
//! 
//! Functions for the system category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
Calculates the magnitude of a vector.  
```



pub fn vmag_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x652D2EEEF1D3E62Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x652D2EEEF1D3E62Cu64;
        
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
pub fn vmag_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x652D2EEEF1D3E62Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x652D2EEEF1D3E62Cu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ```
0 = high
1 = normal
2 = low
```



pub fn _set_thread_priority_safe(
        
        
            priority: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42B65DEEF2EDF2A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42B65DEEF2EDF2A1u64;
        
        let result = invoke_raw!(
            hash,
                priority
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_thread_priority_raw(
        priority: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42B65DEEF2EDF2A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42B65DEEF2EDF2A1u64;

        invoke_raw_typed!(
            hash,
                priority
        )
    }
}

/// Sets the value for the timer A in milliseconds



pub fn settimera_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1B1E9A034A63A62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1B1E9A034A63A62u64;
        
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
pub fn settimera_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1B1E9A034A63A62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1B1E9A034A63A62u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// Left bit shifts a value.
It is advised you use the `<<` operator instead of this native. It does the same and is faster.



pub fn shift_left_safe(
        
        
            value: 
        , 
        
        
            bitShift: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDD95A39E5544DE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDD95A39E5544DE8u64;
        
        let result = invoke_raw!(
            hash,
                value, 
                bitShift
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn shift_left_raw(
        value: , 
        bitShift: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDD95A39E5544DE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDD95A39E5544DE8u64;

        invoke_raw_typed!(
            hash,
                value, 
                bitShift
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _log10_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE816E655DE37FE20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE816E655DE37FE20u64;
        
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
pub fn _log10_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE816E655DE37FE20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE816E655DE37FE20u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Parameters
*



pub fn to_float_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBDA792448DB5A89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBDA792448DB5A89u64;
        
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
pub fn to_float_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBDA792448DB5A89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBDA792448DB5A89u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// Calculates the distance between two points in 3D space. For performance reasons, consider using direct mathematical calculations for distance, as they can be more efficient than calling this native function.

```
NativeDB Introduced: v323
```



pub fn vdist_safe(
        
        
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
        let hash = 0x2A488C176D52CCA5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A488C176D52CCA5u64;
        
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
pub fn vdist_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A488C176D52CCA5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A488C176D52CCA5u64;

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

/// Calculates distance between vectors but does not perform Sqrt operations. Its way faster than [`VDIST`](#_0x2A488C176D52CCA5), but it's not faster than direct mathematical calculations.

```
NativeDB Introduced: v323
```



pub fn vdist2_safe(
        
        
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
        let hash = 0xB7A628320EFF8E47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7A628320EFF8E47u64;
        
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
pub fn vdist2_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7A628320EFF8E47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7A628320EFF8E47u64;

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

/// ```
Counts up. Every 1000 is 1 real-time second. Use SETTIMERA(int value) to set the timer (e.g.: SETTIMERA(0)).  
```



pub fn timera_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83666F9FB8FEBD4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83666F9FB8FEBD4Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn timera_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83666F9FB8FEBD4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83666F9FB8FEBD4Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn round_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2DB717A73826179u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2DB717A73826179u64;
        
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
pub fn round_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2DB717A73826179u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2DB717A73826179u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ```
Gets the current frame time.  
```



pub fn timestep_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0000000050597EE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0000000050597EE2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn timestep_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0000000050597EE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0000000050597EE2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Returns the sine of the given number.



pub fn sin_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BADBFA3B172435Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BADBFA3B172435Fu64;
        
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
pub fn sin_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BADBFA3B172435Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BADBFA3B172435Fu64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Parameters
*



pub fn start_new_script_with_name_hash_and_args_safe(
        
        
            scriptHash: 
        , 
        
        
            args: 
        , 
        
        
            argCount: 
        , 
        
        
            stackSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4BB298BD441BE78u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4BB298BD441BE78u64;
        
        let result = invoke_raw!(
            hash,
                scriptHash, 
                args, 
                argCount, 
                stackSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_new_script_with_name_hash_and_args_raw(
        scriptHash: , 
        args: , 
        argCount: , 
        stackSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4BB298BD441BE78u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4BB298BD441BE78u64;

        invoke_raw_typed!(
            hash,
                scriptHash, 
                args, 
                argCount, 
                stackSize
        )
    }
}

/// ```
Examples:
 g_384A = SYSTEM::START_NEW_SCRIPT("cellphone_flashhand", 1424);
 l_10D = SYSTEM::START_NEW_SCRIPT("taxiService", 1828);
 SYSTEM::START_NEW_SCRIPT("AM_MP_YACHT", 5000);
 SYSTEM::START_NEW_SCRIPT("emergencycall", 512);
 SYSTEM::START_NEW_SCRIPT("emergencycall", 512);
 SYSTEM::START_NEW_SCRIPT("FM_maintain_cloud_header_data", 1424);
 SYSTEM::START_NEW_SCRIPT("FM_Mission_Controller", 31000);
 SYSTEM::START_NEW_SCRIPT("tennis_family", 3650);
 SYSTEM::START_NEW_SCRIPT("Celebrations", 3650);
Decompiled examples of usage when starting a script:

    SCRIPT::REQUEST_SCRIPT(a_0);
    if (SCRIPT::HAS_SCRIPT_LOADED(a_0)) {
        SYSTEM::START_NEW_SCRIPT(a_0, v_3);
        SCRIPT::SET_SCRIPT_AS_NO_LONGER_NEEDED(a_0);
        return 1;
    }

or:
    v_2 = "MrsPhilips2";
    SCRIPT::REQUEST_SCRIPT(v_2);
    while (!SCRIPT::HAS_SCRIPT_LOADED(v_2)) {
    SCRIPT::REQUEST_SCRIPT(v_2);
    SYSTEM::WAIT(0);
    }
    sub_8792(36);
    SYSTEM::START_NEW_SCRIPT(v_2, 17000);
    SCRIPT::SET_SCRIPT_AS_NO_LONGER_NEEDED(v_2);
All native script names: pastebin.com/K9adDsu4 and pastebin.com/yLNWicUi
```



pub fn start_new_script_safe(
        
        
            scriptName: 
        , 
        
        
            stackSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE81651AD79516E48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE81651AD79516E48u64;
        
        let result = invoke_raw!(
            hash,
                scriptName, 
                stackSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_new_script_raw(
        scriptName: , 
        stackSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE81651AD79516E48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE81651AD79516E48u64;

        invoke_raw_typed!(
            hash,
                scriptName, 
                stackSize
        )
    }
}

/// ```
Calculates the magnitude of a vector but does not perform Sqrt operations. (Its way faster)  
```



pub fn vmag2_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8CEACB4F35AE058u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8CEACB4F35AE058u64;
        
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
pub fn vmag2_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8CEACB4F35AE058u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8CEACB4F35AE058u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn sqrt_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71D93B57D07F9804u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71D93B57D07F9804u64;
        
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
pub fn sqrt_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71D93B57D07F9804u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71D93B57D07F9804u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Parameters
*



pub fn pow_safe(
        
        
            base: 
        , 
        
        
            exponent: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3621CC40F31FE2Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3621CC40F31FE2Eu64;
        
        let result = invoke_raw!(
            hash,
                base, 
                exponent
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pow_raw(
        base: , 
        exponent: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3621CC40F31FE2Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3621CC40F31FE2Eu64;

        invoke_raw_typed!(
            hash,
                base, 
                exponent
        )
    }
}

/// ## Parameters
*



pub fn floor_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF34EE736CF047844u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF34EE736CF047844u64;
        
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
pub fn floor_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF34EE736CF047844u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF34EE736CF047844u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Return value



pub fn timerb_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9D9444186B5A374u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9D9444186B5A374u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn timerb_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9D9444186B5A374u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9D9444186B5A374u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
I'm guessing this rounds a float value up to the next whole number, and FLOOR rounds it down  
```



pub fn ceil_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11E019C8F43ACC8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11E019C8F43ACC8Au64;
        
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
pub fn ceil_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11E019C8F43ACC8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11E019C8F43ACC8Au64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Parameters
*



pub fn start_new_script_with_name_hash_safe(
        
        
            scriptHash: 
        , 
        
        
            stackSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB1C67C3A5333A92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB1C67C3A5333A92u64;
        
        let result = invoke_raw!(
            hash,
                scriptHash, 
                stackSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_new_script_with_name_hash_raw(
        scriptHash: , 
        stackSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB1C67C3A5333A92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB1C67C3A5333A92u64;

        invoke_raw_typed!(
            hash,
                scriptHash, 
                stackSize
        )
    }
}

/// ```
return : script thread id, 0 if failed  
Pass pointer to struct of args in p1, size of struct goes into p2  
```



pub fn start_new_script_with_args_safe(
        
        
            scriptName: 
        , 
        
        
            args: 
        , 
        
        
            argCount: 
        , 
        
        
            stackSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8BA7F44DF1575E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8BA7F44DF1575E1u64;
        
        let result = invoke_raw!(
            hash,
                scriptName, 
                args, 
                argCount, 
                stackSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_new_script_with_args_raw(
        scriptName: , 
        args: , 
        argCount: , 
        stackSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8BA7F44DF1575E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8BA7F44DF1575E1u64;

        invoke_raw_typed!(
            hash,
                scriptName, 
                args, 
                argCount, 
                stackSize
        )
    }
}

/// ```
Pauses execution of the current script, please note this behavior is only seen when called from one of the game script files(ysc). In order to wait an asi script use "static void WAIT(DWORD time);" found in main.h
```



pub fn wait_safe(
        
        
            ms: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4EDE34FBADD967A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4EDE34FBADD967A6u64;
        
        let result = invoke_raw!(
            hash,
                ms
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn wait_raw(
        ms: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4EDE34FBADD967A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4EDE34FBADD967A6u64;

        invoke_raw_typed!(
            hash,
                ms
        )
    }
}

/// Sets the value for the timer B in milliseconds



pub fn settimerb_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AE11BC36633DE4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AE11BC36633DE4Eu64;
        
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
pub fn settimerb_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AE11BC36633DE4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AE11BC36633DE4Eu64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// Returns the cosine of the given number.



pub fn cos_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0FFB162F40A139Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0FFB162F40A139Cu64;
        
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
pub fn cos_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0FFB162F40A139Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0FFB162F40A139Cu64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// Right bit shifts a value.
It is advised you use the `>>` operator instead of this native. It does the same and is faster.



pub fn shift_right_safe(
        
        
            value: 
        , 
        
        
            bitShift: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97EF1E5BCE9DC075u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97EF1E5BCE9DC075u64;
        
        let result = invoke_raw!(
            hash,
                value, 
                bitShift
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn shift_right_raw(
        value: , 
        bitShift: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97EF1E5BCE9DC075u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97EF1E5BCE9DC075u64;

        invoke_raw_typed!(
            hash,
                value, 
                bitShift
        )
    }
}

