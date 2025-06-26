//! DATAFILE native functions
//! 
//! Functions for the datafile category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
NativeDB Added Parameter 3: Any p2
```



pub fn datafile_select_ugc_stats_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CB0BFA7A9342C3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CB0BFA7A9342C3Du64;
        
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
pub fn datafile_select_ugc_stats_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CB0BFA7A9342C3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CB0BFA7A9342C3Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0xdbf860cf1db8e599_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBF860CF1DB8E599u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBF860CF1DB8E599u64;
        
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
pub fn _0xdbf860cf1db8e599_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBF860CF1DB8E599u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBF860CF1DB8E599u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn datadict_get_int_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78F06F6B1FB5A80Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78F06F6B1FB5A80Cu64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_get_int_raw(
        objectData: , 
        key: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78F06F6B1FB5A80Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78F06F6B1FB5A80Cu64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key
        )
    }
}

/// ## Parameters
*



pub fn dataarray_get_string_safe(
        
        
            arrayData: 
        , 
        
        
            arrayIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3F2FFEB8D836F52u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3F2FFEB8D836F52u64;
        
        let result = invoke_raw!(
            hash,
                arrayData, 
                arrayIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dataarray_get_string_raw(
        arrayData: , 
        arrayIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3F2FFEB8D836F52u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3F2FFEB8D836F52u64;

        invoke_raw_typed!(
            hash,
                arrayData, 
                arrayIndex
        )
    }
}

/// ```
Example:  
if (!DATAFILE::_BEDB96A7584AA8CF())  
{  
    if (!g_109E3)  
	{  
        if (((sub_d4f() == 2) == 0) && (!NETWORK::NETWORK_IS_GAME_IN_PROGRESS()))  
{  
            if (NETWORK::NETWORK_IS_CLOUD_AVAILABLE())  
	{  
                g_17A8B = 0;  
            }  
            if (!g_D52C)  
	{  
                sub_730();  
            }  
        }  
    }  
}  
```



pub fn datafile_is_save_pending_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBEDB96A7584AA8CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBEDB96A7584AA8CFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datafile_is_save_pending_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBEDB96A7584AA8CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBEDB96A7584AA8CFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn datadict_get_vector_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46CD3CB66E0825CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46CD3CB66E0825CCu64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_get_vector_raw(
        objectData: , 
        key: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46CD3CB66E0825CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46CD3CB66E0825CCu64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key
        )
    }
}

/// ## Parameters
*



pub fn dataarray_get_int_safe(
        
        
            arrayData: 
        , 
        
        
            arrayIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E5AE19425CD74BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E5AE19425CD74BEu64;
        
        let result = invoke_raw!(
            hash,
                arrayData, 
                arrayIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dataarray_get_int_raw(
        arrayData: , 
        arrayIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E5AE19425CD74BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E5AE19425CD74BEu64;

        invoke_raw_typed!(
            hash,
                arrayData, 
                arrayIndex
        )
    }
}

/// ## Parameters
*



pub fn dataarray_get_float_safe(
        
        
            arrayData: 
        , 
        
        
            arrayIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0C527B525D7CFB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0C527B525D7CFB5u64;
        
        let result = invoke_raw!(
            hash,
                arrayData, 
                arrayIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dataarray_get_float_raw(
        arrayData: , 
        arrayIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0C527B525D7CFB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0C527B525D7CFB5u64;

        invoke_raw_typed!(
            hash,
                arrayData, 
                arrayIndex
        )
    }
}

/// ```
NativeDB Added Parameter 1: int p0
```



pub fn datafile_store_mission_header_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2ED61456317B8178u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2ED61456317B8178u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datafile_store_mission_header_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2ED61456317B8178u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2ED61456317B8178u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn datadict_set_float_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC27E1CC2D795105Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC27E1CC2D795105Eu64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_set_float_raw(
        objectData: , 
        key: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC27E1CC2D795105Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC27E1CC2D795105Eu64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key, 
                value
        )
    }
}

/// ## Parameters
*



pub fn datadict_set_int_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7E035450A7948D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7E035450A7948D5u64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_set_int_raw(
        objectData: , 
        key: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7E035450A7948D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7E035450A7948D5u64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key, 
                value
        )
    }
}

/// ## Parameters
*



pub fn datadict_get_bool_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1186940ED72FFEECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1186940ED72FFEECu64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_get_bool_raw(
        objectData: , 
        key: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1186940ED72FFEECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1186940ED72FFEECu64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key
        )
    }
}

/// ## Parameters
*



pub fn dataarray_add_bool_safe(
        
        
            arrayData: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8B0F5A43E928C76u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8B0F5A43E928C76u64;
        
        let result = invoke_raw!(
            hash,
                arrayData, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dataarray_add_bool_raw(
        arrayData: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8B0F5A43E928C76u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8B0F5A43E928C76u64;

        invoke_raw_typed!(
            hash,
                arrayData, 
                value
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn datafile_select_creator_stats_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01095C95CD46B624u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01095C95CD46B624u64;
        
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
pub fn datafile_select_creator_stats_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01095C95CD46B624u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01095C95CD46B624u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Types:  
1 = Boolean  
2 = Integer  
3 = Float  
4 = String  
5 = Vector3  
6 = Object  
7 = Array  
```



