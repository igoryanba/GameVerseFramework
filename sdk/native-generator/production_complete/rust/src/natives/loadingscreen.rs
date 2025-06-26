//! LOADINGSCREEN native functions
//! 
//! Functions for the loadingscreen category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn _loadingscreen_set_is_loading_freemode_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7E7181C09F33B69u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7E7181C09F33B69u64;
        
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
pub fn _loadingscreen_set_is_loading_freemode_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7E7181C09F33B69u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7E7181C09F33B69u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value
Hard-coded to always return 0.



pub fn _0xf2ca003f167e21d2_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2CA003F167E21D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2CA003F167E21D2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf2ca003f167e21d2_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2CA003F167E21D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2CA003F167E21D2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _loadingscreen_get_load_freemode_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF7D17BC6C85264Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF7D17BC6C85264Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _loadingscreen_get_load_freemode_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF7D17BC6C85264Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF7D17BC6C85264Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _loadingscreen_set_load_freemode_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB0C56BD3D808D863u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB0C56BD3D808D863u64;
        
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
pub fn _loadingscreen_set_load_freemode_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB0C56BD3D808D863u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB0C56BD3D808D863u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value



pub fn _loadingscreen_get_load_freemode_with_event_name_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8AA464D4E0F6ACCDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8AA464D4E0F6ACCDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _loadingscreen_get_load_freemode_with_event_name_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8AA464D4E0F6ACCDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8AA464D4E0F6ACCDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Only occurrence was false, in maintransition.  
```



pub fn _loadingscreen_set_load_freemode_with_event_name_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC309E94546FCDB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC309E94546FCDB5u64;
        
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
pub fn _loadingscreen_set_load_freemode_with_event_name_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC309E94546FCDB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC309E94546FCDB5u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0xfa1e0e893d915215_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA1E0E893D915215u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA1E0E893D915215u64;
        
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
pub fn _0xfa1e0e893d915215_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA1E0E893D915215u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA1E0E893D915215u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value



pub fn _loadingscreen_is_loading_freemode_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6DC823253FBB366u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6DC823253FBB366u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _loadingscreen_is_loading_freemode_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6DC823253FBB366u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6DC823253FBB366u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

