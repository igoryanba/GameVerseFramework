//! CAM native functions
//! 
//! Functions for the cam category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn get_cam_anim_current_phase_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA10B2DB49E92A6B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA10B2DB49E92A6B0u64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cam_anim_current_phase_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA10B2DB49E92A6B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA10B2DB49E92A6B0u64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ## Parameters
*



pub fn set_fly_cam_horizontal_response_safe(
        
        
            cam: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x503F5920162365B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x503F5920162365B2u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_fly_cam_horizontal_response_raw(
        cam: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x503F5920162365B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x503F5920162365B2u64;

        invoke_raw_typed!(
            hash,
                cam, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn is_cam_interpolating_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x036F97C908C2B52Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x036F97C908C2B52Cu64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_cam_interpolating_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x036F97C908C2B52Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x036F97C908C2B52Cu64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ## Return value



pub fn is_cinematic_cam_rendering_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB15162CB5826E9E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB15162CB5826E9E8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_cinematic_cam_rendering_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB15162CB5826E9E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB15162CB5826E9E8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// See [`GET_FOLLOW_PED_CAM_VIEW_MODE`](#_0x8D4D46230B2C353A) for the view mode enum.



pub fn get_cam_view_mode_for_context_safe(
        
        
            context: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE778F8C7E1142E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE778F8C7E1142E2u64;
        
        let result = invoke_raw!(
            hash,
                context
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cam_view_mode_for_context_raw(
        context: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE778F8C7E1142E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE778F8C7E1142E2u64;

        invoke_raw_typed!(
            hash,
                context
        )
    }
}

/// ## Parameters
*



pub fn _0xced08cbe8ebb97c7_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCED08CBE8EBB97C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCED08CBE8EBB97C7u64;
        
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
pub fn _0xced08cbe8ebb97c7_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCED08CBE8EBB97C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCED08CBE8EBB97C7u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn get_final_rendered_cam_rot_safe(
        
        
            rotationOrder: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B4E4C817FCC2DFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B4E4C817FCC2DFBu64;
        
        let result = invoke_raw!(
            hash,
                rotationOrder
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_final_rendered_cam_rot_raw(
        rotationOrder: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B4E4C817FCC2DFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B4E4C817FCC2DFBu64;

        invoke_raw_typed!(
            hash,
                rotationOrder
        )
    }
}

/// This native sets the camera's pitch (rotation on the x-axis).



pub fn set_gameplay_cam_relative_pitch_safe(
        
        
            angle: 
        , 
        
        
            scalingFactor: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D0858B8EDFD2B7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D0858B8EDFD2B7Du64;
        
        let result = invoke_raw!(
            hash,
                angle, 
                scalingFactor
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gameplay_cam_relative_pitch_raw(
        angle: , 
        scalingFactor: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D0858B8EDFD2B7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D0858B8EDFD2B7Du64;

        invoke_raw_typed!(
            hash,
                angle, 
                scalingFactor
        )
    }
}

/// ## Parameters
*



pub fn set_third_person_aim_cam_near_clip_this_update_safe(
        
        
            distance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42156508606DE65Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42156508606DE65Eu64;
        
        let result = invoke_raw!(
            hash,
                distance
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_third_person_aim_cam_near_clip_this_update_raw(
        distance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42156508606DE65Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42156508606DE65Eu64;

        invoke_raw_typed!(
            hash,
                distance
        )
    }
}

/// ## Return value



pub fn is_cinematic_idle_cam_rendering_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA9D2AA3E326D720u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA9D2AA3E326D720u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_cinematic_idle_cam_rendering_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA9D2AA3E326D720u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA9D2AA3E326D720u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0xccd078c2665d2973_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCD078C2665D2973u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCD078C2665D2973u64;
        
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
pub fn _0xccd078c2665d2973_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCD078C2665D2973u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCD078C2665D2973u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Only used in R* Script fm_mission_controller_2020
```

```
NativeDB Introduced: v2699
```



pub fn _set_use_hi_dof_in_cutscene_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x731A880555DA3647u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x731A880555DA3647u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_use_hi_dof_in_cutscene_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x731A880555DA3647u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x731A880555DA3647u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2699
```



pub fn _get_cam_near_dof_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2612D223D915A1Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2612D223D915A1Cu64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_cam_near_dof_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2612D223D915A1Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2612D223D915A1Cu64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ## Return value



pub fn is_bonnet_cinematic_cam_rendering_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7360051C885628Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7360051C885628Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_bonnet_cinematic_cam_rendering_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7360051C885628Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7360051C885628Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Enumerated type defined in camControlHelperMetadataViewModes:

```c
enum eContext {
    ON_FOOT = 0, // [G|S]ET_FOLLOW_PED_CAM_*
    IN_VEHICLE = 1, // [G|S]ET_FOLLOW_VEHICLE_CAM_*
    ON_BIKE = 2,
    IN_BOAT = 3,
    IN_AIRCRAFT = 4,
    IN_SUBMARINE = 5,
    IN_HELI = 6,
    IN_TURRET = 7,
}
```



pub fn _get_cam_active_view_mode_context_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19CAFA3C87F7C2FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19CAFA3C87F7C2FFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_cam_active_view_mode_context_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19CAFA3C87F7C2FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19CAFA3C87F7C2FFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_cam_spline_node_ease_safe(
        
        
            cam: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83B8201ED82A9A2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83B8201ED82A9A2Du64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_spline_node_ease_raw(
        cam: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83B8201ED82A9A2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83B8201ED82A9A2Du64;

        invoke_raw_typed!(
            hash,
                cam, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ```
SET_FOLLOW_*
```



pub fn _0x9dfe13ecdc1ec196_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DFE13ECDC1EC196u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DFE13ECDC1EC196u64;
        
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
pub fn _0x9dfe13ecdc1ec196_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DFE13ECDC1EC196u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DFE13ECDC1EC196u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Fades the screen out.  
duration: The time the fade should take, in milliseconds.  
```



pub fn do_screen_fade_out_safe(
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x891B5B39AC6302AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x891B5B39AC6302AFu64;
        
        let result = invoke_raw!(
            hash,
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn do_screen_fade_out_raw(
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x891B5B39AC6302AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x891B5B39AC6302AFu64;

        invoke_raw_typed!(
            hash,
                duration
        )
    }
}

/// ## Parameters
*



pub fn force_cinematic_rendering_this_update_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA41BCD7213805AACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA41BCD7213805AACu64;
        
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
pub fn force_cinematic_rendering_this_update_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA41BCD7213805AACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA41BCD7213805AACu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Sets gameplay camera to hash
```

```
NativeDB Introduced: v1180
```



pub fn _set_gameplay_cam_hash_safe(
        
        
            camName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x425A920FDB9A0DDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x425A920FDB9A0DDAu64;
        
        let result = invoke_raw!(
            hash,
                camName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_gameplay_cam_hash_raw(
        camName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x425A920FDB9A0DDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x425A920FDB9A0DDAu64;

        invoke_raw_typed!(
            hash,
                camName
        )
    }
}

/// ## Return value



pub fn is_follow_vehicle_cam_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCBBDE6D335D6D496u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCBBDE6D335D6D496u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_follow_vehicle_cam_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCBBDE6D335D6D496u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCBBDE6D335D6D496u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x469f2ecdec046337_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x469F2ECDEC046337u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x469F2ECDEC046337u64;
        
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
pub fn _0x469f2ecdec046337_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x469F2ECDEC046337u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x469F2ECDEC046337u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Attaches a camera to a specific bone of a Ped, including full matrix transformations for both rotation and position offsets.
This native works with peds only.

```
NativeDB Introduced: v1180
```



pub fn hard_attach_cam_to_ped_bone_safe(
        
        
            cam: 
        , 
        
        
            ped: 
        , 
        
        
            boneIndex: 
        , 
        
        
            xRot: 
        , 
        
        
            yRot: 
        , 
        
        
            zRot: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
        , 
        
        
            isRelative: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x149916F50C34A40Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x149916F50C34A40Du64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                ped, 
                boneIndex, 
                xRot, 
                yRot, 
                zRot, 
                xOffset, 
                yOffset, 
                zOffset, 
                isRelative
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hard_attach_cam_to_ped_bone_raw(
        cam: , 
        ped: , 
        boneIndex: , 
        xRot: , 
        yRot: , 
        zRot: , 
        xOffset: , 
        yOffset: , 
        zOffset: , 
        isRelative: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x149916F50C34A40Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x149916F50C34A40Du64;

        invoke_raw_typed!(
            hash,
                cam, 
                ped, 
                boneIndex, 
                xRot, 
                yRot, 
                zRot, 
                xOffset, 
                yOffset, 
                zOffset, 
                isRelative
        )
    }
}

/// ```
Allows you to aim and shoot at the direction the camera is facing.  
```



pub fn set_cam_affects_aiming_safe(
        
        
            cam: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C1DC7770C51DC8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C1DC7770C51DC8Du64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_affects_aiming_raw(
        cam: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C1DC7770C51DC8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C1DC7770C51DC8Du64;

        invoke_raw_typed!(
            hash,
                cam, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0xa2767257a320fc82_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2767257A320FC82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2767257A320FC82u64;
        
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
pub fn _0xa2767257a320fc82_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2767257A320FC82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2767257A320FC82u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _0xb1381b97f70c7b30_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1381B97F70C7B30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1381B97F70C7B30u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb1381b97f70c7b30_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1381B97F70C7B30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1381B97F70C7B30u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Parameters p0-p5 seems correct. The bool p6 is unknown, but through every X360 script it's always 1. Please correct p0-p5 if any prove to be wrong.  
```



pub fn point_cam_at_ped_bone_safe(
        
        
            cam: 
        , 
        
        
            ped: 
        , 
        
        
            boneIndex: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68B2B5F33BA63C41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68B2B5F33BA63C41u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                ped, 
                boneIndex, 
                x, 
                y, 
                z, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn point_cam_at_ped_bone_raw(
        cam: , 
        ped: , 
        boneIndex: , 
        x: , 
        y: , 
        z: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68B2B5F33BA63C41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68B2B5F33BA63C41u64;

        invoke_raw_typed!(
            hash,
                cam, 
                ped, 
                boneIndex, 
                x, 
                y, 
                z, 
                p6
        )
    }
}

/// ## Return value



pub fn get_gameplay_cam_coord_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14D6F5678D8F1B37u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14D6F5678D8F1B37u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_gameplay_cam_coord_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14D6F5678D8F1B37u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14D6F5678D8F1B37u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NOTE: Debugging functions are not present in the retail version of the game.  
```



pub fn set_cam_debug_name_safe(
        
        
            camera: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B93E0107865DD40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B93E0107865DD40u64;
        
        let result = invoke_raw!(
            hash,
                camera, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_debug_name_raw(
        camera: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B93E0107865DD40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B93E0107865DD40u64;

        invoke_raw_typed!(
            hash,
                camera, 
                name
        )
    }
}

/// Overrides the ped follow camera (not first person camera) with the specified camera. The game loads all camera metadata from `update/update.rpf/x64/data/metadata/cameras.ymt` and `x64a.rpf/data/metadata/cameras.ymt` with the ped follow cameras being of type `camFollowPedCameraMetadata`.

| Follow Camera Names                          |
|----------------------------------------------|
| DEFAULT_FOLLOW_PED_CAMERA                    |
| FOLLOW_PED_ATTACHED_TO_ROPE_CAMERA           |
| FOLLOW_PED_ON_EXILE1_LADDER_CAMERA           |
| FOLLOW_PED_SKY_DIVING_CAMERA                 |
| FOLLOW_PED_SKY_DIVING_FAMILY5_CAMERA         |
| NIGHTCLUB_FOLLOW_PED_CAMERA                  |
| FOLLOW_PED_INTIMIDATION_CAMERA               |
| FOLLOW_PED_IN_WATER_CAMERA                   |
| FOLLOW_PED_PRONE_CAMERA                      |
| FOLLOW_PED_ON_SEAT_CAMERA                    |
| FOLLOW_PED_HANGING_UPSIDE_DOWN_CAMERA        |
| FOLLOW_PED_ATTACHED_TO_ROPE_CAMERA           |
| CUSTOM_TRANSITION_AFTER_WARP_SKY_DIVE_CAMERA |
| FOLLOW_PED_ON_HORSE_CAMERA                   |
| FOLLOW_PED_ON_LOUNGER_CAMERA                 |

Other camera hashes (names not found yet)
```c
// 0x5DBBFB6E
// 0xA38DB056
// 0x16B702A3
// 0x41D72A2E
```



pub fn set_follow_ped_cam_this_update_safe(
        
        
            camName: 
        , 
        
        
            easeTime: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44A113DD6FFC48D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44A113DD6FFC48D1u64;
        
        let result = invoke_raw!(
            hash,
                camName, 
                easeTime
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_follow_ped_cam_this_update_raw(
        camName: , 
        easeTime: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44A113DD6FFC48D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44A113DD6FFC48D1u64;

        invoke_raw_typed!(
            hash,
                camName, 
                easeTime
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0xaabd62873ffb1a33_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAABD62873FFB1A33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAABD62873FFB1A33u64;
        
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
pub fn _0xaabd62873ffb1a33_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAABD62873FFB1A33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAABD62873FFB1A33u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Override the camera work of the third-person camera to table game for current frame only.

|                  HashKey                    |       Hash        |    Game         |
| :---------------------------------: | :-----------:| :-------------:  |
| `CASINO_LUCKY_WHEEL_CAMERA` |   `5891389`   |  Lucky Wheel    |
| `CASINO_SLOT_MACHINE_CAMERA` |  `518572876`  |     Slots       |
| `CASINO_ROULETTE_CAMERA` |   `71681063`  |    Roulette     |
| `CASINO_BLACKJACK_CAMERA` | `-2124244681` |    Blackjack    |
| `CASINO_POKER_CAMERA` | `-1938411241` |   Three Cards   |
| `CASINO_INSIDE_TRACK_CAMERA` | `1929822423` |   Inside Track    |
| `ARCADE_LOVE_PROFESSOR_P1_CAMERA` | `545868034` |   LoveProfessorP1   |
| `ARCADE_LOVE_PROFESSOR_P2_CAMERA` | `935304251` |   LoveProfessorP2   |



pub fn set_table_games_camera_this_update_safe(
        
        
            hash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79C0E43EB9B944E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79C0E43EB9B944E2u64;
        
        let result = invoke_raw!(
            hash,
                hash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_table_games_camera_this_update_raw(
        hash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79C0E43EB9B944E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79C0E43EB9B944E2u64;

        invoke_raw_typed!(
            hash,
                hash
        )
    }
}

/// Stops the currently active global camera shake that was initiated by a script. You can check if a global camera shake is active using [IS_SCRIPT_GLOBAL_SHAKING](#_0xC912AF078AF19212).

```
NativeDB Introduced: v323
```



pub fn stop_script_global_shaking_safe(
        
        
            bStopImmediately: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C9D7949FA533490u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C9D7949FA533490u64;
        
        let result = invoke_raw!(
            hash,
                bStopImmediately
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_script_global_shaking_raw(
        bStopImmediately: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C9D7949FA533490u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C9D7949FA533490u64;

        invoke_raw_typed!(
            hash,
                bStopImmediately
        )
    }
}

/// ## Parameters
*



pub fn set_gameplay_ped_hint_safe(
        
        
            p0: 
        , 
        
        
            x1: 
        , 
        
        
            y1: 
        , 
        
        
            z1: 
        , 
        
        
            p4: 
        , 
        
        
            duration: 
        , 
        
        
            blendOutDuration: 
        , 
        
        
            blendInDuration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B486269ACD548D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B486269ACD548D3u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                x1, 
                y1, 
                z1, 
                p4, 
                duration, 
                blendOutDuration, 
                blendInDuration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gameplay_ped_hint_raw(
        p0: , 
        x1: , 
        y1: , 
        z1: , 
        p4: , 
        duration: , 
        blendOutDuration: , 
        blendInDuration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B486269ACD548D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B486269ACD548D3u64;

        invoke_raw_typed!(
            hash,
                p0, 
                x1, 
                y1, 
                z1, 
                p4, 
                duration, 
                blendOutDuration, 
                blendInDuration
        )
    }
}

/// ## Parameters
*



pub fn set_cam_view_mode_for_context_safe(
        
        
            context: 
        , 
        
        
            viewMode: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A2173E46DAECD12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A2173E46DAECD12u64;
        
        let result = invoke_raw!(
            hash,
                context, 
                viewMode
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_view_mode_for_context_raw(
        context: , 
        viewMode: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A2173E46DAECD12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A2173E46DAECD12u64;

        invoke_raw_typed!(
            hash,
                context, 
                viewMode
        )
    }
}

/// ## Parameters
*



pub fn _0xf55e4046f6f831dc_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF55E4046F6F831DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF55E4046F6F831DCu64;
        
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
pub fn _0xf55e4046f6f831dc_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF55E4046F6F831DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF55E4046F6F831DCu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// SET_CINEMATIC_NEWS_CHANNEL_ACTIVE_THIS_UPDATE native function



pub fn set_cinematic_news_channel_active_this_update_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC9DA9E8789F5246u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC9DA9E8789F5246u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cinematic_news_channel_active_this_update_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC9DA9E8789F5246u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC9DA9E8789F5246u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn is_cam_rendering_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02EC0AF5C5A49B7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02EC0AF5C5A49B7Au64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_cam_rendering_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02EC0AF5C5A49B7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02EC0AF5C5A49B7Au64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ```
This native has its name defined inside its codE  
```



pub fn _set_cam_dof_fnumber_of_lens_safe(
        
        
            camera: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DD234D6F3914C5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DD234D6F3914C5Bu64;
        
        let result = invoke_raw!(
            hash,
                camera, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_cam_dof_fnumber_of_lens_raw(
        camera: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DD234D6F3914C5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DD234D6F3914C5Bu64;

        invoke_raw_typed!(
            hash,
                camera, 
                p1
        )
    }
}

/// ```
BOOL param indicates whether the cam should be destroyed if it belongs to the calling script.  
```



pub fn destroy_cam_safe(
        
        
            cam: 
        , 
        
        
            bScriptHostCam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x865908C81A2C22E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x865908C81A2C22E9u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                bScriptHostCam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn destroy_cam_raw(
        cam: , 
        bScriptHostCam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x865908C81A2C22E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x865908C81A2C22E9u64;

        invoke_raw_typed!(
            hash,
                cam, 
                bScriptHostCam
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _get_debug_camera_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77C3CEC46BE286F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77C3CEC46BE286F6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_debug_camera_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77C3CEC46BE286F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77C3CEC46BE286F6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_gameplay_cam_follow_ped_this_update_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BBACBF51DA047A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BBACBF51DA047A8u64;
        
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
pub fn set_gameplay_cam_follow_ped_this_update_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BBACBF51DA047A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BBACBF51DA047A8u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _set_gameplay_hint_anim_closeup_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3433EADAAF7EE40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3433EADAAF7EE40u64;
        
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
pub fn _set_gameplay_hint_anim_closeup_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3433EADAAF7EE40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3433EADAAF7EE40u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_widescreen_borders_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCD4EA924F42D01Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCD4EA924F42D01Au64;
        
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
pub fn set_widescreen_borders_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCD4EA924F42D01Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCD4EA924F42D01Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _set_gameplay_cam_relative_rotation_safe(
        
        
            roll: 
        , 
        
        
            pitch: 
        , 
        
        
            yaw: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48608C3464F58AB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48608C3464F58AB4u64;
        
        let result = invoke_raw!(
            hash,
                roll, 
                pitch, 
                yaw
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_gameplay_cam_relative_rotation_raw(
        roll: , 
        pitch: , 
        yaw: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48608C3464F58AB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48608C3464F58AB4u64;

        invoke_raw_typed!(
            hash,
                roll, 
                pitch, 
                yaw
        )
    }
}

/// F*

```
NativeDB Introduced: v1734
```



pub fn _0x28b022a17b068a3a_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28B022A17B068A3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28B022A17B068A3Au64;
        
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
pub fn _0x28b022a17b068a3a_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28B022A17B068A3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28B022A17B068A3Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Sets the camera position relative to heading in float from -360 to +360.  
Heading is alwyas 0 in aiming camera.  
```



pub fn set_gameplay_cam_relative_heading_safe(
        
        
            heading: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4EC2312F4E5B1F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4EC2312F4E5B1F1u64;
        
        let result = invoke_raw!(
            hash,
                heading
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gameplay_cam_relative_heading_raw(
        heading: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4EC2312F4E5B1F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4EC2312F4E5B1F1u64;

        invoke_raw_typed!(
            hash,
                heading
        )
    }
}

/// ```
Does nothing  
```

```
NativeDB Added Parameter 2: Any p1
```



pub fn _set_gameplay_cam_raw_yaw_safe(
        
        
            yaw: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x103991D4A307D472u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x103991D4A307D472u64;
        
        let result = invoke_raw!(
            hash,
                yaw
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_gameplay_cam_raw_yaw_raw(
        yaw: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x103991D4A307D472u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x103991D4A307D472u64;

        invoke_raw_typed!(
            hash,
                yaw
        )
    }
}

/// See [`GET_FOLLOW_PED_CAM_VIEW_MODE`](#_0x8D4D46230B2C353A) for the follow mode enum.



pub fn set_follow_vehicle_cam_view_mode_safe(
        
        
            viewMode: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC253D7842768F48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC253D7842768F48u64;
        
        let result = invoke_raw!(
            hash,
                viewMode
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_follow_vehicle_cam_view_mode_raw(
        viewMode: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC253D7842768F48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC253D7842768F48u64;

        invoke_raw_typed!(
            hash,
                viewMode
        )
    }
}

/// [Animations list](https://alexguirre.github.io/animations-list/)



pub fn is_cam_playing_anim_safe(
        
        
            cam: 
        , 
        
        
            animName: 
        , 
        
        
            animDictionary: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC90621D8A0CEECF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC90621D8A0CEECF2u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                animName, 
                animDictionary
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_cam_playing_anim_raw(
        cam: , 
        animName: , 
        animDictionary: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC90621D8A0CEECF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC90621D8A0CEECF2u64;

        invoke_raw_typed!(
            hash,
                cam, 
                animName, 
                animDictionary
        )
    }
}

/// B*

```
NativeDB Introduced: v1734
```



pub fn _0x9f97da93681f87ea_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F97DA93681F87EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F97DA93681F87EAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x9f97da93681f87ea_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F97DA93681F87EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F97DA93681F87EAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Creates a camera with the specified camera hash, You can use `SET_CAM_` natives to manipulate the camera.
Make sure to call [RENDER_SCRIPT_CAMS](#_0x07E5B515DB0636FC) once the camera is created, or this won't have any visible effect.

Take a look at [CREATE_CAM](#_0xC3981DCE61D9E13F) if you would like to see the available camera names.

```
NativeDB Introduced: v323
```



pub fn create_camera_safe(
        
        
            camHash: 
        , 
        
        
            active: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E3CF89C6BCCA67Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E3CF89C6BCCA67Du64;
        
        let result = invoke_raw!(
            hash,
                camHash, 
                active
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_camera_raw(
        camHash: , 
        active: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E3CF89C6BCCA67Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E3CF89C6BCCA67Du64;

        invoke_raw_typed!(
            hash,
                camHash, 
                active
        )
    }
}

/// ```
This native has a name defined inside its code  
```



pub fn _set_cam_dof_max_near_in_focus_distance_blend_level_safe(
        
        
            camera: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C654B4943BDDF7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C654B4943BDDF7Cu64;
        
        let result = invoke_raw!(
            hash,
                camera, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_cam_dof_max_near_in_focus_distance_blend_level_raw(
        camera: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C654B4943BDDF7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C654B4943BDDF7Cu64;

        invoke_raw_typed!(
            hash,
                camera, 
                p1
        )
    }
}

/// ```
if p0 is 0, effect is cancelled  
if p0 is 1, effect zooms in, gradually tilts cam clockwise apx 30 degrees, wobbles slowly. Motion blur is active until cancelled.  
if p0 is 2, effect immediately tilts cam clockwise apx 30 degrees, begins to wobble slowly, then gradually tilts cam back to normal. The wobbling will continue until the effect is cancelled.  
```



pub fn _set_cam_effect_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80C8B1846639BB19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80C8B1846639BB19u64;
        
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
pub fn _set_cam_effect_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80C8B1846639BB19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80C8B1846639BB19u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn stop_cam_shaking_safe(
        
        
            cam: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBDECF64367884AC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBDECF64367884AC3u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_cam_shaking_raw(
        cam: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBDECF64367884AC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBDECF64367884AC3u64;

        invoke_raw_typed!(
            hash,
                cam, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x5c41e6babc9e2112_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C41E6BABC9E2112u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C41E6BABC9E2112u64;
        
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
pub fn _0x5c41e6babc9e2112_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C41E6BABC9E2112u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C41E6BABC9E2112u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1365
```



pub fn _set_follow_turret_seat_cam_safe(
        
        
            seatIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C90CAB09951A12Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C90CAB09951A12Fu64;
        
        let result = invoke_raw!(
            hash,
                seatIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_follow_turret_seat_cam_raw(
        seatIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C90CAB09951A12Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C90CAB09951A12Fu64;

        invoke_raw_typed!(
            hash,
                seatIndex
        )
    }
}

/// ```
Previous declaration void SET_CAM_ACTIVE_WITH_INTERP(Cam camTo, Cam camFrom, int duration, BOOL easeLocation, BOOL easeRotation) is completely wrong. The last two params are integers not BOOLs...  
```



pub fn set_cam_active_with_interp_safe(
        
        
            camTo: 
        , 
        
        
            camFrom: 
        , 
        
        
            duration: 
        , 
        
        
            easeLocation: 
        , 
        
        
            easeRotation: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FBDA379383A52A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FBDA379383A52A4u64;
        
        let result = invoke_raw!(
            hash,
                camTo, 
                camFrom, 
                duration, 
                easeLocation, 
                easeRotation
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_active_with_interp_raw(
        camTo: , 
        camFrom: , 
        duration: , 
        easeLocation: , 
        easeRotation: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FBDA379383A52A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FBDA379383A52A4u64;

        invoke_raw_typed!(
            hash,
                camTo, 
                camFrom, 
                duration, 
                easeLocation, 
                easeRotation
        )
    }
}

/// ## Parameters
*



pub fn get_cam_fov_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3330A45CCCDB26Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3330A45CCCDB26Au64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cam_fov_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3330A45CCCDB26Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3330A45CCCDB26Au64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ## Parameters
*



pub fn set_gameplay_coord_hint_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            duration: 
        , 
        
        
            blendOutDuration: 
        , 
        
        
            blendInDuration: 
        , 
        
        
            unk: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD51ADCD2D8BC0FB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD51ADCD2D8BC0FB3u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                duration, 
                blendOutDuration, 
                blendInDuration, 
                unk
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gameplay_coord_hint_raw(
        x: , 
        y: , 
        z: , 
        duration: , 
        blendOutDuration: , 
        blendInDuration: , 
        unk: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD51ADCD2D8BC0FB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD51ADCD2D8BC0FB3u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                duration, 
                blendOutDuration, 
                blendInDuration, 
                unk
        )
    }
}

/// ```
Set camera as active/inactive.  
```



pub fn set_cam_active_safe(
        
        
            cam: 
        , 
        
        
            active: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x026FB97D0A425F84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x026FB97D0A425F84u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                active
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_active_raw(
        cam: , 
        active: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x026FB97D0A425F84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x026FB97D0A425F84u64;

        invoke_raw_typed!(
            hash,
                cam, 
                active
        )
    }
}

/// ```
minimum: Degrees between -90f and 90f.
maximum: Degrees between -90f and 90f.
Clamps the gameplay camera's current pitch.
Eg. _CLAMP_GAMEPLAY_CAM_PITCH(0.0f, 0.0f) will set the vertical angle directly behind the player.
```



pub fn _clamp_gameplay_cam_pitch_safe(
        
        
            minimum: 
        , 
        
        
            maximum: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA516C198B7DCA1E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA516C198B7DCA1E1u64;
        
        let result = invoke_raw!(
            hash,
                minimum, 
                maximum
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _clamp_gameplay_cam_pitch_raw(
        minimum: , 
        maximum: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA516C198B7DCA1E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA516C198B7DCA1E1u64;

        invoke_raw_typed!(
            hash,
                minimum, 
                maximum
        )
    }
}

/// ## Parameters
*



pub fn get_cam_near_clip_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC520A34DAFBF24B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC520A34DAFBF24B1u64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cam_near_clip_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC520A34DAFBF24B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC520A34DAFBF24B1u64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ```
The native seems to only be called once.  
The native is used as so,  
CAM::SET_CAM_INHERIT_ROLL_VEHICLE(l_544, getElem(2, &l_525, 4));  
In the exile1 script.  
```



pub fn set_cam_inherit_roll_vehicle_safe(
        
        
            cam: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45F1DE9C34B93AE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45F1DE9C34B93AE6u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_inherit_roll_vehicle_raw(
        cam: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45F1DE9C34B93AE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45F1DE9C34B93AE6u64;

        invoke_raw_typed!(
            hash,
                cam, 
                p1
        )
    }
}

/// ```
A*
```



pub fn _0x4879e4fe39074cdf_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4879E4FE39074CDFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4879E4FE39074CDFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4879e4fe39074cdf_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4879E4FE39074CDFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4879E4FE39074CDFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Points the camera at the specified entity.

Offset works like [GET_OFFSET_FROM_ENTITY_IN_WORLD_COORDS](#_0x1899F328B0E12848).



pub fn point_cam_at_entity_safe(
        
        
            cam: 
        , 
        
        
            entity: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5640BFF86B16E8DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5640BFF86B16E8DCu64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                entity, 
                offsetX, 
                offsetY, 
                offsetZ, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn point_cam_at_entity_raw(
        cam: , 
        entity: , 
        offsetX: , 
        offsetY: , 
        offsetZ: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5640BFF86B16E8DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5640BFF86B16E8DCu64;

        invoke_raw_typed!(
            hash,
                cam, 
                entity, 
                offsetX, 
                offsetY, 
                offsetZ, 
                p5
        )
    }
}

/// ## Return value
Returns if the "Allow Independent Camera Modes" setting in Pause Menu is turned On or Off.



pub fn is_allowed_independent_camera_modes_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAF0FA793D05C592u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAF0FA793D05C592u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_allowed_independent_camera_modes_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAF0FA793D05C592u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAF0FA793D05C592u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn _0x324c5aa411da7737_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x324C5AA411DA7737u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x324C5AA411DA7737u64;
        
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
pub fn _0x324c5aa411da7737_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x324C5AA411DA7737u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x324C5AA411DA7737u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// _0xC8391C309684595A native function



pub fn _0xc8391c309684595a_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8391C309684595Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8391C309684595Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc8391c309684595a_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8391C309684595Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8391C309684595Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _set_gameplay_hint_anim_offsety_safe(
        
        
            yOffset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC92717EF615B6704u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC92717EF615B6704u64;
        
        let result = invoke_raw!(
            hash,
                yOffset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_gameplay_hint_anim_offsety_raw(
        yOffset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC92717EF615B6704u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC92717EF615B6704u64;

        invoke_raw_typed!(
            hash,
                yOffset
        )
    }
}

/// ```
NativeDB Introduced: v2699
```



pub fn _get_cam_dof_strength_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06D153C0B99B6128u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06D153C0B99B6128u64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_cam_dof_strength_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06D153C0B99B6128u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06D153C0B99B6128u64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// Creates a camera with the specified cam name, You can use `SET_CAM_` natives to manipulate the camera.  
Make sure to call [RENDER_SCRIPT_CAMS](#_0x07E5B515DB0636FC) once the camera is created, or this won't have any visible effect.



pub fn create_cam_safe(
        
        
            camName: 
        , 
        
        
            active: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3981DCE61D9E13Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3981DCE61D9E13Fu64;
        
        let result = invoke_raw!(
            hash,
                camName, 
                active
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_cam_raw(
        camName: , 
        active: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3981DCE61D9E13Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3981DCE61D9E13Fu64;

        invoke_raw_typed!(
            hash,
                camName, 
                active
        )
    }
}

/// ## Return value



pub fn is_screen_fading_out_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x797AC7CB535BA28Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x797AC7CB535BA28Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_screen_fading_out_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x797AC7CB535BA28Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x797AC7CB535BA28Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_final_rendered_cam_coord_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA200EB1EE790F448u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA200EB1EE790F448u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_final_rendered_cam_coord_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA200EB1EE790F448u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA200EB1EE790F448u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// See [`GET_FOLLOW_PED_CAM_VIEW_MODE`](#_0x8D4D46230B2C353A) for the follow mode enum.



pub fn set_follow_vehicle_cam_zoom_level_safe(
        
        
            zoomLevel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19464CB6E4078C8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19464CB6E4078C8Au64;
        
        let result = invoke_raw!(
            hash,
                zoomLevel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_follow_vehicle_cam_zoom_level_raw(
        zoomLevel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19464CB6E4078C8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19464CB6E4078C8Au64;

        invoke_raw_typed!(
            hash,
                zoomLevel
        )
    }
}

/// ## Parameters
*



pub fn set_first_person_aim_cam_near_clip_this_update_safe(
        
        
            distance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AF7B437918103B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AF7B437918103B3u64;
        
        let result = invoke_raw!(
            hash,
                distance
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_first_person_aim_cam_near_clip_this_update_raw(
        distance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AF7B437918103B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AF7B437918103B3u64;

        invoke_raw_typed!(
            hash,
                distance
        )
    }
}

/// ## Return value



pub fn _replay_free_cam_get_max_range_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BFCEB5EA1B161B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BFCEB5EA1B161B6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _replay_free_cam_get_max_range_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BFCEB5EA1B161B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BFCEB5EA1B161B6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_cam_dof_planes_safe(
        
        
            cam: 
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
        let hash = 0x3CF48F6F96E749DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3CF48F6F96E749DCu64;
        
        let result = invoke_raw!(
            hash,
                cam, 
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
pub fn set_cam_dof_planes_raw(
        cam: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3CF48F6F96E749DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3CF48F6F96E749DCu64;

        invoke_raw_typed!(
            hash,
                cam, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn _disable_cam_collision_for_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2AED6301F67007D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2AED6301F67007D5u64;
        
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
pub fn _disable_cam_collision_for_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2AED6301F67007D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2AED6301F67007D5u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Return value



pub fn is_cinematic_cam_shaking_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBC08F6B4CB8FF0Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBC08F6B4CB8FF0Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_cinematic_cam_shaking_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBC08F6B4CB8FF0Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBC08F6B4CB8FF0Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn detach_cam_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2FABBE87F4BAD82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2FABBE87F4BAD82u64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn detach_cam_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2FABBE87F4BAD82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2FABBE87F4BAD82u64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ## Parameters
*



pub fn set_in_vehicle_cam_state_this_update_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE9EA16D6E54CDCA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE9EA16D6E54CDCA4u64;
        
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
pub fn set_in_vehicle_cam_state_this_update_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE9EA16D6E54CDCA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE9EA16D6E54CDCA4u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x247acbc4abbc9d1c_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x247ACBC4ABBC9D1Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x247ACBC4ABBC9D1Cu64;
        
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
pub fn _0x247acbc4abbc9d1c_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x247ACBC4ABBC9D1Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x247ACBC4ABBC9D1Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn get_first_person_aim_cam_zoom_factor_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EC52CC40597D170u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EC52CC40597D170u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_first_person_aim_cam_zoom_factor_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EC52CC40597D170u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EC52CC40597D170u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn add_cam_spline_node_using_camera_frame_safe(
        
        
            cam: 
        , 
        
        
            cam2: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A9F2A468B328E74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A9F2A468B328E74u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                cam2, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_cam_spline_node_using_camera_frame_raw(
        cam: , 
        cam2: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A9F2A468B328E74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A9F2A468B328E74u64;

        invoke_raw_typed!(
            hash,
                cam, 
                cam2, 
                p2, 
                p3
        )
    }
}

/// Terminates the current gameplay hint camera, with an option for immediate cessation or a gradual fade out.

```
NativeDB Introduced: v323
```



pub fn stop_gameplay_hint_safe(
        
        
            bStopImmediately: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF46C581C61718916u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF46C581C61718916u64;
        
        let result = invoke_raw!(
            hash,
                bStopImmediately
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_gameplay_hint_raw(
        bStopImmediately: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF46C581C61718916u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF46C581C61718916u64;

        invoke_raw_typed!(
            hash,
                bStopImmediately
        )
    }
}

/// ## Parameters
*



pub fn stop_gameplay_cam_shaking_safe(
        
        
            bStopImmediately: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EF93E9F3D08C178u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EF93E9F3D08C178u64;
        
        let result = invoke_raw!(
            hash,
                bStopImmediately
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_gameplay_cam_shaking_raw(
        bStopImmediately: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EF93E9F3D08C178u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EF93E9F3D08C178u64;

        invoke_raw_typed!(
            hash,
                bStopImmediately
        )
    }
}

/// ## Return value



pub fn get_final_rendered_cam_near_dof_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA03502FC581F7D9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA03502FC581F7D9Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_final_rendered_cam_near_dof_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA03502FC581F7D9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA03502FC581F7D9Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
CAM::SHAKE_SCRIPT_GLOBAL("HAND_SHAKE", 0.2);
```



pub fn shake_script_global_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4C8CF9E353AFECAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4C8CF9E353AFECAu64;
        
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
pub fn shake_script_global_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4C8CF9E353AFECAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4C8CF9E353AFECAu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Return value



pub fn is_gameplay_cam_looking_behind_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70FDA869F3317EA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70FDA869F3317EA9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_gameplay_cam_looking_behind_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70FDA869F3317EA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70FDA869F3317EA9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_cam_near_clip_safe(
        
        
            cam: 
        , 
        
        
            nearClip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7848EFCCC545182u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7848EFCCC545182u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                nearClip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_near_clip_raw(
        cam: , 
        nearClip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7848EFCCC545182u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7848EFCCC545182u64;

        invoke_raw_typed!(
            hash,
                cam, 
                nearClip
        )
    }
}

/// See [`GET_FOLLOW_PED_CAM_VIEW_MODE`](#_0x8D4D46230B2C353A) for the follow mode enum.



pub fn get_follow_vehicle_cam_zoom_level_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE82280AB767B690u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE82280AB767B690u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_follow_vehicle_cam_zoom_level_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE82280AB767B690u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE82280AB767B690u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _set_fly_cam_vertical_speed_multiplier_safe(
        
        
            cam: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE827B9382CFB41BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE827B9382CFB41BAu64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_fly_cam_vertical_speed_multiplier_raw(
        cam: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE827B9382CFB41BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE827B9382CFB41BAu64;

        invoke_raw_typed!(
            hash,
                cam, 
                p1, 
                p2, 
                p3
        )
    }
}

/// See [`GET_FOLLOW_PED_CAM_VIEW_MODE`](#_0x8D4D46230B2C353A) for the follow mode enum.



pub fn get_follow_ped_cam_zoom_level_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33E6C8EFD0CD93E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33E6C8EFD0CD93E9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_follow_ped_cam_zoom_level_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33E6C8EFD0CD93E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33E6C8EFD0CD93E9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
I named p1 as timeDuration as it is obvious. I'm assuming tho it is ran in ms(Milliseconds) as usual.  
```



pub fn set_cam_spline_duration_safe(
        
        
            cam: 
        , 
        
        
            timeDuration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1381539FEE034CDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1381539FEE034CDAu64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                timeDuration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_spline_duration_raw(
        cam: , 
        timeDuration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1381539FEE034CDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1381539FEE034CDAu64;

        invoke_raw_typed!(
            hash,
                cam, 
                timeDuration
        )
    }
}

/// ## Return value



pub fn is_follow_ped_cam_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6D3D26810C8E0F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6D3D26810C8E0F9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_follow_ped_cam_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6D3D26810C8E0F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6D3D26810C8E0F9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Adjusts the field of view (FOV) for a specified camera, allowing for a wider or narrower perspective of the game world. The field of view is measured in degrees and affects how much of the game world is visible at any given moment through the camera.

```
NativeDB Introduced: v323
```



pub fn set_cam_fov_safe(
        
        
            cam: 
        , 
        
        
            fieldOfView: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB13C14F66A00D047u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB13C14F66A00D047u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                fieldOfView
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_fov_raw(
        cam: , 
        fieldOfView: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB13C14F66A00D047u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB13C14F66A00D047u64;

        invoke_raw_typed!(
            hash,
                cam, 
                fieldOfView
        )
    }
}

/// ## Parameters
*



pub fn _0x2f7f2b26dd3f18ee_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F7F2B26DD3F18EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F7F2B26DD3F18EEu64;
        
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
pub fn _0x2f7f2b26dd3f18ee_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F7F2B26DD3F18EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F7F2B26DD3F18EEu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn is_cinematic_shot_active_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC9F3371A7C28BC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC9F3371A7C28BC9u64;
        
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
pub fn is_cinematic_shot_active_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC9F3371A7C28BC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC9F3371A7C28BC9u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// See [`GET_FOLLOW_PED_CAM_VIEW_MODE`](#_0x8D4D46230B2C353A) for the follow mode enum.



pub fn get_follow_vehicle_cam_view_mode_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4FF579AC0E3AAAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4FF579AC0E3AAAEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_follow_vehicle_cam_view_mode_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4FF579AC0E3AAAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4FF579AC0E3AAAEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Native name labeled within its code
```



pub fn _set_cam_dof_focal_length_multiplier_safe(
        
        
            camera: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47B595D60664CFFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47B595D60664CFFAu64;
        
        let result = invoke_raw!(
            hash,
                camera, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_cam_dof_focal_length_multiplier_raw(
        camera: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47B595D60664CFFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47B595D60664CFFAu64;

        invoke_raw_typed!(
            hash,
                camera, 
                multiplier
        )
    }
}

/// _0x5A43C76F7FC7BA5F native function



pub fn _0x5a43c76f7fc7ba5f_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A43C76F7FC7BA5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A43C76F7FC7BA5Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5a43c76f7fc7ba5f_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A43C76F7FC7BA5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A43C76F7FC7BA5Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
_RESET_*

_RESET_GAMEPLAY_CAM_RELATIVE_ORBIT_HOLD_TIME?
```

```
NativeDB Introduced: v2699
```



pub fn _0x7295c203dd659dfe_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7295C203DD659DFEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7295C203DD659DFEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7295c203dd659dfe_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7295C203DD659DFEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7295C203DD659DFEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
From b617 scripts:
CAM::_21E253A7F8DA5DFB("DINGHY");
CAM::_21E253A7F8DA5DFB("ISSI2");
CAM::_21E253A7F8DA5DFB("SPEEDO");
```



pub fn _set_gameplay_cam_vehicle_camera_safe(
        
        
            vehicleName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21E253A7F8DA5DFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21E253A7F8DA5DFBu64;
        
        let result = invoke_raw!(
            hash,
                vehicleName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_gameplay_cam_vehicle_camera_raw(
        vehicleName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21E253A7F8DA5DFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21E253A7F8DA5DFBu64;

        invoke_raw_typed!(
            hash,
                vehicleName
        )
    }
}

/// ## Return value



pub fn get_gameplay_cam_fov_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65019750A0324133u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65019750A0324133u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_gameplay_cam_fov_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65019750A0324133u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65019750A0324133u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_final_rendered_in_when_friendly_fov_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F35F6732C3FBBA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F35F6732C3FBBA0u64;
        
        let result = invoke_raw!(
            hash,
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_final_rendered_in_when_friendly_fov_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F35F6732C3FBBA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F35F6732C3FBBA0u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn set_fly_cam_max_height_safe(
        
        
            cam: 
        , 
        
        
            height: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9D02130ECDD1D77u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9D02130ECDD1D77u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                height
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_fly_cam_max_height_raw(
        cam: , 
        height: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9D02130ECDD1D77u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9D02130ECDD1D77u64;

        invoke_raw_typed!(
            hash,
                cam, 
                height
        )
    }
}

/// ## Parameters
*



pub fn _set_gameplay_cam_raw_pitch_safe(
        
        
            pitch: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x759E13EBC1C15C5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x759E13EBC1C15C5Au64;
        
        let result = invoke_raw!(
            hash,
                pitch
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_gameplay_cam_raw_pitch_raw(
        pitch: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x759E13EBC1C15C5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x759E13EBC1C15C5Au64;

        invoke_raw_typed!(
            hash,
                pitch
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn is_cinematic_cam_input_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5F1E89A970B7796u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5F1E89A970B7796u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_cinematic_cam_input_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5F1E89A970B7796u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5F1E89A970B7796u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Resets the idle camera timer. Calling that in a loop once every few seconds is enough to disable the idle cinematic camera.



pub fn invalidate_idle_cam_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4F2C0D4EE209E20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4F2C0D4EE209E20u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn invalidate_idle_cam_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4F2C0D4EE209E20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4F2C0D4EE209E20u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _set_first_person_cam_pitch_range_safe(
        
        
            minAngle: 
        , 
        
        
            maxAngle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCFC632DB7673BF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCFC632DB7673BF0u64;
        
        let result = invoke_raw!(
            hash,
                minAngle, 
                maxAngle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_first_person_cam_pitch_range_raw(
        minAngle: , 
        maxAngle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCFC632DB7673BF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCFC632DB7673BF0u64;

        invoke_raw_typed!(
            hash,
                minAngle, 
                maxAngle
        )
    }
}

/// ## Parameters
*



pub fn get_cam_far_dof_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x255F8DAFD540D397u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x255F8DAFD540D397u64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cam_far_dof_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x255F8DAFD540D397u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x255F8DAFD540D397u64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// _0x62374889A4D59F72 native function



pub fn _0x62374889a4d59f72_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62374889A4D59F72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62374889A4D59F72u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x62374889a4d59f72_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62374889A4D59F72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62374889A4D59F72u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn stop_cam_pointing_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF33AB75780BA57DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF33AB75780BA57DEu64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_cam_pointing_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF33AB75780BA57DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF33AB75780BA57DEu64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ## Return value
Returns the relative pitch of the gameplay camera



pub fn get_gameplay_cam_relative_pitch_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A6867B4845BEDA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A6867B4845BEDA2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_gameplay_cam_relative_pitch_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A6867B4845BEDA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A6867B4845BEDA2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p0 argument found in the b617d scripts: "DRUNK_SHAKE"  
```



pub fn shake_cinematic_cam_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCE214D9ED58F3CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCE214D9ED58F3CFu64;
        
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
pub fn shake_cinematic_cam_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCE214D9ED58F3CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCE214D9ED58F3CFu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
This native has a name defined inside its code  
```



pub fn _set_cam_dof_max_near_in_focus_distance_safe(
        
        
            camera: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3654A441402562Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3654A441402562Du64;
        
        let result = invoke_raw!(
            hash,
                camera, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_cam_dof_max_near_in_focus_distance_raw(
        camera: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3654A441402562Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3654A441402562Du64;

        invoke_raw_typed!(
            hash,
                camera, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_cinematic_button_active_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51669F7D1FB53D9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51669F7D1FB53D9Fu64;
        
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
pub fn set_cinematic_button_active_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51669F7D1FB53D9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51669F7D1FB53D9Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
BOOL param indicates whether the cam should be destroyed if it belongs to the calling script.  
```



pub fn destroy_all_cams_safe(
        
        
            bScriptHostCam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E5FB15663F79120u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E5FB15663F79120u64;
        
        let result = invoke_raw!(
            hash,
                bScriptHostCam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn destroy_all_cams_raw(
        bScriptHostCam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E5FB15663F79120u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E5FB15663F79120u64;

        invoke_raw_typed!(
            hash,
                bScriptHostCam
        )
    }
}

/// ## Return value



pub fn _0x705a276ebff3133d_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x705A276EBFF3133Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x705A276EBFF3133Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x705a276ebff3133d_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x705A276EBFF3133Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x705A276EBFF3133Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn add_cam_spline_node_using_gameplay_frame_safe(
        
        
            cam: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x609278246A29CA34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x609278246A29CA34u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_cam_spline_node_using_gameplay_frame_raw(
        cam: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x609278246A29CA34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x609278246A29CA34u64;

        invoke_raw_typed!(
            hash,
                cam, 
                p1, 
                p2
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0xa7092afe81944852_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7092AFE81944852u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7092AFE81944852u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa7092afe81944852_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7092AFE81944852u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7092AFE81944852u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Enables or disables the usage of a shallow DOF. Needs to be set to true to use [`SET_CAM_NEAR_DOF`](#_0x3FA4BF0A7AB7DE2C), [`SET_CAM_FAR_DOF`](#_0xEDD91296CD01AEE0), etc. Doesn't need to be called every tick.



pub fn set_cam_use_shallow_dof_mode_safe(
        
        
            cam: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16A96863A17552BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16A96863A17552BBu64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_use_shallow_dof_mode_raw(
        cam: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16A96863A17552BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16A96863A17552BBu64;

        invoke_raw_typed!(
            hash,
                cam, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_cam_far_clip_safe(
        
        
            cam: 
        , 
        
        
            farClip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE306F2A904BF86Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE306F2A904BF86Eu64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                farClip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_far_clip_raw(
        cam: , 
        farClip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE306F2A904BF86Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE306F2A904BF86Eu64;

        invoke_raw_typed!(
            hash,
                cam, 
                farClip
        )
    }
}

/// ```
W*
```



pub fn _0x5c48a1d6e3b33179_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C48A1D6E3B33179u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C48A1D6E3B33179u64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5c48a1d6e3b33179_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C48A1D6E3B33179u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C48A1D6E3B33179u64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ## Parameters
*



pub fn set_cam_shake_amplitude_safe(
        
        
            cam: 
        , 
        
        
            amplitude: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD93DB43B82BC0D00u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD93DB43B82BC0D00u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                amplitude
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_shake_amplitude_raw(
        cam: , 
        amplitude: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD93DB43B82BC0D00u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD93DB43B82BC0D00u64;

        invoke_raw_typed!(
            hash,
                cam, 
                amplitude
        )
    }
}

/// _0xDD79DF9F4D26E1C9 native function



pub fn _0xdd79df9f4d26e1c9_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD79DF9F4D26E1C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD79DF9F4D26E1C9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xdd79df9f4d26e1c9_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD79DF9F4D26E1C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD79DF9F4D26E1C9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _set_gameplay_hint_anim_offsetx_safe(
        
        
            xOffset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D7B620DAE436138u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D7B620DAE436138u64;
        
        let result = invoke_raw!(
            hash,
                xOffset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_gameplay_hint_anim_offsetx_raw(
        xOffset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D7B620DAE436138u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D7B620DAE436138u64;

        invoke_raw_typed!(
            hash,
                xOffset
        )
    }
}

/// This native works with vehicles only.
Bone indexes are usually given by this native [GET_ENTITY_BONE_INDEX_BY_NAME](#_0xFB71170B7E76ACBA).



pub fn _attach_cam_to_vehicle_bone_safe(
        
        
            cam: 
        , 
        
        
            vehicle: 
        , 
        
        
            boneIndex: 
        , 
        
        
            relativeRotation: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            offX: 
        , 
        
        
            offY: 
        , 
        
        
            offZ: 
        , 
        
        
            fixedDirection: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DB3F12A02CAEF72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DB3F12A02CAEF72u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                vehicle, 
                boneIndex, 
                relativeRotation, 
                rotX, 
                rotY, 
                rotZ, 
                offX, 
                offY, 
                offZ, 
                fixedDirection
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _attach_cam_to_vehicle_bone_raw(
        cam: , 
        vehicle: , 
        boneIndex: , 
        relativeRotation: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        offX: , 
        offY: , 
        offZ: , 
        fixedDirection: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DB3F12A02CAEF72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DB3F12A02CAEF72u64;

        invoke_raw_typed!(
            hash,
                cam, 
                vehicle, 
                boneIndex, 
                relativeRotation, 
                rotX, 
                rotY, 
                rotZ, 
                offX, 
                offY, 
                offZ, 
                fixedDirection
        )
    }
}

/// ## Return value



pub fn _is_in_vehicle_cam_disabled_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F32C0D5A90A9B40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F32C0D5A90A9B40u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_in_vehicle_cam_disabled_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F32C0D5A90A9B40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F32C0D5A90A9B40u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn point_cam_at_coord_safe(
        
        
            cam: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF75497BB865F0803u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF75497BB865F0803u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
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
pub fn point_cam_at_coord_raw(
        cam: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF75497BB865F0803u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF75497BB865F0803u64;

        invoke_raw_typed!(
            hash,
                cam, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn _0xe111a7c0d200cbc5_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE111A7C0D200CBC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE111A7C0D200CBC5u64;
        
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
pub fn _0xe111a7c0d200cbc5_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE111A7C0D200CBC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE111A7C0D200CBC5u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Return value



pub fn get_final_rendered_cam_motion_blur_strength_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x162F9D995753DC19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x162F9D995753DC19u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_final_rendered_cam_motion_blur_strength_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x162F9D995753DC19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x162F9D995753DC19u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
minimum: Degrees between -180f and 180f.
maximum: Degrees between -180f and 180f.
Clamps the gameplay camera's current yaw.
Eg. _CLAMP_GAMEPLAY_CAM_YAW(0.0f, 0.0f) will set the horizontal angle directly behind the player.
```



pub fn _clamp_gameplay_cam_yaw_safe(
        
        
            minimum: 
        , 
        
        
            maximum: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F993D26E0CA5E8Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F993D26E0CA5E8Eu64;
        
        let result = invoke_raw!(
            hash,
                minimum, 
                maximum
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _clamp_gameplay_cam_yaw_raw(
        minimum: , 
        maximum: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F993D26E0CA5E8Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F993D26E0CA5E8Eu64;

        invoke_raw_typed!(
            hash,
                minimum, 
                maximum
        )
    }
}

/// Specifies how much the DoF effect should be applied (Set using [`SET_CAM_NEAR_DOF`](#_0x3FA4BF0A7AB7DE2C), [`SET_CAM_FAR_DOF`](#_0xEDD91296CD01AEE0), etc.)



pub fn set_cam_dof_strength_safe(
        
        
            cam: 
        , 
        
        
            dofStrength: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EE29B4D7D5DF897u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EE29B4D7D5DF897u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                dofStrength
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_dof_strength_raw(
        cam: , 
        dofStrength: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EE29B4D7D5DF897u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EE29B4D7D5DF897u64;

        invoke_raw_typed!(
            hash,
                cam, 
                dofStrength
        )
    }
}

/// ## Parameters
*



pub fn set_gameplay_hint_base_orbit_pitch_offset_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1F8363DFAD03848u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1F8363DFAD03848u64;
        
        let result = invoke_raw!(
            hash,
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gameplay_hint_base_orbit_pitch_offset_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1F8363DFAD03848u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1F8363DFAD03848u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Parameters
*



pub fn get_cam_far_clip_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB60A9CFEB21CA6AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB60A9CFEB21CA6AAu64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cam_far_clip_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB60A9CFEB21CA6AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB60A9CFEB21CA6AAu64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ## Parameters
*



pub fn get_final_rendered_in_when_friendly_rot_safe(
        
        
            player: 
        , 
        
        
            rotationOrder: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26903D9CD1175F2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26903D9CD1175F2Cu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                rotationOrder
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_final_rendered_in_when_friendly_rot_raw(
        player: , 
        rotationOrder: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26903D9CD1175F2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26903D9CD1175F2Cu64;

        invoke_raw_typed!(
            hash,
                player, 
                rotationOrder
        )
    }
}

/// This function takes a rotation order and outputs a `Vector3` in degrees. 

It first calls a game function to calculate these values given the rotation order and effectively multiplies those values by `180/PI`, hence degrees since the function it calls outputs radians which are then converted to degrees.



pub fn get_gameplay_cam_rot_safe(
        
        
            rotationOrder: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x837765A25378F0BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x837765A25378F0BBu64;
        
        let result = invoke_raw!(
            hash,
                rotationOrder
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_gameplay_cam_rot_raw(
        rotationOrder: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x837765A25378F0BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x837765A25378F0BBu64;

        invoke_raw_typed!(
            hash,
                rotationOrder
        )
    }
}

/// ## Parameters
*



pub fn _0x4008edf7d6e48175_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4008EDF7D6E48175u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4008EDF7D6E48175u64;
        
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
pub fn _0x4008edf7d6e48175_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4008EDF7D6E48175u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4008EDF7D6E48175u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_cam_anim_current_phase_safe(
        
        
            cam: 
        , 
        
        
            phase: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4145A4C44FF3B5A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4145A4C44FF3B5A6u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                phase
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_anim_current_phase_raw(
        cam: , 
        phase: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4145A4C44FF3B5A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4145A4C44FF3B5A6u64;

        invoke_raw_typed!(
            hash,
                cam, 
                phase
        )
    }
}

/// Sets the smoothing style for a DEFAULT_SPLINE_CAMERA
Ranges from 0 to 3 in rockstar scripts although there are actually 26
```
0: No lead-in or lead-out smoothing
1: Smooth lead-in
2: Smooth lead-out
3: Both lead-in and lead-out are smoothed
4-6: Longer speed up, lead-in, lead-out, and both in order as above. 
6: see above, but missed a node in testing(?)
7: Smoothed lead-in, longer smoothed lead-out
8: Longer lead-in and lead-out than 6, didn't drop node
9: Constant acceleration
10: Constant deceleration. Dropped 2 nodes in testing.
11: Same as 0
12: 10 but slower lead-in, reaches end node less early
13: Extremely close to 3, slightly longer lead-in/lead-out
14: Constant acceleration, dropped last 2 nodes in testing and halted (?)
15: Very similar to 10, did not drop any nodes.
16: Long lead-in, dropped 2 nodes in testing, very long leadout.
17: Constant acceleration, slower speed-up than 9
18: Same as 17 is to 9, slightly longer lead-out, lingers at end node
19: Very long lead in and out
20: Very long, gradual lead-in acceleration at start, gets extremely fast
21: Same as 20 but for constant deceleration
22: 20 and 21 combined, long linger at end node. Dropped 2 nodes in testing
23: Constant acceleration, doesn't complete path before it stops
24: Same as 23 but with constant deceleration, but completes path
25: 23 and 24 combined, insanely fast at middle.
26: No noticable lead-in, misses last 2 nodes in testing
27+: Alternates between 0 and 26
```

The above is documented and graphed at [Spline Cam Interp Graphs](https://docs.google.com/spreadsheets/d/1ejyiMcEYrhhQOL0mLe8664UN-vU4Oh-SBqQnVcKlFIk/edit?usp=sharing)

```

Using 1-3 will result in misalignment from the passed durations for the spline nodes, the overall duration will remain but other nodes will be shortened if smoothing anything.

Graph below demonstrates interpolation between 0-1000 and back 10 times.

![](https://i.imgur.com/cixWh7m.png)



pub fn set_cam_spline_smoothing_style_safe(
        
        
            cam: 
        , 
        
        
            smoothingStyle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1B0F412F109EA5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1B0F412F109EA5Du64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                smoothingStyle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_spline_smoothing_style_raw(
        cam: , 
        smoothingStyle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1B0F412F109EA5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1B0F412F109EA5Du64;

        invoke_raw_typed!(
            hash,
                cam, 
                smoothingStyle
        )
    }
}

/// ## Parameters
*



pub fn set_gameplay_hint_fov_safe(
        
        
            FOV: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x513403FB9C56211Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x513403FB9C56211Fu64;
        
        let result = invoke_raw!(
            hash,
                FOV
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gameplay_hint_fov_raw(
        FOV: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x513403FB9C56211Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x513403FB9C56211Fu64;

        invoke_raw_typed!(
            hash,
                FOV
        )
    }
}

/// ## Parameters
*



pub fn is_sphere_visible_safe(
        
        
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
        let hash = 0xE33D59DA70B58FDFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE33D59DA70B58FDFu64;
        
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
pub fn is_sphere_visible_raw(
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE33D59DA70B58FDFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE33D59DA70B58FDFu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// ## Return value



pub fn is_gameplay_hint_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE520FF1AD2785B40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE520FF1AD2785B40u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_gameplay_hint_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE520FF1AD2785B40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE520FF1AD2785B40u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_cinematic_cam_shake_amplitude_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC724C701C30B2FE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC724C701C30B2FE7u64;
        
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
pub fn set_cinematic_cam_shake_amplitude_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC724C701C30B2FE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC724C701C30B2FE7u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Example from michael2 script.  
CAM::ANIMATED_SHAKE_CAM(l_5069, "shake_cam_all@", "light", "", 1f);  
```



pub fn animated_shake_cam_safe(
        
        
            cam: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            amplitude: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2746EEAE3E577CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2746EEAE3E577CDu64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                p1, 
                p2, 
                p3, 
                amplitude
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn animated_shake_cam_raw(
        cam: , 
        p1: , 
        p2: , 
        p3: , 
        amplitude: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2746EEAE3E577CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2746EEAE3E577CDu64;

        invoke_raw_typed!(
            hash,
                cam, 
                p1, 
                p2, 
                p3, 
                amplitude
        )
    }
}

/// Renders the camera previously created with [CREATE_CAM](#_0xC3981DCE61D9E13F) or [CREATE_CAMERA](#_0x5E3CF89C6BCCA67D)

```
NativeDB Added Parameter 6: Any p5
```



pub fn render_script_cams_safe(
        
        
            render: 
        , 
        
        
            ease: 
        , 
        
        
            easeTime: 
        , 
        
        
            easeCoordsAnim: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07E5B515DB0636FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07E5B515DB0636FCu64;
        
        let result = invoke_raw!(
            hash,
                render, 
                ease, 
                easeTime, 
                easeCoordsAnim, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn render_script_cams_raw(
        render: , 
        ease: , 
        easeTime: , 
        easeCoordsAnim: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07E5B515DB0636FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07E5B515DB0636FCu64;

        invoke_raw_typed!(
            hash,
                render, 
                ease, 
                easeTime, 
                easeCoordsAnim, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn override_cam_spline_velocity_safe(
        
        
            cam: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40B62FA033EB0346u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40B62FA033EB0346u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn override_cam_spline_velocity_raw(
        cam: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40B62FA033EB0346u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40B62FA033EB0346u64;

        invoke_raw_typed!(
            hash,
                cam, 
                p1, 
                p2, 
                p3
        )
    }
}

/// Create a camera with the specified cam name/type, You can use `SET_CAM_` natives to manipulate the camera.

Take a look at [CREATE_CAM](#_0xC3981DCE61D9E13F) if you would like to see the available camera names.



pub fn create_cam_with_params_safe(
        
        
            camName: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            fov: 
        , 
        
        
            active: 
        , 
        
        
            rotationOrder: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB51194800B257161u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB51194800B257161u64;
        
        let result = invoke_raw!(
            hash,
                camName, 
                posX, 
                posY, 
                posZ, 
                rotX, 
                rotY, 
                rotZ, 
                fov, 
                active, 
                rotationOrder
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_cam_with_params_raw(
        camName: , 
        posX: , 
        posY: , 
        posZ: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        fov: , 
        active: , 
        rotationOrder: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB51194800B257161u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB51194800B257161u64;

        invoke_raw_typed!(
            hash,
                camName, 
                posX, 
                posY, 
                posZ, 
                rotX, 
                rotY, 
                rotZ, 
                fov, 
                active, 
                rotationOrder
        )
    }
}

/// ## Parameters
*



pub fn set_cam_spline_node_extra_flags_safe(
        
        
            cam: 
        , 
        
        
            p1: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7BF1A54AE67AC070u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7BF1A54AE67AC070u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                p1, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_spline_node_extra_flags_raw(
        cam: , 
        p1: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7BF1A54AE67AC070u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7BF1A54AE67AC070u64;

        invoke_raw_typed!(
            hash,
                cam, 
                p1, 
                flags
        )
    }
}

/// _0x0AA27680A0BD43FA native function



pub fn _0x0aa27680a0bd43fa_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AA27680A0BD43FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AA27680A0BD43FAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x0aa27680a0bd43fa_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AA27680A0BD43FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AA27680A0BD43FAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x3044240d2e0fa842_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3044240D2E0FA842u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3044240D2E0FA842u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x3044240d2e0fa842_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3044240D2E0FA842u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3044240D2E0FA842u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Examples:  
CAM::PLAY_SYNCHRONIZED_CAM_ANIM(l_2734, NETWORK::_02C40BF885C567B6(l_2739), "PLAYER_EXIT_L_CAM", "mp_doorbell");  
CAM::PLAY_SYNCHRONIZED_CAM_ANIM(l_F0D[7/*1*/], l_F4D[15/*1*/], "ah3b_attackheli_cam2", "missheistfbi3b_helicrash");  
```

[Animations list](https://alexguirre.github.io/animations-list/)



pub fn play_synchronized_cam_anim_safe(
        
        
            camera: 
        , 
        
        
            scene: 
        , 
        
        
            animName: 
        , 
        
        
            animDictionary: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE32EFE9AB4A9AA0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE32EFE9AB4A9AA0Cu64;
        
        let result = invoke_raw!(
            hash,
                camera, 
                scene, 
                animName, 
                animDictionary
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_synchronized_cam_anim_raw(
        camera: , 
        scene: , 
        animName: , 
        animDictionary: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE32EFE9AB4A9AA0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE32EFE9AB4A9AA0Cu64;

        invoke_raw_typed!(
            hash,
                camera, 
                scene, 
                animName, 
                animDictionary
        )
    }
}

/// ## Return value



pub fn is_screen_faded_in_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A859503B0C08678u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A859503B0C08678u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_screen_faded_in_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A859503B0C08678u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A859503B0C08678u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Specifies when the camera should stop being in focus. Can be used together with [`SET_USE_HI_DOF`](#_0xA13B0222F3D94A94), [`SET_CAM_NEAR_DOF`](#_0x3FA4BF0A7AB7DE2C), [`SET_CAM_USE_SHALLOW_DOF_MODE`](#_0x16A96863A17552BB), [`SET_CAM_DOF_STRENGTH`](#_0x5EE29B4D7D5DF897) and other DoF related natives.



pub fn set_cam_far_dof_safe(
        
        
            cam: 
        , 
        
        
            farDOF: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDD91296CD01AEE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDD91296CD01AEE0u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                farDOF
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_far_dof_raw(
        cam: , 
        farDOF: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDD91296CD01AEE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDD91296CD01AEE0u64;

        invoke_raw_typed!(
            hash,
                cam, 
                farDOF
        )
    }
}

/// ## Return value



pub fn is_aim_cam_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68EDDA28A5976D07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68EDDA28A5976D07u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_aim_cam_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68EDDA28A5976D07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68EDDA28A5976D07u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
some camera effect that is (also) used in the drunk-cheat, and turned off (by setting it to 0.0) along with the shaking effects once the drunk cheat is disabled. Possibly a cinematic or script-cam version of _0x487A82C650EB7799  
```



pub fn _0x0225778816fdc28c_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0225778816FDC28Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0225778816FDC28Cu64;
        
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
pub fn _0x0225778816fdc28c_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0225778816FDC28Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0225778816FDC28Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Seems to animate the gameplay camera zoom.  
Eg. _ANIMATE_GAMEPLAY_CAM_ZOOM(1f, 1000f);  
will animate the camera zooming in from 1000 meters away.  
Game scripts use it like this:  
// Setting this to 1 prevents V key from changing zoom  
PLAYER::SET_PLAYER_FORCED_ZOOM(PLAYER::PLAYER_ID(), 1);  
// These restrict how far you can move cam up/down left/right  
CAM::_CLAMP_GAMEPLAY_CAM_YAW(-20f, 50f);  
CAM::_CLAMP_GAMEPLAY_CAM_PITCH(-60f, 0f);  
CAM::_ANIMATE_GAMEPLAY_CAM_ZOOM(1f, 1f);  
```



pub fn _animate_gameplay_cam_zoom_safe(
        
        
            p0: 
        , 
        
        
            distance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF2E1F7742402E81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF2E1F7742402E81u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                distance
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _animate_gameplay_cam_zoom_raw(
        p0: , 
        distance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF2E1F7742402E81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF2E1F7742402E81u64;

        invoke_raw_typed!(
            hash,
                p0, 
                distance
        )
    }
}

/// ## Parameters
*



pub fn _0x271017b9ba825366_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x271017B9BA825366u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x271017B9BA825366u64;
        
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
pub fn _0x271017b9ba825366_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x271017B9BA825366u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x271017B9BA825366u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
CAM::ANIMATED_SHAKE_SCRIPT_GLOBAL("SHAKE_CAM_medium", "medium", "", 0.5f);
```



pub fn animated_shake_script_global_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2EAE3FB8CDBED31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2EAE3FB8CDBED31u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn animated_shake_script_global_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2EAE3FB8CDBED31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2EAE3FB8CDBED31u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn set_cam_spline_node_velocity_scale_safe(
        
        
            cam: 
        , 
        
        
            p1: 
        , 
        
        
            scale: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6385DEB180F319Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6385DEB180F319Fu64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                p1, 
                scale
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_spline_node_velocity_scale_raw(
        cam: , 
        p1: , 
        scale: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6385DEB180F319Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6385DEB180F319Fu64;

        invoke_raw_typed!(
            hash,
                cam, 
                p1, 
                scale
        )
    }
}

/// ```
Shows the crosshair even if it wouldn't show normally. Only works for one frame, so make sure to call it repeatedly.  
```



pub fn _enable_crosshair_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA7F0AD7E9BA676Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA7F0AD7E9BA676Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _enable_crosshair_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA7F0AD7E9BA676Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA7F0AD7E9BA676Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn is_cam_spline_paused_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0290F35C0AD97864u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0290F35C0AD97864u64;
        
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
pub fn is_cam_spline_paused_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0290F35C0AD97864u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0290F35C0AD97864u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Possible shake types (updated b617d):  
DEATH_FAIL_IN_EFFECT_SHAKE  
DRUNK_SHAKE  
FAMILY5_DRUG_TRIP_SHAKE  
HAND_SHAKE  
JOLT_SHAKE  
LARGE_EXPLOSION_SHAKE  
MEDIUM_EXPLOSION_SHAKE  
SMALL_EXPLOSION_SHAKE  
ROAD_VIBRATION_SHAKE  
SKY_DIVING_SHAKE  
VIBRATE_SHAKE  
```



pub fn shake_cam_safe(
        
        
            cam: 
        , 
        
        
            type: 
        , 
        
        
            amplitude: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A25241C340D3822u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A25241C340D3822u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                type, 
                amplitude
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn shake_cam_raw(
        cam: , 
        type: , 
        amplitude: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A25241C340D3822u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A25241C340D3822u64;

        invoke_raw_typed!(
            hash,
                cam, 
                type, 
                amplitude
        )
    }
}

/// ## Return value



pub fn get_rendering_cam_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5234F9F10919EABAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5234F9F10919EABAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_rendering_cam_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5234F9F10919EABAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5234F9F10919EABAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Atleast one time in a script for the zRot Rockstar uses GET_ENTITY_HEADING to help fill the parameter.  
p9 is unknown at this time.  
p10 throughout all the X360 Scripts is always 2.  
```

[Animations list](https://alexguirre.github.io/animations-list/)



pub fn play_cam_anim_safe(
        
        
            cam: 
        , 
        
        
            animName: 
        , 
        
        
            animDictionary: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            xRot: 
        , 
        
        
            yRot: 
        , 
        
        
            zRot: 
        , 
        
        
            p9: 
        , 
        
        
            p10: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A2D0FB2E7852392u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A2D0FB2E7852392u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                animName, 
                animDictionary, 
                x, 
                y, 
                z, 
                xRot, 
                yRot, 
                zRot, 
                p9, 
                p10
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_cam_anim_raw(
        cam: , 
        animName: , 
        animDictionary: , 
        x: , 
        y: , 
        z: , 
        xRot: , 
        yRot: , 
        zRot: , 
        p9: , 
        p10: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A2D0FB2E7852392u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A2D0FB2E7852392u64;

        invoke_raw_typed!(
            hash,
                cam, 
                animName, 
                animDictionary, 
                x, 
                y, 
                z, 
                xRot, 
                yRot, 
                zRot, 
                p9, 
                p10
        )
    }
}

/// ## Return value



pub fn is_first_person_aim_cam_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E346D934122613Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E346D934122613Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_first_person_aim_cam_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E346D934122613Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E346D934122613Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p6 & p7 - possibly length or time  
```



pub fn set_gameplay_entity_hint_safe(
        
        
            entity: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x189E955A8313E298u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x189E955A8313E298u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                xOffset, 
                yOffset, 
                zOffset, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gameplay_entity_hint_raw(
        entity: , 
        xOffset: , 
        yOffset: , 
        zOffset: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x189E955A8313E298u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x189E955A8313E298u64;

        invoke_raw_typed!(
            hash,
                entity, 
                xOffset, 
                yOffset, 
                zOffset, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8
        )
    }
}

/// Interpolates the camera to specified parameters over a set duration using various curve types for position, rotation, and fov.

```
NativeDB Introduced: v3258
```



pub fn _interpolate_cam_with_params_safe(
        
        
            camera: 
        , 
        
        
            camPosX: 
        , 
        
        
            camPosY: 
        , 
        
        
            camPosZ: 
        , 
        
        
            camRotX: 
        , 
        
        
            camRotY: 
        , 
        
        
            camRotZ: 
        , 
        
        
            fov: 
        , 
        
        
            duration: 
        , 
        
        
            posCurveType: 
        , 
        
        
            rotCurveType: 
        , 
        
        
            rotOrder: 
        , 
        
        
            fovCurveType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDA77EE33C005AAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDA77EE33C005AAFu64;
        
        let result = invoke_raw!(
            hash,
                camera, 
                camPosX, 
                camPosY, 
                camPosZ, 
                camRotX, 
                camRotY, 
                camRotZ, 
                fov, 
                duration, 
                posCurveType, 
                rotCurveType, 
                rotOrder, 
                fovCurveType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _interpolate_cam_with_params_raw(
        camera: , 
        camPosX: , 
        camPosY: , 
        camPosZ: , 
        camRotX: , 
        camRotY: , 
        camRotZ: , 
        fov: , 
        duration: , 
        posCurveType: , 
        rotCurveType: , 
        rotOrder: , 
        fovCurveType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDA77EE33C005AAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDA77EE33C005AAFu64;

        invoke_raw_typed!(
            hash,
                camera, 
                camPosX, 
                camPosY, 
                camPosZ, 
                camRotX, 
                camRotY, 
                camRotZ, 
                fov, 
                duration, 
                posCurveType, 
                rotCurveType, 
                rotOrder, 
                fovCurveType
        )
    }
}

/// ## Parameters
*



pub fn stop_cinematic_cam_shaking_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2238E588E588A6D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2238E588E588A6D7u64;
        
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
pub fn stop_cinematic_cam_shaking_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2238E588E588A6D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2238E588E588A6D7u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Sets the ambient ped & vehicle population spawning origin to be based around the active scripted camera for this frame
This will prevent vehicles from being created close to the camera and/or on-screen



pub fn use_script_cam_for_ambient_population_origin_this_frame_safe(
        
        
            vehicles: 
        , 
        
        
            peds: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x271401846BD26E92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x271401846BD26E92u64;
        
        let result = invoke_raw!(
            hash,
                vehicles, 
                peds
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn use_script_cam_for_ambient_population_origin_this_frame_raw(
        vehicles: , 
        peds: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x271401846BD26E92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x271401846BD26E92u64;

        invoke_raw_typed!(
            hash,
                vehicles, 
                peds
        )
    }
}

/// This native works with peds only.



pub fn attach_cam_to_ped_bone_safe(
        
        
            cam: 
        , 
        
        
            ped: 
        , 
        
        
            boneIndex: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
        , 
        
        
            isRelative: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61A3DBA14AB7F411u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61A3DBA14AB7F411u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                ped, 
                boneIndex, 
                xOffset, 
                yOffset, 
                zOffset, 
                isRelative
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn attach_cam_to_ped_bone_raw(
        cam: , 
        ped: , 
        boneIndex: , 
        xOffset: , 
        yOffset: , 
        zOffset: , 
        isRelative: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61A3DBA14AB7F411u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61A3DBA14AB7F411u64;

        invoke_raw_typed!(
            hash,
                cam, 
                ped, 
                boneIndex, 
                xOffset, 
                yOffset, 
                zOffset, 
                isRelative
        )
    }
}

/// ```
I filled p1-p6 (the floats) as they are as other natives with 6 floats in a row are similar and I see no other method. So if a test from anyone proves them wrong please correct.  
p7 (length) determines the length of the spline, affects camera path and duration of transition between previous node and this one  
p8 big values ~100 will slow down the camera movement before reaching this node  
p9 != 0 seems to override the rotation/pitch (bool?)  
```



pub fn add_cam_spline_node_safe(
        
        
            camera: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            xRot: 
        , 
        
        
            yRot: 
        , 
        
        
            zRot: 
        , 
        
        
            length: 
        , 
        
        
            p8: 
        , 
        
        
            transitionType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8609C75EC438FB3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8609C75EC438FB3Bu64;
        
        let result = invoke_raw!(
            hash,
                camera, 
                x, 
                y, 
                z, 
                xRot, 
                yRot, 
                zRot, 
                length, 
                p8, 
                transitionType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_cam_spline_node_raw(
        camera: , 
        x: , 
        y: , 
        z: , 
        xRot: , 
        yRot: , 
        zRot: , 
        length: , 
        p8: , 
        transitionType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8609C75EC438FB3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8609C75EC438FB3Bu64;

        invoke_raw_typed!(
            hash,
                camera, 
                x, 
                y, 
                z, 
                xRot, 
                yRot, 
                zRot, 
                length, 
                p8, 
                transitionType
        )
    }
}

/// STOP_CUTSCENE_CAM_SHAKING native function



pub fn stop_cutscene_cam_shaking_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB629FFD9285FA06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB629FFD9285FA06u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_cutscene_cam_shaking_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB629FFD9285FA06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB629FFD9285FA06u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x62ECFCFDEE7885D6 native function



pub fn _0x62ecfcfdee7885d6_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62ECFCFDEE7885D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62ECFCFDEE7885D6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x62ecfcfdee7885d6_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62ECFCFDEE7885D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62ECFCFDEE7885D6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Fades the screen in.  
duration: The time the fade should take, in milliseconds.  
```



pub fn do_screen_fade_in_safe(
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4E8E24955024033u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4E8E24955024033u64;
        
        let result = invoke_raw!(
            hash,
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn do_screen_fade_in_raw(
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4E8E24955024033u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4E8E24955024033u64;

        invoke_raw_typed!(
            hash,
                duration
        )
    }
}

/// ## Parameters
*



pub fn get_cam_coord_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAC038F7459AE5AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAC038F7459AE5AEu64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cam_coord_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAC038F7459AE5AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAC038F7459AE5AEu64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// Instructs the game engine to stop rendering scripted cameras and transition back to the gameplay camera, optionally applying custom blending and rendering options.

```c
enum eRenderingOptionFlags {
    RO_NO_OPTIONS = 0,
    RO_STOP_RENDERING_OPTION_WHEN_PLAYER_EXITS_INTO_COVER = 1
};
```

```c
enum eCamSplineSmoothingFlags {
	// No smoothing just moves at a constant rate
	CAM_SPLINE_NO_SMOOTH = 0,
	// Decelerates when approaching a node
	CAM_SPLINE_SLOW_IN_SMOOTH = 1, 
	// Accelerates slowly when leaving a node
	CAM_SPLINE_SLOW_OUT_SMOOTH = 2,    
	// Decelerates when approaching a node and accelerates slowly when leaving a node
	CAM_SPLINE_SLOW_IN_OUT_SMOOTH = 3,
	CAM_SPLINE_VERY_SLOW_IN = 4,
	CAM_SPLINE_VERY_SLOW_OUT = 5,
	CAM_SPLINE_VERY_SLOW_IN_SLOW_OUT = 6,
	CAM_SPLINE_SLOW_IN_VERY_SLOW_OUT = 7,
	CAM_SPLINE_VERY_SLOW_IN_VERY_SLOW_OUT = 8,
	CAM_SPLINE_EASE_IN = 9,
	CAM_SPLINE_EASE_OUT = 10,
	CAM_SPLINE_QUADRATIC_EASE_IN = 11,
	CAM_SPLINE_QUADRATIC_EASE_OUT = 12,
	CAM_SPLINE_QUADRATIC_EASE_IN_OUT = 13,
	CAM_SPLINE_CUBIC_EASE_IN = 14,
	CAM_SPLINE_CUBIC_EASE_OUT = 15,
	CAM_SPLINE_CUBIC_EASE_IN_OUT = 16,
	CAM_SPLINE_QUARTIC_EASE_IN = 17,
	CAM_SPLINE_QUARTIC_EASE_OUT = 18,
	CAM_SPLINE_QUARTIC_EASE_IN_OUT = 19,
	CAM_SPLINE_QUINTIC_EASE_IN = 20,
	CAM_SPLINE_QUINTIC_EASE_OUT = 21,
	CAM_SPLINE_QUINTIC_EASE_IN_OUT = 22,
	CAM_SPLINE_CIRCULAR_EASE_IN = 23,
	CAM_SPLINE_CIRCULAR_EASE_OUT = 24,
	CAM_SPLINE_CIRCULAR_EASE_IN_OUT = 25 
};
```

```
NativeDB Added Parameter 4: int renderingOptions : An integer bitmask of eRenderingOptionFlags to apply specific rendering behaviors during the transition. RO_NO_OPTIONS signifies no special options are applied.
```

```
NativeDB Introduced: v323
```



pub fn stop_rendering_script_cams_using_catch_up_safe(
        
        
            bShouldApplyAcrossAllThreads: 
        , 
        
        
            distanceToBlend: 
        , 
        
        
            blendType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC819F3CBB62BF692u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC819F3CBB62BF692u64;
        
        let result = invoke_raw!(
            hash,
                bShouldApplyAcrossAllThreads, 
                distanceToBlend, 
                blendType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_rendering_script_cams_using_catch_up_raw(
        bShouldApplyAcrossAllThreads: , 
        distanceToBlend: , 
        blendType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC819F3CBB62BF692u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC819F3CBB62BF692u64;

        invoke_raw_typed!(
            hash,
                bShouldApplyAcrossAllThreads, 
                distanceToBlend, 
                blendType
        )
    }
}

/// Looks up a camera handle in the current camera pool and returns `true` if the handle is found, otherwise it returns `false`.



pub fn does_cam_exist_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7A932170592B50Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7A932170592B50Eu64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_cam_exist_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7A932170592B50Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7A932170592B50Eu64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ## Parameters
*



pub fn create_cinematic_shot_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x741B0129D4560F31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x741B0129D4560F31u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_cinematic_shot_raw(
        p0: , 
        p1: , 
        p2: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x741B0129D4560F31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x741B0129D4560F31u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                entity
        )
    }
}

/// ## Parameters
*



pub fn stop_cinematic_shot_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7660C6E75D3A078Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7660C6E75D3A078Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_cinematic_shot_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7660C6E75D3A078Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7660C6E75D3A078Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Resets the vehicle idle camera timer. Calling this in a loop will disable the idle camera.



pub fn _invalidate_vehicle_idle_cam_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E4CFFF989258472u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E4CFFF989258472u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _invalidate_vehicle_idle_cam_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E4CFFF989258472u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E4CFFF989258472u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
some camera effect that is used in the drunk-cheat, and turned off (by setting it to 0.0) along with the shaking effects once the drunk cheat is disabled.  
```



pub fn custom_menu_coordinates_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x487A82C650EB7799u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x487A82C650EB7799u64;
        
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
pub fn custom_menu_coordinates_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x487A82C650EB7799u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x487A82C650EB7799u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn is_cam_shaking_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B24BFE83A2BE47Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B24BFE83A2BE47Bu64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_cam_shaking_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B24BFE83A2BE47Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B24BFE83A2BE47Bu64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ```
Sets the amplitude for the gameplay (i.e. 3rd or 1st) camera to shake. Used in script "drunk_controller.ysc.c4" to simulate making the player drunk.  
```



pub fn set_gameplay_cam_shake_amplitude_safe(
        
        
            amplitude: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA87E00932DB4D85Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA87E00932DB4D85Du64;
        
        let result = invoke_raw!(
            hash,
                amplitude
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gameplay_cam_shake_amplitude_raw(
        amplitude: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA87E00932DB4D85Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA87E00932DB4D85Du64;

        invoke_raw_typed!(
            hash,
                amplitude
        )
    }
}

/// Sets the rotation of the camera.



pub fn set_cam_rot_safe(
        
        
            cam: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            rotationOrder: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85973643155D0B07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85973643155D0B07u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                rotX, 
                rotY, 
                rotZ, 
                rotationOrder
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_rot_raw(
        cam: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        rotationOrder: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85973643155D0B07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85973643155D0B07u64;

        invoke_raw_typed!(
            hash,
                cam, 
                rotX, 
                rotY, 
                rotZ, 
                rotationOrder
        )
    }
}

/// ## Return value



pub fn get_gameplay_cam_relative_heading_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x743607648ADD4587u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x743607648ADD4587u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_gameplay_cam_relative_heading_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x743607648ADD4587u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x743607648ADD4587u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Attaches a camera to an entity, including full matrix transformations for both rotation and position offsets.
```
NativeDB Introduced: v2189
```



pub fn hard_attach_cam_to_entity_safe(
        
        
            cam: 
        , 
        
        
            entity: 
        , 
        
        
            xRot: 
        , 
        
        
            yRot: 
        , 
        
        
            zRot: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
        , 
        
        
            isRelative: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x202A5ED9CE01D6E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x202A5ED9CE01D6E7u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                entity, 
                xRot, 
                yRot, 
                zRot, 
                xOffset, 
                yOffset, 
                zOffset, 
                isRelative
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hard_attach_cam_to_entity_raw(
        cam: , 
        entity: , 
        xRot: , 
        yRot: , 
        zRot: , 
        xOffset: , 
        yOffset: , 
        zOffset: , 
        isRelative: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x202A5ED9CE01D6E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x202A5ED9CE01D6E7u64;

        invoke_raw_typed!(
            hash,
                cam, 
                entity, 
                xRot, 
                yRot, 
                zRot, 
                xOffset, 
                yOffset, 
                zOffset, 
                isRelative
        )
    }
}

/// ```
Max value for p1 is 15.  
```



pub fn override_cam_spline_motion_blur_safe(
        
        
            cam: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DCF7C708D292D55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DCF7C708D292D55u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                p1, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn override_cam_spline_motion_blur_raw(
        cam: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DCF7C708D292D55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DCF7C708D292D55u64;

        invoke_raw_typed!(
            hash,
                cam, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn _0x91ef6ee6419e5b97_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x91EF6EE6419E5B97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x91EF6EE6419E5B97u64;
        
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
pub fn _0x91ef6ee6419e5b97_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x91EF6EE6419E5B97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x91EF6EE6419E5B97u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v3258
```



pub fn _activate_cam_with_interp_and_fov_curve_safe(
        
        
            camTo: 
        , 
        
        
            camFrom: 
        , 
        
        
            duration: 
        , 
        
        
            easeLocation: 
        , 
        
        
            easeRotation: 
        , 
        
        
            easeFove: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34CFC4C2A38E83E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34CFC4C2A38E83E3u64;
        
        let result = invoke_raw!(
            hash,
                camTo, 
                camFrom, 
                duration, 
                easeLocation, 
                easeRotation, 
                easeFove
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _activate_cam_with_interp_and_fov_curve_raw(
        camTo: , 
        camFrom: , 
        duration: , 
        easeLocation: , 
        easeRotation: , 
        easeFove: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34CFC4C2A38E83E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34CFC4C2A38E83E3u64;

        invoke_raw_typed!(
            hash,
                camTo, 
                camFrom, 
                duration, 
                easeLocation, 
                easeRotation, 
                easeFove
        )
    }
}

/// ## Return value



pub fn is_screen_fading_in_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C544BC6C57AC575u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C544BC6C57AC575u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_screen_fading_in_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C544BC6C57AC575u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C544BC6C57AC575u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xc8b5c4a79cc18b94_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8B5C4A79CC18B94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8B5C4A79CC18B94u64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc8b5c4a79cc18b94_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8B5C4A79CC18B94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8B5C4A79CC18B94u64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ## Parameters
*



pub fn _set_gameplay_cam_vehicle_camera_name_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11FA5D3479C7DD47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11FA5D3479C7DD47u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_gameplay_cam_vehicle_camera_name_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11FA5D3479C7DD47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11FA5D3479C7DD47u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_fly_cam_coord_and_constrain_safe(
        
        
            cam: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC91C6C55199308CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC91C6C55199308CAu64;
        
        let result = invoke_raw!(
            hash,
                cam, 
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
pub fn set_fly_cam_coord_and_constrain_raw(
        cam: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC91C6C55199308CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC91C6C55199308CAu64;

        invoke_raw_typed!(
            hash,
                cam, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn _0xfd3151cd37ea2245_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD3151CD37EA2245u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD3151CD37EA2245u64;
        
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
pub fn _0xfd3151cd37ea2245_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD3151CD37EA2245u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD3151CD37EA2245u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x380b4968d1e09e55_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x380B4968D1E09E55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x380B4968D1E09E55u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x380b4968d1e09e55_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x380B4968D1E09E55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x380B4968D1E09E55u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_final_rendered_cam_far_dof_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9780F32BCAF72431u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9780F32BCAF72431u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_final_rendered_cam_far_dof_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9780F32BCAF72431u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9780F32BCAF72431u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// See [`GET_FOLLOW_PED_CAM_VIEW_MODE`](#_0x8D4D46230B2C353A) for the follow mode enum.



pub fn set_follow_ped_cam_view_mode_safe(
        
        
            viewMode: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A4F9EDF1673F704u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A4F9EDF1673F704u64;
        
        let result = invoke_raw!(
            hash,
                viewMode
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_follow_ped_cam_view_mode_raw(
        viewMode: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A4F9EDF1673F704u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A4F9EDF1673F704u64;

        invoke_raw_typed!(
            hash,
                viewMode
        )
    }
}

/// ## Parameters
*



pub fn disable_cam_collision_for_object_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49482F9FCD825AAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49482F9FCD825AAAu64;
        
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
pub fn disable_cam_collision_for_object_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49482F9FCD825AAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49482F9FCD825AAAu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn set_gameplay_object_hint_safe(
        
        
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
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83E87508A2CA2AC6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83E87508A2CA2AC6u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gameplay_object_hint_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83E87508A2CA2AC6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83E87508A2CA2AC6u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7
        )
    }
}

/// Needs to be called every tick to make the active camera use a high depth of field.  
The DoF can be customized using [`SET_CAM_NEAR_DOF`](#_0x3FA4BF0A7AB7DE2C), [`SET_CAM_FAR_DOF`](#_0xEDD91296CD01AEE0), [`SET_CAM_USE_SHALLOW_DOF_MODE`](#_0x16A96863A17552BB), [`SET_CAM_DOF_STRENGTH`](#_0x5EE29B4D7D5DF897) and other DoF related natives.



pub fn set_use_hi_dof_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA13B0222F3D94A94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA13B0222F3D94A94u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_use_hi_dof_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA13B0222F3D94A94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA13B0222F3D94A94u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_cam_params_safe(
        
        
            cam: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            fieldOfView: 
        , 
        
        
            transitionSpeed: 
        , 
        
        
            p9: 
        , 
        
        
            p10: 
        , 
        
        
            rotationOrder: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFD8727AEA3CCEBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFD8727AEA3CCEBAu64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                posX, 
                posY, 
                posZ, 
                rotX, 
                rotY, 
                rotZ, 
                fieldOfView, 
                transitionSpeed, 
                p9, 
                p10, 
                rotationOrder
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_params_raw(
        cam: , 
        posX: , 
        posY: , 
        posZ: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        fieldOfView: , 
        transitionSpeed: , 
        p9: , 
        p10: , 
        rotationOrder: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFD8727AEA3CCEBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFD8727AEA3CCEBAu64;

        invoke_raw_typed!(
            hash,
                cam, 
                posX, 
                posY, 
                posZ, 
                rotX, 
                rotY, 
                rotZ, 
                fieldOfView, 
                transitionSpeed, 
                p9, 
                p10, 
                rotationOrder
        )
    }
}

/// ## Parameters
*



pub fn get_focus_ped_on_screen_safe(
        
        
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
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89215EC747DF244Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89215EC747DF244Au64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_focus_ped_on_screen_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89215EC747DF244Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89215EC747DF244Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8
        )
    }
}

/// ```
Last param determines if its relative to the Entity  
```



pub fn attach_cam_to_entity_safe(
        
        
            cam: 
        , 
        
        
            entity: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
        , 
        
        
            isRelative: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEDB7D269E8C60E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEDB7D269E8C60E3u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                entity, 
                xOffset, 
                yOffset, 
                zOffset, 
                isRelative
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn attach_cam_to_entity_raw(
        cam: , 
        entity: , 
        xOffset: , 
        yOffset: , 
        zOffset: , 
        isRelative: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEDB7D269E8C60E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEDB7D269E8C60E3u64;

        invoke_raw_typed!(
            hash,
                cam, 
                entity, 
                xOffset, 
                yOffset, 
                zOffset, 
                isRelative
        )
    }
}

/// ```
Examples when this function will return 0 are:
- During busted screen.
- When player is coming out from a hospital.
- When player is coming out from a police station.
- When player is buying gun from AmmuNation.
```



pub fn is_gameplay_cam_rendering_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39B5D1B10383F0C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39B5D1B10383F0C8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_gameplay_cam_rendering_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39B5D1B10383F0C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39B5D1B10383F0C8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Determines if a global camera shake is currently active. You can stop the currently active global camera shake using [STOP_SCRIPT_GLOBAL_SHAKING](#_0x1C9D7949FA533490).

```
NativeDB Introduced: v323
```



pub fn is_script_global_shaking_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC912AF078AF19212u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC912AF078AF19212u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_script_global_shaking_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC912AF078AF19212u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC912AF078AF19212u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Can use this with SET_CAM_SPLINE_PHASE to set the float it this native returns.  
(returns 1.0f when no nodes has been added, reached end of non existing spline)  
```



pub fn get_cam_spline_phase_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5349E36C546509Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5349E36C546509Au64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cam_spline_phase_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5349E36C546509Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5349E36C546509Au64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ## Return value



pub fn _0xbf72910d0f26f025_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF72910D0F26F025u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF72910D0F26F025u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xbf72910d0f26f025_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF72910D0F26F025u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF72910D0F26F025u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// p1: 0..16

```
NativeDB Introduced: v2060
```



pub fn _0x5d96cfb59da076a0_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D96CFB59DA076A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D96CFB59DA076A0u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5d96cfb59da076a0_raw(
        vehicle: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D96CFB59DA076A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D96CFB59DA076A0u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn set_first_person_aim_cam_zoom_factor_safe(
        
        
            zoomFactor: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70894BD0915C5BCAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70894BD0915C5BCAu64;
        
        let result = invoke_raw!(
            hash,
                zoomFactor
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_first_person_aim_cam_zoom_factor_raw(
        zoomFactor: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70894BD0915C5BCAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70894BD0915C5BCAu64;

        invoke_raw_typed!(
            hash,
                zoomFactor
        )
    }
}

/// ## Return value



pub fn get_final_rendered_cam_near_clip_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0082607100D7193u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0082607100D7193u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_final_rendered_cam_near_clip_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0082607100D7193u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0082607100D7193u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xdb90c6cca48940f1_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB90C6CCA48940F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB90C6CCA48940F1u64;
        
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
pub fn _0xdb90c6cca48940f1_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB90C6CCA48940F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB90C6CCA48940F1u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _0x1f2300cb7fa7b7f6_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F2300CB7FA7B7F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F2300CB7FA7B7F6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x1f2300cb7fa7b7f6_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F2300CB7FA7B7F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F2300CB7FA7B7F6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_cam_motion_blur_strength_safe(
        
        
            cam: 
        , 
        
        
            strength: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F0F77FBA9A8F2E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F0F77FBA9A8F2E6u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                strength
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_motion_blur_strength_raw(
        cam: , 
        strength: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F0F77FBA9A8F2E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F0F77FBA9A8F2E6u64;

        invoke_raw_typed!(
            hash,
                cam, 
                strength
        )
    }
}

/// ```
Returns whether or not the passed camera handle is active.  
```



pub fn is_cam_active_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFB2B516207D3534u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFB2B516207D3534u64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_cam_active_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFB2B516207D3534u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFB2B516207D3534u64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// ```
Toggles the vehicle cinematic cam; requires the player ped to be in a vehicle to work.
```



pub fn set_cinematic_mode_active_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCF0754AC3D6FD4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCF0754AC3D6FD4Eu64;
        
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
pub fn set_cinematic_mode_active_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCF0754AC3D6FD4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCF0754AC3D6FD4Eu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// _0x59424BD75174C9B1 native function



pub fn _0x59424bd75174c9b1_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59424BD75174C9B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59424BD75174C9B1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x59424bd75174c9b1_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59424BD75174C9B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59424BD75174C9B1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Applies a predefined set of vehicle camera settings optimized for capturing stunts, effective for the current game update/frame.

```
NativeDB Introduced: v791
```



pub fn use_vehicle_cam_stunt_settings_this_update_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6493CF69859B116Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6493CF69859B116Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn use_vehicle_cam_stunt_settings_this_update_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6493CF69859B116Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6493CF69859B116Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Disables first person camera while in a vehicle for the current tick.



pub fn disable_cinematic_bonnet_camera_this_update_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADFF1B2A555F5FBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADFF1B2A555F5FBAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_cinematic_bonnet_camera_this_update_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADFF1B2A555F5FBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADFF1B2A555F5FBAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_final_rendered_cam_far_clip_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFC8CBC606FDB0FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFC8CBC606FDB0FCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_final_rendered_cam_far_clip_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFC8CBC606FDB0FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFC8CBC606FDB0FCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Gets a camera's rotation by handle (`cam`) lookup, outputs a `Vector3` in degrees.



pub fn get_cam_rot_safe(
        
        
            cam: 
        , 
        
        
            rotationOrder: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D304C1C955E3E12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D304C1C955E3E12u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                rotationOrder
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cam_rot_raw(
        cam: , 
        rotationOrder: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D304C1C955E3E12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D304C1C955E3E12u64;

        invoke_raw_typed!(
            hash,
                cam, 
                rotationOrder
        )
    }
}

/// ## Parameters
*



pub fn get_cam_spline_node_index_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB22B17DF858716A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB22B17DF858716A6u64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cam_spline_node_index_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB22B17DF858716A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB22B17DF858716A6u64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// Makes the minimap follow a scripted camera's rotation instead of the gameplay cam.



pub fn set_cam_controls_mini_map_heading_safe(
        
        
            cam: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x661B5C8654ADD825u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x661B5C8654ADD825u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_controls_mini_map_heading_raw(
        cam: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x661B5C8654ADD825u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x661B5C8654ADD825u64;

        invoke_raw_typed!(
            hash,
                cam, 
                toggle
        )
    }
}

/// ```
Possible shake types (updated b617d):  
DEATH_FAIL_IN_EFFECT_SHAKE  
DRUNK_SHAKE  
FAMILY5_DRUG_TRIP_SHAKE  
HAND_SHAKE  
JOLT_SHAKE  
LARGE_EXPLOSION_SHAKE  
MEDIUM_EXPLOSION_SHAKE  
SMALL_EXPLOSION_SHAKE  
ROAD_VIBRATION_SHAKE  
SKY_DIVING_SHAKE  
VIBRATE_SHAKE  
```



pub fn shake_gameplay_cam_safe(
        
        
            shakeName: 
        , 
        
        
            intensity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD55E49555E017CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD55E49555E017CFu64;
        
        let result = invoke_raw!(
            hash,
                shakeName, 
                intensity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn shake_gameplay_cam_raw(
        shakeName: , 
        intensity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD55E49555E017CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD55E49555E017CFu64;

        invoke_raw_typed!(
            hash,
                shakeName, 
                intensity
        )
    }
}

/// ## Return value



pub fn get_final_rendered_cam_fov_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80EC114669DAEFF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80EC114669DAEFF4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_final_rendered_cam_fov_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80EC114669DAEFF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80EC114669DAEFF4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Hardcoded to only work in multiplayer.  
```



pub fn _0x12ded8ca53d47ea5_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12DED8CA53D47EA5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12DED8CA53D47EA5u64;
        
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
pub fn _0x12ded8ca53d47ea5_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12DED8CA53D47EA5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12DED8CA53D47EA5u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Takes a camera and uses the information from it as a camera spline node.



pub fn add_cam_spline_node_using_camera_safe(
        
        
            cam: 
        , 
        
        
            cam2: 
        , 
        
        
            length: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0FB82563989CF4FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0FB82563989CF4FBu64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                cam2, 
                length, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_cam_spline_node_using_camera_raw(
        cam: , 
        cam2: , 
        length: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0FB82563989CF4FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0FB82563989CF4FBu64;

        invoke_raw_typed!(
            hash,
                cam, 
                cam2, 
                length, 
                p3
        )
    }
}

/// ```
IS_A*
```



pub fn _is_aim_cam_third_person_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74BD83EA840F6BC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74BD83EA840F6BC9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_aim_cam_third_person_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74BD83EA840F6BC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74BD83EA840F6BC9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```c
// view mode enumeration
enum eCamViewMode 
{
	THIRD_PERSON_NEAR = 0,
	THIRD_PERSON_MEDIUM = 1,
	THIRD_PERSON_FAR = 2,
	CINEMATIC = 3,
	FIRST_PERSON = 4,
};
```



pub fn get_follow_ped_cam_view_mode_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D4D46230B2C353Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D4D46230B2C353Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_follow_ped_cam_view_mode_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D4D46230B2C353Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D4D46230B2C353Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_screen_faded_out_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB16FCE9DDC7BA182u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB16FCE9DDC7BA182u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_screen_faded_out_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB16FCE9DDC7BA182u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB16FCE9DDC7BA182u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_gameplay_hint_follow_distance_scalar_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8BDBF3D573049A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8BDBF3D573049A1u64;
        
        let result = invoke_raw!(
            hash,
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gameplay_hint_follow_distance_scalar_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8BDBF3D573049A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8BDBF3D573049A1u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ```
I'm pretty sure the parameter is the camera as usual, but I am not certain so I'm going to leave it as is.  
```



pub fn get_cam_spline_node_phase_safe(
        
        
            cam: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9D0E694C8282C96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9D0E694C8282C96u64;
        
        let result = invoke_raw!(
            hash,
                cam
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cam_spline_node_phase_raw(
        cam: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9D0E694C8282C96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9D0E694C8282C96u64;

        invoke_raw_typed!(
            hash,
                cam
        )
    }
}

/// Disables first person camera while on foot for the current tick.



pub fn disable_on_foot_first_person_view_this_update_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE2EF5DA284CC8DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE2EF5DA284CC8DFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_on_foot_first_person_view_this_update_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE2EF5DA284CC8DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE2EF5DA284CC8DFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_cam_spline_phase_safe(
        
        
            cam: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x242B5874F0A4E052u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x242B5874F0A4E052u64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_spline_phase_raw(
        cam: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x242B5874F0A4E052u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x242B5874F0A4E052u64;

        invoke_raw_typed!(
            hash,
                cam, 
                p1
        )
    }
}

/// DISABLE_AIM_CAM_THIS_UPDATE native function



pub fn disable_aim_cam_this_update_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A31FE0049E542F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A31FE0049E542F6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_aim_cam_this_update_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A31FE0049E542F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A31FE0049E542F6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Create a camera with the specified camera hash, You can use `SET_CAM_` natives to manipulate the camera.
Make sure to call [RENDER_SCRIPT_CAMS](#_0x07E5B515DB0636FC) once the camera is created, or this won't have any visible effect.

Take a look at [CREATE_CAM](#_0xC3981DCE61D9E13F) if you would like to see the available camera names.

```
NativeDB Introduced: v323
```



pub fn create_camera_with_params_safe(
        
        
            camHash: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            fov: 
        , 
        
        
            active: 
        , 
        
        
            rotationOrder: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6ABFA3E16460F22Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6ABFA3E16460F22Du64;
        
        let result = invoke_raw!(
            hash,
                camHash, 
                posX, 
                posY, 
                posZ, 
                rotX, 
                rotY, 
                rotZ, 
                fov, 
                active, 
                rotationOrder
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_camera_with_params_raw(
        camHash: , 
        posX: , 
        posY: , 
        posZ: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        fov: , 
        active: , 
        rotationOrder: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6ABFA3E16460F22Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6ABFA3E16460F22Du64;

        invoke_raw_typed!(
            hash,
                camHash, 
                posX, 
                posY, 
                posZ, 
                rotX, 
                rotY, 
                rotZ, 
                fov, 
                active, 
                rotationOrder
        )
    }
}

/// ```
Sets the position of the cam.  
```



pub fn set_cam_coord_safe(
        
        
            cam: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D41783FB745E42Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D41783FB745E42Eu64;
        
        let result = invoke_raw!(
            hash,
                cam, 
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
pub fn set_cam_coord_raw(
        cam: , 
        posX: , 
        posY: , 
        posZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D41783FB745E42Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D41783FB745E42Eu64;

        invoke_raw_typed!(
            hash,
                cam, 
                posX, 
                posY, 
                posZ
        )
    }
}

/// Focuses the camera on the specified vehicle.



pub fn set_gameplay_vehicle_hint_safe(
        
        
            vehicle: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        , 
        
        
            p4: 
        , 
        
        
            time: 
        , 
        
        
            easeInTime: 
        , 
        
        
            easeOutTime: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2297E18F3E71C2Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2297E18F3E71C2Eu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                offsetX, 
                offsetY, 
                offsetZ, 
                p4, 
                time, 
                easeInTime, 
                easeOutTime
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gameplay_vehicle_hint_raw(
        vehicle: , 
        offsetX: , 
        offsetY: , 
        offsetZ: , 
        p4: , 
        time: , 
        easeInTime: , 
        easeOutTime: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2297E18F3E71C2Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2297E18F3E71C2Eu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                offsetX, 
                offsetY, 
                offsetZ, 
                p4, 
                time, 
                easeInTime, 
                easeOutTime
        )
    }
}

/// ## Return value



pub fn is_gameplay_cam_shaking_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x016C090630DF1F89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x016C090630DF1F89u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_gameplay_cam_shaking_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x016C090630DF1F89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x016C090630DF1F89u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// IGNORE_MENU_PREFERENCE_FOR_BONNET_CAMERA_THIS_UPDATE native function



pub fn ignore_menu_preference_for_bonnet_camera_this_update_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B8A361C1813FBEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B8A361C1813FBEFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ignore_menu_preference_for_bonnet_camera_this_update_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B8A361C1813FBEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B8A361C1813FBEFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
This native has a name defined inside its code  
```



pub fn _set_cam_dof_focus_distance_bias_safe(
        
        
            camera: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC669EEA5D031B7DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC669EEA5D031B7DEu64;
        
        let result = invoke_raw!(
            hash,
                camera, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_cam_dof_focus_distance_bias_raw(
        camera: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC669EEA5D031B7DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC669EEA5D031B7DEu64;

        invoke_raw_typed!(
            hash,
                camera, 
                p1
        )
    }
}

/// Specifies when the camera should start being in focus. Can be used together with [`SET_USE_HI_DOF`](#_0xA13B0222F3D94A94), [`SET_CAM_FAR_DOF`](#_0xEDD91296CD01AEE0), [`SET_CAM_USE_SHALLOW_DOF_MODE`](#_0x16A96863A17552BB), [`SET_CAM_DOF_STRENGTH`](#_0x5EE29B4D7D5DF897) and other DoF related natives.



pub fn set_cam_near_dof_safe(
        
        
            cam: 
        , 
        
        
            nearDOF: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FA4BF0A7AB7DE2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FA4BF0A7AB7DE2Cu64;
        
        let result = invoke_raw!(
            hash,
                cam, 
                nearDOF
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cam_near_dof_raw(
        cam: , 
        nearDOF: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3FA4BF0A7AB7DE2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3FA4BF0A7AB7DE2Cu64;

        invoke_raw_typed!(
            hash,
                cam, 
                nearDOF
        )
    }
}

/// _0x17FCA7199A530203 native function



pub fn _0x17fca7199a530203_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17FCA7199A530203u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17FCA7199A530203u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x17fca7199a530203_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17FCA7199A530203u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17FCA7199A530203u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