pub fn dataarray_get_type_safe(
        
        
            arrayData: 
        , 
        
        
            arrayIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A0014ADB172A3C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A0014ADB172A3C5u64;
        
        let result = invoke_raw!(
            hash,
                arrayData, 
                arrayIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dataarray_get_type_raw(
        arrayData: , 
        arrayIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A0014ADB172A3C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A0014ADB172A3C5u64;

        invoke_raw_typed!(
            hash,
                arrayData, 
                arrayIndex
        )
    }
}

/// ## Parameters
*



pub fn datafile_delete_requested_file_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F5EA1C01D65A100u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F5EA1C01D65A100u64;
        
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
pub fn datafile_delete_requested_file_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F5EA1C01D65A100u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F5EA1C01D65A100u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn datafile_is_valid_request_id_safe(
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCCAE5B92A830878u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCCAE5B92A830878u64;
        
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
pub fn datafile_is_valid_request_id_raw(
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCCAE5B92A830878u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCCAE5B92A830878u64;

        invoke_raw_typed!(
            hash,
                index
        )
    }
}

/// ```
Adds the given request ID to the watch list.
```



pub fn datafile_watch_request_id_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD6875BBC0FC899Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD6875BBC0FC899Cu64;
        
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
pub fn datafile_watch_request_id_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD6875BBC0FC899Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD6875BBC0FC899Cu64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ## Parameters
*



pub fn dataarray_add_string_safe(
        
        
            arrayData: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F0661C155AEEEAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F0661C155AEEEAAu64;
        
        let result = invoke_raw!(
            hash,
                arrayData, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dataarray_add_string_raw(
        arrayData: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F0661C155AEEEAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F0661C155AEEEAAu64;

        invoke_raw_typed!(
            hash,
                arrayData, 
                value
        )
    }
}

/// ## Parameters
*



pub fn datadict_set_bool_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35124302A556A325u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35124302A556A325u64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_set_bool_raw(
        objectData: , 
        key: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35124302A556A325u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35124302A556A325u64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key, 
                value
        )
    }
}

/// ```
NativeDB Added Parameter 1: int p0
```



pub fn datafile_get_file_dict_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x906B778CA1DC72B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x906B778CA1DC72B6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datafile_get_file_dict_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x906B778CA1DC72B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x906B778CA1DC72B6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 4: Any p3
```



pub fn ugc_set_player_data_safe(
        
        
            contentId: 
        , 
        
        
            rating: 
        , 
        
        
            contentTypeName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x692D808C34A82143u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x692D808C34A82143u64;
        
        let result = invoke_raw!(
            hash,
                contentId, 
                rating, 
                contentTypeName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_set_player_data_raw(
        contentId: , 
        rating: , 
        contentTypeName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x692D808C34A82143u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x692D808C34A82143u64;

        invoke_raw_typed!(
            hash,
                contentId, 
                rating, 
                contentTypeName
        )
    }
}

/// ## Parameters
*



pub fn dataarray_get_bool_safe(
        
        
            arrayData: 
        , 
        
        
            arrayIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50C1B2874E50C114u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50C1B2874E50C114u64;
        
        let result = invoke_raw!(
            hash,
                arrayData, 
                arrayIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dataarray_get_bool_raw(
        arrayData: , 
        arrayIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50C1B2874E50C114u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50C1B2874E50C114u64;

        invoke_raw_typed!(
            hash,
                arrayData, 
                arrayIndex
        )
    }
}

/// DATAFILE_FLUSH_MISSION_HEADER native function



pub fn datafile_flush_mission_header_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC55854C7D7274882u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC55854C7D7274882u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datafile_flush_mission_header_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC55854C7D7274882u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC55854C7D7274882u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn datafile_select_active_file_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22DA66936E0FFF37u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22DA66936E0FFF37u64;
        
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
pub fn datafile_select_active_file_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22DA66936E0FFF37u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22DA66936E0FFF37u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn datadict_get_float_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06610343E73B9727u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06610343E73B9727u64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_get_float_raw(
        objectData: , 
        key: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06610343E73B9727u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06610343E73B9727u64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key
        )
    }
}

/// ## Parameters
*



pub fn datadict_set_vector_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        , 
        
        
            valueX: 
        , 
        
        
            valueY: 
        , 
        
        
            valueZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4CD49B76338C7DEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4CD49B76338C7DEEu64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key, 
                valueX, 
                valueY, 
                valueZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_set_vector_raw(
        objectData: , 
        key: , 
        valueX: , 
        valueY: , 
        valueZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4CD49B76338C7DEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4CD49B76338C7DEEu64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key, 
                valueX, 
                valueY, 
                valueZ
        )
    }
}

