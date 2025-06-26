//! MOBILE native functions
//! 
//! Functions for the mobile category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn cell_cam_is_char_visible_no_face_check_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x439E9BC95B7E7FBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x439E9BC95B7E7FBEu64;
        
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
pub fn cell_cam_is_char_visible_no_face_check_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x439E9BC95B7E7FBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x439E9BC95B7E7FBEu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn _cell_cam_set_head_height_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x466DA42C89865553u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x466DA42C89865553u64;
        
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
pub fn _cell_cam_set_head_height_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x466DA42C89865553u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x466DA42C89865553u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn get_mobile_phone_position_safe(
        
        
            position: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x584FDFDA48805B86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x584FDFDA48805B86u64;
        
        let result = invoke_raw!(
            hash,
                position
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_mobile_phone_position_raw(
        position: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x584FDFDA48805B86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x584FDFDA48805B86u64;

        invoke_raw_typed!(
            hash,
                position
        )
    }
}

/// ## Parameters
*



pub fn _cell_cam_set_head_pitch_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6ADE981781FCA09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6ADE981781FCA09u64;
        
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
pub fn _cell_cam_set_head_pitch_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6ADE981781FCA09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6ADE981781FCA09u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
If bool Toggle = true so the mobile is hide to screen.  
If bool Toggle = false so the mobile is show to screen.  
```



pub fn script_is_moving_mobile_phone_offscreen_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF511F759238A5122u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF511F759238A5122u64;
        
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
pub fn script_is_moving_mobile_phone_offscreen_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF511F759238A5122u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF511F759238A5122u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _cell_cam_set_vertical_offset_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3117D84EFA60F77Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3117D84EFA60F77Bu64;
        
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
pub fn _cell_cam_set_vertical_offset_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3117D84EFA60F77Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3117D84EFA60F77Bu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Destroys the currently active mobile phone.  
```



pub fn destroy_mobile_phone_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BC861DF703E5097u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BC861DF703E5097u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn destroy_mobile_phone_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BC861DF703E5097u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BC861DF703E5097u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xac2890471901861c_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC2890471901861Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC2890471901861Cu64;
        
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
pub fn _0xac2890471901861c_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC2890471901861Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC2890471901861Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _cell_cam_set_head_roll_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1E22DC13F5EEBADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1E22DC13F5EEBADu64;
        
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
pub fn _cell_cam_set_head_roll_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1E22DC13F5EEBADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1E22DC13F5EEBADu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Creates a mobile phone of the specified type.  
Possible phone types:  
0 - Default phone / Michael's phone  
1 - Trevor's phone  
2 - Franklin's phone  
4 - Prologue phone  
These values represent bit flags, so a value of '3' would toggle Trevor and Franklin's phones together, causing unexpected behavior and most likely crash the game.  
```



pub fn create_mobile_phone_safe(
        
        
            phoneType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4E8E696C532FBC7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4E8E696C532FBC7u64;
        
        let result = invoke_raw!(
            hash,
                phoneType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_mobile_phone_raw(
        phoneType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4E8E696C532FBC7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4E8E696C532FBC7u64;

        invoke_raw_typed!(
            hash,
                phoneType
        )
    }
}

/// ## Parameters
*



pub fn get_mobile_phone_render_id_safe(
        
        
            renderId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4A53E05F68B6FA1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4A53E05F68B6FA1u64;
        
        let result = invoke_raw!(
            hash,
                renderId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_mobile_phone_render_id_raw(
        renderId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4A53E05F68B6FA1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4A53E05F68B6FA1u64;

        invoke_raw_typed!(
            hash,
                renderId
        )
    }
}

/// ## Parameters
*



pub fn _cell_cam_set_distance_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53F4892D18EC90A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53F4892D18EC90A4u64;
        
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
pub fn _cell_cam_set_distance_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53F4892D18EC90A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53F4892D18EC90A4u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Activates the cellphone camera. Make sure you have a mobile phone created with [`CREATE_MOBILE_PHONE`](#_0xA4E8E696C532FBC7) or else the camera will not work.



pub fn cell_cam_activate_safe(
        
        
            active: 
        , 
        
        
            bGoFirstPerson: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDE8F069C542D126u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDE8F069C542D126u64;
        
        let result = invoke_raw!(
            hash,
                active, 
                bGoFirstPerson
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn cell_cam_activate_raw(
        active: , 
        bGoFirstPerson: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDE8F069C542D126u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDE8F069C542D126u64;

        invoke_raw_typed!(
            hash,
                active, 
                bGoFirstPerson
        )
    }
}

/// ```
Needs more research. If the "phone_cam12" filter is applied, this function is called with "TRUE"; otherwise, "FALSE".
Example (XBOX 360):
// check current filter selection
if (MISC::ARE_STRINGS_EQUAL(getElem(g_2471024, &l_17, 4), "phone_cam12") != 0)
{
    MOBILE::_0xC273BB4D(0); // FALSE
}
else
{
    MOBILE::_0xC273BB4D(1); // TRUE
}
```



pub fn _0xa2ccbe62cd4c91a4_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2CCBE62CD4C91A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2CCBE62CD4C91A4u64;
        
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
pub fn _0xa2ccbe62cd4c91a4_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2CCBE62CD4C91A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2CCBE62CD4C91A4u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_mobile_phone_rotation_safe(
        
        
            rotation: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CEFB61F193070AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CEFB61F193070AEu64;
        
        let result = invoke_raw!(
            hash,
                rotation, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_mobile_phone_rotation_raw(
        rotation: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CEFB61F193070AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CEFB61F193070AEu64;

        invoke_raw_typed!(
            hash,
                rotation, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _cell_cam_set_horizontal_offset_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B0B4AEED5B9B41Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B0B4AEED5B9B41Cu64;
        
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
pub fn _cell_cam_set_horizontal_offset_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B0B4AEED5B9B41Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B0B4AEED5B9B41Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_mobile_phone_position_safe(
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x693A5C6D6734085Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x693A5C6D6734085Bu64;
        
        let result = invoke_raw!(
            hash,
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
pub fn set_mobile_phone_position_raw(
        posX: , 
        posY: , 
        posZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x693A5C6D6734085Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x693A5C6D6734085Bu64;

        invoke_raw_typed!(
            hash,
                posX, 
                posY, 
                posZ
        )
    }
}

/// ```
This one is weird and seems to return a TRUE state regardless of whether the phone is visible on screen or tucked away.  
I can confirm the above. This function is hard-coded to always return 1.  
```



pub fn can_phone_be_seen_on_screen_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4E2813898C97A4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4E2813898C97A4Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_phone_be_seen_on_screen_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4E2813898C97A4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4E2813898C97A4Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
The minimum/default is 500.0f. If you plan to make it bigger set it's position as well. Also this seems to need to be called in a loop as when you close the phone the scale is reset. If not in a loop you'd need to call it everytime before you re-open the phone.  
```



pub fn set_mobile_phone_scale_safe(
        
        
            scale: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCBDD322A73D6D932u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCBDD322A73D6D932u64;
        
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
pub fn set_mobile_phone_scale_raw(
        scale: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCBDD322A73D6D932u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCBDD322A73D6D932u64;

        invoke_raw_typed!(
            hash,
                scale
        )
    }
}

/// ## Parameters
*



pub fn _cell_cam_set_roll_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15E69E2802C24B8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15E69E2802C24B8Du64;
        
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
pub fn _cell_cam_set_roll_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15E69E2802C24B8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15E69E2802C24B8Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
if the bool "Toggle" is "true" so the phone is lean.  
if the bool "Toggle" is "false" so the phone is not lean.  
```



pub fn _cell_cam_set_lean_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44E44169EF70138Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44E44169EF70138Eu64;
        
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
pub fn _cell_cam_set_lean_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44E44169EF70138Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44E44169EF70138Eu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Toggles the selfie mode on the cellphone camera. Only visible when the cell phone camera is active.



pub fn cell_cam_activate_selfie_mode_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x015C49A93E3E086Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x015C49A93E3E086Eu64;
        
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
pub fn cell_cam_activate_selfie_mode_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x015C49A93E3E086Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x015C49A93E3E086Eu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Toggles depth of field on the cellphone camera.



pub fn set_mobile_phone_dof_state_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x375A706A5C2FD084u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x375A706A5C2FD084u64;
        
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
pub fn set_mobile_phone_dof_state_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x375A706A5C2FD084u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x375A706A5C2FD084u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Last parameter is unknown and always zero.  
```



pub fn set_mobile_phone_rotation_safe(
        
        
            rotX: 
        , 
        
        
            rotY: 
        , 
        
        
            rotZ: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB779C0CA917E865u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB779C0CA917E865u64;
        
        let result = invoke_raw!(
            hash,
                rotX, 
                rotY, 
                rotZ, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mobile_phone_rotation_raw(
        rotX: , 
        rotY: , 
        rotZ: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB779C0CA917E865u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB779C0CA917E865u64;

        invoke_raw_typed!(
            hash,
                rotX, 
                rotY, 
                rotZ, 
                p3
        )
    }
}

/// Moves the character's finger in a swiping motion when holding a cellphone in their hand through the use of the [CREATE_MOBILE_PHONE](#_0xA4E8E696C532FBC7) native.

```c
enum eCellInput {
    CELL_INPUT_NONE = 0,
    CELL_INPUT_UP = 1,
    CELL_INPUT_DOWN = 2,
    CELL_INPUT_LEFT = 3,
    CELL_INPUT_RIGHT = 4,
    CELL_INPUT_SELECT = 5
}
```



pub fn _cell_cam_move_finger_safe(
        
        
            direction: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95C9E72F3D7DEC9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95C9E72F3D7DEC9Bu64;
        
        let result = invoke_raw!(
            hash,
                direction
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _cell_cam_move_finger_raw(
        direction: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95C9E72F3D7DEC9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95C9E72F3D7DEC9Bu64;

        invoke_raw_typed!(
            hash,
                direction
        )
    }
}

