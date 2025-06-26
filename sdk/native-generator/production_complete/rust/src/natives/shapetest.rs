//! SHAPETEST native functions
//! 
//! Functions for the shapetest category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// Performs the same type of trace as START_SHAPE_TEST_CAPSULE, but with some different hardcoded parameters.



pub fn start_shape_test_swept_sphere_safe(
        
        
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
        
        
            radius: 
        , 
        
        
            flags: 
        , 
        
        
            entity: 
        , 
        
        
            p9: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6AC6C45FBE83004u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6AC6C45FBE83004u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                radius, 
                flags, 
                entity, 
                p9
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_shape_test_swept_sphere_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        radius: , 
        flags: , 
        entity: , 
        p9: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6AC6C45FBE83004u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6AC6C45FBE83004u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                radius, 
                flags, 
                entity, 
                p9
        )
    }
}

/// Asynchronously starts a line-of-sight (raycast) world probe shape test.

```c
enum eTraceFlags
{
  None = 0,
  IntersectWorld = 1,
  IntersectVehicles = 2,
  IntersectPeds = 4,
  IntersectRagdolls = 8,
  IntersectObjects = 16,
  IntersectPickup = 32,
  IntersectGlass = 64,
  IntersectRiver = 128,
  IntersectFoliage = 256,

  IntersectEverything = 511
}
```

```c
enum eTraceOptionFlags
{
  None = 0,
  OptionIgnoreGlass = 1,
  OptionIgnoreSeeThrough = 2,
  OptionIgnoreNoCollision = 4,

  OptionDefault = 7
}
```

NOTE: Raycasts that intersect with mission_entites (flag = 2) has limited range and will not register for far away entites. The range seems to be about 30 metres.  

Use the handle with [GET_SHAPE_TEST_RESULT](#_0x3D87450E15D98694) or [GET_SHAPE_TEST_RESULT_INCLUDING_MATERIAL](#_0x65287525D951F6BE) until it returns 0 or 2.



pub fn start_shape_test_los_probe_safe(
        
        
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
        
        
            traceFlags: 
        , 
        
        
            entity: 
        , 
        
        
            optionFlags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EE9F5D83DD4F90Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EE9F5D83DD4F90Eu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                traceFlags, 
                entity, 
                optionFlags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_shape_test_los_probe_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        traceFlags: , 
        entity: , 
        optionFlags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EE9F5D83DD4F90Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EE9F5D83DD4F90Eu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                traceFlags, 
                entity, 
                optionFlags
        )
    }
}

/// Raycast from point to point, where the ray has a radius.



pub fn start_shape_test_capsule_safe(
        
        
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
        
        
            radius: 
        , 
        
        
            flags: 
        , 
        
        
            entity: 
        , 
        
        
            p9: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28579D1B8F8AAC80u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28579D1B8F8AAC80u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                radius, 
                flags, 
                entity, 
                p9
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_shape_test_capsule_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        radius: , 
        flags: , 
        entity: , 
        p9: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28579D1B8F8AAC80u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28579D1B8F8AAC80u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                radius, 
                flags, 
                entity, 
                p9
        )
    }
}

/// See [`START_SHAPE_TEST_LOS_PROBE`](#_0x7EE9F5D83DD4F90E) for flags.



pub fn start_shape_test_bound_safe(
        
        
            entity: 
        , 
        
        
            flags1: 
        , 
        
        
            flags2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37181417CE7C8900u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37181417CE7C8900u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                flags1, 
                flags2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_shape_test_bound_raw(
        entity: , 
        flags1: , 
        flags2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37181417CE7C8900u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37181417CE7C8900u64;

        invoke_raw_typed!(
            hash,
                entity, 
                flags1, 
                flags2
        )
    }
}

/// Returns the result of a shape test, also returning the material of any touched surface.

When used with an asynchronous shape test, this native should be looped until returning 0 or 2, after which the handle is invalidated.

Unless the return value is 2, the other return values are undefined.