/// ```
Types:  
1 = Boolean  
2 = Integer  
3 = Float  
4 = String  
5 = Vector3  
6 = Object  
7 = Array  
```



pub fn datadict_get_type_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x031C55ED33227371u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x031C55ED33227371u64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_get_type_raw(
        objectData: , 
        key: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x031C55ED33227371u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x031C55ED33227371u64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn datafile_select_ugc_data_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA69AC4ADE82B57A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA69AC4ADE82B57A4u64;
        
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
pub fn datafile_select_ugc_data_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA69AC4ADE82B57A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA69AC4ADE82B57A4u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn dataarray_add_vector_safe(
        
        
            arrayData: 
        , 
        
        
            valueX: 
        , 
        
        
            valueY: 
        , 
        
        
            valueZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x407F8D034F70F0C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x407F8D034F70F0C2u64;
        
        let result = invoke_raw!(
            hash,
                arrayData, 
                valueX, 
                valueY, 
                valueZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dataarray_add_vector_raw(
        arrayData: , 
        valueX: , 
        valueY: , 
        valueZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x407F8D034F70F0C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x407F8D034F70F0C2u64;

        invoke_raw_typed!(
            hash,
                arrayData, 
                valueX, 
                valueY, 
                valueZ
        )
    }
}

/// ## Parameters
*



pub fn datafile_update_save_to_cloud_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4DFDD9EB705F8140u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4DFDD9EB705F8140u64;
        
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
pub fn datafile_update_save_to_cloud_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4DFDD9EB705F8140u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4DFDD9EB705F8140u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn datadict_create_array_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B11728527CA6E5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B11728527CA6E5Fu64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_create_array_raw(
        objectData: , 
        key: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B11728527CA6E5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B11728527CA6E5Fu64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn datafile_select_ugc_player_data_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52818819057F2B40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52818819057F2B40u64;
        
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
pub fn datafile_select_ugc_player_data_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52818819057F2B40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52818819057F2B40u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn datadict_set_string_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8FF3847DADD8E30Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8FF3847DADD8E30Cu64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_set_string_raw(
        objectData: , 
        key: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8FF3847DADD8E30Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8FF3847DADD8E30Cu64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key, 
                value
        )
    }
}

/// ## Parameters
*



pub fn dataarray_add_int_safe(
        
        
            arrayData: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCABDB751D86FE93Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCABDB751D86FE93Bu64;
        
        let result = invoke_raw!(
            hash,
                arrayData, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dataarray_add_int_raw(
        arrayData: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCABDB751D86FE93Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCABDB751D86FE93Bu64;

        invoke_raw_typed!(
            hash,
                arrayData, 
                value
        )
    }
}

/// ```
NativeDB Added Parameter 6: Any p5
```



pub fn ugc_create_mission_safe(
        
        
            contentName: 
        , 
        
        
            description: 
        , 
        
        
            tagsCsv: 
        , 
        
        
            contentTypeName: 
        , 
        
        
            publish: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5EFC3E847D60507u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5EFC3E847D60507u64;
        
        let result = invoke_raw!(
            hash,
                contentName, 
                description, 
                tagsCsv, 
                contentTypeName, 
                publish
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_create_mission_raw(
        contentName: , 
        description: , 
        tagsCsv: , 
        contentTypeName: , 
        publish: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5EFC3E847D60507u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5EFC3E847D60507u64;

        invoke_raw_typed!(
            hash,
                contentName, 
                description, 
                tagsCsv, 
                contentTypeName, 
                publish
        )
    }
}

/// ## Parameters
*



pub fn dataarray_get_count_safe(
        
        
            arrayData: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x065DB281590CEA2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x065DB281590CEA2Du64;
        
        let result = invoke_raw!(
            hash,
                arrayData
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dataarray_get_count_raw(
        arrayData: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x065DB281590CEA2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x065DB281590CEA2Du64;

        invoke_raw_typed!(
            hash,
                arrayData
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn datafile_start_save_to_cloud_safe(
        
        
            filename: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83BCCE3224735F05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83BCCE3224735F05u64;
        
        let result = invoke_raw!(
            hash,
                filename
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datafile_start_save_to_cloud_raw(
        filename: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83BCCE3224735F05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83BCCE3224735F05u64;

        invoke_raw_typed!(
            hash,
                filename
        )
    }
}

/// ```
Loads a User-Generated Content (UGC) file. These files can be found in "[GTA5]\data\ugc" and "[GTA5]\common\patch\ugc". They seem to follow a naming convention, most likely of "[name]_[part].ugc". See example below for usage.
Returns whether or not the file was successfully loaded.
Example:
DATAFILE::_LOAD_UGC_FILE("RockstarPlaylists") // loads "rockstarplaylists_00.ugc"
```

```
NativeDB Added Parameter 2: Any p1
```



pub fn datafile_load_offline_ugc_safe(
        
        
            filename: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5238C011AF405E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5238C011AF405E4u64;
        
        let result = invoke_raw!(
            hash,
                filename
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datafile_load_offline_ugc_raw(
        filename: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5238C011AF405E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5238C011AF405E4u64;

        invoke_raw_typed!(
            hash,
                filename
        )
    }
}

/// ```
NativeDB Added Parameter 8: Any p7
```



pub fn ugc_create_content_safe(
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC84527E235FCA219u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC84527E235FCA219u64;
        
        let result = invoke_raw!(
            hash,
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_create_content_raw(
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC84527E235FCA219u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC84527E235FCA219u64;

        invoke_raw_typed!(
            hash,
                data
        )
    }
}

/// ## Parameters
*



pub fn datafile_has_loaded_file_data_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15FF52B809DB2353u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15FF52B809DB2353u64;
        
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
pub fn datafile_has_loaded_file_data_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15FF52B809DB2353u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15FF52B809DB2353u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Added Parameter 1: int p0
```



