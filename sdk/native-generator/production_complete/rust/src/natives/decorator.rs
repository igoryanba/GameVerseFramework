//! DECORATOR native functions
//! 
//! Functions for the decorator category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn decor_get_bool_safe(
        
        
            entity: 
        , 
        
        
            propertyName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDACE671663F2F5DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDACE671663F2F5DBu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                propertyName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn decor_get_bool_raw(
        entity: , 
        propertyName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDACE671663F2F5DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDACE671663F2F5DBu64;

        invoke_raw_typed!(
            hash,
                entity, 
                propertyName
        )
    }
}

/// ## Parameters
*



pub fn decor_get_float_safe(
        
        
            entity: 
        , 
        
        
            propertyName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6524A2F114706F43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6524A2F114706F43u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                propertyName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn decor_get_float_raw(
        entity: , 
        propertyName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6524A2F114706F43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6524A2F114706F43u64;

        invoke_raw_typed!(
            hash,
                entity, 
                propertyName
        )
    }
}

/// ```
This function sets metadata of type bool to specified entity.  
```



pub fn decor_set_bool_safe(
        
        
            entity: 
        , 
        
        
            propertyName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B1E8E2ED1335B71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B1E8E2ED1335B71u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                propertyName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn decor_set_bool_raw(
        entity: , 
        propertyName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B1E8E2ED1335B71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B1E8E2ED1335B71u64;

        invoke_raw_typed!(
            hash,
                entity, 
                propertyName, 
                value
        )
    }
}

/// ```c
enum eDecorType
{
    DECOR_TYPE_FLOAT = 1,
    DECOR_TYPE_BOOL = 2,
    DECOR_TYPE_INT = 3,
    DECOR_TYPE_STRING = 4,
    DECOR_TYPE_TIME = 5
};
```



pub fn decor_register_safe(
        
        
            propertyName: 
        , 
        
        
            type: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FD90732F56403CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FD90732F56403CEu64;
        
        let result = invoke_raw!(
            hash,
                propertyName, 
                type
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn decor_register_raw(
        propertyName: , 
        type: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FD90732F56403CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FD90732F56403CEu64;

        invoke_raw_typed!(
            hash,
                propertyName, 
                type
        )
    }
}

/// ## Parameters
*



pub fn decor_set_float_safe(
        
        
            entity: 
        , 
        
        
            propertyName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x211AB1DD8D0F363Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x211AB1DD8D0F363Au64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                propertyName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn decor_set_float_raw(
        entity: , 
        propertyName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x211AB1DD8D0F363Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x211AB1DD8D0F363Au64;

        invoke_raw_typed!(
            hash,
                entity, 
                propertyName, 
                value
        )
    }
}

/// ## Parameters
*



pub fn decor_set_time_safe(
        
        
            entity: 
        , 
        
        
            propertyName: 
        , 
        
        
            timestamp: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95AED7B8E39ECAA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95AED7B8E39ECAA4u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                propertyName, 
                timestamp
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn decor_set_time_raw(
        entity: , 
        propertyName: , 
        timestamp: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95AED7B8E39ECAA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95AED7B8E39ECAA4u64;

        invoke_raw_typed!(
            hash,
                entity, 
                propertyName, 
                timestamp
        )
    }
}

/// ```
Returns whether or not the specified property is set for the entity.  
```



pub fn decor_exist_on_safe(
        
        
            entity: 
        , 
        
        
            propertyName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x05661B80A8C9165Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x05661B80A8C9165Fu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                propertyName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn decor_exist_on_raw(
        entity: , 
        propertyName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x05661B80A8C9165Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x05661B80A8C9165Fu64;

        invoke_raw_typed!(
            hash,
                entity, 
                propertyName
        )
    }
}

/// ## Parameters
*



pub fn decor_is_registered_as_type_safe(
        
        
            propertyName: 
        , 
        
        
            type: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F14F9F870D6FBC8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F14F9F870D6FBC8u64;
        
        let result = invoke_raw!(
            hash,
                propertyName, 
                type
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn decor_is_registered_as_type_raw(
        propertyName: , 
        type: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F14F9F870D6FBC8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F14F9F870D6FBC8u64;

        invoke_raw_typed!(
            hash,
                propertyName, 
                type
        )
    }
}

/// ```
Sets property to int.  
```



pub fn decor_set_int_safe(
        
        
            entity: 
        , 
        
        
            propertyName: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CE3AA5E1CA19E10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CE3AA5E1CA19E10u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                propertyName, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn decor_set_int_raw(
        entity: , 
        propertyName: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CE3AA5E1CA19E10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CE3AA5E1CA19E10u64;

        invoke_raw_typed!(
            hash,
                entity, 
                propertyName, 
                value
        )
    }
}

/// ## Parameters
*



pub fn decor_remove_safe(
        
        
            entity: 
        , 
        
        
            propertyName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00EE9F297C738720u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00EE9F297C738720u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                propertyName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn decor_remove_raw(
        entity: , 
        propertyName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00EE9F297C738720u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00EE9F297C738720u64;

        invoke_raw_typed!(
            hash,
                entity, 
                propertyName
        )
    }
}

/// ## Parameters
*



pub fn decor_get_int_safe(
        
        
            entity: 
        , 
        
        
            propertyName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA06C969B02A97298u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA06C969B02A97298u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                propertyName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn decor_get_int_raw(
        entity: , 
        propertyName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA06C969B02A97298u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA06C969B02A97298u64;

        invoke_raw_typed!(
            hash,
                entity, 
                propertyName
        )
    }
}

/// ```
Called after all decorator type initializations.  
```



pub fn decor_register_lock_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9D14EEA259F9248u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9D14EEA259F9248u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn decor_register_lock_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9D14EEA259F9248u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9D14EEA259F9248u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