pub fn get_shape_test_result_including_material_safe(
        
        
            shapeTestHandle: 
        , 
        
        
            hit: 
        , 
        
        
            endCoords: 
        , 
        
        
            surfaceNormal: 
        , 
        
        
            materialHash: 
        , 
        
        
            entityHit: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65287525D951F6BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65287525D951F6BEu64;
        
        let result = invoke_raw!(
            hash,
                shapeTestHandle, 
                hit, 
                endCoords, 
                surfaceNormal, 
                materialHash, 
                entityHit
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_shape_test_result_including_material_raw(
        shapeTestHandle: , 
        hit: , 
        endCoords: , 
        surfaceNormal: , 
        materialHash: , 
        entityHit: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65287525D951F6BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65287525D951F6BEu64;

        invoke_raw_typed!(
            hash,
                shapeTestHandle, 
                hit, 
                endCoords, 
                surfaceNormal, 
                materialHash, 
                entityHit
        )
    }
}

/// See [`START_SHAPE_TEST_LOS_PROBE`](#_0x7EE9F5D83DD4F90E) for flags.



pub fn start_shape_test_bounding_box_safe(
        
        
            entity: 
        , 
        
        
            flags1: 
        , 
        
        
            flags2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x052837721A854EC7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x052837721A854EC7u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                flags1, 
                flags2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_shape_test_bounding_box_raw(
        entity: , 
        flags1: , 
        flags2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x052837721A854EC7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x052837721A854EC7u64;

        invoke_raw_typed!(
            hash,
                entity, 
                flags1, 
                flags2
        )
    }
}

/// Does the same as [START_SHAPE_TEST_LOS_PROBE](#_0x7EE9F5D83DD4F90E), except blocking until the shape test completes.

Use [START_SHAPE_TEST_LOS_PROBE](#_0x7EE9F5D83DD4F90E) instead. Literally. Rockstar named this correctly: it's expensive, and it's synchronous.



pub fn start_expensive_synchronous_shape_test_los_probe_safe(
        
        
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
        
        
            flags: 
        , 
        
        
            entity: 
        , 
        
        
            p8: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x377906D8A31E5586u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x377906D8A31E5586u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                flags, 
                entity, 
                p8
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_expensive_synchronous_shape_test_los_probe_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        flags: , 
        entity: , 
        p8: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x377906D8A31E5586u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x377906D8A31E5586u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                flags, 
                entity, 
                p8
        )
    }
}

/// Since it is only used in the PC version, likely some mouse-friendly shape test. Uses



pub fn _start_shape_test_surrounding_coords_safe(
        
        
            pVec1: 
        , 
        
        
            pVec2: 
        , 
        
        
            flag: 
        , 
        
        
            entity: 
        , 
        
        
            flag2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF6BE494C7987F34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF6BE494C7987F34u64;
        
        let result = invoke_raw!(
            hash,
                pVec1, 
                pVec2, 
                flag, 
                entity, 
                flag2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _start_shape_test_surrounding_coords_raw(
        pVec1: , 
        pVec2: , 
        flag: , 
        entity: , 
        flag2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF6BE494C7987F34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF6BE494C7987F34u64;

        invoke_raw_typed!(
            hash,
                pVec1, 
                pVec2, 
                flag, 
                entity, 
                flag2
        )
    }
}

/// Invalidates the entity handle passed by removing the fwScriptGuid from the entity. This should be used when receiving an
ambient entity from shape testing natives, but can also be used for other natives returning an 'irrelevant' entity handle.



pub fn release_script_guid_from_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B3334BCA57CD799u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B3334BCA57CD799u64;
        
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
pub fn release_script_guid_from_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B3334BCA57CD799u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B3334BCA57CD799u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// Returns the result of a shape test.

When used with an asynchronous shape test, this native should be looped until returning 0 or 2, after which the handle is invalidated.

Unless the return value is 2, the other return values are undefined.



pub fn get_shape_test_result_safe(
        
        
            shapeTestHandle: 
        , 
        
        
            hit: 
        , 
        
        
            endCoords: 
        , 
        
        
            surfaceNormal: 
        , 
        
        
            entityHit: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D87450E15D98694u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D87450E15D98694u64;
        
        let result = invoke_raw!(
            hash,
                shapeTestHandle, 
                hit, 
                endCoords, 
                surfaceNormal, 
                entityHit
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_shape_test_result_raw(
        shapeTestHandle: , 
        hit: , 
        endCoords: , 
        surfaceNormal: , 
        entityHit: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D87450E15D98694u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D87450E15D98694u64;

        invoke_raw_typed!(
            hash,
                shapeTestHandle, 
                hit, 
                endCoords, 
                surfaceNormal, 
                entityHit
        )
    }
}

/// For more information, see [`START_EXPENSIVE_SYNCHRONOUS_SHAPE_TEST_LOS_PROBE`](#_0x377906D8A31E5586) and [`START_SHAPE_TEST_LOS_PROBE`](#_0x7EE9F5D83DD4F90E).



pub fn start_shape_test_box_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            p9: 
        , 
        
        
            flags: 
        , 
        
        
            entity: 
        , 
        
        
            p12: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE466162C4401D18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE466162C4401D18u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                x1, 
                y1, 
                z1, 
                rotX, 
                rotY, 
                rotZ, 
                p9, 
                flags, 
                entity, 
                p12
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_shape_test_box_raw(
        x: , 
        y: , 
        z: , 
        x1: , 
        y1: , 
        z1: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        p9: , 
        flags: , 
        entity: , 
        p12: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE466162C4401D18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE466162C4401D18u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                x1, 
                y1, 
                z1, 
                rotX, 
                rotY, 
                rotZ, 
                p9, 
                flags, 
                entity, 
                p12
        )
    }
}