pub fn datafile_create_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD27058A1CA2B13EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD27058A1CA2B13EEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datafile_create_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD27058A1CA2B13EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD27058A1CA2B13EEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn dataarray_add_dict_safe(
        
        
            arrayData: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6889498B3E19C797u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6889498B3E19C797u64;
        
        let result = invoke_raw!(
            hash,
                arrayData
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dataarray_add_dict_raw(
        arrayData: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6889498B3E19C797u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6889498B3E19C797u64;

        invoke_raw_typed!(
            hash,
                arrayData
        )
    }
}

/// ## Parameters
*



pub fn datadict_get_dict_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6B9DDC412FCEEE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6B9DDC412FCEEE2u64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_get_dict_raw(
        objectData: , 
        key: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6B9DDC412FCEEE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6B9DDC412FCEEE2u64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key
        )
    }
}

/// ```
NativeDB Added Parameter 8: Any p7
```



pub fn ugc_update_content_safe(
        
        
            contentId: 
        , 
        
        
            data: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x648E7A5434AF7969u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x648E7A5434AF7969u64;
        
        let result = invoke_raw!(
            hash,
                contentId, 
                data
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_update_content_raw(
        contentId: , 
        data: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x648E7A5434AF7969u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x648E7A5434AF7969u64;

        invoke_raw_typed!(
            hash,
                contentId, 
                data
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0x6ad0bd5e087866cb_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6AD0BD5E087866CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6AD0BD5E087866CBu64;
        
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
pub fn _0x6ad0bd5e087866cb_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6AD0BD5E087866CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6AD0BD5E087866CBu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn datadict_get_array_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A983AA9DA2659EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A983AA9DA2659EDu64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_get_array_raw(
        objectData: , 
        key: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A983AA9DA2659EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A983AA9DA2659EDu64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key
        )
    }
}

/// ## Parameters
*



pub fn dataarray_get_vector_safe(
        
        
            arrayData: 
        , 
        
        
            arrayIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D2064E5B64A628Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D2064E5B64A628Au64;
        
        let result = invoke_raw!(
            hash,
                arrayData, 
                arrayIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dataarray_get_vector_raw(
        arrayData: , 
        arrayIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D2064E5B64A628Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D2064E5B64A628Au64;

        invoke_raw_typed!(
            hash,
                arrayData, 
                arrayIndex
        )
    }
}

/// ```
NativeDB Added Parameter 6: Any p5
```



pub fn ugc_update_mission_safe(
        
        
            contentId: 
        , 
        
        
            contentName: 
        , 
        
        
            description: 
        , 
        
        
            tagsCsv: 
        , 
        
        
            contentTypeName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4645DE9980999E93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4645DE9980999E93u64;
        
        let result = invoke_raw!(
            hash,
                contentId, 
                contentName, 
                description, 
                tagsCsv, 
                contentTypeName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ugc_update_mission_raw(
        contentId: , 
        contentName: , 
        description: , 
        tagsCsv: , 
        contentTypeName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4645DE9980999E93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4645DE9980999E93u64;

        invoke_raw_typed!(
            hash,
                contentId, 
                contentName, 
                description, 
                tagsCsv, 
                contentTypeName
        )
    }
}

/// ```
NativeDB Added Parameter 1: int p0
```



pub fn datafile_delete_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AB9C1CFC8862DFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AB9C1CFC8862DFBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datafile_delete_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AB9C1CFC8862DFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AB9C1CFC8862DFBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn datadict_get_string_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D2FD9E763B24472u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D2FD9E763B24472u64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_get_string_raw(
        objectData: , 
        key: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D2FD9E763B24472u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D2FD9E763B24472u64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key
        )
    }
}

/// ## Parameters
*



pub fn datadict_create_dict_safe(
        
        
            objectData: 
        , 
        
        
            key: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA358F56F10732EE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA358F56F10732EE1u64;
        
        let result = invoke_raw!(
            hash,
                objectData, 
                key
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datadict_create_dict_raw(
        objectData: , 
        key: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA358F56F10732EE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA358F56F10732EE1u64;

        invoke_raw_typed!(
            hash,
                objectData, 
                key
        )
    }
}

/// DATAFILE_CLEAR_WATCH_LIST native function



pub fn datafile_clear_watch_list_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CC86E78358D5119u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CC86E78358D5119u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn datafile_clear_watch_list_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CC86E78358D5119u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CC86E78358D5119u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0xa6eef01087181edd_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6EEF01087181EDDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6EEF01087181EDDu64;
        
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
pub fn _0xa6eef01087181edd_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6EEF01087181EDDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6EEF01087181EDDu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn dataarray_add_float_safe(
        
        
            arrayData: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57A995FD75D37F56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57A995FD75D37F56u64;
        
        let result = invoke_raw!(
            hash,
                arrayData, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dataarray_add_float_raw(
        arrayData: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57A995FD75D37F56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57A995FD75D37F56u64;

        invoke_raw_typed!(
            hash,
                arrayData, 
                value
        )
    }
}

/// ## Parameters
*



pub fn dataarray_get_dict_safe(
        
        
            arrayData: 
        , 
        
        
            arrayIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B5FADCC4E3A145Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B5FADCC4E3A145Fu64;
        
        let result = invoke_raw!(
            hash,
                arrayData, 
                arrayIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dataarray_get_dict_raw(
        arrayData: , 
        arrayIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B5FADCC4E3A145Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B5FADCC4E3A145Fu64;

        invoke_raw_typed!(
            hash,
                arrayData, 
                arrayIndex
        )
    }
}

/// ## Parameters
*



pub fn datafile_has_valid_file_data_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8CC1EBE0B62E29Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8CC1EBE0B62E29Fu64;
        
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
pub fn datafile_has_valid_file_data_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8CC1EBE0B62E29Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8CC1EBE0B62E29Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

