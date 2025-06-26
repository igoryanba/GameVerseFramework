//! INTERIOR native functions
//! 
//! Functions for the interior category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn _enable_script_cull_model_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50C375537449F369u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50C375537449F369u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _enable_script_cull_model_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50C375537449F369u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50C375537449F369u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0x7ecdf98587e92dec_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7ECDF98587E92DECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7ECDF98587E92DECu64;
        
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
pub fn _0x7ecdf98587e92dec_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7ECDF98587E92DECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7ECDF98587E92DECu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn get_room_key_for_game_viewport_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6575914D2A0B450u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6575914D2A0B450u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_room_key_for_game_viewport_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6575914D2A0B450u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6575914D2A0B450u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns the handle of the interior that the entity is in. Returns 0 if outside.  
```



pub fn get_interior_from_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2107BA504071A6BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2107BA504071A6BBu64;
        
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
pub fn get_interior_from_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2107BA504071A6BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2107BA504071A6BBu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Does something similar to INTERIOR::DISABLE_INTERIOR.  
You don't fall through the floor but everything is invisible inside and looks the same as when INTERIOR::DISABLE_INTERIOR is used. Peds behaves normally inside.  
```



pub fn unpin_interior_safe(
        
        
            interior: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x261CCE7EED010641u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x261CCE7EED010641u64;
        
        let result = invoke_raw!(
            hash,
                interior
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn unpin_interior_raw(
        interior: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x261CCE7EED010641u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x261CCE7EED010641u64;

        invoke_raw_typed!(
            hash,
                interior
        )
    }
}

/// ## Return value



pub fn is_interior_scene_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC72B5D7A1CBD54Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC72B5D7A1CBD54Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_interior_scene_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC72B5D7A1CBD54Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC72B5D7A1CBD54Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Exemple of use(carmod_shop.c4)  
INTERIOR::_AF348AFCB575A441("V_CarModRoom");  
```



pub fn _0xaf348afcb575a441_safe(
        
        
            roomName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF348AFCB575A441u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF348AFCB575A441u64;
        
        let result = invoke_raw!(
            hash,
                roomName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xaf348afcb575a441_raw(
        roomName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF348AFCB575A441u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF348AFCB575A441u64;

        invoke_raw_typed!(
            hash,
                roomName
        )
    }
}

/// ## Parameters
*



pub fn is_valid_interior_safe(
        
        
            interior: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26B0E73D7EAAF4D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26B0E73D7EAAF4D3u64;
        
        let result = invoke_raw!(
            hash,
                interior
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_valid_interior_raw(
        interior: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26B0E73D7EAAF4D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26B0E73D7EAAF4D3u64;

        invoke_raw_typed!(
            hash,
                interior
        )
    }
}

/// Returns true if the collision at the specified coords is marked as being outside (false if there's an interior)



pub fn is_collision_marked_outside_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEA5AC2EDA7C33E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEA5AC2EDA7C33E8u64;
        
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
pub fn is_collision_marked_outside_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEA5AC2EDA7C33E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEA5AC2EDA7C33E8u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ```
Usage: INTERIOR::_0x405DC2AEF6AF95B9(INTERIOR::GET_KEY_FOR_ENTITY_IN_ROOM(PLAYER::PLAYER_PED_ID()));  
```



pub fn _0x405dc2aef6af95b9_safe(
        
        
            roomHashKey: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x405DC2AEF6AF95B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x405DC2AEF6AF95B9u64;
        
        let result = invoke_raw!(
            hash,
                roomHashKey
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x405dc2aef6af95b9_raw(
        roomHashKey: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x405DC2AEF6AF95B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x405DC2AEF6AF95B9u64;

        invoke_raw_typed!(
            hash,
                roomHashKey
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _0x38c1cb1cb119a016_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38C1CB1CB119A016u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38C1CB1CB119A016u64;
        
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
pub fn _0x38c1cb1cb119a016_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38C1CB1CB119A016u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38C1CB1CB119A016u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Only used once in the entire game scripts.
Does not actually return anything.
```



pub fn _0x4c2330e61d3deb56_safe(
        
        
            interior: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C2330E61D3DEB56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C2330E61D3DEB56u64;
        
        let result = invoke_raw!(
            hash,
                interior
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4c2330e61d3deb56_raw(
        interior: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C2330E61D3DEB56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C2330E61D3DEB56u64;

        invoke_raw_typed!(
            hash,
                interior
        )
    }
}

/// ## Parameters
*



pub fn pin_interior_in_memory_safe(
        
        
            interior: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CA429C029CCF247u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CA429C029CCF247u64;
        
        let result = invoke_raw!(
            hash,
                interior
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pin_interior_in_memory_raw(
        interior: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CA429C029CCF247u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CA429C029CCF247u64;

        invoke_raw_typed!(
            hash,
                interior
        )
    }
}

/// ## Parameters
*



pub fn is_interior_entity_set_active_safe(
        
        
            interior: 
        , 
        
        
            entitySetName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35F7DD45E8C0A16Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35F7DD45E8C0A16Du64;
        
        let result = invoke_raw!(
            hash,
                interior, 
                entitySetName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_interior_entity_set_active_raw(
        interior: , 
        entitySetName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35F7DD45E8C0A16Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35F7DD45E8C0A16Du64;

        invoke_raw_typed!(
            hash,
                interior, 
                entitySetName
        )
    }
}

/// ```
More info: http://gtaforums.com/topic/836367-adding-props-to-interiors/  
```



pub fn activate_interior_entity_set_safe(
        
        
            interior: 
        , 
        
        
            entitySetName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55E86AF2712B36A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55E86AF2712B36A1u64;
        
        let result = invoke_raw!(
            hash,
                interior, 
                entitySetName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn activate_interior_entity_set_raw(
        interior: , 
        entitySetName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55E86AF2712B36A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55E86AF2712B36A1u64;

        invoke_raw_typed!(
            hash,
                interior, 
                entitySetName
        )
    }
}

/// ```
Forces the particular room in an interior to load incase not teleporting into the portal.
```



pub fn force_room_for_entity_safe(
        
        
            entity: 
        , 
        
        
            interior: 
        , 
        
        
            roomHashKey: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52923C4710DD9907u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52923C4710DD9907u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                interior, 
                roomHashKey
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_room_for_entity_raw(
        entity: , 
        interior: , 
        roomHashKey: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52923C4710DD9907u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52923C4710DD9907u64;

        invoke_raw_typed!(
            hash,
                entity, 
                interior, 
                roomHashKey
        )
    }
}

/// ```
Returns the group ID of the specified interior. For example, regular interiors have group 0, subway interiors have group 1. There are a few other groups too.  
```



pub fn get_interior_group_id_safe(
        
        
            interior: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4A84ABF135EF91Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4A84ABF135EF91Au64;
        
        let result = invoke_raw!(
            hash,
                interior
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_interior_group_id_raw(
        interior: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4A84ABF135EF91Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4A84ABF135EF91Au64;

        invoke_raw_typed!(
            hash,
                interior
        )
    }
}

/// ```
Jenkins hash _might_ be 0xFC227584.
```



pub fn _0x7241ccb7d020db69_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7241CCB7D020DB69u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7241CCB7D020DB69u64;
        
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
pub fn _0x7241ccb7d020db69_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7241CCB7D020DB69u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7241CCB7D020DB69u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ```
Gets the room hash key from the room that the specified entity is in. Each room in every interior has a unique key. Returns 0 if the entity is outside.  
```



pub fn get_room_key_from_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47C2A06D4F5F424Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47C2A06D4F5F424Bu64;
        
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
pub fn get_room_key_from_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47C2A06D4F5F424Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47C2A06D4F5F424Bu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn _set_interior_entity_set_color_safe(
        
        
            interior: 
        , 
        
        
            entitySetName: 
        , 
        
        
            color: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1F1920BAF281317u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1F1920BAF281317u64;
        
        let result = invoke_raw!(
            hash,
                interior, 
                entitySetName, 
                color
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_interior_entity_set_color_raw(
        interior: , 
        entitySetName: , 
        color: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1F1920BAF281317u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1F1920BAF281317u64;

        invoke_raw_typed!(
            hash,
                interior, 
                entitySetName, 
                color
        )
    }
}

/// ## Parameters
*



pub fn get_interior_from_collision_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC4CF9FCB29A4424u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC4CF9FCB29A4424u64;
        
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
pub fn get_interior_from_collision_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC4CF9FCB29A4424u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC4CF9FCB29A4424u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ```
Hashed version of GET_INTERIOR_AT_COORDS_WITH_TYPE
```



pub fn get_interior_at_coords_with_typehash_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0F77ADB9F67E79Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0F77ADB9F67E79Du64;
        
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
pub fn get_interior_at_coords_with_typehash_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0F77ADB9F67E79Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0F77ADB9F67E79Du64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// _0x483ACA1176CA93F1 native function



pub fn _0x483aca1176ca93f1_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x483ACA1176CA93F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x483ACA1176CA93F1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x483aca1176ca93f1_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x483ACA1176CA93F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x483ACA1176CA93F1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns the interior ID representing the requested interior at that location (if found?). The supplied interior string is not the same as the one used to load the interior.  
Use: INTERIOR::UNPIN_INTERIOR(INTERIOR::GET_INTERIOR_AT_COORDS_WITH_TYPE(x, y, z, interior))  
Interior types include: "V_Michael", "V_Franklins", "V_Franklinshouse", etc.. you can find them in the scripts.  
Not a very useful native as you could just use GET_INTERIOR_AT_COORDS instead and get the same result, without even having to specify the interior type.  
```



pub fn get_interior_at_coords_with_type_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            interiorType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x05B7A89BD78797FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x05B7A89BD78797FCu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                interiorType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_interior_at_coords_with_type_raw(
        x: , 
        y: , 
        z: , 
        interiorType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x05B7A89BD78797FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x05B7A89BD78797FCu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                interiorType
        )
    }
}

/// ## Parameters
*



pub fn deactivate_interior_entity_set_safe(
        
        
            interior: 
        , 
        
        
            entitySetName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x420BD37289EEE162u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x420BD37289EEE162u64;
        
        let result = invoke_raw!(
            hash,
                interior, 
                entitySetName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn deactivate_interior_entity_set_raw(
        interior: , 
        entitySetName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x420BD37289EEE162u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x420BD37289EEE162u64;

        invoke_raw_typed!(
            hash,
                interior, 
                entitySetName
        )
    }
}

/// ```
Does something similar to INTERIOR::DISABLE_INTERIOR  
```



pub fn cap_interior_safe(
        
        
            interiorID: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9175F941610DB54u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9175F941610DB54u64;
        
        let result = invoke_raw!(
            hash,
                interiorID, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn cap_interior_raw(
        interiorID: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9175F941610DB54u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9175F941610DB54u64;

        invoke_raw_typed!(
            hash,
                interiorID, 
                toggle
        )
    }
}

/// Immediately removes entity from an interior. Like sets entity to `limbo` room.

```
NativeDB Introduced: v2189
```



pub fn _clear_interior_for_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85D5422B2039A70Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85D5422B2039A70Du64;
        
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
pub fn _clear_interior_for_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85D5422B2039A70Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85D5422B2039A70Du64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
This is the native that is used to hide the exterior of GTA Online apartment buildings when you are inside an apartment.
```



pub fn enable_exterior_cull_model_this_frame_safe(
        
        
            mapObjectHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA97F257D0151A6ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA97F257D0151A6ABu64;
        
        let result = invoke_raw!(
            hash,
                mapObjectHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn enable_exterior_cull_model_this_frame_raw(
        mapObjectHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA97F257D0151A6ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA97F257D0151A6ABu64;

        invoke_raw_typed!(
            hash,
                mapObjectHash
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn get_interior_from_primary_view_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7D267EC6CA966C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7D267EC6CA966C3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_interior_from_primary_view_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7D267EC6CA966C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7D267EC6CA966C3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Seems to do the exact same as INTERIOR::GET_ROOM_KEY_FROM_ENTITY  
```



pub fn get_key_for_entity_in_room_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x399685DB942336BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x399685DB942336BCu64;
        
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
pub fn get_key_for_entity_in_room_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x399685DB942336BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x399685DB942336BCu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn add_pickup_to_interior_room_by_name_safe(
        
        
            pickup: 
        , 
        
        
            roomName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F6167F351168730u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F6167F351168730u64;
        
        let result = invoke_raw!(
            hash,
                pickup, 
                roomName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_pickup_to_interior_room_by_name_raw(
        pickup: , 
        roomName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F6167F351168730u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F6167F351168730u64;

        invoke_raw_typed!(
            hash,
                pickup, 
                roomName
        )
    }
}

/// ## Parameters
*



pub fn _0x82ebb79e258fa2b7_safe(
        
        
            entity: 
        , 
        
        
            interiorID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82EBB79E258FA2B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82EBB79E258FA2B7u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                interiorID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x82ebb79e258fa2b7_raw(
        entity: , 
        interiorID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82EBB79E258FA2B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82EBB79E258FA2B7u64;

        invoke_raw_typed!(
            hash,
                entity, 
                interiorID
        )
    }
}

/// ## Parameters
*



pub fn is_interior_disabled_safe(
        
        
            interior: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC5115A5A939DD15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC5115A5A939DD15u64;
        
        let result = invoke_raw!(
            hash,
                interior
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_interior_disabled_raw(
        interior: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC5115A5A939DD15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC5115A5A939DD15u64;

        invoke_raw_typed!(
            hash,
                interior
        )
    }
}

/// ```
Example:   
This removes the interior from the strip club and when trying to walk inside the player just falls:  
INTERIOR::DISABLE_INTERIOR(118018, true);  
```



pub fn disable_interior_safe(
        
        
            interiorID: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6170941419D7D8ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6170941419D7D8ECu64;
        
        let result = invoke_raw!(
            hash,
                interiorID, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_interior_raw(
        interiorID: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6170941419D7D8ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6170941419D7D8ECu64;

        invoke_raw_typed!(
            hash,
                interiorID, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_interior_ready_safe(
        
        
            interiorID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6726BDCCC1932F0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6726BDCCC1932F0Eu64;
        
        let result = invoke_raw!(
            hash,
                interiorID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_interior_ready_raw(
        interiorID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6726BDCCC1932F0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6726BDCCC1932F0Eu64;

        invoke_raw_typed!(
            hash,
                interiorID
        )
    }
}

/// ## Parameters
*



pub fn get_offset_from_interior_in_world_coords_safe(
        
        
            interior: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E3B3E6D66F6E22Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E3B3E6D66F6E22Fu64;
        
        let result = invoke_raw!(
            hash,
                interior, 
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
pub fn get_offset_from_interior_in_world_coords_raw(
        interior: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E3B3E6D66F6E22Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E3B3E6D66F6E22Fu64;

        invoke_raw_typed!(
            hash,
                interior, 
                x, 
                y, 
                z
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn get_interior_heading_safe(
        
        
            interior: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF49B58631D9E22D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF49B58631D9E22D9u64;
        
        let result = invoke_raw!(
            hash,
                interior
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_interior_heading_raw(
        interior: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF49B58631D9E22D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF49B58631D9E22D9u64;

        invoke_raw_typed!(
            hash,
                interior
        )
    }
}

/// ## Parameters
*



pub fn is_interior_capped_safe(
        
        
            interiorID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92BAC8ACF88CEC26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92BAC8ACF88CEC26u64;
        
        let result = invoke_raw!(
            hash,
                interiorID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_interior_capped_raw(
        interiorID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92BAC8ACF88CEC26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92BAC8ACF88CEC26u64;

        invoke_raw_typed!(
            hash,
                interiorID
        )
    }
}

/// CLEAR_ROOM_FOR_GAME_VIEWPORT native function



pub fn clear_room_for_game_viewport_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23B59D8912F94246u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23B59D8912F94246u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_room_for_game_viewport_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23B59D8912F94246u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23B59D8912F94246u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn force_room_for_game_viewport_safe(
        
        
            interiorID: 
        , 
        
        
            roomHashKey: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x920D853F3E17F1DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x920D853F3E17F1DAu64;
        
        let result = invoke_raw!(
            hash,
                interiorID, 
                roomHashKey
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_room_for_game_viewport_raw(
        interiorID: , 
        roomHashKey: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x920D853F3E17F1DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x920D853F3E17F1DAu64;

        invoke_raw_typed!(
            hash,
                interiorID, 
                roomHashKey
        )
    }
}

/// ## Parameters
*



pub fn clear_room_for_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB365FC0C4E27FFA7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB365FC0C4E27FFA7u64;
        
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
pub fn clear_room_for_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB365FC0C4E27FFA7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB365FC0C4E27FFA7u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Returns interior ID from specified coordinates. If coordinates are outside, then it returns 0.  
Example for VB.NET  
Dim interiorID As Integer = Native.Function.Call(Of Integer)(Hash.GET_INTERIOR_AT_COORDS, X, Y, Z)  
```



pub fn get_interior_at_coords_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB0F7F8663821D9C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB0F7F8663821D9C3u64;
        
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
pub fn get_interior_at_coords_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB0F7F8663821D9C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB0F7F8663821D9C3u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ```
DISABLE_*
```



pub fn _0x9e6542f0ce8e70a3_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E6542F0CE8E70A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E6542F0CE8E70A3u64;
        
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
pub fn _0x9e6542f0ce8e70a3_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E6542F0CE8E70A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E6542F0CE8E70A3u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn get_interior_location_and_namehash_safe(
        
        
            interior: 
        , 
        
        
            position: 
        , 
        
        
            nameHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x252BDC06B73FA6EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x252BDC06B73FA6EAu64;
        
        let result = invoke_raw!(
            hash,
                interior, 
                position, 
                nameHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_interior_location_and_namehash_raw(
        interior: , 
        position: , 
        nameHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x252BDC06B73FA6EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x252BDC06B73FA6EAu64;

        invoke_raw_typed!(
            hash,
                interior, 
                position, 
                nameHash
        )
    }
}

/// ## Parameters
*



pub fn refresh_interior_safe(
        
        
            interiorID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41F37C3427C75AE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41F37C3427C75AE0u64;
        
        let result = invoke_raw!(
            hash,
                interiorID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn refresh_interior_raw(
        interiorID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41F37C3427C75AE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41F37C3427C75AE0u64;

        invoke_raw_typed!(
            hash,
                interiorID
        )
    }
}

