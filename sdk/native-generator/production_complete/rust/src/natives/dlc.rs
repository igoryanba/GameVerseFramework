//! DLC native functions
//! 
//! Functions for the dlc category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Return value
Hard-coded to always return 1.



pub fn _0xa213b11dff526300_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA213B11DFF526300u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA213B11DFF526300u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa213b11dff526300_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA213B11DFF526300u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA213B11DFF526300u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _get_extra_content_pack_has_been_installed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D30F648014A92B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D30F648014A92B5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_extra_content_pack_has_been_installed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D30F648014A92B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D30F648014A92B5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// GET_IS_LOADING_*

```
NativeDB Introduced: v1734
```



pub fn _0xc4637a6d03c24cc3_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4637A6D03C24CC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4637A6D03C24CC3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc4637a6d03c24cc3_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4637A6D03C24CC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4637A6D03C24CC3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Unloads GROUP_MAP (GTAO/MP) DLC data and loads GROUP_MAP_SP DLC. Neither are loaded by default, 0888C3502DBBEEF5 is a cognate to this function and loads MP DLC (and unloads SP DLC by extension).
The original (and wrong) definition is below:
This unload the GTA:O DLC map parts (like high end garages/apartments).
Works in singleplayer.
```



pub fn on_enter_sp_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7C10C4A637992C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7C10C4A637992C9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn on_enter_sp_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7C10C4A637992C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7C10C4A637992C9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value
Hard-coded to always return 0.



pub fn _0x9489659372a81585_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9489659372A81585u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9489659372A81585u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x9489659372a81585_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9489659372A81585u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9489659372A81585u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
This loads the GTA:O dlc map parts (high end garages, apartments).
Works in singleplayer.
In order to use GTA:O heist IPL's you have to call this native with the following params: SET_INSTANCE_PRIORITY_MODE(1);
```



pub fn on_enter_mp_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0888C3502DBBEEF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0888C3502DBBEEF5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn on_enter_mp_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0888C3502DBBEEF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0888C3502DBBEEF5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_is_loading_screen_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10D0A8F259E93EC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10D0A8F259E93EC9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_is_loading_screen_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10D0A8F259E93EC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10D0A8F259E93EC9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Example:
DLC::IS_DLC_PRESENT($\mpbusiness2\);
($ = gethashkey)
bruteforce these:
0xB119F6D
0x96F02EE6
```



pub fn is_dlc_present_safe(
        
        
            dlcHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x812595A0644CE1DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x812595A0644CE1DEu64;
        
        let result = invoke_raw!(
            hash,
                dlcHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_dlc_present_raw(
        dlcHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x812595A0644CE1DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x812595A0644CE1DEu64;

        invoke_raw_typed!(
            hash,
                dlcHash
        )
    }
}

/// ```
Only used once in scripts, in maintransition.
maintransition.c4, line ~82432:
if (PED::_7350823473013C02(PLAYER::PLAYER_PED_ID()) && (DECORATOR::_241FCA5B1AA14F75() == 0)) {
    g_2542A5 = a_1; // 'g_2542A5' used in 'building_controller.ysc' for IPL stuff?
    return 1;
}
Likely used solely for the players ped. The function it's in seems to only be used for initialization/quitting. Called among natives to discard scaleforms, disable frontend, fading in/out, etc. Neighboring strings to some calls include "HUD_JOINING", "HUD_QUITTING".
Most likely ARE_*
```



pub fn _0x241fca5b1aa14f75_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x241FCA5B1AA14F75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x241FCA5B1AA14F75u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x241fca5b1aa14f75_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x241FCA5B1AA14F75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x241FCA5B1AA14F75u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value
Hard-coded to always return 1.



pub fn _0xf2e07819ef1a5289_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2E07819EF1A5289u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2E07819EF1A5289u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf2e07819ef1a5289_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2E07819EF1A5289u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2E07819EF1A5289u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Sets the value of the specified variable to 0.
Always returns true.
```



pub fn has_cloud_requests_finished_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46E2B844905BC5F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46E2B844905BC5F0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_cloud_requests_finished_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46E2B844905BC5F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46E2B844905BC5F0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

