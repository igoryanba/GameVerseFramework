//! ENTITY native functions
//! 
//! Functions for the entity category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// [Animations list](https://alexguirre.github.io/animations-list/)



pub fn play_synchronized_entity_anim_safe(
        
        
            entity: 
        , 
        
        
            syncedScene: 
        , 
        
        
            animName: 
        , 
        
        
            animDictName: 
        , 
        
        
            fBlendInDelta: 
        , 
        
        
            fBlendOutDelta: 
        , 
        
        
            iFlags: 
        , 
        
        
            fMoverBlendInDelta: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC77720A12FE14A86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC77720A12FE14A86u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                syncedScene, 
                animName, 
                animDictName, 
                fBlendInDelta, 
                fBlendOutDelta, 
                iFlags, 
                fMoverBlendInDelta
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_synchronized_entity_anim_raw(
        entity: , 
        syncedScene: , 
        animName: , 
        animDictName: , 
        fBlendInDelta: , 
        fBlendOutDelta: , 
        iFlags: , 
        fMoverBlendInDelta: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC77720A12FE14A86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC77720A12FE14A86u64;

        invoke_raw_typed!(
            hash,
                entity, 
                syncedScene, 
                animName, 
                animDictName, 
                fBlendInDelta, 
                fBlendOutDelta, 
                iFlags, 
                fMoverBlendInDelta
        )
    }
}

/// Checks if entity1 has a clear line of sight to entity2. So a simple raycast which if it collides with any of the given colliderTypes returns false.

The direction of the check matters with for example bushes, so checking from inside to outside a bush with traceType 256 returns true, but the other way around returns false.



