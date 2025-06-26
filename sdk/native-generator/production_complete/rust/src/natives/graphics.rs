//! GRAPHICS native functions
//! 
//! Functions for the graphics category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn _disable_script_ambient_effects_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFD97FF47B745B8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFD97FF47B745B8Du64;
        
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
pub fn _disable_script_ambient_effects_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFD97FF47B745B8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFD97FF47B745B8Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_scaleform_movie_as_no_longer_needed_safe(
        
        
            scaleformHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D132D614DD86811u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D132D614DD86811u64;
        
        let result = invoke_raw!(
            hash,
                scaleformHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_scaleform_movie_as_no_longer_needed_raw(
        scaleformHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D132D614DD86811u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D132D614DD86811u64;

        invoke_raw_typed!(
            hash,
                scaleformHandle
        )
    }
}

/// ```
This function is hard-coded to always return 0.  
```



pub fn _0xe791df1f73ed2c8b_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE791DF1F73ED2C8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE791DF1F73ED2C8Bu64;
        
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
pub fn _0xe791df1f73ed2c8b_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE791DF1F73ED2C8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE791DF1F73ED2C8Bu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xba0127da25fd54c9_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA0127DA25FD54C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA0127DA25FD54C9u64;
        
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
pub fn _0xba0127da25fd54c9_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA0127DA25FD54C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA0127DA25FD54C9u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Usage example for C#:  
Function.Call(Hash.SET_PARTICLE_FX_NON_LOOPED_ALPHA, new InputArgument[] { 0.1f });  
		Note: the argument alpha ranges from 0.0f-1.0f !  
```



pub fn set_particle_fx_non_looped_alpha_safe(
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77168D722C58B2FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77168D722C58B2FCu64;
        
        let result = invoke_raw!(
            hash,
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_particle_fx_non_looped_alpha_raw(
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77168D722C58B2FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77168D722C58B2FCu64;

        invoke_raw_typed!(
            hash,
                alpha
        )
    }
}

/// ## Parameters
*



pub fn fade_decals_in_range_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD77EDADB0420E6E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD77EDADB0420E6E0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn fade_decals_in_range_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD77EDADB0420E6E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD77EDADB0420E6E0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn trigger_screenblur_fade_in_safe(
        
        
            transitionTime: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA328A24AAA6B7FDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA328A24AAA6B7FDCu64;
        
        let result = invoke_raw!(
            hash,
                transitionTime
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn trigger_screenblur_fade_in_raw(
        transitionTime: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA328A24AAA6B7FDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA328A24AAA6B7FDCu64;

        invoke_raw_typed!(
            hash,
                transitionTime
        )
    }
}

/// ## Parameters
*



pub fn get_status_of_load_mission_creator_photo_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1670F8D05056F257u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1670F8D05056F257u64;
        
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
pub fn get_status_of_load_mission_creator_photo_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1670F8D05056F257u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1670F8D05056F257u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
network fx  
```

```
NativeDB Added Parameter 13: Any p12
NativeDB Added Parameter 14: Any p13
NativeDB Added Parameter 15: Any p14
NativeDB Added Parameter 16: Any p15
```



pub fn start_networked_particle_fx_looped_on_entity_safe(
        
        
            effectName: 
        , 
        
        
            entity: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
        , 
        
        
            xRot: 
        , 
        
        
            yRot: 
        , 
        
        
            zRot: 
        , 
        
        
            scale: 
        , 
        
        
            xAxis: 
        , 
        
        
            yAxis: 
        , 
        
        
            zAxis: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F60E89A7B64EE1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F60E89A7B64EE1Du64;
        
        let result = invoke_raw!(
            hash,
                effectName, 
                entity, 
                xOffset, 
                yOffset, 
                zOffset, 
                xRot, 
                yRot, 
                zRot, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_networked_particle_fx_looped_on_entity_raw(
        effectName: , 
        entity: , 
        xOffset: , 
        yOffset: , 
        zOffset: , 
        xRot: , 
        yRot: , 
        zRot: , 
        scale: , 
        xAxis: , 
        yAxis: , 
        zAxis: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F60E89A7B64EE1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F60E89A7B64EE1Du64;

        invoke_raw_typed!(
            hash,
                effectName, 
                entity, 
                xOffset, 
                yOffset, 
                zOffset, 
                xRot, 
                yRot, 
                zRot, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis
        )
    }
}

/// ```
SET_PARTICLE_FX_*
```



pub fn _0xba3d194057c79a7b_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA3D194057C79A7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA3D194057C79A7Bu64;
        
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
pub fn _0xba3d194057c79a7b_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA3D194057C79A7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA3D194057C79A7Bu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Similar to [_DRAW_SPRITE](#_0xE7FFAE5EBF23D890), but seems to be some kind of "interactive" sprite, at least used by render targets.
These seem to be the only dicts ever requested by this native:
```
prop_screen_biker_laptop
Prop_Screen_GR_Disruption
Prop_Screen_TaleOfUs
prop_screen_nightclub
Prop_Screen_IE_Adhawk
prop_screen_sm_free_trade_shipping
prop_screen_hacker_truck
MPDesktop
Prop_Screen_Nightclub
And a few others
```



pub fn _draw_interactive_sprite_safe(
        
        
            textureDict: 
        , 
        
        
            textureName: 
        , 
        
        
            screenX: 
        , 
        
        
            screenY: 
        , 
        
        
            width: 
        , 
        
        
            height: 
        , 
        
        
            heading: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BC54A8188768488u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BC54A8188768488u64;
        
        let result = invoke_raw!(
            hash,
                textureDict, 
                textureName, 
                screenX, 
                screenY, 
                width, 
                height, 
                heading, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _draw_interactive_sprite_raw(
        textureDict: , 
        textureName: , 
        screenX: , 
        screenY: , 
        width: , 
        height: , 
        heading: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BC54A8188768488u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BC54A8188768488u64;

        invoke_raw_typed!(
            hash,
                textureDict, 
                textureName, 
                screenX, 
                screenY, 
                width, 
                height, 
                heading, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ```
GRAPHICS::START_PARTICLE_FX_LOOPED_AT_COORD("scr_fbi_falling_debris", 93.7743f, -749.4572f, 70.86904f, 0f, 0f, 0f, 0x3F800000, 0, 0, 0, 0)  
p11 seems to be always 0  
```



pub fn start_particle_fx_looped_at_coord_safe(
        
        
            effectName: 
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
        
        
            scale: 
        , 
        
        
            xAxis: 
        , 
        
        
            yAxis: 
        , 
        
        
            zAxis: 
        , 
        
        
            p11: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE184F4F0DC5910E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE184F4F0DC5910E7u64;
        
        let result = invoke_raw!(
            hash,
                effectName, 
                x, 
                y, 
                z, 
                xRot, 
                yRot, 
                zRot, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis, 
                p11
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_particle_fx_looped_at_coord_raw(
        effectName: , 
        x: , 
        y: , 
        z: , 
        xRot: , 
        yRot: , 
        zRot: , 
        scale: , 
        xAxis: , 
        yAxis: , 
        zAxis: , 
        p11: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE184F4F0DC5910E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE184F4F0DC5910E7u64;

        invoke_raw_typed!(
            hash,
                effectName, 
                x, 
                y, 
                z, 
                xRot, 
                yRot, 
                zRot, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis, 
                p11
        )
    }
}

/// ## Parameters
*



pub fn set_particle_fx_looped_offsets_safe(
        
        
            ptfxHandle: 
        , 
        
        
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7DDEBEC43483C43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7DDEBEC43483C43u64;
        
        let result = invoke_raw!(
            hash,
                ptfxHandle, 
                x, 
                y, 
                z, 
                rotX, 
                rotY, 
                rotZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_particle_fx_looped_offsets_raw(
        ptfxHandle: , 
        x: , 
        y: , 
        z: , 
        rotX: , 
        rotY: , 
        rotZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7DDEBEC43483C43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7DDEBEC43483C43u64;

        invoke_raw_typed!(
            hash,
                ptfxHandle, 
                x, 
                y, 
                z, 
                rotX, 
                rotY, 
                rotZ
        )
    }
}

/// ## Parameters
*



pub fn override_interior_smoke_level_safe(
        
        
            level: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1600FD8CF72EBC12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1600FD8CF72EBC12u64;
        
        let result = invoke_raw!(
            hash,
                level
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn override_interior_smoke_level_raw(
        level: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1600FD8CF72EBC12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1600FD8CF72EBC12u64;

        invoke_raw_typed!(
            hash,
                level
        )
    }
}

/// This native draws a textured polygon between three vectors in the game world. It's commonly utilized for rendering deadline trailing lights, with additional details available in the `deadline.ytd` file. UVW mapping details (u,v,w parameters) can be found on various internet resources. This native is specifically used for drawing textured polygons on the screen, where UV coordinates define the texture mapping and color/alpha parameters define the appearance of the polygon. This native should be called every frame for continuous rendering.

```
NativeDB Introduced: v877
```



pub fn draw_textured_poly_safe(
        
        
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
        
        
            x3: 
        , 
        
        
            y3: 
        , 
        
        
            z3: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        , 
        
        
            textureDict: 
        , 
        
        
            textureName: 
        , 
        
        
            u1: 
        , 
        
        
            v1: 
        , 
        
        
            w1: 
        , 
        
        
            u2: 
        , 
        
        
            v2: 
        , 
        
        
            w2: 
        , 
        
        
            u3: 
        , 
        
        
            v3: 
        , 
        
        
            w3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29280002282F1928u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29280002282F1928u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                x3, 
                y3, 
                z3, 
                red, 
                green, 
                blue, 
                alpha, 
                textureDict, 
                textureName, 
                u1, 
                v1, 
                w1, 
                u2, 
                v2, 
                w2, 
                u3, 
                v3, 
                w3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_textured_poly_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        x3: , 
        y3: , 
        z3: , 
        red: , 
        green: , 
        blue: , 
        alpha: , 
        textureDict: , 
        textureName: , 
        u1: , 
        v1: , 
        w1: , 
        u2: , 
        v2: , 
        w2: , 
        u3: , 
        v3: , 
        w3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29280002282F1928u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29280002282F1928u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                x3, 
                y3, 
                z3, 
                red, 
                green, 
                blue, 
                alpha, 
                textureDict, 
                textureName, 
                u1, 
                v1, 
                w1, 
                u2, 
                v2, 
                w2, 
                u3, 
                v3, 
                w3
        )
    }
}

/// _0xC35A6D07C93802B2 native function



pub fn _0xc35a6d07c93802b2_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC35A6D07C93802B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC35A6D07C93802B2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc35a6d07c93802b2_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC35A6D07C93802B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC35A6D07C93802B2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 26: BOOL p25
```



pub fn _draw_marker_2_safe(
        
        
            type: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            dirX: 
        , 
        
        
            dirY: 
        , 
        
        
            dirZ: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            scaleX: 
        , 
        
        
            scaleY: 
        , 
        
        
            scaleZ: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        , 
        
        
            bobUpAndDown: 
        , 
        
        
            faceCamera: 
        , 
        
        
            rotationOrder: 
        , 
        
        
            rotate: 
        , 
        
        
            textureDict: 
        , 
        
        
            textureName: 
        , 
        
        
            drawOnEnts: 
        , 
        
        
            p24: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE82728F0DE75D13Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE82728F0DE75D13Au64;
        
        let result = invoke_raw!(
            hash,
                type, 
                posX, 
                posY, 
                posZ, 
                dirX, 
                dirY, 
                dirZ, 
                rotX, 
                rotY, 
                rotZ, 
                scaleX, 
                scaleY, 
                scaleZ, 
                red, 
                green, 
                blue, 
                alpha, 
                bobUpAndDown, 
                faceCamera, 
                rotationOrder, 
                rotate, 
                textureDict, 
                textureName, 
                drawOnEnts, 
                p24
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _draw_marker_2_raw(
        type: , 
        posX: , 
        posY: , 
        posZ: , 
        dirX: , 
        dirY: , 
        dirZ: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        scaleX: , 
        scaleY: , 
        scaleZ: , 
        red: , 
        green: , 
        blue: , 
        alpha: , 
        bobUpAndDown: , 
        faceCamera: , 
        rotationOrder: , 
        rotate: , 
        textureDict: , 
        textureName: , 
        drawOnEnts: , 
        p24: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE82728F0DE75D13Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE82728F0DE75D13Au64;

        invoke_raw_typed!(
            hash,
                type, 
                posX, 
                posY, 
                posZ, 
                dirX, 
                dirY, 
                dirZ, 
                rotX, 
                rotY, 
                rotZ, 
                scaleX, 
                scaleY, 
                scaleZ, 
                red, 
                green, 
                blue, 
                alpha, 
                bobUpAndDown, 
                faceCamera, 
                rotationOrder, 
                rotate, 
                textureDict, 
                textureName, 
                drawOnEnts, 
                p24
        )
    }
}

/// ## Return value



pub fn get_status_of_take_high_quality_photo_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D6CA79EEEBD8CA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D6CA79EEEBD8CA3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_status_of_take_high_quality_photo_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D6CA79EEEBD8CA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D6CA79EEEBD8CA3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x1612C45F9E3E0D44 native function



pub fn _0x1612c45f9e3e0d44_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1612C45F9E3E0D44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1612C45F9E3E0D44u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x1612c45f9e3e0d44_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1612C45F9E3E0D44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1612C45F9E3E0D44u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
This method is the equivalent to PUSH_SCALEFORM_MOVIE_FUNCTION_PARAMETER_STRING when using it to add a new button (like "INSTRUCTIONAL_BUTTONS").  
When switching with a controller, the icons update and become the controller's icons.  
```



pub fn scaleform_movie_method_add_param_player_name_string_safe(
        
        
            string: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE83A3E3557A56640u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE83A3E3557A56640u64;
        
        let result = invoke_raw!(
            hash,
                string
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn scaleform_movie_method_add_param_player_name_string_raw(
        string: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE83A3E3557A56640u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE83A3E3557A56640u64;

        invoke_raw_typed!(
            hash,
                string
        )
    }
}

/// ## Parameters
*



pub fn _0xb569f41f3e7e83a4_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB569F41F3E7E83A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB569F41F3E7E83A4u64;
        
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
pub fn _0xb569f41f3e7e83a4_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB569F41F3E7E83A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB569F41F3E7E83A4u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Probably changes tvs from being a 3d audio to being "global" audio
```



pub fn set_tv_audio_frontend_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x113D2C5DC57E1774u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x113D2C5DC57E1774u64;
        
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
pub fn set_tv_audio_frontend_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x113D2C5DC57E1774u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x113D2C5DC57E1774u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn query_movie_mesh_set_state_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B6E70C5CEEF4EEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B6E70C5CEEF4EEBu64;
        
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
pub fn query_movie_mesh_set_state_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B6E70C5CEEF4EEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B6E70C5CEEF4EEBu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Returns the texture resolution of the passed texture dict+name.  
Note: Most texture resolutions are doubled compared to the console version of the game.  
```



pub fn get_texture_resolution_safe(
        
        
            textureDict: 
        , 
        
        
            textureName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35736EE65BD00C11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35736EE65BD00C11u64;
        
        let result = invoke_raw!(
            hash,
                textureDict, 
                textureName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_texture_resolution_raw(
        textureDict: , 
        textureName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35736EE65BD00C11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35736EE65BD00C11u64;

        invoke_raw_typed!(
            hash,
                textureDict, 
                textureName
        )
    }
}

/// _0x27CFB1B1E078CB2D native function



pub fn _0x27cfb1b1e078cb2d_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27CFB1B1E078CB2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27CFB1B1E078CB2Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x27cfb1b1e078cb2d_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27CFB1B1E078CB2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27CFB1B1E078CB2Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0xf3f776ada161e47d_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3F776ADA161E47Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3F776ADA161E47Du64;
        
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
pub fn _0xf3f776ada161e47d_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3F776ADA161E47Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3F776ADA161E47Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Returns status of gallery photo fetch, which was requested by [`QUEUE_OPERATION_TO_CREATE_SORTED_LIST_OF_PHOTOS`](#_0x2A893980E96B659A).



pub fn get_status_of_sorted_list_operation_safe(
        
        
            scanForSaving: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5BED327CEA362B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5BED327CEA362B1u64;
        
        let result = invoke_raw!(
            hash,
                scanForSaving
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_status_of_sorted_list_operation_raw(
        scanForSaving: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5BED327CEA362B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5BED327CEA362B1u64;

        invoke_raw_typed!(
            hash,
                scanForSaving
        )
    }
}

/// FREE_MEMORY_FOR_LOW_QUALITY_PHOTO native function



pub fn free_memory_for_low_quality_photo_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A12D88881435DCAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A12D88881435DCAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn free_memory_for_low_quality_photo_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A12D88881435DCAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A12D88881435DCAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Requests a scaleform movie that doesn't render when the game is paused (With [`SET_GAME_PAUSED`](#_0x577D1284D6873711)).



pub fn request_scaleform_movie_skip_render_while_paused_safe(
        
        
            scaleformName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD06C611BB9048C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD06C611BB9048C2u64;
        
        let result = invoke_raw!(
            hash,
                scaleformName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_scaleform_movie_skip_render_while_paused_raw(
        scaleformName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD06C611BB9048C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD06C611BB9048C2u64;

        invoke_raw_typed!(
            hash,
                scaleformName
        )
    }
}

/// Similar to DRAW_SPRITE, but allows to specify the texture coordinates used to draw the sprite.
u1, v1 - texture coordinates for the top-left corner
u2, v2 - texture coordinates for the bottom-right corner

```
NativeDB Introduced: v1868
```



pub fn _draw_sprite_uv_safe(
        
        
            textureDict: 
        , 
        
        
            textureName: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            width: 
        , 
        
        
            height: 
        , 
        
        
            u1: 
        , 
        
        
            v1: 
        , 
        
        
            u2: 
        , 
        
        
            v2: 
        , 
        
        
            heading: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95812F9B26074726u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95812F9B26074726u64;
        
        let result = invoke_raw!(
            hash,
                textureDict, 
                textureName, 
                x, 
                y, 
                width, 
                height, 
                u1, 
                v1, 
                u2, 
                v2, 
                heading, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _draw_sprite_uv_raw(
        textureDict: , 
        textureName: , 
        x: , 
        y: , 
        width: , 
        height: , 
        u1: , 
        v1: , 
        u2: , 
        v2: , 
        heading: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95812F9B26074726u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95812F9B26074726u64;

        invoke_raw_typed!(
            hash,
                textureDict, 
                textureName, 
                x, 
                y, 
                width, 
                height, 
                u1, 
                v1, 
                u2, 
                v2, 
                heading, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ## Return value



pub fn get_status_of_take_mission_creator_photo_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90A78ECAA4E78453u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90A78ECAA4E78453u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_status_of_take_mission_creator_photo_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90A78ECAA4E78453u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90A78ECAA4E78453u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Calculates the effective X/Y fractions when applying the values set by `SET_SCRIPT_GFX_ALIGN` and
`SET_SCRIPT_GFX_ALIGN_PARAMS`.



pub fn _get_script_gfx_position_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            calculatedX: 
        , 
        
        
            calculatedY: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6DD8F5AA635EB4B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6DD8F5AA635EB4B2u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                calculatedX, 
                calculatedY
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_script_gfx_position_raw(
        x: , 
        y: , 
        calculatedX: , 
        calculatedY: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6DD8F5AA635EB4B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6DD8F5AA635EB4B2u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                calculatedX, 
                calculatedY
        )
    }
}

/// ## Parameters
*



pub fn set_entity_icon_visibility_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0E8BEECCA96BA31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0E8BEECCA96BA31u64;
        
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
pub fn set_entity_icon_visibility_raw(
        entity: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0E8BEECCA96BA31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0E8BEECCA96BA31u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn _0x908311265d42a820_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x908311265D42A820u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x908311265D42A820u64;
        
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
pub fn _0x908311265d42a820_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x908311265D42A820u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x908311265D42A820u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _0x393bd2275ceb7793_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x393BD2275CEB7793u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x393BD2275CEB7793u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x393bd2275ceb7793_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x393BD2275CEB7793u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x393BD2275CEB7793u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
x/y/z - Location of a vertex (in world coords), presumably.



pub fn draw_poly_safe(
        
        
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
        
        
            x3: 
        , 
        
        
            y3: 
        , 
        
        
            z3: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC26716048436851u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC26716048436851u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                x3, 
                y3, 
                z3, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_poly_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        x3: , 
        y3: , 
        z3: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC26716048436851u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC26716048436851u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                x3, 
                y3, 
                z3, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _release_bink_movie_safe(
        
        
            binkMovie: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04D950EEFA4EED8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04D950EEFA4EED8Cu64;
        
        let result = invoke_raw!(
            hash,
                binkMovie
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _release_bink_movie_raw(
        binkMovie: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04D950EEFA4EED8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04D950EEFA4EED8Cu64;

        invoke_raw_typed!(
            hash,
                binkMovie
        )
    }
}

/// ## Parameters
*



pub fn _seethrough_set_noise_amount_min_safe(
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF5992E1C9E65D05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF5992E1C9E65D05u64;
        
        let result = invoke_raw!(
            hash,
                amount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _seethrough_set_noise_amount_min_raw(
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF5992E1C9E65D05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF5992E1C9E65D05u64;

        invoke_raw_typed!(
            hash,
                amount
        )
    }
}

/// ## Parameters
*



pub fn set_scaleform_movie_to_use_system_time_safe(
        
        
            scaleform: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D8EB211944DCE08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D8EB211944DCE08u64;
        
        let result = invoke_raw!(
            hash,
                scaleform, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_scaleform_movie_to_use_system_time_raw(
        scaleform: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D8EB211944DCE08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D8EB211944DCE08u64;

        invoke_raw_typed!(
            hash,
                scaleform, 
                toggle
        )
    }
}

/// Sets the draw offset/calculated size for `SET_SCRIPT_GFX_ALIGN`. If using any alignment other than left/top, the game
expects the width/height to be configured using this native in order to get a proper starting position for the draw
command.



pub fn set_script_gfx_align_params_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            w: 
        , 
        
        
            h: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5A2C681787E579Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5A2C681787E579Du64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                w, 
                h
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_script_gfx_align_params_raw(
        x: , 
        y: , 
        w: , 
        h: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5A2C681787E579Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5A2C681787E579Du64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                w, 
                h
        )
    }
}

/// ## Parameters
*



pub fn _0xbb90e12cac1dab25_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB90E12CAC1DAB25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB90E12CAC1DAB25u64;
        
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
pub fn _0xbb90e12cac1dab25_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB90E12CAC1DAB25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB90E12CAC1DAB25u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Removes all decals in range from a position, it includes the bullet holes, blood pools, petrol...  
```



pub fn remove_decals_in_range_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            range: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D6B2D4830A67C62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D6B2D4830A67C62u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                range
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_decals_in_range_raw(
        x: , 
        y: , 
        z: , 
        range: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D6B2D4830A67C62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D6B2D4830A67C62u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                range
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _0x98d18905bf723b99_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98D18905BF723B99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98D18905BF723B99u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x98d18905bf723b99_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98D18905BF723B99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98D18905BF723B99u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
unk is not used so no need  
```



pub fn draw_scaleform_movie_fullscreen_safe(
        
        
            scaleform: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        , 
        
        
            unk: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0DF606929C105BE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0DF606929C105BE1u64;
        
        let result = invoke_raw!(
            hash,
                scaleform, 
                red, 
                green, 
                blue, 
                alpha, 
                unk
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_scaleform_movie_fullscreen_raw(
        scaleform: , 
        red: , 
        green: , 
        blue: , 
        alpha: , 
        unk: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0DF606929C105BE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0DF606929C105BE1u64;

        invoke_raw_typed!(
            hash,
                scaleform, 
                red, 
                green, 
                blue, 
                alpha, 
                unk
        )
    }
}

/// Starts frontend (pause menu) scaleform movie methods for header options.

Use [`BEGIN_SCALEFORM_MOVIE_METHOD_ON_FRONTEND`](#_0xAB58C27C2E6123C6) to customize the content inside the frontend menus.



pub fn begin_scaleform_movie_method_on_frontend_header_safe(
        
        
            functionName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9449845F73F5E9Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9449845F73F5E9Cu64;
        
        let result = invoke_raw!(
            hash,
                functionName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_scaleform_movie_method_on_frontend_header_raw(
        functionName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9449845F73F5E9Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9449845F73F5E9Cu64;

        invoke_raw_typed!(
            hash,
                functionName
        )
    }
}

/// FREE_MEMORY_FOR_HIGH_QUALITY_PHOTO native function



pub fn free_memory_for_high_quality_photo_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD801CC02177FA3F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD801CC02177FA3F1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn free_memory_for_high_quality_photo_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD801CC02177FA3F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD801CC02177FA3F1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Toggles Heatvision on/off.  
```



pub fn set_seethrough_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E08924259E08CE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E08924259E08CE0u64;
        
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
pub fn set_seethrough_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E08924259E08CE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E08924259E08CE0u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn enable_movie_keyframe_wait_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74C180030FDE4B69u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74C180030FDE4B69u64;
        
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
pub fn enable_movie_keyframe_wait_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74C180030FDE4B69u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74C180030FDE4B69u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Draws a rectangle on the screen.  
-x: The relative X point of the center of the rectangle. (0.0-1.0, 0.0 is the left edge of the screen, 1.0 is the right edge of the screen)  
-y: The relative Y point of the center of the rectangle. (0.0-1.0, 0.0 is the top edge of the screen, 1.0 is the bottom edge of the screen)  
-width: The relative width of the rectangle. (0.0-1.0, 1.0 means the whole screen width)  
-height: The relative height of the rectangle. (0.0-1.0, 1.0 means the whole screen height)  
-R: Red part of the color. (0-255)  
-G: Green part of the color. (0-255)  
-B: Blue part of the color. (0-255)  
-A: Alpha part of the color. (0-255, 0 means totally transparent, 255 means totally opaque)  
The total number of rectangles to be drawn in one frame is apparently limited to 399.  
```

```
NativeDB Added Parameter 9: BOOL p8
```



pub fn draw_rect_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            width: 
        , 
        
        
            height: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        , 
        
        
            a: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A618A217E5154F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A618A217E5154F0u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                width, 
                height, 
                r, 
                g, 
                b, 
                a
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_rect_raw(
        x: , 
        y: , 
        width: , 
        height: , 
        r: , 
        g: , 
        b: , 
        a: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A618A217E5154F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A618A217E5154F0u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                width, 
                height, 
                r, 
                g, 
                b, 
                a
        )
    }
}

/// ```
This function is hard-coded to always return 96.
```



pub fn get_maximum_number_of_cloud_photos_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC54A7AF8B3A14EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC54A7AF8B3A14EFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_maximum_number_of_cloud_photos_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC54A7AF8B3A14EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC54A7AF8B3A14EFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_entity_icon_color_safe(
        
        
            entity: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D5F595CCAE2E238u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D5F595CCAE2E238u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_entity_icon_color_raw(
        entity: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D5F595CCAE2E238u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D5F595CCAE2E238u64;

        invoke_raw_typed!(
            hash,
                entity, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ## Parameters
*



pub fn release_movie_mesh_set_safe(
        
        
            movieMeshSet: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB119AA014E89183u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB119AA014E89183u64;
        
        let result = invoke_raw!(
            hash,
                movieMeshSet
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn release_movie_mesh_set_raw(
        movieMeshSet: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB119AA014E89183u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB119AA014E89183u64;

        invoke_raw_typed!(
            hash,
                movieMeshSet
        )
    }
}

/// Seeks a BINK movie to a specified position.



pub fn _set_bink_movie_time_safe(
        
        
            binkMovie: 
        , 
        
        
            progress: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CB6B3446855B57Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CB6B3446855B57Au64;
        
        let result = invoke_raw!(
            hash,
                binkMovie, 
                progress
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_bink_movie_time_raw(
        binkMovie: , 
        progress: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CB6B3446855B57Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CB6B3446855B57Au64;

        invoke_raw_typed!(
            hash,
                binkMovie, 
                progress
        )
    }
}

/// ```
NativeDB Introduced: v1180
```

This native is used for the "larger" circular checkpoints, and sets the circle/ring around the checkpoint to point in the same direction as the inner arrow



pub fn _0xdb1ea9411c8911ec_safe(
        
        
            checkpointHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB1EA9411C8911ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB1EA9411C8911ECu64;
        
        let result = invoke_raw!(
            hash,
                checkpointHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xdb1ea9411c8911ec_raw(
        checkpointHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB1EA9411C8911ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB1EA9411C8911ECu64;

        invoke_raw_typed!(
            hash,
                checkpointHandle
        )
    }
}

/// ```
NOTE: Debugging functions are not present in the retail version of the game.  
```



pub fn draw_debug_box_safe(
        
        
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
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        , 
        
        
            a: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x083A2CA4F2E573BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x083A2CA4F2E573BDu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                r, 
                g, 
                b, 
                a
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_debug_box_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        r: , 
        g: , 
        b: , 
        a: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x083A2CA4F2E573BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x083A2CA4F2E573BDu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                r, 
                g, 
                b, 
                a
        )
    }
}

/// _GRASS_LOD_RESET_SCRIPT_AREAS native function



pub fn _grass_lod_reset_script_areas_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x302C91AB2D477F7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x302C91AB2D477F7Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _grass_lod_reset_script_areas_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x302C91AB2D477F7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x302C91AB2D477F7Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _draw_light_with_range_and_shadow_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        , 
        
        
            range: 
        , 
        
        
            intensity: 
        , 
        
        
            shadow: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF49E9A9716A04595u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF49E9A9716A04595u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                r, 
                g, 
                b, 
                range, 
                intensity, 
                shadow
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _draw_light_with_range_and_shadow_raw(
        x: , 
        y: , 
        z: , 
        r: , 
        g: , 
        b: , 
        range: , 
        intensity: , 
        shadow: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF49E9A9716A04595u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF49E9A9716A04595u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                r, 
                g, 
                b, 
                range, 
                intensity, 
                shadow
        )
    }
}

/// ```
Called prior to adding a text component to the UI. After doing so, GRAPHICS::END_TEXT_COMMAND_SCALEFORM_STRING is called.
Examples:
GRAPHICS::BEGIN_TEXT_COMMAND_SCALEFORM_STRING("NUMBER");
HUD::ADD_TEXT_COMPONENT_INTEGER(MISC::ABSI(a_1));
GRAPHICS::END_TEXT_COMMAND_SCALEFORM_STRING();
GRAPHICS::BEGIN_TEXT_COMMAND_SCALEFORM_STRING("STRING");
HUD::_ADD_TEXT_COMPONENT_STRING(a_2);
GRAPHICS::END_TEXT_COMMAND_SCALEFORM_STRING();
GRAPHICS::BEGIN_TEXT_COMMAND_SCALEFORM_STRING("STRTNM2");
HUD::_0x17299B63C7683A2B(v_3);
HUD::_0x17299B63C7683A2B(v_4);
GRAPHICS::END_TEXT_COMMAND_SCALEFORM_STRING();
GRAPHICS::BEGIN_TEXT_COMMAND_SCALEFORM_STRING("STRTNM1");
HUD::_0x17299B63C7683A2B(v_3);
GRAPHICS::END_TEXT_COMMAND_SCALEFORM_STRING();
```



pub fn begin_text_command_scaleform_string_safe(
        
        
            textLabel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80338406F3475E55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80338406F3475E55u64;
        
        let result = invoke_raw!(
            hash,
                textLabel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_text_command_scaleform_string_raw(
        textLabel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80338406F3475E55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80338406F3475E55u64;

        invoke_raw_typed!(
            hash,
                textLabel
        )
    }
}

/// END_PETROL_TRAIL_DECALS native function



pub fn end_petrol_trail_decals_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A123435A26C36CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A123435A26C36CDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_petrol_trail_decals_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A123435A26C36CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A123435A26C36CDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// This native is used along with these two natives: [`TERRAINGRID_ACTIVATE`](#_0xA356990E161C9E65) and [`TERRAINGRID_SET_COLOURS`](#_0x5CE62918F8D703C7).

This native configures the location, size, rotation, normal height, and the difference ratio between min, normal and max.

All those natives combined they will output something like this: https://i.imgur.com/TC6cku6.png



pub fn terraingrid_set_params_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p3: 
        , 
        
        
            rotation: 
        , 
        
        
            p5: 
        , 
        
        
            width: 
        , 
        
        
            height: 
        , 
        
        
            p8: 
        , 
        
        
            scale: 
        , 
        
        
            glowIntensity: 
        , 
        
        
            normalHeight: 
        , 
        
        
            heightDiff: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C4FC5752BCD8E48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C4FC5752BCD8E48u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                p3, 
                rotation, 
                p5, 
                width, 
                height, 
                p8, 
                scale, 
                glowIntensity, 
                normalHeight, 
                heightDiff
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn terraingrid_set_params_raw(
        x: , 
        y: , 
        z: , 
        p3: , 
        rotation: , 
        p5: , 
        width: , 
        height: , 
        p8: , 
        scale: , 
        glowIntensity: , 
        normalHeight: , 
        heightDiff: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C4FC5752BCD8E48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C4FC5752BCD8E48u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                p3, 
                rotation, 
                p5, 
                width, 
                height, 
                p8, 
                scale, 
                glowIntensity, 
                normalHeight, 
                heightDiff
        )
    }
}

/// Sets a flag defining whether or not script draw commands should continue being drawn behind the pause menu. This is usually used for TV channels and other draw commands that are used with a world render target.



pub fn set_script_gfx_draw_behind_pausemenu_safe(
        
        
            flag: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6372ECD45D73BCDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6372ECD45D73BCDu64;
        
        let result = invoke_raw!(
            hash,
                flag
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_script_gfx_draw_behind_pausemenu_raw(
        flag: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6372ECD45D73BCDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6372ECD45D73BCDu64;

        invoke_raw_typed!(
            hash,
                flag
        )
    }
}

/// ## Parameters
*



pub fn draw_scaleform_movie_safe(
        
        
            scaleformHandle: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            width: 
        , 
        
        
            height: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        , 
        
        
            unk: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54972ADAF0294A93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54972ADAF0294A93u64;
        
        let result = invoke_raw!(
            hash,
                scaleformHandle, 
                x, 
                y, 
                width, 
                height, 
                red, 
                green, 
                blue, 
                alpha, 
                unk
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_scaleform_movie_raw(
        scaleformHandle: , 
        x: , 
        y: , 
        width: , 
        height: , 
        red: , 
        green: , 
        blue: , 
        alpha: , 
        unk: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54972ADAF0294A93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54972ADAF0294A93u64;

        invoke_raw_typed!(
            hash,
                scaleformHandle, 
                x, 
                y, 
                width, 
                height, 
                red, 
                green, 
                blue, 
                alpha, 
                unk
        )
    }
}

/// ```
p8 seems to always be false.  
```



pub fn golf_trail_set_path_safe(
        
        
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
        let hash = 0x312342E1A4874F3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x312342E1A4874F3Fu64;
        
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
pub fn golf_trail_set_path_raw(
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
        let hash = 0x312342E1A4874F3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x312342E1A4874F3Fu64;

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

/// ## Return value



pub fn _0xb2ebe8cbc58b90e9_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2EBE8CBC58B90E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2EBE8CBC58B90E9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb2ebe8cbc58b90e9_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2EBE8CBC58B90E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2EBE8CBC58B90E9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2060
```



pub fn _0xcfd16f0db5a3535c_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFD16F0DB5A3535Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFD16F0DB5A3535Cu64;
        
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
pub fn _0xcfd16f0db5a3535c_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFD16F0DB5A3535Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFD16F0DB5A3535Cu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_particle_fx_cam_inside_nonplayer_vehicle_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xACEE6F360FC1F6B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xACEE6F360FC1F6B6u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_particle_fx_cam_inside_nonplayer_vehicle_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xACEE6F360FC1F6B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xACEE6F360FC1F6B6u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn destroy_tracked_point_safe(
        
        
            point: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB25DC90BAD56CA42u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB25DC90BAD56CA42u64;
        
        let result = invoke_raw!(
            hash,
                point
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn destroy_tracked_point_raw(
        point: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB25DC90BAD56CA42u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB25DC90BAD56CA42u64;

        invoke_raw_typed!(
            hash,
                point
        )
    }
}

/// ```
Resets the effect of SET_PARTICLE_FX_OVERRIDE
```



pub fn reset_particle_fx_override_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89C8553DD3274AAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89C8553DD3274AAEu64;
        
        let result = invoke_raw!(
            hash,
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_particle_fx_override_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89C8553DD3274AAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89C8553DD3274AAEu64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ```
All calls to this native are preceded by calls to GRAPHICS::_0x61BB1D9B3A95D802 and GRAPHICS::_0xC6372ECD45D73BCD, respectively.
"act_cinema.ysc", line 1483:
HUD::SET_HUD_COMPONENT_POSITION(15, 0.0, -0.0375);
HUD::SET_TEXT_RENDER_ID(l_AE);
GRAPHICS::_0x61BB1D9B3A95D802(4);
GRAPHICS::_0xC6372ECD45D73BCD(1);
if (GRAPHICS::_0x0AD973CA1E077B60(${movie_arthouse})) {
    GRAPHICS::DRAW_TV_CHANNEL(0.5, 0.5, 0.7375, 1.0, 0.0, 255, 255, 255, 255);
} else {
    GRAPHICS::DRAW_TV_CHANNEL(0.5, 0.5, 1.0, 1.0, 0.0, 255, 255, 255, 255);
}
"am_mp_property_int.ysc", line 102545:
if (ENTITY::DOES_ENTITY_EXIST(a_2._f3)) {
    if (HUD::IS_NAMED_RENDERTARGET_LINKED(ENTITY::GET_ENTITY_MODEL(a_2._f3))) {
        HUD::SET_TEXT_RENDER_ID(a_2._f1);
        GRAPHICS::_0x61BB1D9B3A95D802(4);
        GRAPHICS::_0xC6372ECD45D73BCD(1);
        GRAPHICS::DRAW_TV_CHANNEL(0.5, 0.5, 1.0, 1.0, 0.0, 255, 255, 255, 255);
        if (GRAPHICS::GET_TV_CHANNEL() == -1) {
            sub_a8fa5(a_2, 1);
        } else {
            sub_a8fa5(a_2, 1);
            GRAPHICS::ATTACH_TV_AUDIO_TO_ENTITY(a_2._f3);
        }
        HUD::SET_TEXT_RENDER_ID(HUD::GET_DEFAULT_SCRIPT_RENDERTARGET_RENDER_ID());
    }
}
```



pub fn draw_tv_channel_safe(
        
        
            xPos: 
        , 
        
        
            yPos: 
        , 
        
        
            xScale: 
        , 
        
        
            yScale: 
        , 
        
        
            rotation: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDDC2B4ED3C69DF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDDC2B4ED3C69DF0u64;
        
        let result = invoke_raw!(
            hash,
                xPos, 
                yPos, 
                xScale, 
                yScale, 
                rotation, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_tv_channel_raw(
        xPos: , 
        yPos: , 
        xScale: , 
        yScale: , 
        rotation: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDDC2B4ED3C69DF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDDC2B4ED3C69DF0u64;

        invoke_raw_typed!(
            hash,
                xPos, 
                yPos, 
                xScale, 
                yScale, 
                rotation, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// Used to get a return value from a scaleform function. Returns an int in the same way GET_SCALEFORM_MOVIE_METHOD_RETURN_VALUE_STRING returns a string.



pub fn get_scaleform_movie_method_return_value_int_safe(
        
        
            method_return: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DE7EFA66B906036u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DE7EFA66B906036u64;
        
        let result = invoke_raw!(
            hash,
                method_return
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_scaleform_movie_method_return_value_int_raw(
        method_return: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DE7EFA66B906036u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DE7EFA66B906036u64;

        invoke_raw_typed!(
            hash,
                method_return
        )
    }
}

/// This function resets the alignment set using `SET_SCRIPT_GFX_ALIGN` and `SET_SCRIPT_GFX_ALIGN_PARAMS` to the default
values ('I', 'I'; 0, 0, 0, 0). This should be used after having used the aforementioned functions in order to not affect
any other scripts attempting to draw.



pub fn reset_script_gfx_align_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3A3DB414A373DABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3A3DB414A373DABu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_script_gfx_align_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3A3DB414A373DABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3A3DB414A373DABu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NOTE: Debugging functions are not present in the retail version of the game.  
```



pub fn set_debug_lines_and_spheres_drawing_active_safe(
        
        
            enabled: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x175B6BFC15CDD0C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x175B6BFC15CDD0C5u64;
        
        let result = invoke_raw!(
            hash,
                enabled
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_debug_lines_and_spheres_drawing_active_raw(
        enabled: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x175B6BFC15CDD0C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x175B6BFC15CDD0C5u64;

        invoke_raw_typed!(
            hash,
                enabled
        )
    }
}

/// ```
Only appeared in Golf & Golf_mp. Parameters were all ptrs  
```



pub fn golf_trail_set_shader_params_safe(
        
        
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
        let hash = 0x9CFDD90B2B844BF7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CFDD90B2B844BF7u64;
        
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
pub fn golf_trail_set_shader_params_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CFDD90B2B844BF7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CFDD90B2B844BF7u64;

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

/// ## Parameters
*



pub fn seethrough_set_color_near_safe(
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1086127B3A63505Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1086127B3A63505Eu64;
        
        let result = invoke_raw!(
            hash,
                red, 
                green, 
                blue
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn seethrough_set_color_near_raw(
        red: , 
        green: , 
        blue: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1086127B3A63505Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1086127B3A63505Eu64;

        invoke_raw_typed!(
            hash,
                red, 
                green, 
                blue
        )
    }
}

/// Creates an integer (usually 1) for a BINK movie to be called with other natives.
[List of all BINK movies (alphabetically ordered) as of b2802](https://gist.github.com/ItsJunction/8046f28c29ea8ff2821e9e4f933f595f)



pub fn _set_bink_movie_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x338D9F609FD632DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x338D9F609FD632DBu64;
        
        let result = invoke_raw!(
            hash,
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_bink_movie_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x338D9F609FD632DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x338D9F609FD632DBu64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ```
NativeDB Introduced: v573
```

Configures a Scaleform movie to render to a large render target (1280x720), which is useful for ensuring higher quality and clarity in certain display scenarios. Such as displaying the name of an organization (CEO Office) in a visually impactful way for example.



pub fn set_scaleform_movie_to_use_large_rt_safe(
        
        
            scaleformMovieId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32F34FF7F617643Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32F34FF7F617643Bu64;
        
        let result = invoke_raw!(
            hash,
                scaleformMovieId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_scaleform_movie_to_use_large_rt_raw(
        scaleformMovieId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32F34FF7F617643Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32F34FF7F617643Bu64;

        invoke_raw_typed!(
            hash,
                scaleformMovieId
        )
    }
}

/// Creates a tracked point: useful for checking the visibility of a 3D point on screen.

Tracked points must be manually managed and will not be destroyed on resource stop (they are not an instance of CScriptResource). See [`DESTROY_TRACKED_POINT`](#_0xB25DC90BAD56CA42) and [onResourceStop](https://docs.fivem.net/docs/scripting-reference/events/list/onResourceStop/).

Only 64 points may be tracked at a given time.



pub fn create_tracked_point_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2C9439ED45DEA60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2C9439ED45DEA60u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_tracked_point_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2C9439ED45DEA60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2C9439ED45DEA60u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Used for drawling Deadline trailing lights, see deadline.ytd 

Each vertex has its own colour that is blended/illuminated on the texture. Additionally, the R, G, and B components are floats that are int-casted internally.

For UVW mapping (u,v,w parameters), reference your favourite internet resource for more details.



pub fn _draw_sprite_poly_2_safe(
        
        
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
        
        
            x3: 
        , 
        
        
            y3: 
        , 
        
        
            z3: 
        , 
        
        
            red1: 
        , 
        
        
            green1: 
        , 
        
        
            blue1: 
        , 
        
        
            alpha1: 
        , 
        
        
            red2: 
        , 
        
        
            green2: 
        , 
        
        
            blue2: 
        , 
        
        
            alpha2: 
        , 
        
        
            red3: 
        , 
        
        
            green3: 
        , 
        
        
            blue3: 
        , 
        
        
            alpha3: 
        , 
        
        
            textureDict: 
        , 
        
        
            textureName: 
        , 
        
        
            u1: 
        , 
        
        
            v1: 
        , 
        
        
            w1: 
        , 
        
        
            u2: 
        , 
        
        
            v2: 
        , 
        
        
            w2: 
        , 
        
        
            u3: 
        , 
        
        
            v3: 
        , 
        
        
            w3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x736D7AA1B750856Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x736D7AA1B750856Bu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                x3, 
                y3, 
                z3, 
                red1, 
                green1, 
                blue1, 
                alpha1, 
                red2, 
                green2, 
                blue2, 
                alpha2, 
                red3, 
                green3, 
                blue3, 
                alpha3, 
                textureDict, 
                textureName, 
                u1, 
                v1, 
                w1, 
                u2, 
                v2, 
                w2, 
                u3, 
                v3, 
                w3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _draw_sprite_poly_2_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        x3: , 
        y3: , 
        z3: , 
        red1: , 
        green1: , 
        blue1: , 
        alpha1: , 
        red2: , 
        green2: , 
        blue2: , 
        alpha2: , 
        red3: , 
        green3: , 
        blue3: , 
        alpha3: , 
        textureDict: , 
        textureName: , 
        u1: , 
        v1: , 
        w1: , 
        u2: , 
        v2: , 
        w2: , 
        u3: , 
        v3: , 
        w3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x736D7AA1B750856Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x736D7AA1B750856Bu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                x3, 
                y3, 
                z3, 
                red1, 
                green1, 
                blue1, 
                alpha1, 
                red2, 
                green2, 
                blue2, 
                alpha2, 
                red3, 
                green3, 
                blue3, 
                alpha3, 
                textureDict, 
                textureName, 
                u1, 
                v1, 
                w1, 
                u2, 
                v2, 
                w2, 
                u3, 
                v3, 
                w3
        )
    }
}

/// ## Parameters
*



pub fn _0x0e4299c549f0d1f1_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E4299C549F0D1F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E4299C549F0D1F1u64;
        
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
pub fn _0x0e4299c549f0d1f1_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E4299C549F0D1F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E4299C549F0D1F1u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// This native retrieves the aspect ratio of the game window. If `physicalAspect` is `true`, it returns the physical aspect ratio of the game window, which is useful for 3x1 modes. Otherwise, it returns the aspect ratio of the main game window, considering any custom overrides from the settings menu.

```
NativeDB Introduced: v323
```



pub fn get_aspect_ratio_safe(
        
        
            physicalAspect: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1307EF624A80D87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1307EF624A80D87u64;
        
        let result = invoke_raw!(
            hash,
                physicalAspect
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_aspect_ratio_raw(
        physicalAspect: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1307EF624A80D87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1307EF624A80D87u64;

        invoke_raw_typed!(
            hash,
                physicalAspect
        )
    }
}

/// ```
Push a function from the Scaleform onto the stack  
```



pub fn begin_scaleform_movie_method_safe(
        
        
            scaleform: 
        , 
        
        
            methodName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6E48914C7A8694Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6E48914C7A8694Eu64;
        
        let result = invoke_raw!(
            hash,
                scaleform, 
                methodName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_scaleform_movie_method_raw(
        scaleform: , 
        methodName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6E48914C7A8694Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6E48914C7A8694Eu64;

        invoke_raw_typed!(
            hash,
                scaleform, 
                methodName
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _seethrough_get_max_thickness_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43DBAE39626CE83Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43DBAE39626CE83Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _seethrough_get_max_thickness_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43DBAE39626CE83Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43DBAE39626CE83Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Pushes a function from the Hud component Scaleform onto the stack. Same behavior as GRAPHICS::BEGIN_SCALEFORM_MOVIE_METHOD, just a hud component id instead of a Scaleform.
Known components:
19 - MP_RANK_BAR
20 - HUD_DIRECTOR_MODE
This native requires more research - all information can be found inside of 'hud.gfx'. Using a decompiler, the different components are located under "scripts\__Packages\com\rockstargames\gtav\hud\hudComponents" and "scripts\__Packages\com\rockstargames\gtav\Multiplayer".
```



pub fn begin_scaleform_script_hud_movie_method_safe(
        
        
            hudComponent: 
        , 
        
        
            methodName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98C494FD5BDFBFD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98C494FD5BDFBFD5u64;
        
        let result = invoke_raw!(
            hash,
                hudComponent, 
                methodName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_scaleform_script_hud_movie_method_raw(
        hudComponent: , 
        methodName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98C494FD5BDFBFD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98C494FD5BDFBFD5u64;

        invoke_raw_typed!(
            hash,
                hudComponent, 
                methodName
        )
    }
}

/// ## Parameters
*



pub fn does_particle_fx_looped_exist_safe(
        
        
            ptfxHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74AFEF0D2E1E409Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74AFEF0D2E1E409Bu64;
        
        let result = invoke_raw!(
            hash,
                ptfxHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_particle_fx_looped_exist_raw(
        ptfxHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74AFEF0D2E1E409Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74AFEF0D2E1E409Bu64;

        invoke_raw_typed!(
            hash,
                ptfxHandle
        )
    }
}

/// ## Parameters
*



pub fn add_tcmodifier_override_safe(
        
        
            modifierName1: 
        , 
        
        
            modifierName2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A8E2C8B9CF4549Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A8E2C8B9CF4549Cu64;
        
        let result = invoke_raw!(
            hash,
                modifierName1, 
                modifierName2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_tcmodifier_override_raw(
        modifierName1: , 
        modifierName2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A8E2C8B9CF4549Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A8E2C8B9CF4549Cu64;

        invoke_raw_typed!(
            hash,
                modifierName1, 
                modifierName2
        )
    }
}

/// ## Parameters
*



pub fn _0x814af7dcaacc597b_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x814AF7DCAACC597Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x814AF7DCAACC597Bu64;
        
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
pub fn _0x814af7dcaacc597b_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x814AF7DCAACC597Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x814AF7DCAACC597Bu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
boneIndex is always chassis_dummy in the scripts. The x/y/z params are location relative to the chassis bone.
```



pub fn add_vehicle_crew_emblem_safe(
        
        
            vehicle: 
        , 
        
        
            ped: 
        , 
        
        
            boneIndex: 
        , 
        
        
            x1: 
        , 
        
        
            x2: 
        , 
        
        
            x3: 
        , 
        
        
            y1: 
        , 
        
        
            y2: 
        , 
        
        
            y3: 
        , 
        
        
            z1: 
        , 
        
        
            z2: 
        , 
        
        
            z3: 
        , 
        
        
            scale: 
        , 
        
        
            p13: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x428BDCB9DA58DA53u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x428BDCB9DA58DA53u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                ped, 
                boneIndex, 
                x1, 
                x2, 
                x3, 
                y1, 
                y2, 
                y3, 
                z1, 
                z2, 
                z3, 
                scale, 
                p13, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_vehicle_crew_emblem_raw(
        vehicle: , 
        ped: , 
        boneIndex: , 
        x1: , 
        x2: , 
        x3: , 
        y1: , 
        y2: , 
        y3: , 
        z1: , 
        z2: , 
        z3: , 
        scale: , 
        p13: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x428BDCB9DA58DA53u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x428BDCB9DA58DA53u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                ped, 
                boneIndex, 
                x1, 
                x2, 
                x3, 
                y1, 
                y2, 
                y3, 
                z1, 
                z2, 
                z3, 
                scale, 
                p13, 
                alpha
        )
    }
}

/// This native draws a line between two vectors in the game world. It is typically used for visualizing paths or connections between points. The color of the line is specified by the red, green, and blue parameters, with alpha determining its opacity. This native should be called every frame for continuous rendering.

```
NativeDB Introduced: v323
```



pub fn draw_line_safe(
        
        
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
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B7256074AE34680u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B7256074AE34680u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_line_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B7256074AE34680u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B7256074AE34680u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ## Parameters
*



pub fn _0x759650634f07b6b4_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x759650634F07B6B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x759650634F07B6B4u64;
        
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
pub fn _0x759650634f07b6b4_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x759650634F07B6B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x759650634F07B6B4u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _seethrough_set_max_thickness_safe(
        
        
            thickness: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C8FAC83902A62DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C8FAC83902A62DFu64;
        
        let result = invoke_raw!(
            hash,
                thickness
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _seethrough_set_max_thickness_raw(
        thickness: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C8FAC83902A62DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C8FAC83902A62DFu64;

        invoke_raw_typed!(
            hash,
                thickness
        )
    }
}

/// ```
UI3DSCENE_*
```



pub fn _0x108be26959a9d9bb_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x108BE26959A9D9BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x108BE26959A9D9BBu64;
        
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
pub fn _0x108be26959a9d9bb_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x108BE26959A9D9BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x108BE26959A9D9BBu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Enables Night Vision.  
Example:  
C#: Function.Call(Hash.SET_NIGHTVISION, true);  
C++: GRAPHICS::SET_NIGHTVISION(true);  
BOOL toggle:  
true = turns night vision on for your player.  
false = turns night vision off for your player.  
```



pub fn set_nightvision_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x18F621F7A5B1F85Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x18F621F7A5B1F85Du64;
        
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
pub fn set_nightvision_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x18F621F7A5B1F85Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x18F621F7A5B1F85Du64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1180
```

Sets the given checkpoint target to the new coords



pub fn _0x3c788e7f6438754d_safe(
        
        
            checkpointHandle: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C788E7F6438754Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C788E7F6438754Du64;
        
        let result = invoke_raw!(
            hash,
                checkpointHandle, 
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
pub fn _0x3c788e7f6438754d_raw(
        checkpointHandle: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C788E7F6438754Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C788E7F6438754Du64;

        invoke_raw_typed!(
            hash,
                checkpointHandle, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn set_tv_volume_safe(
        
        
            volume: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2982BF73F66E9DDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2982BF73F66E9DDCu64;
        
        let result = invoke_raw!(
            hash,
                volume
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_tv_volume_raw(
        volume: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2982BF73F66E9DDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2982BF73F66E9DDCu64;

        invoke_raw_typed!(
            hash,
                volume
        )
    }
}

/// ```
Purpose of p0 and p1 unknown.  
```



pub fn set_flash_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            fadeIn: 
        , 
        
        
            duration: 
        , 
        
        
            fadeOut: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AB84296FED9CFC6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AB84296FED9CFC6u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                fadeIn, 
                duration, 
                fadeOut
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_flash_raw(
        p0: , 
        p1: , 
        fadeIn: , 
        duration: , 
        fadeOut: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AB84296FED9CFC6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AB84296FED9CFC6u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                fadeIn, 
                duration, 
                fadeOut
        )
    }
}

/// ```
Pops and calls the Scaleform function on the stack  
```



pub fn end_scaleform_movie_method_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6796A8FFA375E53u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6796A8FFA375E53u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_scaleform_movie_method_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6796A8FFA375E53u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6796A8FFA375E53u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
When this is set to ON, shadows only draw as you get nearer.
When OFF, they draw from a further distance.
```



pub fn cascade_shadows_enable_entity_tracker_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80ECBC0C856D3B0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80ECBC0C856D3B0Bu64;
        
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
pub fn cascade_shadows_enable_entity_tracker_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80ECBC0C856D3B0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80ECBC0C856D3B0Bu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Adjusts a scaleform movie's dimensions to fit a large rendertarget. Mostly used in casino scripts.



pub fn set_scaleform_movie_to_use_super_large_rt_safe(
        
        
            scaleformHandle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6A9F00D4240B519u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6A9F00D4240B519u64;
        
        let result = invoke_raw!(
            hash,
                scaleformHandle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_scaleform_movie_to_use_super_large_rt_raw(
        scaleformHandle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6A9F00D4240B519u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6A9F00D4240B519u64;

        invoke_raw_typed!(
            hash,
                scaleformHandle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn dont_render_in_game_ui_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22A249A53034450Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22A249A53034450Au64;
        
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
pub fn dont_render_in_game_ui_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22A249A53034450Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22A249A53034450Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Pushes a boolean for the Scaleform function onto the stack.  
```



pub fn scaleform_movie_method_add_param_bool_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC58424BA936EB458u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC58424BA936EB458u64;
        
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
pub fn scaleform_movie_method_add_param_bool_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC58424BA936EB458u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC58424BA936EB458u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn seethrough_reset_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70A64C0234EF522Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70A64C0234EF522Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn seethrough_reset_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70A64C0234EF522Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70A64C0234EF522Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x7ac24eab6d74118d_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AC24EAB6D74118Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AC24EAB6D74118Du64;
        
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
pub fn _0x7ac24eab6d74118d_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AC24EAB6D74118Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AC24EAB6D74118Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// END_TEXT_COMMAND_SCALEFORM_STRING native function



pub fn end_text_command_scaleform_string_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x362E2D3FE93A9959u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x362E2D3FE93A9959u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_scaleform_string_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x362E2D3FE93A9959u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x362E2D3FE93A9959u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn add_petrol_decal_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            groundLvl: 
        , 
        
        
            width: 
        , 
        
        
            transparency: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F5212C7AD880DF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F5212C7AD880DF8u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                groundLvl, 
                width, 
                transparency
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_petrol_decal_raw(
        x: , 
        y: , 
        z: , 
        groundLvl: , 
        width: , 
        transparency: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F5212C7AD880DF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F5212C7AD880DF8u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                groundLvl, 
                width, 
                transparency
        )
    }
}

/// ## Return value



pub fn _0x5b0316762afd4a64_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B0316762AFD4A64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B0316762AFD4A64u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5b0316762afd4a64_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B0316762AFD4A64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B0316762AFD4A64u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x2fcb133ca50a49eb_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FCB133CA50A49EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FCB133CA50A49EBu64;
        
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
pub fn _0x2fcb133ca50a49eb_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FCB133CA50A49EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FCB133CA50A49EBu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x949f397a288b28b3_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x949F397A288B28B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x949F397A288B28B3u64;
        
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
pub fn _0x949f397a288b28b3_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x949F397A288B28B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x949F397A288B28B3u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _start_networked_particle_fx_non_looped_on_entity_bone_safe(
        
        
            effectName: 
        , 
        
        
            entity: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            boneIndex: 
        , 
        
        
            scale: 
        , 
        
        
            axisX: 
        , 
        
        
            axisY: 
        , 
        
        
            axisZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02B1F2A72E0F5325u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02B1F2A72E0F5325u64;
        
        let result = invoke_raw!(
            hash,
                effectName, 
                entity, 
                offsetX, 
                offsetY, 
                offsetZ, 
                rotX, 
                rotY, 
                rotZ, 
                boneIndex, 
                scale, 
                axisX, 
                axisY, 
                axisZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _start_networked_particle_fx_non_looped_on_entity_bone_raw(
        effectName: , 
        entity: , 
        offsetX: , 
        offsetY: , 
        offsetZ: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        boneIndex: , 
        scale: , 
        axisX: , 
        axisY: , 
        axisZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02B1F2A72E0F5325u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02B1F2A72E0F5325u64;

        invoke_raw_typed!(
            hash,
                effectName, 
                entity, 
                offsetX, 
                offsetY, 
                offsetZ, 
                rotX, 
                rotY, 
                rotZ, 
                boneIndex, 
                scale, 
                axisX, 
                axisY, 
                axisZ
        )
    }
}

/// ## Parameters
*



pub fn delete_checkpoint_safe(
        
        
            checkpoint: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5ED37F54CD4D52Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5ED37F54CD4D52Eu64;
        
        let result = invoke_raw!(
            hash,
                checkpoint
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn delete_checkpoint_raw(
        checkpoint: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5ED37F54CD4D52Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5ED37F54CD4D52Eu64;

        invoke_raw_typed!(
            hash,
                checkpoint
        )
    }
}

/// ## Parameters
*



pub fn remove_particle_fx_in_range_safe(
        
        
            X: 
        , 
        
        
            Y: 
        , 
        
        
            Z: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD19FA1C6D657305u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD19FA1C6D657305u64;
        
        let result = invoke_raw!(
            hash,
                X, 
                Y, 
                Z, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_particle_fx_in_range_raw(
        X: , 
        Y: , 
        Z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD19FA1C6D657305u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD19FA1C6D657305u64;

        invoke_raw_typed!(
            hash,
                X, 
                Y, 
                Z, 
                radius
        )
    }
}

/// _0x5DEBD9C4DC995692 native function



pub fn _0x5debd9c4dc995692_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DEBD9C4DC995692u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DEBD9C4DC995692u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5debd9c4dc995692_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DEBD9C4DC995692u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DEBD9C4DC995692u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn start_particle_fx_looped_on_ped_bone_safe(
        
        
            effectName: 
        , 
        
        
            ped: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
        , 
        
        
            xRot: 
        , 
        
        
            yRot: 
        , 
        
        
            zRot: 
        , 
        
        
            boneIndex: 
        , 
        
        
            scale: 
        , 
        
        
            xAxis: 
        , 
        
        
            yAxis: 
        , 
        
        
            zAxis: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF28DA9F38CD1787Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF28DA9F38CD1787Cu64;
        
        let result = invoke_raw!(
            hash,
                effectName, 
                ped, 
                xOffset, 
                yOffset, 
                zOffset, 
                xRot, 
                yRot, 
                zRot, 
                boneIndex, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_particle_fx_looped_on_ped_bone_raw(
        effectName: , 
        ped: , 
        xOffset: , 
        yOffset: , 
        zOffset: , 
        xRot: , 
        yRot: , 
        zRot: , 
        boneIndex: , 
        scale: , 
        xAxis: , 
        yAxis: , 
        zAxis: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF28DA9F38CD1787Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF28DA9F38CD1787Cu64;

        invoke_raw_typed!(
            hash,
                effectName, 
                ped, 
                xOffset, 
                yOffset, 
                zOffset, 
                xRot, 
                yRot, 
                zRot, 
                boneIndex, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis
        )
    }
}

/// ## Parameters
*



pub fn remove_vehicle_crew_emblem_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2300034310557E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2300034310557E4u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_vehicle_crew_emblem_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2300034310557E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2300034310557E4u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _seethrough_set_fade_start_distance_safe(
        
        
            distance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA78DE25577300BA1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA78DE25577300BA1u64;
        
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
pub fn _seethrough_set_fade_start_distance_raw(
        distance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA78DE25577300BA1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA78DE25577300BA1u64;

        invoke_raw_typed!(
            hash,
                distance
        )
    }
}

/// ## Parameters
*



pub fn set_timecycle_modifier_strength_safe(
        
        
            strength: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82E7FFCD5B2326B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82E7FFCD5B2326B3u64;
        
        let result = invoke_raw!(
            hash,
                strength
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_timecycle_modifier_strength_raw(
        strength: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82E7FFCD5B2326B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82E7FFCD5B2326B3u64;

        invoke_raw_typed!(
            hash,
                strength
        )
    }
}

/// If true, this native will create purple explosions upon projectile impact, add comic-like PTFX when firing a weapon, create a sound on bullet impact and have its own "blood effect".

If the PTFX asset "scr_rcbarry2" is not requested using ([`RequestNamedPtfxAsset`](#_0xD821490579791273)) then this native



pub fn enable_clown_blood_vfx_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD821490579791273u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD821490579791273u64;
        
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
pub fn enable_clown_blood_vfx_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD821490579791273u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD821490579791273u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
SET_TV_???  
```



pub fn _0xd1c55b110e4df534_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1C55B110E4DF534u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1C55B110E4DF534u64;
        
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
pub fn _0xd1c55b110e4df534_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1C55B110E4DF534u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1C55B110E4DF534u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
It's called after 0xD3A10FC7FD8D98CD and 0xF1CEA8A4198D8E9A  
p0 was always "CELEBRATION_WINNER"  
```



pub fn _draw_showroom_safe(
        
        
            p0: 
        , 
        
        
            ped: 
        , 
        
        
            p2: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98C4FE6EC34154CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98C4FE6EC34154CAu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                ped, 
                p2, 
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
pub fn _draw_showroom_raw(
        p0: , 
        ped: , 
        p2: , 
        posX: , 
        posY: , 
        posZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98C4FE6EC34154CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98C4FE6EC34154CAu64;

        invoke_raw_typed!(
            hash,
                p0, 
                ped, 
                p2, 
                posX, 
                posY, 
                posZ
        )
    }
}

/// ```
Sets the checkpoint color.  
```



pub fn set_checkpoint_rgba_safe(
        
        
            checkpoint: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7167371E8AD747F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7167371E8AD747F7u64;
        
        let result = invoke_raw!(
            hash,
                checkpoint, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_checkpoint_rgba_raw(
        checkpoint: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7167371E8AD747F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7167371E8AD747F7u64;

        invoke_raw_typed!(
            hash,
                checkpoint, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ```
UI3DSCENE_*
```



pub fn _0x7a42b2e236e71415_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A42B2E236E71415u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A42B2E236E71415u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7a42b2e236e71415_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A42B2E236E71415u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A42B2E236E71415u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn draw_light_with_range_safe(
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            colorR: 
        , 
        
        
            colorG: 
        , 
        
        
            colorB: 
        , 
        
        
            range: 
        , 
        
        
            intensity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2A1B2771A01DBD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2A1B2771A01DBD4u64;
        
        let result = invoke_raw!(
            hash,
                posX, 
                posY, 
                posZ, 
                colorR, 
                colorG, 
                colorB, 
                range, 
                intensity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_light_with_range_raw(
        posX: , 
        posY: , 
        posZ: , 
        colorR: , 
        colorG: , 
        colorB: , 
        range: , 
        intensity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2A1B2771A01DBD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2A1B2771A01DBD4u64;

        invoke_raw_typed!(
            hash,
                posX, 
                posY, 
                posZ, 
                colorR, 
                colorG, 
                colorB, 
                range, 
                intensity
        )
    }
}

/// This function anchors script draws to a side of the safe zone. This needs to be called to make the interface
independent of the player's safe zone configuration.

These values are equivalent to `alignX` and `alignY` in `common:/data/ui/frontend.xml`, which can be used as a baseline
for default alignment.
  
Valid values for `horizontalAlign`, from original documentation:
*



pub fn set_script_gfx_align_safe(
        
        
            horizontalAlign: 
        , 
        
        
            verticalAlign: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8A850F20A067EB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8A850F20A067EB6u64;
        
        let result = invoke_raw!(
            hash,
                horizontalAlign, 
                verticalAlign
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_script_gfx_align_raw(
        horizontalAlign: , 
        verticalAlign: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8A850F20A067EB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8A850F20A067EB6u64;

        invoke_raw_typed!(
            hash,
                horizontalAlign, 
                verticalAlign
        )
    }
}

/// ## Parameters
*



pub fn set_particle_fx_shootout_boat_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96EF97DAEB89BEF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96EF97DAEB89BEF5u64;
        
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
pub fn set_particle_fx_shootout_boat_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96EF97DAEB89BEF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96EF97DAEB89BEF5u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x1bbc135a4d25edde_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1BBC135A4D25EDDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1BBC135A4D25EDDEu64;
        
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
pub fn _0x1bbc135a4d25edde_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1BBC135A4D25EDDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1BBC135A4D25EDDEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Loads the specified timecycle modifier. Modifiers are defined separately in another file (e.g. "timecycle_mods_1.xml")
Parameters:
modifierName - The modifier to load (e.g. "V_FIB_IT3", "scanline_cam", etc.)
```



pub fn set_timecycle_modifier_safe(
        
        
            modifierName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C933ABF17A1DF41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C933ABF17A1DF41u64;
        
        let result = invoke_raw!(
            hash,
                modifierName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_timecycle_modifier_raw(
        modifierName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C933ABF17A1DF41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C933ABF17A1DF41u64;

        invoke_raw_typed!(
            hash,
                modifierName
        )
    }
}

/// ## Parameters
*



pub fn remove_tcmodifier_override_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15E33297C3E8DC60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15E33297C3E8DC60u64;
        
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
pub fn remove_tcmodifier_override_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15E33297C3E8DC60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15E33297C3E8DC60u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ANIMPOSTFX_STOP_ALL native function



pub fn animpostfx_stop_all_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4EDDC19532BFB85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4EDDC19532BFB85u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn animpostfx_stop_all_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4EDDC19532BFB85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4EDDC19532BFB85u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Prevents gas / petrol decals (aka gas / petrol trails and puddles) to be ignited on fire during the frame in which the native is called.



pub fn set_disable_petrol_decals_igniting_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9454B5752C857DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9454B5752C857DCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_disable_petrol_decals_igniting_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9454B5752C857DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9454B5752C857DCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Returns current screen resolution.

```
NativeDB Introduced: v323
```



pub fn get_actual_screen_resolution_safe(
        
        
            x: 
        , 
        
        
            y: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x873C9F3104101DD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x873C9F3104101DD3u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_actual_screen_resolution_raw(
        x: , 
        y: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x873C9F3104101DD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x873C9F3104101DD3u64;

        invoke_raw_typed!(
            hash,
                x, 
                y
        )
    }
}

/// ```
Getter for 0xB3C641F3630BF6DA

GET_M*
```



pub fn _0xe59343e9e96529e7_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE59343E9E96529E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE59343E9E96529E7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xe59343e9e96529e7_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE59343E9E96529E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE59343E9E96529E7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Loads specified video sequence into the TV Channel
TV_Channel ranges from 0-2
VideoSequence can be any of the following:
"PL_STD_CNT" CNT Standard Channel
"PL_STD_WZL" Weazel Standard Channel
"PL_LO_CNT"
"PL_LO_WZL"
"PL_SP_WORKOUT"
"PL_SP_INV" - Jay Norris Assassination Mission Fail
"PL_SP_INV_EXP" - Jay Norris Assassination Mission Success
"PL_LO_RS" - Righteous Slaughter Ad
"PL_LO_RS_CUTSCENE" - Righteous Slaughter Cut-scene
"PL_SP_PLSH1_INTRO"
"PL_LES1_FAME_OR_SHAME"
"PL_STD_WZL_FOS_EP2"
"PL_MP_WEAZEL" - Weazel Logo on loop
"PL_MP_CCTV" - Generic CCTV loop
Restart:
0=video sequence continues as normal
1=sequence restarts from beginning every time that channel is selected
The above playlists work as intended, and are commonly used, but there are many more playlists, as seen in `tvplaylists.xml`. A pastebin below outlines all playlists, they will be surronded by the name tag I.E. (<Name>PL_STD_CNT</Name> = PL_STD_CNT).
https://pastebin.com/zUzGB6h7



pub fn set_tv_channel_playlist_safe(
        
        
            tvChannel: 
        , 
        
        
            playlistName: 
        , 
        
        
            restart: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7B38B8305F1FE8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7B38B8305F1FE8Bu64;
        
        let result = invoke_raw!(
            hash,
                tvChannel, 
                playlistName, 
                restart
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_tv_channel_playlist_raw(
        tvChannel: , 
        playlistName: , 
        restart: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7B38B8305F1FE8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7B38B8305F1FE8Bu64;

        invoke_raw_typed!(
            hash,
                tvChannel, 
                playlistName, 
                restart
        )
    }
}

/// ```
Calls the Scaleform function and passes the parameters as floats.  
The number of parameters passed to the function varies, so the end of the parameter list is represented by -1.0.  
```



pub fn call_scaleform_movie_method_with_number_safe(
        
        
            scaleform: 
        , 
        
        
            methodName: 
        , 
        
        
            param1: 
        , 
        
        
            param2: 
        , 
        
        
            param3: 
        , 
        
        
            param4: 
        , 
        
        
            param5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0837058AE2E4BEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0837058AE2E4BEEu64;
        
        let result = invoke_raw!(
            hash,
                scaleform, 
                methodName, 
                param1, 
                param2, 
                param3, 
                param4, 
                param5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn call_scaleform_movie_method_with_number_raw(
        scaleform: , 
        methodName: , 
        param1: , 
        param2: , 
        param3: , 
        param4: , 
        param5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0837058AE2E4BEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0837058AE2E4BEEu64;

        invoke_raw_typed!(
            hash,
                scaleform, 
                methodName, 
                param1, 
                param2, 
                param3, 
                param4, 
                param5
        )
    }
}

/// ```
Example:  
GRAPHICS::ADD_ENTITY_ICON(a_0, "MP_Arrow");  
I tried this and nothing happened...  
```



pub fn add_entity_icon_safe(
        
        
            entity: 
        , 
        
        
            icon: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CD43EEE12BF4DD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CD43EEE12BF4DD0u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                icon
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_entity_icon_raw(
        entity: , 
        icon: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CD43EEE12BF4DD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CD43EEE12BF4DD0u64;

        invoke_raw_typed!(
            hash,
                entity, 
                icon
        )
    }
}

/// ## Parameters
*



pub fn set_backfaceculling_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23BA6B0C2AD7B0D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23BA6B0C2AD7B0D3u64;
        
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
pub fn set_backfaceculling_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23BA6B0C2AD7B0D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23BA6B0C2AD7B0D3u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn remove_particle_fx_safe(
        
        
            ptfxHandle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC401503DFE8D53CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC401503DFE8D53CFu64;
        
        let result = invoke_raw!(
            hash,
                ptfxHandle, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_particle_fx_raw(
        ptfxHandle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC401503DFE8D53CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC401503DFE8D53CFu64;

        invoke_raw_typed!(
            hash,
                ptfxHandle, 
                p1
        )
    }
}

/// Removes any custom moon cycle overrides that have been configured with [ENABLE_MOON_CYCLE_OVERRIDE](#_0x2C328AF17210F009)



pub fn disable_moon_cycle_override_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BF72AD5B41AA739u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BF72AD5B41AA739u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_moon_cycle_override_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BF72AD5B41AA739u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BF72AD5B41AA739u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn golf_trail_set_enabled_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA51C4B86B71652AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA51C4B86B71652AEu64;
        
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
pub fn golf_trail_set_enabled_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA51C4B86B71652AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA51C4B86B71652AEu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _stop_bink_movie_safe(
        
        
            binkMovie: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63606A61DE68898Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63606A61DE68898Au64;
        
        let result = invoke_raw!(
            hash,
                binkMovie
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _stop_bink_movie_raw(
        binkMovie: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63606A61DE68898Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63606A61DE68898Au64;

        invoke_raw_typed!(
            hash,
                binkMovie
        )
    }
}

/// Used in pi_menu.c. Checks if there is a brief entry for specified value.
Values:
0 - Dialogue brief
1 - Help text brief
2 - Mission Objective brief



pub fn does_latest_brief_string_exist_safe(
        
        
            briefValue: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E657EF1099EDD65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E657EF1099EDD65u64;
        
        let result = invoke_raw!(
            hash,
                briefValue
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_latest_brief_string_exist_raw(
        briefValue: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E657EF1099EDD65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E657EF1099EDD65u64;

        invoke_raw_typed!(
            hash,
                briefValue
        )
    }
}

/// ## Parameters
*



pub fn seethrough_set_heatscale_safe(
        
        
            index: 
        , 
        
        
            heatScale: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7D0B00177485411u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7D0B00177485411u64;
        
        let result = invoke_raw!(
            hash,
                index, 
                heatScale
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn seethrough_set_heatscale_raw(
        index: , 
        heatScale: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7D0B00177485411u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7D0B00177485411u64;

        invoke_raw_typed!(
            hash,
                index, 
                heatScale
        )
    }
}

/// ## Parameters
*



pub fn _set_extra_timecycle_modifier_safe(
        
        
            modifierName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5096FD9CCB49056Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5096FD9CCB49056Du64;
        
        let result = invoke_raw!(
            hash,
                modifierName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_extra_timecycle_modifier_raw(
        modifierName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5096FD9CCB49056Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5096FD9CCB49056Du64;

        invoke_raw_typed!(
            hash,
                modifierName
        )
    }
}

/// ```
FORCE_*
```



pub fn _0x9b079e5221d984d3_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B079E5221D984D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B079E5221D984D3u64;
        
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
pub fn _0x9b079e5221d984d3_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B079E5221D984D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B079E5221D984D3u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x61f95e5bb3e0a8c6_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61F95E5BB3E0A8C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61F95E5BB3E0A8C6u64;
        
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
pub fn _0x61f95e5bb3e0a8c6_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61F95E5BB3E0A8C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61F95E5BB3E0A8C6u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
TOGGLE_*
```



pub fn _0xe63d7c6eececb66b_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE63D7C6EECECB66Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE63D7C6EECECB66Bu64;
        
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
pub fn _0xe63d7c6eececb66b_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE63D7C6EECECB66Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE63D7C6EECECB66Bu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0xf78b803082d4386f_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF78B803082D4386Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF78B803082D4386Fu64;
        
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
pub fn _0xf78b803082d4386f_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF78B803082D4386Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF78B803082D4386Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Unknown. Called after creating a checkpoint (type: 51) in the creators.  
```



pub fn _0x615d3925e87a3b26_safe(
        
        
            checkpoint: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x615D3925E87A3B26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x615D3925E87A3B26u64;
        
        let result = invoke_raw!(
            hash,
                checkpoint
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x615d3925e87a3b26_raw(
        checkpoint: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x615D3925E87A3B26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x615D3925E87A3B26u64;

        invoke_raw_typed!(
            hash,
                checkpoint
        )
    }
}

/// ## Parameters
*



pub fn _seethrough_set_hi_light_noise_safe(
        
        
            noise: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1636D7FC127B10D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1636D7FC127B10D2u64;
        
        let result = invoke_raw!(
            hash,
                noise
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _seethrough_set_hi_light_noise_raw(
        noise: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1636D7FC127B10D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1636D7FC127B10D2u64;

        invoke_raw_typed!(
            hash,
                noise
        )
    }
}

/// ```
NativeDB Introduced: v2699
```



pub fn _set_particle_fx_non_looped_emitter_scale_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            scale: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E2E01C00837D26Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E2E01C00837D26Eu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                scale
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_particle_fx_non_looped_emitter_scale_raw(
        p0: , 
        p1: , 
        scale: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E2E01C00837D26Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E2E01C00837D26Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                scale
        )
    }
}

/// This native enables/disables the gold putting grid display (https://i.imgur.com/TC6cku6.png).
This requires these two natives to be called as well to configure the grid: [`TERRAINGRID_SET_PARAMS`](#_0x1C4FC5752BCD8E48) and [`TERRAINGRID_SET_COLOURS`](#_0x5CE62918F8D703C7).



pub fn terraingrid_activate_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA356990E161C9E65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA356990E161C9E65u64;
        
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
pub fn terraingrid_activate_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA356990E161C9E65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA356990E161C9E65u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn cascade_shadows_set_cascade_bounds_scale_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F0F3F56635809EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F0F3F56635809EFu64;
        
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
pub fn cascade_shadows_set_cascade_bounds_scale_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F0F3F56635809EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F0F3F56635809EFu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xca465d9cc0d231ba_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA465D9CC0D231BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA465D9CC0D231BAu64;
        
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
pub fn _0xca465d9cc0d231ba_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA465D9CC0D231BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA465D9CC0D231BAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// This native is used along with these two natives: [`TERRAINGRID_ACTIVATE`](#_0xA356990E161C9E65) and [`TERRAINGRID_SET_PARAMS`](#_0x1C4FC5752BCD8E48). 
This native sets the colors for the golf putting grid. the 'min...' values are for the lower areas that the grid covers, the 'max...' values are for the higher areas that the grid covers, all remaining values are for the 'normal' ground height.
All those natives combined they will output something like this: https://i.imgur.com/TC6cku6.png

Old description:
Only called in golf and golf_mp  
parameters used are   
GRAPHICS::_0x5CE62918F8D703C7(255, 0, 0, 64, 255, 255, 255, 5, 255, 255, 0, 64);



pub fn terraingrid_set_colours_safe(
        
        
            lowR: 
        , 
        
        
            lowG: 
        , 
        
        
            lowB: 
        , 
        
        
            lowAlpha: 
        , 
        
        
            R: 
        , 
        
        
            G: 
        , 
        
        
            B: 
        , 
        
        
            Alpha: 
        , 
        
        
            highR: 
        , 
        
        
            highG: 
        , 
        
        
            highB: 
        , 
        
        
            highAlpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CE62918F8D703C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CE62918F8D703C7u64;
        
        let result = invoke_raw!(
            hash,
                lowR, 
                lowG, 
                lowB, 
                lowAlpha, 
                R, 
                G, 
                B, 
                Alpha, 
                highR, 
                highG, 
                highB, 
                highAlpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn terraingrid_set_colours_raw(
        lowR: , 
        lowG: , 
        lowB: , 
        lowAlpha: , 
        R: , 
        G: , 
        B: , 
        Alpha: , 
        highR: , 
        highG: , 
        highB: , 
        highAlpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CE62918F8D703C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CE62918F8D703C7u64;

        invoke_raw_typed!(
            hash,
                lowR, 
                lowG, 
                lowB, 
                lowAlpha, 
                R, 
                G, 
                B, 
                Alpha, 
                highR, 
                highG, 
                highB, 
                highAlpha
        )
    }
}

/// _CASCADE_SHADOWS_CLEAR_SHADOW_SAMPLE_TYPE native function



pub fn _cascade_shadows_clear_shadow_sample_type_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27CB772218215325u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27CB772218215325u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _cascade_shadows_clear_shadow_sample_type_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27CB772218215325u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27CB772218215325u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Only works on some fx's, while on others it might SEEM to work "properly", but the colors can be "strange" or even completly different from what you've expected. Reason for this is that those fx's might already have colors "baked into them" which then start to act as a "mixing palette", resulting in a different color than expected. A hypothetical example of this would be if the fx itself is already full (bright) red (RGB: 1.0, 0.0, 0.0) and you then set the color to (bright) green (RGB: 0.0, 1.0, 0.0), that it MIGHT result in Yellow (RGB: 1.0, 1.0, 0.0).

This doc previously stated that the set color is



pub fn set_particle_fx_non_looped_colour_safe(
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26143A59EF48B262u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26143A59EF48B262u64;
        
        let result = invoke_raw!(
            hash,
                r, 
                g, 
                b
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_particle_fx_non_looped_colour_raw(
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26143A59EF48B262u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26143A59EF48B262u64;

        invoke_raw_typed!(
            hash,
                r, 
                g, 
                b
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0xadd6627c4d325458_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADD6627C4D325458u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADD6627C4D325458u64;
        
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
pub fn _0xadd6627c4d325458_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADD6627C4D325458u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADD6627C4D325458u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Same as [REQUEST_SCALEFORM_MOVIE](#_0x11FE353CF9733E6F), except it seems to fix stretched scaleforms on ultrawide.



pub fn request_scaleform_movie_instance_safe(
        
        
            scaleformName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC514489CFB8AF806u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC514489CFB8AF806u64;
        
        let result = invoke_raw!(
            hash,
                scaleformName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_scaleform_movie_instance_raw(
        scaleformName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC514489CFB8AF806u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC514489CFB8AF806u64;

        invoke_raw_typed!(
            hash,
                scaleformName
        )
    }
}

/// ## Parameters
*



pub fn set_particle_fx_bullet_impact_scale_safe(
        
        
            scale: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27E32866E9A5C416u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27E32866E9A5C416u64;
        
        let result = invoke_raw!(
            hash,
                scale
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_particle_fx_bullet_impact_scale_raw(
        scale: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27E32866E9A5C416u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27E32866E9A5C416u64;

        invoke_raw_typed!(
            hash,
                scale
        )
    }
}

/// ## Parameters
*



pub fn request_scaleform_script_hud_movie_safe(
        
        
            hudComponent: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9304881D6F6537EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9304881D6F6537EAu64;
        
        let result = invoke_raw!(
            hash,
                hudComponent
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_scaleform_script_hud_movie_raw(
        hudComponent: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9304881D6F6537EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9304881D6F6537EAu64;

        invoke_raw_typed!(
            hash,
                hudComponent
        )
    }
}

/// Enable a custom moon cycle, allowing control of which lunar phase the moon is in.

Valid values are from `0.0` to `1.0`, with `0.5` representing a full moon.

| Value |   Lunar Phase   |
| :---: | :-------------: |
| `0.1` | Waxing Crescent |
| `0.2` |  First Quarter  |
| `0.3` | Waxing Gibbous  |
| `0.5` |    Full Moon    |
| `0.7` | Waning Gibbous  |
| `0.8` |  Third Quarter  |
| `0.9` | Waning Crescent |

The moon phase can be disabled with [DISABLE_MOON_CYCLE_OVERRIDE](#_0x2BF72AD5B41AA739)



pub fn enable_moon_cycle_override_safe(
        
        
            phase: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C328AF17210F009u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C328AF17210F009u64;
        
        let result = invoke_raw!(
            hash,
                phase
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn enable_moon_cycle_override_raw(
        phase: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C328AF17210F009u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C328AF17210F009u64;

        invoke_raw_typed!(
            hash,
                phase
        )
    }
}

/// ## Return value



pub fn _0xbcedb009461da156_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCEDB009461DA156u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCEDB009461DA156u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xbcedb009461da156_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCEDB009461DA156u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCEDB009461DA156u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x2c42340f916c5930_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C42340F916C5930u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C42340F916C5930u64;
        
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
pub fn _0x2c42340f916c5930_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C42340F916C5930u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C42340F916C5930u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xa46b73faa3460ae1_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA46B73FAA3460AE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA46B73FAA3460AE1u64;
        
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
pub fn _0xa46b73faa3460ae1_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA46B73FAA3460AE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA46B73FAA3460AE1u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
network fx  
```

```
NativeDB Added Parameter 14: Any p13
NativeDB Added Parameter 15: Any p14
NativeDB Added Parameter 16: Any p15
NativeDB Added Parameter 17: Any p16
```



pub fn start_networked_particle_fx_looped_on_entity_bone_safe(
        
        
            effectName: 
        , 
        
        
            entity: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
        , 
        
        
            xRot: 
        , 
        
        
            yRot: 
        , 
        
        
            zRot: 
        , 
        
        
            boneIndex: 
        , 
        
        
            scale: 
        , 
        
        
            xAxis: 
        , 
        
        
            yAxis: 
        , 
        
        
            zAxis: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDE23F30CC5A0F03u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDE23F30CC5A0F03u64;
        
        let result = invoke_raw!(
            hash,
                effectName, 
                entity, 
                xOffset, 
                yOffset, 
                zOffset, 
                xRot, 
                yRot, 
                zRot, 
                boneIndex, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_networked_particle_fx_looped_on_entity_bone_raw(
        effectName: , 
        entity: , 
        xOffset: , 
        yOffset: , 
        zOffset: , 
        xRot: , 
        yRot: , 
        zRot: , 
        boneIndex: , 
        scale: , 
        xAxis: , 
        yAxis: , 
        zAxis: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDE23F30CC5A0F03u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDE23F30CC5A0F03u64;

        invoke_raw_typed!(
            hash,
                effectName, 
                entity, 
                xOffset, 
                yOffset, 
                zOffset, 
                xRot, 
                yRot, 
                zRot, 
                boneIndex, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis
        )
    }
}

/// ## Parameters
*



pub fn set_particle_fx_override_safe(
        
        
            oldAsset: 
        , 
        
        
            newAsset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA1E2D93F6F75ED9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA1E2D93F6F75ED9u64;
        
        let result = invoke_raw!(
            hash,
                oldAsset, 
                newAsset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_particle_fx_override_raw(
        oldAsset: , 
        newAsset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA1E2D93F6F75ED9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA1E2D93F6F75ED9u64;

        invoke_raw_typed!(
            hash,
                oldAsset, 
                newAsset
        )
    }
}

/// ## Parameters
*



pub fn set_tv_channel_playlist_at_hour_safe(
        
        
            tvChannel: 
        , 
        
        
            playlistName: 
        , 
        
        
            hour: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2201C576FACAEBE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2201C576FACAEBE8u64;
        
        let result = invoke_raw!(
            hash,
                tvChannel, 
                playlistName, 
                hour
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_tv_channel_playlist_at_hour_raw(
        tvChannel: , 
        playlistName: , 
        hour: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2201C576FACAEBE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2201C576FACAEBE8u64;

        invoke_raw_typed!(
            hash,
                tvChannel, 
                playlistName, 
                hour
        )
    }
}

/// ## Parameters
*



pub fn is_tracked_point_visible_safe(
        
        
            point: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC45CCDAAC9221CA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC45CCDAAC9221CA8u64;
        
        let result = invoke_raw!(
            hash,
                point
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_tracked_point_visible_raw(
        point: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC45CCDAAC9221CA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC45CCDAAC9221CA8u64;

        invoke_raw_typed!(
            hash,
                point
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0xaae9be70ec7c69ab_safe(
        
        
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
        let hash = 0xAAE9BE70EC7C69ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAE9BE70EC7C69ABu64;
        
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
pub fn _0xaae9be70ec7c69ab_raw(
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
        let hash = 0xAAE9BE70EC7C69ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAE9BE70EC7C69ABu64;

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

/// ## Parameters
*



pub fn set_tv_channel_safe(
        
        
            channel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAABBB23EB6E484Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAABBB23EB6E484Eu64;
        
        let result = invoke_raw!(
            hash,
                channel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_tv_channel_raw(
        channel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAABBB23EB6E484Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAABBB23EB6E484Eu64;

        invoke_raw_typed!(
            hash,
                channel
        )
    }
}

/// ## Parameters
*



pub fn get_is_petrol_decal_in_range_safe(
        
        
            xCoord: 
        , 
        
        
            yCoord: 
        , 
        
        
            zCoord: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F09F7976C512404u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F09F7976C512404u64;
        
        let result = invoke_raw!(
            hash,
                xCoord, 
                yCoord, 
                zCoord, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_is_petrol_decal_in_range_raw(
        xCoord: , 
        yCoord: , 
        zCoord: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F09F7976C512404u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F09F7976C512404u64;

        invoke_raw_typed!(
            hash,
                xCoord, 
                yCoord, 
                zCoord, 
                radius
        )
    }
}

/// ## Parameters
*



pub fn set_particle_fx_cam_inside_vehicle_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEC4047028426510u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEC4047028426510u64;
        
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
pub fn set_particle_fx_cam_inside_vehicle_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEC4047028426510u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEC4047028426510u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn has_scaleform_container_movie_loaded_into_parent_safe(
        
        
            scaleformHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8217150E1217EBFDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8217150E1217EBFDu64;
        
        let result = invoke_raw!(
            hash,
                scaleformHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_scaleform_container_movie_loaded_into_parent_raw(
        scaleformHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8217150E1217EBFDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8217150E1217EBFDu64;

        invoke_raw_typed!(
            hash,
                scaleformHandle
        )
    }
}

/// ```
NOTE: Debugging functions are not present in the retail version of the game.  
```



pub fn draw_debug_text_safe(
        
        
            text: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3903E216620488E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3903E216620488E8u64;
        
        let result = invoke_raw!(
            hash,
                text, 
                x, 
                y, 
                z, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_debug_text_raw(
        text: , 
        x: , 
        y: , 
        z: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3903E216620488E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3903E216620488E8u64;

        invoke_raw_typed!(
            hash,
                text, 
                x, 
                y, 
                z, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ## Parameters
*



pub fn wash_decals_in_range_safe(
        
        
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
        let hash = 0x9C30613D50A6ADEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C30613D50A6ADEFu64;
        
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
pub fn wash_decals_in_range_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C30613D50A6ADEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C30613D50A6ADEFu64;

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

/// ## Parameters
*



pub fn _0x54e22ea2c1956a8d_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54E22EA2C1956A8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54E22EA2C1956A8Du64;
        
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
pub fn _0x54e22ea2c1956a8d_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54E22EA2C1956A8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54E22EA2C1956A8Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Only one match in the scripts:
GRAPHICS::PRESET_INTERIOR_AMBIENT_CACHE("int_carrier_hanger");
```



pub fn preset_interior_ambient_cache_safe(
        
        
            timecycleModifierName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7021272EB0A451Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7021272EB0A451Eu64;
        
        let result = invoke_raw!(
            hash,
                timecycleModifierName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn preset_interior_ambient_cache_raw(
        timecycleModifierName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7021272EB0A451Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7021272EB0A451Eu64;

        invoke_raw_typed!(
            hash,
                timecycleModifierName
        )
    }
}

/// ```
12 matches across 4 scripts. All 4 scripts were job creators.
type ranged from 0 - 2.
p4 was always 0.2f. Likely scale.
assuming p5 - p8 is RGBA, the graphic is always yellow (255, 255, 0, 255).
Tested but noticed nothing.
```



pub fn golf_trail_set_fixed_control_point_safe(
        
        
            type: 
        , 
        
        
            xPos: 
        , 
        
        
            yPos: 
        , 
        
        
            zPos: 
        , 
        
        
            p4: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1BB03742917A5D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1BB03742917A5D6u64;
        
        let result = invoke_raw!(
            hash,
                type, 
                xPos, 
                yPos, 
                zPos, 
                p4, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn golf_trail_set_fixed_control_point_raw(
        type: , 
        xPos: , 
        yPos: , 
        zPos: , 
        p4: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1BB03742917A5D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1BB03742917A5D6u64;

        invoke_raw_typed!(
            hash,
                type, 
                xPos, 
                yPos, 
                zPos, 
                p4, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// Must be called each frame, will play at specified position on screen when called with [`_PLAY_BINK_MOVIE`](#_0x70D2CC8A542A973C)



pub fn _draw_bink_movie_safe(
        
        
            binkMovie: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            scaleX: 
        , 
        
        
            scaleY: 
        , 
        
        
            rotation: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        , 
        
        
            a: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7118E83EEB9F7238u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7118E83EEB9F7238u64;
        
        let result = invoke_raw!(
            hash,
                binkMovie, 
                posX, 
                posY, 
                scaleX, 
                scaleY, 
                rotation, 
                r, 
                g, 
                b, 
                a
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _draw_bink_movie_raw(
        binkMovie: , 
        posX: , 
        posY: , 
        scaleX: , 
        scaleY: , 
        rotation: , 
        r: , 
        g: , 
        b: , 
        a: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7118E83EEB9F7238u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7118E83EEB9F7238u64;

        invoke_raw_typed!(
            hash,
                binkMovie, 
                posX, 
                posY, 
                scaleX, 
                scaleY, 
                rotation, 
                r, 
                g, 
                b, 
                a
        )
    }
}

/// ## Return value



pub fn get_usingnightvision_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2202A3F42C8E5F79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2202A3F42C8E5F79u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_usingnightvision_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2202A3F42C8E5F79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2202A3F42C8E5F79u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Does not affect weapons, particles, fire/explosions, flashlights or the sun.

When set to true, all emissive textures (including ped components that have light effects), street lights, building lights, vehicle lights, etc will all be turned off.

Used in Humane Labs Heist for EMP.



pub fn set_artificial_lights_state_safe(
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1268615ACE24D504u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1268615ACE24D504u64;
        
        let result = invoke_raw!(
            hash,
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_artificial_lights_state_raw(
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1268615ACE24D504u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1268615ACE24D504u64;

        invoke_raw_typed!(
            hash,
                state
        )
    }
}

/// ## Parameters
*



pub fn _0x25fc3e33a31ad0c9_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25FC3E33A31AD0C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25FC3E33A31AD0C9u64;
        
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
pub fn _0x25fc3e33a31ad0c9_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25FC3E33A31AD0C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25FC3E33A31AD0C9u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Might be more appropriate in AUDIO?  
```



pub fn attach_tv_audio_to_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x845BAD77CC770633u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x845BAD77CC770633u64;
        
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
pub fn attach_tv_audio_to_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x845BAD77CC770633u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x845BAD77CC770633u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
Overriding ped badge texture to a passed texture. It's synced between players (even custom textures!), don't forget to request used dict on *all* clients to make it sync properly. Can be removed by passing empty strings.
```



pub fn _override_ped_badge_texture_safe(
        
        
            ped: 
        , 
        
        
            txd: 
        , 
        
        
            txn: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95EB5E34F821BABEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95EB5E34F821BABEu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                txd, 
                txn
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _override_ped_badge_texture_raw(
        ped: , 
        txd: , 
        txn: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95EB5E34F821BABEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95EB5E34F821BABEu64;

        invoke_raw_typed!(
            hash,
                ped, 
                txd, 
                txn
        )
    }
}

/// ## Return value



pub fn begin_take_high_quality_photo_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA67C35C56EB1BD9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA67C35C56EB1BD9Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_take_high_quality_photo_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA67C35C56EB1BD9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA67C35C56EB1BD9Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Sets the checkpoint icon color.
```



pub fn set_checkpoint_rgba2_safe(
        
        
            checkpoint: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9EA40907C680580u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9EA40907C680580u64;
        
        let result = invoke_raw!(
            hash,
                checkpoint, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_checkpoint_rgba2_raw(
        checkpoint: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9EA40907C680580u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9EA40907C680580u64;

        invoke_raw_typed!(
            hash,
                checkpoint, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ## Parameters
*



pub fn has_streamed_texture_dict_loaded_safe(
        
        
            textureDict: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0145F696AAAAD2E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0145F696AAAAD2E4u64;
        
        let result = invoke_raw!(
            hash,
                textureDict
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_streamed_texture_dict_loaded_raw(
        textureDict: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0145F696AAAAD2E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0145F696AAAAD2E4u64;

        invoke_raw_typed!(
            hash,
                textureDict
        )
    }
}

/// ```
Sets the cylinder height of the checkpoint.  
Parameters:  
* nearHeight - The height of the checkpoint when inside of the radius.  
* farHeight - The height of the checkpoint when outside of the radius.  
* radius - The radius of the checkpoint.  
```



pub fn set_checkpoint_cylinder_height_safe(
        
        
            checkpoint: 
        , 
        
        
            nearHeight: 
        , 
        
        
            farHeight: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2707AAE9D9297D89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2707AAE9D9297D89u64;
        
        let result = invoke_raw!(
            hash,
                checkpoint, 
                nearHeight, 
                farHeight, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_checkpoint_cylinder_height_raw(
        checkpoint: , 
        nearHeight: , 
        farHeight: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2707AAE9D9297D89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2707AAE9D9297D89u64;

        invoke_raw_typed!(
            hash,
                checkpoint, 
                nearHeight, 
                farHeight, 
                radius
        )
    }
}

/// ```
GOLF_TRAIL_SET_*
```



pub fn _0xc0416b061f2b7e5e_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0416B061F2B7E5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0416B061F2B7E5Eu64;
        
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
pub fn _0xc0416b061f2b7e5e_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0416B061F2B7E5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0416B061F2B7E5Eu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn end_scaleform_movie_method_return_value_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC50AA39A577AF886u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC50AA39A577AF886u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_scaleform_movie_method_return_value_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC50AA39A577AF886u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC50AA39A577AF886u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Used in arcade games and Beam hack minigame in Doomsday Heist. For example, [Penetrator Arcade Game](https://streamable.com/8igrzw)

NativeDB Introduced: v1290
```



pub fn _0x2d3b147afad49de0_safe(
        
        
            textureDict: 
        , 
        
        
            textureName: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            width: 
        , 
        
        
            height: 
        , 
        
        
            p6: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        , 
        
        
            p11: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D3B147AFAD49DE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D3B147AFAD49DE0u64;
        
        let result = invoke_raw!(
            hash,
                textureDict, 
                textureName, 
                x, 
                y, 
                width, 
                height, 
                p6, 
                red, 
                green, 
                blue, 
                alpha, 
                p11
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2d3b147afad49de0_raw(
        textureDict: , 
        textureName: , 
        x: , 
        y: , 
        width: , 
        height: , 
        p6: , 
        red: , 
        green: , 
        blue: , 
        alpha: , 
        p11: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D3B147AFAD49DE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D3B147AFAD49DE0u64;

        invoke_raw_typed!(
            hash,
                textureDict, 
                textureName, 
                x, 
                y, 
                width, 
                height, 
                p6, 
                red, 
                green, 
                blue, 
                alpha, 
                p11
        )
    }
}

/// ```
GRAPHICS::START_PARTICLE_FX_NON_LOOPED_ON_PED_BONE("scr_sh_bong_smoke", PLAYER::PLAYER_PED_ID(), -0.025f, 0.13f, 0f, 0f, 0f, 0f, 31086, 0x3F800000, 0, 0, 0);  
Axis - Invert Axis Flags  
list: pastebin.com/N9unUFWY  
```



pub fn start_particle_fx_non_looped_on_ped_bone_safe(
        
        
            effectName: 
        , 
        
        
            ped: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            boneIndex: 
        , 
        
        
            scale: 
        , 
        
        
            axisX: 
        , 
        
        
            axisY: 
        , 
        
        
            axisZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E7E72961BA18619u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E7E72961BA18619u64;
        
        let result = invoke_raw!(
            hash,
                effectName, 
                ped, 
                offsetX, 
                offsetY, 
                offsetZ, 
                rotX, 
                rotY, 
                rotZ, 
                boneIndex, 
                scale, 
                axisX, 
                axisY, 
                axisZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_particle_fx_non_looped_on_ped_bone_raw(
        effectName: , 
        ped: , 
        offsetX: , 
        offsetY: , 
        offsetZ: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        boneIndex: , 
        scale: , 
        axisX: , 
        axisY: , 
        axisZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E7E72961BA18619u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E7E72961BA18619u64;

        invoke_raw_typed!(
            hash,
                effectName, 
                ped, 
                offsetX, 
                offsetY, 
                offsetZ, 
                rotX, 
                rotY, 
                rotZ, 
                boneIndex, 
                scale, 
                axisX, 
                axisY, 
                axisZ
        )
    }
}

/// ```
This function is hard-coded to always return 0.  
```



pub fn _0xbe197eaa669238f4_safe(
        
        
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
        let hash = 0xBE197EAA669238F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE197EAA669238F4u64;
        
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
pub fn _0xbe197eaa669238f4_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE197EAA669238F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE197EAA669238F4u64;

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



pub fn wash_decals_from_vehicle_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B712761429DBC14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B712761429DBC14u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn wash_decals_from_vehicle_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B712761429DBC14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B712761429DBC14u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
Pushes a float for the Scaleform function onto the stack.  
```



pub fn scaleform_movie_method_add_param_float_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD69736AAE04DB51Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD69736AAE04DB51Au64;
        
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
pub fn scaleform_movie_method_add_param_float_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD69736AAE04DB51Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD69736AAE04DB51Au64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ```
This function can requests texture dictonaries from following RPFs:
scaleform_generic.rpf
scaleform_minigames.rpf
scaleform_minimap.rpf
scaleform_web.rpf
last param isnt a toggle
```



pub fn request_streamed_texture_dict_safe(
        
        
            textureDict: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFA2EF8E04127DD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFA2EF8E04127DD5u64;
        
        let result = invoke_raw!(
            hash,
                textureDict, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_streamed_texture_dict_raw(
        textureDict: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFA2EF8E04127DD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFA2EF8E04127DD5u64;

        invoke_raw_typed!(
            hash,
                textureDict, 
                p1
        )
    }
}

/// This native draws a box between two vectors in the game world. It is typically used for visualizing boundaries or areas of interest. The color of the box is specified by the red, green, and blue parameters, with alpha determining its opacity. This native should be called every frame for continuous rendering.

```
NativeDB Introduced: v323
```



pub fn draw_box_safe(
        
        
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
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3A9971CADAC7252u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3A9971CADAC7252u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_box_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3A9971CADAC7252u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3A9971CADAC7252u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ## Parameters
*



pub fn remove_decals_from_object_safe(
        
        
            obj: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCF71CBDDF5B6CB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCF71CBDDF5B6CB9u64;
        
        let result = invoke_raw!(
            hash,
                obj
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_decals_from_object_raw(
        obj: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCF71CBDDF5B6CB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCF71CBDDF5B6CB9u64;

        invoke_raw_typed!(
            hash,
                obj
        )
    }
}

/// Used to get a return value from a scaleform function. Returns a string in the same way GET_SCALEFORM_MOVIE_METHOD_RETURN_VALUE_INT returns an int.



pub fn get_scaleform_movie_method_return_value_string_safe(
        
        
            method_return: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1E258829A885245u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1E258829A885245u64;
        
        let result = invoke_raw!(
            hash,
                method_return
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_scaleform_movie_method_return_value_string_raw(
        method_return: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1E258829A885245u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1E258829A885245u64;

        invoke_raw_typed!(
            hash,
                method_return
        )
    }
}

/// ## Return value



pub fn get_tv_volume_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2170813D3DD8661Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2170813D3DD8661Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_tv_volume_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2170813D3DD8661Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2170813D3DD8661Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Creates a motion-blur sort of effect, this native does not seem to work, however by using the [`ANIMPOSTFX_PLAY`](#_0x2206BF9A37B7F724) native with `"DrugsMichaelAliensFight"` as the effect parameter, you should be able to get the effect.


This native does not seem to work, however by using the [ANIMPOSTFX_PLAY](#_0x2206BF9A37B7F724) native with "DrugsMichaelAliensFight" as the effect parameter, you should be able to get the effect.



pub fn enable_alien_blood_vfx_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DCE1F0F78260875u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DCE1F0F78260875u64;
        
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
pub fn enable_alien_blood_vfx_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DCE1F0F78260875u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DCE1F0F78260875u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn clear_tv_channel_playlist_safe(
        
        
            tvChannel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBEB3D46BB7F043C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBEB3D46BB7F043C0u64;
        
        let result = invoke_raw!(
            hash,
                tvChannel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_tv_channel_playlist_raw(
        tvChannel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBEB3D46BB7F043C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBEB3D46BB7F043C0u64;

        invoke_raw_typed!(
            hash,
                tvChannel
        )
    }
}

/// ```
Gets the scale of safe zone. if the safe zone size scale is max, it will return 1.0.  
```



pub fn get_safe_zone_size_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAF107B6BB2C97F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAF107B6BB2C97F0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_safe_zone_size_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAF107B6BB2C97F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAF107B6BB2C97F0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _seethrough_set_noise_amount_max_safe(
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEBFBFDFB66039DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEBFBFDFB66039DEu64;
        
        let result = invoke_raw!(
            hash,
                amount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _seethrough_set_noise_amount_max_raw(
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEBFBFDFB66039DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEBFBFDFB66039DEu64;

        invoke_raw_typed!(
            hash,
                amount
        )
    }
}

/// ```
Possible values:
"CSM_ST_POINT"
"CSM_ST_LINEAR"
"CSM_ST_TWOTAP"
"CSM_ST_BOX3x3"
"CSM_ST_BOX4x4"
"CSM_ST_DITHER2_LINEAR"
"CSM_ST_CUBIC"
"CSM_ST_DITHER4"
"CSM_ST_DITHER16"
"CSM_ST_SOFT16"
"CSM_ST_DITHER16_RPDB"
"CSM_ST_POISSON16_RPDB_GNORM"
"CSM_ST_HIGHRES_BOX4x4"
"CSM_ST_CLOUDS_SIMPLE"
"CSM_ST_CLOUDS_LINEAR"
"CSM_ST_CLOUDS_TWOTAP"
"CSM_ST_CLOUDS_BOX3x3"
"CSM_ST_CLOUDS_BOX4x4"
"CSM_ST_CLOUDS_DITHER2_LINEAR"
"CSM_ST_CLOUDS_SOFT16"
"CSM_ST_CLOUDS_DITHER16_RPDB"
"CSM_ST_CLOUDS_POISSON16_RPDB_GNORM"
```



pub fn cascade_shadows_set_shadow_sample_type_safe(
        
        
            type: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB11D94BC55F41932u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB11D94BC55F41932u64;
        
        let result = invoke_raw!(
            hash,
                type
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn cascade_shadows_set_shadow_sample_type_raw(
        type: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB11D94BC55F41932u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB11D94BC55F41932u64;

        invoke_raw_typed!(
            hash,
                type
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_crew_emblem_request_state_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE26117A5841B2FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE26117A5841B2FFu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_crew_emblem_request_state_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE26117A5841B2FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE26117A5841B2FFu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// CLEAR_TIMECYCLE_MODIFIER native function



pub fn clear_timecycle_modifier_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F07E7745A236711u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F07E7745A236711u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_timecycle_modifier_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F07E7745A236711u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F07E7745A236711u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// See [`ANIMPOSTFX_PLAY`](#_0x2206BF9A37B7F724).



pub fn animpostfx_is_running_safe(
        
        
            effectName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36AD3E690DA5ACEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36AD3E690DA5ACEBu64;
        
        let result = invoke_raw!(
            hash,
                effectName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn animpostfx_is_running_raw(
        effectName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36AD3E690DA5ACEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36AD3E690DA5ACEBu64;

        invoke_raw_typed!(
            hash,
                effectName
        )
    }
}

/// In percentage: 0.0 - 100.0

```
NativeDB Introduced: v1734
```



pub fn _get_bink_movie_time_safe(
        
        
            binkMovie: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E17DDD6B9D5BF29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E17DDD6B9D5BF29u64;
        
        let result = invoke_raw!(
            hash,
                binkMovie
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_bink_movie_time_raw(
        binkMovie: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E17DDD6B9D5BF29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E17DDD6B9D5BF29u64;

        invoke_raw_typed!(
            hash,
                binkMovie
        )
    }
}

/// ## Return value



pub fn get_tv_channel_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC1E275A90D39995u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC1E275A90D39995u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_tv_channel_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC1E275A90D39995u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC1E275A90D39995u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn has_scaleform_movie_loaded_safe(
        
        
            scaleformHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85F01B8D5B90570Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85F01B8D5B90570Eu64;
        
        let result = invoke_raw!(
            hash,
                scaleformHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_scaleform_movie_loaded_raw(
        scaleformHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85F01B8D5B90570Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85F01B8D5B90570Eu64;

        invoke_raw_typed!(
            hash,
                scaleformHandle
        )
    }
}

/// ## Parameters
*



pub fn _0xe2892e7e55d7073a_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2892E7E55D7073Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2892E7E55D7073Au64;
        
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
pub fn _0xe2892e7e55d7073a_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2892E7E55D7073Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2892E7E55D7073Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn request_scaleform_movie_safe(
        
        
            scaleformName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11FE353CF9733E6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11FE353CF9733E6Fu64;
        
        let result = invoke_raw!(
            hash,
                scaleformName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_scaleform_movie_raw(
        scaleformName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11FE353CF9733E6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11FE353CF9733E6Fu64;

        invoke_raw_typed!(
            hash,
                scaleformName
        )
    }
}

/// ```
Only use for this in the PC scripts is:
if (GRAPHICS::GET_TIMECYCLE_MODIFIER_INDEX() != -1)
For a full list, see here: pastebin.com/cnk7FTF2
```



pub fn get_timecycle_modifier_index_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDF3D97C674AFB66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDF3D97C674AFB66u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_timecycle_modifier_index_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDF3D97C674AFB66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDF3D97C674AFB66u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _set_bink_should_skip_safe(
        
        
            binkMovie: 
        , 
        
        
            shouldSkip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6805D58CAA427B72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6805D58CAA427B72u64;
        
        let result = invoke_raw!(
            hash,
                binkMovie, 
                shouldSkip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_bink_should_skip_raw(
        binkMovie: , 
        shouldSkip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6805D58CAA427B72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6805D58CAA427B72u64;

        invoke_raw_typed!(
            hash,
                binkMovie, 
                shouldSkip
        )
    }
}

/// Hardcoded to always return 1280x720. Use [`_GET_ACTIVE_SCREEN_RESOLUTION`](#_0x873C9F3104101DD3) to retrieve the correct screen resolution.



pub fn get_screen_resolution_safe(
        
        
            x: 
        , 
        
        
            y: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x888D57E407E63624u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x888D57E407E63624u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_screen_resolution_raw(
        x: , 
        y: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x888D57E407E63624u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x888D57E407E63624u64;

        invoke_raw_typed!(
            hash,
                x, 
                y
        )
    }
}

/// ## Parameters
*



pub fn _0x2b40a97646381508_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B40A97646381508u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B40A97646381508u64;
        
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
pub fn _0x2b40a97646381508_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B40A97646381508u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B40A97646381508u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0xc5c8f970d4edff71_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5C8F970D4EDFF71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5C8F970D4EDFF71u64;
        
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
pub fn _0xc5c8f970d4edff71_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5C8F970D4EDFF71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5C8F970D4EDFF71u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
GET_CURRENT_*

NativeDB Introduced: v1493
```



pub fn _0x30432a0118736e00_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30432A0118736E00u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30432A0118736E00u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x30432a0118736e00_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30432A0118736E00u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30432A0118736E00u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn load_movie_mesh_set_safe(
        
        
            movieMeshSetName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB66064452270E8F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB66064452270E8F1u64;
        
        let result = invoke_raw!(
            hash,
                movieMeshSetName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn load_movie_mesh_set_raw(
        movieMeshSetName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB66064452270E8F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB66064452270E8F1u64;

        invoke_raw_typed!(
            hash,
                movieMeshSetName
        )
    }
}

/// ```
NOTE: Debugging functions are not present in the retail version of the game.  
```



pub fn draw_debug_line_safe(
        
        
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
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        , 
        
        
            a: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FDFADE676AA3CB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FDFADE676AA3CB0u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                r, 
                g, 
                b, 
                a
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_debug_line_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        r: , 
        g: , 
        b: , 
        a: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FDFADE676AA3CB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FDFADE676AA3CB0u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                r, 
                g, 
                b, 
                a
        )
    }
}

/// ## Parameters
*



pub fn _0xf51d36185993515d_safe(
        
        
            checkpoint: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            unkX: 
        , 
        
        
            unkY: 
        , 
        
        
            unkZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF51D36185993515Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF51D36185993515Du64;
        
        let result = invoke_raw!(
            hash,
                checkpoint, 
                posX, 
                posY, 
                posZ, 
                unkX, 
                unkY, 
                unkZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf51d36185993515d_raw(
        checkpoint: , 
        posX: , 
        posY: , 
        posZ: , 
        unkX: , 
        unkY: , 
        unkZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF51D36185993515Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF51D36185993515Du64;

        invoke_raw_typed!(
            hash,
                checkpoint, 
                posX, 
                posY, 
                posZ, 
                unkX, 
                unkY, 
                unkZ
        )
    }
}

/// ```
Forces vehicle trails on all surfaces.
USE_/USING_*
```



pub fn _set_force_vehicle_trails_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4CC7F0FEA5283FE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4CC7F0FEA5283FE0u64;
        
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
pub fn _set_force_vehicle_trails_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4CC7F0FEA5283FE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4CC7F0FEA5283FE0u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Setter for 0xE59343E9E96529E7

SET_M*
```



pub fn _0xb3c641f3630bf6da_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3C641F3630BF6DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3C641F3630BF6DAu64;
        
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
pub fn _0xb3c641f3630bf6da_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3C641F3630BF6DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3C641F3630BF6DAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x0ae73d8df3a762b2_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AE73D8DF3A762B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AE73D8DF3A762B2u64;
        
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
pub fn _0x0ae73d8df3a762b2_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AE73D8DF3A762B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AE73D8DF3A762B2u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn add_petrol_trail_decal_info_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x967278682CB6967Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x967278682CB6967Au64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_petrol_trail_decal_info_raw(
        x: , 
        y: , 
        z: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x967278682CB6967Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x967278682CB6967Au64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn trigger_screenblur_fade_out_safe(
        
        
            transitionTime: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFACC8AEF94430D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFACC8AEF94430D5u64;
        
        let result = invoke_raw!(
            hash,
                transitionTime
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn trigger_screenblur_fade_out_raw(
        transitionTime: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFACC8AEF94430D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFACC8AEF94430D5u64;

        invoke_raw_typed!(
            hash,
                transitionTime
        )
    }
}

/// Adds a literal string to a scaleform movie method.
There doesn't seem to be any difference between this and other `SCALEFORM_MOVIE_METHOD_ADD_PARAM_*_STRING` natives in game code.



pub fn scaleform_movie_method_add_param_literal_string_safe(
        
        
            string: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77FE3402004CD1B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77FE3402004CD1B0u64;
        
        let result = invoke_raw!(
            hash,
                string
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn scaleform_movie_method_add_param_literal_string_raw(
        string: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77FE3402004CD1B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77FE3402004CD1B0u64;

        invoke_raw_typed!(
            hash,
                string
        )
    }
}

/// ## Parameters
*



pub fn golf_trail_set_radius_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2485D34E50A22E84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2485D34E50A22E84u64;
        
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
pub fn golf_trail_set_radius_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2485D34E50A22E84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2485D34E50A22E84u64;

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



pub fn _return_two_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40AFB081F8ADD4EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40AFB081F8ADD4EEu64;
        
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
pub fn _return_two_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40AFB081F8ADD4EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40AFB081F8ADD4EEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn draw_scaleform_movie_fullscreen_masked_safe(
        
        
            scaleform1: 
        , 
        
        
            scaleform2: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF537FDE4FBD4CE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF537FDE4FBD4CE5u64;
        
        let result = invoke_raw!(
            hash,
                scaleform1, 
                scaleform2, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_scaleform_movie_fullscreen_masked_raw(
        scaleform1: , 
        scaleform2: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF537FDE4FBD4CE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF537FDE4FBD4CE5u64;

        invoke_raw_typed!(
            hash,
                scaleform1, 
                scaleform2, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ```
REQUEST_STREAMED_TEXTURE_DICT("MPOnMissMarkers", false);  
*uParam0.f_809 = add_decal(9120, vParam1, vVar4, vVar7, 2f, 2f, to_float(iVar0) / 255f, to_float(iVar1) / 255f, to_float(iVar2) / 255f, 1f, -1f, 1, 0, 0);  
PATCH_DECAL_DIFFUSE_MAP(9120, "MPOnMissMarkers", "Capture_The_Flag_Base_Icon");  
```



pub fn patch_decal_diffuse_map_safe(
        
        
            decalType: 
        , 
        
        
            textureDict: 
        , 
        
        
            textureName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A35C742130C6080u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A35C742130C6080u64;
        
        let result = invoke_raw!(
            hash,
                decalType, 
                textureDict, 
                textureName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn patch_decal_diffuse_map_raw(
        decalType: , 
        textureDict: , 
        textureName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A35C742130C6080u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A35C742130C6080u64;

        invoke_raw_typed!(
            hash,
                decalType, 
                textureDict, 
                textureName
        )
    }
}

/// This native retrieves whether the game is running in widescreen mode or not.

```
NativeDB Introduced: v323
```



pub fn get_is_widescreen_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30CF4BDA4FCB1905u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30CF4BDA4FCB1905u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_is_widescreen_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30CF4BDA4FCB1905u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30CF4BDA4FCB1905u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NOTE: Debugging functions are not present in the retail version of the game.  
```



pub fn draw_debug_text_2d_safe(
        
        
            text: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3BB2E9555C05A8Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3BB2E9555C05A8Fu64;
        
        let result = invoke_raw!(
            hash,
                text, 
                x, 
                y, 
                z, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_debug_text_2d_raw(
        text: , 
        x: , 
        y: , 
        z: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3BB2E9555C05A8Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3BB2E9555C05A8Fu64;

        invoke_raw_typed!(
            hash,
                text, 
                x, 
                y, 
                z, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ## Parameters
*



pub fn move_vehicle_decals_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84C8D7C2D30D3280u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84C8D7C2D30D3280u64;
        
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
pub fn move_vehicle_decals_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84C8D7C2D30D3280u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84C8D7C2D30D3280u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Starts frontend (pause menu) scaleform movie methods.
This can be used when you want to make custom frontend menus, and customize things like images or text in the menus etc.

Use [`BEGIN_SCALEFORM_MOVIE_METHOD_ON_FRONTEND_HEADER`](#_0xB9449845F73F5E9C) for header scaleform functions.



pub fn begin_scaleform_movie_method_on_frontend_safe(
        
        
            functionName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB58C27C2E6123C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB58C27C2E6123C6u64;
        
        let result = invoke_raw!(
            hash,
                functionName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_scaleform_movie_method_on_frontend_raw(
        functionName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB58C27C2E6123C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB58C27C2E6123C6u64;

        invoke_raw_typed!(
            hash,
                functionName
        )
    }
}

/// ```
All presets can be found in common\data\ui\uiscenes.meta
```



pub fn ui3dscene_push_preset_safe(
        
        
            presetName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1CEA8A4198D8E9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1CEA8A4198D8E9Au64;
        
        let result = invoke_raw!(
            hash,
                presetName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ui3dscene_push_preset_raw(
        presetName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1CEA8A4198D8E9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1CEA8A4198D8E9Au64;

        invoke_raw_typed!(
            hash,
                presetName
        )
    }
}

/// ## Parameters
*



pub fn _0x9641588dab93b4b5_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9641588DAB93B4B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9641588DAB93B4B5u64;
        
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
pub fn _0x9641588dab93b4b5_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9641588DAB93B4B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9641588DAB93B4B5u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn draw_scaleform_movie_3d_safe(
        
        
            scaleform: 
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
        
        
            p7: 
        , 
        
        
            sharpness: 
        , 
        
        
            p9: 
        , 
        
        
            scaleX: 
        , 
        
        
            scaleY: 
        , 
        
        
            scaleZ: 
        , 
        
        
            p13: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87D51D72255D4E78u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87D51D72255D4E78u64;
        
        let result = invoke_raw!(
            hash,
                scaleform, 
                posX, 
                posY, 
                posZ, 
                rotX, 
                rotY, 
                rotZ, 
                p7, 
                sharpness, 
                p9, 
                scaleX, 
                scaleY, 
                scaleZ, 
                p13
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_scaleform_movie_3d_raw(
        scaleform: , 
        posX: , 
        posY: , 
        posZ: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        p7: , 
        sharpness: , 
        p9: , 
        scaleX: , 
        scaleY: , 
        scaleZ: , 
        p13: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87D51D72255D4E78u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87D51D72255D4E78u64;

        invoke_raw_typed!(
            hash,
                scaleform, 
                posX, 
                posY, 
                posZ, 
                rotX, 
                rotY, 
                rotZ, 
                p7, 
                sharpness, 
                p9, 
                scaleX, 
                scaleY, 
                scaleZ, 
                p13
        )
    }
}

/// ## Return value



pub fn get_status_of_save_high_quality_photo_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C0C4E81E1AC60A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C0C4E81E1AC60A0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_status_of_save_high_quality_photo_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C0C4E81E1AC60A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C0C4E81E1AC60A0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// DISABLE_OCCLUSION_THIS_FRAME native function



pub fn disable_occlusion_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3669F1B198DCAA4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3669F1B198DCAA4Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_occlusion_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3669F1B198DCAA4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3669F1B198DCAA4Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn scaleform_movie_method_add_param_texture_name_string_safe(
        
        
            string: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA7148484BD90365u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA7148484BD90365u64;
        
        let result = invoke_raw!(
            hash,
                string
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn scaleform_movie_method_add_param_texture_name_string_raw(
        string: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA7148484BD90365u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA7148484BD90365u64;

        invoke_raw_typed!(
            hash,
                string
        )
    }
}

/// ```
Starts a particle effect on an entity for example your player.  
List: pastebin.com/N9unUFWY  
Example:  
C#:  
Function.Call(Hash.REQUEST_NAMED_PTFX_ASSET, "scr_rcbarry2");                     Function.Call(Hash._SET_PTFX_ASSET_NEXT_CALL, "scr_rcbarry2");                             Function.Call(Hash.START_PARTICLE_FX_NON_LOOPED_ON_ENTITY, "scr_clown_appears", Game.Player.Character, 0.0, 0.0, -0.5, 0.0, 0.0, 0.0, 1.0, false, false, false);  
Internally this calls the same function as GRAPHICS::START_PARTICLE_FX_NON_LOOPED_ON_PED_BONE  
however it uses -1 for the specified bone index, so it should be possible to start a non looped fx on an entity bone using that native  
```



pub fn start_particle_fx_non_looped_on_entity_safe(
        
        
            effectName: 
        , 
        
        
            entity: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            scale: 
        , 
        
        
            axisX: 
        , 
        
        
            axisY: 
        , 
        
        
            axisZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D53A3B8DA0809D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D53A3B8DA0809D2u64;
        
        let result = invoke_raw!(
            hash,
                effectName, 
                entity, 
                offsetX, 
                offsetY, 
                offsetZ, 
                rotX, 
                rotY, 
                rotZ, 
                scale, 
                axisX, 
                axisY, 
                axisZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_particle_fx_non_looped_on_entity_raw(
        effectName: , 
        entity: , 
        offsetX: , 
        offsetY: , 
        offsetZ: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        scale: , 
        axisX: , 
        axisY: , 
        axisZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D53A3B8DA0809D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D53A3B8DA0809D2u64;

        invoke_raw_typed!(
            hash,
                effectName, 
                entity, 
                offsetX, 
                offsetY, 
                offsetZ, 
                rotX, 
                rotY, 
                rotZ, 
                scale, 
                axisX, 
                axisY, 
                axisZ
        )
    }
}

/// ## Parameters
*



pub fn does_vehicle_have_crew_emblem_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x060D935D3981A275u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x060D935D3981A275u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_vehicle_have_crew_emblem_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x060D935D3981A275u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x060D935D3981A275u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x27feb5254759cde3_safe(
        
        
            textureDict: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27FEB5254759CDE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27FEB5254759CDE3u64;
        
        let result = invoke_raw!(
            hash,
                textureDict, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x27feb5254759cde3_raw(
        textureDict: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27FEB5254759CDE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27FEB5254759CDE3u64;

        invoke_raw_typed!(
            hash,
                textureDict, 
                p1
        )
    }
}

/// _0x14FC5833464340A8 native function



pub fn _0x14fc5833464340a8_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14FC5833464340A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14FC5833464340A8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x14fc5833464340a8_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14FC5833464340A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14FC5833464340A8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
GRAPHICS::START_PARTICLE_FX_NON_LOOPED_AT_COORD("scr_paleto_roof_impact", -140.8576f, 6420.789f, 41.1391f, 0f, 0f, 267.3957f, 0x3F800000, 0, 0, 0);  
Axis - Invert Axis Flags  
list: pastebin.com/N9unUFWY



pub fn start_particle_fx_non_looped_at_coord_safe(
        
        
            effectName: 
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
        
        
            scale: 
        , 
        
        
            xAxis: 
        , 
        
        
            yAxis: 
        , 
        
        
            zAxis: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25129531F77B9ED3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25129531F77B9ED3u64;
        
        let result = invoke_raw!(
            hash,
                effectName, 
                xPos, 
                yPos, 
                zPos, 
                xRot, 
                yRot, 
                zRot, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_particle_fx_non_looped_at_coord_raw(
        effectName: , 
        xPos: , 
        yPos: , 
        zPos: , 
        xRot: , 
        yRot: , 
        zRot: , 
        scale: , 
        xAxis: , 
        yAxis: , 
        zAxis: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25129531F77B9ED3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25129531F77B9ED3u64;

        invoke_raw_typed!(
            hash,
                effectName, 
                xPos, 
                yPos, 
                zPos, 
                xRot, 
                yRot, 
                zRot, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis
        )
    }
}

/// ## Parameters
*



pub fn set_tracked_point_info_safe(
        
        
            point: 
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
        let hash = 0x164ECBB3CF750CB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x164ECBB3CF750CB0u64;
        
        let result = invoke_raw!(
            hash,
                point, 
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
pub fn set_tracked_point_info_raw(
        point: , 
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x164ECBB3CF750CB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x164ECBB3CF750CB0u64;

        invoke_raw_typed!(
            hash,
                point, 
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// Resets the screen's draw-origin which was changed by the function [`SET_DRAW_ORIGIN`](#_0xAA0008F3BBB8F416) back to `x=0, y=0`. See [`SET_DRAW_ORIGIN`](#_0xAA0008F3BBB8F416) for further information.



pub fn clear_draw_origin_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF0B610F6BE0D7AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF0B610F6BE0D7AFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_draw_origin_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF0B610F6BE0D7AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF0B610F6BE0D7AFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
This function is hard-coded to always return 0.
```



pub fn get_maximum_number_of_photos_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34D23450F028B0BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34D23450F028B0BFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_maximum_number_of_photos_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34D23450F028B0BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34D23450F028B0BFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_current_number_of_cloud_photos_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x473151EBC762C6DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x473151EBC762C6DAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_current_number_of_cloud_photos_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x473151EBC762C6DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x473151EBC762C6DAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// NOTE: the [USE_PARTICLE_FX_ASSET](#_0x6C38AF3693A69A91) needs to be called before EVERY StartNetworkedParticleFxNonLoopedAtCoord(....) call!

List with lots of particle effects: https://vespura.com/fivem/particle-list/

Note: Not all particles on this list are for non looped and vice versa, neither are all of them suited/meant to have SetParticleFxNonLoopedColour(....) called on them.

```
NativeDB Added Parameter 12: BOOL p11
```



pub fn start_networked_particle_fx_non_looped_at_coord_safe(
        
        
            effectName: 
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
        
        
            scale: 
        , 
        
        
            xAxis: 
        , 
        
        
            yAxis: 
        , 
        
        
            zAxis: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF56B8137DF10135Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF56B8137DF10135Du64;
        
        let result = invoke_raw!(
            hash,
                effectName, 
                xPos, 
                yPos, 
                zPos, 
                xRot, 
                yRot, 
                zRot, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_networked_particle_fx_non_looped_at_coord_raw(
        effectName: , 
        xPos: , 
        yPos: , 
        zPos: , 
        xRot: , 
        yRot: , 
        zRot: , 
        scale: , 
        xAxis: , 
        yAxis: , 
        zAxis: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF56B8137DF10135Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF56B8137DF10135Du64;

        invoke_raw_typed!(
            hash,
                effectName, 
                xPos, 
                yPos, 
                zPos, 
                xRot, 
                yRot, 
                zRot, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis
        )
    }
}

/// ## Parameters
*



pub fn save_high_quality_photo_safe(
        
        
            unused: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DEC726C25A11BACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DEC726C25A11BACu64;
        
        let result = invoke_raw!(
            hash,
                unused
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn save_high_quality_photo_raw(
        unused: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DEC726C25A11BACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DEC726C25A11BACu64;

        invoke_raw_typed!(
            hash,
                unused
        )
    }
}

/// ## Parameters
*



pub fn update_lights_on_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEADC0DEDEADC0DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEADC0DEDEADC0DEu64;
        
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
pub fn update_lights_on_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEADC0DEDEADC0DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEADC0DEDEADC0DEu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn load_mission_creator_photo_safe(
        
        
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
        let hash = 0x4862437A486F91B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4862437A486F91B0u64;
        
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
pub fn load_mission_creator_photo_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4862437A486F91B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4862437A486F91B0u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ```
NOTE: Debugging functions are not present in the retail version of the game.  
```



pub fn draw_debug_cross_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            size: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73B1189623049839u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73B1189623049839u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                size, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_debug_cross_raw(
        x: , 
        y: , 
        z: , 
        size: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73B1189623049839u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73B1189623049839u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                size, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// Passes keyboard input to scaleform. You must call this native every frame. Once an input occurs, this native will return true and call `SET_PC_KEY` scaleform movie method with the key that has been inputted.

The key parameter which is passed to the scaleform can also be: "BACKSPACE", "ENTER" or "\x1b" (Which is ESC).
This native is only used in `web_browser.c` as of game build 2944.



pub fn pass_keyboard_input_to_scaleform_safe(
        
        
            scaleformHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1C7CB175E012964u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1C7CB175E012964u64;
        
        let result = invoke_raw!(
            hash,
                scaleformHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pass_keyboard_input_to_scaleform_raw(
        scaleformHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1C7CB175E012964u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1C7CB175E012964u64;

        invoke_raw_typed!(
            hash,
                scaleformHandle
        )
    }
}

/// ## Parameters
*



pub fn set_particle_fx_looped_alpha_safe(
        
        
            ptfxHandle: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x726845132380142Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x726845132380142Eu64;
        
        let result = invoke_raw!(
            hash,
                ptfxHandle, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_particle_fx_looped_alpha_raw(
        ptfxHandle: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x726845132380142Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x726845132380142Eu64;

        invoke_raw_typed!(
            hash,
                ptfxHandle, 
                alpha
        )
    }
}

/// ## Parameters
*



pub fn cascade_shadows_set_aircraft_mode_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6DDBF9DFFC4AC080u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6DDBF9DFFC4AC080u64;
        
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
pub fn cascade_shadows_set_aircraft_mode_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6DDBF9DFFC4AC080u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6DDBF9DFFC4AC080u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_particle_fx_looped_far_clip_dist_safe(
        
        
            ptfxHandle: 
        , 
        
        
            range: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCB194B85EF7B541u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCB194B85EF7B541u64;
        
        let result = invoke_raw!(
            hash,
                ptfxHandle, 
                range
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_particle_fx_looped_far_clip_dist_raw(
        ptfxHandle: , 
        range: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCB194B85EF7B541u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCB194B85EF7B541u64;

        invoke_raw_typed!(
            hash,
                ptfxHandle, 
                range
        )
    }
}

/// ## Parameters
*



pub fn set_player_tcmodifier_transition_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBDEB86F4D5809204u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBDEB86F4D5809204u64;
        
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
pub fn set_player_tcmodifier_transition_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBDEB86F4D5809204u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBDEB86F4D5809204u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// Draws a 3D sphere, typically seen in the GTA:O freemode event "Penned In".

Example [image](https://imgur.com/nCbtS4H):
```lua
DrawSphere(35.45, 172.66, 126.22, 1.0, 0, 0, 255, 0.2)
```



pub fn _draw_sphere_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        , 
        
        
            opacity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x799017F9E3B10112u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x799017F9E3B10112u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                r, 
                g, 
                b, 
                opacity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _draw_sphere_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        r: , 
        g: , 
        b: , 
        opacity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x799017F9E3B10112u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x799017F9E3B10112u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                r, 
                g, 
                b, 
                opacity
        )
    }
}

/// ```
Forces footstep tracks on all surfaces.
USE_/USING_*
```



pub fn _set_force_ped_footsteps_tracks_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAEEDAD1420C65CC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAEEDAD1420C65CC0u64;
        
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
pub fn _set_force_ped_footsteps_tracks_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAEEDAD1420C65CC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAEEDAD1420C65CC0u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn golf_trail_set_facing_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06F761EA47C1D3EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06F761EA47C1D3EDu64;
        
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
pub fn golf_trail_set_facing_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06F761EA47C1D3EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06F761EA47C1D3EDu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn get_scaleform_movie_method_return_value_bool_safe(
        
        
            methodReturn: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD80A80346A45D761u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD80A80346A45D761u64;
        
        let result = invoke_raw!(
            hash,
                methodReturn
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_scaleform_movie_method_return_value_bool_raw(
        methodReturn: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD80A80346A45D761u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD80A80346A45D761u64;

        invoke_raw_typed!(
            hash,
                methodReturn
        )
    }
}

/// ## Parameters
*



pub fn _0xae51bc858f32ba66_safe(
        
        
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
        let hash = 0xAE51BC858F32BA66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE51BC858F32BA66u64;
        
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
pub fn _0xae51bc858f32ba66_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE51BC858F32BA66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE51BC858F32BA66u64;

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

/// ## Return value



pub fn ui3dscene_is_available_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3A10FC7FD8D98CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3A10FC7FD8D98CDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn ui3dscene_is_available_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3A10FC7FD8D98CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3A10FC7FD8D98CDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// This native doesn't work like [`SetWeatherTypeTransition`](#_0x578C752848ECFA0C).



pub fn set_transition_timecycle_modifier_safe(
        
        
            modifierName: 
        , 
        
        
            transition: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BCF567485E1971Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BCF567485E1971Cu64;
        
        let result = invoke_raw!(
            hash,
                modifierName, 
                transition
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_transition_timecycle_modifier_raw(
        modifierName: , 
        transition: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BCF567485E1971Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BCF567485E1971Cu64;

        invoke_raw_typed!(
            hash,
                modifierName, 
                transition
        )
    }
}

/// ## Parameters
*



pub fn cascade_shadows_set_dynamic_depth_mode_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD39D13C9FEBF0511u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD39D13C9FEBF0511u64;
        
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
pub fn cascade_shadows_set_dynamic_depth_mode_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD39D13C9FEBF0511u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD39D13C9FEBF0511u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn override_interior_smoke_name_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A2A52824DB96700u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A2A52824DB96700u64;
        
        let result = invoke_raw!(
            hash,
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn override_interior_smoke_name_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A2A52824DB96700u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A2A52824DB96700u64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ## Parameters
*



pub fn cascade_shadows_set_dynamic_depth_value_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02AC28F3A01FA04Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02AC28F3A01FA04Au64;
        
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
pub fn cascade_shadows_set_dynamic_depth_value_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02AC28F3A01FA04Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02AC28F3A01FA04Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v2699
```



pub fn _add_oil_decal_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            groundLvl: 
        , 
        
        
            width: 
        , 
        
        
            transparency: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x126D7F89FE859A5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x126D7F89FE859A5Eu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                groundLvl, 
                width, 
                transparency
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _add_oil_decal_raw(
        x: , 
        y: , 
        z: , 
        groundLvl: , 
        width: , 
        transparency: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x126D7F89FE859A5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x126D7F89FE859A5Eu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                groundLvl, 
                width, 
                transparency
        )
    }
}

/// This multiplies the height of the icon inside a checkpoint with the default height of about 2 units above the checkpoint's coordinates.



pub fn _set_checkpoint_icon_height_safe(
        
        
            checkpoint: 
        , 
        
        
            height_multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B5B4DA5D79F1943u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B5B4DA5D79F1943u64;
        
        let result = invoke_raw!(
            hash,
                checkpoint, 
                height_multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_checkpoint_icon_height_raw(
        checkpoint: , 
        height_multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B5B4DA5D79F1943u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B5B4DA5D79F1943u64;

        invoke_raw_typed!(
            hash,
                checkpoint, 
                height_multiplier
        )
    }
}

/// ## Parameters
*



pub fn golf_trail_get_visual_control_point_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4664972A9B8F8BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4664972A9B8F8BAu64;
        
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
pub fn golf_trail_get_visual_control_point_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4664972A9B8F8BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4664972A9B8F8BAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// See [`GET_TIMECYCLE_MODIFIER_INDEX`](#_0xFDF3D97C674AFB66) for use, works the same just for the secondary timecycle modifier.



pub fn _get_extra_timecycle_modifier_index_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB0527EC6341496Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB0527EC6341496Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_extra_timecycle_modifier_index_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB0527EC6341496Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB0527EC6341496Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Convert a world coordinate into its relative screen coordinate.  (WorldToScreen)
Returns a boolean; whether or not the operation was successful. It will return false if the coordinates given are not visible to the rendering camera.
For .NET users...
VB:
Public Shared Function World3DToScreen2d(pos as vector3) As Vector2
        Dim x2dp, y2dp As New Native.OutputArgument
        Native.Function.Call(Of Boolean)(Native.Hash.GET_SCREEN_COORD_FROM_WORLD_COORD , pos.x, pos.y, pos.z, x2dp, y2dp)
        Return New Vector2(x2dp.GetResult(Of Single), y2dp.GetResult(Of Single))

    End Function
C#:
Vector2 World3DToScreen2d(Vector3 pos)
    {
        var x2dp = new OutputArgument();
        var y2dp = new OutputArgument();
        Function.Call<bool>(Hash.GET_SCREEN_COORD_FROM_WORLD_COORD , pos.X, pos.Y, pos.Z, x2dp, y2dp);
        return new Vector2(x2dp.GetResult<float>(), y2dp.GetResult<float>());
    }
//USE VERY SMALL VALUES FOR THE SCALE OF RECTS/TEXT because it is dramatically larger on screen than in 3D, e.g '0.05' small.
Used to be called _WORLD3D_TO_SCREEN2D
I thought we lost you from the scene forever. It does seem however that calling SET_DRAW_ORIGIN then your natives, then ending it. Seems to work better for certain things such as keeping boxes around people for a predator missile e.g.
```



pub fn get_screen_coord_from_world_coord_safe(
        
        
            worldX: 
        , 
        
        
            worldY: 
        , 
        
        
            worldZ: 
        , 
        
        
            screenX: 
        , 
        
        
            screenY: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34E82F05DF2974F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34E82F05DF2974F5u64;
        
        let result = invoke_raw!(
            hash,
                worldX, 
                worldY, 
                worldZ, 
                screenX, 
                screenY
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_screen_coord_from_world_coord_raw(
        worldX: , 
        worldY: , 
        worldZ: , 
        screenX: , 
        screenY: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34E82F05DF2974F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34E82F05DF2974F5u64;

        invoke_raw_typed!(
            hash,
                worldX, 
                worldY, 
                worldZ, 
                screenX, 
                screenY
        )
    }
}

/// ## Parameters
*



pub fn set_particle_fx_looped_evolution_safe(
        
        
            ptfxHandle: 
        , 
        
        
            propertyName: 
        , 
        
        
            amount: 
        , 
        
        
            noNetwork: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F0C4B5B1C393BE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F0C4B5B1C393BE2u64;
        
        let result = invoke_raw!(
            hash,
                ptfxHandle, 
                propertyName, 
                amount, 
                noNetwork
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_particle_fx_looped_evolution_raw(
        ptfxHandle: , 
        propertyName: , 
        amount: , 
        noNetwork: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F0C4B5B1C393BE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F0C4B5B1C393BE2u64;

        invoke_raw_typed!(
            hash,
                ptfxHandle, 
                propertyName, 
                amount, 
                noNetwork
        )
    }
}

/// ## Parameters
*



pub fn set_noisinessoveride_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB6A7C3BB17A0C67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB6A7C3BB17A0C67u64;
        
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
pub fn set_noisinessoveride_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB6A7C3BB17A0C67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB6A7C3BB17A0C67u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ```
Parameters:  
* pos - coordinate where the spotlight is located  
* dir - the direction vector the spotlight should aim at from its current position  
* r,g,b - color of the spotlight  
* distance - the maximum distance the light can reach  
* brightness - the brightness of the light  
* roundness - "smoothness" of the circle edge  
* radius - the radius size of the spotlight  
* falloff - the falloff size of the light's edge (example: www.i.imgur.com/DemAWeO.jpg)  
Example in C# (spotlight aims at the closest vehicle):  
Vector3 myPos = Game.Player.Character.Position;  
Vehicle nearest = World.GetClosestVehicle(myPos , 1000f);  
Vector3 destinationCoords = nearest.Position;  
Vector3 dirVector = destinationCoords - myPos;  
dirVector.Normalize();  
Function.Call(Hash.DRAW_SPOT_LIGHT, pos.X, pos.Y, pos.Z, dirVector.X, dirVector.Y, dirVector.Z, 255, 255, 255, 100.0f, 1f, 0.0f, 13.0f, 1f);  
```



pub fn draw_spot_light_safe(
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            dirX: 
        , 
        
        
            dirY: 
        , 
        
        
            dirZ: 
        , 
        
        
            colorR: 
        , 
        
        
            colorG: 
        , 
        
        
            colorB: 
        , 
        
        
            distance: 
        , 
        
        
            brightness: 
        , 
        
        
            hardness: 
        , 
        
        
            radius: 
        , 
        
        
            falloff: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0F64B265C8C8B33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0F64B265C8C8B33u64;
        
        let result = invoke_raw!(
            hash,
                posX, 
                posY, 
                posZ, 
                dirX, 
                dirY, 
                dirZ, 
                colorR, 
                colorG, 
                colorB, 
                distance, 
                brightness, 
                hardness, 
                radius, 
                falloff
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_spot_light_raw(
        posX: , 
        posY: , 
        posZ: , 
        dirX: , 
        dirY: , 
        dirZ: , 
        colorR: , 
        colorG: , 
        colorB: , 
        distance: , 
        brightness: , 
        hardness: , 
        radius: , 
        falloff: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0F64B265C8C8B33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0F64B265C8C8B33u64;

        invoke_raw_typed!(
            hash,
                posX, 
                posY, 
                posZ, 
                dirX, 
                dirY, 
                dirZ, 
                colorR, 
                colorG, 
                colorB, 
                distance, 
                brightness, 
                hardness, 
                radius, 
                falloff
        )
    }
}

/// DISABLE_SCREENBLUR_FADE native function



pub fn disable_screenblur_fade_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE81239437E8C5A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE81239437E8C5A8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_screenblur_fade_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE81239437E8C5A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE81239437E8C5A8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p1 is always 0 in the native scripts  
```



pub fn stop_particle_fx_looped_safe(
        
        
            ptfxHandle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F75998877616996u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F75998877616996u64;
        
        let result = invoke_raw!(
            hash,
                ptfxHandle, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_particle_fx_looped_raw(
        ptfxHandle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F75998877616996u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F75998877616996u64;

        invoke_raw_typed!(
            hash,
                ptfxHandle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x82acc484ffa3b05f_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82ACC484FFA3B05Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82ACC484FFA3B05Fu64;
        
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
pub fn _0x82acc484ffa3b05f_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82ACC484FFA3B05Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82ACC484FFA3B05Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
From the b678d decompiled scripts:
 GRAPHICS::_SET_PTFX_ASSET_NEXT_CALL("FM_Mission_Controler");
 GRAPHICS::_SET_PTFX_ASSET_NEXT_CALL("scr_apartment_mp");
 GRAPHICS::_SET_PTFX_ASSET_NEXT_CALL("scr_indep_fireworks");
 GRAPHICS::_SET_PTFX_ASSET_NEXT_CALL("scr_mp_cig_plane");
 GRAPHICS::_SET_PTFX_ASSET_NEXT_CALL("scr_mp_creator");
 GRAPHICS::_SET_PTFX_ASSET_NEXT_CALL("scr_ornate_heist");
 GRAPHICS::_SET_PTFX_ASSET_NEXT_CALL("scr_prison_break_heist_station");
```



pub fn use_particle_fx_asset_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C38AF3693A69A91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C38AF3693A69A91u64;
        
        let result = invoke_raw!(
            hash,
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn use_particle_fx_asset_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C38AF3693A69A91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C38AF3693A69A91u64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// RESET_PAUSED_RENDERPHASES native function



pub fn reset_paused_renderphases_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1C8709406F2C41Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1C8709406F2C41Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_paused_renderphases_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1C8709406F2C41Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1C8709406F2C41Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x851CD923176EBA7C native function



pub fn _0x851cd923176eba7c_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x851CD923176EBA7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x851CD923176EBA7Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x851cd923176eba7c_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x851CD923176EBA7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x851CD923176EBA7Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x7fa5d82b8f58ec06_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FA5D82B8F58EC06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FA5D82B8F58EC06u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7fa5d82b8f58ec06_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FA5D82B8F58EC06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FA5D82B8F58EC06u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn golf_trail_get_max_height_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4819F5E23E2FFADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4819F5E23E2FFADu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn golf_trail_get_max_height_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4819F5E23E2FFADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4819F5E23E2FFADu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn _0x2a251aa48b2b46db_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A251AA48B2B46DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A251AA48B2B46DBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2a251aa48b2b46db_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A251AA48B2B46DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A251AA48B2B46DBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _set_bink_movie_unk_2_safe(
        
        
            binkMovie: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF816F2933752322Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF816F2933752322Du64;
        
        let result = invoke_raw!(
            hash,
                binkMovie, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_bink_movie_unk_2_raw(
        binkMovie: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF816F2933752322Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF816F2933752322Du64;

        invoke_raw_typed!(
            hash,
                binkMovie, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn remove_decals_from_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE91F1B65F2B48D57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE91F1B65F2B48D57u64;
        
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
pub fn remove_decals_from_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE91F1B65F2B48D57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE91F1B65F2B48D57u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn remove_decals_from_object_facing_safe(
        
        
            obj: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6F6F70FDC6D144Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6F6F70FDC6D144Cu64;
        
        let result = invoke_raw!(
            hash,
                obj, 
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
pub fn remove_decals_from_object_facing_raw(
        obj: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6F6F70FDC6D144Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6F6F70FDC6D144Cu64;

        invoke_raw_typed!(
            hash,
                obj, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn set_noiseoveride_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE787BF1C5CF823C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE787BF1C5CF823C9u64;
        
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
pub fn set_noiseoveride_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE787BF1C5CF823C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE787BF1C5CF823C9u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn remove_particle_fx_from_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8FEAEEBCC127425u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8FEAEEBCC127425u64;
        
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
pub fn remove_particle_fx_from_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8FEAEEBCC127425u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8FEAEEBCC127425u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
DISABLE_S*
```



pub fn _0x02369d5c8a51fdcf_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02369D5C8A51FDCFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02369D5C8A51FDCFu64;
        
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
pub fn _0x02369d5c8a51fdcf_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02369D5C8A51FDCFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02369D5C8A51FDCFu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Calls the Scaleform function and passes both float and string parameters (in their respective order).  
The number of parameters passed to the function varies, so the end of the float parameters is represented by -1.0, and the end of the string parameters is represented by 0 (NULL).  
NOTE: The order of parameters in the function prototype is important! All float parameters must come first, followed by the string parameters.  
Examples:  
// function MY_FUNCTION(floatParam1, floatParam2, stringParam)  
GRAPHICS::_CALL_SCALEFORM_MOVIE_FUNCTION_MIXED_PARAMS(scaleform, "MY_FUNCTION", 10.0, 20.0, -1.0, -1.0, -1.0, "String param", 0, 0, 0, 0);  
// function MY_FUNCTION_2(floatParam, stringParam1, stringParam2)  
GRAPHICS::_CALL_SCALEFORM_MOVIE_FUNCTION_MIXED_PARAMS(scaleform, "MY_FUNCTION_2", 10.0, -1.0, -1.0, -1.0, -1.0, "String param #1", "String param #2", 0, 0, 0);  
```



pub fn call_scaleform_movie_method_with_number_and_string_safe(
        
        
            scaleform: 
        , 
        
        
            methodName: 
        , 
        
        
            floatParam1: 
        , 
        
        
            floatParam2: 
        , 
        
        
            floatParam3: 
        , 
        
        
            floatParam4: 
        , 
        
        
            floatParam5: 
        , 
        
        
            stringParam1: 
        , 
        
        
            stringParam2: 
        , 
        
        
            stringParam3: 
        , 
        
        
            stringParam4: 
        , 
        
        
            stringParam5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF662D8D57E290B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF662D8D57E290B1u64;
        
        let result = invoke_raw!(
            hash,
                scaleform, 
                methodName, 
                floatParam1, 
                floatParam2, 
                floatParam3, 
                floatParam4, 
                floatParam5, 
                stringParam1, 
                stringParam2, 
                stringParam3, 
                stringParam4, 
                stringParam5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn call_scaleform_movie_method_with_number_and_string_raw(
        scaleform: , 
        methodName: , 
        floatParam1: , 
        floatParam2: , 
        floatParam3: , 
        floatParam4: , 
        floatParam5: , 
        stringParam1: , 
        stringParam2: , 
        stringParam3: , 
        stringParam4: , 
        stringParam5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF662D8D57E290B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF662D8D57E290B1u64;

        invoke_raw_typed!(
            hash,
                scaleform, 
                methodName, 
                floatParam1, 
                floatParam2, 
                floatParam3, 
                floatParam4, 
                floatParam5, 
                stringParam1, 
                stringParam2, 
                stringParam3, 
                stringParam4, 
                stringParam5
        )
    }
}

/// ```
AD*
```



pub fn _0xefabc7722293da7c_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFABC7722293DA7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFABC7722293DA7Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xefabc7722293da7c_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFABC7722293DA7Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFABC7722293DA7Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn golf_trail_set_tessellation_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBAA5EC848BA2D46u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBAA5EC848BA2D46u64;
        
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
pub fn golf_trail_set_tessellation_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBAA5EC848BA2D46u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBAA5EC848BA2D46u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// See [`ANIMPOSTFX_PLAY`](#_0x2206BF9A37B7F724).



pub fn animpostfx_stop_safe(
        
        
            effectName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x068E835A1D0DC0E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x068E835A1D0DC0E3u64;
        
        let result = invoke_raw!(
            hash,
                effectName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn animpostfx_stop_raw(
        effectName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x068E835A1D0DC0E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x068E835A1D0DC0E3u64;

        invoke_raw_typed!(
            hash,
                effectName
        )
    }
}

/// ```
Wraps 0xAAE9BE70EC7C69AB with FLT_MAX as p7, Jenkins: 0x73E96210?
```



pub fn _grass_lod_shrink_script_areas_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D955F6A9E0295B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D955F6A9E0295B1u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                p4, 
                p5, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _grass_lod_shrink_script_areas_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        p4: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D955F6A9E0295B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D955F6A9E0295B1u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                p4, 
                p5, 
                p6
        )
    }
}

/// ## Parameters
*



pub fn force_render_in_game_ui_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC459CFA0CCE245Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC459CFA0CCE245Bu64;
        
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
pub fn force_render_in_game_ui_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC459CFA0CCE245Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC459CFA0CCE245Bu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// See [`ANIMPOSTFX_PLAY`](#_0x2206BF9A37B7F724)



pub fn _animpostfx_get_unk_safe(
        
        
            effectName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE35B38A27E8E7179u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE35B38A27E8E7179u64;
        
        let result = invoke_raw!(
            hash,
                effectName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _animpostfx_get_unk_raw(
        effectName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE35B38A27E8E7179u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE35B38A27E8E7179u64;

        invoke_raw_typed!(
            hash,
                effectName
        )
    }
}

/// Used with 'NG_filmnoir_BW{01,02}' timecycles and the "NOIR_FILTER_SOUNDS" audioref.



pub fn _register_noir_screen_effect_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA44FF770DFBC5DAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA44FF770DFBC5DAEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _register_noir_screen_effect_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA44FF770DFBC5DAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA44FF770DFBC5DAEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Calls the Scaleform function.  
```



pub fn call_scaleform_movie_method_safe(
        
        
            scaleform: 
        , 
        
        
            method: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFBD96D87AC96D533u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFBD96D87AC96D533u64;
        
        let result = invoke_raw!(
            hash,
                scaleform, 
                method
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn call_scaleform_movie_method_raw(
        scaleform: , 
        method: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFBD96D87AC96D533u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFBD96D87AC96D533u64;

        invoke_raw_typed!(
            hash,
                scaleform, 
                method
        )
    }
}

/// SET_DISABLE_DECAL_RENDERING_THIS_FRAME native function



pub fn set_disable_decal_rendering_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B5CFC83122DF602u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B5CFC83122DF602u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_disable_decal_rendering_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B5CFC83122DF602u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B5CFC83122DF602u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
GRAPHICS::UNPATCH_DECAL_DIFFUSE_MAP(9123);  
GRAPHICS::SET_STREAMED_TEXTURE_DICT_AS_NO_LONGER_NEEDED("MPMissMarkers256");  
```



pub fn unpatch_decal_diffuse_map_safe(
        
        
            decalType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7ED70C49521A61Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7ED70C49521A61Du64;
        
        let result = invoke_raw!(
            hash,
                decalType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn unpatch_decal_diffuse_map_raw(
        decalType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7ED70C49521A61Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7ED70C49521A61Du64;

        invoke_raw_typed!(
            hash,
                decalType
        )
    }
}

/// ## Parameters
*



pub fn _seethrough_set_hi_light_intensity_safe(
        
        
            intensity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19E50EB6E33E1D28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19E50EB6E33E1D28u64;
        
        let result = invoke_raw!(
            hash,
                intensity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _seethrough_set_hi_light_intensity_raw(
        intensity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19E50EB6E33E1D28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19E50EB6E33E1D28u64;

        invoke_raw_typed!(
            hash,
                intensity
        )
    }
}

/// Draws a marker with the specified appearance at the target location. This has to be called every frame, e.g. in a `Wait(0)` loop.

There's a [list of markers](https://docs.fivem.net/game-references/markers/) on the FiveM documentation site.



pub fn draw_marker_safe(
        
        
            type: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            dirX: 
        , 
        
        
            dirY: 
        , 
        
        
            dirZ: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            scaleX: 
        , 
        
        
            scaleY: 
        , 
        
        
            scaleZ: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        , 
        
        
            bobUpAndDown: 
        , 
        
        
            faceCamera: 
        , 
        
        
            rotationOrder: 
        , 
        
        
            rotate: 
        , 
        
        
            textureDict: 
        , 
        
        
            textureName: 
        , 
        
        
            drawOnEnts: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28477EC23D892089u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28477EC23D892089u64;
        
        let result = invoke_raw!(
            hash,
                type, 
                posX, 
                posY, 
                posZ, 
                dirX, 
                dirY, 
                dirZ, 
                rotX, 
                rotY, 
                rotZ, 
                scaleX, 
                scaleY, 
                scaleZ, 
                red, 
                green, 
                blue, 
                alpha, 
                bobUpAndDown, 
                faceCamera, 
                rotationOrder, 
                rotate, 
                textureDict, 
                textureName, 
                drawOnEnts
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_marker_raw(
        type: , 
        posX: , 
        posY: , 
        posZ: , 
        dirX: , 
        dirY: , 
        dirZ: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        scaleX: , 
        scaleY: , 
        scaleZ: , 
        red: , 
        green: , 
        blue: , 
        alpha: , 
        bobUpAndDown: , 
        faceCamera: , 
        rotationOrder: , 
        rotate: , 
        textureDict: , 
        textureName: , 
        drawOnEnts: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28477EC23D892089u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28477EC23D892089u64;

        invoke_raw_typed!(
            hash,
                type, 
                posX, 
                posY, 
                posZ, 
                dirX, 
                dirY, 
                dirZ, 
                rotX, 
                rotY, 
                rotZ, 
                scaleX, 
                scaleY, 
                scaleZ, 
                red, 
                green, 
                blue, 
                alpha, 
                bobUpAndDown, 
                faceCamera, 
                rotationOrder, 
                rotate, 
                textureDict, 
                textureName, 
                drawOnEnts
        )
    }
}

/// ## Parameters
*



pub fn set_streamed_texture_dict_as_no_longer_needed_safe(
        
        
            textureDict: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE2CACCF5A8AA805u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE2CACCF5A8AA805u64;
        
        let result = invoke_raw!(
            hash,
                textureDict
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_streamed_texture_dict_as_no_longer_needed_raw(
        textureDict: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE2CACCF5A8AA805u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE2CACCF5A8AA805u64;

        invoke_raw_typed!(
            hash,
                textureDict
        )
    }
}

/// ## Parameters
*



pub fn start_petrol_trail_decals_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99AC7F0D8B9C893Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99AC7F0D8B9C893Du64;
        
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
pub fn start_petrol_trail_decals_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99AC7F0D8B9C893Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99AC7F0D8B9C893Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn cascade_shadows_set_entity_tracker_scale_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E9DAF5A20F15908u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E9DAF5A20F15908u64;
        
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
pub fn cascade_shadows_set_entity_tracker_scale_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E9DAF5A20F15908u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E9DAF5A20F15908u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// SET_CHECKPOINT_*

```
NativeDB Introduced: v1734
```



pub fn _0xfcf6788fc4860cd4_safe(
        
        
            checkpoint: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCF6788FC4860CD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCF6788FC4860CD4u64;
        
        let result = invoke_raw!(
            hash,
                checkpoint
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xfcf6788fc4860cd4_raw(
        checkpoint: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCF6788FC4860CD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCF6788FC4860CD4u64;

        invoke_raw_typed!(
            hash,
                checkpoint
        )
    }
}

/// ## Parameters
*



pub fn _set_checkpoint_icon_scale_safe(
        
        
            checkpoint: 
        , 
        
        
            scale: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44621483FF966526u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44621483FF966526u64;
        
        let result = invoke_raw!(
            hash,
                checkpoint, 
                scale
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_checkpoint_icon_scale_raw(
        checkpoint: , 
        scale: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44621483FF966526u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44621483FF966526u64;

        invoke_raw_typed!(
            hash,
                checkpoint, 
                scale
        )
    }
}

/// ## Parameters
*



pub fn fade_up_ped_light_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9B18B4619F48F7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9B18B4619F48F7Bu64;
        
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
pub fn fade_up_ped_light_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9B18B4619F48F7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9B18B4619F48F7Bu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Draws a 2D sprite on the screen.  
Parameters:  
textureDict - Name of texture dictionary to load texture from (e.g. "CommonMenu", "MPWeaponsCommon", etc.)  
textureName - Name of texture to load from texture dictionary (e.g. "last_team_standing_icon", "tennis_icon", etc.)  
screenX/Y - Screen offset (0.5 = center)  
scaleX/Y - Texture scaling. Negative values can be used to flip the texture on that axis. (0.5 = half)  
heading - Texture rotation in degrees (default = 0.0) positive is clockwise, measured in degrees  
red,green,blue - Sprite color (default = 255/255/255)  
alpha - opacity level  
```

```
NativeDB Added Parameter 12: BOOL p11
```



pub fn draw_sprite_safe(
        
        
            textureDict: 
        , 
        
        
            textureName: 
        , 
        
        
            screenX: 
        , 
        
        
            screenY: 
        , 
        
        
            width: 
        , 
        
        
            height: 
        , 
        
        
            heading: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7FFAE5EBF23D890u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7FFAE5EBF23D890u64;
        
        let result = invoke_raw!(
            hash,
                textureDict, 
                textureName, 
                screenX, 
                screenY, 
                width, 
                height, 
                heading, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_sprite_raw(
        textureDict: , 
        textureName: , 
        screenX: , 
        screenY: , 
        width: , 
        height: , 
        heading: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7FFAE5EBF23D890u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7FFAE5EBF23D890u64;

        invoke_raw_typed!(
            hash,
                textureDict, 
                textureName, 
                screenX, 
                screenY, 
                width, 
                height, 
                heading, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ## Parameters
*



pub fn _0x649c97d52332341a_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x649C97D52332341Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x649C97D52332341Au64;
        
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
pub fn _0x649c97d52332341a_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x649C97D52332341Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x649C97D52332341Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn golf_trail_set_colour_safe(
        
        
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
        , 
        
        
            p9: 
        , 
        
        
            p10: 
        , 
        
        
            p11: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12995F2E53FFA601u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12995F2E53FFA601u64;
        
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
                p8, 
                p9, 
                p10, 
                p11
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn golf_trail_set_colour_raw(
        p0: , 
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
        p11: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12995F2E53FFA601u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12995F2E53FFA601u64;

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
                p8, 
                p9, 
                p10, 
                p11
        )
    }
}

/// ```
Pushes an integer for the Scaleform function onto the stack.  
```



pub fn scaleform_movie_method_add_param_int_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3D0841A0CC546A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3D0841A0CC546A6u64;
        
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
pub fn scaleform_movie_method_add_param_int_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3D0841A0CC546A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3D0841A0CC546A6u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x43fa7cbe20dab219_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43FA7CBE20DAB219u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43FA7CBE20DAB219u64;
        
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
pub fn _0x43fa7cbe20dab219_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43FA7CBE20DAB219u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43FA7CBE20DAB219u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x03300b57fcac6ddb_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03300B57FCAC6DDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03300B57FCAC6DDBu64;
        
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
pub fn _0x03300b57fcac6ddb_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03300B57FCAC6DDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03300B57FCAC6DDBu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn is_decal_alive_safe(
        
        
            decal: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC694D74949CAFD0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC694D74949CAFD0Cu64;
        
        let result = invoke_raw!(
            hash,
                decal
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_decal_alive_raw(
        decal: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC694D74949CAFD0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC694D74949CAFD0Cu64;

        invoke_raw_typed!(
            hash,
                decal
        )
    }
}

/// Sets the draw order for script draw commands.
Examples from decompiled scripts:
GRAPHICS::SET_SCRIPT_GFX_DRAW_ORDER(7);
GRAPHICS::DRAW_RECT(0.5, 0.5, 3.0, 3.0, v_4, v_5, v_6, a_0._f172, 0);
GRAPHICS::SET_SCRIPT_GFX_DRAW_ORDER(1);
GRAPHICS::DRAW_RECT(0.5, 0.5, 1.5, 1.5, 0, 0, 0, 255, 0);

```c
enum eGfxDrawOrder
{
    GFX_ORDER_BEFORE_HUD_PRIORITY_LOW = 0,
    GFX_ORDER_BEFORE_HUD = 1,
    GFX_ORDER_BEFORE_HUD_PRIORITY_HIGH = 2,
    GFX_ORDER_AFTER_HUD_PRIORITY_LOW = 3,
    GFX_ORDER_AFTER_HUD = 4,
    GFX_ORDER_AFTER_HUD_PRIORITY_HIGH = 5,
    GFX_ORDER_AFTER_FADE_PRIORITY_LOW = 6,
    GFX_ORDER_AFTER_FADE = 7,
    GFX_ORDER_AFTER_FADE_PRIORITY_HIGH = 8,
}
```



pub fn set_script_gfx_draw_order_safe(
        
        
            order: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61BB1D9B3A95D802u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61BB1D9B3A95D802u64;
        
        let result = invoke_raw!(
            hash,
                order
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_script_gfx_draw_order_raw(
        order: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61BB1D9B3A95D802u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61BB1D9B3A95D802u64;

        invoke_raw_typed!(
            hash,
                order
        )
    }
}

/// OVERRIDE_INTERIOR_SMOKE_END native function



pub fn override_interior_smoke_end_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFB55E7C25D3B3BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFB55E7C25D3B3BEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn override_interior_smoke_end_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFB55E7C25D3B3BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFB55E7C25D3B3BEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Calls the Scaleform function and passes the parameters as strings.  
The number of parameters passed to the function varies, so the end of the parameter list is represented by 0 (NULL).  
```



pub fn call_scaleform_movie_method_with_string_safe(
        
        
            scaleform: 
        , 
        
        
            methodName: 
        , 
        
        
            param1: 
        , 
        
        
            param2: 
        , 
        
        
            param3: 
        , 
        
        
            param4: 
        , 
        
        
            param5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51BC1ED3CC44E8F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51BC1ED3CC44E8F7u64;
        
        let result = invoke_raw!(
            hash,
                scaleform, 
                methodName, 
                param1, 
                param2, 
                param3, 
                param4, 
                param5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn call_scaleform_movie_method_with_string_raw(
        scaleform: , 
        methodName: , 
        param1: , 
        param2: , 
        param3: , 
        param4: , 
        param5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51BC1ED3CC44E8F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51BC1ED3CC44E8F7u64;

        invoke_raw_typed!(
            hash,
                scaleform, 
                methodName, 
                param1, 
                param2, 
                param3, 
                param4, 
                param5
        )
    }
}

/// ## Parameters
*



pub fn has_scaleform_script_hud_movie_loaded_safe(
        
        
            hudComponent: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF6E5987D2B4D140u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF6E5987D2B4D140u64;
        
        let result = invoke_raw!(
            hash,
                hudComponent
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_scaleform_script_hud_movie_loaded_raw(
        hudComponent: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF6E5987D2B4D140u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF6E5987D2B4D140u64;

        invoke_raw_typed!(
            hash,
                hudComponent
        )
    }
}

/// ```
NOTE: Debugging functions are not present in the retail version of the game.  
```



pub fn draw_debug_line_with_two_colours_safe(
        
        
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
        
        
            r1: 
        , 
        
        
            g1: 
        , 
        
        
            b1: 
        , 
        
        
            r2: 
        , 
        
        
            g2: 
        , 
        
        
            b2: 
        , 
        
        
            alpha1: 
        , 
        
        
            alpha2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8B9A8AC5608FF94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8B9A8AC5608FF94u64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                r1, 
                g1, 
                b1, 
                r2, 
                g2, 
                b2, 
                alpha1, 
                alpha2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_debug_line_with_two_colours_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        r1: , 
        g1: , 
        b1: , 
        r2: , 
        g2: , 
        b2: , 
        alpha1: , 
        alpha2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8B9A8AC5608FF94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8B9A8AC5608FF94u64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                r1, 
                g1, 
                b1, 
                r2, 
                g2, 
                b2, 
                alpha1, 
                alpha2
        )
    }
}

/// Sets the on-screen drawing origin for draw-functions in world coordinates.

The effect can be reset by calling [`CLEAR_DRAW_ORIGIN`](#_0xFF0B610F6BE0D7AF) and is limited to 32 different origins each frame.



pub fn set_draw_origin_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA0008F3BBB8F416u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA0008F3BBB8F416u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_draw_origin_raw(
        x: , 
        y: , 
        z: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA0008F3BBB8F416u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA0008F3BBB8F416u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn enable_movie_subtitles_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x873FA65C778AD970u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x873FA65C778AD970u64;
        
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
pub fn enable_movie_subtitles_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x873FA65C778AD970u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x873FA65C778AD970u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x5dbf05db5926d089_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DBF05DB5926D089u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DBF05DB5926D089u64;
        
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
pub fn _0x5dbf05db5926d089_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DBF05DB5926D089u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DBF05DB5926D089u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Only values used in the scripts are:
"heist_mp"
"heistmap_mp"
"instructional_buttons"
"heist_pre"
```



pub fn has_scaleform_movie_filename_loaded_safe(
        
        
            scaleformName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C1C5D756FB5F337u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C1C5D756FB5F337u64;
        
        let result = invoke_raw!(
            hash,
                scaleformName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_scaleform_movie_filename_loaded_raw(
        scaleformName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C1C5D756FB5F337u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C1C5D756FB5F337u64;

        invoke_raw_typed!(
            hash,
                scaleformName
        )
    }
}

/// Switches the rendering display to exclude everything except PostFX, resulting in a frozen screen before the UI pass.



pub fn toggle_paused_renderphases_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFC252D8A3E15AB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFC252D8A3E15AB7u64;
        
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
pub fn toggle_paused_renderphases_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFC252D8A3E15AB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFC252D8A3E15AB7u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value



pub fn begin_take_mission_creator_photo_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DD2139A9A20DCE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DD2139A9A20DCE8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_take_mission_creator_photo_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DD2139A9A20DCE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DD2139A9A20DCE8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Check if a Scaleform movie with the given name has been loaded.

```
NativeDB Introduced: v3407
```



pub fn _has_scaleform_movie_named_loaded_safe(
        
        
            scaleformHandle: 
        , 
        
        
            scaleformName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9743BCCF7CD6E1F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9743BCCF7CD6E1F6u64;
        
        let result = invoke_raw!(
            hash,
                scaleformHandle, 
                scaleformName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _has_scaleform_movie_named_loaded_raw(
        scaleformHandle: , 
        scaleformName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9743BCCF7CD6E1F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9743BCCF7CD6E1F6u64;

        invoke_raw_typed!(
            hash,
                scaleformHandle, 
                scaleformName
        )
    }
}

/// Requests a scaleform movie, which has no widescreen adjustments while rendering (Useful for when your scaleform doesn't fully draw on the screen and borders are visible).



pub fn request_scaleform_movie_with_ignore_super_widescreen_safe(
        
        
            scaleformName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65E7E78842E74CDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65E7E78842E74CDBu64;
        
        let result = invoke_raw!(
            hash,
                scaleformName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_scaleform_movie_with_ignore_super_widescreen_raw(
        scaleformName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65E7E78842E74CDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65E7E78842E74CDBu64;

        invoke_raw_typed!(
            hash,
                scaleformName
        )
    }
}

/// POP_TIMECYCLE_MODIFIER native function



pub fn pop_timecycle_modifier_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C8938D7D872211Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C8938D7D872211Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pop_timecycle_modifier_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C8938D7D872211Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C8938D7D872211Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_usingseethrough_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44B80ABAB9D80BD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44B80ABAB9D80BD3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_usingseethrough_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44B80ABAB9D80BD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44B80ABAB9D80BD3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
binkMovie: Is return value from _SET_BINK_MOVIE. Has something to do with bink volume? (audRequestedSettings::SetVolumeCurveScale)
```

```
NativeDB Introduced: v1290
```



pub fn _set_bink_movie_volume_safe(
        
        
            binkMovie: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFF33B1178172223u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFF33B1178172223u64;
        
        let result = invoke_raw!(
            hash,
                binkMovie, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_bink_movie_volume_raw(
        binkMovie: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFF33B1178172223u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFF33B1178172223u64;

        invoke_raw_typed!(
            hash,
                binkMovie, 
                value
        )
    }
}

/// ## Parameters
*



pub fn _0xef398beee4ef45f9_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF398BEEE4EF45F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF398BEEE4EF45F9u64;
        
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
pub fn _0xef398beee4ef45f9_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF398BEEE4EF45F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF398BEEE4EF45F9u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x36f6626459d91457_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36F6626459D91457u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36F6626459D91457u64;
        
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
pub fn _0x36f6626459d91457_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36F6626459D91457u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36F6626459D91457u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn get_timecycle_transition_modifier_index_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x459FD2C8D0AB78BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x459FD2C8D0AB78BCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_timecycle_transition_modifier_index_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x459FD2C8D0AB78BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x459FD2C8D0AB78BCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Stops the effect and sets a value (bool) in its data (+0x199) to false; See [`ANIMPOSTFX_PLAY`](#_0x2206BF9A37B7F724).



pub fn _animpostfx_stop_and_do_unk_safe(
        
        
            effectName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2209BE128B5418Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2209BE128B5418Cu64;
        
        let result = invoke_raw!(
            hash,
                effectName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _animpostfx_stop_and_do_unk_raw(
        effectName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2209BE128B5418Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2209BE128B5418Cu64;

        invoke_raw_typed!(
            hash,
                effectName
        )
    }
}

/// ```
SET_F*
```



pub fn _0x6a51f78772175a51_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A51F78772175A51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A51F78772175A51u64;
        
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
pub fn _0x6a51f78772175a51_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A51F78772175A51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A51F78772175A51u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// PUSH_TIMECYCLE_MODIFIER native function



pub fn push_timecycle_modifier_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58F735290861E6B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58F735290861E6B4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn push_timecycle_modifier_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58F735290861E6B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58F735290861E6B4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_screenblur_fade_current_time_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CCABFFCA31DDE33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CCABFFCA31DDE33u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_screenblur_fade_current_time_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CCABFFCA31DDE33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CCABFFCA31DDE33u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _seethrough_set_fade_end_distance_safe(
        
        
            distance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D75795B9DC6EBBFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D75795B9DC6EBBFu64;
        
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
pub fn _seethrough_set_fade_end_distance_raw(
        distance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D75795B9DC6EBBFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D75795B9DC6EBBFu64;

        invoke_raw_typed!(
            hash,
                distance
        )
    }
}

/// ## Parameters
*



pub fn disable_vehicle_distantlights_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9F98AC1884E73A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9F98AC1884E73A2u64;
        
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
pub fn disable_vehicle_distantlights_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9F98AC1884E73A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9F98AC1884E73A2u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn is_scaleform_movie_deleting_safe(
        
        
            scaleformIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x86255B1FC929E33Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x86255B1FC929E33Eu64;
        
        let result = invoke_raw!(
            hash,
                scaleformIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_scaleform_movie_deleting_raw(
        scaleformIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x86255B1FC929E33Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x86255B1FC929E33Eu64;

        invoke_raw_typed!(
            hash,
                scaleformIndex
        )
    }
}

/// Same as END_TEXT_COMMAND_SCALEFORM_STRING but does not perform HTML conversion for text tokens.
Also useful for when you are trying to add blips and inputs in your scaleform (If the scaleform supports it).



pub fn end_text_command_unparsed_scaleform_string_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE4E8157D9ECF087u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE4E8157D9ECF087u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_unparsed_scaleform_string_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE4E8157D9ECF087u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE4E8157D9ECF087u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _is_playlist_unk_safe(
        
        
            tvChannel: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F710BFF7DAE6261u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F710BFF7DAE6261u64;
        
        let result = invoke_raw!(
            hash,
                tvChannel, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_playlist_unk_raw(
        tvChannel: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F710BFF7DAE6261u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F710BFF7DAE6261u64;

        invoke_raw_typed!(
            hash,
                tvChannel, 
                p1
        )
    }
}

/// ```
IS_*
```



pub fn _is_tv_playlist_item_playing_safe(
        
        
            videoCliphash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AD973CA1E077B60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AD973CA1E077B60u64;
        
        let result = invoke_raw!(
            hash,
                videoCliphash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_tv_playlist_item_playing_raw(
        videoCliphash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AD973CA1E077B60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AD973CA1E077B60u64;

        invoke_raw_typed!(
            hash,
                videoCliphash
        )
    }
}

/// ```
Creates a checkpoint. Returns the handle of the checkpoint.  
20/03/17 : Attention, checkpoints are already handled by the game itself, so you must not loop it like markers.
Parameters:  
* type - The type of checkpoint to create. See below for a list of checkpoint types.  
* pos1 - The position of the checkpoint.  
* pos2 - The position of the next checkpoint to point to.  
* diameter - The diameter of the checkpoint.
* color - The color of the checkpoint.  
* reserved - Special parameter, see below for details. Usually set to 0 in the scripts.  
Checkpoint types (prior to game build 2189):  
0-4---------Cylinder: 1 arrow, 2 arrow, 3 arrows, CycleArrow, Checker  
5-9---------Cylinder: 1 arrow, 2 arrow, 3 arrows, CycleArrow, Checker  
10-14-------Ring: 1 arrow, 2 arrow, 3 arrows, CycleArrow, Checker  
15-19-------1 arrow, 2 arrow, 3 arrows, CycleArrow, Checker        
20-24-------Cylinder: 1 arrow, 2 arrow, 3 arrows, CycleArrow, Checker   
25-29-------Cylinder: 1 arrow, 2 arrow, 3 arrows, CycleArrow, Checker      
30-34-------Cylinder: 1 arrow, 2 arrow, 3 arrows, CycleArrow, Checker   
35-38-------Ring: Airplane Up, Left, Right, UpsideDown  
39----------?  
40----------Ring: just a ring  
41----------?  
42-44-------Cylinder w/ number (uses 'reserved' parameter)  
45-47-------Cylinder no arrow or number  
If using type 42-44, reserved sets number / number and shape to display  
0-99------------Just numbers (0-99)  
100-109-----------------Arrow (0-9)  
110-119------------Two arrows (0-9)  
120-129----------Three arrows (0-9)  
130-139----------------Circle (0-9)  
140-149------------CycleArrow (0-9)  
150-159----------------Circle (0-9)  
160-169----Circle  w/ pointer (0-9)  
170-179-------Perforated ring (0-9)  
180-189----------------Sphere (0-9)  
```

[Checkpoint Types](https://docs.fivem.net/docs/game-references/checkpoints/) as of game build 2189



pub fn create_checkpoint_safe(
        
        
            type: 
        , 
        
        
            posX1: 
        , 
        
        
            posY1: 
        , 
        
        
            posZ1: 
        , 
        
        
            posX2: 
        , 
        
        
            posY2: 
        , 
        
        
            posZ2: 
        , 
        
        
            diameter: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        , 
        
        
            reserved: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0134F0835AB6BFCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0134F0835AB6BFCBu64;
        
        let result = invoke_raw!(
            hash,
                type, 
                posX1, 
                posY1, 
                posZ1, 
                posX2, 
                posY2, 
                posZ2, 
                diameter, 
                red, 
                green, 
                blue, 
                alpha, 
                reserved
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_checkpoint_raw(
        type: , 
        posX1: , 
        posY1: , 
        posZ1: , 
        posX2: , 
        posY2: , 
        posZ2: , 
        diameter: , 
        red: , 
        green: , 
        blue: , 
        alpha: , 
        reserved: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0134F0835AB6BFCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0134F0835AB6BFCBu64;

        invoke_raw_typed!(
            hash,
                type, 
                posX1, 
                posY1, 
                posZ1, 
                posX2, 
                posY2, 
                posZ2, 
                diameter, 
                red, 
                green, 
                blue, 
                alpha, 
                reserved
        )
    }
}

/// ```
This function is hard-coded to always return 0.  
```



pub fn _0xec72c258667be5ea_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC72C258667BE5EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC72C258667BE5EAu64;
        
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
pub fn _0xec72c258667be5ea_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC72C258667BE5EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC72C258667BE5EAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn cascade_shadows_set_cascade_bounds_safe(
        
        
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
        let hash = 0xD2936CAB8B58FCBDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2936CAB8B58FCBDu64;
        
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
pub fn cascade_shadows_set_cascade_bounds_raw(
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
        let hash = 0xD2936CAB8B58FCBDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2936CAB8B58FCBDu64;

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

/// _0x346EF3ECAAAB149E native function



pub fn _0x346ef3ecaaab149e_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x346EF3ECAAAB149Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x346EF3ECAAAB149Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x346ef3ecaaab149e_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x346EF3ECAAAB149Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x346EF3ECAAAB149Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NOTE: Debugging functions are not present in the retail version of the game.  
```



pub fn draw_debug_sphere_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAD68E1AB39DA632u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAD68E1AB39DA632u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                red, 
                green, 
                blue, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_debug_sphere_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAD68E1AB39DA632u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAD68E1AB39DA632u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ```
duration - is how long to play the effect for in milliseconds. If 0, it plays the default length
if loop is true, the effect won't stop until you call ANIMPOSTFX_STOP on it. (only loopable effects)
```



pub fn animpostfx_play_safe(
        
        
            effectName: 
        , 
        
        
            duration: 
        , 
        
        
            looped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2206BF9A37B7F724u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2206BF9A37B7F724u64;
        
        let result = invoke_raw!(
            hash,
                effectName, 
                duration, 
                looped
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn animpostfx_play_raw(
        effectName: , 
        duration: , 
        looped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2206BF9A37B7F724u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2206BF9A37B7F724u64;

        invoke_raw_typed!(
            hash,
                effectName, 
                duration, 
                looped
        )
    }
}

/// ## Parameters
*



pub fn start_networked_particle_fx_non_looped_on_entity_safe(
        
        
            effectName: 
        , 
        
        
            entity: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            scale: 
        , 
        
        
            axisX: 
        , 
        
        
            axisY: 
        , 
        
        
            axisZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC95EB1DB6E92113Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC95EB1DB6E92113Du64;
        
        let result = invoke_raw!(
            hash,
                effectName, 
                entity, 
                offsetX, 
                offsetY, 
                offsetZ, 
                rotX, 
                rotY, 
                rotZ, 
                scale, 
                axisX, 
                axisY, 
                axisZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_networked_particle_fx_non_looped_on_entity_raw(
        effectName: , 
        entity: , 
        offsetX: , 
        offsetY: , 
        offsetZ: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        scale: , 
        axisX: , 
        axisY: , 
        axisZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC95EB1DB6E92113Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC95EB1DB6E92113Du64;

        invoke_raw_typed!(
            hash,
                effectName, 
                entity, 
                offsetX, 
                offsetY, 
                offsetZ, 
                rotX, 
                rotY, 
                rotZ, 
                scale, 
                axisX, 
                axisY, 
                axisZ
        )
    }
}

/// ## Parameters
*



pub fn remove_decal_safe(
        
        
            decal: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED3F346429CCD659u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED3F346429CCD659u64;
        
        let result = invoke_raw!(
            hash,
                decal
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_decal_raw(
        decal: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED3F346429CCD659u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED3F346429CCD659u64;

        invoke_raw_typed!(
            hash,
                decal
        )
    }
}

/// ```
Used only once in the scripts (taxi_clowncar)

SET_PARTICLE_FX_*
```



pub fn _0x8cde909a0370bb3a_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8CDE909A0370BB3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8CDE909A0370BB3Au64;
        
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
pub fn _0x8cde909a0370bb3a_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8CDE909A0370BB3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8CDE909A0370BB3Au64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_next_player_tcmodifier_safe(
        
        
            modifierName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF59707B3E5ED531u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF59707B3E5ED531u64;
        
        let result = invoke_raw!(
            hash,
                modifierName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_next_player_tcmodifier_raw(
        modifierName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF59707B3E5ED531u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF59707B3E5ED531u64;

        invoke_raw_typed!(
            hash,
                modifierName
        )
    }
}

/// ## Parameters
*



pub fn _0xca4ae345a153d573_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA4AE345A153D573u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA4AE345A153D573u64;
        
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
pub fn _0xca4ae345a153d573_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA4AE345A153D573u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA4AE345A153D573u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn start_particle_fx_looped_on_entity_safe(
        
        
            effectName: 
        , 
        
        
            entity: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
        , 
        
        
            xRot: 
        , 
        
        
            yRot: 
        , 
        
        
            zRot: 
        , 
        
        
            scale: 
        , 
        
        
            xAxis: 
        , 
        
        
            yAxis: 
        , 
        
        
            zAxis: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1AE42C1660FD6517u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1AE42C1660FD6517u64;
        
        let result = invoke_raw!(
            hash,
                effectName, 
                entity, 
                xOffset, 
                yOffset, 
                zOffset, 
                xRot, 
                yRot, 
                zRot, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_particle_fx_looped_on_entity_raw(
        effectName: , 
        entity: , 
        xOffset: , 
        yOffset: , 
        zOffset: , 
        xRot: , 
        yRot: , 
        zRot: , 
        scale: , 
        xAxis: , 
        yAxis: , 
        zAxis: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1AE42C1660FD6517u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1AE42C1660FD6517u64;

        invoke_raw_typed!(
            hash,
                effectName, 
                entity, 
                xOffset, 
                yOffset, 
                zOffset, 
                xRot, 
                yRot, 
                zRot, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis
        )
    }
}

/// ## Parameters
*



pub fn start_networked_particle_fx_non_looped_on_ped_bone_safe(
        
        
            effectName: 
        , 
        
        
            ped: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        , 
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            boneIndex: 
        , 
        
        
            scale: 
        , 
        
        
            axisX: 
        , 
        
        
            axisY: 
        , 
        
        
            axisZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA41B6A43642AC2CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA41B6A43642AC2CFu64;
        
        let result = invoke_raw!(
            hash,
                effectName, 
                ped, 
                offsetX, 
                offsetY, 
                offsetZ, 
                rotX, 
                rotY, 
                rotZ, 
                boneIndex, 
                scale, 
                axisX, 
                axisY, 
                axisZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_networked_particle_fx_non_looped_on_ped_bone_raw(
        effectName: , 
        ped: , 
        offsetX: , 
        offsetY: , 
        offsetZ: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        boneIndex: , 
        scale: , 
        axisX: , 
        axisY: , 
        axisZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA41B6A43642AC2CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA41B6A43642AC2CFu64;

        invoke_raw_typed!(
            hash,
                effectName, 
                ped, 
                offsetX, 
                offsetY, 
                offsetZ, 
                rotX, 
                rotY, 
                rotZ, 
                boneIndex, 
                scale, 
                axisX, 
                axisY, 
                axisZ
        )
    }
}

/// CASCADE_SHADOWS_INIT_SESSION native function



pub fn cascade_shadows_init_session_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03FC694AE06C5A20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03FC694AE06C5A20u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn cascade_shadows_init_session_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03FC694AE06C5A20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03FC694AE06C5A20u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Places a decal into the world

```cs
public enum DecalTypes  
{  
    splatters_blood = 1010,  
    splatters_blood_dir = 1015,  
    splatters_blood_mist = 1017,  
    splatters_mud = 1020,  
    splatters_paint = 1030,  
    splatters_water = 1040,  
    splatters_water_hydrant = 1050,  
    splatters_blood2 = 1110,  
    weapImpact_metal = 4010,  
    weapImpact_concrete = 4020,  
    weapImpact_mattress = 4030,  
    weapImpact_mud = 4032,  
    weapImpact_wood = 4050,  
    weapImpact_sand = 4053,  
    weapImpact_cardboard = 4040,  
    weapImpact_melee_glass = 4100,  
    weapImpact_glass_blood = 4102,  
    weapImpact_glass_blood2 = 4104,  
    weapImpact_shotgun_paper = 4200,  
    weapImpact_shotgun_mattress,  
    weapImpact_shotgun_metal,  
    weapImpact_shotgun_wood,  
    weapImpact_shotgun_dirt,  
    weapImpact_shotgun_tvscreen,  
    weapImpact_shotgun_tvscreen2,  
    weapImpact_shotgun_tvscreen3,  
    weapImpact_melee_concrete = 4310,  
    weapImpact_melee_wood = 4312,  
    weapImpact_melee_metal = 4314,  
    burn1 = 4421,  
    burn2,  
    burn3,  
    burn4,  
    burn5,  
    bang_concrete_bang = 5000,  
    bang_concrete_bang2,  
    bang_bullet_bang,  
    bang_bullet_bang2 = 5004,  
    bang_glass = 5031,  
    bang_glass2,  
    solidPool_water = 9000,  
    solidPool_blood,  
    solidPool_oil,  
    solidPool_petrol,  
    solidPool_mud,  
    porousPool_water,  
    porousPool_blood,  
    porousPool_oil,  
    porousPool_petrol,  
    porousPool_mud,  
    porousPool_water_ped_drip,  
    liquidTrail_water = 9050  
}  
```



pub fn add_decal_safe(
        
        
            decalType: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            dirX: 
        , 
        
        
            dirY: 
        , 
        
        
            dirZ: 
        , 
        
        
            sideX: 
        , 
        
        
            sideY: 
        , 
        
        
            sideZ: 
        , 
        
        
            width: 
        , 
        
        
            height: 
        , 
        
        
            rCoef: 
        , 
        
        
            gCoef: 
        , 
        
        
            bCoef: 
        , 
        
        
            opacity: 
        , 
        
        
            timeout: 
        , 
        
        
            isLongRange: 
        , 
        
        
            isDynamic: 
        , 
        
        
            useComplexColn: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB302244A1839BDADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB302244A1839BDADu64;
        
        let result = invoke_raw!(
            hash,
                decalType, 
                posX, 
                posY, 
                posZ, 
                dirX, 
                dirY, 
                dirZ, 
                sideX, 
                sideY, 
                sideZ, 
                width, 
                height, 
                rCoef, 
                gCoef, 
                bCoef, 
                opacity, 
                timeout, 
                isLongRange, 
                isDynamic, 
                useComplexColn
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_decal_raw(
        decalType: , 
        posX: , 
        posY: , 
        posZ: , 
        dirX: , 
        dirY: , 
        dirZ: , 
        sideX: , 
        sideY: , 
        sideZ: , 
        width: , 
        height: , 
        rCoef: , 
        gCoef: , 
        bCoef: , 
        opacity: , 
        timeout: , 
        isLongRange: , 
        isDynamic: , 
        useComplexColn: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB302244A1839BDADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB302244A1839BDADu64;

        invoke_raw_typed!(
            hash,
                decalType, 
                posX, 
                posY, 
                posZ, 
                dirX, 
                dirY, 
                dirZ, 
                sideX, 
                sideY, 
                sideZ, 
                width, 
                height, 
                rCoef, 
                gCoef, 
                bCoef, 
                opacity, 
                timeout, 
                isLongRange, 
                isDynamic, 
                useComplexColn
        )
    }
}

/// If "blackout" is enabled, this native allows you to ignore "blackout" for vehicles.

```
NativeDB Introduced: v2060
```



pub fn _set_artificial_lights_state_affects_vehicles_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2B187C0939B3D32u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2B187C0939B3D32u64;
        
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
pub fn _set_artificial_lights_state_affects_vehicles_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2B187C0939B3D32u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2B187C0939B3D32u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// _0x0218BA067D249DEA native function



pub fn _0x0218ba067d249dea_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0218BA067D249DEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0218BA067D249DEAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x0218ba067d249dea_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0218BA067D249DEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0218BA067D249DEAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _play_bink_movie_safe(
        
        
            binkMovie: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70D2CC8A542A973Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70D2CC8A542A973Cu64;
        
        let result = invoke_raw!(
            hash,
                binkMovie
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _play_bink_movie_raw(
        binkMovie: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70D2CC8A542A973Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70D2CC8A542A973Cu64;

        invoke_raw_typed!(
            hash,
                binkMovie
        )
    }
}

/// ## Parameters
*



pub fn _draw_spot_light_with_shadow_safe(
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            dirX: 
        , 
        
        
            dirY: 
        , 
        
        
            dirZ: 
        , 
        
        
            colorR: 
        , 
        
        
            colorG: 
        , 
        
        
            colorB: 
        , 
        
        
            distance: 
        , 
        
        
            brightness: 
        , 
        
        
            roundness: 
        , 
        
        
            radius: 
        , 
        
        
            falloff: 
        , 
        
        
            shadowId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BCA583A583194DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BCA583A583194DBu64;
        
        let result = invoke_raw!(
            hash,
                posX, 
                posY, 
                posZ, 
                dirX, 
                dirY, 
                dirZ, 
                colorR, 
                colorG, 
                colorB, 
                distance, 
                brightness, 
                roundness, 
                radius, 
                falloff, 
                shadowId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _draw_spot_light_with_shadow_raw(
        posX: , 
        posY: , 
        posZ: , 
        dirX: , 
        dirY: , 
        dirZ: , 
        colorR: , 
        colorG: , 
        colorB: , 
        distance: , 
        brightness: , 
        roundness: , 
        radius: , 
        falloff: , 
        shadowId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BCA583A583194DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BCA583A583194DBu64;

        invoke_raw_typed!(
            hash,
                posX, 
                posY, 
                posZ, 
                dirX, 
                dirY, 
                dirZ, 
                colorR, 
                colorG, 
                colorB, 
                distance, 
                brightness, 
                roundness, 
                radius, 
                falloff, 
                shadowId
        )
    }
}

/// Resets AnimPostFX adaptation.



pub fn reset_adaptation_safe(
        
        
            numFrames: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3E2C1B4C59DBC77u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3E2C1B4C59DBC77u64;
        
        let result = invoke_raw!(
            hash,
                numFrames
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_adaptation_raw(
        numFrames: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3E2C1B4C59DBC77u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3E2C1B4C59DBC77u64;

        invoke_raw_typed!(
            hash,
                numFrames
        )
    }
}

/// ## Parameters
*



pub fn _0x46d1a61a21f566fc_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46D1A61A21F566FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46D1A61A21F566FCu64;
        
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
pub fn _0x46d1a61a21f566fc_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46D1A61A21F566FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46D1A61A21F566FCu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// FREE_MEMORY_FOR_MISSION_CREATOR_PHOTO native function



pub fn free_memory_for_mission_creator_photo_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A46AF8A78DC5E0Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A46AF8A78DC5E0Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn free_memory_for_mission_creator_photo_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A46AF8A78DC5E0Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A46AF8A78DC5E0Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x4AF92ACD3141D96C native function



pub fn _0x4af92acd3141d96c_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AF92ACD3141D96Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AF92ACD3141D96Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4af92acd3141d96c_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AF92ACD3141D96Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AF92ACD3141D96Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Clears the secondary timecycle modifier usually set with [`SetExtraTimecycleModifier`](#_0x5096FD9CCB49056D)



pub fn _clear_extra_timecycle_modifier_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92CCC17A7A2285DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92CCC17A7A2285DAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _clear_extra_timecycle_modifier_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92CCC17A7A2285DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92CCC17A7A2285DAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// This native indicates whether the game is running in high-definition (HD) resolution. It returns `false` if the resolution is less than `1280x720` and `true` if it's equal to or greater than `1280x720`.

```
NativeDB Introduced: v323
```



pub fn get_is_hidef_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84ED31191CC5D2C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84ED31191CC5D2C9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_is_hidef_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84ED31191CC5D2C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84ED31191CC5D2C9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
SET_TRA*
```



pub fn _0x1cba05ae7bd7ee05_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CBA05AE7BD7EE05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CBA05AE7BD7EE05u64;
        
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
pub fn _0x1cba05ae7bd7ee05_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CBA05AE7BD7EE05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CBA05AE7BD7EE05u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn draw_low_quality_photo_to_phone_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1072F115DAB0717Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1072F115DAB0717Eu64;
        
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
pub fn draw_low_quality_photo_to_phone_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1072F115DAB0717Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1072F115DAB0717Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Queues a scan of all gallery photos.
Also see [`GET_STATUS_OF_SORTED_LIST_OPERATION`](#_0xF5BED327CEA362B1)



pub fn queue_operation_to_create_sorted_list_of_photos_safe(
        
        
            scanForSaving: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A893980E96B659Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A893980E96B659Au64;
        
        let result = invoke_raw!(
            hash,
                scanForSaving
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn queue_operation_to_create_sorted_list_of_photos_raw(
        scanForSaving: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A893980E96B659Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A893980E96B659Au64;

        invoke_raw_typed!(
            hash,
                scanForSaving
        )
    }
}

/// ## Parameters
*



pub fn draw_scaleform_movie_3d_solid_safe(
        
        
            scaleform: 
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
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            p9: 
        , 
        
        
            scaleX: 
        , 
        
        
            scaleY: 
        , 
        
        
            scaleZ: 
        , 
        
        
            p13: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CE592FDC749D6F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CE592FDC749D6F5u64;
        
        let result = invoke_raw!(
            hash,
                scaleform, 
                posX, 
                posY, 
                posZ, 
                rotX, 
                rotY, 
                rotZ, 
                p7, 
                p8, 
                p9, 
                scaleX, 
                scaleY, 
                scaleZ, 
                p13
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_scaleform_movie_3d_solid_raw(
        scaleform: , 
        posX: , 
        posY: , 
        posZ: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        p7: , 
        p8: , 
        p9: , 
        scaleX: , 
        scaleY: , 
        scaleZ: , 
        p13: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CE592FDC749D6F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CE592FDC749D6F5u64;

        invoke_raw_typed!(
            hash,
                scaleform, 
                posX, 
                posY, 
                posZ, 
                rotX, 
                rotY, 
                rotZ, 
                p7, 
                p8, 
                p9, 
                scaleX, 
                scaleY, 
                scaleZ, 
                p13
        )
    }
}

/// ## Parameters
*



pub fn start_particle_fx_looped_on_entity_bone_safe(
        
        
            effectName: 
        , 
        
        
            entity: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
        , 
        
        
            xRot: 
        , 
        
        
            yRot: 
        , 
        
        
            zRot: 
        , 
        
        
            boneIndex: 
        , 
        
        
            scale: 
        , 
        
        
            xAxis: 
        , 
        
        
            yAxis: 
        , 
        
        
            zAxis: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6EB449E33977F0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6EB449E33977F0Bu64;
        
        let result = invoke_raw!(
            hash,
                effectName, 
                entity, 
                xOffset, 
                yOffset, 
                zOffset, 
                xRot, 
                yRot, 
                zRot, 
                boneIndex, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_particle_fx_looped_on_entity_bone_raw(
        effectName: , 
        entity: , 
        xOffset: , 
        yOffset: , 
        zOffset: , 
        xRot: , 
        yRot: , 
        zRot: , 
        boneIndex: , 
        scale: , 
        xAxis: , 
        yAxis: , 
        zAxis: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6EB449E33977F0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6EB449E33977F0Bu64;

        invoke_raw_typed!(
            hash,
                effectName, 
                entity, 
                xOffset, 
                yOffset, 
                zOffset, 
                xRot, 
                yRot, 
                zRot, 
                boneIndex, 
                scale, 
                xAxis, 
                yAxis, 
                zAxis
        )
    }
}

/// ## Parameters
*



pub fn set_hidof_override_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            nearplaneOut: 
        , 
        
        
            nearplaneIn: 
        , 
        
        
            farplaneOut: 
        , 
        
        
            farplaneIn: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA3D65906822BED5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA3D65906822BED5u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                nearplaneOut, 
                nearplaneIn, 
                farplaneOut, 
                farplaneIn
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_hidof_override_raw(
        p0: , 
        p1: , 
        nearplaneOut: , 
        nearplaneIn: , 
        farplaneOut: , 
        farplaneIn: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA3D65906822BED5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA3D65906822BED5u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                nearplaneOut, 
                nearplaneIn, 
                farplaneOut, 
                farplaneIn
        )
    }
}

/// ## Parameters
*



pub fn set_particle_fx_looped_scale_safe(
        
        
            ptfxHandle: 
        , 
        
        
            scale: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB44250AAA456492Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB44250AAA456492Du64;
        
        let result = invoke_raw!(
            hash,
                ptfxHandle, 
                scale
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_particle_fx_looped_scale_raw(
        ptfxHandle: , 
        scale: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB44250AAA456492Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB44250AAA456492Du64;

        invoke_raw_typed!(
            hash,
                ptfxHandle, 
                scale
        )
    }
}

/// Values:
0 - Dialogue Brief
1 - Help Text Brief
2 - Mission Objective Brief



pub fn scaleform_movie_method_add_param_latest_brief_string_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC52C631A1831C03u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC52C631A1831C03u64;
        
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
pub fn scaleform_movie_method_add_param_latest_brief_string_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC52C631A1831C03u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC52C631A1831C03u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// methodReturn: The return value of this native: END_SCALEFORM_MOVIE_METHOD_RETURN_VALUE
Returns true if the return value of a scaleform function is ready to be collected (using GET_SCALEFORM_MOVIE_METHOD_RETURN_VALUE_STRING or GET_SCALEFORM_MOVIE_METHOD_RETURN_VALUE_INT).



pub fn is_scaleform_movie_method_return_value_ready_safe(
        
        
            method_return: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x768FF8961BA904D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x768FF8961BA904D6u64;
        
        let result = invoke_raw!(
            hash,
                method_return
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_scaleform_movie_method_return_value_ready_raw(
        method_return: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x768FF8961BA904D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x768FF8961BA904D6u64;

        invoke_raw_typed!(
            hash,
                method_return
        )
    }
}

/// ```
REQUEST_*
```



pub fn _0x98edf76a7271e4f2_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98EDF76A7271E4F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98EDF76A7271E4F2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x98edf76a7271e4f2_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98EDF76A7271E4F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98EDF76A7271E4F2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_requestingnightvision_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35FB78DC42B7BD21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35FB78DC42B7BD21u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_requestingnightvision_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35FB78DC42B7BD21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35FB78DC42B7BD21u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_toggle_paused_renderphases_status_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB3DAC2C86001E5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB3DAC2C86001E5Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_toggle_paused_renderphases_status_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB3DAC2C86001E5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB3DAC2C86001E5Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn remove_scaleform_script_hud_movie_safe(
        
        
            hudComponent: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF44A5456AC3F4F97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF44A5456AC3F4F97u64;
        
        let result = invoke_raw!(
            hash,
                hudComponent
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_scaleform_script_hud_movie_raw(
        hudComponent: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF44A5456AC3F4F97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF44A5456AC3F4F97u64;

        invoke_raw_typed!(
            hash,
                hudComponent
        )
    }
}

/// ```
Returns whether screen transition to blur/from blur is running.
```



pub fn is_screenblur_fade_running_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B226C785A52A0A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B226C785A52A0A9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_screenblur_fade_running_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B226C785A52A0A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B226C785A52A0A9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Sets the colour tint of a previously started looped particle effect

You can use the [inverse lerp](https://www.gamedev.net/articles/programming/general-and-gameplay-programming/inverse-lerp-a-super-useful-yet-often-overlooked-function-r5230/) method to normalize in a range from 0.0 to 1.0 an rgb



pub fn set_particle_fx_looped_colour_safe(
        
        
            ptfxHandle: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        , 
        
        
            bLocalOnly: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F8F65877F88783Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F8F65877F88783Bu64;
        
        let result = invoke_raw!(
            hash,
                ptfxHandle, 
                r, 
                g, 
                b, 
                bLocalOnly
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_particle_fx_looped_colour_raw(
        ptfxHandle: , 
        r: , 
        g: , 
        b: , 
        bLocalOnly: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F8F65877F88783Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F8F65877F88783Bu64;

        invoke_raw_typed!(
            hash,
                ptfxHandle, 
                r, 
                g, 
                b, 
                bLocalOnly
        )
    }
}

/// ## Parameters
*



pub fn _0x259ba6d4e6f808f1_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x259BA6D4E6F808F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x259BA6D4E6F808F1u64;
        
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
pub fn _0x259ba6d4e6f808f1_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x259BA6D4E6F808F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x259BA6D4E6F808F1u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
DISABLE_*
```



pub fn _0x5f6df3d92271e8a1_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F6DF3D92271E8A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F6DF3D92271E8A1u64;
        
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
pub fn _0x5f6df3d92271e8a1_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F6DF3D92271E8A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F6DF3D92271E8A1u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_decal_wash_level_safe(
        
        
            decal: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x323F647679A09103u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x323F647679A09103u64;
        
        let result = invoke_raw!(
            hash,
                decal
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_decal_wash_level_raw(
        decal: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x323F647679A09103u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x323F647679A09103u64;

        invoke_raw_typed!(
            hash,
                decal
        )
    }
}

/// ## Parameters
*



pub fn _0xcb82a0bf0e3e3265_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB82A0BF0E3E3265u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB82A0BF0E3E3265u64;
        
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
pub fn _0xcb82a0bf0e3e3265_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB82A0BF0E3E3265u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB82A0BF0E3E3265u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_current_player_tcmodifier_safe(
        
        
            modifierName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBF327DED94E4DEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBF327DED94E4DEBu64;
        
        let result = invoke_raw!(
            hash,
                modifierName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_current_player_tcmodifier_raw(
        modifierName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBF327DED94E4DEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBF327DED94E4DEBu64;

        invoke_raw_typed!(
            hash,
                modifierName
        )
    }
}

