//! EVENT native functions
//! 
//! Functions for the event category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// SUPPRESS_SHOCKING_EVENTS_NEXT_FRAME native function



pub fn suppress_shocking_events_next_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F9A292AD0A3BD89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F9A292AD0A3BD89u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn suppress_shocking_events_next_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F9A292AD0A3BD89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F9A292AD0A3BD89u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_decision_maker_safe(
        
        
            ped: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB604A2942ADED0EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB604A2942ADED0EEu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_decision_maker_raw(
        ped: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB604A2942ADED0EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB604A2942ADED0EEu64;

        invoke_raw_typed!(
            hash,
                ped, 
                name
        )
    }
}

/// ```
eventType: https://alloc8or.re/gta5/doc/enums/eEventType.txt
```



pub fn suppress_shocking_event_type_next_frame_safe(
        
        
            eventType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FD2EC8BF1F1CF30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FD2EC8BF1F1CF30u64;
        
        let result = invoke_raw!(
            hash,
                eventType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn suppress_shocking_event_type_next_frame_raw(
        eventType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FD2EC8BF1F1CF30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FD2EC8BF1F1CF30u64;

        invoke_raw_typed!(
            hash,
                eventType
        )
    }
}

/// REMOVE_SHOCKING_EVENT_SPAWN_BLOCKING_AREAS native function



pub fn remove_shocking_event_spawn_blocking_areas_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x340F1415B68AEADEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x340F1415B68AEADEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_shocking_event_spawn_blocking_areas_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x340F1415B68AEADEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x340F1415B68AEADEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
eventType: https://alloc8or.re/gta5/doc/enums/eEventType.txt
```



pub fn add_shocking_event_at_position_safe(
        
        
            eventType: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9F8455409B525E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9F8455409B525E9u64;
        
        let result = invoke_raw!(
            hash,
                eventType, 
                x, 
                y, 
                z, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_shocking_event_at_position_raw(
        eventType: , 
        x: , 
        y: , 
        z: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9F8455409B525E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9F8455409B525E9u64;

        invoke_raw_typed!(
            hash,
                eventType, 
                x, 
                y, 
                z, 
                duration
        )
    }
}

/// ```
eventType: https://alloc8or.re/gta5/doc/enums/eEventType.txt
This is limited to 4 blocked events at a time.
```



pub fn block_decision_maker_event_safe(
        
        
            name: 
        , 
        
        
            eventType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE42FCDFD0E4196F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE42FCDFD0E4196F7u64;
        
        let result = invoke_raw!(
            hash,
                name, 
                eventType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn block_decision_maker_event_raw(
        name: , 
        eventType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE42FCDFD0E4196F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE42FCDFD0E4196F7u64;

        invoke_raw_typed!(
            hash,
                name, 
                eventType
        )
    }
}

/// ## Parameters
*



pub fn remove_all_shocking_events_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAABE8FDFA21274Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAABE8FDFA21274Cu64;
        
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
pub fn remove_all_shocking_events_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAABE8FDFA21274Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAABE8FDFA21274Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
eventType: https://alloc8or.re/gta5/doc/enums/eEventType.txt
```



pub fn is_shocking_event_in_sphere_safe(
        
        
            eventType: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1374ABB7C15BAB92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1374ABB7C15BAB92u64;
        
        let result = invoke_raw!(
            hash,
                eventType, 
                x, 
                y, 
                z, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_shocking_event_in_sphere_raw(
        eventType: , 
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1374ABB7C15BAB92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1374ABB7C15BAB92u64;

        invoke_raw_typed!(
            hash,
                eventType, 
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// ```
eventType: https://alloc8or.re/gta5/doc/enums/eEventType.txt
```



pub fn unblock_decision_maker_event_safe(
        
        
            name: 
        , 
        
        
            eventType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7CD9CF34F2C99E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7CD9CF34F2C99E8u64;
        
        let result = invoke_raw!(
            hash,
                name, 
                eventType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn unblock_decision_maker_event_raw(
        name: , 
        eventType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7CD9CF34F2C99E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7CD9CF34F2C99E8u64;

        invoke_raw_typed!(
            hash,
                name, 
                eventType
        )
    }
}

/// SUPPRESS_AGITATION_EVENTS_NEXT_FRAME native function



pub fn suppress_agitation_events_next_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F3B7749C112D552u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F3B7749C112D552u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn suppress_agitation_events_next_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F3B7749C112D552u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F3B7749C112D552u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn remove_shocking_event_safe(
        
        
            event: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CDA538C44C6CCE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CDA538C44C6CCE5u64;
        
        let result = invoke_raw!(
            hash,
                event
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_shocking_event_raw(
        event: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CDA538C44C6CCE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CDA538C44C6CCE5u64;

        invoke_raw_typed!(
            hash,
                event
        )
    }
}

/// ```
eventType: https://alloc8or.re/gta5/doc/enums/eEventType.txt
```



pub fn add_shocking_event_for_entity_safe(
        
        
            eventType: 
        , 
        
        
            entity: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FD8F3BE76F89422u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FD8F3BE76F89422u64;
        
        let result = invoke_raw!(
            hash,
                eventType, 
                entity, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_shocking_event_for_entity_raw(
        eventType: , 
        entity: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FD8F3BE76F89422u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FD8F3BE76F89422u64;

        invoke_raw_typed!(
            hash,
                eventType, 
                entity, 
                duration
        )
    }
}

/// ```
eventType: https://alloc8or.re/gta5/doc/enums/eEventType.txt
```



pub fn clear_decision_maker_event_response_safe(
        
        
            name: 
        , 
        
        
            eventType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4FC9381A7AEE8968u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4FC9381A7AEE8968u64;
        
        let result = invoke_raw!(
            hash,
                name, 
                eventType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_decision_maker_event_response_raw(
        name: , 
        eventType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4FC9381A7AEE8968u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4FC9381A7AEE8968u64;

        invoke_raw_typed!(
            hash,
                name, 
                eventType
        )
    }
}

