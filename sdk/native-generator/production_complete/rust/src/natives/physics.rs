//! PHYSICS native functions
//! 
//! Functions for the physics category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn rope_get_distance_between_ends_safe(
        
        
            ropeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73040398DFF9A4A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73040398DFF9A4A6u64;
        
        let result = invoke_raw!(
            hash,
                ropeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn rope_get_distance_between_ends_raw(
        ropeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73040398DFF9A4A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73040398DFF9A4A6u64;

        invoke_raw_typed!(
            hash,
                ropeId
        )
    }
}

/// ## Parameters
*



pub fn set_disable_frag_damage_safe(
        
        
            object: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01BA3AED21C16CFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01BA3AED21C16CFBu64;
        
        let result = invoke_raw!(
            hash,
                object, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_disable_frag_damage_raw(
        object: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01BA3AED21C16CFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01BA3AED21C16CFBu64;

        invoke_raw_typed!(
            hash,
                object, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn rope_draw_shadow_enabled_safe(
        
        
            ropeId: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF159A63806BB5BA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF159A63806BB5BA8u64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn rope_draw_shadow_enabled_raw(
        ropeId: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF159A63806BB5BA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF159A63806BB5BA8u64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn detach_rope_from_entity_safe(
        
        
            ropeId: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCF3026912A8647Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCF3026912A8647Du64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn detach_rope_from_entity_raw(
        ropeId: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCF3026912A8647Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCF3026912A8647Du64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                entity
        )
    }
}

/// ## Parameters
*



pub fn set_disable_breaking_safe(
        
        
            object: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CEC1A84620E7D5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CEC1A84620E7D5Bu64;
        
        let result = invoke_raw!(
            hash,
                object, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_disable_breaking_raw(
        object: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CEC1A84620E7D5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CEC1A84620E7D5Bu64;

        invoke_raw_typed!(
            hash,
                object, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0xb1b6216ca2e7b55e_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1B6216CA2E7B55Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1B6216CA2E7B55Eu64;
        
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
pub fn _0xb1b6216ca2e7b55e_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1B6216CA2E7B55Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1B6216CA2E7B55Eu64;

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



pub fn stop_rope_winding_safe(
        
        
            ropeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB2D4AB84A19AA7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB2D4AB84A19AA7Cu64;
        
        let result = invoke_raw!(
            hash,
                ropeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_rope_winding_raw(
        ropeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB2D4AB84A19AA7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB2D4AB84A19AA7Cu64;

        invoke_raw_typed!(
            hash,
                ropeId
        )
    }
}

/// ```
Reset a rope to a certain length.  
```



pub fn rope_reset_length_safe(
        
        
            ropeId: 
        , 
        
        
            length: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC16DE94D9BEA14A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC16DE94D9BEA14A0u64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                length
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn rope_reset_length_raw(
        ropeId: , 
        length: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC16DE94D9BEA14A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC16DE94D9BEA14A0u64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                length
        )
    }
}

/// ```
SET_*
```



pub fn _0x9ebd751e5787baf2_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9EBD751E5787BAF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9EBD751E5787BAF2u64;
        
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
pub fn _0x9ebd751e5787baf2_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9EBD751E5787BAF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9EBD751E5787BAF2u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v3407
```



pub fn get_damping_safe(
        
        
            entity: 
        , 
        
        
            type: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C520A929415BCD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C520A929415BCD2u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                type
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_damping_raw(
        entity: , 
        type: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C520A929415BCD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C520A929415BCD2u64;

        invoke_raw_typed!(
            hash,
                entity, 
                type
        )
    }
}

/// ## Parameters
*



pub fn start_rope_unwinding_front_safe(
        
        
            ropeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x538D1179EC1AA9A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x538D1179EC1AA9A9u64;
        
        let result = invoke_raw!(
            hash,
                ropeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_rope_unwinding_front_raw(
        ropeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x538D1179EC1AA9A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x538D1179EC1AA9A9u64;

        invoke_raw_typed!(
            hash,
                ropeId
        )
    }
}

/// ## Parameters
*



pub fn start_rope_winding_safe(
        
        
            ropeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1461C72C889E343Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1461C72C889E343Eu64;
        
        let result = invoke_raw!(
            hash,
                ropeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_rope_winding_raw(
        ropeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1461C72C889E343Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1461C72C889E343Eu64;

        invoke_raw_typed!(
            hash,
                ropeId
        )
    }
}

/// ```
The position supplied can be anywhere, and the entity should anchor relative to that point from it's origin.  
```



pub fn attach_rope_to_entity_safe(
        
        
            ropeId: 
        , 
        
        
            entity: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B490A6832559A65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B490A6832559A65u64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                entity, 
                x, 
                y, 
                z, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn attach_rope_to_entity_raw(
        ropeId: , 
        entity: , 
        x: , 
        y: , 
        z: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B490A6832559A65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B490A6832559A65u64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                entity, 
                x, 
                y, 
                z, 
                p5
        )
    }
}

/// ```
GET_*
```



pub fn _get_has_object_frag_inst_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C112765300C7E1Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C112765300C7E1Eu64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_has_object_frag_inst_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C112765300C7E1Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C112765300C7E1Eu64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ```
Rope presets can be found in the gamefiles. One example is "ropeFamily3", it is NOT a hash but rather a string.
```



pub fn load_rope_data_safe(
        
        
            ropeId: 
        , 
        
        
            rope_preset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCBB203C04D1ABD27u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCBB203C04D1ABD27u64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                rope_preset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn load_rope_data_raw(
        ropeId: , 
        rope_preset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCBB203C04D1ABD27u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCBB203C04D1ABD27u64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                rope_preset
        )
    }
}

/// ## Parameters
*



pub fn delete_child_rope_safe(
        
        
            ropeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA5D6B1888E4DB20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA5D6B1888E4DB20u64;
        
        let result = invoke_raw!(
            hash,
                ropeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn delete_child_rope_raw(
        ropeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA5D6B1888E4DB20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA5D6B1888E4DB20u64;

        invoke_raw_typed!(
            hash,
                ropeId
        )
    }
}

/// ## Parameters
*



pub fn _0x84de3b5fb3e666f0_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84DE3B5FB3E666F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84DE3B5FB3E666F0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x84de3b5fb3e666f0_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84DE3B5FB3E666F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84DE3B5FB3E666F0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Forces a rope to a certain length.
```



pub fn rope_force_length_safe(
        
        
            ropeId: 
        , 
        
        
            length: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD009F759A723DB1Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD009F759A723DB1Bu64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                length
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn rope_force_length_raw(
        ropeId: , 
        length: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD009F759A723DB1Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD009F759A723DB1Bu64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                length
        )
    }
}

/// ## Parameters
*



pub fn rope_convert_to_simple_safe(
        
        
            ropeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5389D48EFA2F079Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5389D48EFA2F079Au64;
        
        let result = invoke_raw!(
            hash,
                ropeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn rope_convert_to_simple_raw(
        ropeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5389D48EFA2F079Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5389D48EFA2F079Au64;

        invoke_raw_typed!(
            hash,
                ropeId
        )
    }
}

/// ```
ROPE_*
```



pub fn _0x36ccb9be67b970fd_safe(
        
        
            ropeId: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36CCB9BE67B970FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36CCB9BE67B970FDu64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x36ccb9be67b970fd_raw(
        ropeId: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36CCB9BE67B970FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36CCB9BE67B970FDu64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                p1
        )
    }
}

/// ```
Most likely ROPE_ATTACH_*  
```



pub fn _0xbc0ce682d4d05650_safe(
        
        
            ropeId: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        , 
        
        
            p10: 
        , 
        
        
            p11: 
        , 
        
        
            p12: 
        , 
        
        
            p13: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC0CE682D4D05650u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC0CE682D4D05650u64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9, 
                p10, 
                p11, 
                p12, 
                p13
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xbc0ce682d4d05650_raw(
        ropeId: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: , 
        p10: , 
        p11: , 
        p12: , 
        p13: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC0CE682D4D05650u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC0CE682D4D05650u64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9, 
                p10, 
                p11, 
                p12, 
                p13
        )
    }
}

/// ## Parameters
*



pub fn break_entity_glass_safe(
        
        
            entity: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        , 
        
        
            p10: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E648D16F6E308F3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E648D16F6E308F3u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9, 
                p10
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn break_entity_glass_raw(
        entity: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: , 
        p10: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E648D16F6E308F3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E648D16F6E308F3u64;

        invoke_raw_typed!(
            hash,
                entity, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9, 
                p10
        )
    }
}

/// ## Parameters
*



pub fn set_cg_at_boundcenter_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE520D9761FF811Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE520D9761FF811Fu64;
        
        let result = invoke_raw!(
            hash,
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cg_at_boundcenter_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE520D9761FF811Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE520D9761FF811Fu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn rope_set_update_pinverts_safe(
        
        
            ropeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8D667EE52114ABAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8D667EE52114ABAu64;
        
        let result = invoke_raw!(
            hash,
                ropeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn rope_set_update_pinverts_raw(
        ropeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8D667EE52114ABAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8D667EE52114ABAu64;

        invoke_raw_typed!(
            hash,
                ropeId
        )
    }
}

/// ## Parameters
*



pub fn get_rope_vertex_coord_safe(
        
        
            ropeId: 
        , 
        
        
            vertex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA61CA8E80F09E4Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA61CA8E80F09E4Du64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                vertex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_rope_vertex_coord_raw(
        ropeId: , 
        vertex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA61CA8E80F09E4Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA61CA8E80F09E4Du64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                vertex
        )
    }
}

/// Return if the rope was generated or not by the script where the native is called.



pub fn _does_rope_belong_to_this_script_safe(
        
        
            ropeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x271C9D3ACA5D6409u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x271C9D3ACA5D6409u64;
        
        let result = invoke_raw!(
            hash,
                ropeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _does_rope_belong_to_this_script_raw(
        ropeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x271C9D3ACA5D6409u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x271C9D3ACA5D6409u64;

        invoke_raw_typed!(
            hash,
                ropeId
        )
    }
}

/// ```
RESET_*  
```



pub fn _0xcc6e963682533882_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC6E963682533882u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC6E963682533882u64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xcc6e963682533882_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC6E963682533882u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC6E963682533882u64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ## Parameters
*



pub fn pin_rope_vertex_safe(
        
        
            ropeId: 
        , 
        
        
            vertex: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B320CF14146B69Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B320CF14146B69Au64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                vertex, 
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
pub fn pin_rope_vertex_raw(
        ropeId: , 
        vertex: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B320CF14146B69Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B320CF14146B69Au64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                vertex, 
                x, 
                y, 
                z
        )
    }
}

/// ```
Unloads rope textures for all ropes in the current scene.
```



pub fn rope_unload_textures_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CE36C35C1AC8163u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CE36C35C1AC8163u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn rope_unload_textures_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CE36C35C1AC8163u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CE36C35C1AC8163u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// For an example on how to use this native please refer to [ADD_ROPE](#_0xE832D760399EB220)



pub fn does_rope_exist_safe(
        
        
            ropeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD5448BE3111ED96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD5448BE3111ED96u64;
        
        let result = invoke_raw!(
            hash,
                ropeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_rope_exist_raw(
        ropeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD5448BE3111ED96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD5448BE3111ED96u64;

        invoke_raw_typed!(
            hash,
                ropeId
        )
    }
}

/// ROPE_*

```
NativeDB Introduced: v1868
```



pub fn _0xa1ae736541b0fca3_safe(
        
        
            ropeId: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1AE736541B0FCA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1AE736541B0FCA3u64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa1ae736541b0fca3_raw(
        ropeId: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1AE736541B0FCA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1AE736541B0FCA3u64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                p1
        )
    }
}

/// ```
Loads rope textures for all ropes in the current scene.
```



pub fn rope_load_textures_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B9039DBF2D258C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B9039DBF2D258C1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn rope_load_textures_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B9039DBF2D258C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B9039DBF2D258C1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Deletes the rope with the specified handle.

You should check if the rope exists before trying to delete it, see [DOES_ROPE_EXIST](#_0xFD5448BE3111ED96).

For an example on how to use this native please refer to [ADD_ROPE](#_0xE832D760399EB220)



pub fn delete_rope_safe(
        
        
            ropeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52B4829281364649u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52B4829281364649u64;
        
        let result = invoke_raw!(
            hash,
                ropeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn delete_rope_raw(
        ropeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52B4829281364649u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52B4829281364649u64;

        invoke_raw_typed!(
            hash,
                ropeId
        )
    }
}

/// ## Parameters
*



pub fn get_rope_last_vertex_coord_safe(
        
        
            ropeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21BB0FBD3E217C2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21BB0FBD3E217C2Du64;
        
        let result = invoke_raw!(
            hash,
                ropeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_rope_last_vertex_coord_raw(
        ropeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21BB0FBD3E217C2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21BB0FBD3E217C2Du64;

        invoke_raw_typed!(
            hash,
                ropeId
        )
    }
}

/// ## Return value



pub fn rope_are_textures_loaded_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2D0E6A75CC05597u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2D0E6A75CC05597u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn rope_are_textures_loaded_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2D0E6A75CC05597u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2D0E6A75CC05597u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Attaches entity 1 to entity 2.  
```



pub fn attach_entities_to_rope_safe(
        
        
            ropeId: 
        , 
        
        
            ent1: 
        , 
        
        
            ent2: 
        , 
        
        
            ent1_x: 
        , 
        
        
            ent1_y: 
        , 
        
        
            ent1_z: 
        , 
        
        
            ent2_x: 
        , 
        
        
            ent2_y: 
        , 
        
        
            ent2_z: 
        , 
        
        
            length: 
        , 
        
        
            p10: 
        , 
        
        
            p11: 
        , 
        
        
            boneName1: 
        , 
        
        
            boneName2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D95EC8B6D940AC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D95EC8B6D940AC3u64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                ent1, 
                ent2, 
                ent1_x, 
                ent1_y, 
                ent1_z, 
                ent2_x, 
                ent2_y, 
                ent2_z, 
                length, 
                p10, 
                p11, 
                boneName1, 
                boneName2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn attach_entities_to_rope_raw(
        ropeId: , 
        ent1: , 
        ent2: , 
        ent1_x: , 
        ent1_y: , 
        ent1_z: , 
        ent2_x: , 
        ent2_y: , 
        ent2_z: , 
        length: , 
        p10: , 
        p11: , 
        boneName1: , 
        boneName2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D95EC8B6D940AC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D95EC8B6D940AC3u64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                ent1, 
                ent2, 
                ent1_x, 
                ent1_y, 
                ent1_z, 
                ent2_x, 
                ent2_y, 
                ent2_z, 
                length, 
                p10, 
                p11, 
                boneName1, 
                boneName2
        )
    }
}

/// ```
Creates a rope at the specific position, that extends in the specified direction when not attached to any entities.
__
Rope does NOT interact with anything you attach it to, in some cases it make interact with the world AFTER it breaks (seems to occur if you set the type to -1).
Rope will sometimes contract and fall to the ground like you'd expect it to, but since it doesn't interact with the world the effect is just jaring.
```

There are 8 different rope types in the base game. Full rope data can be found in `ropedata.xml`.

```c
enum ePhysicsRopeType {
    RopeThin = 0, // Verticies: 1, Radius: 0.03, Textures: rope & rope_n
    RopeWire6 = 1, // Verticies: 4, Radius: 0.015, Textures: steel_cable & steel_cable_n
    RopeWire32 = 2, // Verticies: 32, Radius: 0.025, Textures: steel_cable & steel_cable_n
    RopeMesh = 3, // Verticies: 6, Radius: 0.03, Textures: rope & rope_n
    RopeThinWire32 = 4, // Verticies: 32, Radius: 0.01, Textures: rope & rope_n
    RopeReins = 5, // Verticies: 32, Radius: 0.005, Textures: rope & rope_n
    RopeThin4 = 6, // Verticies: 4, Radius: 0.03, Textures: rope & rope_n
    RopeWire64 = 7 // Verticies: 64, Radius: 0.025, Textures: steel_cable & steel_cable_n
}
```



pub fn add_rope_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            maxLength: 
        , 
        
        
            ropeType: 
        , 
        
        
            initLength: 
        , 
        
        
            minLength: 
        , 
        
        
            lengthChangeRate: 
        , 
        
        
            onlyPPU: 
        , 
        
        
            collisionOn: 
        , 
        
        
            lockFromFront: 
        , 
        
        
            timeMultiplier: 
        , 
        
        
            breakable: 
        , 
        
        
            unkPtr: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE832D760399EB220u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE832D760399EB220u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                rotX, 
                rotY, 
                rotZ, 
                maxLength, 
                ropeType, 
                initLength, 
                minLength, 
                lengthChangeRate, 
                onlyPPU, 
                collisionOn, 
                lockFromFront, 
                timeMultiplier, 
                breakable, 
                unkPtr
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_rope_raw(
        x: , 
        y: , 
        z: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        maxLength: , 
        ropeType: , 
        initLength: , 
        minLength: , 
        lengthChangeRate: , 
        onlyPPU: , 
        collisionOn: , 
        lockFromFront: , 
        timeMultiplier: , 
        breakable: , 
        unkPtr: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE832D760399EB220u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE832D760399EB220u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                rotX, 
                rotY, 
                rotZ, 
                maxLength, 
                ropeType, 
                initLength, 
                minLength, 
                lengthChangeRate, 
                onlyPPU, 
                collisionOn, 
                lockFromFront, 
                timeMultiplier, 
                breakable, 
                unkPtr
        )
    }
}

/// Related to the lower-end of a vehicles fTractionCurve, e.g., from standing starts and acceleration from low/zero speeds.

```
NativeDB Introduced: v1604
```



pub fn _set_launch_control_enabled_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA6A6098851C396Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA6A6098851C396Fu64;
        
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
pub fn _set_launch_control_enabled_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA6A6098851C396Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA6A6098851C396Fu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_cgoffset_safe(
        
        
            entity: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8FA3908D7B86904u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8FA3908D7B86904u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
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
pub fn set_cgoffset_raw(
        entity: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8FA3908D7B86904u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8FA3908D7B86904u64;

        invoke_raw_typed!(
            hash,
                entity, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn _set_entity_proof_unk_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15F944730C832252u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15F944730C832252u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_entity_proof_unk_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15F944730C832252u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15F944730C832252u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ```
ROPE_*
```



pub fn _0xb743f735c03d7810_safe(
        
        
            ropeId: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB743F735C03D7810u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB743F735C03D7810u64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb743f735c03d7810_raw(
        ropeId: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB743F735C03D7810u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB743F735C03D7810u64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn get_rope_vertex_count_safe(
        
        
            ropeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3655F544CD30F0B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3655F544CD30F0B5u64;
        
        let result = invoke_raw!(
            hash,
                ropeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_rope_vertex_count_raw(
        ropeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3655F544CD30F0B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3655F544CD30F0B5u64;

        invoke_raw_typed!(
            hash,
                ropeId
        )
    }
}

/// ## Parameters
*



pub fn stop_rope_unwinding_front_safe(
        
        
            ropeId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFFF3A50779EFBBB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFFF3A50779EFBBB3u64;
        
        let result = invoke_raw!(
            hash,
                ropeId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_rope_unwinding_front_raw(
        ropeId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFFF3A50779EFBBB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFFF3A50779EFBBB3u64;

        invoke_raw_typed!(
            hash,
                ropeId
        )
    }
}

/// ## Parameters
*



pub fn unpin_rope_vertex_safe(
        
        
            ropeId: 
        , 
        
        
            vertex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B5AE2EEE4A8F180u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B5AE2EEE4A8F180u64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                vertex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn unpin_rope_vertex_raw(
        ropeId: , 
        vertex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B5AE2EEE4A8F180u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B5AE2EEE4A8F180u64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                vertex
        )
    }
}

/// ## Parameters
*



pub fn activate_physics_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x710311ADF0E20730u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x710311ADF0E20730u64;
        
        let result = invoke_raw!(
            hash,
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn activate_physics_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x710311ADF0E20730u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x710311ADF0E20730u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn get_cgoffset_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8214A4B5A7A33612u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8214A4B5A7A33612u64;
        
        let result = invoke_raw!(
            hash,
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cgoffset_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8214A4B5A7A33612u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8214A4B5A7A33612u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn set_damping_safe(
        
        
            entity: 
        , 
        
        
            vertex: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEA3B200A6FEB65Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEA3B200A6FEB65Bu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                vertex, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_damping_raw(
        entity: , 
        vertex: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEA3B200A6FEB65Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEA3B200A6FEB65Bu64;

        invoke_raw_typed!(
            hash,
                entity, 
                vertex, 
                value
        )
    }
}

/// ## Parameters
*



pub fn rope_set_update_order_safe(
        
        
            ropeId: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC57A637A20006EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC57A637A20006EDu64;
        
        let result = invoke_raw!(
            hash,
                ropeId, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn rope_set_update_order_raw(
        ropeId: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC57A637A20006EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC57A637A20006EDu64;

        invoke_raw_typed!(
            hash,
                ropeId, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn apply_impulse_to_cloth_safe(
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            vecX: 
        , 
        
        
            vecY: 
        , 
        
        
            vecZ: 
        , 
        
        
            impulse: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE37F721824571784u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE37F721824571784u64;
        
        let result = invoke_raw!(
            hash,
                posX, 
                posY, 
                posZ, 
                vecX, 
                vecY, 
                vecZ, 
                impulse
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn apply_impulse_to_cloth_raw(
        posX: , 
        posY: , 
        posZ: , 
        vecX: , 
        vecY: , 
        vecZ: , 
        impulse: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE37F721824571784u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE37F721824571784u64;

        invoke_raw_typed!(
            hash,
                posX, 
                posY, 
                posZ, 
                vecX, 
                vecY, 
                vecZ, 
                impulse
        )
    }
}

