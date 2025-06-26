//! APP native functions
//! 
//! Functions for the app category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// APP_SAVE_DATA native function



pub fn app_save_data_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95C5D356CDA6E85Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95C5D356CDA6E85Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_save_data_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95C5D356CDA6E85Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95C5D356CDA6E85Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// APP_CLOSE_BLOCK native function



pub fn app_close_block_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8E3FCF72EAC0EF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8E3FCF72EAC0EF8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_close_block_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8E3FCF72EAC0EF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8E3FCF72EAC0EF8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn app_set_block_safe(
        
        
            blockName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x262AB456A3D21F93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x262AB456A3D21F93u64;
        
        let result = invoke_raw!(
            hash,
                blockName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_set_block_raw(
        blockName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x262AB456A3D21F93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x262AB456A3D21F93u64;

        invoke_raw_typed!(
            hash,
                blockName
        )
    }
}

/// APP_CLEAR_BLOCK native function



pub fn app_clear_block_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FE1DF3342DB7DBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FE1DF3342DB7DBAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_clear_block_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FE1DF3342DB7DBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FE1DF3342DB7DBAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn app_get_deleted_file_status_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9853A2BE3DED1A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9853A2BE3DED1A6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_get_deleted_file_status_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9853A2BE3DED1A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9853A2BE3DED1A6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn app_has_linked_social_club_account_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71EEE69745088DA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71EEE69745088DA0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_has_linked_social_club_account_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71EEE69745088DA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71EEE69745088DA0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn app_has_synced_data_safe(
        
        
            appName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA52279A7271517Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA52279A7271517Fu64;
        
        let result = invoke_raw!(
            hash,
                appName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_has_synced_data_raw(
        appName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA52279A7271517Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA52279A7271517Fu64;

        invoke_raw_typed!(
            hash,
                appName
        )
    }
}

/// APP_CLOSE_APP native function



pub fn app_close_app_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE41C65E07A5F05FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE41C65E07A5F05FCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_close_app_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE41C65E07A5F05FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE41C65E07A5F05FCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn app_set_int_safe(
        
        
            property: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x607E8E3D3E4F9611u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x607E8E3D3E4F9611u64;
        
        let result = invoke_raw!(
            hash,
                property, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_set_int_raw(
        property: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x607E8E3D3E4F9611u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x607E8E3D3E4F9611u64;

        invoke_raw_typed!(
            hash,
                property, 
                value
        )
    }
}

/// ## Parameters
*



pub fn app_set_float_safe(
        
        
            property: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25D7687C68E0DAA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25D7687C68E0DAA4u64;
        
        let result = invoke_raw!(
            hash,
                property, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_set_float_raw(
        property: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25D7687C68E0DAA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25D7687C68E0DAA4u64;

        invoke_raw_typed!(
            hash,
                property, 
                value
        )
    }
}

/// ## Parameters
*



pub fn app_delete_app_data_safe(
        
        
            appName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44151AEA95C8A003u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44151AEA95C8A003u64;
        
        let result = invoke_raw!(
            hash,
                appName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_delete_app_data_raw(
        appName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44151AEA95C8A003u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44151AEA95C8A003u64;

        invoke_raw_typed!(
            hash,
                appName
        )
    }
}

/// ## Parameters
*



pub fn app_get_int_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3A58A12C77D9D4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3A58A12C77D9D4Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_get_int_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3A58A12C77D9D4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3A58A12C77D9D4Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn app_get_string_safe(
        
        
            property: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x749B023950D2311Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x749B023950D2311Cu64;
        
        let result = invoke_raw!(
            hash,
                property
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_get_string_raw(
        property: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x749B023950D2311Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x749B023950D2311Cu64;

        invoke_raw_typed!(
            hash,
                property
        )
    }
}

/// ```
Called in the gamescripts like:  
APP::APP_SET_APP("car");  
APP::APP_SET_APP("dog");  
```



pub fn app_set_app_safe(
        
        
            appName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFD0406ADAF90D2Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFD0406ADAF90D2Bu64;
        
        let result = invoke_raw!(
            hash,
                appName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_set_app_raw(
        appName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFD0406ADAF90D2Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFD0406ADAF90D2Bu64;

        invoke_raw_typed!(
            hash,
                appName
        )
    }
}

/// ## Parameters
*



pub fn app_set_string_safe(
        
        
            property: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FF2FCEC4B7721B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FF2FCEC4B7721B4u64;
        
        let result = invoke_raw!(
            hash,
                property, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_set_string_raw(
        property: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FF2FCEC4B7721B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FF2FCEC4B7721B4u64;

        invoke_raw_typed!(
            hash,
                property, 
                value
        )
    }
}

/// ## Parameters
*



pub fn app_get_float_safe(
        
        
            property: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1514FB24C02C2322u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1514FB24C02C2322u64;
        
        let result = invoke_raw!(
            hash,
                property
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_get_float_raw(
        property: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1514FB24C02C2322u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1514FB24C02C2322u64;

        invoke_raw_typed!(
            hash,
                property
        )
    }
}

/// ## Return value



pub fn app_data_valid_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x846AA8E7D55EE5B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x846AA8E7D55EE5B6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn app_data_valid_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x846AA8E7D55EE5B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x846AA8E7D55EE5B6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

