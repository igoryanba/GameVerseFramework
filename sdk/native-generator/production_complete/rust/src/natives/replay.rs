//! REPLAY native functions
//! 
//! Functions for the replay category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
Sets (almost, not sure) all Rockstar Editor values (bIsRecording etc) to 0.  
```



pub fn _reset_editor_values_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3353D13F09307691u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3353D13F09307691u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _reset_editor_values_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3353D13F09307691u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3353D13F09307691u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Disables some other rendering (internal)  
```



pub fn _0x5ad3932daeb1e5d3_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AD3932DAEB1E5D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AD3932DAEB1E5D3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5ad3932daeb1e5d3_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AD3932DAEB1E5D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AD3932DAEB1E5D3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**

```
Something to do with phone cameras.  
startup.c4:  
void sub_2a3d() {  
    UNK2::_7E2BD3EF6C205F09("No_Filter", 1);  
    UNK2::_7E2BD3EF6C205F09("phone_cam1", 1);  
    UNK2::_7E2BD3EF6C205F09("phone_cam2", 1);  
    UNK2::_7E2BD3EF6C205F09("phone_cam3", 1);  
    UNK2::_7E2BD3EF6C205F09("phone_cam4", 1);  
    UNK2::_7E2BD3EF6C205F09("phone_cam5", 1);  
    UNK2::_7E2BD3EF6C205F09("phone_cam6", 1);  
    UNK2::_7E2BD3EF6C205F09("phone_cam7", 1);  
    UNK2::_7E2BD3EF6C205F09("phone_cam9", 1);  
    UNK2::_7E2BD3EF6C205F09("phone_cam12", 0);  
}  
```



pub fn _0x7e2bd3ef6c205f09_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E2BD3EF6C205F09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E2BD3EF6C205F09u64;
        
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
pub fn _0x7e2bd3ef6c205f09_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E2BD3EF6C205F09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E2BD3EF6C205F09u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Please note that you will need to call DO_SCREEN_FADE_IN after exiting the Rockstar Editor when you call this.

```
NativeDB Added Parameter 1: int p0
```



pub fn _activate_rockstar_editor_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49DA8145672B2725u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49DA8145672B2725u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _activate_rockstar_editor_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49DA8145672B2725u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49DA8145672B2725u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns a bool if interior rendering is disabled, if yes, all "normal" rendered interiors are invisible  
```



pub fn _is_interior_rendering_disabled_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95AB8B5C992C7B58u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95AB8B5C992C7B58u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_interior_rendering_disabled_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95AB8B5C992C7B58u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95AB8B5C992C7B58u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xe058175f8eafe79a_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE058175F8EAFE79Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE058175F8EAFE79Au64;
        
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
pub fn _0xe058175f8eafe79a_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE058175F8EAFE79Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE058175F8EAFE79Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

