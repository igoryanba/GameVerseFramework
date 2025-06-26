//! SCRIPT native functions
//! 
//! Functions for the script category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn terminate_thread_safe(
        
        
            threadId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8B189ED9138BCD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8B189ED9138BCD4u64;
        
        let result = invoke_raw!(
            hash,
                threadId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn terminate_thread_raw(
        threadId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8B189ED9138BCD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8B189ED9138BCD4u64;

        invoke_raw_typed!(
            hash,
                threadId
        )
    }
}

/// SHUTDOWN_LOADING_SCREEN native function



pub fn shutdown_loading_screen_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x078EBE9809CCD637u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x078EBE9809CCD637u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn shutdown_loading_screen_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x078EBE9809CCD637u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x078EBE9809CCD637u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_script_with_name_hash_as_no_longer_needed_safe(
        
        
            scriptHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5BC038960E9DB27u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5BC038960E9DB27u64;
        
        let result = invoke_raw!(
            hash,
                scriptHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_script_with_name_hash_as_no_longer_needed_raw(
        scriptHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5BC038960E9DB27u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5BC038960E9DB27u64;

        invoke_raw_typed!(
            hash,
                scriptHash
        )
    }
}

/// ```
If the function returns 0, the end of the iteration has been reached.
```



pub fn script_thread_iterator_get_next_thread_id_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30B4FA1C82DD4B9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30B4FA1C82DD4B9Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn script_thread_iterator_get_next_thread_id_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30B4FA1C82DD4B9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30B4FA1C82DD4B9Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_script_as_no_longer_needed_safe(
        
        
            scriptName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC90D2DCACD56184Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC90D2DCACD56184Cu64;
        
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
pub fn set_script_as_no_longer_needed_raw(
        scriptName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC90D2DCACD56184Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC90D2DCACD56184Cu64;

        invoke_raw_typed!(
            hash,
                scriptName
        )
    }
}

/// ```
eventGroup: 0 = SCRIPT_EVENT_QUEUE_AI (CEventGroupScriptAI), 1 = SCRIPT_EVENT_QUEUE_NETWORK (CEventGroupScriptNetwork)
Note: eventDataSize is NOT the size in bytes, it is the size determined by the SIZE_OF operator (RAGE Script operator, not C/C++ sizeof). That is, the size in bytes divided by 8 (script variables are always 8-byte aligned!).
```



pub fn get_event_data_safe(
        
        
            eventGroup: 
        , 
        
        
            eventIndex: 
        , 
        
        
            eventData: 
        , 
        
        
            eventDataSize: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2902843FCD2B2D79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2902843FCD2B2D79u64;
        
        let result = invoke_raw!(
            hash,
                eventGroup, 
                eventIndex, 
                eventData, 
                eventDataSize
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_event_data_raw(
        eventGroup: , 
        eventIndex: , 
        eventData: , 
        eventDataSize: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2902843FCD2B2D79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2902843FCD2B2D79u64;

        invoke_raw_typed!(
            hash,
                eventGroup, 
                eventIndex, 
                eventData, 
                eventDataSize
        )
    }
}

/// Starts a new iteration of the current threads.
Call this first, then SCRIPT_THREAD_ITERATOR_GET_NEXT_THREAD_ID (0x30B4FA1C82DD4B9F)



pub fn script_thread_iterator_reset_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDADFADA5A20143A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDADFADA5A20143A8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn script_thread_iterator_reset_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDADFADA5A20143A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDADFADA5A20143A8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_this_script_name_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x442E0A7EDE4A738Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x442E0A7EDE4A738Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_this_script_name_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x442E0A7EDE4A738Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x442E0A7EDE4A738Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
See TRIGGER_SCRIPT_EVENT
```



pub fn _trigger_script_event_2_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA40CC53DF8E50837u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA40CC53DF8E50837u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _trigger_script_event_2_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA40CC53DF8E50837u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA40CC53DF8E50837u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Returns if a script has been loaded into the game. Used to see if a script was loaded after requesting.



pub fn has_script_loaded_safe(
        
        
            scriptName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6CC9F3BA0FB9EF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6CC9F3BA0FB9EF1u64;
        
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
pub fn has_script_loaded_raw(
        scriptName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6CC9F3BA0FB9EF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6CC9F3BA0FB9EF1u64;

        invoke_raw_typed!(
            hash,
                scriptName
        )
    }
}

/// ```
Deletes the given context from the background scripts context map.

NativeDB Introduced: v323
```



pub fn bg_end_context_safe(
        
        
            contextName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC2BACD920D0A0DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC2BACD920D0A0DDu64;
        
        let result = invoke_raw!(
            hash,
                contextName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn bg_end_context_raw(
        contextName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC2BACD920D0A0DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC2BACD920D0A0DDu64;

        invoke_raw_typed!(
            hash,
                contextName
        )
    }
}

/// ```
eventGroup: 0 = SCRIPT_EVENT_QUEUE_AI (CEventGroupScriptAI), 1 = SCRIPT_EVENT_QUEUE_NETWORK (CEventGroupScriptNetwork)
```



pub fn get_event_exists_safe(
        
        
            eventGroup: 
        , 
        
        
            eventIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x936E6168A9BCEDB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x936E6168A9BCEDB5u64;
        
        let result = invoke_raw!(
            hash,
                eventGroup, 
                eventIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_event_exists_raw(
        eventGroup: , 
        eventIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x936E6168A9BCEDB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x936E6168A9BCEDB5u64;

        invoke_raw_typed!(
            hash,
                eventGroup, 
                eventIndex
        )
    }
}

/// ## Return value



pub fn get_no_loading_screen_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x18C1270EA7F199BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x18C1270EA7F199BCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_no_loading_screen_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x18C1270EA7F199BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x18C1270EA7F199BCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
BG_*

NativeDB Introduced: v323
```



pub fn _0x22e21fbcfc88c149_safe(
        
        
            scriptIndex: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22E21FBCFC88C149u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22E21FBCFC88C149u64;
        
        let result = invoke_raw!(
            hash,
                scriptIndex, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x22e21fbcfc88c149_raw(
        scriptIndex: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22E21FBCFC88C149u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22E21FBCFC88C149u64;

        invoke_raw_typed!(
            hash,
                scriptIndex, 
                p1
        )
    }
}

/// ## Return value



pub fn get_id_of_this_thread_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC30338E8088E2E21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC30338E8088E2E21u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_id_of_this_thread_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC30338E8088E2E21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC30338E8088E2E21u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_hash_of_this_script_name_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A1C8B1738FFE87Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A1C8B1738FFE87Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_hash_of_this_script_name_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A1C8B1738FFE87Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A1C8B1738FFE87Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_no_loading_screen_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5262CC1995D07E09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5262CC1995D07E09u64;
        
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
pub fn set_no_loading_screen_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5262CC1995D07E09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5262CC1995D07E09u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn does_script_with_name_hash_exist_safe(
        
        
            scriptHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF86AA3C56BA31381u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF86AA3C56BA31381u64;
        
        let result = invoke_raw!(
            hash,
                scriptHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_script_with_name_hash_exist_raw(
        scriptHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF86AA3C56BA31381u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF86AA3C56BA31381u64;

        invoke_raw_typed!(
            hash,
                scriptHash
        )
    }
}

/// TERMINATE_THIS_THREAD native function



pub fn terminate_this_thread_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1090044AD1DA76FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1090044AD1DA76FAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn terminate_this_thread_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1090044AD1DA76FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1090044AD1DA76FAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
BG_*

NativeDB Introduced: v323
```



pub fn _0x0f6f1ebbc4e1d5e6_safe(
        
        
            scriptIndex: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F6F1EBBC4E1D5E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F6F1EBBC4E1D5E6u64;
        
        let result = invoke_raw!(
            hash,
                scriptIndex, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x0f6f1ebbc4e1d5e6_raw(
        scriptIndex: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F6F1EBBC4E1D5E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F6F1EBBC4E1D5E6u64;

        invoke_raw_typed!(
            hash,
                scriptIndex, 
                p1
        )
    }
}

/// ```
formerly _REQUEST_STREAMED_SCRIPT  
```



pub fn request_script_with_name_hash_safe(
        
        
            scriptHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD62A67D26D9653E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD62A67D26D9653E6u64;
        
        let result = invoke_raw!(
            hash,
                scriptHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_script_with_name_hash_raw(
        scriptHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD62A67D26D9653E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD62A67D26D9653E6u64;

        invoke_raw_typed!(
            hash,
                scriptHash
        )
    }
}

/// ```
eventGroup: 0 = SCRIPT_EVENT_QUEUE_AI (CEventGroupScriptAI), 1 = SCRIPT_EVENT_QUEUE_NETWORK (CEventGroupScriptNetwork)
Note: eventDataSize is NOT the size in bytes, it is the size determined by the SIZE_OF operator (RAGE Script operator, not C/C++ sizeof). That is, the size in bytes divided by 8 (script variables are always 8-byte aligned!).
playerBits (also known as playersToBroadcastTo) is a bitset that indicates which players this event should be sent to. In order to send the event to specific players only, use (1 << playerIndex). Set all bits if it should be broadcast to all players.
```



pub fn trigger_script_event_safe(
        
        
            eventGroup: 
        , 
        
        
            eventData: 
        , 
        
        
            eventDataSize: 
        , 
        
        
            playerBits: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AE99C571D5BBE5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AE99C571D5BBE5Du64;
        
        let result = invoke_raw!(
            hash,
                eventGroup, 
                eventData, 
                eventDataSize, 
                playerBits
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn trigger_script_event_raw(
        eventGroup: , 
        eventData: , 
        eventDataSize: , 
        playerBits: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AE99C571D5BBE5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AE99C571D5BBE5Du64;

        invoke_raw_typed!(
            hash,
                eventGroup, 
                eventData, 
                eventDataSize, 
                playerBits
        )
    }
}

/// ## Parameters
*



pub fn request_script_safe(
        
        
            scriptName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6EB5F71AA68F2E8Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6EB5F71AA68F2E8Eu64;
        
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
pub fn request_script_raw(
        scriptName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6EB5F71AA68F2E8Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6EB5F71AA68F2E8Eu64;

        invoke_raw_typed!(
            hash,
                scriptName
        )
    }
}

/// ## Parameters
*



pub fn does_script_exist_safe(
        
        
            scriptName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC04745FBE67C19Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC04745FBE67C19Au64;
        
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
pub fn does_script_exist_raw(
        scriptName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC04745FBE67C19Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC04745FBE67C19Au64;

        invoke_raw_typed!(
            hash,
                scriptName
        )
    }
}

/// ```
Hashed version of 0xDC2BACD920D0A0DD.

NativeDB Introduced: v323
```



pub fn bg_end_context_hash_safe(
        
        
            contextHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x107E5CC7CA942BC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x107E5CC7CA942BC1u64;
        
        let result = invoke_raw!(
            hash,
                contextHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn bg_end_context_hash_raw(
        contextHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x107E5CC7CA942BC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x107E5CC7CA942BC1u64;

        invoke_raw_typed!(
            hash,
                contextHash
        )
    }
}

/// ```
Hashed version of 0x9D5A25BADB742ACD.

NativeDB Introduced: v323
```



pub fn bg_start_context_hash_safe(
        
        
            contextHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75B18E49607874C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75B18E49607874C7u64;
        
        let result = invoke_raw!(
            hash,
                contextHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn bg_start_context_hash_raw(
        contextHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75B18E49607874C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75B18E49607874C7u64;

        invoke_raw_typed!(
            hash,
                contextHash
        )
    }
}

/// ```
Inserts the given context into the background scripts context map.

NativeDB Introduced: v323
```



pub fn bg_start_context_safe(
        
        
            contextName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D5A25BADB742ACDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D5A25BADB742ACDu64;
        
        let result = invoke_raw!(
            hash,
                contextName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn bg_start_context_raw(
        contextName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D5A25BADB742ACDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D5A25BADB742ACDu64;

        invoke_raw_typed!(
            hash,
                contextName
        )
    }
}

/// ```
Returns true if bit 0 in GtaThread+0x154 is set.

BG_*

NativeDB Introduced: v323
```



pub fn _0x836b62713e0534ca_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x836B62713E0534CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x836B62713E0534CAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x836b62713e0534ca_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x836B62713E0534CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x836B62713E0534CAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
eventGroup: 0 = SCRIPT_EVENT_QUEUE_AI (CEventGroupScriptAI), 1 = SCRIPT_EVENT_QUEUE_NETWORK (CEventGroupScriptNetwork)
```



pub fn get_number_of_events_safe(
        
        
            eventGroup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F92A689A06620AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F92A689A06620AAu64;
        
        let result = invoke_raw!(
            hash,
                eventGroup
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_number_of_events_raw(
        eventGroup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F92A689A06620AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F92A689A06620AAu64;

        invoke_raw_typed!(
            hash,
                eventGroup
        )
    }
}

/// ```
Gets the number of instances of the specified script is currently running.
Actually returns numRefs - 1.
if (program)
	v3 = rage::scrProgram::GetNumRefs(program) - 1;
return v3;
```



pub fn _get_number_of_references_of_script_with_name_hash_safe(
        
        
            scriptHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C83A9DA6BFFC4F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C83A9DA6BFFC4F9u64;
        
        let result = invoke_raw!(
            hash,
                scriptHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_number_of_references_of_script_with_name_hash_raw(
        scriptHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C83A9DA6BFFC4F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C83A9DA6BFFC4F9u64;

        invoke_raw_typed!(
            hash,
                scriptHash
        )
    }
}

/// ## Parameters
*



pub fn is_thread_active_safe(
        
        
            threadId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46E9AE36D8FA6417u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46E9AE36D8FA6417u64;
        
        let result = invoke_raw!(
            hash,
                threadId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_thread_active_raw(
        threadId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46E9AE36D8FA6417u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46E9AE36D8FA6417u64;

        invoke_raw_typed!(
            hash,
                threadId
        )
    }
}

/// ```
eventGroup: 0 = SCRIPT_EVENT_QUEUE_AI (CEventGroupScriptAI), 1 = SCRIPT_EVENT_QUEUE_NETWORK (CEventGroupScriptNetwork)
```



pub fn get_event_at_index_safe(
        
        
            eventGroup: 
        , 
        
        
            eventIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8F66A3A60C62153u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8F66A3A60C62153u64;
        
        let result = invoke_raw!(
            hash,
                eventGroup, 
                eventIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_event_at_index_raw(
        eventGroup: , 
        eventIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8F66A3A60C62153u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8F66A3A60C62153u64;

        invoke_raw_typed!(
            hash,
                eventGroup, 
                eventIndex
        )
    }
}

/// ```
BG_*

NativeDB Introduced: v323
```



pub fn _0x829cd22e043a2577_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x829CD22E043A2577u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x829CD22E043A2577u64;
        
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
pub fn _0x829cd22e043a2577_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x829CD22E043A2577u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x829CD22E043A2577u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _get_name_of_thread_safe(
        
        
            threadId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x05A42BA9FC8DA96Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x05A42BA9FC8DA96Bu64;
        
        let result = invoke_raw!(
            hash,
                threadId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_name_of_thread_raw(
        threadId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x05A42BA9FC8DA96Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x05A42BA9FC8DA96Bu64;

        invoke_raw_typed!(
            hash,
                threadId
        )
    }
}

/// ## Parameters
*



pub fn has_script_with_name_hash_loaded_safe(
        
        
            scriptHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F0F0C783EB16C04u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F0F0C783EB16C04u64;
        
        let result = invoke_raw!(
            hash,
                scriptHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_script_with_name_hash_loaded_raw(
        scriptHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F0F0C783EB16C04u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F0F0C783EB16C04u64;

        invoke_raw_typed!(
            hash,
                scriptHash
        )
    }
}

/// ```
Sets bit 1 in GtaThread+0x154

BG_*

NativeDB Introduced: v323
```



pub fn _0x760910b49d2b98ea_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x760910B49D2B98EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x760910B49D2B98EAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x760910b49d2b98ea_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x760910B49D2B98EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x760910B49D2B98EAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Updates the display of the MP/SP loading buttons, and locks the state so that other options are not displayed or changed. This can only be done once.



pub fn _lock_loading_screen_buttons_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1577667C3708F9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1577667C3708F9Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _lock_loading_screen_buttons_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1577667C3708F9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1577667C3708F9Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