pub fn has_entity_clear_los_to_entity_safe(
        
        
            entity1: 
        , 
        
        
            entity2: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCDFF7B72D23A1ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCDFF7B72D23A1ACu64;
        
        let result = invoke_raw!(
            hash,
                entity1, 
                entity2, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_entity_clear_los_to_entity_raw(
        entity1: , 
        entity2: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCDFF7B72D23A1ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCDFF7B72D23A1ACu64;

        invoke_raw_typed!(
            hash,
                entity1, 
                entity2, 
                flags
        )
    }
}

/// ```
All ambient entities in-world seem to have the same value for the second argument (Any *script), depending on when the scripthook was activated/re-activated. I've seen numbers from ~5 to almost 70 when the value was translated with to_string. The function return value seems to always be 0.  
```



pub fn get_entity_script_safe(
        
        
            entity: 
        , 
        
        
            script: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6E9C38DB51D7748u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6E9C38DB51D7748u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                script
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_entity_script_raw(
        entity: , 
        script: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6E9C38DB51D7748u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6E9C38DB51D7748u64;

        invoke_raw_typed!(
            hash,
                entity, 
                script
        )
    }
}

/// ## Parameters
*



pub fn is_entity_attached_to_any_ped_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1632E9A5F988D11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1632E9A5F988D11u64;
        
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
pub fn is_entity_attached_to_any_ped_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1632E9A5F988D11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1632E9A5F988D11u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
SET_*
Only called within 1 script for x360. 'fm_mission_controller' and it used on an object.
Ran after these 2 natives,
set_object_targettable(uParam0, 0);
set_entity_invincible(uParam0, 1);
```



pub fn set_wait_for_collisions_before_probe_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC6F8601FAF2E893u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC6F8601FAF2E893u64;
        
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
pub fn set_wait_for_collisions_before_probe_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC6F8601FAF2E893u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC6F8601FAF2E893u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_entity_is_target_priority_safe(
        
        
            entity: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA02E132F5C68722u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA02E132F5C68722u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_is_target_priority_raw(
        entity: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA02E132F5C68722u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA02E132F5C68722u64;

        invoke_raw_typed!(
            hash,
                entity, 
                p1, 
                p2
        )
    }
}

/// Gets the local rotation of the specified bone of the specified entity.

```
NativeDB Introduced: v1734
```



pub fn _get_entity_bone_rotation_local_safe(
        
        
            entity: 
        , 
        
        
            boneIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD8D32550E5CEBFEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD8D32550E5CEBFEu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                boneIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_entity_bone_rotation_local_raw(
        entity: , 
        boneIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD8D32550E5CEBFEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD8D32550E5CEBFEu64;

        invoke_raw_typed!(
            hash,
                entity, 
                boneIndex
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _0x68b562e124cc0aef_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68B562E124CC0AEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68B562E124CC0AEFu64;
        
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
pub fn _0x68b562e124cc0aef_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68B562E124CC0AEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68B562E124CC0AEFu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Simply returns whatever is passed to it (Regardless of whether the handle is valid or not).  
```



pub fn get_object_index_from_entity_index_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7E3B9735C0F89D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7E3B9735C0F89D6u64;
        
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
pub fn get_object_index_from_entity_index_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7E3B9735C0F89D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7E3B9735C0F89D6u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn set_entity_lights_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CFBA6A80BDF3874u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CFBA6A80BDF3874u64;
        
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
pub fn set_entity_lights_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CFBA6A80BDF3874u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CFBA6A80BDF3874u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_entity_velocity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4805D2B1D8CF94A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4805D2B1D8CF94A9u64;
        
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
pub fn get_entity_velocity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4805D2B1D8CF94A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4805D2B1D8CF94A9u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
This is an alias of SET_ENTITY_AS_NO_LONGER_NEEDED.  
```



pub fn set_ped_as_no_longer_needed_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2595DD4236549CE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2595DD4236549CE3u64;
        
        let result = invoke_raw!(
            hash,
                ped
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_as_no_longer_needed_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2595DD4236549CE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2595DD4236549CE3u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// p10 is some entity flag check, also used in [`IS_ENTITY_AT_ENTITY`](#_0x751B70C3D034E187), [`IS_ENTITY_IN_AREA`](#_0x54736AA40E271165) and [`IS_ENTITY_AT_COORD`](#_0x20B60995556D004F).

See [`IS_POINT_IN_ANGLED_AREA`](#_0x2A70BAE8883E4C81) for the definition of an angled area.



pub fn is_entity_in_angled_area_safe(
        
        
            entity: 
        , 
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            width: 
        , 
        
        
            debug: 
        , 
        
        
            includez: 
        , 
        
        
            p10: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51210CED3DA1C78Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51210CED3DA1C78Au64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width, 
                debug, 
                includez, 
                p10
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_entity_in_angled_area_raw(
        entity: , 
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        width: , 
        debug: , 
        includez: , 
        p10: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51210CED3DA1C78Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51210CED3DA1C78Au64;

        invoke_raw_typed!(
            hash,
                entity, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width, 
                debug, 
                includez, 
                p10
        )
    }
}

/// ## Parameters
*



pub fn set_entity_coords_without_plants_reset_safe(
        
        
            entity: 
        , 
        
        
            xPos: 
        , 
        
        
            yPos: 
        , 
        
        
            zPos: 
        , 
        
        
            alive: 
        , 
        
        
            deadFlag: 
        , 
        
        
            ragdollFlag: 
        , 
        
        
            clearArea: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x621873ECE1178967u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x621873ECE1178967u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                xPos, 
                yPos, 
                zPos, 
                alive, 
                deadFlag, 
                ragdollFlag, 
                clearArea
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_coords_without_plants_reset_raw(
        entity: , 
        xPos: , 
        yPos: , 
        zPos: , 
        alive: , 
        deadFlag: , 
        ragdollFlag: , 
        clearArea: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x621873ECE1178967u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x621873ECE1178967u64;

        invoke_raw_typed!(
            hash,
                entity, 
                xPos, 
                yPos, 
                zPos, 
                alive, 
                deadFlag, 
                ragdollFlag, 
                clearArea
        )
    }
}

/// Sets the rotation of a specified entity in the game world.

```
NativeDB Introduced: v323
```



pub fn set_entity_rotation_safe(
        
        
            entity: 
        , 
        
        
            pitch: 
        , 
        
        
            roll: 
        , 
        
        
            yaw: 
        , 
        
        
            rotationOrder: 
        , 
        
        
            bDeadCheck: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8524A8B0171D5E07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8524A8B0171D5E07u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                pitch, 
                roll, 
                yaw, 
                rotationOrder, 
                bDeadCheck
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_rotation_raw(
        entity: , 
        pitch: , 
        roll: , 
        yaw: , 
        rotationOrder: , 
        bDeadCheck: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8524A8B0171D5E07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8524A8B0171D5E07u64;

        invoke_raw_typed!(
            hash,
                entity, 
                pitch, 
                roll, 
                yaw, 
                rotationOrder, 
                bDeadCheck
        )
    }
}

/// ## Parameters
*



pub fn _get_entity_bone_count_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB328DCC3A3AA401Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB328DCC3A3AA401Bu64;
        
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
pub fn _get_entity_bone_count_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB328DCC3A3AA401Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB328DCC3A3AA401Bu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn is_entity_touching_entity_safe(
        
        
            entity: 
        , 
        
        
            targetEntity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17FFC1B2BA35A494u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17FFC1B2BA35A494u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                targetEntity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_entity_touching_entity_raw(
        entity: , 
        targetEntity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17FFC1B2BA35A494u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17FFC1B2BA35A494u64;

        invoke_raw_typed!(
            hash,
                entity, 
                targetEntity
        )
    }
}

/// ```
P3 is always 3 as far as i cant tell  
```

[Animations list](https://alexguirre.github.io/animations-list/)



pub fn has_entity_anim_finished_safe(
        
        
            entity: 
        , 
        
        
            animDict: 
        , 
        
        
            animName: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20B711662962B472u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20B711662962B472u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                animDict, 
                animName, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_entity_anim_finished_raw(
        entity: , 
        animDict: , 
        animName: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20B711662962B472u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20B711662962B472u64;

        invoke_raw_typed!(
            hash,
                entity, 
                animDict, 
                animName, 
                p3
        )
    }
}

/// Checks if entity is within the specified axis aligned box around the target entity.

```c
enum eTransportMode {
    SCRIPT_TM_ANY = 0,
    SCRIPT_TM_ON_FOOT = 1,
    SCRIPT_TM_IN_VEHICLE = 2
};
```



pub fn is_entity_at_entity_safe(
        
        
            entity: 
        , 
        
        
            target: 
        , 
        
        
            xSize: 
        , 
        
        
            ySize: 
        , 
        
        
            zSize: 
        , 
        
        
            highlightArea: 
        , 
        
        
            do3dCheck: 
        , 
        
        
            transportMode: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x751B70C3D034E187u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x751B70C3D034E187u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                target, 
                xSize, 
                ySize, 
                zSize, 
                highlightArea, 
                do3dCheck, 
                transportMode
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_entity_at_entity_raw(
        entity: , 
        target: , 
        xSize: , 
        ySize: , 
        zSize: , 
        highlightArea: , 
        do3dCheck: , 
        transportMode: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x751B70C3D034E187u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x751B70C3D034E187u64;

        invoke_raw_typed!(
            hash,
                entity, 
                target, 
                xSize, 
                ySize, 
                zSize, 
                highlightArea, 
                do3dCheck, 
                transportMode
        )
    }
}

/// ```
Get how much of the entity is submerged.  1.0f is whole entity.  
```



pub fn get_entity_submerged_level_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE81AFC1BC4CC41CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE81AFC1BC4CC41CEu64;
        
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
pub fn get_entity_submerged_level_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE81AFC1BC4CC41CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE81AFC1BC4CC41CEu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
SET_ENTITY_*
```



pub fn _set_entity_decals_disabled_safe(
        
        
            entity: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C2E3DC128F44309u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C2E3DC128F44309u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_entity_decals_disabled_raw(
        entity: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C2E3DC128F44309u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C2E3DC128F44309u64;

        invoke_raw_typed!(
            hash,
                entity, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn get_entity_alpha_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A47B3B5E63E94C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A47B3B5E63E94C6u64;
        
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
pub fn get_entity_alpha_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A47B3B5E63E94C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A47B3B5E63E94C6u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Return height (z-dimension) above ground.   
Example: The pilot in a titan plane is 1.844176 above ground.  
How can i convert it to meters?  
Everything seems to be in meters, probably this too.  
```



pub fn get_entity_height_above_ground_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DD55701034110E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DD55701034110E5u64;
        
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
pub fn get_entity_height_above_ground_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DD55701034110E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DD55701034110E5u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn has_entity_been_damaged_by_any_vehicle_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFD5033FDBA0A9C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFD5033FDBA0A9C8u64;
        
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
pub fn has_entity_been_damaged_by_any_vehicle_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFD5033FDBA0A9C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFD5033FDBA0A9C8u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Returns the LOD distance of an entity.  
```



pub fn get_entity_lod_dist_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4159C2762B5791D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4159C2762B5791D6u64;
        
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
pub fn get_entity_lod_dist_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4159C2762B5791D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4159C2762B5791D6u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn has_collision_loaded_around_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE9676F61BC0B3321u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE9676F61BC0B3321u64;
        
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
pub fn has_collision_loaded_around_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE9676F61BC0B3321u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE9676F61BC0B3321u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn _attach_entity_bone_to_entity_bone_physically_safe(
        
        
            entity1: 
        , 
        
        
            entity2: 
        , 
        
        
            entityBone: 
        , 
        
        
            entityBone2: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD1695C5D3B05439u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD1695C5D3B05439u64;
        
        let result = invoke_raw!(
            hash,
                entity1, 
                entity2, 
                entityBone, 
                entityBone2, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _attach_entity_bone_to_entity_bone_physically_raw(
        entity1: , 
        entity2: , 
        entityBone: , 
        entityBone2: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD1695C5D3B05439u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD1695C5D3B05439u64;

        invoke_raw_typed!(
            hash,
                entity1, 
                entity2, 
                entityBone, 
                entityBone2, 
                p4, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn is_entity_visible_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47D6F43D77935C75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47D6F43D77935C75u64;
        
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
pub fn is_entity_visible_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47D6F43D77935C75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47D6F43D77935C75u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// Delete the specified entity, and invalidate the passed handle (i.e., the in/out argument).
You might want to check if the entity exists before with [DOES_ENTITY_EXIST](#_0x7239B21A38F536BA).



pub fn delete_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE3CBE5BF394C9C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE3CBE5BF394C9C9u64;
        
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
pub fn delete_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE3CBE5BF394C9C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE3CBE5BF394C9C9u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Based on carmod_shop script decompile this takes a vehicle parameter. It is called when repair is done on initial enter.  
```



pub fn force_entity_ai_and_animation_update_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40FDEDB72F8293B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40FDEDB72F8293B2u64;
        
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
pub fn force_entity_ai_and_animation_update_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40FDEDB72F8293B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40FDEDB72F8293B2u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Enable / disable each type of damage.



pub fn set_entity_proofs_safe(
        
        
            entity: 
        , 
        
        
            bulletProof: 
        , 
        
        
            fireProof: 
        , 
        
        
            explosionProof: 
        , 
        
        
            collisionProof: 
        , 
        
        
            meleeProof: 
        , 
        
        
            steamProof: 
        , 
        
        
            p7: 
        , 
        
        
            drownProof: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAEE099C6F890BB8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAEE099C6F890BB8u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                bulletProof, 
                fireProof, 
                explosionProof, 
                collisionProof, 
                meleeProof, 
                steamProof, 
                p7, 
                drownProof
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_proofs_raw(
        entity: , 
        bulletProof: , 
        fireProof: , 
        explosionProof: , 
        collisionProof: , 
        meleeProof: , 
        steamProof: , 
        p7: , 
        drownProof: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAEE099C6F890BB8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAEE099C6F890BB8u64;

        invoke_raw_typed!(
            hash,
                entity, 
                bulletProof, 
                fireProof, 
                explosionProof, 
                collisionProof, 
                meleeProof, 
                steamProof, 
                p7, 
                drownProof
        )
    }
}

/// SET_ENTITY_M*

```
NativeDB Introduced: v1734
```



pub fn _0xe66377cddada4810_safe(
        
        
            entity: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE66377CDDADA4810u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE66377CDDADA4810u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xe66377cddada4810_raw(
        entity: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE66377CDDADA4810u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE66377CDDADA4810u64;

        invoke_raw_typed!(
            hash,
                entity, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_entity_render_scorched_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x730F5F8D3F0F2050u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x730F5F8D3F0F2050u64;
        
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
pub fn set_entity_render_scorched_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x730F5F8D3F0F2050u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x730F5F8D3F0F2050u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// Returns the model hash from an entity.



pub fn get_entity_model_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F47B058362C84B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F47B058362C84B5u64;
        
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
pub fn get_entity_model_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F47B058362C84B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F47B058362C84B5u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _get_entity_proofs_safe(
        
        
            entity: 
        , 
        
        
            bulletProof: 
        , 
        
        
            fireProof: 
        , 
        
        
            explosionProof: 
        , 
        
        
            collisionProof: 
        , 
        
        
            meleeProof: 
        , 
        
        
            steamProof: 
        , 
        
        
            p7: 
        , 
        
        
            drownProof: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE8CD9BE829BBEBFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE8CD9BE829BBEBFu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                bulletProof, 
                fireProof, 
                explosionProof, 
                collisionProof, 
                meleeProof, 
                steamProof, 
                p7, 
                drownProof
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_entity_proofs_raw(
        entity: , 
        bulletProof: , 
        fireProof: , 
        explosionProof: , 
        collisionProof: , 
        meleeProof: , 
        steamProof: , 
        p7: , 
        drownProof: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE8CD9BE829BBEBFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE8CD9BE829BBEBFu64;

        invoke_raw_typed!(
            hash,
                entity, 
                bulletProof, 
                fireProof, 
                explosionProof, 
                collisionProof, 
                meleeProof, 
                steamProof, 
                p7, 
                drownProof
        )
    }
}

/// ```
Gets the world rotation of the specified bone of the specified entity.
This native is used in casinoroulette.c but I don't know yet what is the difference with _GET_ENTITY_BONE_ROTATION
```



pub fn _get_entity_bone_position_2_safe(
        
        
            entity: 
        , 
        
        
            boneIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46F8696933A63C9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46F8696933A63C9Bu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                boneIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_entity_bone_position_2_raw(
        entity: , 
        boneIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46F8696933A63C9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46F8696933A63C9Bu64;

        invoke_raw_typed!(
            hash,
                entity, 
                boneIndex
        )
    }
}

/// ```
Marks the specified entity (ped, vehicle or object) as no longer needed.  
Entities marked as no longer needed, will be deleted as the engine sees fit.  
```



pub fn set_entity_as_no_longer_needed_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB736A491E64A32CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB736A491E64A32CFu64;
        
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
pub fn set_entity_as_no_longer_needed_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB736A491E64A32CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB736A491E64A32CFu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn get_entity_collision_disabled_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCF1E97BEFDAE480u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCF1E97BEFDAE480u64;
        
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
pub fn get_entity_collision_disabled_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCF1E97BEFDAE480u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCF1E97BEFDAE480u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn set_entity_motion_blur_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x295D82A8559F9150u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x295D82A8559F9150u64;
        
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
pub fn set_entity_motion_blur_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x295D82A8559F9150u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x295D82A8559F9150u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_entity_in_area_safe(
        
        
            entity: 
        , 
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            x2: 
        , 
        
        
            y2: 
        , 
        
        
            z2: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54736AA40E271165u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54736AA40E271165u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                p7, 
                p8, 
                p9
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_entity_in_area_raw(
        entity: , 
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        p7: , 
        p8: , 
        p9: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54736AA40E271165u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54736AA40E271165u64;

        invoke_raw_typed!(
            hash,
                entity, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                p7, 
                p8, 
                p9
        )
    }
}

/// **NOTE**: What you use for rotationOrder when getting must be the same as rotationOrder when setting the rotation.


```c
enum eRotationOrder {
    // Rotate around the z-axis, then the y-axis and finally the x-axis.
    ROT_ZYX = 0,
    // Rotate around the y-axis, then the z-axis and finally the x-axis.
    ROT_YZX = 1,
    // Rotate around the z-axis, then the x-axis and finally the y-axis.
    ROT_ZXY = 2,
    // Rotate around the x-axis, then the z-axis and finally the y-axis.
    ROT_XZY = 3,
    // Rotate around the y-axis, then the x-axis and finally the z-axis.
    ROT_YXZ = 4,
    // Rotate around the x-axis, then the y-axis and finally the z-axis.
    ROT_XYZ = 5,
}
```



pub fn get_entity_rotation_safe(
        
        
            entity: 
        , 
        
        
            rotationOrder: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFBD61CC738D9EB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFBD61CC738D9EB9u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                rotationOrder
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_entity_rotation_raw(
        entity: , 
        rotationOrder: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFBD61CC738D9EB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFBD61CC738D9EB9u64;

        invoke_raw_typed!(
            hash,
                entity, 
                rotationOrder
        )
    }
}

/// ## Parameters
*



pub fn set_entity_only_damaged_by_player_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79F020FF9EDC0748u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79F020FF9EDC0748u64;
        
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
pub fn set_entity_only_damaged_by_player_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79F020FF9EDC0748u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79F020FF9EDC0748u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// Teleports an entity to specified coordinates directly, with options to maintain certain behaviors post-teleportation.



pub fn set_entity_coords_no_offset_safe(
        
        
            entity: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            keepTasks: 
        , 
        
        
            keepIK: 
        , 
        
        
            doWarp: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x239A3351AC1DA385u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x239A3351AC1DA385u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                x, 
                y, 
                z, 
                keepTasks, 
                keepIK, 
                doWarp
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_coords_no_offset_raw(
        entity: , 
        x: , 
        y: , 
        z: , 
        keepTasks: , 
        keepIK: , 
        doWarp: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x239A3351AC1DA385u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x239A3351AC1DA385u64;

        invoke_raw_typed!(
            hash,
                entity, 
                x, 
                y, 
                z, 
                keepTasks, 
                keepIK, 
                doWarp
        )
    }
}

/// ```
Note that the third parameter(denoted as z) is "up and down" with positive numbers encouraging upwards movement.
```



pub fn set_entity_velocity_safe(
        
        
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
        let hash = 0x1C99BB7B6E96D16Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C99BB7B6E96D16Fu64;
        
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
pub fn set_entity_velocity_raw(
        entity: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C99BB7B6E96D16Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C99BB7B6E96D16Fu64;

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



pub fn set_entity_requires_more_expensive_river_check_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x694E00132F2823EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x694E00132F2823EDu64;
        
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
pub fn set_entity_requires_more_expensive_river_check_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x694E00132F2823EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x694E00132F2823EDu64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ```
Does nothing (essentially a nullsub).
```



pub fn _0x490861b88f4fd846_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x490861B88F4FD846u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x490861B88F4FD846u64;
        
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
pub fn _0x490861b88f4fd846_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x490861B88F4FD846u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x490861B88F4FD846u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x352e2b5cf420bf3b_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x352E2B5CF420BF3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x352E2B5CF420BF3Bu64;
        
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
pub fn _0x352e2b5cf420bf3b_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x352E2B5CF420BF3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x352E2B5CF420BF3Bu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Assigns an existing entity to be owned by the current script. If the entity was not owned by a script yet, this also means the entity will remain persistent until released.

Note that this is not needed right after creating an entity as a script-created entity will automatically be assigned.



pub fn set_entity_as_mission_entity_safe(
        
        
            entity: 
        , 
        
        
            scriptHostObject: 
        , 
        
        
            bGrabFromOtherScript: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD738C3085FE7E11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD738C3085FE7E11u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                scriptHostObject, 
                bGrabFromOtherScript
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_as_mission_entity_raw(
        entity: , 
        scriptHostObject: , 
        bGrabFromOtherScript: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD738C3085FE7E11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD738C3085FE7E11u64;

        invoke_raw_typed!(
            hash,
                entity, 
                scriptHostObject, 
                bGrabFromOtherScript
        )
    }
}

/// ```
GET_ENTITY_*

Seems to return the handle of the entity's portable pickup.

NativeDB Introduced: v1180
```



pub fn _get_entity_pickup_safe(
        
        
            entity: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F922734E259BD26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F922734E259BD26u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_entity_pickup_raw(
        entity: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F922734E259BD26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F922734E259BD26u64;

        invoke_raw_typed!(
            hash,
                entity, 
                modelHash
        )
    }
}

/// ```
This is an alias of SET_ENTITY_AS_NO_LONGER_NEEDED.  
```



pub fn set_vehicle_as_no_longer_needed_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x629BFA74418D6239u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x629BFA74418D6239u64;
        
        let result = invoke_raw!(
            hash,
                vehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_as_no_longer_needed_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x629BFA74418D6239u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x629BFA74418D6239u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Returns a float value representing animation's total playtime in milliseconds.  
Example:  
GET_ENTITY_ANIM_TOTAL_TIME(PLAYER_ID(),"amb@world_human_yoga@female@base","base_b")   
return 20800.000000  
```

[Animations list](https://alexguirre.github.io/animations-list/)



pub fn get_entity_anim_total_time_safe(
        
        
            entity: 
        , 
        
        
            animDict: 
        , 
        
        
            animName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50BD2730B191E360u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50BD2730B191E360u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                animDict, 
                animName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_entity_anim_total_time_raw(
        entity: , 
        animDict: , 
        animName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50BD2730B191E360u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50BD2730B191E360u64;

        invoke_raw_typed!(
            hash,
                entity, 
                animDict, 
                animName
        )
    }
}

/// Checks if the entity is within the given square of size xSize, ySize, zSize centered around the given coordinates.

The sizes given are the apothem (half of side) of the square, so a size of 5 would result in a square of 10x10, not 5x5.


For the highlightArea, if do3dCheck is true, the marker will be drawn at the bottom of the target area. So if the square is centered on the ground with a zSize larger than 0, the marker will appear under the ground.
The marker also doesn't scale, so it is always the same size (around half a meter).
So unfortunately the marker isn't that useful as it doesn't convey the correct information about the area (the marker doesn't reflect when the player is actually in the marker or not)



pub fn is_entity_at_coord_safe(
        
        
            entity: 
        , 
        
        
            xPos: 
        , 
        
        
            yPos: 
        , 
        
        
            zPos: 
        , 
        
        
            xSize: 
        , 
        
        
            ySize: 
        , 
        
        
            zSize: 
        , 
        
        
            highlightArea: 
        , 
        
        
            do3dCheck: 
        , 
        
        
            transportMode: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20B60995556D004Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20B60995556D004Fu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                xPos, 
                yPos, 
                zPos, 
                xSize, 
                ySize, 
                zSize, 
                highlightArea, 
                do3dCheck, 
                transportMode
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_entity_at_coord_raw(
        entity: , 
        xPos: , 
        yPos: , 
        zPos: , 
        xSize: , 
        ySize: , 
        zSize: , 
        highlightArea: , 
        do3dCheck: , 
        transportMode: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20B60995556D004Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20B60995556D004Fu64;

        invoke_raw_typed!(
            hash,
                entity, 
                xPos, 
                yPos, 
                zPos, 
                xSize, 
                ySize, 
                zSize, 
                highlightArea, 
                do3dCheck, 
                transportMode
        )
    }
}

/// ```
In the script "player_scene_t_bbfight.c4":  
"if (ENTITY::FIND_ANIM_EVENT_PHASE(&l_16E, &l_19F[v_4/*16*/], v_9, &v_A, &v_B))"  
-- &l_16E (p0) is requested as an anim dictionary earlier in the script.  
-- &l_19F[v_4/*16*/] (p1) is used in other natives in the script as the "animation" param.  
-- v_9 (p2) is instantiated as "victim_fall"; I'm guessing that's another anim  
--v_A and v_B (p3 & p4) are both set as -1.0, but v_A is used immediately after this native for:   
"if (v_A < ENTITY::GET_ENTITY_ANIM_CURRENT_TIME(...))"  
Both v_A and v_B are seemingly used to contain both Vector3's and floats, so I can't say what either really is other than that they are both output parameters. p4 looks more like a *Vector3 though  
-alphazolam  
```

[Animations list](https://alexguirre.github.io/animations-list/)



pub fn find_anim_event_phase_safe(
        
        
            animDictionary: 
        , 
        
        
            animName: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07F1BE2BCCAA27A7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07F1BE2BCCAA27A7u64;
        
        let result = invoke_raw!(
            hash,
                animDictionary, 
                animName, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn find_anim_event_phase_raw(
        animDictionary: , 
        animName: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07F1BE2BCCAA27A7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07F1BE2BCCAA27A7u64;

        invoke_raw_typed!(
            hash,
                animDictionary, 
                animName, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn is_entity_attached_to_any_object_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF511840CEEDE0CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF511840CEEDE0CCu64;
        
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
pub fn is_entity_attached_to_any_object_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF511840CEEDE0CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF511840CEEDE0CCu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn _0xb17bc6453f6cf5ac_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB17BC6453F6CF5ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB17BC6453F6CF5ACu64;
        
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
pub fn _0xb17bc6453f6cf5ac_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB17BC6453F6CF5ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB17BC6453F6CF5ACu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_entity_dynamic_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1718DE8E3F2823CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1718DE8E3F2823CAu64;
        
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
pub fn set_entity_dynamic_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1718DE8E3F2823CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1718DE8E3F2823CAu64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// Set the heading of an entity in degrees also known as "Yaw".



pub fn set_entity_heading_safe(
        
        
            entity: 
        , 
        
        
            heading: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E2530AA8ADA980Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E2530AA8ADA980Eu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                heading
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_heading_raw(
        entity: , 
        heading: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E2530AA8ADA980Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E2530AA8ADA980Eu64;

        invoke_raw_typed!(
            hash,
                entity, 
                heading
        )
    }
}

/// ## Parameters
*



pub fn get_last_material_hit_by_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C3D0A935F535C4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C3D0A935F535C4Cu64;
        
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
pub fn get_last_material_hit_by_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C3D0A935F535C4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C3D0A935F535C4Cu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn set_entity_completely_disable_collision_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        , 
        
        
            keepPhysics: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9EBC85ED0FFFE51Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9EBC85ED0FFFE51Cu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                toggle, 
                keepPhysics
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_completely_disable_collision_raw(
        entity: , 
        toggle: , 
        keepPhysics: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9EBC85ED0FFFE51Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9EBC85ED0FFFE51Cu64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle, 
                keepPhysics
        )
    }
}

/// ## Parameters
*



pub fn is_entity_in_water_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFB0A0D8EDD145A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFB0A0D8EDD145A3u64;
        
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
pub fn is_entity_in_water_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFB0A0D8EDD145A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFB0A0D8EDD145A3u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Converts world coords (posX - Z) to coords relative to the entity  
Example:  
posX = 50  
posY = 1000  
posZ = 60  
Entity's coords are: x=30, y=1000, z=60.  
All three returned coords will then be in range of [-20,20] depending on rotation of the entity.  
```



pub fn get_offset_from_entity_given_world_coords_safe(
        
        
            entity: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2274BC1C4885E333u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2274BC1C4885E333u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                posX, 
                posY, 
                posZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_offset_from_entity_given_world_coords_raw(
        entity: , 
        posX: , 
        posY: , 
        posZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2274BC1C4885E333u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2274BC1C4885E333u64;

        invoke_raw_typed!(
            hash,
                entity, 
                posX, 
                posY, 
                posZ
        )
    }
}

/// ```c
enum eApplyForceTypes {
    APPLY_TYPE_FORCE = 0,
    APPLY_TYPE_IMPULSE = 1,
    APPLY_TYPE_EXTERNAL_FORCE = 2,
    APPLY_TYPE_EXTERNAL_IMPULSE = 3,
    APPLY_TYPE_TORQUE = 4,
    APPLY_TYPE_ANGULAR_IMPULSE = 5
}
```



pub fn apply_force_to_entity_safe(
        
        
            entity: 
        , 
        
        
            forceType: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            offX: 
        , 
        
        
            offY: 
        , 
        
        
            offZ: 
        , 
        
        
            nComponent: 
        , 
        
        
            bLocalForce: 
        , 
        
        
            bLocalOffset: 
        , 
        
        
            bScaleByMass: 
        , 
        
        
            bPlayAudio: 
        , 
        
        
            bScaleByTimeWarp: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5F68BE9613E2D18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5F68BE9613E2D18u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                forceType, 
                x, 
                y, 
                z, 
                offX, 
                offY, 
                offZ, 
                nComponent, 
                bLocalForce, 
                bLocalOffset, 
                bScaleByMass, 
                bPlayAudio, 
                bScaleByTimeWarp
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn apply_force_to_entity_raw(
        entity: , 
        forceType: , 
        x: , 
        y: , 
        z: , 
        offX: , 
        offY: , 
        offZ: , 
        nComponent: , 
        bLocalForce: , 
        bLocalOffset: , 
        bScaleByMass: , 
        bPlayAudio: , 
        bScaleByTimeWarp: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5F68BE9613E2D18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5F68BE9613E2D18u64;

        invoke_raw_typed!(
            hash,
                entity, 
                forceType, 
                x, 
                y, 
                z, 
                offX, 
                offY, 
                offZ, 
                nComponent, 
                bLocalForce, 
                bLocalOffset, 
                bScaleByMass, 
                bPlayAudio, 
                bScaleByTimeWarp
        )
    }
}

/// ```
Returns the coordinates of an entity-bone.  
```



pub fn get_world_position_of_entity_bone_safe(
        
        
            entity: 
        , 
        
        
            boneIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44A8FCB8ED227738u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44A8FCB8ED227738u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                boneIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_world_position_of_entity_bone_raw(
        entity: , 
        boneIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44A8FCB8ED227738u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44A8FCB8ED227738u64;

        invoke_raw_typed!(
            hash,
                entity, 
                boneIndex
        )
    }
}

/// ```
Gets the world rotation of the specified bone of the specified entity.
```



pub fn _get_entity_bone_rotation_safe(
        
        
            entity: 
        , 
        
        
            boneIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE6294A232D03786u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE6294A232D03786u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                boneIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_entity_bone_rotation_raw(
        entity: , 
        boneIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE6294A232D03786u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE6294A232D03786u64;

        invoke_raw_typed!(
            hash,
                entity, 
                boneIndex
        )
    }
}

/// ```
if (ENTITY::HAS_ANIM_EVENT_FIRED(PLAYER::PLAYER_PED_ID(), MISC::GET_HASH_KEY("CreateObject")))
```



pub fn has_anim_event_fired_safe(
        
        
            entity: 
        , 
        
        
            actionHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAF4CD9EA3E7E922u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAF4CD9EA3E7E922u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                actionHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_anim_event_fired_raw(
        entity: , 
        actionHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAF4CD9EA3E7E922u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAF4CD9EA3E7E922u64;

        invoke_raw_typed!(
            hash,
                entity, 
                actionHash
        )
    }
}

/// ## Parameters
*



pub fn get_collision_normal_of_last_hit_for_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE465D4AB7CA6AE72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE465D4AB7CA6AE72u64;
        
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
pub fn get_collision_normal_of_last_hit_for_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE465D4AB7CA6AE72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE465D4AB7CA6AE72u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn is_entity_visible_to_script_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD796CB5BA8F20E32u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD796CB5BA8F20E32u64;
        
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
pub fn is_entity_visible_to_script_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD796CB5BA8F20E32u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD796CB5BA8F20E32u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn get_nearest_player_to_entity_on_team_safe(
        
        
            entity: 
        , 
        
        
            team: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4DC9A62F844D9337u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4DC9A62F844D9337u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                team
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_nearest_player_to_entity_on_team_raw(
        entity: , 
        team: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4DC9A62F844D9337u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4DC9A62F844D9337u64;

        invoke_raw_typed!(
            hash,
                entity, 
                team
        )
    }
}

/// Checks whether an entity exists in the game world.



pub fn does_entity_exist_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7239B21A38F536BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7239B21A38F536BAu64;
        
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
pub fn does_entity_exist_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7239B21A38F536BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7239B21A38F536BAu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Sets whether the entity can be targeted without being in line-of-sight.  
```



pub fn set_entity_can_be_targeted_without_los_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3997889736FD899u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3997889736FD899u64;
        
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
pub fn set_entity_can_be_targeted_without_los_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3997889736FD899u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3997889736FD899u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_entity_can_be_damaged_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD95CC5D2AB15A09Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD95CC5D2AB15A09Fu64;
        
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
pub fn get_entity_can_be_damaged_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD95CC5D2AB15A09Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD95CC5D2AB15A09Fu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
RAGEPluginHook list: docs.ragepluginhook.net/html/62951c37-a440-478c-b389-c471230ddfc5.htm
```



pub fn stop_entity_anim_safe(
        
        
            entity: 
        , 
        
        
            animation: 
        , 
        
        
            animGroup: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28004F88151E03E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28004F88151E03E0u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                animation, 
                animGroup, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_entity_anim_raw(
        entity: , 
        animation: , 
        animGroup: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28004F88151E03E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28004F88151E03E0u64;

        invoke_raw_typed!(
            hash,
                entity, 
                animation, 
                animGroup, 
                p3
        )
    }
}

/// ```
Gets the X-component of the entity's forward vector.  
```



pub fn get_entity_forward_x_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BB4EF4214E0E6D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BB4EF4214E0E6D5u64;
        
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
pub fn get_entity_forward_x_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BB4EF4214E0E6D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BB4EF4214E0E6D5u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Called to update entity attachments.
```



pub fn process_entity_attachments_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4080490ADC51C6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4080490ADC51C6Fu64;
        
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
pub fn process_entity_attachments_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4080490ADC51C6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4080490ADC51C6Fu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Called on tick.  
Tested with vehicles, returns true whenever the vehicle is touching any entity.  
Note: for vehicles, the wheels can touch the ground and it will still return false, but if the body of the vehicle touches the ground, it will return true.  
```



pub fn has_entity_collided_with_anything_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BAD02F0368D9E14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BAD02F0368D9E14u64;
        
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
pub fn has_entity_collided_with_anything_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BAD02F0368D9E14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BAD02F0368D9E14u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn get_entity_upright_value_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95EED5A694951F9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95EED5A694951F9Fu64;
        
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
pub fn get_entity_upright_value_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95EED5A694951F9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95EED5A694951F9Fu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Only works with objects!  
Network players do not see changes done with this.  
```



pub fn create_model_swap_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            originalModel: 
        , 
        
        
            newModel: 
        , 
        
        
            bSurviveMapReload: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92C47782FDA8B2A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92C47782FDA8B2A3u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                originalModel, 
                newModel, 
                bSurviveMapReload
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_model_swap_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        originalModel: , 
        newModel: , 
        bSurviveMapReload: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92C47782FDA8B2A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92C47782FDA8B2A3u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                originalModel, 
                newModel, 
                bSurviveMapReload
        )
    }
}

/// ## Parameters
*



pub fn set_can_climb_on_entity_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA80AE305E0A3044Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA80AE305E0A3044Fu64;
        
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
pub fn set_can_climb_on_entity_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA80AE305E0A3044Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA80AE305E0A3044Fu64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_entity_a_vehicle_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6AC7003FA6E5575Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6AC7003FA6E5575Eu64;
        
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
pub fn is_entity_a_vehicle_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6AC7003FA6E5575Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6AC7003FA6E5575Eu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn set_entity_collision_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        , 
        
        
            keepPhysics: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A9205C1B9EE827Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A9205C1B9EE827Fu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                toggle, 
                keepPhysics
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_collision_raw(
        entity: , 
        toggle: , 
        keepPhysics: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A9205C1B9EE827Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A9205C1B9EE827Fu64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle, 
                keepPhysics
        )
    }
}

/// ## Parameters
*



pub fn get_entity_pitch_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD45DC2893621E1FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD45DC2893621E1FEu64;
        
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
pub fn get_entity_pitch_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD45DC2893621E1FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD45DC2893621E1FEu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Gets the Y-component of the entity's forward vector.  
```



pub fn get_entity_forward_y_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x866A4A5FAE349510u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x866A4A5FAE349510u64;
        
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
pub fn get_entity_forward_y_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x866A4A5FAE349510u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x866A4A5FAE349510u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn set_entity_only_damaged_by_relationship_group_safe(
        
        
            entity: 
        , 
        
        
            p1: 
        , 
        
        
            relationshipHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7022BD828FA0B082u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7022BD828FA0B082u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                p1, 
                relationshipHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_only_damaged_by_relationship_group_raw(
        entity: , 
        p1: , 
        relationshipHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7022BD828FA0B082u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7022BD828FA0B082u64;

        invoke_raw_typed!(
            hash,
                entity, 
                p1, 
                relationshipHash
        )
    }
}

/// ```
Returns:
0 = no entity
1 = ped
2 = vehicle
3 = object
```



pub fn get_entity_type_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8ACD366038D14505u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8ACD366038D14505u64;
        
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
pub fn get_entity_type_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8ACD366038D14505u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8ACD366038D14505u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn get_entity_rotation_velocity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x213B91045D09B983u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x213B91045D09B983u64;
        
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
pub fn get_entity_rotation_velocity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x213B91045D09B983u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x213B91045D09B983u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn stop_synchronized_map_entity_anim_safe(
        
        
            p0: 
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11E79CAB7183B6F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11E79CAB7183B6F5u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_synchronized_map_entity_anim_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11E79CAB7183B6F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11E79CAB7183B6F5u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn remove_model_swap_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            oldModelHash: 
        , 
        
        
            newModelHash: 
        , 
        
        
            bLazy: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x033C0F9A64E229AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x033C0F9A64E229AEu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                oldModelHash, 
                newModelHash, 
                bLazy
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_model_swap_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        oldModelHash: , 
        newModelHash: , 
        bLazy: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x033C0F9A64E229AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x033C0F9A64E229AEu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                oldModelHash, 
                newModelHash, 
                bLazy
        )
    }
}

/// ## Parameters
*



pub fn is_entity_upsidedown_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DBD58820FA61D71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DBD58820FA61D71u64;
        
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
pub fn is_entity_upsidedown_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DBD58820FA61D71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DBD58820FA61D71u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// Attach an entity to the specified entity.



pub fn attach_entity_to_entity_safe(
        
        
            entity1: 
        , 
        
        
            entity2: 
        , 
        
        
            boneIndex: 
        , 
        
        
            xPos: 
        , 
        
        
            yPos: 
        , 
        
        
            zPos: 
        , 
        
        
            xRot: 
        , 
        
        
            yRot: 
        , 
        
        
            zRot: 
        , 
        
        
            p9: 
        , 
        
        
            useSoftPinning: 
        , 
        
        
            collision: 
        , 
        
        
            isPed: 
        , 
        
        
            rotationOrder: 
        , 
        
        
            syncRot: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B9BBD38AB0796DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B9BBD38AB0796DFu64;
        
        let result = invoke_raw!(
            hash,
                entity1, 
                entity2, 
                boneIndex, 
                xPos, 
                yPos, 
                zPos, 
                xRot, 
                yRot, 
                zRot, 
                p9, 
                useSoftPinning, 
                collision, 
                isPed, 
                rotationOrder, 
                syncRot
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn attach_entity_to_entity_raw(
        entity1: , 
        entity2: , 
        boneIndex: , 
        xPos: , 
        yPos: , 
        zPos: , 
        xRot: , 
        yRot: , 
        zRot: , 
        p9: , 
        useSoftPinning: , 
        collision: , 
        isPed: , 
        rotationOrder: , 
        syncRot: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B9BBD38AB0796DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B9BBD38AB0796DFu64;

        invoke_raw_typed!(
            hash,
                entity1, 
                entity2, 
                boneIndex, 
                xPos, 
                yPos, 
                zPos, 
                xRot, 
                yRot, 
                zRot, 
                p9, 
                useSoftPinning, 
                collision, 
                isPed, 
                rotationOrder, 
                syncRot
        )
    }
}

/// ## Parameters
*



pub fn set_entity_can_be_damaged_by_relationship_group_safe(
        
        
            entity: 
        , 
        
        
            bCanBeDamaged: 
        , 
        
        
            relGroup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE22D8FDE858B8119u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE22D8FDE858B8119u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                bCanBeDamaged, 
                relGroup
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_can_be_damaged_by_relationship_group_raw(
        entity: , 
        bCanBeDamaged: , 
        relGroup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE22D8FDE858B8119u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE22D8FDE858B8119u64;

        invoke_raw_typed!(
            hash,
                entity, 
                bCanBeDamaged, 
                relGroup
        )
    }
}

/// ## Parameters
*



pub fn is_entity_an_object_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D68C8FD0FACA94Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D68C8FD0FACA94Eu64;
        
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
pub fn is_entity_an_object_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D68C8FD0FACA94Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D68C8FD0FACA94Eu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn is_entity_attached_to_entity_safe(
        
        
            from: 
        , 
        
        
            to: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFBE71898A993728u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFBE71898A993728u64;
        
        let result = invoke_raw!(
            hash,
                from, 
                to
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_entity_attached_to_entity_raw(
        from: , 
        to: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFBE71898A993728u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFBE71898A993728u64;

        invoke_raw_typed!(
            hash,
                from, 
                to
        )
    }
}

/// ```
Gets the entity's forward vector.
```



pub fn get_entity_forward_vector_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A794A5A57F8DF91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A794A5A57F8DF91u64;
        
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
pub fn get_entity_forward_vector_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A794A5A57F8DF91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A794A5A57F8DF91u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// This native sets the entity's alpha level.



pub fn set_entity_alpha_safe(
        
        
            entity: 
        , 
        
        
            alphaLevel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44A0870B7E92D7C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44A0870B7E92D7C0u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                alphaLevel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_alpha_raw(
        entity: , 
        alphaLevel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44A0870B7E92D7C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44A0870B7E92D7C0u64;

        invoke_raw_typed!(
            hash,
                entity, 
                alphaLevel
        )
    }
}

/// ## Parameters
*



pub fn is_entity_in_zone_safe(
        
        
            entity: 
        , 
        
        
            zone: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6463CF6AF527071u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6463CF6AF527071u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                zone
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_entity_in_zone_raw(
        entity: , 
        zone: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6463CF6AF527071u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6463CF6AF527071u64;

        invoke_raw_typed!(
            hash,
                entity, 
                zone
        )
    }
}

/// ## Parameters
*



pub fn _attach_entity_bone_to_entity_bone_safe(
        
        
            entity1: 
        , 
        
        
            entity2: 
        , 
        
        
            entityBone: 
        , 
        
        
            entityBone2: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C48B75732C8456Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C48B75732C8456Cu64;
        
        let result = invoke_raw!(
            hash,
                entity1, 
                entity2, 
                entityBone, 
                entityBone2, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _attach_entity_bone_to_entity_bone_raw(
        entity1: , 
        entity2: , 
        entityBone: , 
        entityBone2: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C48B75732C8456Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C48B75732C8456Cu64;

        invoke_raw_typed!(
            hash,
                entity1, 
                entity2, 
                entityBone, 
                entityBone2, 
                p4, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn set_entity_anim_speed_safe(
        
        
            entity: 
        , 
        
        
            animDictionary: 
        , 
        
        
            animName: 
        , 
        
        
            speedMultiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28D1A16553C51776u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28D1A16553C51776u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                animDictionary, 
                animName, 
                speedMultiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_anim_speed_raw(
        entity: , 
        animDictionary: , 
        animName: , 
        speedMultiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28D1A16553C51776u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28D1A16553C51776u64;

        invoke_raw_typed!(
            hash,
                entity, 
                animDictionary, 
                animName, 
                speedMultiplier
        )
    }
}

/// ```
NativeDB Introduced: v2699
```



pub fn _does_entity_have_skeleton_data_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x764EB96874EFFDC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x764EB96874EFFDC1u64;
        
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
pub fn _does_entity_have_skeleton_data_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x764EB96874EFFDC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x764EB96874EFFDC1u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Simply returns whatever is passed to it (Regardless of whether the handle is valid or not).  
```



pub fn get_vehicle_index_from_entity_index_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B53F92932ADFAC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B53F92932ADFAC0u64;
        
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
pub fn get_vehicle_index_from_entity_index_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B53F92932ADFAC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B53F92932ADFAC0u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// Freezes or unfreezes an entity preventing its coordinates to change by the player if set to `true`. You can still change the entity position using [`SET_ENTITY_COORDS`](#_0x06843DA7060A026B).



pub fn freeze_entity_position_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x428CA6DBD1094446u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x428CA6DBD1094446u64;
        
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
pub fn freeze_entity_position_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x428CA6DBD1094446u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x428CA6DBD1094446u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn has_entity_been_damaged_by_any_object_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95EB9964FF5C5C65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95EB9964FF5C5C65u64;
        
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
pub fn has_entity_been_damaged_by_any_object_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95EB9964FF5C5C65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95EB9964FF5C5C65u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Related to cutscene entities. Unsure about the use.
SET_ENTITY_*
```



pub fn _0x78e8e3a640178255_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78E8E3A640178255u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78E8E3A640178255u64;
        
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
pub fn _0x78e8e3a640178255_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78E8E3A640178255u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78E8E3A640178255u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn would_entity_be_occluded_safe(
        
        
            entityModelHash: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE5D2A122E09EC42u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE5D2A122E09EC42u64;
        
        let result = invoke_raw!(
            hash,
                entityModelHash, 
                x, 
                y, 
                z, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn would_entity_be_occluded_raw(
        entityModelHash: , 
        x: , 
        y: , 
        z: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE5D2A122E09EC42u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE5D2A122E09EC42u64;

        invoke_raw_typed!(
            hash,
                entityModelHash, 
                x, 
                y, 
                z, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn _0xcea7c8e1b48ff68c_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEA7C8E1B48FF68Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEA7C8E1B48FF68Cu64;
        
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
pub fn _0xcea7c8e1b48ff68c_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEA7C8E1B48FF68Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEA7C8E1B48FF68Cu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// p5 requires more research. See also [`CREATE_MODEL_HIDE`](#_0x8A97BCA30A0CE478) and [`CREATE_MODEL_SWAP`](#_0x92C47782FDA8B2A3).

Network players do not see changes done with this.



pub fn remove_model_hide_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9E3006FB3CBD765u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9E3006FB3CBD765u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_model_hide_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9E3006FB3CBD765u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9E3006FB3CBD765u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Return an integer value of entity's maximum health.  
Example:  
- Player = 200  
```



pub fn get_entity_max_health_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15D757606D170C3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15D757606D170C3Cu64;
        
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
pub fn get_entity_max_health_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15D757606D170C3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15D757606D170C3Cu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// [Animations list](https://alexguirre.github.io/animations-list/)



pub fn play_synchronized_map_entity_anim_safe(
        
        
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
        let hash = 0xB9C54555ED30FBC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9C54555ED30FBC4u64;
        
        let result = invoke_raw!(
            hash,
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
pub fn play_synchronized_map_entity_anim_raw(
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9C54555ED30FBC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9C54555ED30FBC4u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// ```
True means it can be deleted by the engine when switching lobbies/missions/etc, false means the script is expected to clean it up.
```



pub fn _set_entity_cleanup_by_engine_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3910051CCECDB00Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3910051CCECDB00Cu64;
        
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
pub fn _set_entity_cleanup_by_engine_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3910051CCECDB00Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3910051CCECDB00Cu64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// Gets the heading of the entity physics in degrees, which tends to be more accurate than just [`GET_ENTITY_HEADING`](#_0xE83D4F9BA2A38914). This can be clearly seen while, for example, ragdolling a ped/player.



pub fn get_entity_heading_from_eulers_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x846BF6291198A71Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x846BF6291198A71Eu64;
        
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
pub fn get_entity_heading_from_eulers_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x846BF6291198A71Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x846BF6291198A71Eu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
LOD distance can be 0 to 0xFFFF (higher values will result in 0xFFFF) as it is actually stored as a 16-bit value (aka uint16_t).  
```



pub fn set_entity_lod_dist_safe(
        
        
            entity: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5927F96A78577363u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5927F96A78577363u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_lod_dist_raw(
        entity: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5927F96A78577363u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5927F96A78577363u64;

        invoke_raw_typed!(
            hash,
                entity, 
                value
        )
    }
}

/// Loads collision grid for an entity spawned outside of a player's loaded area. This allows peds to execute tasks rather than sit dormant because of a lack of a physics grid.

Certainly not the main usage of this native but when set to true for a Vehicle, it will prevent the vehicle to explode if it is spawned far away from the player.  

```
NativeDB Added Parameter 3: Any p2
```



pub fn set_entity_load_collision_flag_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0DC7CABAB1E9B67Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0DC7CABAB1E9B67Eu64;
        
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
pub fn set_entity_load_collision_flag_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0DC7CABAB1E9B67Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0DC7CABAB1E9B67Eu64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ```
Simply returns whatever is passed to it (Regardless of whether the handle is valid or not).  
```



pub fn get_ped_index_from_entity_index_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04A2A40C73395041u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04A2A40C73395041u64;
        
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
pub fn get_ped_index_from_entity_index_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04A2A40C73395041u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04A2A40C73395041u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// When setting health for a player ped, the game will clamp the health value to ensure it does not exceed the maximum health. This maximum health can be retrieved by calling [`GET_PED_MAX_HEALTH`](#_0x4700A416E8324EF3). It can also be modified by calling [`SET_PED_MAX_HEALTH`](#_0xF5F6378C4F3419D3).

When setting the health for non-player peds or entities, the maximum health will be increased if the new health value exceeds the current maximum.

Default health for male peds is `200`, for female peds it is `175`.



pub fn set_entity_health_safe(
        
        
            entity: 
        , 
        
        
            health: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B76DC1F3AE6E6A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B76DC1F3AE6E6A3u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                health
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_health_raw(
        entity: , 
        health: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B76DC1F3AE6E6A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B76DC1F3AE6E6A3u64;

        invoke_raw_typed!(
            hash,
                entity, 
                health
        )
    }
}

/// Determines whether the screen position of the specified entity is within the 2D bounds of the screen.

This native will not check if the entity is not visible due to being occluded (for example, behind a wall). To check if a entity is on screen and is not occluded, use [IS_ENTITY_OCCLUDED](#_0xE31C2C72B8692B64).



pub fn is_entity_on_screen_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE659E47AF827484Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE659E47AF827484Bu64;
        
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
pub fn is_entity_on_screen_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE659E47AF827484Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE659E47AF827484Bu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn is_an_entity_safe(
        
        
            handle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x731EC8A916BD11A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x731EC8A916BD11A1u64;
        
        let result = invoke_raw!(
            hash,
                handle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_an_entity_raw(
        handle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x731EC8A916BD11A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x731EC8A916BD11A1u64;

        invoke_raw_typed!(
            hash,
                handle
        )
    }
}

/// ```
Returns an integer value of entity's current health.
Example of range for ped:
- Player [0 to 200]
- Ped [100 to 200]
- Vehicle [0 to 1000]
- Object [0 to 1000]
Health is actually a float value but this native casts it to int.
In order to get the actual value, do:
float health = *(float *)(entityAddress + 0x280);
```



pub fn get_entity_health_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEF059FAD016D209u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEF059FAD016D209u64;
        
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
pub fn get_entity_health_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEF059FAD016D209u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEF059FAD016D209u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn _0x36f32de87082343e_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36F32DE87082343Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36F32DE87082343Eu64;
        
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
pub fn _0x36f32de87082343e_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36F32DE87082343Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36F32DE87082343Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _has_entity_clear_los_to_entity_2_safe(
        
        
            entity1: 
        , 
        
        
            entity2: 
        , 
        
        
            traceType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x394BDE2A7BBA031Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x394BDE2A7BBA031Eu64;
        
        let result = invoke_raw!(
            hash,
                entity1, 
                entity2, 
                traceType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _has_entity_clear_los_to_entity_2_raw(
        entity1: , 
        entity2: , 
        traceType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x394BDE2A7BBA031Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x394BDE2A7BBA031Eu64;

        invoke_raw_typed!(
            hash,
                entity1, 
                entity2, 
                traceType
        )
    }
}

/// ## Parameters
*



pub fn create_model_hide_excluding_script_objects_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            model: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A52AE588830BF7Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A52AE588830BF7Fu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                model, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_model_hide_excluding_script_objects_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        model: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A52AE588830BF7Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A52AE588830BF7Fu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                model, 
                p5
        )
    }
}

/// [Animations list](https://alexguirre.github.io/animations-list/)



pub fn play_entity_anim_safe(
        
        
            entity: 
        , 
        
        
            animName: 
        , 
        
        
            animDict: 
        , 
        
        
            fBlendDelta: 
        , 
        
        
            bLoop: 
        , 
        
        
            bHoldLastFrame: 
        , 
        
        
            bDriveToPose: 
        , 
        
        
            fStartPhase: 
        , 
        
        
            iFlags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FB218262B810701u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FB218262B810701u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                animName, 
                animDict, 
                fBlendDelta, 
                bLoop, 
                bHoldLastFrame, 
                bDriveToPose, 
                fStartPhase, 
                iFlags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_entity_anim_raw(
        entity: , 
        animName: , 
        animDict: , 
        fBlendDelta: , 
        bLoop: , 
        bHoldLastFrame: , 
        bDriveToPose: , 
        fStartPhase: , 
        iFlags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FB218262B810701u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FB218262B810701u64;

        invoke_raw_typed!(
            hash,
                entity, 
                animName, 
                animDict, 
                fBlendDelta, 
                bLoop, 
                bHoldLastFrame, 
                bDriveToPose, 
                fStartPhase, 
                iFlags
        )
    }
}

/// ```
SET_ENTITY_*  
```



pub fn _0x1a092bb0c3808b96_safe(
        
        
            entity: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A092BB0C3808B96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A092BB0C3808B96u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x1a092bb0c3808b96_raw(
        entity: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A092BB0C3808B96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A092BB0C3808B96u64;

        invoke_raw_typed!(
            hash,
                entity, 
                p1
        )
    }
}

/// ```
For instance: ENTITY::SET_ENTITY_MAX_HEALTH(PLAYER::PLAYER_PED_ID(), 200); // director_mode.c4: 67849  
```



pub fn set_entity_max_health_safe(
        
        
            entity: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x166E7CF68597D8B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x166E7CF68597D8B5u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_max_health_raw(
        entity: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x166E7CF68597D8B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x166E7CF68597D8B5u64;

        invoke_raw_typed!(
            hash,
                entity, 
                value
        )
    }
}

/// ```
p5 = sets as true in scripts  
Same as the comment for CREATE_MODEL_SWAP unless for some reason p5 affects it this only works with objects as well.  
Network players do not see changes done with this.  
```



pub fn create_model_hide_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            model: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A97BCA30A0CE478u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A97BCA30A0CE478u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                model, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_model_hide_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        model: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A97BCA30A0CE478u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A97BCA30A0CE478u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                model, 
                p5
        )
    }
}

/// ```
Has the entity1 got a clear line of sight to the other entity2 from the direction entity1 is facing.  
This is one of the most CPU demanding BOOL natives in the game; avoid calling this in things like nested for-loops  
```



pub fn has_entity_clear_los_to_entity_in_front_safe(
        
        
            entity1: 
        , 
        
        
            entity2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0267D00AF114F17Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0267D00AF114F17Au64;
        
        let result = invoke_raw!(
            hash,
                entity1, 
                entity2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_entity_clear_los_to_entity_in_front_raw(
        entity1: , 
        entity2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0267D00AF114F17Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0267D00AF114F17Au64;

        invoke_raw_typed!(
            hash,
                entity1, 
                entity2
        )
    }
}

/// ```
NativeDB Added Parameter 2: BOOL p1
```



pub fn is_entity_dead_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F9532F3B5CC2551u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F9532F3B5CC2551u64;
        
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
pub fn is_entity_dead_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F9532F3B5CC2551u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F9532F3B5CC2551u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn create_forced_object_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p3: 
        , 
        
        
            modelHash: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x150E808B375A385Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x150E808B375A385Au64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                p3, 
                modelHash, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_forced_object_raw(
        x: , 
        y: , 
        z: , 
        p3: , 
        modelHash: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x150E808B375A385Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x150E808B375A385Au64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                p3, 
                modelHash, 
                p5
        )
    }
}

/// ```
ENABLE_*
```



pub fn _enable_entity_unk_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CE177D014502E8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CE177D014502E8Au64;
        
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
pub fn _enable_entity_unk_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CE177D014502E8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CE177D014502E8Au64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn set_entity_has_gravity_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A4722448F18EEF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A4722448F18EEF5u64;
        
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
pub fn set_entity_has_gravity_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A4722448F18EEF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A4722448F18EEF5u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn has_entity_been_damaged_by_entity_safe(
        
        
            entity: 
        , 
        
        
            damager: 
        , 
        
        
            bCheckDamagerVehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC86D67D52A707CF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC86D67D52A707CF8u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                damager, 
                bCheckDamagerVehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_entity_been_damaged_by_entity_raw(
        entity: , 
        damager: , 
        bCheckDamagerVehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC86D67D52A707CF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC86D67D52A707CF8u64;

        invoke_raw_typed!(
            hash,
                entity, 
                damager, 
                bCheckDamagerVehicle
        )
    }
}

/// ## Parameters
*



pub fn is_entity_in_air_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x886E37EC497200B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x886E37EC497200B6u64;
        
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
pub fn is_entity_in_air_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x886E37EC497200B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x886E37EC497200B6u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _set_entity_angular_velocity_safe(
        
        
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
        let hash = 0x8339643499D1222Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8339643499D1222Eu64;
        
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
pub fn _set_entity_angular_velocity_raw(
        entity: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8339643499D1222Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8339643499D1222Eu64;

        invoke_raw_typed!(
            hash,
                entity, 
                x, 
                y, 
                z
        )
    }
}

/// ```
Only called once in the scripts.  
Related to weapon objects.  
```



pub fn _0x5c3b791d580e0bc2_safe(
        
        
            entity: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C3B791D580E0BC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C3B791D580E0BC2u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5c3b791d580e0bc2_raw(
        entity: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C3B791D580E0BC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C3B791D580E0BC2u64;

        invoke_raw_typed!(
            hash,
                entity, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn does_entity_belong_to_this_script_safe(
        
        
            entity: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDE6DF5AE89981D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDE6DF5AE89981D2u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_entity_belong_to_this_script_raw(
        entity: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDE6DF5AE89981D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDE6DF5AE89981D2u64;

        invoke_raw_typed!(
            hash,
                entity, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn is_entity_attached_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB346476EF1A64897u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB346476EF1A64897u64;
        
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
pub fn is_entity_attached_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB346476EF1A64897u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB346476EF1A64897u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Relative can be used for getting speed relative to the frame of the vehicle, to determine for example, if you are going in reverse (-y speed) or not (+y speed).  
```



pub fn get_entity_speed_vector_safe(
        
        
            entity: 
        , 
        
        
            relative: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A8D700A51CB7B0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A8D700A51CB7B0Du64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                relative
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_entity_speed_vector_raw(
        entity: , 
        relative: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A8D700A51CB7B0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A8D700A51CB7B0Du64;

        invoke_raw_typed!(
            hash,
                entity, 
                relative
        )
    }
}

/// ## Parameters
*



pub fn is_entity_occluded_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE31C2C72B8692B64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE31C2C72B8692B64u64;
        
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
pub fn is_entity_occluded_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE31C2C72B8692B64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE31C2C72B8692B64u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Returns the index of the bone. If the bone was not found, -1 will be returned.   
list:  
pastebin.com/D7JMnX1g  
BoneNames:  
	chassis,  
	windscreen,  
	seat_pside_r,  
	seat_dside_r,  
	bodyshell,  
	suspension_lm,  
	suspension_lr,  
	platelight,  
	attach_female,  
	attach_male,  
	bonnet,  
	boot,  
	chassis_dummy,	//Center of the dummy  
	chassis_Control,	//Not found yet  
	door_dside_f,	//Door left, front  
	door_dside_r,	//Door left, back  
	door_pside_f,	//Door right, front  
	door_pside_r,	//Door right, back  
	Gun_GripR,  
	windscreen_f,  
	platelight,	//Position where the light above the numberplate is located  
	VFX_Emitter,  
	window_lf,	//Window left, front  
	window_lr,	//Window left, back  
	window_rf,	//Window right, front  
	window_rr,	//Window right, back  
	engine,	//Position of the engine  
	gun_ammo,  
	ROPE_ATTATCH,	//Not misspelled. In script "finale_heist2b.c4".  
	wheel_lf,	//Wheel left, front  
	wheel_lr,	//Wheel left, back  
	wheel_rf,	//Wheel right, front  
	wheel_rr,	//Wheel right, back  
	exhaust,	//Exhaust. shows only the position of the stock-exhaust  
	overheat,	//A position on the engine(not exactly sure, how to name it)  
	misc_e,	//Not a car-bone.  
	seat_dside_f,	//Driver-seat  
	seat_pside_f,	//Seat next to driver  
	Gun_Nuzzle,  
	seat_r  
I doubt that the function is case-sensitive, since I found a "Chassis" and a "chassis". - Just tested: Definitely not case-sensitive.  
```



pub fn get_entity_bone_index_by_name_safe(
        
        
            entity: 
        , 
        
        
            boneName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB71170B7E76ACBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB71170B7E76ACBAu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                boneName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_entity_bone_index_by_name_raw(
        entity: , 
        boneName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB71170B7E76ACBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB71170B7E76ACBAu64;

        invoke_raw_typed!(
            hash,
                entity, 
                boneName
        )
    }
}

/// ## Parameters
*



pub fn detach_entity_safe(
        
        
            entity: 
        , 
        
        
            dynamic: 
        , 
        
        
            collision: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x961AC54BF0613F5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x961AC54BF0613F5Du64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                dynamic, 
                collision
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn detach_entity_raw(
        entity: , 
        dynamic: , 
        collision: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x961AC54BF0613F5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x961AC54BF0613F5Du64;

        invoke_raw_typed!(
            hash,
                entity, 
                dynamic, 
                collision
        )
    }
}

/// ## Parameters
*



pub fn set_entity_max_speed_safe(
        
        
            entity: 
        , 
        
        
            speed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E46A3FCBDE2A1B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E46A3FCBDE2A1B1u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                speed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_max_speed_raw(
        entity: , 
        speed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E46A3FCBDE2A1B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E46A3FCBDE2A1B1u64;

        invoke_raw_typed!(
            hash,
                entity, 
                speed
        )
    }
}

/// ## Parameters
*



pub fn set_entity_always_prerender_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xACAD101E1FB66689u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xACAD101E1FB66689u64;
        
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
pub fn set_entity_always_prerender_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xACAD101E1FB66689u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xACAD101E1FB66689u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn has_entity_been_damaged_by_any_ped_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x605F5A140F202491u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x605F5A140F202491u64;
        
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
pub fn has_entity_been_damaged_by_any_ped_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x605F5A140F202491u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x605F5A140F202491u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
A static ped will not react to natives like "APPLY_FORCE_TO_ENTITY" or "SET_ENTITY_VELOCITY" and oftentimes will not react to task-natives like "AI::TASK_COMBAT_PED". The only way I know of to make one of these peds react is to ragdoll them (or sometimes to use CLEAR_PED_TASKS_IMMEDIATELY(). Static peds include almost all far-away peds, beach-combers, peds in certain scenarios, peds crossing a crosswalk, peds walking to get back into their cars, and others. If anyone knows how to make a ped non-static without ragdolling them, please edit this with the solution.  
^ Attach a phCollider to the ped.  
```



pub fn is_entity_static_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1218E6886D3D8327u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1218E6886D3D8327u64;
        
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
pub fn is_entity_static_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1218E6886D3D8327u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1218E6886D3D8327u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// A population type, from the following enum: https://alloc8or.re/gta5/doc/enums/ePopulationType.txt



pub fn get_entity_population_type_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6F5161F4534EDFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6F5161F4534EDFFu64;
        
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
pub fn get_entity_population_type_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6F5161F4534EDFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6F5161F4534EDFFu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn reset_entity_alpha_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B1E824FFBB7027Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B1E824FFBB7027Au64;
        
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
pub fn reset_entity_alpha_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B1E824FFBB7027Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B1E824FFBB7027Au64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn is_entity_a_mission_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A7B270912999B3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A7B270912999B3Cu64;
        
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
pub fn is_entity_a_mission_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A7B270912999B3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A7B270912999B3Cu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn set_can_auto_vault_on_entity_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE12ABE5E3A389A6Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE12ABE5E3A389A6Cu64;
        
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
pub fn set_can_auto_vault_on_entity_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE12ABE5E3A389A6Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE12ABE5E3A389A6Cu64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ```
Displays the current ROLL axis of the entity [-180.0000/180.0000+]  
(Sideways Roll) such as a vehicle tipped on its side  
```



pub fn get_entity_roll_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x831E0242595560DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x831E0242595560DFu64;
        
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
pub fn get_entity_roll_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x831E0242595560DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x831E0242595560DFu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
SET_ENTITY_*
```



pub fn _0xc34bc448da29f5e9_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC34BC448DA29F5E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC34BC448DA29F5E9u64;
        
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
pub fn _0xc34bc448da29f5e9_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC34BC448DA29F5E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC34BC448DA29F5E9u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ```
Returns a float value representing animation's current playtime with respect to its total playtime. This value increasing in a range from [0 to 1] and wrap back to 0 when it reach 1.  
Example:  
0.000000 - mark the starting of animation.  
0.500000 - mark the midpoint of the animation.  
1.000000 - mark the end of animation.  
```

[Animations list](https://alexguirre.github.io/animations-list/)



pub fn get_entity_anim_current_time_safe(
        
        
            entity: 
        , 
        
        
            animDict: 
        , 
        
        
            animName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x346D81500D088F42u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x346D81500D088F42u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                animDict, 
                animName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_entity_anim_current_time_raw(
        entity: , 
        animDict: , 
        animName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x346D81500D088F42u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x346D81500D088F42u64;

        invoke_raw_typed!(
            hash,
                entity, 
                animDict, 
                animName
        )
    }
}

/// ## Parameters
*



pub fn is_entity_waiting_for_world_collision_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD05BFF0C0A12C68Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD05BFF0C0A12C68Fu64;
        
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
pub fn is_entity_waiting_for_world_collision_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD05BFF0C0A12C68Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD05BFF0C0A12C68Fu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn is_entity_attached_to_any_vehicle_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26AA915AD89BFB4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26AA915AD89BFB4Bu64;
        
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
pub fn is_entity_attached_to_any_vehicle_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26AA915AD89BFB4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26AA915AD89BFB4Bu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn get_entity_attached_to_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48C2BED9180FE123u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48C2BED9180FE123u64;
        
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
pub fn get_entity_attached_to_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48C2BED9180FE123u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48C2BED9180FE123u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// This native



pub fn clear_entity_last_damage_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA72CD9CA74A5ECBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA72CD9CA74A5ECBAu64;
        
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
pub fn clear_entity_last_damage_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA72CD9CA74A5ECBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA72CD9CA74A5ECBAu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn is_entity_touching_model_safe(
        
        
            entity: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F42323798A58C8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F42323798A58C8Cu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_entity_touching_model_raw(
        entity: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F42323798A58C8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F42323798A58C8Cu64;

        invoke_raw_typed!(
            hash,
                entity, 
                modelHash
        )
    }
}

/// ```
Returns the heading of the entity in degrees. Also know as the "Yaw" of an entity.
```



pub fn get_entity_heading_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE83D4F9BA2A38914u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE83D4F9BA2A38914u64;
        
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
pub fn get_entity_heading_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE83D4F9BA2A38914u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE83D4F9BA2A38914u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn remove_forced_object_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61B6775E83C0DB6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61B6775E83C0DB6Fu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_forced_object_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61B6775E83C0DB6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61B6775E83C0DB6Fu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ```
w is the correct parameter name!  
```



pub fn set_entity_quaternion_safe(
        
        
            entity: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            w: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77B21BE7AC540F07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77B21BE7AC540F07u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                x, 
                y, 
                z, 
                w
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_quaternion_raw(
        entity: , 
        x: , 
        y: , 
        z: , 
        w: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77B21BE7AC540F07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77B21BE7AC540F07u64;

        invoke_raw_typed!(
            hash,
                entity, 
                x, 
                y, 
                z, 
                w
        )
    }
}

/// ```
breakForce is the amount of force required to break the bond.
p14 - is always 1 in scripts
p15 - is 1 or 0 in scripts - unknown what it does
p16 - controls collision between the two entities (FALSE disables collision).
p17 - do not teleport entity to be attached to the position of the bone Index of the target entity (if 1, entity will not be teleported to target bone)
p18 - is always 2 in scripts.
```



pub fn attach_entity_to_entity_physically_safe(
        
        
            entity1: 
        , 
        
        
            entity2: 
        , 
        
        
            boneIndex1: 
        , 
        
        
            boneIndex2: 
        , 
        
        
            xPos1: 
        , 
        
        
            yPos1: 
        , 
        
        
            zPos1: 
        , 
        
        
            xPos2: 
        , 
        
        
            yPos2: 
        , 
        
        
            zPos2: 
        , 
        
        
            xRot: 
        , 
        
        
            yRot: 
        , 
        
        
            zRot: 
        , 
        
        
            breakForce: 
        , 
        
        
            fixedRot: 
        , 
        
        
            p15: 
        , 
        
        
            collision: 
        , 
        
        
            teleport: 
        , 
        
        
            p18: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3675780C92F90F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3675780C92F90F9u64;
        
        let result = invoke_raw!(
            hash,
                entity1, 
                entity2, 
                boneIndex1, 
                boneIndex2, 
                xPos1, 
                yPos1, 
                zPos1, 
                xPos2, 
                yPos2, 
                zPos2, 
                xRot, 
                yRot, 
                zRot, 
                breakForce, 
                fixedRot, 
                p15, 
                collision, 
                teleport, 
                p18
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn attach_entity_to_entity_physically_raw(
        entity1: , 
        entity2: , 
        boneIndex1: , 
        boneIndex2: , 
        xPos1: , 
        yPos1: , 
        zPos1: , 
        xPos2: , 
        yPos2: , 
        zPos2: , 
        xRot: , 
        yRot: , 
        zRot: , 
        breakForce: , 
        fixedRot: , 
        p15: , 
        collision: , 
        teleport: , 
        p18: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3675780C92F90F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3675780C92F90F9u64;

        invoke_raw_typed!(
            hash,
                entity1, 
                entity2, 
                boneIndex1, 
                boneIndex2, 
                xPos1, 
                yPos1, 
                zPos1, 
                xPos2, 
                yPos2, 
                zPos2, 
                xRot, 
                yRot, 
                zRot, 
                breakForce, 
                fixedRot, 
                p15, 
                collision, 
                teleport, 
                p18
        )
    }
}

/// ## Parameters
*



pub fn get_entity_matrix_safe(
        
        
            entity: 
        , 
        
        
            forwardVector: 
        , 
        
        
            rightVector: 
        , 
        
        
            upVector: 
        , 
        
        
            position: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xECB2FC7235A7D137u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xECB2FC7235A7D137u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                forwardVector, 
                rightVector, 
                upVector, 
                position
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_entity_matrix_raw(
        entity: , 
        forwardVector: , 
        rightVector: , 
        upVector: , 
        position: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xECB2FC7235A7D137u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xECB2FC7235A7D137u64;

        invoke_raw_typed!(
            hash,
                entity, 
                forwardVector, 
                rightVector, 
                upVector, 
                position
        )
    }
}

/// Toggle the visibility of a given entity.



pub fn set_entity_visible_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        , 
        
        
            unk: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA1C610A04DB6BBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA1C610A04DB6BBBu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                toggle, 
                unk
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_visible_raw(
        entity: , 
        toggle: , 
        unk: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA1C610A04DB6BBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA1C610A04DB6BBBu64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle, 
                unk
        )
    }
}

/// ## Parameters
*



pub fn does_entity_have_drawable_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x060D6E96F8B8E48Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x060D6E96F8B8E48Du64;
        
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
pub fn does_entity_have_drawable_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x060D6E96F8B8E48Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x060D6E96F8B8E48Du64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn get_nearest_player_to_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7196842CB375CDB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7196842CB375CDB3u64;
        
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
pub fn get_nearest_player_to_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7196842CB375CDB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7196842CB375CDB3u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// Get the speed of a entity.



pub fn get_entity_speed_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5037BA82E12416Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5037BA82E12416Fu64;
        
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
pub fn get_entity_speed_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5037BA82E12416Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5037BA82E12416Fu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
NativeDB Introduced: v3407
```



pub fn _set_entity_no_collision_with_networked_entity_safe(
        
        
            entity1: 
        , 
        
        
            entity2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A27A7827347B3B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A27A7827347B3B1u64;
        
        let result = invoke_raw!(
            hash,
                entity1, 
                entity2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_entity_no_collision_with_networked_entity_raw(
        entity1: , 
        entity2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A27A7827347B3B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A27A7827347B3B1u64;

        invoke_raw_typed!(
            hash,
                entity1, 
                entity2
        )
    }
}

/// ```
NativeDB Introduced: v2699
```



pub fn _does_entity_have_anim_director_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2158E81A6AF65EA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2158E81A6AF65EA9u64;
        
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
pub fn _does_entity_have_anim_director_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2158E81A6AF65EA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2158E81A6AF65EA9u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn set_entity_can_be_damaged_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1760FFA8AB074D66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1760FFA8AB074D66u64;
        
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
pub fn set_entity_can_be_damaged_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1760FFA8AB074D66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1760FFA8AB074D66u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_entity_upright_safe(
        
        
            entity: 
        , 
        
        
            angle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5333F526F6AB19AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5333F526F6AB19AAu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                angle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_entity_upright_raw(
        entity: , 
        angle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5333F526F6AB19AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5333F526F6AB19AAu64;

        invoke_raw_typed!(
            hash,
                entity, 
                angle
        )
    }
}

/// Gets the current coordinates (world position) for a specified entity.



pub fn get_entity_coords_safe(
        
        
            entity: 
        , 
        
        
            alive: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FEF770D40960D5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FEF770D40960D5Au64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                alive
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_entity_coords_raw(
        entity: , 
        alive: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FEF770D40960D5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FEF770D40960D5Au64;

        invoke_raw_typed!(
            hash,
                entity, 
                alive
        )
    }
}

/// Apply a force to an entities center of mass.



pub fn apply_force_to_entity_center_of_mass_safe(
        
        
            entity: 
        , 
        
        
            forceType: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x18FF00FC7EFF559Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x18FF00FC7EFF559Eu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                forceType, 
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
pub fn apply_force_to_entity_center_of_mass_raw(
        entity: , 
        forceType: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x18FF00FC7EFF559Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x18FF00FC7EFF559Eu64;

        invoke_raw_typed!(
            hash,
                entity, 
                forceType, 
                x, 
                y, 
                z
        )
    }
}

/// [Animations list](https://alexguirre.github.io/animations-list/)



pub fn set_entity_anim_current_time_safe(
        
        
            entity: 
        , 
        
        
            animDictionary: 
        , 
        
        
            animName: 
        , 
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4487C259F0F70977u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4487C259F0F70977u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                animDictionary, 
                animName, 
                time
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_anim_current_time_raw(
        entity: , 
        animDictionary: , 
        animName: , 
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4487C259F0F70977u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4487C259F0F70977u64;

        invoke_raw_typed!(
            hash,
                entity, 
                animDictionary, 
                animName, 
                time
        )
    }
}

/// ## Parameters
*



pub fn get_entity_height_safe(
        
        
            entity: 
        , 
        
        
            X: 
        , 
        
        
            Y: 
        , 
        
        
            Z: 
        , 
        
        
            atTop: 
        , 
        
        
            inWorldCoords: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A504562485944DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A504562485944DDu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                X, 
                Y, 
                Z, 
                atTop, 
                inWorldCoords
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_entity_height_raw(
        entity: , 
        X: , 
        Y: , 
        Z: , 
        atTop: , 
        inWorldCoords: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A504562485944DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A504562485944DDu64;

        invoke_raw_typed!(
            hash,
                entity, 
                X, 
                Y, 
                Z, 
                atTop, 
                inWorldCoords
        )
    }
}

/// ```
Calling this function disables collision between two entities.
The importance of the order for entity1 and entity2 is unclear.
The third parameter, `thisFrame`, decides whether the collision is to be disabled until it is turned back on, or if it's just this frame.
```



pub fn set_entity_no_collision_entity_safe(
        
        
            entity1: 
        , 
        
        
            entity2: 
        , 
        
        
            thisFrameOnly: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA53ED5520C07654Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA53ED5520C07654Au64;
        
        let result = invoke_raw!(
            hash,
                entity1, 
                entity2, 
                thisFrameOnly
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_no_collision_entity_raw(
        entity1: , 
        entity2: , 
        thisFrameOnly: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA53ED5520C07654Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA53ED5520C07654Au64;

        invoke_raw_typed!(
            hash,
                entity1, 
                entity2, 
                thisFrameOnly
        )
    }
}

/// Sets the coordinates (world position) for a specified entity, offset by the radius of the entity on the Z axis.



pub fn set_entity_coords_safe(
        
        
            entity: 
        , 
        
        
            xPos: 
        , 
        
        
            yPos: 
        , 
        
        
            zPos: 
        , 
        
        
            alive: 
        , 
        
        
            deadFlag: 
        , 
        
        
            ragdollFlag: 
        , 
        
        
            clearArea: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06843DA7060A026Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06843DA7060A026Bu64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                xPos, 
                yPos, 
                zPos, 
                alive, 
                deadFlag, 
                ragdollFlag, 
                clearArea
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_coords_raw(
        entity: , 
        xPos: , 
        yPos: , 
        zPos: , 
        alive: , 
        deadFlag: , 
        ragdollFlag: , 
        clearArea: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06843DA7060A026Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06843DA7060A026Bu64;

        invoke_raw_typed!(
            hash,
                entity, 
                xPos, 
                yPos, 
                zPos, 
                alive, 
                deadFlag, 
                ragdollFlag, 
                clearArea
        )
    }
}

/// ## Parameters

-



pub fn is_entity_a_ped_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x524AC5ECEA15343Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x524AC5ECEA15343Eu64;
        
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
pub fn is_entity_a_ped_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x524AC5ECEA15343Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x524AC5ECEA15343Eu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Offset values are relative to the entity.  
x = left/right  
y = forward/backward  
z = up/down  
```



pub fn get_offset_from_entity_in_world_coords_safe(
        
        
            entity: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1899F328B0E12848u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1899F328B0E12848u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                offsetX, 
                offsetY, 
                offsetZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_offset_from_entity_in_world_coords_raw(
        entity: , 
        offsetX: , 
        offsetY: , 
        offsetZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1899F328B0E12848u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1899F328B0E12848u64;

        invoke_raw_typed!(
            hash,
                entity, 
                offsetX, 
                offsetY, 
                offsetZ
        )
    }
}

/// Configures an entity to either allow or prevent it from being picked up by Cargobobs.

```
NativeDB Introduced: v1180
```



pub fn set_pick_up_by_cargobob_disabled_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7B80E7C3BEFC396u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7B80E7C3BEFC396u64;
        
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
pub fn set_pick_up_by_cargobob_disabled_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7B80E7C3BEFC396u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7B80E7C3BEFC396u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
w is the correct parameter name!  
```



pub fn get_entity_quaternion_safe(
        
        
            entity: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            w: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B3703D2D32DFA18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B3703D2D32DFA18u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                x, 
                y, 
                z, 
                w
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_entity_quaternion_raw(
        entity: , 
        x: , 
        y: , 
        z: , 
        w: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B3703D2D32DFA18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B3703D2D32DFA18u64;

        invoke_raw_typed!(
            hash,
                entity, 
                x, 
                y, 
                z, 
                w
        )
    }
}

/// ```
p1 sync task id?  
```



pub fn stop_synchronized_entity_anim_safe(
        
        
            entity: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43D3807C077261E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43D3807C077261E3u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_synchronized_entity_anim_raw(
        entity: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43D3807C077261E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43D3807C077261E3u64;

        invoke_raw_typed!(
            hash,
                entity, 
                p1, 
                p2
        )
    }
}

/// ```
Sets a ped or an object totally invincible. It doesn't take any kind of damage. Peds will not ragdoll on explosions and the tazer animation won't apply either.  
If you use this for a ped and you want Ragdoll to stay enabled, then do:  
*(DWORD *)(pedAddress + 0x188) |= (1 << 9);  
Use this if you want to get the invincibility status:  
	bool IsPedInvincible(Ped ped)  
	{  
auto addr = getScriptHandleBaseAddress(ped);	  
if (addr)  
{  
	DWORD flag = *(DWORD *)(addr + 0x188);  
	return ((flag & (1 << 8)) != 0) || ((flag & (1 << 9)) != 0);  
}  
return false;  
	}  
```



pub fn set_entity_invincible_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3882114BDE571AD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3882114BDE571AD4u64;
        
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
pub fn set_entity_invincible_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3882114BDE571AD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3882114BDE571AD4u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn does_entity_have_physics_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA95EA3317CC5064u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA95EA3317CC5064u64;
        
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
pub fn does_entity_have_physics_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA95EA3317CC5064u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA95EA3317CC5064u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// [Animations list](https://alexguirre.github.io/animations-list/)



pub fn get_anim_duration_safe(
        
        
            animDict: 
        , 
        
        
            animName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEDDF04D62B8D790u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEDDF04D62B8D790u64;
        
        let result = invoke_raw!(
            hash,
                animDict, 
                animName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_anim_duration_raw(
        animDict: , 
        animName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEDDF04D62B8D790u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEDDF04D62B8D790u64;

        invoke_raw_typed!(
            hash,
                animDict, 
                animName
        )
    }
}

/// See also [`IS_SCRIPTED_SCENARIO_PED_USING_CONDITIONAL_ANIM`](#_0x6EC47A344923E1ED)

```
Taken from ENTITY::IS_ENTITY_PLAYING_ANIM(PLAYER::PLAYER_PED_ID(), "creatures@shark@move", "attack_player", 3)  
p4 is always 3 in the scripts.  
taskFlag:  
2 - Check synchronized scene  
```

[Animations list](https://alexguirre.github.io/animations-list/)



pub fn is_entity_playing_anim_safe(
        
        
            entity: 
        , 
        
        
            animDict: 
        , 
        
        
            animName: 
        , 
        
        
            taskFlag: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F0B79228E461EC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F0B79228E461EC9u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                animDict, 
                animName, 
                taskFlag
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_entity_playing_anim_raw(
        entity: , 
        animDict: , 
        animName: , 
        taskFlag: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F0B79228E461EC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F0B79228E461EC9u64;

        invoke_raw_typed!(
            hash,
                entity, 
                animDict, 
                animName, 
                taskFlag
        )
    }
}

/// Changing traffic-lights will not change the behavior of NPCs.

Example: [here](https://www.gtaforums.com/topic/830463-help-with-turning-lights-green-and-causing-peds-to-crash-into-each-other/#entry1068211340)

```c
enum eTrafficlightOverrideMode
{
    TLO_RED = 0,
    TLO_AMBER = 1,
    TLO_GREEN = 2,
    TLO_NONE = 3
}
```



pub fn set_entity_trafficlight_override_safe(
        
        
            entity: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57C5DB656185EAC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57C5DB656185EAC4u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_trafficlight_override_raw(
        entity: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57C5DB656185EAC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57C5DB656185EAC4u64;

        invoke_raw_typed!(
            hash,
                entity, 
                state
        )
    }
}

/// ```
This is an alias of SET_ENTITY_AS_NO_LONGER_NEEDED.  
```



pub fn set_object_as_no_longer_needed_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3AE22DEB5BA5A3E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3AE22DEB5BA5A3E6u64;
        
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
pub fn set_object_as_no_longer_needed_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3AE22DEB5BA5A3E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3AE22DEB5BA5A3E6u64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ## Parameters
*



pub fn set_entity_records_collisions_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A50A1EEDAD01E65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A50A1EEDAD01E65u64;
        
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
pub fn set_entity_records_collisions_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A50A1EEDAD01E65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A50A1EEDAD01E65u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

