//! HUD native functions
//! 
//! Functions for the hud category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Return value



pub fn _get_north_radar_blip_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F0CF9CB7E589B88u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F0CF9CB7E589B88u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_north_radar_blip_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F0CF9CB7E589B88u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F0CF9CB7E589B88u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Enables or disables the blue half circle ![](https://i.imgur.com/iZes9Ec.png) around the specified blip on the left side of the blip. This is used to indicate that the player is in your crew in GTA:O. Color is changeable by using [`SET_BLIP_SECONDARY_COLOUR`](#_0x14892474891E09EB).

To toggle the right side of the circle use: [`SHOW_FRIEND_INDICATOR_ON_BLIP`](#_0x23C3EB807312F01A).

Example code result:
![](https://i.imgur.com/iZ9tNWl.png)



pub fn show_crew_indicator_on_blip_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCFB5D4DB8BF367Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCFB5D4DB8BF367Eu64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn show_crew_indicator_on_blip_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCFB5D4DB8BF367Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCFB5D4DB8BF367Eu64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// THEFEED_SPS_EXTEND_WIDESCREEN_OFF native function



pub fn thefeed_sps_extend_widescreen_off_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB695E2CD0A2DA9EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB695E2CD0A2DA9EEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_sps_extend_widescreen_off_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB695E2CD0A2DA9EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB695E2CD0A2DA9EEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _get_closest_blip_of_type_safe(
        
        
            blipSprite: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD484BF71050CA1EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD484BF71050CA1EEu64;
        
        let result = invoke_raw!(
            hash,
                blipSprite
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_closest_blip_of_type_raw(
        blipSprite: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD484BF71050CA1EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD484BF71050CA1EEu64;

        invoke_raw_typed!(
            hash,
                blipSprite
        )
    }
}

/// ## Parameters
*



pub fn clear_help_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DFCED7A656F8802u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DFCED7A656F8802u64;
        
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
pub fn clear_help_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DFCED7A656F8802u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DFCED7A656F8802u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Adds a float to a text component placeholder, replacing `~1~` in the current text command's text label.

![Example output](https://i.imgur.com/jvuQ0II.png)



pub fn add_text_component_float_safe(
        
        
            value: 
        , 
        
        
            decimalPlaces: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7DCB5B874BCD96Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7DCB5B874BCD96Eu64;
        
        let result = invoke_raw!(
            hash,
                value, 
                decimalPlaces
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_text_component_float_raw(
        value: , 
        decimalPlaces: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7DCB5B874BCD96Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7DCB5B874BCD96Eu64;

        invoke_raw_typed!(
            hash,
                value, 
                decimalPlaces
        )
    }
}

/// ## Parameters
*



pub fn _0x24a49beaf468dc90_safe(
        
        
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
        let hash = 0x24A49BEAF468DC90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24A49BEAF468DC90u64;
        
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
pub fn _0x24a49beaf468dc90_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24A49BEAF468DC90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24A49BEAF468DC90u64;

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
Calling this each frame, stops the player from receiving a weapon via the weapon wheel.
```



pub fn _hud_weapon_wheel_ignore_selection_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AFC4AF510774B47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AFC4AF510774B47u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _hud_weapon_wheel_ignore_selection_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AFC4AF510774B47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AFC4AF510774B47u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_pause_menu_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB0034A223497FFCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB0034A223497FFCBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_pause_menu_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB0034A223497FFCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB0034A223497FFCBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn add_point_to_gps_multi_route_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA905192A6781C41Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA905192A6781C41Bu64;
        
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
pub fn add_point_to_gps_multi_route_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA905192A6781C41Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA905192A6781C41Bu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn set_multiplayer_hud_cash_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD1D220394BCB824u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD1D220394BCB824u64;
        
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
pub fn set_multiplayer_hud_cash_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD1D220394BCB824u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD1D220394BCB824u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_blip_as_mission_creator_blip_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24AC0137444F9FD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24AC0137444F9FD5u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_as_mission_creator_blip_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24AC0137444F9FD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24AC0137444F9FD5u64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_max_health_hud_display_safe(
        
        
            maximumValue: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x975D66A0BC17064Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x975D66A0BC17064Cu64;
        
        let result = invoke_raw!(
            hash,
                maximumValue
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_max_health_hud_display_raw(
        maximumValue: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x975D66A0BC17064Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x975D66A0BC17064Cu64;

        invoke_raw_typed!(
            hash,
                maximumValue
        )
    }
}

/// ## Parameters
*



pub fn _0x7c226d5346d4d10a_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C226D5346D4D10Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C226D5346D4D10Au64;
        
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
pub fn _0x7c226d5346d4d10a_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C226D5346D4D10Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C226D5346D4D10Au64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_text_centre_safe(
        
        
            align: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC02F4DBFB51D988Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC02F4DBFB51D988Bu64;
        
        let result = invoke_raw!(
            hash,
                align
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_text_centre_raw(
        align: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC02F4DBFB51D988Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC02F4DBFB51D988Bu64;

        invoke_raw_typed!(
            hash,
                align
        )
    }
}

/// THEFEED_CLEAR_FROZEN_POST native function



pub fn thefeed_clear_frozen_post_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80FE4F3AB4E1B62Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80FE4F3AB4E1B62Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_clear_frozen_post_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80FE4F3AB4E1B62Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80FE4F3AB4E1B62Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_hovering_over_mission_creator_blip_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4167EFE0527D706Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4167EFE0527D706Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_hovering_over_mission_creator_blip_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4167EFE0527D706Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4167EFE0527D706Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 3: Any p2
```



pub fn set_blip_show_cone_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13127EC3665E8EE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13127EC3665E8EE1u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_show_cone_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13127EC3665E8EE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13127EC3665E8EE1u64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v3407
```



pub fn _show_purchase_instructional_button_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6865E26067B708Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6865E26067B708Cu64;
        
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
pub fn _show_purchase_instructional_button_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6865E26067B708Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6865E26067B708Cu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Disables frontend (works in custom frontends, not sure about regular pause menu) navigation keys on keyboard. Not sure about controller. Does not disable mouse controls. No need to call this every tick.

To enable the keys again, use [`0x14621BB1DF14E2B2`](#_0x14621BB1DF14E2B2).



pub fn take_control_of_frontend_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC9264727EEC0F28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC9264727EEC0F28u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn take_control_of_frontend_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC9264727EEC0F28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC9264727EEC0F28u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Sets a gamer tag's component colour



pub fn set_mp_gamer_tag_colour_safe(
        
        
            gamerTagId: 
        , 
        
        
            component: 
        , 
        
        
            hudColorIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x613ED644950626AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x613ED644950626AEu64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId, 
                component, 
                hudColorIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mp_gamer_tag_colour_raw(
        gamerTagId: , 
        component: , 
        hudColorIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x613ED644950626AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x613ED644950626AEu64;

        invoke_raw_typed!(
            hash,
                gamerTagId, 
                component, 
                hudColorIndex
        )
    }
}

/// ```
Does not require whole number/integer rotations.
```



pub fn _set_blip_squared_rotation_safe(
        
        
            blip: 
        , 
        
        
            heading: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8B6AFDAC320AC87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8B6AFDAC320AC87u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                heading
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_blip_squared_rotation_raw(
        blip: , 
        heading: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8B6AFDAC320AC87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8B6AFDAC320AC87u64;

        invoke_raw_typed!(
            hash,
                blip, 
                heading
        )
    }
}

/// Disables the loading spinner in Pause Menu when switching from one header tab to another.



pub fn _pause_menu_disable_busyspinner_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9245E81072704B8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9245E81072704B8Au64;
        
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
pub fn _pause_menu_disable_busyspinner_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9245E81072704B8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9245E81072704B8Au64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
END_TEXT_COMMAND_*
In scripts font most of the time is passed as 1.
Use _BEGIN_TEXT_GET_COMMAND_GET_WIDTH
param is not font from what i've tested
```



pub fn _end_text_command_get_width_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85F061DA64ED2F67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85F061DA64ED2F67u64;
        
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
pub fn _end_text_command_get_width_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85F061DA64ED2F67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85F061DA64ED2F67u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _0x593feae1f73392d4_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x593FEAE1F73392D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x593FEAE1F73392D4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x593feae1f73392d4_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x593FEAE1F73392D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x593FEAE1F73392D4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_widescreen_format_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3B07BA00A83B0F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3B07BA00A83B0F1u64;
        
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
pub fn set_widescreen_format_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3B07BA00A83B0F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3B07BA00A83B0F1u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Changes the mouse style.

```c
enum eMousePointerStyle {
	ARROW = 1,
	ARROW_DIMMED = 2,
	HAND_OPEN = 3,
	HAND_GRAB = 4,
	HAND_MIDDLE_FINGER = 5,
	ARROW_LEFT = 6,
	ARROW_RIGHT = 7,
	ARROW_UP = 8,
	ARROW_DOWN = 9,
	ARROW_TRIMMING = 10,
	ARROW_PLUS = 11,
	ARROW_MINUS = 12
};
```



pub fn set_mouse_cursor_style_safe(
        
        
            style: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DB8CFFD58B62552u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DB8CFFD58B62552u64;
        
        let result = invoke_raw!(
            hash,
                style
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mouse_cursor_style_raw(
        style: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DB8CFFD58B62552u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DB8CFFD58B62552u64;

        invoke_raw_typed!(
            hash,
                style
        )
    }
}

/// ## Parameters
*



pub fn is_mp_gamer_tag_active_safe(
        
        
            gamerTagId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E929E7A5796FD26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E929E7A5796FD26u64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_mp_gamer_tag_active_raw(
        gamerTagId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E929E7A5796FD26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E929E7A5796FD26u64;

        invoke_raw_typed!(
            hash,
                gamerTagId
        )
    }
}

/// ## Parameters
*



pub fn _0x62e849b7eb28e770_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62E849B7EB28E770u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62E849B7EB28E770u64;
        
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
pub fn _0x62e849b7eb28e770_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62E849B7EB28E770u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62E849B7EB28E770u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_blip_flashes_alternate_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E8D9498C56DD0D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E8D9498C56DD0D1u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_flashes_alternate_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E8D9498C56DD0D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E8D9498C56DD0D1u64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// ```
"DISPLAY_CASH(true);" makes the cash amount render on the screen when appropriate  
"DISPLAY_CASH(false);" disables cash amount rendering  
```



pub fn display_cash_safe(
        
        
            display: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96DEC8D5430208B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96DEC8D5430208B7u64;
        
        let result = invoke_raw!(
            hash,
                display
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn display_cash_raw(
        display: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96DEC8D5430208B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96DEC8D5430208B7u64;

        invoke_raw_typed!(
            hash,
                display
        )
    }
}

/// ```
Clears the GPS flags. Only the script that originally called SET_GPS_FLAGS can clear them.  
Doesn't seem like the flags are actually read by the game at all.  
```



pub fn clear_gps_flags_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21986729D6A3A830u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21986729D6A3A830u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_gps_flags_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21986729D6A3A830u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21986729D6A3A830u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_colour_of_next_text_component_safe(
        
        
            hudColor: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39BBF623FC803EACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39BBF623FC803EACu64;
        
        let result = invoke_raw!(
            hash,
                hudColor
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_colour_of_next_text_component_raw(
        hudColor: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39BBF623FC803EACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39BBF623FC803EACu64;

        invoke_raw_typed!(
            hash,
                hudColor
        )
    }
}

/// ## Parameters
*



pub fn end_text_command_override_button_text_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA86911979638106Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA86911979638106Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_override_button_text_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA86911979638106Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA86911979638106Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn does_blip_exist_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6DB27D19ECBB7DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6DB27D19ECBB7DAu64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_blip_exist_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6DB27D19ECBB7DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6DB27D19ECBB7DAu64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// ## Parameters
*



pub fn set_blip_bright_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB203913733F27884u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB203913733F27884u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_bright_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB203913733F27884u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB203913733F27884u64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// Clears all active blip routes that have been set with [`SetBlipRoute`](#_0x4F7D8A9BFB0B43E9).



pub fn _clear_all_blip_routes_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD12882D3FF82BF11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD12882D3FF82BF11u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _clear_all_blip_routes_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD12882D3FF82BF11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD12882D3FF82BF11u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xa17784fca9548d15_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA17784FCA9548D15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA17784FCA9548D15u64;
        
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
pub fn _0xa17784fca9548d15_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA17784FCA9548D15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA17784FCA9548D15u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// When this native returns true, do not call [`SET_MP_GAMER_TAG_NAME`](#_0xDEA2B8283BAA3944).



pub fn is_updating_mp_gamer_tag_name_and_crew_details_safe(
        
        
            playerId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB709A36958ABE0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB709A36958ABE0Du64;
        
        let result = invoke_raw!(
            hash,
                playerId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_updating_mp_gamer_tag_name_and_crew_details_raw(
        playerId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB709A36958ABE0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB709A36958ABE0Du64;

        invoke_raw_typed!(
            hash,
                playerId
        )
    }
}

/// ```
Returns a substring of a specified length starting at a specified position.
Example:
// Get "STRING" text from "MY_STRING"
subStr = HUD::_GET_TEXT_SUBSTRING("MY_STRING", 3, 6);
```



pub fn _get_text_substring_safe(
        
        
            text: 
        , 
        
        
            position: 
        , 
        
        
            length: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x169BD9382084C8C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x169BD9382084C8C0u64;
        
        let result = invoke_raw!(
            hash,
                text, 
                position, 
                length
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_text_substring_raw(
        text: , 
        position: , 
        length: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x169BD9382084C8C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x169BD9382084C8C0u64;

        invoke_raw_typed!(
            hash,
                text, 
                position, 
                length
        )
    }
}

/// ```
If true, the entire map will be revealed.
```



pub fn set_minimap_hide_fow_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8DEE0A5600CBB93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8DEE0A5600CBB93u64;
        
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
pub fn set_minimap_hide_fow_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8DEE0A5600CBB93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8DEE0A5600CBB93u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Requires manual management of game stream handles (i.e., 0xBE4390CB40B3E627).
```



pub fn thefeed_freeze_next_post_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDEC055AB549E328u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDEC055AB549E328u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_freeze_next_post_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDEC055AB549E328u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDEC055AB549E328u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
This function is hard-coded to always return 0.  
```



pub fn get_blip_info_id_pickup_index_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B6786E4C03DD382u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B6786E4C03DD382u64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_blip_info_id_pickup_index_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B6786E4C03DD382u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B6786E4C03DD382u64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// ## Return value



pub fn get_current_website_id_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97D47996FC48CBADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97D47996FC48CBADu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_current_website_id_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97D47996FC48CBADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97D47996FC48CBADu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns the Blip handle of given Entity.  
```



pub fn get_blip_from_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC8DBDCA2436F7E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC8DBDCA2436F7E8u64;
        
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
pub fn get_blip_from_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC8DBDCA2436F7E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC8DBDCA2436F7E8u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
NativeDB Introduced: v2060
```



pub fn _0x243296a510b562b6_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x243296A510B562B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x243296A510B562B6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x243296a510b562b6_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x243296A510B562B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x243296A510B562B6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_ped_ai_blip_notice_range_safe(
        
        
            ped: 
        , 
        
        
            range: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97C65887D4B37FA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97C65887D4B37FA9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                range
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_ai_blip_notice_range_raw(
        ped: , 
        range: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97C65887D4B37FA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97C65887D4B37FA9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                range
        )
    }
}

/// ## Return value



pub fn get_number_of_active_blips_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A3FF3DE163034E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A3FF3DE163034E8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_number_of_active_blips_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A3FF3DE163034E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A3FF3DE163034E8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Shows a menu for reporting UGC content.
```



pub fn open_reportugc_menu_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x523A590C1A3CC0D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x523A590C1A3CC0D3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn open_reportugc_menu_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x523A590C1A3CC0D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x523A590C1A3CC0D3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0xde03620f8703a9df_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE03620F8703A9DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE03620F8703A9DFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xde03620f8703a9df_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE03620F8703A9DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE03620F8703A9DFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
You can only use text entries. No custom text.  
```

```
NativeDB Added Parameter 11: Any p10
```



pub fn set_warning_message_with_header_and_substring_flags_safe(
        
        
            entryHeader: 
        , 
        
        
            entryLine1: 
        , 
        
        
            instructionalKey: 
        , 
        
        
            entryLine2: 
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x701919482C74B5ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x701919482C74B5ABu64;
        
        let result = invoke_raw!(
            hash,
                entryHeader, 
                entryLine1, 
                instructionalKey, 
                entryLine2, 
                p4, 
                p5, 
                p6, 
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
pub fn set_warning_message_with_header_and_substring_flags_raw(
        entryHeader: , 
        entryLine1: , 
        instructionalKey: , 
        entryLine2: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x701919482C74B5ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x701919482C74B5ABu64;

        invoke_raw_typed!(
            hash,
                entryHeader, 
                entryLine1, 
                instructionalKey, 
                entryLine2, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9
        )
    }
}

/// Toggles pause menu map rendering.



pub fn pause_toggle_fullscreen_map_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DE6C5E2E996F178u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DE6C5E2E996F178u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pause_toggle_fullscreen_map_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DE6C5E2E996F178u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DE6C5E2E996F178u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn begin_text_command_override_button_text_safe(
        
        
            gxtEntry: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F9EE5687F8EECCDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F9EE5687F8EECCDu64;
        
        let result = invoke_raw!(
            hash,
                gxtEntry
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_text_command_override_button_text_raw(
        gxtEntry: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F9EE5687F8EECCDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F9EE5687F8EECCDu64;

        invoke_raw_typed!(
            hash,
                gxtEntry
        )
    }
}

/// ## Return value



pub fn get_minimap_fow_discovery_ratio_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0130B41D3CF4574u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0130B41D3CF4574u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_minimap_fow_discovery_ratio_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0130B41D3CF4574u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0130B41D3CF4574u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
IS_*
```



pub fn _0x801879a9b4f4b2fb_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x801879A9B4F4B2FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x801879A9B4F4B2FBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x801879a9b4f4b2fb_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x801879A9B4F4B2FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x801879A9B4F4B2FBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _set_interior_zoom_level_decreased_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EC8ABA5E74B3D7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EC8ABA5E74B3D7Au64;
        
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
pub fn _set_interior_zoom_level_decreased_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EC8ABA5E74B3D7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EC8ABA5E74B3D7Au64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// CLEAR_SMALL_PRINTS native function



pub fn clear_small_prints_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CEA2839313C09ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CEA2839313C09ACu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_small_prints_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CEA2839313C09ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CEA2839313C09ACu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Does the same as [`SET_GPS_MULTI_ROUTE_RENDER(false)`](#_0x3DDA37128DD1ACA8)



pub fn clear_gps_multi_route_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67EEDEA1B9BAFD94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67EEDEA1B9BAFD94u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_gps_multi_route_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67EEDEA1B9BAFD94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67EEDEA1B9BAFD94u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Returns true if a Warning Message or ReportUGC menu is active.



pub fn is_warning_message_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE18B138FABC53103u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE18B138FABC53103u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_warning_message_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE18B138FABC53103u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE18B138FABC53103u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Toggles the big minimap state like in GTA:Online.

To get the current state of the minimap, use [`IS_BIGMAP_ACTIVE`](#_0xFFF65C63).



pub fn set_bigmap_active_safe(
        
        
            toggleBigMap: 
        , 
        
        
            showFullMap: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x231C8F89D0539D8Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x231C8F89D0539D8Fu64;
        
        let result = invoke_raw!(
            hash,
                toggleBigMap, 
                showFullMap
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_bigmap_active_raw(
        toggleBigMap: , 
        showFullMap: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x231C8F89D0539D8Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x231C8F89D0539D8Fu64;

        invoke_raw_typed!(
            hash,
                toggleBigMap, 
                showFullMap
        )
    }
}

/// ## Return value



pub fn is_pause_menu_restarting_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C491717107431C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C491717107431C7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_pause_menu_restarting_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C491717107431C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C491717107431C7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Full list of components below  
HUD = 0;  
HUD_WANTED_STARS = 1;  
HUD_WEAPON_ICON = 2;  
HUD_CASH = 3;  
HUD_MP_CASH = 4;  
HUD_MP_MESSAGE = 5;  
HUD_VEHICLE_NAME = 6;  
HUD_AREA_NAME = 7;  
HUD_VEHICLE_CLASS = 8;  
HUD_STREET_NAME = 9;  
HUD_HELP_TEXT = 10;  
HUD_FLOATING_HELP_TEXT_1 = 11;  
HUD_FLOATING_HELP_TEXT_2 = 12;  
HUD_CASH_CHANGE = 13;  
HUD_RETICLE = 14;  
HUD_SUBTITLE_TEXT = 15;  
HUD_RADIO_STATIONS = 16;  
HUD_SAVING_GAME = 17;  
HUD_GAME_STREAM = 18;  
HUD_WEAPON_WHEEL = 19;  
HUD_WEAPON_WHEEL_STATS = 20;  
MAX_HUD_COMPONENTS = 21;  
MAX_HUD_WEAPONS = 22;  
MAX_SCRIPTED_HUD_COMPONENTS = 141;  
```



pub fn is_hud_component_active_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC4C9EA5391ECC0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC4C9EA5391ECC0Du64;
        
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
pub fn is_hud_component_active_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC4C9EA5391ECC0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC4C9EA5391ECC0Du64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ## Parameters
*



pub fn get_blip_coords_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x586AFE3FF72D996Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x586AFE3FF72D996Eu64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_blip_coords_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x586AFE3FF72D996Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x586AFE3FF72D996Eu64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// Displays "normal" notifications again after calling `_0x56C8B608CFD49854` (those that were drawn before calling this native too), though those will have a weird offset and stay on screen forever (tested with notifications created from same script).



pub fn _0xaded7f5748acafe6_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADED7F5748ACAFE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADED7F5748ACAFE6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xaded7f5748acafe6_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADED7F5748ACAFE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADED7F5748ACAFE6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Once called each frame hides all above radar notifications.



pub fn thefeed_hide_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25F87B30C382FCA7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25F87B30C382FCA7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_hide_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25F87B30C382FCA7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25F87B30C382FCA7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
List of picNames: pastebin.com/XdpJVbHz  
flash is a bool for fading in.  
iconTypes:  
1 : Chat Box  
2 : Email  
3 : Add Friend Request  
4 : Nothing  
5 : Nothing  
6 : Nothing  
7 : Right Jumping Arrow  
8 : RP Icon  
9 : $ Icon  
"sender" is the very top header. This can be any old string.  
"subject" is the header under the sender.  
"duration" is a multiplier, so 1.0 is normal, 2.0 is twice as long (very slow), and 0.5 is half as long.  
"clanTag" shows a crew tag in the "sender" header, after the text. You need to use 3 underscores as padding. Maximum length of this field seems to be 7. (e.g. "MK" becomes "___MK", "ACE" becomes "___ACE", etc.)  
iconType2 is a mirror of iconType. It shows in the "subject" line, right under the original iconType.  
int IconNotification(char *text, char *text2, char *Subject)  
{  
	_SET_NOTIFICATION_TEXT_ENTRY("STRING");  
	ADD_TEXT_COMPONENT_SUBSTRING_PLAYER_NAME(text);  
	_SET_NOTIFICATION_MESSAGE_CLAN_TAG_2("CHAR_SOCIAL_CLUB", "CHAR_SOCIAL_CLUB", 1, 7, text2, Subject, 1.0f, "__EXAMPLE", 7);  
	return _DRAW_NOTIFICATION(1, 1);  
}  
```



pub fn end_text_command_thefeed_post_messagetext_with_crew_tag_and_additional_icon_safe(
        
        
            picTxd: 
        , 
        
        
            picTxn: 
        , 
        
        
            flash: 
        , 
        
        
            iconType1: 
        , 
        
        
            nameStr: 
        , 
        
        
            subtitleStr: 
        , 
        
        
            duration: 
        , 
        
        
            crewPackedStr: 
        , 
        
        
            iconType2: 
        , 
        
        
            textColor: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x531B84E7DA981FB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x531B84E7DA981FB6u64;
        
        let result = invoke_raw!(
            hash,
                picTxd, 
                picTxn, 
                flash, 
                iconType1, 
                nameStr, 
                subtitleStr, 
                duration, 
                crewPackedStr, 
                iconType2, 
                textColor
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_messagetext_with_crew_tag_and_additional_icon_raw(
        picTxd: , 
        picTxn: , 
        flash: , 
        iconType1: , 
        nameStr: , 
        subtitleStr: , 
        duration: , 
        crewPackedStr: , 
        iconType2: , 
        textColor: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x531B84E7DA981FB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x531B84E7DA981FB6u64;

        invoke_raw_typed!(
            hash,
                picTxd, 
                picTxn, 
                flash, 
                iconType1, 
                nameStr, 
                subtitleStr, 
                duration, 
                crewPackedStr, 
                iconType2, 
                textColor
        )
    }
}

/// ## Parameters
*



pub fn set_minimap_in_spectator_mode_safe(
        
        
            toggle: 
        , 
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A5CD7752DD28CD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A5CD7752DD28CD3u64;
        
        let result = invoke_raw!(
            hash,
                toggle, 
                ped
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_minimap_in_spectator_mode_raw(
        toggle: , 
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A5CD7752DD28CD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A5CD7752DD28CD3u64;

        invoke_raw_typed!(
            hash,
                toggle, 
                ped
        )
    }
}

/// THEFEED_RESET_ALL_PARAMETERS native function



pub fn thefeed_reset_all_parameters_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDD85225B2DEA55Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDD85225B2DEA55Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_reset_all_parameters_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDD85225B2DEA55Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDD85225B2DEA55Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _override_multiplayer_chat_colour_safe(
        
        
            p0: 
        , 
        
        
            hudColor: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF47E567B3630DD12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF47E567B3630DD12u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                hudColor
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _override_multiplayer_chat_colour_raw(
        p0: , 
        hudColor: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF47E567B3630DD12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF47E567B3630DD12u64;

        invoke_raw_typed!(
            hash,
                p0, 
                hudColor
        )
    }
}

/// Should be enabled as component (2). Has 0 alpha by default.



pub fn set_mp_gamer_tag_health_bar_colour_safe(
        
        
            gamerTagId: 
        , 
        
        
            hudColorIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3158C77A7E888AB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3158C77A7E888AB4u64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId, 
                hudColorIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mp_gamer_tag_health_bar_colour_raw(
        gamerTagId: , 
        hudColorIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3158C77A7E888AB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3158C77A7E888AB4u64;

        invoke_raw_typed!(
            hash,
                gamerTagId, 
                hudColorIndex
        )
    }
}

/// DISPLAY_HUD_WHEN_PAUSED_THIS_FRAME native function



pub fn display_hud_when_paused_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x402F9ED62087E898u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x402F9ED62087E898u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn display_hud_when_paused_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x402F9ED62087E898u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x402F9ED62087E898u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn does_blip_have_gps_route_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD2238F57B977751u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD2238F57B977751u64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_blip_have_gps_route_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD2238F57B977751u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD2238F57B977751u64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// ```
Locks the minimap to the specified angle in integer degrees.  
angle: The angle in whole degrees. If less than 0 or greater than 360, unlocks the angle.  
```



pub fn lock_minimap_angle_safe(
        
        
            angle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x299FAEBB108AE05Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x299FAEBB108AE05Bu64;
        
        let result = invoke_raw!(
            hash,
                angle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn lock_minimap_angle_raw(
        angle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x299FAEBB108AE05Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x299FAEBB108AE05Bu64;

        invoke_raw_typed!(
            hash,
                angle
        )
    }
}

/// ```
Getter for 0xCD74233600C4EA6B

GET_*
```



pub fn _0xc2d2ad9eaae265b8_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2D2AD9EAAE265B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2D2AD9EAAE265B8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc2d2ad9eaae265b8_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2D2AD9EAAE265B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2D2AD9EAAE265B8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_help_message_on_screen_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDAD37F45428801AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDAD37F45428801AEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_help_message_on_screen_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDAD37F45428801AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDAD37F45428801AEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _set_ped_ai_blip_sprite_safe(
        
        
            ped: 
        , 
        
        
            spriteId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCFACD0DB9D7A57Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCFACD0DB9D7A57Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                spriteId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_ped_ai_blip_sprite_raw(
        ped: , 
        spriteId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCFACD0DB9D7A57Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCFACD0DB9D7A57Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                spriteId
        )
    }
}

/// ## Parameters
*



pub fn set_mission_name_safe(
        
        
            p0: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F28ECF5FC84772Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F28ECF5FC84772Fu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mission_name_raw(
        p0: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F28ECF5FC84772Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F28ECF5FC84772Fu64;

        invoke_raw_typed!(
            hash,
                p0, 
                name
        )
    }
}

/// Adds options on a warning message.



pub fn set_warning_message_option_items_safe(
        
        
            index: 
        , 
        
        
            name: 
        , 
        
        
            cash: 
        , 
        
        
            rp: 
        , 
        
        
            lvl: 
        , 
        
        
            colour: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C5A80A9E096D529u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C5A80A9E096D529u64;
        
        let result = invoke_raw!(
            hash,
                index, 
                name, 
                cash, 
                rp, 
                lvl, 
                colour
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_warning_message_option_items_raw(
        index: , 
        name: , 
        cash: , 
        rp: , 
        lvl: , 
        colour: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C5A80A9E096D529u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C5A80A9E096D529u64;

        invoke_raw_typed!(
            hash,
                index, 
                name, 
                cash, 
                rp, 
                lvl, 
                colour
        )
    }
}

/// ```
Sets alpha-channel for blip color.
Example:
Blip blip = HUD::ADD_BLIP_FOR_ENTITY(entity);
HUD::SET_BLIP_COLOUR(blip , 3);
HUD::SET_BLIP_ALPHA(blip , 64);
```



pub fn set_blip_alpha_safe(
        
        
            blip: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45FF974EEE1C8734u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45FF974EEE1C8734u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_alpha_raw(
        blip: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45FF974EEE1C8734u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45FF974EEE1C8734u64;

        invoke_raw_typed!(
            hash,
                blip, 
                alpha
        )
    }
}

/// ## Parameters
*



pub fn _0xb552929b85fc27ec_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB552929B85FC27ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB552929B85FC27ECu64;
        
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
pub fn _0xb552929b85fc27ec_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB552929B85FC27ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB552929B85FC27ECu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Returns the ActionScript flagValue.
ActionScript flags are global flags that scaleforms use
Flags found during testing
0: Returns 1 if the web_browser keyboard is open, otherwise 0
1: Returns 1 if the player has clicked back twice on the opening page, otherwise 0 (web_browser)
2: Returns how many links the player has clicked in the web_browser scaleform, returns 0 when the browser gets closed
9: Returns the current selection on the mobile phone scaleform
There are 20 flags in total.
```



pub fn get_global_actionscript_flag_safe(
        
        
            flagIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3B05614DCE1D014u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3B05614DCE1D014u64;
        
        let result = invoke_raw!(
            hash,
                flagIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_global_actionscript_flag_raw(
        flagIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3B05614DCE1D014u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3B05614DCE1D014u64;

        invoke_raw_typed!(
            hash,
                flagIndex
        )
    }
}

/// Stops loading screen tips shown by invoking either `0x488043841BBE156F` or `0x15CFA549788D35EF`



pub fn _thefeed_disable_loading_screen_tips_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32888337579A5970u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32888337579A5970u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _thefeed_disable_loading_screen_tips_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32888337579A5970u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32888337579A5970u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns the current AI BLIP for the specified ped  
```



pub fn _get_ai_blip_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56176892826A4FE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56176892826A4FE8u64;
        
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
pub fn _get_ai_blip_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56176892826A4FE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56176892826A4FE8u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Sets the color of HUD_COLOUR_SCRIPT_VARIABLE_2
```



pub fn _set_script_variable_2_hud_colour_safe(
        
        
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
        let hash = 0x16A304E6CB2BFAB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16A304E6CB2BFAB9u64;
        
        let result = invoke_raw!(
            hash,
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
pub fn _set_script_variable_2_hud_colour_raw(
        r: , 
        g: , 
        b: , 
        a: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16A304E6CB2BFAB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16A304E6CB2BFAB9u64;

        invoke_raw_typed!(
            hash,
                r, 
                g, 
                b, 
                a
        )
    }
}

/// Enables or disables the sonar sweep animation on the minimap.

```
NativeDB Introduced: v2189
```



pub fn set_minimap_sonar_sweep_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B50FC8749632EC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B50FC8749632EC1u64;
        
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
pub fn set_minimap_sonar_sweep_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B50FC8749632EC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B50FC8749632EC1u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn clear_reminder_message_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB57D8DD645CFA2CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB57D8DD645CFA2CFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_reminder_message_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB57D8DD645CFA2CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB57D8DD645CFA2CFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn clear_additional_text_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A179DF17CCF04CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A179DF17CCF04CDu64;
        
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
pub fn clear_additional_text_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A179DF17CCF04CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A179DF17CCF04CDu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_gps_custom_route_render_safe(
        
        
            toggle: 
        , 
        
        
            radarThickness: 
        , 
        
        
            mapThickness: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x900086F371220B6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x900086F371220B6Fu64;
        
        let result = invoke_raw!(
            hash,
                toggle, 
                radarThickness, 
                mapThickness
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_gps_custom_route_render_raw(
        toggle: , 
        radarThickness: , 
        mapThickness: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x900086F371220B6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x900086F371220B6Fu64;

        invoke_raw_typed!(
            hash,
                toggle, 
                radarThickness, 
                mapThickness
        )
    }
}

/// ```
clears a print text command with this text  
```



pub fn begin_text_command_clear_print_safe(
        
        
            text: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE124FA80A759019Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE124FA80A759019Cu64;
        
        let result = invoke_raw!(
            hash,
                text
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_text_command_clear_print_raw(
        text: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE124FA80A759019Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE124FA80A759019Cu64;

        invoke_raw_typed!(
            hash,
                text
        )
    }
}

/// ```
Displays "normal" notifications again after calling `_0x56C8B608CFD49854` (those that were drawn before calling this native too), though those will have a weird offset and stay on screen forever (tested with notifications created from same script).
```



pub fn thefeed_comment_teleport_pool_off_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADED7F5748ACAFE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADED7F5748ACAFE6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_comment_teleport_pool_off_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADED7F5748ACAFE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADED7F5748ACAFE6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Removes the loading prompt at the bottom right of the screen.
```



pub fn busyspinner_off_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10D373323E5B9C0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10D373323E5B9C0Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn busyspinner_off_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10D373323E5B9C0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10D373323E5B9C0Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Must be toggled before being queued for animation
```



pub fn _set_blip_display_indicator_on_blip_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4278F70131BAA6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4278F70131BAA6Du64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_blip_display_indicator_on_blip_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4278F70131BAA6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4278F70131BAA6Du64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// REMOVE_MULTIPLAYER_BANK_CASH native function



pub fn remove_multiplayer_bank_cash_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7C6789AA1CFEDD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7C6789AA1CFEDD0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_multiplayer_bank_cash_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7C6789AA1CFEDD0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7C6789AA1CFEDD0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_menu_ped_masked_int_stat_safe(
        
        
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
        let hash = 0x90A6526CF0381030u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90A6526CF0381030u64;
        
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
pub fn get_menu_ped_masked_int_stat_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90A6526CF0381030u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90A6526CF0381030u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3
        )
    }
}

/// distance - shadow distance in pixels, both horizontal and vertical
r, g, b, a - color



pub fn set_text_dropshadow_safe(
        
        
            distance: 
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
        let hash = 0x465C84BC39F1C351u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x465C84BC39F1C351u64;
        
        let result = invoke_raw!(
            hash,
                distance, 
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
pub fn set_text_dropshadow_raw(
        distance: , 
        r: , 
        g: , 
        b: , 
        a: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x465C84BC39F1C351u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x465C84BC39F1C351u64;

        invoke_raw_typed!(
            hash,
                distance, 
                r, 
                g, 
                b, 
                a
        )
    }
}

/// ```
HIDE_*_THIS_FRAME

Hides area and vehicle name HUD components for one frame.
```



pub fn _hide_area_and_vehicle_name_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4DEDE28B1814289u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4DEDE28B1814289u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _hide_area_and_vehicle_name_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4DEDE28B1814289u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4DEDE28B1814289u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_mp_gamer_tag_big_text_safe(
        
        
            gamerTagId: 
        , 
        
        
            string: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B7723747CCB55B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B7723747CCB55B6u64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId, 
                string
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mp_gamer_tag_big_text_raw(
        gamerTagId: , 
        string: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B7723747CCB55B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B7723747CCB55B6u64;

        invoke_raw_typed!(
            hash,
                gamerTagId, 
                string
        )
    }
}

/// ```
Returns a value based on what the blip is attached to
1 - Vehicle
2 - Ped
3 - Object
4 - Coord
5 - unk
6 - Pickup
7 - Radius
```



pub fn get_blip_info_id_type_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE9B0959FFD0779Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE9B0959FFD0779Bu64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_blip_info_id_type_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE9B0959FFD0779Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE9B0959FFD0779Bu64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// Enables loading screen tips to be be shown (`_0x15CFA549788D35EF` and `_0x488043841BBE156F`), blocks other kinds of notifications from being displayed (at least from current script). Call `0xADED7F5748ACAFE6` to display those again.



pub fn thefeed_force_render_off_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x583049884A2EEE3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x583049884A2EEE3Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_force_render_off_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x583049884A2EEE3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x583049884A2EEE3Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_text_leading_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA50ABC31E3CDFAFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA50ABC31E3CDFAFFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_text_leading_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA50ABC31E3CDFAFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA50ABC31E3CDFAFFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_blip_route_colour_safe(
        
        
            blip: 
        , 
        
        
            colour: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x837155CD2F63DA09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x837155CD2F63DA09u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                colour
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_route_colour_raw(
        blip: , 
        colour: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x837155CD2F63DA09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x837155CD2F63DA09u64;

        invoke_raw_typed!(
            hash,
                blip, 
                colour
        )
    }
}

/// ## Parameters
*



pub fn _0xdaf87174be7454ff_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDAF87174BE7454FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDAF87174BE7454FFu64;
        
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
pub fn _0xdaf87174be7454ff_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDAF87174BE7454FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDAF87174BE7454FFu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Creates a blip for the specified coordinates. You can use `SET_BLIP_` natives to change the blip.



pub fn add_blip_for_coord_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A039BB0BCA604B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A039BB0BCA604B6u64;
        
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
pub fn add_blip_for_coord_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A039BB0BCA604B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A039BB0BCA604B6u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn _0x8f08017f9d7c47bd_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F08017F9D7C47BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F08017F9D7C47BDu64;
        
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
pub fn _0x8f08017f9d7c47bd_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F08017F9D7C47BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F08017F9D7C47BDu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
HAS_*
```



pub fn _has_director_mode_been_triggered_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA277800A9EAE340Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA277800A9EAE340Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _has_director_mode_been_triggered_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA277800A9EAE340Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA277800A9EAE340Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_social_club_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC406BE343FC4B9AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC406BE343FC4B9AFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_social_club_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC406BE343FC4B9AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC406BE343FC4B9AFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Toggles the North Yankton map
```



pub fn set_minimap_in_prologue_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9133955F1A2DA957u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9133955F1A2DA957u64;
        
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
pub fn set_minimap_in_prologue_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9133955F1A2DA957u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9133955F1A2DA957u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Shows an "award" notification above the minimap, lua example result:

![](https://i.imgur.com/e2DNaKX.png)



Old description:
```
Example:  
UI::_SET_NOTIFICATION_TEXT_ENTRY("HUNT");  
UI::_0xAA295B6F28BD587D("Hunting", "Hunting_Gold_128", 0, 109, "HUD_MED_UNLKED");  
```



pub fn end_text_command_thefeed_post_award_safe(
        
        
            textureDict: 
        , 
        
        
            textureName: 
        , 
        
        
            rpBonus: 
        , 
        
        
            colorOverlay: 
        , 
        
        
            titleLabel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA295B6F28BD587Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA295B6F28BD587Du64;
        
        let result = invoke_raw!(
            hash,
                textureDict, 
                textureName, 
                rpBonus, 
                colorOverlay, 
                titleLabel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_award_raw(
        textureDict: , 
        textureName: , 
        rpBonus: , 
        colorOverlay: , 
        titleLabel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA295B6F28BD587Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA295B6F28BD587Du64;

        invoke_raw_typed!(
            hash,
                textureDict, 
                textureName, 
                rpBonus, 
                colorOverlay, 
                titleLabel
        )
    }
}

/// ```
Returns whether the ped's blip is controlled by the game.   
It's the default blip you can see on enemies during freeroam in singleplayer (the one that fades out quickly).  
```



pub fn does_ped_have_ai_blip_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15B8ECF844EE67EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15B8ECF844EE67EDu64;
        
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
pub fn does_ped_have_ai_blip_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15B8ECF844EE67EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15B8ECF844EE67EDu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
If toggle is true, hides special ability bar / character name in the pause menu
If toggle is false, shows special ability bar / character name in the pause menu
SET_PLAYER_*
```



pub fn _set_player_is_in_director_mode_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x808519373FD336A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x808519373FD336A3u64;
        
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
pub fn _set_player_is_in_director_mode_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x808519373FD336A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x808519373FD336A3u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Enabling this on a radius blip will make it outline only.  
Please note that this only works on a



pub fn set_radius_blip_edge_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25615540D894B814u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25615540D894B814u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_radius_blip_edge_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25615540D894B814u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25615540D894B814u64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _set_interior_zoom_level_increased_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x504DFE62A1692296u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x504DFE62A1692296u64;
        
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
pub fn _set_interior_zoom_level_increased_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x504DFE62A1692296u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x504DFE62A1692296u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Name between `GET_ONSCREEN_KEYBOARD_RESULT` and `GET_PAUSE_MENU_STATE`. Likely, `GET_PAUSE_MENU_*`.



pub fn _get_pause_menu_cursor_position_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BFF36D6ED83E0AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BFF36D6ED83E0AEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_pause_menu_cursor_position_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BFF36D6ED83E0AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BFF36D6ED83E0AEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0xf284ac67940c6812_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF284AC67940C6812u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF284AC67940C6812u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf284ac67940c6812_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF284AC67940C6812u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF284AC67940C6812u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn end_text_command_thefeed_post_unlock_tu_safe(
        
        
            chTitle: 
        , 
        
        
            iconType: 
        , 
        
        
            chSubtitle: 
        , 
        
        
            isImportant: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8F3AAF93D0600BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8F3AAF93D0600BFu64;
        
        let result = invoke_raw!(
            hash,
                chTitle, 
                iconType, 
                chSubtitle, 
                isImportant
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_unlock_tu_raw(
        chTitle: , 
        iconType: , 
        chSubtitle: , 
        isImportant: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8F3AAF93D0600BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8F3AAF93D0600BFu64;

        invoke_raw_typed!(
            hash,
                chTitle, 
                iconType, 
                chSubtitle, 
                isImportant
        )
    }
}

/// ## Parameters
*



pub fn set_blip_colour_safe(
        
        
            blip: 
        , 
        
        
            color: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03D7FB09E75D6B7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03D7FB09E75D6B7Eu64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                color
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_colour_raw(
        blip: , 
        color: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03D7FB09E75D6B7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03D7FB09E75D6B7Eu64;

        invoke_raw_typed!(
            hash,
                blip, 
                color
        )
    }
}

/// Declares the entry type of a notification, for example "STRING".
int ShowNotification(char *text)
{
	BEGIN_TEXT_COMMAND_THEFEED_POST("STRING");
	ADD_TEXT_COMPONENT_SUBSTRING_PLAYER_NAME(text);
	return _DRAW_NOTIFICATION(1, 1);
}



pub fn begin_text_command_thefeed_post_safe(
        
        
            text: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x202709F4C58A0424u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x202709F4C58A0424u64;
        
        let result = invoke_raw!(
            hash,
                text
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_text_command_thefeed_post_raw(
        text: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x202709F4C58A0424u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x202709F4C58A0424u64;

        invoke_raw_typed!(
            hash,
                text
        )
    }
}

/// Shows this warning message when trying to switch pause menu header tabs: https://i.imgur.com/8qmfztu.png



pub fn pause_menu_set_warn_on_tab_change_safe(
        
        
            setWarn: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF06EBB91A81E09E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF06EBB91A81E09E3u64;
        
        let result = invoke_raw!(
            hash,
                setWarn
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pause_menu_set_warn_on_tab_change_raw(
        setWarn: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF06EBB91A81E09E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF06EBB91A81E09E3u64;

        invoke_raw_typed!(
            hash,
                setWarn
        )
    }
}

/// Example output preview:


![](https://i.imgur.com/TJvqkYq.png)



pub fn end_text_command_thefeed_post_ticker_safe(
        
        
            isImportant: 
        , 
        
        
            bHasTokens: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2ED7843F8F801023u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2ED7843F8F801023u64;
        
        let result = invoke_raw!(
            hash,
                isImportant, 
                bHasTokens
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_ticker_raw(
        isImportant: , 
        bHasTokens: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2ED7843F8F801023u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2ED7843F8F801023u64;

        invoke_raw_typed!(
            hash,
                isImportant, 
                bHasTokens
        )
    }
}

/// ## Return value



pub fn is_message_being_displayed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7984C03AA5CC2F41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7984C03AA5CC2F41u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_message_being_displayed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7984C03AA5CC2F41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7984C03AA5CC2F41u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn end_text_command_display_help_safe(
        
        
            shape: 
        , 
        
        
            loop: 
        , 
        
        
            beep: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x238FFE5C7B0498A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x238FFE5C7B0498A6u64;
        
        let result = invoke_raw!(
            hash,
                shape, 
                loop, 
                beep, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_display_help_raw(
        shape: , 
        loop: , 
        beep: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x238FFE5C7B0498A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x238FFE5C7B0498A6u64;

        invoke_raw_typed!(
            hash,
                shape, 
                loop, 
                beep, 
                duration
        )
    }
}

/// ## Parameters
*



pub fn display_ammo_this_frame_safe(
        
        
            display: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5E78BA2B1331C55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5E78BA2B1331C55u64;
        
        let result = invoke_raw!(
            hash,
                display
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn display_ammo_this_frame_raw(
        display: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5E78BA2B1331C55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5E78BA2B1331C55u64;

        invoke_raw_typed!(
            hash,
                display
        )
    }
}

/// ## Parameters
*



pub fn flash_minimap_display_with_color_safe(
        
        
            hudColorIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B1DE27EE78E6A19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B1DE27EE78E6A19u64;
        
        let result = invoke_raw!(
            hash,
                hudColorIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn flash_minimap_display_with_color_raw(
        hudColorIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B1DE27EE78E6A19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B1DE27EE78E6A19u64;

        invoke_raw_typed!(
            hash,
                hudColorIndex
        )
    }
}

/// ## Parameters
*



pub fn register_named_rendertarget_safe(
        
        
            name: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57D9C12635E25CE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57D9C12635E25CE3u64;
        
        let result = invoke_raw!(
            hash,
                name, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn register_named_rendertarget_raw(
        name: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57D9C12635E25CE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57D9C12635E25CE3u64;

        invoke_raw_typed!(
            hash,
                name, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_text_right_justify_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B3C4650BC8BEE47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B3C4650BC8BEE47u64;
        
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
pub fn set_text_right_justify_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B3C4650BC8BEE47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B3C4650BC8BEE47u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Request a gxt into the passed slot.  
```



pub fn request_additional_text_safe(
        
        
            gxt: 
        , 
        
        
            slot: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71A78003C8E71424u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71A78003C8E71424u64;
        
        let result = invoke_raw!(
            hash,
                gxt, 
                slot
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_additional_text_raw(
        gxt: , 
        slot: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71A78003C8E71424u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71A78003C8E71424u64;

        invoke_raw_typed!(
            hash,
                gxt, 
                slot
        )
    }
}

/// REMOVE_MULTIPLAYER_WALLET_CASH native function



pub fn remove_multiplayer_wallet_cash_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95CF81BD06EE1887u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95CF81BD06EE1887u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_multiplayer_wallet_cash_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95CF81BD06EE1887u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95CF81BD06EE1887u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// **Note:** The buttons need to support mouse (with the `TOGGLE_MOUSE_SUPPORT` scaleform movie method) for it to return `true`.



pub fn is_mouse_rolled_over_instructional_buttons_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D9ACB1EB139E702u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D9ACB1EB139E702u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_mouse_rolled_over_instructional_buttons_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D9ACB1EB139E702u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D9ACB1EB139E702u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
displays wanted star above head  
```



pub fn set_mp_gamer_tag_wanted_level_safe(
        
        
            gamerTagId: 
        , 
        
        
            wantedlvl: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF228E2AA03099C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF228E2AA03099C3u64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId, 
                wantedlvl
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mp_gamer_tag_wanted_level_raw(
        gamerTagId: , 
        wantedlvl: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF228E2AA03099C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF228E2AA03099C3u64;

        invoke_raw_typed!(
            hash,
                gamerTagId, 
                wantedlvl
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn set_text_proportional_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x038C1F517D7FDCF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x038C1F517D7FDCF8u64;
        
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
pub fn set_text_proportional_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x038C1F517D7FDCF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x038C1F517D7FDCF8u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_new_waypoint_safe(
        
        
            x: 
        , 
        
        
            y: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE43368D2AA4F2FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE43368D2AA4F2FCu64;
        
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
pub fn set_new_waypoint_raw(
        x: , 
        y: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE43368D2AA4F2FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE43368D2AA4F2FCu64;

        invoke_raw_typed!(
            hash,
                x, 
                y
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _set_ability_bar_visibility_in_multiplayer_safe(
        
        
            visible: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DFEDD15019315A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DFEDD15019315A9u64;
        
        let result = invoke_raw!(
            hash,
                visible
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_ability_bar_visibility_in_multiplayer_raw(
        visible: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DFEDD15019315A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DFEDD15019315A9u64;

        invoke_raw_typed!(
            hash,
                visible
        )
    }
}

/// Returns the same as `IS_SOCIAL_CLUB_ACTIVE`.



pub fn is_online_policies_menu_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F72CD94F7B5B68Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F72CD94F7B5B68Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_online_policies_menu_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F72CD94F7B5B68Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F72CD94F7B5B68Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn add_text_component_substring_blip_name_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80EAD8E2E1D5D52Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80EAD8E2E1D5D52Eu64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_text_component_substring_blip_name_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80EAD8E2E1D5D52Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80EAD8E2E1D5D52Eu64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// ## Parameters
*



pub fn show_height_on_blip_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75A16C3DA34F1245u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75A16C3DA34F1245u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn show_height_on_blip_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75A16C3DA34F1245u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75A16C3DA34F1245u64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// ```
GET_F*
```



pub fn _0x98c3cf913d895111_safe(
        
        
            string: 
        , 
        
        
            length: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98C3CF913D895111u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98C3CF913D895111u64;
        
        let result = invoke_raw!(
            hash,
                string, 
                length
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x98c3cf913d895111_raw(
        string: , 
        length: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98C3CF913D895111u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98C3CF913D895111u64;

        invoke_raw_typed!(
            hash,
                string, 
                length
        )
    }
}

/// Enables frontend (works in custom frontends, not sure about regular pause menu) navigation keys on keyboard if they were disabled using the native below.

To disable the keys, use [`0xEC9264727EEC0F28`](#_0xEC9264727EEC0F28).



pub fn release_control_of_frontend_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14621BB1DF14E2B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14621BB1DF14E2B2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn release_control_of_frontend_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14621BB1DF14E2B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14621BB1DF14E2B2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Allows the user to set a blip as friendly or enemy based on the toggle.



pub fn set_blip_as_friendly_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F6F290102C02AB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F6F290102C02AB4u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_as_friendly_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F6F290102C02AB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F6F290102C02AB4u64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// ## Return value



pub fn is_help_message_being_displayed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D79439A6B55AC67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D79439A6B55AC67u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_help_message_being_displayed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D79439A6B55AC67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D79439A6B55AC67u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
displays wanted star above head  
```



pub fn _set_mp_gamer_tag_mp_bag_large_count_safe(
        
        
            gamerTagId: 
        , 
        
        
            count: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C16459B2324B2CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C16459B2324B2CFu64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId, 
                count
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_mp_gamer_tag_mp_bag_large_count_raw(
        gamerTagId: , 
        count: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C16459B2324B2CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C16459B2324B2CFu64;

        invoke_raw_typed!(
            hash,
                gamerTagId, 
                count
        )
    }
}

/// ## Parameters
*



pub fn set_floating_help_text_screen_position_safe(
        
        
            hudIndex: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7679CC1BCEBE3D4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7679CC1BCEBE3D4Cu64;
        
        let result = invoke_raw!(
            hash,
                hudIndex, 
                x, 
                y
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_floating_help_text_screen_position_raw(
        hudIndex: , 
        x: , 
        y: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7679CC1BCEBE3D4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7679CC1BCEBE3D4Cu64;

        invoke_raw_typed!(
            hash,
                hudIndex, 
                x, 
                y
        )
    }
}

/// ## Return value



pub fn _0x66e7cb63c97b7d20_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66E7CB63C97B7D20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66E7CB63C97B7D20u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x66e7cb63c97b7d20_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66E7CB63C97B7D20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66E7CB63C97B7D20u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
FORCE_*
```



pub fn _0x57d760d55f54e071_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57D760D55F54E071u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57D760D55F54E071u64;
        
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
pub fn _0x57d760d55f54e071_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57D760D55F54E071u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57D760D55F54E071u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// SET_C*

```
NativeDB Introduced: v1734
```



pub fn _0x9fcb3cbfb3ead69a_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FCB3CBFB3EAD69Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FCB3CBFB3EAD69Au64;
        
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
pub fn _0x9fcb3cbfb3ead69a_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FCB3CBFB3EAD69Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FCB3CBFB3EAD69Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn pause_menu_deactivate_context_safe(
        
        
            contextHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x444D8CF241EC25C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x444D8CF241EC25C5u64;
        
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
pub fn pause_menu_deactivate_context_raw(
        contextHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x444D8CF241EC25C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x444D8CF241EC25C5u64;

        invoke_raw_typed!(
            hash,
                contextHash
        )
    }
}

/// UNLOCK_MINIMAP_ANGLE native function



pub fn unlock_minimap_angle_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8183455E16C42E3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8183455E16C42E3Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn unlock_minimap_angle_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8183455E16C42E3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8183455E16C42E3Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_named_rendertarget_render_id_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A6478B61C6BDC3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A6478B61C6BDC3Bu64;
        
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
pub fn get_named_rendertarget_render_id_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A6478B61C6BDC3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A6478B61C6BDC3Bu64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ## Parameters
*



pub fn end_text_command_thefeed_post_mpticker_safe(
        
        
            blink: 
        , 
        
        
            bHasTokens: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF020C96915705B3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF020C96915705B3Au64;
        
        let result = invoke_raw!(
            hash,
                blink, 
                bHasTokens
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_mpticker_raw(
        blink: , 
        bHasTokens: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF020C96915705B3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF020C96915705B3Au64;

        invoke_raw_typed!(
            hash,
                blink, 
                bHasTokens
        )
    }
}

/// ## Parameters
*



pub fn does_text_block_exist_safe(
        
        
            gxt: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C7302E725259789u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C7302E725259789u64;
        
        let result = invoke_raw!(
            hash,
                gxt
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_text_block_exist_raw(
        gxt: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C7302E725259789u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C7302E725259789u64;

        invoke_raw_typed!(
            hash,
                gxt
        )
    }
}

/// ```
When calling this, the current frame will have the players "arrow icon" be focused on the dead center of the radar.
```



pub fn dont_tilt_minimap_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D14BFDC33B34F55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D14BFDC33B34F55u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn dont_tilt_minimap_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D14BFDC33B34F55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D14BFDC33B34F55u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// END_TEXT_COMMAND_CLEAR_PRINT native function



pub fn end_text_command_clear_print_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCC75460ABA29378u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCC75460ABA29378u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_clear_print_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCC75460ABA29378u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCC75460ABA29378u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Removes a notification instantly instead of waiting for it to disappear  
```



pub fn thefeed_remove_item_safe(
        
        
            notificationId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE4390CB40B3E627u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE4390CB40B3E627u64;
        
        let result = invoke_raw!(
            hash,
                notificationId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_remove_item_raw(
        notificationId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE4390CB40B3E627u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE4390CB40B3E627u64;

        invoke_raw_typed!(
            hash,
                notificationId
        )
    }
}

/// ## Parameters
*



pub fn end_text_command_thefeed_post_unlock_tu_with_color_safe(
        
        
            chTitle: 
        , 
        
        
            iconType: 
        , 
        
        
            chSubtitle: 
        , 
        
        
            isImportant: 
        , 
        
        
            titleColor: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AE0589093A2E088u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AE0589093A2E088u64;
        
        let result = invoke_raw!(
            hash,
                chTitle, 
                iconType, 
                chSubtitle, 
                isImportant, 
                titleColor, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_unlock_tu_with_color_raw(
        chTitle: , 
        iconType: , 
        chSubtitle: , 
        isImportant: , 
        titleColor: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AE0589093A2E088u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AE0589093A2E088u64;

        invoke_raw_typed!(
            hash,
                chTitle, 
                iconType, 
                chSubtitle, 
                isImportant, 
                titleColor, 
                p5
        )
    }
}

/// Allows HUD to be drawn over screen fade every frame this function is called.



pub fn draw_hud_over_fade_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF4F34A85CA2970Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF4F34A85CA2970Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn draw_hud_over_fade_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF4F34A85CA2970Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF4F34A85CA2970Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn reset_hud_component_values_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x450930E616475D0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x450930E616475D0Du64;
        
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
pub fn reset_hud_component_values_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x450930E616475D0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x450930E616475D0Du64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ```
After applying the properties to the text (See UI::SET_TEXT_), this will draw the text in the applied position. Also 0.0f < x, y < 1.0f, percentage of the axis.  
```

```
NativeDB Added Parameter 3: int p2
```



pub fn end_text_command_display_text_safe(
        
        
            x: 
        , 
        
        
            y: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD015E5BB0D96A57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD015E5BB0D96A57u64;
        
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
pub fn end_text_command_display_text_raw(
        x: , 
        y: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD015E5BB0D96A57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD015E5BB0D96A57u64;

        invoke_raw_typed!(
            hash,
                x, 
                y
        )
    }
}

/// SET_TEXT_DROP_SHADOW native function



pub fn set_text_drop_shadow_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CA3E9EAC9D93E5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CA3E9EAC9D93E5Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_text_drop_shadow_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CA3E9EAC9D93E5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CA3E9EAC9D93E5Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// instructionalKey enum list:
```
Buttons = {
      Empty = 0,
      Select = 1, -- (RETURN)
      Ok = 2, -- (RETURN)
      Yes = 4, -- (RETURN)
      Back = 8, -- (ESC)
      Cancel = 16, -- (ESC)
      No = 32, -- (ESC)
      RetrySpace = 64, -- (SPACE)
      Restart = 128, -- (SPACE)
      Skip = 256, -- (SPACE)
      Quit = 512, -- (ESC)
      Adjust = 1024, -- (ARROWS)
      SpaceKey = 2048, -- (SPACE)
      Share = 4096, -- (SPACE)
      SignIn = 8192, -- (SPACE)
      Continue = 16384, -- (RETURN)
      AdjustLeftRight = 32768, -- (SCROLL L/R)
      AdjustUpDown = 65536, -- (SCROLL U/D)
      Overwrite = 131072, -- (SPACE)
      SocialClubSignup = 262144, -- (RETURN)
      Confirm = 524288, -- (RETURN)
      Queue = 1048576, -- (RETURN)
      RetryReturn = 2097152, -- (RETURN)
      BackEsc = 4194304, -- (ESC)
      SocialClub = 8388608, -- (RETURN)
      Spectate = 16777216, -- (SPACE)
      OkEsc = 33554432, -- (ESC)
      CancelTransfer = 67108864, -- (ESC)
      LoadingSpinner = 134217728,
      NoReturnToGTA = 268435456, -- (ESC)
      CancelEsc = 536870912, -- (ESC)
}

Alt = {
      Empty = 0,
      No = 1, -- (SPACE)
      Host = 2, -- (ESC)
      SearchForJob = 4, -- (RETURN)
      ReturnKey = 8, -- (TURN)
      Freemode = 16, -- (ESC)
}
```



pub fn _set_warning_message_with_alert_safe(
        
        
            labelTitle: 
        , 
        
        
            labelMsg: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            labelMsg2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15803FEC3B9A872Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15803FEC3B9A872Bu64;
        
        let result = invoke_raw!(
            hash,
                labelTitle, 
                labelMsg, 
                p2, 
                p3, 
                labelMsg2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_warning_message_with_alert_raw(
        labelTitle: , 
        labelMsg: , 
        p2: , 
        p3: , 
        labelMsg2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15803FEC3B9A872Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15803FEC3B9A872Bu64;

        invoke_raw_typed!(
            hash,
                labelTitle, 
                labelMsg, 
                p2, 
                p3, 
                labelMsg2
        )
    }
}

/// ## Parameters
*



pub fn add_text_component_integer_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03B504CF259931BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03B504CF259931BCu64;
        
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
pub fn add_text_component_integer_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03B504CF259931BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03B504CF259931BCu64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// _REMOVE_WARNING_MESSAGE_LIST_ITEMS native function



pub fn _remove_warning_message_list_items_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6EF54AB721DC6242u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6EF54AB721DC6242u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _remove_warning_message_list_items_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6EF54AB721DC6242u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6EF54AB721DC6242u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
After some testing, looks like you need to use CEIL() on the rotation (vehicle/ped heading) before using it there.
```



pub fn set_blip_rotation_safe(
        
        
            blip: 
        , 
        
        
            rotation: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF87683CDF73C3F6Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF87683CDF73C3F6Eu64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                rotation
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_rotation_raw(
        blip: , 
        rotation: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF87683CDF73C3F6Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF87683CDF73C3F6Eu64;

        invoke_raw_typed!(
            hash,
                blip, 
                rotation
        )
    }
}

/// Highlights a blip by a half cyan circle on the right side of the blip. ![](https://i.imgur.com/FrV9M4e.png) Indicating that that player is a friend (in GTA:O). This color can not be changed.

To toggle the left side (crew member indicator) of the half circle around the blip, use: [`SHOW_CREW_INDICATOR_ON_BLIP`](#_0xDCFB5D4DB8BF367E).



pub fn show_friend_indicator_on_blip_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23C3EB807312F01Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23C3EB807312F01Au64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn show_friend_indicator_on_blip_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23C3EB807312F01Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23C3EB807312F01Au64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// Sets the interval in milliseconds before flashing the blip.



pub fn set_blip_flash_interval_safe(
        
        
            blip: 
        , 
        
        
            interval: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA51DB313C010A7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA51DB313C010A7Eu64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                interval
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_flash_interval_raw(
        blip: , 
        interval: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA51DB313C010A7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA51DB313C010A7Eu64;

        invoke_raw_typed!(
            hash,
                blip, 
                interval
        )
    }
}

/// OPEN_ONLINE_POLICIES_MENU native function



pub fn open_online_policies_menu_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x805D7CBB36FD6C4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x805D7CBB36FD6C4Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn open_online_policies_menu_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x805D7CBB36FD6C4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x805D7CBB36FD6C4Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// HIDE_LOADING_ON_FADE_THIS_FRAME native function



pub fn hide_loading_on_fade_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B0311D3CDC4648Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B0311D3CDC4648Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hide_loading_on_fade_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B0311D3CDC4648Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B0311D3CDC4648Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p1 was always -1
```



pub fn add_text_component_substring_phone_number_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x761B77454205A61Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x761B77454205A61Du64;
        
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
pub fn add_text_component_substring_phone_number_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x761B77454205A61Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x761B77454205A61Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0xe4c3b169876d33d7_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4C3B169876D33D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4C3B169876D33D7u64;
        
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
pub fn _0xe4c3b169876d33d7_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4C3B169876D33D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4C3B169876D33D7u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Related to notification color flashing, setting p0 to 0 invalidates a `_SET_NOTIFICATION_FLASH_COLOR` call for the target notification.



pub fn _thefeed_set_animpostfx_count_safe(
        
        
            count: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17AD8C9706BDD88Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17AD8C9706BDD88Au64;
        
        let result = invoke_raw!(
            hash,
                count
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _thefeed_set_animpostfx_count_raw(
        count: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17AD8C9706BDD88Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17AD8C9706BDD88Au64;

        invoke_raw_typed!(
            hash,
                count
        )
    }
}

/// ```
NativeDB Introduced: v2060
```



pub fn _get_blip_rotation_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x003E92BA477F9D7Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x003E92BA477F9D7Fu64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_blip_rotation_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x003E92BA477F9D7Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x003E92BA477F9D7Fu64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// Starts a new GPS multi-route, allowing you to create custom GPS paths.
GPS functions like the waypoint, except it can contain multiple points it's forced to go through.
Once the player has passed a point, the GPS will no longer force its path through it.

Works independently from the player-placed waypoint and blip routes.



pub fn start_gps_multi_route_safe(
        
        
            hudColor: 
        , 
        
        
            routeFromPlayer: 
        , 
        
        
            displayOnFoot: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D3D15AF7BCAAF83u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D3D15AF7BCAAF83u64;
        
        let result = invoke_raw!(
            hash,
                hudColor, 
                routeFromPlayer, 
                displayOnFoot
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_gps_multi_route_raw(
        hudColor: , 
        routeFromPlayer: , 
        displayOnFoot: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D3D15AF7BCAAF83u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D3D15AF7BCAAF83u64;

        invoke_raw_typed!(
            hash,
                hudColor, 
                routeFromPlayer, 
                displayOnFoot
        )
    }
}

/// Shows a help message for one frame.
Do note that this message doesn't get added to the Pause Menu info section.



pub fn display_help_text_this_frame_safe(
        
        
            pTextLabel: 
        , 
        
        
            bCurvedWindow: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x960C9FF8F616E41Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x960C9FF8F616E41Cu64;
        
        let result = invoke_raw!(
            hash,
                pTextLabel, 
                bCurvedWindow
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn display_help_text_this_frame_raw(
        pTextLabel: , 
        bCurvedWindow: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x960C9FF8F616E41Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x960C9FF8F616E41Cu64;

        invoke_raw_typed!(
            hash,
                pTextLabel, 
                bCurvedWindow
        )
    }
}

/// Sets current pause menu page/component to the specified value.
Available page IDs: https://pastebin.com/qxuhwjPT



pub fn pause_menuception_go_deeper_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77F16B447824DA6Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77F16B447824DA6Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pause_menuception_go_deeper_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77F16B447824DA6Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77F16B447824DA6Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0xd2049635deb9c375_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2049635DEB9C375u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2049635DEB9C375u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xd2049635deb9c375_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2049635DEB9C375u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2049635DEB9C375u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Doesn't work if the label text of gxtEntry is >= 80.  
```



pub fn set_blip_name_from_text_file_safe(
        
        
            blip: 
        , 
        
        
            gxtEntry: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAA0FFE120D92784u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAA0FFE120D92784u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                gxtEntry
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_name_from_text_file_raw(
        blip: , 
        gxtEntry: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAA0FFE120D92784u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAA0FFE120D92784u64;

        invoke_raw_typed!(
            hash,
                blip, 
                gxtEntry
        )
    }
}

/// ## Parameters
*



pub fn set_radar_zoom_to_blip_safe(
        
        
            blip: 
        , 
        
        
            zoom: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF98E4B3E56AFC7B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF98E4B3E56AFC7B1u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                zoom
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_radar_zoom_to_blip_raw(
        blip: , 
        zoom: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF98E4B3E56AFC7B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF98E4B3E56AFC7B1u64;

        invoke_raw_typed!(
            hash,
                blip, 
                zoom
        )
    }
}

/// Uses the `SOCIAL_CLUB2` scaleform.



pub fn open_social_club_menu_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75D3691713C3B05Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75D3691713C3B05Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn open_social_club_menu_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75D3691713C3B05Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75D3691713C3B05Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// CLOSE_SOCIAL_CLUB_MENU native function



pub fn close_social_club_menu_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2B32BE3FC1626C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2B32BE3FC1626C6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn close_social_club_menu_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2B32BE3FC1626C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2B32BE3FC1626C6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// THEFEED_FORCE_RENDER_ON native function



pub fn thefeed_force_render_on_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA13C11E1B5C06BFCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA13C11E1B5C06BFCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_force_render_on_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA13C11E1B5C06BFCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA13C11E1B5C06BFCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// **displayId Behaviour**

| display ID 	| Behaviour                                                   	|
|------------	|-------------------------------------------------------------	|
| 0          	| Doesn't show up, ever, anywhere.                            	|
| 1          	| Doesn't show up, ever, anywhere.                            	|
| 2          	| Shows on both main map and minimap. (Selectable on map)     	|
| 3          	| Shows on main map only. (Selectable on map)                 	|
| 4          	| Shows on main map only. (Selectable on map)                 	|
| 5          	| Shows on minimap only.                                      	|
| 6          	| Shows on both main map and minimap. (Selectable on map)     	|
| 7          	| Doesn't show up, ever, anywhere.                            	|
| 8          	| Shows on both main map and minimap. (Not selectable on map) 	|
| 9          	| Shows on minimap only.                                      	|
| 10         	| Shows on both main map and minimap. (Not selectable on map) 	|

Anything higher than 10 seems to be exactly the same as 10.

Rockstar seem to only use 0, 2, 3, 4, 5 and 8 in the decompiled scripts.



pub fn set_blip_display_safe(
        
        
            blip: 
        , 
        
        
            displayId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9029B2F3DA924928u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9029B2F3DA924928u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                displayId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_display_raw(
        blip: , 
        displayId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9029B2F3DA924928u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9029B2F3DA924928u64;

        invoke_raw_typed!(
            hash,
                blip, 
                displayId
        )
    }
}

/// Removes the gamer tag associated with the provided ID. This does not happen instantly. Use [IS_MP_GAMER_TAG_FREE](#_0x595B5178E412E199) to determine when the ID is free for reuse.



pub fn remove_mp_gamer_tag_safe(
        
        
            gamerTagId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31698AA80E0223F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31698AA80E0223F8u64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_mp_gamer_tag_raw(
        gamerTagId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31698AA80E0223F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31698AA80E0223F8u64;

        invoke_raw_typed!(
            hash,
                gamerTagId
        )
    }
}

/// ## Return value



pub fn get_standard_blip_enum_id_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A9923385BDB9DADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A9923385BDB9DADu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_standard_blip_enum_id_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A9923385BDB9DADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A9923385BDB9DADu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Finalizes a text command started with [`BEGIN_TEXT_COMMAND_SET_BLIP_NAME`](#_0xF9113A30DE5C6670), setting the name
of the specified blip.



pub fn end_text_command_set_blip_name_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC38B49BCB83BC9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC38B49BCB83BC9Bu64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_set_blip_name_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC38B49BCB83BC9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC38B49BCB83BC9Bu64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// SHOW_*

```
NativeDB Introduced: v1734
```



pub fn _show_scripted_hud_component_this_frame_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F38DCA127DAAEA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F38DCA127DAAEA2u64;
        
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
pub fn _show_scripted_hud_component_this_frame_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F38DCA127DAAEA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F38DCA127DAAEA2u64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ```
SET_*
```



pub fn _0x04655f9d075d0ae5_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04655F9D075D0AE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04655F9D075D0AE5u64;
        
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
pub fn _0x04655f9d075d0ae5_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04655F9D075D0AE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04655F9D075D0AE5u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Removes multiplayer cash hud each frame  
```



pub fn remove_multiplayer_hud_cash_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x968F270E39141ECAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x968F270E39141ECAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_multiplayer_hud_cash_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x968F270E39141ECAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x968F270E39141ECAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Controls the visibility of the "Contact" instructional buttons on the map screen.

```
NativeDB Introduced: 2545
```



pub fn show_contact_instructional_button_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC772A904CDE1186Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC772A904CDE1186Fu64;
        
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
pub fn show_contact_instructional_button_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC772A904CDE1186Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC772A904CDE1186Fu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn thefeed_set_scripted_menu_height_safe(
        
        
            pos: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55598D21339CB998u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55598D21339CB998u64;
        
        let result = invoke_raw!(
            hash,
                pos
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_set_scripted_menu_height_raw(
        pos: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55598D21339CB998u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55598D21339CB998u64;

        invoke_raw_typed!(
            hash,
                pos
        )
    }
}

/// Toggles the pause menu ped sleep state for frontend menus.

[Example GIF](https://vespura.com/hi/i/2019-04-01_15-51_8ed38_1014.gif)



pub fn set_pause_menu_ped_sleep_state_safe(
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xECF128344E9FF9F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xECF128344E9FF9F1u64;
        
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
pub fn set_pause_menu_ped_sleep_state_raw(
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xECF128344E9FF9F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xECF128344E9FF9F1u64;

        invoke_raw_typed!(
            hash,
                state
        )
    }
}

/// Adds an arbitrary string as a text component placeholder, replacing `~a~` in the current text command's text label.

See the documentation on text formatting for more information.



pub fn add_text_component_substring_player_name_safe(
        
        
            text: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C188BE134E074AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C188BE134E074AAu64;
        
        let result = invoke_raw!(
            hash,
                text
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_text_component_substring_player_name_raw(
        text: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C188BE134E074AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C188BE134E074AAu64;

        invoke_raw_typed!(
            hash,
                text
        )
    }
}

/// Overrides the position of the main player blip for the current frame.



pub fn set_fake_pausemap_player_position_this_frame_safe(
        
        
            x: 
        , 
        
        
            y: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77E2DD177910E1CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77E2DD177910E1CFu64;
        
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
pub fn set_fake_pausemap_player_position_this_frame_raw(
        x: , 
        y: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77E2DD177910E1CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77E2DD177910E1CFu64;

        invoke_raw_typed!(
            hash,
                x, 
                y
        )
    }
}

/// ## Parameters
*



pub fn _set_mission_name_2_safe(
        
        
            p0: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE45087D85F468BC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE45087D85F468BC2u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_mission_name_2_raw(
        p0: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE45087D85F468BC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE45087D85F468BC2u64;

        invoke_raw_typed!(
            hash,
                p0, 
                name
        )
    }
}

/// ```c
enum ePauseMenuState
{
    PM_INACTIVE 	 =  0,
    PM_STARTING_UP 	 =  5,
    PM_RESTARTING 	 = 10,
    PM_READY 		 = 15,
    PM_IN_STORE 	 = 20,
    PM_IN_SC_MENU 	 = 25,
    PM_SHUTTING_DOWN = 30,
    PM_IN_VIDEOEDITOR = 35,
}
```



pub fn get_pause_menu_state_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x272ACD84970869C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x272ACD84970869C5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_pause_menu_state_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x272ACD84970869C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x272ACD84970869C5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// This native is deprecated.

If you're looking to:
* Toggle the pause menu on, use [`ACTIVATE_FRONTEND_MENU`](#_0xEF01D36B9C9D0C7B)
* Toggle the pause menu off, use [`SET_FRONTEND_ACTIVE`](#_0x745711A75AB09277)
* Disable toggling the pause menu, use [`DISABLE_FRONTEND_THIS_FRAME`](#_0x6D3465A73092F0E6)



pub fn set_pause_menu_active_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF47FC56C71569CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF47FC56C71569CFu64;
        
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
pub fn set_pause_menu_active_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF47FC56C71569CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF47FC56C71569CFu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value



pub fn is_radar_hidden_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x157F93B036700462u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x157F93B036700462u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_radar_hidden_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x157F93B036700462u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x157F93B036700462u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Sets the color of HUD_COLOUR_SCRIPT_VARIABLE
```



pub fn set_script_variable_hud_colour_safe(
        
        
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
        let hash = 0xD68A5FF8A3A89874u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD68A5FF8A3A89874u64;
        
        let result = invoke_raw!(
            hash,
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
pub fn set_script_variable_hud_colour_raw(
        r: , 
        g: , 
        b: , 
        a: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD68A5FF8A3A89874u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD68A5FF8A3A89874u64;

        invoke_raw_typed!(
            hash,
                r, 
                g, 
                b, 
                a
        )
    }
}

/// ## Parameters
*



pub fn add_point_to_gps_custom_route_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x311438A071DD9B1Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x311438A071DD9B1Au64;
        
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
pub fn add_point_to_gps_custom_route_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x311438A071DD9B1Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x311438A071DD9B1Au64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// Sets the displayed sprite for a specific blip.

There's a [list of sprites](https://docs.fivem.net/game-references/blips/) on the FiveM documentation site.



pub fn set_blip_sprite_safe(
        
        
            blip: 
        , 
        
        
            spriteId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF735600A4696DAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF735600A4696DAFu64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                spriteId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_sprite_raw(
        blip: , 
        spriteId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF735600A4696DAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF735600A4696DAFu64;

        invoke_raw_typed!(
            hash,
                blip, 
                spriteId
        )
    }
}

/// ## Return value



pub fn is_hud_hidden_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA86478C6958735C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA86478C6958735C5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_hud_hidden_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA86478C6958735C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA86478C6958735C5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Checks if the passed gxt name exists in the game files.  
```



pub fn does_text_label_exist_safe(
        
        
            gxt: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC09CA973C564252u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC09CA973C564252u64;
        
        let result = invoke_raw!(
            hash,
                gxt
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_text_label_exist_raw(
        gxt: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC09CA973C564252u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC09CA973C564252u64;

        invoke_raw_typed!(
            hash,
                gxt
        )
    }
}

/// ## Return value



pub fn is_radar_preference_switched_on_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9EB6522EA68F22FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9EB6522EA68F22FEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_radar_preference_switched_on_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9EB6522EA68F22FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9EB6522EA68F22FEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_ability_bar_value_safe(
        
        
            value: 
        , 
        
        
            maxValue: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9969599CCFF5D85Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9969599CCFF5D85Eu64;
        
        let result = invoke_raw!(
            hash,
                value, 
                maxValue
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ability_bar_value_raw(
        value: , 
        maxValue: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9969599CCFF5D85Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9969599CCFF5D85Eu64;

        invoke_raw_typed!(
            hash,
                value, 
                maxValue
        )
    }
}

/// ```
Sets a global that disables many weapon input tasks (shooting, aiming, etc.). Does not work with vehicle weapons, only used in selector.ysc
```



pub fn _hud_weapon_wheel_ignore_control_input_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14C9FDCC41F81F63u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14C9FDCC41F81F63u64;
        
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
pub fn _hud_weapon_wheel_ignore_control_input_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14C9FDCC41F81F63u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14C9FDCC41F81F63u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Example, only occurrence in the scripts:

```
v_8 = UI::END_TEXT_COMMAND_THEFEED_POST_MESSAGETEXT_TU("CHAR_SOCIAL_CLUB", "CHAR_SOCIAL_CLUB", 0, 0, &v_9, "", a_5);
```

Example result:
![](https://i.imgur.com/YrN4Bcm.png)



pub fn end_text_command_thefeed_post_messagetext_tu_safe(
        
        
            picTxd: 
        , 
        
        
            picTxn: 
        , 
        
        
            flash: 
        , 
        
        
            iconType: 
        , 
        
        
            nameStr: 
        , 
        
        
            subtitleStr: 
        , 
        
        
            durationMultiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E6611149DB3DB6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E6611149DB3DB6Bu64;
        
        let result = invoke_raw!(
            hash,
                picTxd, 
                picTxn, 
                flash, 
                iconType, 
                nameStr, 
                subtitleStr, 
                durationMultiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_messagetext_tu_raw(
        picTxd: , 
        picTxn: , 
        flash: , 
        iconType: , 
        nameStr: , 
        subtitleStr: , 
        durationMultiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E6611149DB3DB6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E6611149DB3DB6Bu64;

        invoke_raw_typed!(
            hash,
                picTxd, 
                picTxn, 
                flash, 
                iconType, 
                nameStr, 
                subtitleStr, 
                durationMultiplier
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _get_warning_message_title_hash_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x81DF9ABA6C83DFF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x81DF9ABA6C83DFF9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_warning_message_title_hash_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x81DF9ABA6C83DFF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x81DF9ABA6C83DFF9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_blip_hidden_on_legend_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54318C915D27E4CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54318C915D27E4CEu64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_hidden_on_legend_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54318C915D27E4CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54318C915D27E4CEu64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// Sets the visibility of all components of the gamer tag to the specified value.



pub fn _set_mp_gamer_tag_visibility_all_safe(
        
        
            gamerTagId: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE76FF7E6A0166B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE76FF7E6A0166B0u64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_mp_gamer_tag_visibility_all_raw(
        gamerTagId: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE76FF7E6A0166B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE76FF7E6A0166B0u64;

        invoke_raw_typed!(
            hash,
                gamerTagId, 
                toggle
        )
    }
}

/// ```
This function and the one below it are for after you receive an invite, not sending it.  
p0 = 1 or 0  
nothin doin.   
int invite(Player player)  
	{  
int iVar2, iVar3;  
networkHandleMgr handle;  
NETWORK_HANDLE_FROM_PLAYER(player, &handle.netHandle, 13);  
networkClanMgr clan;  
char *playerName = GET_PLAYER_NAME(player);  
_SET_NOTIFICATION_TEXT_ENTRY("STRING");  
_SET_NOTIFACTION_COLOR_NEXT(0);  
ADD_TEXT_COMPONENT_SUBSTRING_PLAYER_NAME(playerName);  
if (NETWORK_CLAN_PLAYER_GET_DESC(&clan, 35, &handle.netHandle))  
{  
	iVar2 = 0;  
	if (ARE_STRINGS_EQUAL(clan.unk22, "Leader") && clan.unk30 == 0)  
	{  
iVar2 = 1;  
	}  
	if (clan.unk21 > 0)  
	{  
iVar3 = 0;  
	}  
	else  
	{  
iVar3 = 1;  
	}  
	BOOL unused = _0x54E79E9C(&clan.clanHandle, 35);  
	return _DRAW_NOTIFICATION_APARTMENT_INVITE(iVar3, 0 /*unused*/, &clan.unk17, clan.unk30, iVar2, 0, clan.clanHandle, 0, 0, 0);  
}  
	}  
```



pub fn end_text_command_thefeed_post_crewtag_safe(
        
        
            crewTypeIsPrivate: 
        , 
        
        
            crewTagContainsRockstar: 
        , 
        
        
            crewTag: 
        , 
        
        
            rank: 
        , 
        
        
            hasFounderStatus: 
        , 
        
        
            isImportant: 
        , 
        
        
            clanHandle: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97C9E4E7024A8F2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97C9E4E7024A8F2Cu64;
        
        let result = invoke_raw!(
            hash,
                crewTypeIsPrivate, 
                crewTagContainsRockstar, 
                crewTag, 
                rank, 
                hasFounderStatus, 
                isImportant, 
                clanHandle, 
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
pub fn end_text_command_thefeed_post_crewtag_raw(
        crewTypeIsPrivate: , 
        crewTagContainsRockstar: , 
        crewTag: , 
        rank: , 
        hasFounderStatus: , 
        isImportant: , 
        clanHandle: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97C9E4E7024A8F2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97C9E4E7024A8F2Cu64;

        invoke_raw_typed!(
            hash,
                crewTypeIsPrivate, 
                crewTagContainsRockstar, 
                crewTag, 
                rank, 
                hasFounderStatus, 
                isImportant, 
                clanHandle, 
                r, 
                g, 
                b
        )
    }
}

/// [List of picture names](https://pastebin.com/XdpJVbHz)


Example result:


![](https://i.imgur.com/SdEZ22m.png)



pub fn end_text_command_thefeed_post_stats_safe(
        
        
            statTitle: 
        , 
        
        
            iconEnum: 
        , 
        
        
            stepVal: 
        , 
        
        
            barValue: 
        , 
        
        
            isImportant: 
        , 
        
        
            picTxd: 
        , 
        
        
            picTxn: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B7E9A4EAAA93C89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B7E9A4EAAA93C89u64;
        
        let result = invoke_raw!(
            hash,
                statTitle, 
                iconEnum, 
                stepVal, 
                barValue, 
                isImportant, 
                picTxd, 
                picTxn
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_stats_raw(
        statTitle: , 
        iconEnum: , 
        stepVal: , 
        barValue: , 
        isImportant: , 
        picTxd: , 
        picTxn: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B7E9A4EAAA93C89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B7E9A4EAAA93C89u64;

        invoke_raw_typed!(
            hash,
                statTitle, 
                iconEnum, 
                stepVal, 
                barValue, 
                isImportant, 
                picTxd, 
                picTxn
        )
    }
}

/// ```
HAS_S*
```



pub fn _0x214cd562a939246a_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x214CD562A939246Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x214CD562A939246Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x214cd562a939246a_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x214CD562A939246Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x214CD562A939246Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// DISABLE_FRONTEND_THIS_FRAME native function



pub fn disable_frontend_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D3465A73092F0E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D3465A73092F0E6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_frontend_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D3465A73092F0E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D3465A73092F0E6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_blip_info_id_coord_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA7C7F0AADF25D09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA7C7F0AADF25D09u64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_blip_info_id_coord_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA7C7F0AADF25D09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA7C7F0AADF25D09u64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// Sets the health bar of a gamer tag to show the health of the last (or current) vehicle of the ped the gamer tag is attached to.
The vehicle health value is stored separate from the player health and using it won't clear any player health overrides.



pub fn _set_mp_gamer_tag_use_vehicle_health_safe(
        
        
            gamerTagId: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA67F9C46D612B6F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA67F9C46D612B6F1u64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_mp_gamer_tag_use_vehicle_health_raw(
        gamerTagId: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA67F9C46D612B6F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA67F9C46D612B6F1u64;

        invoke_raw_typed!(
            hash,
                gamerTagId, 
                toggle
        )
    }
}

/// Examples result:

![](https://i.imgur.com/skY6vAJ.png)



pub fn set_blip_category_safe(
        
        
            blip: 
        , 
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x234CDD44D996FD9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x234CDD44D996FD9Au64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_category_raw(
        blip: , 
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x234CDD44D996FD9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x234CDD44D996FD9Au64;

        invoke_raw_typed!(
            hash,
                blip, 
                index
        )
    }
}

/// Starts a new GPS custom-route, allowing you to plot lines on the map.
Lines are drawn directly between points.
The GPS custom route works like the GPS multi route, except it does not follow roads.



pub fn start_gps_custom_route_safe(
        
        
            hudColor: 
        , 
        
        
            displayOnFoot: 
        , 
        
        
            followPlayer: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB34E8D56FC13B08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB34E8D56FC13B08u64;
        
        let result = invoke_raw!(
            hash,
                hudColor, 
                displayOnFoot, 
                followPlayer
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_gps_custom_route_raw(
        hudColor: , 
        displayOnFoot: , 
        followPlayer: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB34E8D56FC13B08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB34E8D56FC13B08u64;

        invoke_raw_typed!(
            hash,
                hudColor, 
                displayOnFoot, 
                followPlayer
        )
    }
}

/// ### Arrow Positions
* 0 = Off / No arrow
* 1 = Top
* 2 = Left
* 3 = Bottom
* 4 = Right



pub fn set_floating_help_text_style_safe(
        
        
            hudIndex: 
        , 
        
        
            style: 
        , 
        
        
            hudColor: 
        , 
        
        
            alpha: 
        , 
        
        
            arrowPosition: 
        , 
        
        
            boxOffset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x788E7FD431BD67F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x788E7FD431BD67F1u64;
        
        let result = invoke_raw!(
            hash,
                hudIndex, 
                style, 
                hudColor, 
                alpha, 
                arrowPosition, 
                boxOffset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_floating_help_text_style_raw(
        hudIndex: , 
        style: , 
        hudColor: , 
        alpha: , 
        arrowPosition: , 
        boxOffset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x788E7FD431BD67F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x788E7FD431BD67F1u64;

        invoke_raw_typed!(
            hash,
                hudIndex, 
                style, 
                hudColor, 
                alpha, 
                arrowPosition, 
                boxOffset
        )
    }
}

/// CLEAR_DYNAMIC_PAUSE_MENU_ERROR_MESSAGE native function



pub fn clear_dynamic_pause_menu_error_message_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7792424AA0EAC32Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7792424AA0EAC32Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_dynamic_pause_menu_error_message_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7792424AA0EAC32Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7792424AA0EAC32Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_health_hud_display_values_safe(
        
        
            health: 
        , 
        
        
            capacity: 
        , 
        
        
            wasAdded: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F5CC444DCAAA8F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F5CC444DCAAA8F2u64;
        
        let result = invoke_raw!(
            hash,
                health, 
                capacity, 
                wasAdded
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_health_hud_display_values_raw(
        health: , 
        capacity: , 
        wasAdded: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F5CC444DCAAA8F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F5CC444DCAAA8F2u64;

        invoke_raw_typed!(
            hash,
                health, 
                capacity, 
                wasAdded
        )
    }
}

/// Sets a warning message for one frame with header and upper buttons bit field that don't fit the standard 32 bit set.



pub fn set_warning_message_with_header_extended_safe(
        
        
            headerTextLabel: 
        , 
        
        
            line1TextLabel: 
        , 
        
        
            buttonsBitField: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38B55259C2E078EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38B55259C2E078EDu64;
        
        let result = invoke_raw!(
            hash,
                headerTextLabel, 
                line1TextLabel, 
                buttonsBitField
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_warning_message_with_header_extended_raw(
        headerTextLabel: , 
        line1TextLabel: , 
        buttonsBitField: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38B55259C2E078EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38B55259C2E078EDu64;

        invoke_raw_typed!(
            hash,
                headerTextLabel, 
                line1TextLabel, 
                buttonsBitField
        )
    }
}

/// CLEAR_GPS_PLAYER_WAYPOINT native function



pub fn clear_gps_player_waypoint_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF4FB7C8CDFA3DA7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF4FB7C8CDFA3DA7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_gps_player_waypoint_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF4FB7C8CDFA3DA7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF4FB7C8CDFA3DA7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns the string length of the string from the gxt string .  
```



pub fn get_length_of_string_with_this_text_label_safe(
        
        
            gxt: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x801BD273D3A23F74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x801BD273D3A23F74u64;
        
        let result = invoke_raw!(
            hash,
                gxt
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_length_of_string_with_this_text_label_raw(
        gxt: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x801BD273D3A23F74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x801BD273D3A23F74u64;

        invoke_raw_typed!(
            hash,
                gxt
        )
    }
}

/// ```
List of picNames pastebin.com/XdpJVbHz  
flash is a bool for fading in.  
iconTypes:  
1 : Chat Box  
2 : Email  
3 : Add Friend Request  
4 : Nothing  
5 : Nothing  
6 : Nothing  
7 : Right Jumping Arrow  
8 : RP Icon  
9 : $ Icon  
"sender" is the very top header. This can be any old string.  
"subject" is the header under the sender.  
"duration" is a multiplier, so 1.0 is normal, 2.0 is twice as long (very slow), and 0.5 is half as long.  
"clanTag" shows a crew tag in the "sender" header, after the text. You need to use 3 underscores as padding. Maximum length of this field seems to be 7. (e.g. "MK" becomes "___MK", "ACE" becomes "___ACE", etc.)  
```



pub fn end_text_command_thefeed_post_messagetext_with_crew_tag_safe(
        
        
            picTxd: 
        , 
        
        
            picTxn: 
        , 
        
        
            flash: 
        , 
        
        
            iconType: 
        , 
        
        
            nameStr: 
        , 
        
        
            subtitleStr: 
        , 
        
        
            duration: 
        , 
        
        
            crewPackedStr: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CBF7BADE20DB93Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CBF7BADE20DB93Eu64;
        
        let result = invoke_raw!(
            hash,
                picTxd, 
                picTxn, 
                flash, 
                iconType, 
                nameStr, 
                subtitleStr, 
                duration, 
                crewPackedStr
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_messagetext_with_crew_tag_raw(
        picTxd: , 
        picTxn: , 
        flash: , 
        iconType: , 
        nameStr: , 
        subtitleStr: , 
        duration: , 
        crewPackedStr: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CBF7BADE20DB93Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CBF7BADE20DB93Eu64;

        invoke_raw_typed!(
            hash,
                picTxd, 
                picTxn, 
                flash, 
                iconType, 
                nameStr, 
                subtitleStr, 
                duration, 
                crewPackedStr
        )
    }
}

/// ```
Not present in retail version of the game, actual definiton seems to be
_LOG_DEBUG_INFO(const char* category, const char* debugText);
```



pub fn _log_debug_info_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2162C446DFDF38FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2162C446DFDF38FDu64;
        
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
pub fn _log_debug_info_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2162C446DFDF38FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2162C446DFDF38FDu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Returns the length of the string passed (much like strlen).  
```



pub fn get_length_of_literal_string_safe(
        
        
            string: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF030907CCBB8A9FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF030907CCBB8A9FDu64;
        
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
pub fn get_length_of_literal_string_raw(
        string: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF030907CCBB8A9FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF030907CCBB8A9FDu64;

        invoke_raw_typed!(
            hash,
                string
        )
    }
}

/// **NOTE:** This must be called before you open the text input box.



pub fn set_allow_comma_on_text_input_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x577599CCED639CA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x577599CCED639CA2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_allow_comma_on_text_input_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x577599CCED639CA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x577599CCED639CA2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// CLEAR_ALL_HELP_MESSAGES native function



pub fn clear_all_help_messages_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6178F68A87A4D3A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6178F68A87A4D3A0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_all_help_messages_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6178F68A87A4D3A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6178F68A87A4D3A0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns whether or not the text chat (MULTIPLAYER_CHAT Scaleform component) is active.  
```



pub fn _is_multiplayer_chat_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB118AF58B5F332A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB118AF58B5F332A1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_multiplayer_chat_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB118AF58B5F332A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB118AF58B5F332A1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Displays loading screen tips, requires `0x56C8B608CFD49854` to be called beforehand.



pub fn _thefeed_display_loading_screen_tips_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15CFA549788D35EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15CFA549788D35EFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _thefeed_display_loading_screen_tips_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x15CFA549788D35EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x15CFA549788D35EFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Setter for 0xC2D2AD9EAAE265B8

SET_*
```



pub fn _0xcd74233600c4ea6b_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD74233600C4EA6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD74233600C4EA6Bu64;
        
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
pub fn _0xcd74233600c4ea6b_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD74233600C4EA6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD74233600C4EA6Bu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Allows opening the pause menu this frame, when the player is dead.



pub fn _allow_pause_menu_when_dead_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC3FDDED67BCFC63u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC3FDDED67BCFC63u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _allow_pause_menu_when_dead_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC3FDDED67BCFC63u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC3FDDED67BCFC63u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Enables drawing some hud components, such as help labels, this frame, when the player is dead.



pub fn _display_hud_when_dead_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7669F9E39DC17063u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7669F9E39DC17063u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _display_hud_when_dead_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7669F9E39DC17063u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7669F9E39DC17063u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn is_blip_on_minimap_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE41CA53051197A27u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE41CA53051197A27u64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_blip_on_minimap_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE41CA53051197A27u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE41CA53051197A27u64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// ```
NativeDB Added Parameter 7: int hudColorIndex1
NativeDB Added Parameter 8: int hudColorIndex2
```



pub fn end_text_command_thefeed_post_versus_tu_safe(
        
        
            ch1TXD: 
        , 
        
        
            ch1TXN: 
        , 
        
        
            val1: 
        , 
        
        
            ch2TXD: 
        , 
        
        
            ch2TXN: 
        , 
        
        
            val2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6871B0555B02996u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6871B0555B02996u64;
        
        let result = invoke_raw!(
            hash,
                ch1TXD, 
                ch1TXN, 
                val1, 
                ch2TXD, 
                ch2TXN, 
                val2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_versus_tu_raw(
        ch1TXD: , 
        ch1TXN: , 
        val1: , 
        ch2TXD: , 
        ch2TXN: , 
        val2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6871B0555B02996u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6871B0555B02996u64;

        invoke_raw_typed!(
            hash,
                ch1TXD, 
                ch1TXN, 
                val1, 
                ch2TXD, 
                ch2TXN, 
                val2
        )
    }
}

/// HIDE_MINIMAP_INTERIOR_MAP_THIS_FRAME native function



pub fn hide_minimap_interior_map_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20FE7FDFEEAD38C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20FE7FDFEEAD38C0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hide_minimap_interior_map_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20FE7FDFEEAD38C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20FE7FDFEEAD38C0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn is_named_rendertarget_registered_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78DCDC15C9F116B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78DCDC15C9F116B4u64;
        
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
pub fn is_named_rendertarget_registered_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78DCDC15C9F116B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78DCDC15C9F116B4u64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ## Parameters
*



pub fn link_named_rendertarget_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6C09E276AEB3F2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6C09E276AEB3F2Du64;
        
        let result = invoke_raw!(
            hash,
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn link_named_rendertarget_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6C09E276AEB3F2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6C09E276AEB3F2Du64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// ```
END_TEXT_COMMAND_*
Determines how many lines the text string will use when drawn on screen.
Must use 0x521FB041D93DD0E4 for setting up
```



pub fn _end_text_command_line_count_safe(
        
        
            x: 
        , 
        
        
            y: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9040DFB09BE75706u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9040DFB09BE75706u64;
        
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
pub fn _end_text_command_line_count_raw(
        x: , 
        y: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9040DFB09BE75706u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9040DFB09BE75706u64;

        invoke_raw_typed!(
            hash,
                x, 
                y
        )
    }
}

/// ```
Adds a timer (e.g. "00:00:00:000"). The appearance of the timer depends on the flags, which needs more research.  
```



pub fn add_text_component_substring_time_safe(
        
        
            timestamp: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1115F16B8AB9E8BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1115F16B8AB9E8BFu64;
        
        let result = invoke_raw!(
            hash,
                timestamp, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_text_component_substring_time_raw(
        timestamp: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1115F16B8AB9E8BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1115F16B8AB9E8BFu64;

        invoke_raw_typed!(
            hash,
                timestamp, 
                flags
        )
    }
}

/// ```
returns a notification handle, prints out a notification like below:  
type range: 0   
if you set type to 1, image goes from 0 - 39 - Xbox you can add text to  
example:   
UI::_0xD202B92CBF1D816F(1, 20, "Who you trynna get crazy with, ese? Don't you know I'm LOCO?!");  
```



pub fn _end_text_command_thefeed_post_replay_icon_safe(
        
        
            eType: 
        , 
        
        
            iIcon: 
        , 
        
        
            sTitle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD202B92CBF1D816Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD202B92CBF1D816Fu64;
        
        let result = invoke_raw!(
            hash,
                eType, 
                iIcon, 
                sTitle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _end_text_command_thefeed_post_replay_icon_raw(
        eType: , 
        iIcon: , 
        sTitle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD202B92CBF1D816Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD202B92CBF1D816Fu64;

        invoke_raw_typed!(
            hash,
                eType, 
                iIcon, 
                sTitle
        )
    }
}

/// Preview image:

![](https://i.imgur.com/1BTmdyv.png)

To change money value use [`STAT_SET_INT`](#_0xB3271D7AB655B441) with "MP0_WALLET_BALANCE" to whatever value you need to.



pub fn set_multiplayer_wallet_cash_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2D15BEF167E27BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2D15BEF167E27BCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_multiplayer_wallet_cash_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2D15BEF167E27BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2D15BEF167E27BCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_hud_preference_switched_on_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1930DFA731813EC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1930DFA731813EC4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_hud_preference_switched_on_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1930DFA731813EC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1930DFA731813EC4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Sets a global mode which makes the pause menu map show 'Destination' instead of 'Waypoint' in the key legend on the
bottom of the screen.

<!--

Name guess:

  - alphabetical function order, below [SET_USER_RADIO_CONTROL_ENABLED, SET_USE_HI_DOF], above SET_VARIABLE_ON_SOUND.

-->



pub fn _set_use_waypoint_as_destination_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CDD58146A436083u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CDD58146A436083u64;
        
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
pub fn _set_use_waypoint_as_destination_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CDD58146A436083u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CDD58146A436083u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_floating_help_text_to_entity_safe(
        
        
            hudIndex: 
        , 
        
        
            entity: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB094BC1DB4018240u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB094BC1DB4018240u64;
        
        let result = invoke_raw!(
            hash,
                hudIndex, 
                entity, 
                offsetX, 
                offsetY
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_floating_help_text_to_entity_raw(
        hudIndex: , 
        entity: , 
        offsetX: , 
        offsetY: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB094BC1DB4018240u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB094BC1DB4018240u64;

        invoke_raw_typed!(
            hash,
                hudIndex, 
                entity, 
                offsetX, 
                offsetY
        )
    }
}

/// ```
Checks if the specified gxt has loaded into the passed slot.  
```



pub fn has_this_additional_text_loaded_safe(
        
        
            gxt: 
        , 
        
        
            slot: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADBF060E2B30C5BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADBF060E2B30C5BCu64;
        
        let result = invoke_raw!(
            hash,
                gxt, 
                slot
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_this_additional_text_loaded_raw(
        gxt: , 
        slot: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADBF060E2B30C5BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADBF060E2B30C5BCu64;

        invoke_raw_typed!(
            hash,
                gxt, 
                slot
        )
    }
}

/// ## Parameters
*



pub fn set_frontend_active_safe(
        
        
            active: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x745711A75AB09277u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x745711A75AB09277u64;
        
        let result = invoke_raw!(
            hash,
                active
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_frontend_active_raw(
        active: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x745711A75AB09277u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x745711A75AB09277u64;

        invoke_raw_typed!(
            hash,
                active
        )
    }
}

/// CLEAR_PRINTS native function



pub fn clear_prints_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC33FA791322B9D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC33FA791322B9D9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_prints_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC33FA791322B9D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC33FA791322B9D9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p0 = 1 or 0  
crashes my game...  
this is for sending invites to network players - jobs/apartment/ect...   
return notification handle  
int invite(Player player)  
	{  
networkHandleMgr netHandle;  
networkClanMgr clan;  
char *playerName = GET_PLAYER_NAME(player);  
_SET_NOTIFICATION_TEXT_ENTRY("STRING");  
_SET_NOTIFACTION_COLOR_NEXT(1);  
ADD_TEXT_COMPONENT_SUBSTRING_PLAYER_NAME(playerName);  
NETWORK_HANDLE_FROM_PLAYER(player, &netHandle.netHandle, 13);  
if (NETWORK_CLAN_PLAYER_IS_ACTIVE(&netHandle.netHandle))  
{  
	NETWORK_CLAN_PLAYER_GET_DESC(&clan.clanHandle, 35, &netHandle.netHandle);  
	_DRAW_NOTIFICATION_CLAN_INVITE(0, _0x54E79E9C(&clan.clanHandle, 35), &clan.unk17, clan.isLeader, 0, 0, clan.clanHandle, playerName, 0, 0, 0);  
}  
	}  
```



pub fn end_text_command_thefeed_post_crewtag_with_game_name_safe(
        
        
            crewTypeIsPrivate: 
        , 
        
        
            crewTagContainsRockstar: 
        , 
        
        
            crewTag: 
        , 
        
        
            rank: 
        , 
        
        
            isLeader: 
        , 
        
        
            isImportant: 
        , 
        
        
            clanHandle: 
        , 
        
        
            gamerStr: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x137BC35589E34E1Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x137BC35589E34E1Eu64;
        
        let result = invoke_raw!(
            hash,
                crewTypeIsPrivate, 
                crewTagContainsRockstar, 
                crewTag, 
                rank, 
                isLeader, 
                isImportant, 
                clanHandle, 
                gamerStr, 
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
pub fn end_text_command_thefeed_post_crewtag_with_game_name_raw(
        crewTypeIsPrivate: , 
        crewTagContainsRockstar: , 
        crewTag: , 
        rank: , 
        isLeader: , 
        isImportant: , 
        clanHandle: , 
        gamerStr: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x137BC35589E34E1Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x137BC35589E34E1Eu64;

        invoke_raw_typed!(
            hash,
                crewTypeIsPrivate, 
                crewTagContainsRockstar, 
                crewTag, 
                rank, 
                isLeader, 
                isImportant, 
                clanHandle, 
                gamerStr, 
                r, 
                g, 
                b
        )
    }
}

/// Removes the blip from your map.



pub fn remove_blip_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x86A652570E5F25DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x86A652570E5F25DDu64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_blip_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x86A652570E5F25DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x86A652570E5F25DDu64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// THEFEED_RESUME native function



pub fn thefeed_resume_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1CD1E48E025E661u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1CD1E48E025E661u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_resume_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1CD1E48E025E661u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1CD1E48E025E661u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Related to displaying cash on the HUD
Always called before HUD::CHANGE_FAKE_MP_CASH in decompiled scripts
```



pub fn use_fake_mp_cash_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x170F541E1CADD1DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x170F541E1CADD1DEu64;
        
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
pub fn use_fake_mp_cash_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x170F541E1CADD1DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x170F541E1CADD1DEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x8410c5e0cd847b9d_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8410C5E0CD847B9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8410C5E0CD847B9Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8410c5e0cd847b9d_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8410C5E0CD847B9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8410C5E0CD847B9Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_new_selected_mission_creator_blip_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C90988E7C8E1AF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C90988E7C8E1AF4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_new_selected_mission_creator_blip_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C90988E7C8E1AF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C90988E7C8E1AF4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
It sets the text in a specified box and wraps the text if it exceeds the boundries. Both values are for X axis. Useful when positioning text set to center or aligned to the right.  
start - left boundry on screen position (0.0 - 1.0)  
end - right boundry on screen position (0.0 - 1.0)  
```



pub fn set_text_wrap_safe(
        
        
            start: 
        , 
        
        
            end: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63145D9C883A1A70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63145D9C883A1A70u64;
        
        let result = invoke_raw!(
            hash,
                start, 
                end
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_text_wrap_raw(
        start: , 
        end: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63145D9C883A1A70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63145D9C883A1A70u64;

        invoke_raw_typed!(
            hash,
                start, 
                end
        )
    }
}

/// ```
get's line count  
int GetLineCount(char *text, float x, float y)  
	{  
_BEGIN_TEXT_COMMAND_LINE_COUNT("STRING");  
                ADD_TEXT_COMPONENT_SUBSTRING_PLAYER_NAME(text);  
return _END_TEXT_COMMAND_GET_LINE_COUNT(x, y);  
	}  
```



pub fn _begin_text_command_line_count_safe(
        
        
            entry: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x521FB041D93DD0E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x521FB041D93DD0E4u64;
        
        let result = invoke_raw!(
            hash,
                entry
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _begin_text_command_line_count_raw(
        entry: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x521FB041D93DD0E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x521FB041D93DD0E4u64;

        invoke_raw_typed!(
            hash,
                entry
        )
    }
}

/// ## Parameters
*



pub fn _0xca6b2f7ce32ab653_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA6B2F7CE32AB653u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA6B2F7CE32AB653u64;
        
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
pub fn _0xca6b2f7ce32ab653_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA6B2F7CE32AB653u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA6B2F7CE32AB653u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0xeb81a3dadd503187_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB81A3DADD503187u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB81A3DADD503187u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xeb81a3dadd503187_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB81A3DADD503187u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB81A3DADD503187u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn pause_menu_is_context_menu_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A25ADC48F87841Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A25ADC48F87841Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pause_menu_is_context_menu_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A25ADC48F87841Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A25ADC48F87841Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _get_ai_blip_2_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CD934010E115C2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CD934010E115C2Cu64;
        
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
pub fn _get_ai_blip_2_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CD934010E115C2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CD934010E115C2Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// This native is used to colorize/toggle certain map components like the army base.

Component IDs 6 through 14 are used by the freemode event King of the Castle in GTA Online.



pub fn set_minimap_component_safe(
        
        
            componentID: 
        , 
        
        
            toggle: 
        , 
        
        
            hudColor: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75A9A10948D1DEA6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75A9A10948D1DEA6u64;
        
        let result = invoke_raw!(
            hash,
                componentID, 
                toggle, 
                hudColor
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_minimap_component_raw(
        componentID: , 
        toggle: , 
        hudColor: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75A9A10948D1DEA6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75A9A10948D1DEA6u64;

        invoke_raw_typed!(
            hash,
                componentID, 
                toggle, 
                hudColor
        )
    }
}

/// ## Parameters
*



pub fn set_blip_scale_safe(
        
        
            blip: 
        , 
        
        
            scale: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD38744167B2FA257u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD38744167B2FA257u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                scale
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_scale_raw(
        blip: , 
        scale: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD38744167B2FA257u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD38744167B2FA257u64;

        invoke_raw_typed!(
            hash,
                blip, 
                scale
        )
    }
}

/// Preview image:

![](https://i.imgur.com/1BTmdyv.png)

To change the bank balance use [`STAT_SET_INT`](#_0xB3271D7AB655B441) with "BANK_BALANCE" to whatever value you need to.



pub fn set_multiplayer_bank_cash_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD21B55DF695CD0Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD21B55DF695CD0Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_multiplayer_bank_cash_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD21B55DF695CD0Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD21B55DF695CD0Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_radar_zoom_to_distance_safe(
        
        
            zoom: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB7CC0D58405AD41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB7CC0D58405AD41u64;
        
        let result = invoke_raw!(
            hash,
                zoom
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_radar_zoom_to_distance_raw(
        zoom: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB7CC0D58405AD41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB7CC0D58405AD41u64;

        invoke_raw_typed!(
            hash,
                zoom
        )
    }
}

/// ## Parameters
*



pub fn set_gps_multi_route_render_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DDA37128DD1ACA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DDA37128DD1ACA8u64;
        
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
pub fn set_gps_multi_route_render_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DDA37128DD1ACA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DDA37128DD1ACA8u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _end_text_command_objective_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFDBDF5AE59BA0F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFDBDF5AE59BA0F4u64;
        
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
pub fn _end_text_command_objective_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFDBDF5AE59BA0F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFDBDF5AE59BA0F4u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```c
enum eBusySpinnerType
{
	BUSY_SPINNER_LEFT = 0,
	BUSY_SPINNER_LEFT_2 = 1,
	BUSY_SPINNER_LEFT_3 = 2,
	BUSY_SPINNER_SAVE = 3,
	BUSY_SPINNER_RIGHT = 4,
};
```



pub fn end_text_command_busyspinner_on_safe(
        
        
            busySpinnerType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD12F8228410D9B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD12F8228410D9B4u64;
        
        let result = invoke_raw!(
            hash,
                busySpinnerType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_busyspinner_on_raw(
        busySpinnerType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD12F8228410D9B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD12F8228410D9B4u64;

        invoke_raw_typed!(
            hash,
                busySpinnerType
        )
    }
}

/// ## Parameters
*



pub fn get_blip_hud_colour_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x729B5F1EFBC0AAEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x729B5F1EFBC0AAEEu64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_blip_hud_colour_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x729B5F1EFBC0AAEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x729B5F1EFBC0AAEEu64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// Set a custom color for the MP HUD, providing a way to customize the HUD's appearance similar to character color changes in MP.

```c
enum eHudColours {
    HUD_COLOUR_PURE_WHITE = 0,
    HUD_COLOUR_WHITE = 1,
    HUD_COLOUR_BLACK = 2,
    HUD_COLOUR_GREY = 3,
    HUD_COLOUR_GREYLIGHT = 4,
    HUD_COLOUR_GREYDARK = 5,
    HUD_COLOUR_RED = 6,
    HUD_COLOUR_REDLIGHT = 7,
    HUD_COLOUR_REDDARK = 8,
    HUD_COLOUR_BLUE = 9,
    HUD_COLOUR_BLUELIGHT = 10,
    HUD_COLOUR_BLUEDARK = 11,
    HUD_COLOUR_YELLOW = 12,
    HUD_COLOUR_YELLOWLIGHT = 13,
    HUD_COLOUR_YELLOWDARK = 14,
    HUD_COLOUR_ORANGE = 15,
    HUD_COLOUR_ORANGELIGHT = 16,
    HUD_COLOUR_ORANGEDARK = 17,
    HUD_COLOUR_GREEN = 18,
    HUD_COLOUR_GREENLIGHT = 19,
    HUD_COLOUR_GREENDARK = 20,
    HUD_COLOUR_PURPLE = 21,
    HUD_COLOUR_PURPLELIGHT = 22,
    HUD_COLOUR_PURPLEDARK = 23,
    HUD_COLOUR_PINK = 24,
    HUD_COLOUR_STAR = 25,
    HUD_COLOUR_STARLIGHT = 26,
    HUD_COLOUR_STARDARK = 27,
    HUD_COLOUR_NET_PLAYER1 = 28,
    HUD_COLOUR_NET_PLAYER2 = 29,
    HUD_COLOUR_NET_PLAYER3 = 30,
    HUD_COLOUR_NET_PLAYER4 = 31,
    HUD_COLOUR_NET_PLAYER5 = 32,
    HUD_COLOUR_NET_PLAYER6 = 33,
    HUD_COLOUR_NET_PLAYER7 = 34,
    HUD_COLOUR_NET_PLAYER8 = 35,
    HUD_COLOUR_NET_PLAYER9 = 36,
    HUD_COLOUR_NET_PLAYER10 = 37,
    HUD_COLOUR_NET_PLAYER11 = 38,
    HUD_COLOUR_NET_PLAYER12 = 39,
    HUD_COLOUR_NET_PLAYER13 = 40,
    HUD_COLOUR_NET_PLAYER14 = 41,
    HUD_COLOUR_NET_PLAYER15 = 42,
    HUD_COLOUR_NET_PLAYER16 = 43,
    HUD_COLOUR_NET_PLAYER17 = 44,
    HUD_COLOUR_NET_PLAYER18 = 45,
    HUD_COLOUR_NET_PLAYER19 = 46,
    HUD_COLOUR_NET_PLAYER20 = 47,
    HUD_COLOUR_NET_PLAYER21 = 48,
    HUD_COLOUR_NET_PLAYER22 = 49,
    HUD_COLOUR_NET_PLAYER23 = 50,
    HUD_COLOUR_NET_PLAYER24 = 51,
    HUD_COLOUR_NET_PLAYER25 = 52,
    HUD_COLOUR_NET_PLAYER26 = 53,
    HUD_COLOUR_NET_PLAYER27 = 54,
    HUD_COLOUR_NET_PLAYER28 = 55,
    HUD_COLOUR_NET_PLAYER29 = 56,
    HUD_COLOUR_NET_PLAYER30 = 57,
    HUD_COLOUR_NET_PLAYER31 = 58,
    HUD_COLOUR_NET_PLAYER32 = 59,
    HUD_COLOUR_SIMPLEBLIP_DEFAULT = 60,
    HUD_COLOUR_MENU_BLUE = 61,
    HUD_COLOUR_MENU_GREY_LIGHT = 62,
    HUD_COLOUR_MENU_BLUE_EXTRA_DARK = 63,
    HUD_COLOUR_MENU_YELLOW = 64,
    HUD_COLOUR_MENU_YELLOW_DARK = 65,
    HUD_COLOUR_MENU_GREEN = 66,
    HUD_COLOUR_MENU_GREY = 67,
    HUD_COLOUR_MENU_GREY_DARK = 68,
    HUD_COLOUR_MENU_HIGHLIGHT = 69,
    HUD_COLOUR_MENU_STANDARD = 70,
    HUD_COLOUR_MENU_DIMMED = 71,
    HUD_COLOUR_MENU_EXTRA_DIMMED = 72,
    HUD_COLOUR_BRIEF_TITLE = 73,
    HUD_COLOUR_MID_GREY_MP = 74,
    HUD_COLOUR_NET_PLAYER1_DARK = 75,
    HUD_COLOUR_NET_PLAYER2_DARK = 76,
    HUD_COLOUR_NET_PLAYER3_DARK = 77,
    HUD_COLOUR_NET_PLAYER4_DARK = 78,
    HUD_COLOUR_NET_PLAYER5_DARK = 79,
    HUD_COLOUR_NET_PLAYER6_DARK = 80,
    HUD_COLOUR_NET_PLAYER7_DARK = 81,
    HUD_COLOUR_NET_PLAYER8_DARK = 82,
    HUD_COLOUR_NET_PLAYER9_DARK = 83,
    HUD_COLOUR_NET_PLAYER10_DARK = 84,
    HUD_COLOUR_NET_PLAYER11_DARK = 85,
    HUD_COLOUR_NET_PLAYER12_DARK = 86,
    HUD_COLOUR_NET_PLAYER13_DARK = 87,
    HUD_COLOUR_NET_PLAYER14_DARK = 88,
    HUD_COLOUR_NET_PLAYER15_DARK = 89,
    HUD_COLOUR_NET_PLAYER16_DARK = 90,
    HUD_COLOUR_NET_PLAYER17_DARK = 91,
    HUD_COLOUR_NET_PLAYER18_DARK = 92,
    HUD_COLOUR_NET_PLAYER19_DARK = 93,
    HUD_COLOUR_NET_PLAYER20_DARK = 94,
    HUD_COLOUR_NET_PLAYER21_DARK = 95,
    HUD_COLOUR_NET_PLAYER22_DARK = 96,
    HUD_COLOUR_NET_PLAYER23_DARK = 97,
    HUD_COLOUR_NET_PLAYER24_DARK = 98,
    HUD_COLOUR_NET_PLAYER25_DARK = 99,
    HUD_COLOUR_NET_PLAYER26_DARK = 100,
    HUD_COLOUR_NET_PLAYER27_DARK = 101,
    HUD_COLOUR_NET_PLAYER28_DARK = 102,
    HUD_COLOUR_NET_PLAYER29_DARK = 103,
    HUD_COLOUR_NET_PLAYER30_DARK = 104,
    HUD_COLOUR_NET_PLAYER31_DARK = 105,
    HUD_COLOUR_NET_PLAYER32_DARK = 106,
    HUD_COLOUR_BRONZE = 107,
    HUD_COLOUR_SILVER = 108,
    HUD_COLOUR_GOLD = 109,
    HUD_COLOUR_PLATINUM = 110,
    HUD_COLOUR_GANG1 = 111,
    HUD_COLOUR_GANG2 = 112,
    HUD_COLOUR_GANG3 = 113,
    HUD_COLOUR_GANG4 = 114,
    HUD_COLOUR_SAME_CREW = 115,
    HUD_COLOUR_FREEMODE = 116,
    HUD_COLOUR_PAUSE_BG = 117,
    HUD_COLOUR_FRIENDLY = 118,
    HUD_COLOUR_ENEMY = 119,
    HUD_COLOUR_LOCATION = 120,
    HUD_COLOUR_PICKUP = 121,
    HUD_COLOUR_PAUSE_SINGLEPLAYER = 122,
    HUD_COLOUR_FREEMODE_DARK = 123,
    HUD_COLOUR_INACTIVE_MISSION = 124,
    HUD_COLOUR_DAMAGE = 125,
    HUD_COLOUR_PINKLIGHT = 126,
    HUD_COLOUR_PM_MITEM_HIGHLIGHT = 127,
    HUD_COLOUR_SCRIPT_VARIABLE = 128,
    HUD_COLOUR_YOGA = 129,
    HUD_COLOUR_TENNIS = 130,
    HUD_COLOUR_GOLF = 131,
    HUD_COLOUR_SHOOTING_RANGE = 132,
    HUD_COLOUR_FLIGHT_SCHOOL = 133,
    HUD_COLOUR_NORTH_BLUE = 134,
    HUD_COLOUR_SOCIAL_CLUB = 135,
    HUD_COLOUR_PLATFORM_BLUE = 136,
    HUD_COLOUR_PLATFORM_GREEN = 137,
    HUD_COLOUR_PLATFORM_GREY = 138,
    HUD_COLOUR_FACEBOOK_BLUE = 139,
    HUD_COLOUR_INGAME_BG = 140,
    HUD_COLOUR_DARTS = 141,
    HUD_COLOUR_WAYPOINT = 142,
    HUD_COLOUR_MICHAEL = 143,
    HUD_COLOUR_FRANKLIN = 144,
    HUD_COLOUR_TREVOR = 145,
    HUD_COLOUR_GOLF_P1 = 146,
    HUD_COLOUR_GOLF_P2 = 147,
    HUD_COLOUR_GOLF_P3 = 148,
    HUD_COLOUR_GOLF_P4 = 149,
    HUD_COLOUR_WAYPOINTLIGHT = 150,
    HUD_COLOUR_WAYPOINTDARK = 151,
    HUD_COLOUR_PANEL_LIGHT = 152,
    HUD_COLOUR_MICHAEL_DARK = 153,
    HUD_COLOUR_FRANKLIN_DARK = 154,
    HUD_COLOUR_TREVOR_DARK = 155,
    HUD_COLOUR_OBJECTIVE_ROUTE = 156,
    HUD_COLOUR_PAUSEMAP_TINT = 157,
    HUD_COLOUR_PAUSE_DESELECT = 158,
    HUD_COLOUR_PM_WEAPONS_PURCHASABLE = 159,
    HUD_COLOUR_PM_WEAPONS_LOCKED = 160,
    HUD_COLOUR_END_SCREEN_BG = 161,
    HUD_COLOUR_CHOP = 162,
    HUD_COLOUR_PAUSEMAP_TINT_HALF = 163,
    HUD_COLOUR_NORTH_BLUE_OFFICIAL = 164,
    HUD_COLOUR_SCRIPT_VARIABLE_2 = 165,
    HUD_COLOUR_H = 166,
    HUD_COLOUR_HDARK = 167,
    HUD_COLOUR_T = 168,
    HUD_COLOUR_TDARK = 169,
    HUD_COLOUR_HSHARD = 170,
    HUD_COLOUR_CONTROLLER_MICHAEL = 171 
    HUD_COLOUR_CONTROLLER_FRANKLIN = 172 
    HUD_COLOUR_CONTROLLER_TREVOR = 173 
    HUD_COLOUR_CONTROLLER_CHOP = 174,
    HUD_COLOUR_VIDEO_EDITOR_VIDEO = 175,
    HUD_COLOUR_VIDEO_EDITOR_AUDIO = 176,
    HUD_COLOUR_VIDEO_EDITOR_TEXT = 177,
    HUD_COLOUR_HB_BLUE = 178,
    HUD_COLOUR_HB_YELLOW = 179,
    HUD_COLOUR_VIDEO_EDITOR_SCORE = 180,
    HUD_COLOUR_VIDEO_EDITOR_AUDIO_FADEOUT = 181,
    HUD_COLOUR_VIDEO_EDITOR_TEXT_FADEOUT = 182,
    HUD_COLOUR_VIDEO_EDITOR_SCORE_FADEOUT = 183,
    HUD_COLOUR_HEIST_BACKGROUND = 184,
    HUD_COLOUR_VIDEO_EDITOR_AMBIENT = 185,
    HUD_COLOUR_VIDEO_EDITOR_AMBIENT_FADEOUT = 186,
    HUD_COLOUR_GANG_BOSS = 187,
    HUD_COLOUR_GOON = 188,
    HUD_COLOUR_BOSS = 189,
    HUD_COLOUR_LOW_FLOW = 190,
    HUD_COLOUR_LOW_FLOW_DARK = 191,
    HUD_COLOUR_G1 = 192,
    HUD_COLOUR_G2 = 193,
    HUD_COLOUR_G3 = 194,
    HUD_COLOUR_G4 = 195,
    HUD_COLOUR_G5 = 196,
    HUD_COLOUR_G6 = 197,
    HUD_COLOUR_G7 = 198,
    HUD_COLOUR_G8 = 199,
    HUD_COLOUR_G9 = 200,
    HUD_COLOUR_G10 = 201,
    HUD_COLOUR_G11 = 202,
    HUD_COLOUR_G12 = 203,
    HUD_COLOUR_G13 = 204,
    HUD_COLOUR_G14 = 205,
    HUD_COLOUR_G15 = 206,
    HUD_COLOUR_ADVERSARY = 207,
    HUD_COLOUR_DEGEN_RED = 208,
    HUD_COLOUR_DEGEN_YELLOW = 209,
    HUD_COLOUR_DEGEN_GREEN = 210,
    HUD_COLOUR_DEGEN_CYAN = 211,
    HUD_COLOUR_DEGEN_BLUE = 212,
    HUD_COLOUR_DEGEN_MAGENTA = 213,
    HUD_COLOUR_STUNT_1 = 214,
    HUD_COLOUR_STUNT_2 = 215,
    HUD_COLOUR_SPECIAL_RACE_SERIES = 216 
    HUD_COLOUR_SPECIAL_RACE_SERIES_DARK = 217,
    HUD_COLOUR_CS = 218 
    HUD_COLOUR_CS_DARK = 219,
    HUD_COLOUR_TECH_GREEN = 220,
    HUD_COLOUR_TECH_GREEN_DARK = 221,
    HUD_COLOUR_TECH_RED = 222,
    HUD_COLOUR_TECH_GREEN_VERY_DARK = 223,
    HUD_COLOUR_PLACEHOLDER_01 = 224,
    HUD_COLOUR_PLACEHOLDER_02 = 225,
    HUD_COLOUR_PLACEHOLDER_03 = 226,
    HUD_COLOUR_PLACEHOLDER_04 = 227,
    HUD_COLOUR_PLACEHOLDER_05 = 228,
    HUD_COLOUR_PLACEHOLDER_06 = 229,
    HUD_COLOUR_PLACEHOLDER_07 = 230,
    HUD_COLOUR_PLACEHOLDER_08 = 231,
    HUD_COLOUR_PLACEHOLDER_09 = 232,
    HUD_COLOUR_PLACEHOLDER_10 = 233,
    HUD_COLOUR_JUNK_ENERGY = 234
};
```

```
NativeDB Introduced: 2545
```



pub fn set_custom_mp_hud_color_safe(
        
        
            hudColorId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2ACCB195F3CCD9DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2ACCB195F3CCD9DEu64;
        
        let result = invoke_raw!(
            hash,
                hudColorId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_custom_mp_hud_color_raw(
        hudColorId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2ACCB195F3CCD9DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2ACCB195F3CCD9DEu64;

        invoke_raw_typed!(
            hash,
                hudColorId
        )
    }
}

/// ```
This native (along with 0x5F68520888E69014 and 0x6C188BE134E074AA) do not actually filter anything. They simply add the provided text (as of 944)  
```



pub fn add_text_component_substring_website_safe(
        
        
            website: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94CF4AC034C9C986u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94CF4AC034C9C986u64;
        
        let result = invoke_raw!(
            hash,
                website
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_text_component_substring_website_raw(
        website: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94CF4AC034C9C986u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94CF4AC034C9C986u64;

        invoke_raw_typed!(
            hash,
                website
        )
    }
}

/// CLEAR_GPS_CUSTOM_ROUTE native function



pub fn clear_gps_custom_route_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6DE0561D9232A64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6DE0561D9232A64u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_gps_custom_route_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6DE0561D9232A64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6DE0561D9232A64u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// If mouse is hovering on a slot, it returns the slot's index, else it returns -1.



pub fn _pause_menu_get_index_of_mouse_hovered_slot_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x359AF31A4B52F5EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x359AF31A4B52F5EDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _pause_menu_get_index_of_mouse_hovered_slot_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x359AF31A4B52F5EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x359AF31A4B52F5EDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Converts the hash of a street name into a readable string. To retrieve a hash for a given (street) coordinate, see [`GET_STREET_NAME_AT_COORD`](#_0x2EB41072B4C1E4C0).



pub fn get_street_name_from_hash_key_safe(
        
        
            hash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0EF8A959B8A4CB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0EF8A959B8A4CB9u64;
        
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
pub fn get_street_name_from_hash_key_raw(
        hash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0EF8A959B8A4CB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0EF8A959B8A4CB9u64;

        invoke_raw_typed!(
            hash,
                hash
        )
    }
}

/// ```
p0: found arguments in the b617d scripts: pastebin.com/X5akCN7z  
```



pub fn clear_this_print_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF708001E1E536DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF708001E1E536DDu64;
        
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
pub fn clear_this_print_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF708001E1E536DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF708001E1E536DDu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
nothin doin.   
BOOL Message(char* text)  
	{  
BEGIN_TEXT_COMMAND_IS_MESSAGE_DISPLAYED("STRING");  
ADD_TEXT_COMPONENT_SUBSTRING_PLAYER_NAME(text);  
return END_TEXT_COMMAND_IS_MESSAGE_DISPLAYED();  
	}  
```



pub fn begin_text_command_is_message_displayed_safe(
        
        
            text: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x853648FD1063A213u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x853648FD1063A213u64;
        
        let result = invoke_raw!(
            hash,
                text
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_text_command_is_message_displayed_raw(
        text: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x853648FD1063A213u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x853648FD1063A213u64;

        invoke_raw_typed!(
            hash,
                text
        )
    }
}

/// ## Parameters
*



pub fn _set_help_message_text_style_safe(
        
        
            style: 
        , 
        
        
            hudColor: 
        , 
        
        
            alpha: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9C362BABECDDC7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9C362BABECDDC7Au64;
        
        let result = invoke_raw!(
            hash,
                style, 
                hudColor, 
                alpha, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_help_message_text_style_raw(
        style: , 
        hudColor: , 
        alpha: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9C362BABECDDC7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9C362BABECDDC7Au64;

        invoke_raw_typed!(
            hash,
                style, 
                hudColor, 
                alpha, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn pulse_blip_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x742D6FD43115AF73u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x742D6FD43115AF73u64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pulse_blip_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x742D6FD43115AF73u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x742D6FD43115AF73u64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// PRELOAD_BUSYSPINNER native function



pub fn preload_busyspinner_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC65AB383CD91DF98u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC65AB383CD91DF98u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn preload_busyspinner_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC65AB383CD91DF98u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC65AB383CD91DF98u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_menu_ped_int_stat_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF4CED81CEBEDC6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF4CED81CEBEDC6Du64;
        
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
pub fn get_menu_ped_int_stat_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF4CED81CEBEDC6Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF4CED81CEBEDC6Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// See https://imgur.com/a/lLkEsMN

```
NativeDB Introduced: v1734
```



pub fn _set_blip_scale_transformation_safe(
        
        
            blip: 
        , 
        
        
            xScale: 
        , 
        
        
            yScale: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD6524439909C979u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD6524439909C979u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                xScale, 
                yScale
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_blip_scale_transformation_raw(
        blip: , 
        xScale: , 
        yScale: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD6524439909C979u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD6524439909C979u64;

        invoke_raw_typed!(
            hash,
                blip, 
                xScale, 
                yScale
        )
    }
}

/// ## Parameters
*



pub fn hide_scripted_hud_component_this_frame_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE374C498D8BADC14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE374C498D8BADC14u64;
        
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
pub fn hide_scripted_hud_component_this_frame_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE374C498D8BADC14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE374C498D8BADC14u64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ## Parameters
*



pub fn _begin_text_command_objective_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23D69E0465570028u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23D69E0465570028u64;
        
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
pub fn _begin_text_command_objective_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23D69E0465570028u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23D69E0465570028u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn request_additional_text_for_dlc_safe(
        
        
            gxt: 
        , 
        
        
            slot: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6009F9F1AE90D8A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6009F9F1AE90D8A6u64;
        
        let result = invoke_raw!(
            hash,
                gxt, 
                slot
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_additional_text_for_dlc_raw(
        gxt: , 
        slot: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6009F9F1AE90D8A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6009F9F1AE90D8A6u64;

        invoke_raw_typed!(
            hash,
                gxt, 
                slot
        )
    }
}

/// ## Parameters
*



pub fn set_gps_flashes_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x320D0E0D936A0E9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x320D0E0D936A0E9Bu64;
        
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
pub fn set_gps_flashes_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x320D0E0D936A0E9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x320D0E0D936A0E9Bu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
lastItemMenuId: this is the menuID of the last selected item minus 1000 (lastItem.menuID - 1000)
selectedItemMenuId: same as lastItemMenuId except for the currently selected menu item
selectedItemUniqueId: this is uniqueID of the currently selected menu item
when the pausemenu is closed:
lastItemMenuId = -1
selectedItemMenuId = -1
selectedItemUniqueId = 0
when the header gains focus:
lastItemMenuId updates as normal or 0 if the pausemenu was just opened
selectedItemMenuId becomes a unique id for the pausemenu page that focus was taken from (?) or 0 if the pausemenu was just opened
selectedItemUniqueId = -1
when focus is moved from the header to a pausemenu page:
lastItemMenuId becomes a unique id for the pausemenu page that focus was moved to (?)
selectedItemMenuId = -1
selectedItemUniqueId updates as normal
```



pub fn _get_pause_menu_selection_data_safe(
        
        
            lastItemMenuId: 
        , 
        
        
            selectedItemMenuId: 
        , 
        
        
            selectedItemUniqueId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E17BE53E1AAABAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E17BE53E1AAABAFu64;
        
        let result = invoke_raw!(
            hash,
                lastItemMenuId, 
                selectedItemMenuId, 
                selectedItemUniqueId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_pause_menu_selection_data_raw(
        lastItemMenuId: , 
        selectedItemMenuId: , 
        selectedItemUniqueId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E17BE53E1AAABAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E17BE53E1AAABAFu64;

        invoke_raw_typed!(
            hash,
                lastItemMenuId, 
                selectedItemMenuId, 
                selectedItemUniqueId
        )
    }
}

/// ## Parameters
*



pub fn is_mp_gamer_tag_free_safe(
        
        
            gamerTagId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x595B5178E412E199u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x595B5178E412E199u64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_mp_gamer_tag_free_raw(
        gamerTagId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x595B5178E412E199u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x595B5178E412E199u64;

        invoke_raw_typed!(
            hash,
                gamerTagId
        )
    }
}

/// ```
Adds a green checkmark on top of a blip.  
```



pub fn show_tick_on_blip_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74513EA3E505181Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74513EA3E505181Eu64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn show_tick_on_blip_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74513EA3E505181Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74513EA3E505181Eu64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_menu_ped_float_stat_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FBD7095FE7AE57Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FBD7095FE7AE57Fu64;
        
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
pub fn get_menu_ped_float_stat_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FBD7095FE7AE57Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FBD7095FE7AE57Fu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
If toggle is true, the map is shown in full screen
If toggle is false, the map is shown in normal mode
```



pub fn _race_gallery_fullscreen_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5354C5BA2EA868A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5354C5BA2EA868A4u64;
        
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
pub fn _race_gallery_fullscreen_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5354C5BA2EA868A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5354C5BA2EA868A4u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_minimap_fow_reveal_coordinate_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0923DBF87DFF735Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0923DBF87DFF735Eu64;
        
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
pub fn set_minimap_fow_reveal_coordinate_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0923DBF87DFF735Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0923DBF87DFF735Eu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// If true is passed, the player won't be able to open the multiplayer chat



pub fn _disable_multiplayer_chat_safe(
        
        
            disable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DB21A44B09E8BA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DB21A44B09E8BA3u64;
        
        let result = invoke_raw!(
            hash,
                disable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _disable_multiplayer_chat_raw(
        disable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DB21A44B09E8BA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DB21A44B09E8BA3u64;

        invoke_raw_typed!(
            hash,
                disable
        )
    }
}

/// Activates the specified frontend menu context.

pausemenu.xml defines some specific menu options using 'context'. Context is basically a 'condition'. 

The `*ALL*` part of the context means that whatever is being defined, will be active when any or all of those conditions after `*ALL*` are met.

The `*NONE*` part of the context section means that whatever is being defined, will NOT be active if any or all of the conditions after `*NONE*` are met.

This basically allows you to hide certain menu sections, or things like instructional buttons.

See the old description below for more info.



pub fn pause_menu_activate_context_safe(
        
        
            hash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD564BDD0472C936u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD564BDD0472C936u64;
        
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
pub fn pause_menu_activate_context_raw(
        hash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD564BDD0472C936u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD564BDD0472C936u64;

        invoke_raw_typed!(
            hash,
                hash
        )
    }
}

/// Updates instructional buttons in Pause Menu after menu contexts have been toggled. p0 purpose is currently unknown, only 0 is used in scripts.



pub fn pause_menu_redraw_instructional_buttons_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4895BDEA16E7C080u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4895BDEA16E7C080u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pause_menu_redraw_instructional_buttons_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4895BDEA16E7C080u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4895BDEA16E7C080u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_hud_component_position_safe(
        
        
            id: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAABB1F56E2A17CEDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAABB1F56E2A17CEDu64;
        
        let result = invoke_raw!(
            hash,
                id, 
                x, 
                y
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_hud_component_position_raw(
        id: , 
        x: , 
        y: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAABB1F56E2A17CEDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAABB1F56E2A17CEDu64;

        invoke_raw_typed!(
            hash,
                id, 
                x, 
                y
        )
    }
}

/// ## Parameters
*



pub fn end_text_command_thefeed_post_ticker_with_tokens_safe(
        
        
            isImportant: 
        , 
        
        
            bHasTokens: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x378E809BF61EC840u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x378E809BF61EC840u64;
        
        let result = invoke_raw!(
            hash,
                isImportant, 
                bHasTokens
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_ticker_with_tokens_raw(
        isImportant: , 
        bHasTokens: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x378E809BF61EC840u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x378E809BF61EC840u64;

        invoke_raw_typed!(
            hash,
                isImportant, 
                bHasTokens
        )
    }
}

/// ## Parameters
*



pub fn set_radar_zoom_precise_safe(
        
        
            zoom: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD12C5EEE184C337u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD12C5EEE184C337u64;
        
        let result = invoke_raw!(
            hash,
                zoom
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_radar_zoom_precise_raw(
        zoom: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD12C5EEE184C337u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD12C5EEE184C337u64;

        invoke_raw_typed!(
            hash,
                zoom
        )
    }
}

/// ```
Argument must be 0.0f or above 38.0f, or it will be ignored.  
```

```
NativeDB Added Parameter 3: Any p2
```



pub fn _set_minimap_altitude_indicator_level_safe(
        
        
            altitude: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD201F3FF917A506Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD201F3FF917A506Du64;
        
        let result = invoke_raw!(
            hash,
                altitude, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_minimap_altitude_indicator_level_raw(
        altitude: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD201F3FF917A506Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD201F3FF917A506Du64;

        invoke_raw_typed!(
            hash,
                altitude, 
                p1
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn set_text_edge_safe(
        
        
            p0: 
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
        let hash = 0x441603240D202FA6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x441603240D202FA6u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
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
pub fn set_text_edge_raw(
        p0: , 
        r: , 
        g: , 
        b: , 
        a: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x441603240D202FA6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x441603240D202FA6u64;

        invoke_raw_typed!(
            hash,
                p0, 
                r, 
                g, 
                b, 
                a
        )
    }
}

/// SET_MINIMAP_GOLF_COURSE_OFF native function



pub fn set_minimap_golf_course_off_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35EDD5B2E3FF01C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35EDD5B2E3FF01C0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_minimap_golf_course_off_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35EDD5B2E3FF01C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35EDD5B2E3FF01C0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Creates a gamer tag for the specified local player ID, automatically attached to the player's current ped.
The created gamer tag will have the same ID as the player. You can use [IS_MP_GAMER_TAG_ACTIVE](#_0x4E929E7A5796FD26) to check if a gamer tag already exists for a player.
After the gamer tag is created, all components will be set as invisible. Use [SET_MP_GAMER_TAG_VISIBILITY](#_0x63BB75ABEDC1F6A0) to change the visibility of individual components or [_SET_MP_GAMER_TAG_VISIBILITY_ALL](#_0xEE76FF7E6A0166B0) to set all of them at once.

To create a gamer tag for a ped that is not a player, see [CREATE_FAKE_MP_GAMER_TAG](#_0xBFEFE3321A3F5015).



pub fn create_mp_gamer_tag_with_crew_color_safe(
        
        
            player: 
        , 
        
        
            username: 
        , 
        
        
            crewIsPrivate: 
        , 
        
        
            crewIsRockstar: 
        , 
        
        
            crewName: 
        , 
        
        
            crewRank: 
        , 
        
        
            crewR: 
        , 
        
        
            crewG: 
        , 
        
        
            crewB: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6DD05E9D83EFA4C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6DD05E9D83EFA4C9u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                username, 
                crewIsPrivate, 
                crewIsRockstar, 
                crewName, 
                crewRank, 
                crewR, 
                crewG, 
                crewB
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_mp_gamer_tag_with_crew_color_raw(
        player: , 
        username: , 
        crewIsPrivate: , 
        crewIsRockstar: , 
        crewName: , 
        crewRank: , 
        crewR: , 
        crewG: , 
        crewB: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6DD05E9D83EFA4C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6DD05E9D83EFA4C9u64;

        invoke_raw_typed!(
            hash,
                player, 
                username, 
                crewIsPrivate, 
                crewIsRockstar, 
                crewName, 
                crewRank, 
                crewR, 
                crewG, 
                crewB
        )
    }
}

/// ## Return value



pub fn is_subtitle_preference_switched_on_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD6DACA4BA53E0A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD6DACA4BA53E0A4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_subtitle_preference_switched_on_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD6DACA4BA53E0A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD6DACA4BA53E0A4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// SET_RADAR_AS_EXTERIOR_THIS_FRAME native function



pub fn set_radar_as_exterior_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE81B7D2A3DAB2D81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE81B7D2A3DAB2D81u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_radar_as_exterior_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE81B7D2A3DAB2D81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE81B7D2A3DAB2D81u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_blip_name_to_player_name_safe(
        
        
            blip: 
        , 
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x127DE7B20C60A6A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x127DE7B20C60A6A3u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_name_to_player_name_raw(
        blip: , 
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x127DE7B20C60A6A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x127DE7B20C60A6A3u64;

        invoke_raw_typed!(
            hash,
                blip, 
                player
        )
    }
}

/// ## Return value



pub fn is_minimap_rendering_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF754F20EB5CD51Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF754F20EB5CD51Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_minimap_rendering_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF754F20EB5CD51Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF754F20EB5CD51Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn is_streaming_additional_text_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B6817B71B85EBF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B6817B71B85EBF0u64;
        
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
pub fn is_streaming_additional_text_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B6817B71B85EBF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B6817B71B85EBF0u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn end_text_command_is_message_displayed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A9BA1AB3E237613u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A9BA1AB3E237613u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_is_message_displayed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A9BA1AB3E237613u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A9BA1AB3E237613u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xc8e1071177a23be5_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8E1071177A23BE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8E1071177A23BE5u64;
        
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
pub fn _0xc8e1071177a23be5_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8E1071177A23BE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8E1071177A23BE5u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
adds a short flash to the Radar/Minimap  
Usage: UI.FLASH_MINIMAP_DISPLAY  
```



pub fn flash_minimap_display_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2DD778C22B15BDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2DD778C22B15BDAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn flash_minimap_display_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2DD778C22B15BDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2DD778C22B15BDAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Forces the weapon wheel to show/hide.
```



pub fn hud_force_weapon_wheel_safe(
        
        
            show: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB354E5376BC81A7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB354E5376BC81A7u64;
        
        let result = invoke_raw!(
            hash,
                show
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hud_force_weapon_wheel_raw(
        show: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB354E5376BC81A7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB354E5376BC81A7u64;

        invoke_raw_typed!(
            hash,
                show
        )
    }
}

/// ## Parameters
*



pub fn get_blip_info_id_entity_index_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4BA4E2553AFEDC2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4BA4E2553AFEDC2Cu64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_blip_info_id_entity_index_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4BA4E2553AFEDC2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4BA4E2553AFEDC2Cu64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// ## Parameters
*



pub fn is_blip_flashing_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5E41FD83AD6CEF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5E41FD83AD6CEF0u64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_blip_flashing_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5E41FD83AD6CEF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5E41FD83AD6CEF0u64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// ## Parameters
*



pub fn is_mission_creator_blip_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26F49BF3381D933Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26F49BF3381D933Du64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_mission_creator_blip_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26F49BF3381D933Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26F49BF3381D933Du64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// Also see [`GET_NEXT_BLIP_INFO_ID`](#_0x14F96AA50D6FBEA7) for an example.



pub fn get_first_blip_info_id_safe(
        
        
            blipSprite: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1BEDE233E6CD2A1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1BEDE233E6CD2A1Fu64;
        
        let result = invoke_raw!(
            hash,
                blipSprite
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_first_blip_info_id_raw(
        blipSprite: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1BEDE233E6CD2A1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1BEDE233E6CD2A1Fu64;

        invoke_raw_typed!(
            hash,
                blipSprite
        )
    }
}

/// ## Parameters
*



pub fn add_text_component_substring_text_label_safe(
        
        
            labelName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC63CD5D2920ACBE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC63CD5D2920ACBE7u64;
        
        let result = invoke_raw!(
            hash,
                labelName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_text_component_substring_text_label_raw(
        labelName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC63CD5D2920ACBE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC63CD5D2920ACBE7u64;

        invoke_raw_typed!(
            hash,
                labelName
        )
    }
}

/// ## Parameters
*



pub fn add_text_component_formatted_integer_safe(
        
        
            value: 
        , 
        
        
            commaSeparated: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E4C749FF9DE9CC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E4C749FF9DE9CC4u64;
        
        let result = invoke_raw!(
            hash,
                value, 
                commaSeparated
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_text_component_formatted_integer_raw(
        value: , 
        commaSeparated: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E4C749FF9DE9CC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E4C749FF9DE9CC4u64;

        invoke_raw_typed!(
            hash,
                value, 
                commaSeparated
        )
    }
}

/// ## Parameters
*



pub fn end_text_command_thefeed_post_crew_rankup_safe(
        
        
            chTitle: 
        , 
        
        
            clanTxd: 
        , 
        
        
            clanTxn: 
        , 
        
        
            isImportant: 
        , 
        
        
            showSubtitle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EFCCF6EC66D85E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EFCCF6EC66D85E4u64;
        
        let result = invoke_raw!(
            hash,
                chTitle, 
                clanTxd, 
                clanTxn, 
                isImportant, 
                showSubtitle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_crew_rankup_raw(
        chTitle: , 
        clanTxd: , 
        clanTxn: , 
        isImportant: , 
        showSubtitle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EFCCF6EC66D85E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EFCCF6EC66D85E4u64;

        invoke_raw_typed!(
            hash,
                chTitle, 
                clanTxd, 
                clanTxn, 
                isImportant, 
                showSubtitle
        )
    }
}

/// ## Return value



pub fn is_navigating_menu_content_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E3CD0EF8A489541u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E3CD0EF8A489541u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_navigating_menu_content_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E3CD0EF8A489541u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E3CD0EF8A489541u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_ped_ai_blip_has_cone_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3EED80DFF7325CAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3EED80DFF7325CAAu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_ai_blip_has_cone_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3EED80DFF7325CAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3EED80DFF7325CAAu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _set_allow_ability_bar_in_multiplayer_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x889329C80FE5963Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x889329C80FE5963Cu64;
        
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
pub fn _set_allow_ability_bar_in_multiplayer_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x889329C80FE5963Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x889329C80FE5963Cu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x41350b4fc28e3941_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41350B4FC28E3941u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41350B4FC28E3941u64;
        
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
pub fn _0x41350b4fc28e3941_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41350B4FC28E3941u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41350B4FC28E3941u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_text_render_id_safe(
        
        
            renderId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F15302936E07111u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F15302936E07111u64;
        
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
pub fn set_text_render_id_raw(
        renderId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F15302936E07111u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F15302936E07111u64;

        invoke_raw_typed!(
            hash,
                renderId
        )
    }
}

/// ## Parameters
*



pub fn show_start_mission_instructional_button_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1A6C18B35BCADE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1A6C18B35BCADE6u64;
        
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
pub fn show_start_mission_instructional_button_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1A6C18B35BCADE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1A6C18B35BCADE6u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _thefeed_set_animpostfx_color_safe(
        
        
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
        let hash = 0x17430B918701C342u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17430B918701C342u64;
        
        let result = invoke_raw!(
            hash,
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
pub fn _thefeed_set_animpostfx_color_raw(
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17430B918701C342u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17430B918701C342u64;

        invoke_raw_typed!(
            hash,
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ## Parameters
*



pub fn get_hud_component_position_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x223CA69A8C4417FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x223CA69A8C4417FDu64;
        
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
pub fn get_hud_component_position_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x223CA69A8C4417FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x223CA69A8C4417FDu64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ## Return value



pub fn thefeed_is_paused_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9CBFD40B3FA3010u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9CBFD40B3FA3010u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_is_paused_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9CBFD40B3FA3010u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9CBFD40B3FA3010u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Forces the Pause Menu to back out of unique pages such as Awards, Unlocks, Key Bindings etc



pub fn pause_menuception_the_kick_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCDCA26E80FAECB8Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCDCA26E80FAECB8Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pause_menuception_the_kick_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCDCA26E80FAECB8Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCDCA26E80FAECB8Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
This native removes the current waypoint from the map.
Example:
C#:
Function.Call(Hash.SET_WAYPOINT_OFF);
C++:
HUD::SET_WAYPOINT_OFF();
```



pub fn set_waypoint_off_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7E4E2D361C2627Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7E4E2D361C2627Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_waypoint_off_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7E4E2D361C2627Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7E4E2D361C2627Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// GET_BLIP_*
Seems to always return 0 from what I can tell. I've tried a lot of different blip related natives and it always seems to return 0. Decompiled scripts always pass a blip handle as p0.



pub fn _0x2c173ae2bdb9385e_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C173AE2BDB9385Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C173AE2BDB9385Eu64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2c173ae2bdb9385e_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C173AE2BDB9385Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C173AE2BDB9385Eu64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// ```
Sets the sprite of the next BLIP_GALLERY blip, values used in the native scripts: 143 (ObjectiveBlue), 144 (ObjectiveGreen), 145 (ObjectiveRed), 146 (ObjectiveYellow).
```



pub fn _race_gallery_next_blip_sprite_safe(
        
        
            spriteId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1EAE6DD17B7A5EFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1EAE6DD17B7A5EFAu64;
        
        let result = invoke_raw!(
            hash,
                spriteId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _race_gallery_next_blip_sprite_raw(
        spriteId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1EAE6DD17B7A5EFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1EAE6DD17B7A5EFAu64;

        invoke_raw_typed!(
            hash,
                spriteId
        )
    }
}

/// ## Parameters
*



pub fn set_ped_ai_blip_gang_id_safe(
        
        
            ped: 
        , 
        
        
            gangId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE52B8E7F85D39A08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE52B8E7F85D39A08u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                gangId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_ai_blip_gang_id_raw(
        ped: , 
        gangId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE52B8E7F85D39A08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE52B8E7F85D39A08u64;

        invoke_raw_typed!(
            hash,
                ped, 
                gangId
        )
    }
}

/// Gets a localized string literal from a label name. This is used to get the filename of the audio conversation associated with the provided label name.



pub fn get_filename_for_audio_conversation_safe(
        
        
            labelName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B5280EBA9840C72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B5280EBA9840C72u64;
        
        let result = invoke_raw!(
            hash,
                labelName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_filename_for_audio_conversation_raw(
        labelName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B5280EBA9840C72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B5280EBA9840C72u64;

        invoke_raw_typed!(
            hash,
                labelName
        )
    }
}

/// ## Parameters
*



pub fn _set_ped_has_ai_blip_with_color_safe(
        
        
            ped: 
        , 
        
        
            hasCone: 
        , 
        
        
            color: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB13DCB4C6FAAD238u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB13DCB4C6FAAD238u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                hasCone, 
                color
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_ped_has_ai_blip_with_color_raw(
        ped: , 
        hasCone: , 
        color: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB13DCB4C6FAAD238u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB13DCB4C6FAAD238u64;

        invoke_raw_typed!(
            hash,
                ped, 
                hasCone, 
                color
        )
    }
}

/// ```
List of interior hashes: pastebin.com/1FUyXNqY  
Not for every interior zoom > 0 available.  
```



pub fn set_radar_as_interior_this_frame_safe(
        
        
            interior: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            heading: 
        , 
        
        
            zoom: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59E727A1C9D3E31Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59E727A1C9D3E31Au64;
        
        let result = invoke_raw!(
            hash,
                interior, 
                x, 
                y, 
                heading, 
                zoom
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_radar_as_interior_this_frame_raw(
        interior: , 
        x: , 
        y: , 
        heading: , 
        zoom: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59E727A1C9D3E31Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59E727A1C9D3E31Au64;

        invoke_raw_typed!(
            hash,
                interior, 
                x, 
                y, 
                heading, 
                zoom
        )
    }
}

/// By default, the player health value shown by a gamer tag's health bar is synchronised with the health of the ped it is attached to.
This native disables that behaviour, allowing [`_SET_MP_GAMER_TAG_OVERRIDE_PLAYER_HEALTH`](#_0x1563FE35E9928E67) to have an effect.



pub fn _set_mp_gamer_tag_disable_player_health_sync_safe(
        
        
            gamerTagId: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD29EC58C2F6B5014u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD29EC58C2F6B5014u64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_mp_gamer_tag_disable_player_health_sync_raw(
        gamerTagId: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD29EC58C2F6B5014u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD29EC58C2F6B5014u64;

        invoke_raw_typed!(
            hash,
                gamerTagId, 
                toggle
        )
    }
}

/// UNLOCK_MINIMAP_POSITION native function



pub fn unlock_minimap_position_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E93E06DB8EF1F30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E93E06DB8EF1F30u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn unlock_minimap_position_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E93E06DB8EF1F30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E93E06DB8EF1F30u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_race_track_render_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1EAC5F91BCBC5073u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1EAC5F91BCBC5073u64;
        
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
pub fn set_race_track_render_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1EAC5F91BCBC5073u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1EAC5F91BCBC5073u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// FORCE_CLOSE_TEXT_INPUT_BOX native function



pub fn force_close_text_input_box_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8817605C2BA76200u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8817605C2BA76200u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_close_text_input_box_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8817605C2BA76200u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8817605C2BA76200u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Used in the native scripts to reference "GET_PEDHEADSHOT_TXD_STRING" and "CHAR_DEFAULT".

NativeDB Introduced: v323
```



pub fn thefeed_update_item_texture_safe(
        
        
            txdString1: 
        , 
        
        
            txnString1: 
        , 
        
        
            txdString2: 
        , 
        
        
            txnString2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x317EBA71D7543F52u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x317EBA71D7543F52u64;
        
        let result = invoke_raw!(
            hash,
                txdString1, 
                txnString1, 
                txdString2, 
                txnString2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_update_item_texture_raw(
        txdString1: , 
        txnString1: , 
        txdString2: , 
        txnString2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x317EBA71D7543F52u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x317EBA71D7543F52u64;

        invoke_raw_typed!(
            hash,
                txdString1, 
                txnString1, 
                txdString2, 
                txnString2
        )
    }
}

/// ## Parameters
*



pub fn get_length_of_literal_string_in_bytes_safe(
        
        
            string: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43E4111189E54F0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43E4111189E54F0Eu64;
        
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
pub fn get_length_of_literal_string_in_bytes_raw(
        string: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43E4111189E54F0Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43E4111189E54F0Eu64;

        invoke_raw_typed!(
            hash,
                string
        )
    }
}

/// ```
Adds a orange checkmark on top of a given blip handle: https://imgur.com/a/aw5OTMF
_SHOW_FRIEND_INDICATOR_ON_BLIP* - _SHOW_HEADING_INDICATOR_ON_BLIP*
```

```
NativeDB Introduced: v2699
```



pub fn _show_has_completed_indicator_on_blip_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCAC2031EBF79B1A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCAC2031EBF79B1A8u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _show_has_completed_indicator_on_blip_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCAC2031EBF79B1A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCAC2031EBF79B1A8u64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// ```
If Minimap / Radar should be displayed.
```



pub fn display_radar_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0EBB943C300E693u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0EBB943C300E693u64;
        
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
pub fn display_radar_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0EBB943C300E693u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0EBB943C300E693u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Switches the display of the in-game minimap to the Cayo Perico map. This native needs to be called every frame to maintain the toggled state effectively.

```
NativeDB Introduced: v2189
```



pub fn set_use_island_map_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E1460624D194A38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E1460624D194A38u64;
        
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
pub fn set_use_island_map_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E1460624D194A38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E1460624D194A38u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
IS_WARNING_MESSAGE_*
```



pub fn _is_warning_message_active_2_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF42195A42C63BBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF42195A42C63BBAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_warning_message_active_2_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF42195A42C63BBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF42195A42C63BBAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// CLEAR_BRIEF native function



pub fn clear_brief_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D292F73ADBD9313u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D292F73ADBD9313u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_brief_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D292F73ADBD9313u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D292F73ADBD9313u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0xf13fe2a80c05c561_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF13FE2A80C05C561u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF13FE2A80C05C561u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf13fe2a80c05c561_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF13FE2A80C05C561u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF13FE2A80C05C561u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn show_number_on_blip_safe(
        
        
            blip: 
        , 
        
        
            number: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3C0B359DCB848B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3C0B359DCB848B6u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                number
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn show_number_on_blip_raw(
        blip: , 
        number: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3C0B359DCB848B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3C0B359DCB848B6u64;

        invoke_raw_typed!(
            hash,
                blip, 
                number
        )
    }
}

/// ## Parameters
*



pub fn hide_number_on_blip_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x532CFF637EF80148u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x532CFF637EF80148u64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hide_number_on_blip_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x532CFF637EF80148u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x532CFF637EF80148u64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// THEFEED_FLUSH_QUEUE native function



pub fn thefeed_flush_queue_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8FDB297A8D25FBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8FDB297A8D25FBAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_flush_queue_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA8FDB297A8D25FBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA8FDB297A8D25FBAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Sets flag's sprite transparency. 0-255.  
```



pub fn set_mp_gamer_tag_alpha_safe(
        
        
            gamerTagId: 
        , 
        
        
            component: 
        , 
        
        
            alpha: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD48FE545CD46F857u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD48FE545CD46F857u64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId, 
                component, 
                alpha
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mp_gamer_tag_alpha_raw(
        gamerTagId: , 
        component: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD48FE545CD46F857u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD48FE545CD46F857u64;

        invoke_raw_typed!(
            hash,
                gamerTagId, 
                component, 
                alpha
        )
    }
}

/// ```
Returns a substring of a specified length starting at a specified position. The result is guaranteed not to exceed the specified max length.
NOTE: The 'maxLength' parameter might actually be the size of the buffer that is returned. More research is needed. -CL69
Example:
// Condensed example of how Rockstar uses this function
strLen = HUD::GET_LENGTH_OF_LITERAL_STRING(MISC::GET_ONSCREEN_KEYBOARD_RESULT());
subStr = HUD::_GET_TEXT_SUBSTRING_SAFE(MISC::GET_ONSCREEN_KEYBOARD_RESULT(), 0, strLen, 63);
--
"fm_race_creator.ysc", line 85115:
// parameters modified for clarity
BOOL sub_8e5aa(char *text, int length) {
    for (i = 0; i <= (length - 2); i += 1) {
        if (!MISC::ARE_STRINGS_EQUAL(HUD::_GET_TEXT_SUBSTRING_SAFE(text, i, i + 1, 1), " ")) {
            return FALSE;
        }
    }
    return TRUE;
}
```



pub fn _get_text_substring_safe_safe(
        
        
            text: 
        , 
        
        
            position: 
        , 
        
        
            length: 
        , 
        
        
            maxLength: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2798643312205C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2798643312205C5u64;
        
        let result = invoke_raw!(
            hash,
                text, 
                position, 
                length, 
                maxLength
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_text_substring_safe_raw(
        text: , 
        position: , 
        length: , 
        maxLength: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2798643312205C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2798643312205C5u64;

        invoke_raw_typed!(
            hash,
                text, 
                position, 
                length, 
                maxLength
        )
    }
}

/// REFRESH_WAYPOINT native function



pub fn refresh_waypoint_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x81FA173F170560D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x81FA173F170560D1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn refresh_waypoint_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x81FA173F170560D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x81FA173F170560D1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Gets the sprite id of the specified blip. Blip sprite ids and images can be found [here](https://docs.fivem.net/docs/game-references/blips/).



pub fn get_blip_sprite_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1FC877464A04FC4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1FC877464A04FC4Fu64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_blip_sprite_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1FC877464A04FC4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1FC877464A04FC4Fu64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// ```
Used to be known as _SET_TEXT_COMPONENT_FORMAT  
```



pub fn begin_text_command_display_help_safe(
        
        
            inputType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8509B634FBE7DA11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8509B634FBE7DA11u64;
        
        let result = invoke_raw!(
            hash,
                inputType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_text_command_display_help_raw(
        inputType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8509B634FBE7DA11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8509B634FBE7DA11u64;

        invoke_raw_typed!(
            hash,
                inputType
        )
    }
}

/// Gets mouse event data from scaleforms with mouse support. Must be checked every frame.
Returns item index if using the COLOUR_SWITCHER_02 scaleform.
Selection types, found in MOUSE_EVENTS.as:
MOUSE_DRAG_OUT = 0;
MOUSE_DRAG_OVER = 1;
MOUSE_DOWN = 2;
MOUSE_MOVE = 3;
MOUSE_UP = 4;
MOUSE_PRESS = 5;
MOUSE_RELEASE = 6;
MOUSE_RELEASE_OUTSIDE = 7;
MOUSE_ROLL_OUT = 8;
MOUSE_ROLL_OVER = 9;
MOUSE_WHEEL_UP = 10;
MOUSE_WHEEL_DOWN = 11;
 
Scaleforms that this works with: 
- COLOUR_SWITCHER_02
- MP_RESULTS_PANEL
- MP_NEXT_JOB_SELECTION
- SC_LEADERBOARD
Probably works with other scaleforms, needs more research.
In order to use this Native you MUST have controls 239, 240, 237, 238 enabled!
This native, due to its erroneous redundancy of the returned boolean value, works differently in C#: shifting the parameters (where `received` becomes `selectionType` and so on making the fourth parameter unused and always 0).



pub fn get_mouse_event_safe(
        
        
            scaleformHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x632B2940C67F4EA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x632B2940C67F4EA9u64;
        
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
pub fn get_mouse_event_raw(
        scaleformHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x632B2940C67F4EA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x632B2940C67F4EA9u64;

        invoke_raw_typed!(
            hash,
                scaleformHandle
        )
    }
}

/// ```
Used to be known as _SET_TEXT_ENTRY_2  
void ShowSubtitle(char *text)  
{  
	BEGIN_TEXT_COMMAND_PRINT("STRING");  
	ADD_TEXT_COMPONENT_SUBSTRING_PLAYER_NAME(text);  
	END_TEXT_COMMAND_PRINT(2000, 1);  
}  
```



pub fn begin_text_command_print_safe(
        
        
            GxtEntry: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB87A37EEB7FAA67Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB87A37EEB7FAA67Du64;
        
        let result = invoke_raw!(
            hash,
                GxtEntry
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_text_command_print_raw(
        GxtEntry: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB87A37EEB7FAA67Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB87A37EEB7FAA67Du64;

        invoke_raw_typed!(
            hash,
                GxtEntry
        )
    }
}

/// Same as [`SET_WAYPOINT_OFF`](#_0xA7E4E2D361C2627F), except it checks if the local player is the owner of the waypoint.



pub fn delete_waypoints_from_this_player_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8E694757BCEA8E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8E694757BCEA8E9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn delete_waypoints_from_this_player_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8E694757BCEA8E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8E694757BCEA8E9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_help_message_fading_out_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x327EDEEEAC55C369u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x327EDEEEAC55C369u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_help_message_fading_out_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x327EDEEEAC55C369u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x327EDEEEAC55C369u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_main_player_blip_id_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCD4EC3F419D02FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCD4EC3F419D02FAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_main_player_blip_id_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCD4EC3F419D02FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCD4EC3F419D02FAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Requires GAME_STREAM_ENUMS.MSGTEXT. Default sounds: "DPAD_WEAPON_SCROLL" and "HUD_FRONTEND_DEFAULT_SOUNDSET"
```



pub fn _thefeed_set_animpostfx_sound_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A0C7C9BB10ABB36u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A0C7C9BB10ABB36u64;
        
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
pub fn _thefeed_set_animpostfx_sound_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A0C7C9BB10ABB36u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A0C7C9BB10ABB36u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Displays loading screen tips, requires `_0x56C8B608CFD49854` to be called beforehand.



pub fn _hud_display_loading_screen_tips_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x488043841BBE156Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x488043841BBE156Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _hud_display_loading_screen_tips_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x488043841BBE156Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x488043841BBE156Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x583049884A2EEE3C native function



pub fn _0x583049884a2eee3c_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x583049884A2EEE3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x583049884A2EEE3Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x583049884a2eee3c_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x583049884A2EEE3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x583049884A2EEE3Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn flash_ability_bar_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02CFBA0C9E9275CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02CFBA0C9E9275CEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn flash_ability_bar_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02CFBA0C9E9275CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02CFBA0C9E9275CEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns the handle for the notification currently displayed on the screen. Name may be a hash collision, but describes the function accurately.
```



pub fn thefeed_get_first_visible_delete_remaining_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82352748437638CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82352748437638CAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_get_first_visible_delete_remaining_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82352748437638CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82352748437638CAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// RESET_RETICULE_VALUES native function



pub fn reset_reticule_values_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12782CE0A636E9F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12782CE0A636E9F0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_reticule_values_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12782CE0A636E9F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12782CE0A636E9F0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _0x0c698d8f099174c7_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C698D8F099174C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C698D8F099174C7u64;
        
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
pub fn _0x0c698d8f099174c7_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C698D8F099174C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C698D8F099174C7u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Forces the map menu to reload.



pub fn reload_map_menu_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2916A928514C9827u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2916A928514C9827u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reload_map_menu_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2916A928514C9827u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2916A928514C9827u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// HIDE_MINIMAP_EXTERIOR_MAP_THIS_FRAME native function



pub fn hide_minimap_exterior_map_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FBAE526203990C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FBAE526203990C9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hide_minimap_exterior_map_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FBAE526203990C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FBAE526203990C9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn thefeed_only_show_tooltips_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F1554B0CC2089FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F1554B0CC2089FAu64;
        
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
pub fn thefeed_only_show_tooltips_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F1554B0CC2089FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F1554B0CC2089FAu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_blip_fade_safe(
        
        
            blip: 
        , 
        
        
            opacity: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2AEE8F8390D2298Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2AEE8F8390D2298Cu64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                opacity, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_fade_raw(
        blip: , 
        opacity: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2AEE8F8390D2298Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2AEE8F8390D2298Cu64;

        invoke_raw_typed!(
            hash,
                blip, 
                opacity, 
                duration
        )
    }
}

/// SET_C*

```
NativeDB Introduced: v1734
```



pub fn _0xb7b873520c84c118_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7B873520C84C118u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7B873520C84C118u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb7b873520c84c118_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7B873520C84C118u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7B873520C84C118u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _set_main_player_blip_colour_safe(
        
        
            color: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B21E0BB01E8224Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B21E0BB01E8224Au64;
        
        let result = invoke_raw!(
            hash,
                color
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_main_player_blip_colour_raw(
        color: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B21E0BB01E8224Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B21E0BB01E8224Au64;

        invoke_raw_typed!(
            hash,
                color
        )
    }
}

/// ```
SET_*
```



pub fn _set_director_mode_clear_triggered_flag_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2632482FD6B9AB87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2632482FD6B9AB87u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_director_mode_clear_triggered_flag_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2632482FD6B9AB87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2632482FD6B9AB87u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
This native turns on the AI blip on the specified ped. It also disappears automatically when the ped is too far or if the ped is dead. You don't need to control it with other natives.
See gtaforums.com/topic/884370-native-research-ai-blips for further information.
```



pub fn set_ped_has_ai_blip_safe(
        
        
            ped: 
        , 
        
        
            hasCone: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD30C50DF888D58B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD30C50DF888D58B5u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                hasCone
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_has_ai_blip_raw(
        ped: , 
        hasCone: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD30C50DF888D58B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD30C50DF888D58B5u64;

        invoke_raw_typed!(
            hash,
                ped, 
                hasCone
        )
    }
}

/// Hud colors can be found [here](https://docs.fivem.net/docs/game-references/hud-colors/)



pub fn replace_hud_colour_with_rgba_safe(
        
        
            hudColorIndex: 
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
        let hash = 0xF314CF4F0211894Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF314CF4F0211894Eu64;
        
        let result = invoke_raw!(
            hash,
                hudColorIndex, 
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
pub fn replace_hud_colour_with_rgba_raw(
        hudColorIndex: , 
        r: , 
        g: , 
        b: , 
        a: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF314CF4F0211894Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF314CF4F0211894Eu64;

        invoke_raw_typed!(
            hash,
                hudColorIndex, 
                r, 
                g, 
                b, 
                a
        )
    }
}

/// ```
Set the active slotIndex in the wheel weapon to the slot associated with the provided Weapon hash
```



pub fn hud_set_weapon_wheel_top_slot_safe(
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72C1056D678BB7D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72C1056D678BB7D8u64;
        
        let result = invoke_raw!(
            hash,
                weaponHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hud_set_weapon_wheel_top_slot_raw(
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72C1056D678BB7D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72C1056D678BB7D8u64;

        invoke_raw_typed!(
            hash,
                weaponHash
        )
    }
}

/// Adds a rectangular blip for the specified coordinates/area.

It is recommended to use [SET_BLIP_ROTATION](#_0xF87683CDF73C3F6E) and [SET_BLIP_COLOUR](#_0x03D7FB09E75D6B7E) to make the blip not rotate along with the camera.

By default, the blip will show as a _regular_ blip with the specified color/sprite if it is outside of the minimap view.

Example image:
![minimap](https://i.imgur.com/qLbXWcQ.png)
![big map](https://i.imgur.com/0j7O7Rh.png)

(Native name is _likely_ to actually be ADD_BLIP_FOR_AREA, but due to the usual reasons this can't be confirmed)



pub fn _add_blip_for_area_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            width: 
        , 
        
        
            height: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE5D0E5E315DB238u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE5D0E5E315DB238u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                width, 
                height
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _add_blip_for_area_raw(
        x: , 
        y: , 
        z: , 
        width: , 
        height: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE5D0E5E315DB238u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE5D0E5E315DB238u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                width, 
                height
        )
    }
}

/// ## Parameters
*



pub fn add_blip_for_pickup_safe(
        
        
            pickup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE339365C863BD36u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE339365C863BD36u64;
        
        let result = invoke_raw!(
            hash,
                pickup
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_blip_for_pickup_raw(
        pickup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE339365C863BD36u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE339365C863BD36u64;

        invoke_raw_typed!(
            hash,
                pickup
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x211c4ef450086857_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x211C4EF450086857u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x211C4EF450086857u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x211c4ef450086857_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x211C4EF450086857u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x211C4EF450086857u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Initializes the text entry for the the text next to a loading prompt. All natives for for building UI texts can be used here  
BEGIN_TEXT_COMMAND_PRINT  
e.g  
void StartLoadingMessage(char *text, int spinnerType = 3)  
	{  
_SET_LOADING_PROMPT_TEXT_ENTRY("STRING");  
ADD_TEXT_COMPONENT_SUBSTRING_PLAYER_NAME(text);  
_SHOW_LOADING_PROMPT(spinnerType);  
	}  
/*OR*/  
	void ShowLoadingMessage(char *text, int spinnerType = 3, int timeMs = 10000)  
	{  
_SET_LOADING_PROMPT_TEXT_ENTRY("STRING");  
ADD_TEXT_COMPONENT_SUBSTRING_PLAYER_NAME(text);  
_SHOW_LOADING_PROMPT(spinnerType);  
WAIT(timeMs);  
_REMOVE_LOADING_PROMPT();  
	}  
These are some localized strings used in the loading spinner.  
"PM_WAIT"                   = Please Wait  
"CELEB_WPLYRS"              = Waiting For Players.  
"CELL_SPINNER2"             = Scanning storage.  
"ERROR_CHECKYACHTNAME" = Registering your yacht's name. Please wait.  
"ERROR_CHECKPROFANITY"   = Checking your text for profanity. Please wait.  
"FM_COR_AUTOD"                        = Just spinner no text  
"FM_IHELP_WAT2"                        = Waiting for other players  
"FM_JIP_WAITO"                            = Game options are being set  
"FMMC_DOWNLOAD"                    = Downloading  
"FMMC_PLYLOAD"                         = Loading  
"FMMC_STARTTRAN"                    = Launching session  
"HUD_QUITTING"                           =  Quiting session  
"KILL_STRIP_IDM"                         = Waiting for to accept  
"MP_SPINLOADING"                      = Loading  
```



pub fn begin_text_command_busyspinner_on_safe(
        
        
            string: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xABA17D7CE615ADBFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xABA17D7CE615ADBFu64;
        
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
pub fn begin_text_command_busyspinner_on_raw(
        string: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xABA17D7CE615ADBFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xABA17D7CE615ADBFu64;

        invoke_raw_typed!(
            hash,
                string
        )
    }
}

/// ## Return value



pub fn is_frontend_ready_for_control_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BAB9A4E4F2FF5C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BAB9A4E4F2FF5C7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_frontend_ready_for_control_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BAB9A4E4F2FF5C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BAB9A4E4F2FF5C7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Adds the GTA: Online player heading indicator to a blip.  
```



pub fn show_heading_indicator_on_blip_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FBCA48327B914DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FBCA48327B914DFu64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn show_heading_indicator_on_blip_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FBCA48327B914DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FBCA48327B914DFu64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// Sets some extra options for a notification. It adds an image (or icon type) and sets a notification title (sender) and subtitle (subject).

Texture dictionary and texture name parameters are usually the same exact value.

Example result:

![](https://i.imgur.com/LviutDl.png)

Old description with list of possible icons and texture names:

```
List of picNames: pastebin.com/XdpJVbHz  
flash is a bool for fading in.  
iconTypes:  
1 : Chat Box  
2 : Email  
3 : Add Friend Request  
4 : Nothing  
5 : Nothing  
6 : Nothing  
7 : Right Jumping Arrow  
8 : RP Icon  
9 : $ Icon  
"sender" is the very top header. This can be any old string.  
"subject" is the header under the sender.  
```



pub fn end_text_command_thefeed_post_messagetext_safe(
        
        
            textureDict: 
        , 
        
        
            textureName: 
        , 
        
        
            flash: 
        , 
        
        
            iconType: 
        , 
        
        
            sender: 
        , 
        
        
            subject: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CCD9A37359072CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CCD9A37359072CFu64;
        
        let result = invoke_raw!(
            hash,
                textureDict, 
                textureName, 
                flash, 
                iconType, 
                sender, 
                subject
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_messagetext_raw(
        textureDict: , 
        textureName: , 
        flash: , 
        iconType: , 
        sender: , 
        subject: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CCD9A37359072CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CCD9A37359072CFu64;

        invoke_raw_typed!(
            hash,
                textureDict, 
                textureName, 
                flash, 
                iconType, 
                sender, 
                subject
        )
    }
}

/// CLEAR_PED_IN_PAUSE_MENU native function



pub fn clear_ped_in_pause_menu_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E62BE5DC58E9E06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E62BE5DC58E9E06u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_ped_in_pause_menu_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E62BE5DC58E9E06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E62BE5DC58E9E06u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns a substring that is between two specified positions. The length of the string will be calculated using (endPosition - startPosition).
Example:
// Get "STRING" text from "MY_STRING"
subStr = HUD::_GET_TEXT_SUBSTRING_SLICE("MY_STRING", 3, 9);
// Overflows are possibly replaced with underscores (needs verification)
subStr = HUD::_GET_TEXT_SUBSTRING_SLICE("MY_STRING", 3, 10); // "STRING_"?
```



pub fn _get_text_substring_slice_safe(
        
        
            text: 
        , 
        
        
            startPosition: 
        , 
        
        
            endPosition: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE94AEBA5D82908Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE94AEBA5D82908Au64;
        
        let result = invoke_raw!(
            hash,
                text, 
                startPosition, 
                endPosition
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_text_substring_slice_raw(
        text: , 
        startPosition: , 
        endPosition: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE94AEBA5D82908Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE94AEBA5D82908Au64;

        invoke_raw_typed!(
            hash,
                text, 
                startPosition, 
                endPosition
        )
    }
}

/// Does stuff like this:  
gyazo.com/7fcb78ea3520e3dbc5b2c0c0f3712617  
Example:  
int GetHash = GET_HASH_KEY("fe_menu_version_corona_lobby");  
ACTIVATE_FRONTEND_MENU(GetHash, 0, -1);  
BOOL p1 is a toggle to define the game in pause.  
int p2 is unknown but -1 always works, not sure why though.  
[30/03/2017] ins1de :  
the int p2 is actually a component variable. When the pause menu is visible, it opens the tab related to it.  
Example : Function.Call(Hash.ACTIVATE_FRONTEND_MENU,-1171018317, 0, 42);  
Result : Opens the "Online" tab without pausing the menu, with -1 it opens the map.  


Below is a list of all known Frontend Menu Hashes.

- FE_MENU_VERSION_SP_PAUSE
- FE_MENU_VERSION_MP_PAUSE
- FE_MENU_VERSION_CREATOR_PAUSE
- FE_MENU_VERSION_CUTSCENE_PAUSE
- FE_MENU_VERSION_SAVEGAME
- FE_MENU_VERSION_PRE_LOBBY
- FE_MENU_VERSION_LOBBY
- FE_MENU_VERSION_MP_CHARACTER_SELECT
- FE_MENU_VERSION_MP_CHARACTER_CREATION
- FE_MENU_VERSION_EMPTY
- FE_MENU_VERSION_EMPTY_NO_BACKGROUND
- FE_MENU_VERSION_TEXT_SELECTION
- FE_MENU_VERSION_CORONA
- FE_MENU_VERSION_CORONA_LOBBY
- FE_MENU_VERSION_CORONA_JOINED_PLAYERS
- FE_MENU_VERSION_CORONA_INVITE_PLAYERS
- FE_MENU_VERSION_CORONA_INVITE_FRIENDS
- FE_MENU_VERSION_CORONA_INVITE_CREWS
- FE_MENU_VERSION_CORONA_INVITE_MATCHED_PLAYERS
- FE_MENU_VERSION_CORONA_INVITE_LAST_JOB_PLAYERS
- FE_MENU_VERSION_CORONA_RACE
- FE_MENU_VERSION_CORONA_BETTING
- FE_MENU_VERSION_JOINING_SCREEN
- FE_MENU_VERSION_LANDING_MENU
- FE_MENU_VERSION_LANDING_KEYMAPPING_MENU



pub fn activate_frontend_menu_safe(
        
        
            menuhash: 
        , 
        
        
            togglePause: 
        , 
        
        
            component: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF01D36B9C9D0C7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF01D36B9C9D0C7Bu64;
        
        let result = invoke_raw!(
            hash,
                menuhash, 
                togglePause, 
                component
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn activate_frontend_menu_raw(
        menuhash: , 
        togglePause: , 
        component: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF01D36B9C9D0C7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF01D36B9C9D0C7Bu64;

        invoke_raw_typed!(
            hash,
                menuhash, 
                togglePause, 
                component
        )
    }
}

/// ```
I think this works, but seems to prohibit switching to other weapons (or accessing the weapon wheel)  
```



pub fn hide_hud_and_radar_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x719FF505F097FD20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x719FF505F097FD20u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hide_hud_and_radar_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x719FF505F097FD20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x719FF505F097FD20u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_next_blip_info_id_safe(
        
        
            blipSprite: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14F96AA50D6FBEA7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14F96AA50D6FBEA7u64;
        
        let result = invoke_raw!(
            hash,
                blipSprite
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_next_blip_info_id_raw(
        blipSprite: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14F96AA50D6FBEA7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14F96AA50D6FBEA7u64;

        invoke_raw_typed!(
            hash,
                blipSprite
        )
    }
}

/// ## Parameters
*



pub fn is_floating_help_text_on_screen_safe(
        
        
            hudIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2432784ACA090DA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2432784ACA090DA4u64;
        
        let result = invoke_raw!(
            hash,
                hudIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_floating_help_text_on_screen_raw(
        hudIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2432784ACA090DA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2432784ACA090DA4u64;

        invoke_raw_typed!(
            hash,
                hudIndex
        )
    }
}

/// ## Parameters
*



pub fn set_blip_high_detail_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2590BC29220CEBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2590BC29220CEBBu64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_high_detail_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2590BC29220CEBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2590BC29220CEBBu64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_blip_info_id_display_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E314167F701DC3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E314167F701DC3Bu64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_blip_info_id_display_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E314167F701DC3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E314167F701DC3Bu64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// ```
Does the same as SET_RACE_TRACK_RENDER(false);
```



pub fn clear_gps_race_track_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AA5B4CE533C858Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AA5B4CE533C858Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_gps_race_track_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AA5B4CE533C858Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AA5B4CE533C858Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// FORCE_CLOSE_REPORTUGC_MENU native function



pub fn force_close_reportugc_menu_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE4C0E6DBC6F2C6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE4C0E6DBC6F2C6Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_close_reportugc_menu_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE4C0E6DBC6F2C6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE4C0E6DBC6F2C6Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _CLOSE_MULTIPLAYER_CHAT native function



pub fn _close_multiplayer_chat_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1AC8F4AD40E22127u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1AC8F4AD40E22127u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _close_multiplayer_chat_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1AC8F4AD40E22127u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1AC8F4AD40E22127u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
HUD::SET_SOCIAL_CLUB_TOUR("Gallery");
HUD::SET_SOCIAL_CLUB_TOUR("Missions");
HUD::SET_SOCIAL_CLUB_TOUR("General");
HUD::SET_SOCIAL_CLUB_TOUR("Playlists");
```



pub fn set_social_club_tour_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E778248D6685FE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E778248D6685FE0u64;
        
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
pub fn set_social_club_tour_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E778248D6685FE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E778248D6685FE0u64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ```
If Hud should be displayed  
```



pub fn display_hud_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6294919E56FF02Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6294919E56FF02Au64;
        
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
pub fn display_hud_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6294919E56FF02Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6294919E56FF02Au64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_max_armour_hud_display_safe(
        
        
            maximumValue: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06A320535F5F0248u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06A320535F5F0248u64;
        
        let result = invoke_raw!(
            hash,
                maximumValue
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_max_armour_hud_display_raw(
        maximumValue: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06A320535F5F0248u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06A320535F5F0248u64;

        invoke_raw_typed!(
            hash,
                maximumValue
        )
    }
}

/// ## Return value



pub fn busyspinner_is_displaying_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2A592B04648A9CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2A592B04648A9CBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn busyspinner_is_displaying_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2A592B04648A9CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2A592B04648A9CBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Displays the crosshair for this frame.  
```



pub fn display_sniper_scope_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73115226F4814E62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73115226F4814E62u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn display_sniper_scope_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73115226F4814E62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73115226F4814E62u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn add_next_message_to_previous_briefs_safe(
        
        
            addToBrief: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60296AF4BA14ABC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60296AF4BA14ABC5u64;
        
        let result = invoke_raw!(
            hash,
                addToBrief
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_next_message_to_previous_briefs_raw(
        addToBrief: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60296AF4BA14ABC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60296AF4BA14ABC5u64;

        invoke_raw_typed!(
            hash,
                addToBrief
        )
    }
}

/// Starts a text command to change the name of a blip displayed in the pause menu.

This should be paired with [`END_TEXT_COMMAND_SET_BLIP_NAME`](#_0xBC38B49BCB83BC9B), once adding all required text components.



pub fn begin_text_command_set_blip_name_safe(
        
        
            textLabel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9113A30DE5C6670u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9113A30DE5C6670u64;
        
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
pub fn begin_text_command_set_blip_name_raw(
        textLabel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9113A30DE5C6670u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9113A30DE5C6670u64;

        invoke_raw_typed!(
            hash,
                textLabel
        )
    }
}

/// ```
From the decompiled scripts:
HUD::_92F0DA1E27DB96DC(6);
HUD::_92F0DA1E27DB96DC(184);
HUD::_92F0DA1E27DB96DC(190);
sets background color for the next notification
6 = red
184 = green
190 = yellow
Here is a list of some colors that can be used: gyazo.com/68bd384455fceb0a85a8729e48216e15
```



pub fn _thefeed_set_next_post_background_color_safe(
        
        
            hudColorIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92F0DA1E27DB96DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92F0DA1E27DB96DCu64;
        
        let result = invoke_raw!(
            hash,
                hudColorIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _thefeed_set_next_post_background_color_raw(
        hudColorIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92F0DA1E27DB96DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92F0DA1E27DB96DCu64;

        invoke_raw_typed!(
            hash,
                hudColorIndex
        )
    }
}

/// ## Return value



pub fn _0x2e22fefa0100275e_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E22FEFA0100275Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E22FEFA0100275Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2e22fefa0100275e_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E22FEFA0100275Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E22FEFA0100275Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
If true, remove all feed components instantly. Otherwise tween/animate close each component
```



pub fn _thefeed_set_flush_animpostfx_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAE4F9B97CD43B30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAE4F9B97CD43B30u64;
        
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
pub fn _thefeed_set_flush_animpostfx_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAE4F9B97CD43B30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAE4F9B97CD43B30u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_blip_colour_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF729E8D20CF7327u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF729E8D20CF7327u64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_blip_colour_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF729E8D20CF7327u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF729E8D20CF7327u64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// ## Parameters
*



pub fn is_blip_short_range_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA5F8727EB75B926u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA5F8727EB75B926u64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_blip_short_range_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA5F8727EB75B926u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA5F8727EB75B926u64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// ## Parameters
*



pub fn is_scripted_hud_component_hidden_this_frame_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x09C0403ED9A751C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x09C0403ED9A751C2u64;
        
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
pub fn is_scripted_hud_component_hidden_this_frame_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x09C0403ED9A751C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x09C0403ED9A751C2u64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ## Parameters
*



pub fn _0x0cf54f20de43879c_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CF54F20DE43879Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CF54F20DE43879Cu64;
        
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
pub fn _0x0cf54f20de43879c_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CF54F20DE43879Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CF54F20DE43879Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Returns the weapon hash active in a specific weapon wheel slotList
```



pub fn _hud_weapon_wheel_get_slot_hash_safe(
        
        
            weaponTypeIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA13E93403F26C812u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA13E93403F26C812u64;
        
        let result = invoke_raw!(
            hash,
                weaponTypeIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _hud_weapon_wheel_get_slot_hash_raw(
        weaponTypeIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA13E93403F26C812u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA13E93403F26C812u64;

        invoke_raw_typed!(
            hash,
                weaponTypeIndex
        )
    }
}

/// ```
Types -  
0: Center-Justify  
1: Left-Justify  
2: Right-Justify  
Right-Justify requires SET_TEXT_WRAP, otherwise it will draw to the far right of the screen  
```



pub fn set_text_justification_safe(
        
        
            justifyType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E096588B13FFECAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E096588B13FFECAu64;
        
        let result = invoke_raw!(
            hash,
                justifyType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_text_justification_raw(
        justifyType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E096588B13FFECAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E096588B13FFECAu64;

        invoke_raw_typed!(
            hash,
                justifyType
        )
    }
}

/// ```
Only the script that originally called SET_GPS_FLAGS can set them again. Another script cannot set the flags, until the first script that called it has called CLEAR_GPS_FLAGS.
Doesn't seem like the flags are actually read by the game at all.
```



pub fn set_gps_flags_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B440763A4C8D15Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B440763A4C8D15Bu64;
        
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
pub fn set_gps_flags_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B440763A4C8D15Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B440763A4C8D15Bu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Enable / disable showing route for the Blip-object.  
```



pub fn set_blip_route_safe(
        
        
            blip: 
        , 
        
        
            enabled: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F7D8A9BFB0B43E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F7D8A9BFB0B43E9u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                enabled
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_route_raw(
        blip: , 
        enabled: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F7D8A9BFB0B43E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F7D8A9BFB0B43E9u64;

        invoke_raw_typed!(
            hash,
                blip, 
                enabled
        )
    }
}

/// ```
Locks the minimap to the specified world position.  
```



pub fn lock_minimap_position_safe(
        
        
            x: 
        , 
        
        
            y: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1279E861A329E73Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1279E861A329E73Fu64;
        
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
pub fn lock_minimap_position_raw(
        x: , 
        y: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1279E861A329E73Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1279E861A329E73Fu64;

        invoke_raw_typed!(
            hash,
                x, 
                y
        )
    }
}

/// ## Parameters
*



pub fn toggle_stealth_radar_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6AFDFB93754950C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6AFDFB93754950C7u64;
        
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
pub fn toggle_stealth_radar_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6AFDFB93754950C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6AFDFB93754950C7u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn begin_text_command_is_this_help_message_being_displayed_safe(
        
        
            labelName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A24DA3A41B718F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A24DA3A41B718F5u64;
        
        let result = invoke_raw!(
            hash,
                labelName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_text_command_is_this_help_message_being_displayed_raw(
        labelName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A24DA3A41B718F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A24DA3A41B718F5u64;

        invoke_raw_typed!(
            hash,
                labelName
        )
    }
}

/// ## Parameters
*



pub fn _override_multiplayer_chat_prefix_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A1738B4323FE2D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A1738B4323FE2D9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _override_multiplayer_chat_prefix_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A1738B4323FE2D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A1738B4323FE2D9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Toggles the visibility of the cursor. Do note that if the game loses focus and then regains it, the cursor will become visible again.



pub fn set_mouse_cursor_visible_safe(
        
        
            isVisible: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98215325A695E78Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98215325A695E78Au64;
        
        let result = invoke_raw!(
            hash,
                isVisible
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mouse_cursor_visible_raw(
        isVisible: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98215325A695E78Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98215325A695E78Au64;

        invoke_raw_typed!(
            hash,
                isVisible
        )
    }
}

/// ```
Returns the weapon hash to the selected/highlighted weapon in the wheel
```



pub fn _hud_weapon_wheel_get_selected_hash_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA48931185F0536FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA48931185F0536FEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _hud_weapon_wheel_get_selected_hash_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA48931185F0536FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA48931185F0536FEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Flashes blip for time in milliseconds before stopping.



pub fn set_blip_flash_timer_safe(
        
        
            blip: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3CD6FD297AE87CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3CD6FD297AE87CCu64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_flash_timer_raw(
        blip: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3CD6FD297AE87CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3CD6FD297AE87CCu64;

        invoke_raw_typed!(
            hash,
                blip, 
                duration
        )
    }
}

/// ## Parameters
*



pub fn is_scripted_hud_component_active_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD100EB17A94FF65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD100EB17A94FF65u64;
        
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
pub fn is_scripted_hud_component_active_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD100EB17A94FF65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD100EB17A94FF65u64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ```
It adds the localized text of the specified GXT entry name. Eg. if the argument is GET_HASH_KEY("ES_HELP"), adds "Continue". Just uses a text labels hash key  
```



pub fn add_text_component_substring_text_label_hash_key_safe(
        
        
            gxtEntryHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17299B63C7683A2Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17299B63C7683A2Bu64;
        
        let result = invoke_raw!(
            hash,
                gxtEntryHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_text_component_substring_text_label_hash_key_raw(
        gxtEntryHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17299B63C7683A2Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17299B63C7683A2Bu64;

        invoke_raw_typed!(
            hash,
                gxtEntryHash
        )
    }
}

/// ## Return value



pub fn is_mp_gamer_tag_movie_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E0EB3EB47C8D7AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E0EB3EB47C8D7AAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_mp_gamer_tag_movie_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E0EB3EB47C8D7AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E0EB3EB47C8D7AAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
colors you input not same as you think?
A: for some reason its R B G A
```



pub fn set_text_colour_safe(
        
        
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
        let hash = 0xBE6B23FFA53FB442u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE6B23FFA53FB442u64;
        
        let result = invoke_raw!(
            hash,
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
pub fn set_text_colour_raw(
        red: , 
        green: , 
        blue: , 
        alpha: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE6B23FFA53FB442u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE6B23FFA53FB442u64;

        invoke_raw_typed!(
            hash,
                red, 
                green, 
                blue, 
                alpha
        )
    }
}

/// ```
This function is hard-coded to always return 1.  
```



pub fn get_default_script_rendertarget_render_id_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52F0982D7FD156B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52F0982D7FD156B6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_default_script_rendertarget_render_id_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52F0982D7FD156B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52F0982D7FD156B6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x4b5b620c9b59ed34_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B5B620C9B59ED34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B5B620C9B59ED34u64;
        
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
pub fn _0x4b5b620c9b59ed34_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B5B620C9B59ED34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B5B620C9B59ED34u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```c
enum eInstructionalButtonTypes
{
    NONE = 0,
    SELECT = 1,
    OK = 2,
    YES = 4,
    BACK = 8,
    BACK_SELECT = 9,
    BACK_OK = 10,
    BACK_YES = 12,
    CANCEL = 16,
    CANCEL_SELECT = 17,
    CANCEL_OK = 18,
    CANCEL_YES = 20,
    NO = 32,
    NO_SELECT = 33,
    NO_OK = 34,
    YES_NO = 36,
    RETRY = 64,
    RETRY_SELECT = 65,
    RETRY_OK = 66,
    RETRY_YES = 68,
    RETRY_BACK = 72,
    RETRY_BACK_SELECT = 73,
    RETRY_BACK_OK = 74,
    RETRY_BACK_YES = 76,
    RETRY_CANCEL = 80,
    RETRY_CANCEL_SELECT = 81,
    RETRY_CANCEL_OK = 82,
    RETRY_CANCEL_YES = 84,
    SKIP = 256,
    SKIP_SELECT = 257,
    SKIP_OK = 258,
    SKIP_YES = 260,
    SKIP_BACK = 264,
    SKIP_BACK_SELECT = 265,
    SKIP_BACK_OK = 266,
    SKIP_BACK_YES = 268,
    SKIP_CANCEL = 272,
    SKIP_CANCEL_SELECT = 273,
    SKIP_CANCEL_OK = 274,
    SKIP_CANCEL_YES = 276,
    CONTINUE = 16384,
    BACK_CONTINUE = 16392,
    CANCEL_CONTINUE = 16400,
    LOADING_SPINNER = 134217728,
    SELECT_LOADING_SPINNER = 134217729,
    OK_LOADING_SPINNER = 134217730,
    YES_LOADING_SPINNER = 134217732,
    BACK_LOADING_SPINNER = 134217736,
    BACK_SELECT_LOADING_SPINNER = 134217737,
    BACK_OK_LOADING_SPINNER = 134217738,
    BACK_YES_LOADING_SPINNER = 134217740,
    CANCEL_LOADING_SPINNER = 134217744,
    CANCEL_SELECT_LOADING_SPINNER = 134217745,
    CANCEL_OK_LOADING_SPINNER = 134217746,
    CANCEL_YES_LOADING_SPINNER = 134217748
}
```

Note: this list is definitely NOT complete, but these are the ones I've been able to find before giving up because it's such a boring thing to look for 'good' combinations.



pub fn set_warning_message_safe(
        
        
            entryLine1: 
        , 
        
        
            instructionalKey: 
        , 
        
        
            entryLine2: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B1776B3B53F8D74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B1776B3B53F8D74u64;
        
        let result = invoke_raw!(
            hash,
                entryLine1, 
                instructionalKey, 
                entryLine2, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_warning_message_raw(
        entryLine1: , 
        instructionalKey: , 
        entryLine2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7B1776B3B53F8D74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7B1776B3B53F8D74u64;

        invoke_raw_typed!(
            hash,
                entryLine1, 
                instructionalKey, 
                entryLine2, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn get_blip_alpha_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x970F608F0EE6C885u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x970F608F0EE6C885u64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_blip_alpha_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x970F608F0EE6C885u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x970F608F0EE6C885u64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// ## Return value



pub fn is_reportugc_menu_open_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9135584D09A3437Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9135584D09A3437Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_reportugc_menu_open_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9135584D09A3437Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9135584D09A3437Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Changes the hud color at a given index (hudColorIndex) by another one (hudColorIndex2).

HUD colors can be found [here](https://docs.fivem.net/docs/game-references/hud-colors/)



pub fn replace_hud_colour_safe(
        
        
            hudColorIndex: 
        , 
        
        
            hudColorIndex2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CCC708F0F850613u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CCC708F0F850613u64;
        
        let result = invoke_raw!(
            hash,
                hudColorIndex, 
                hudColorIndex2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn replace_hud_colour_raw(
        hudColorIndex: , 
        hudColorIndex2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CCC708F0F850613u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CCC708F0F850613u64;

        invoke_raw_typed!(
            hash,
                hudColorIndex, 
                hudColorIndex2
        )
    }
}

/// ## Parameters
*



pub fn allow_sonar_blips_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60734CC207C9833Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60734CC207C9833Cu64;
        
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
pub fn allow_sonar_blips_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60734CC207C9833Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60734CC207C9833Cu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// This function shows various HUD (Heads-up Display) components.

Listed below are the integers and the corresponding HUD component.
- 1 : WANTED_STARS
- 2 : WEAPON_ICON
- 3 : CASH
- 4 : MP_CASH
- 5 : MP_MESSAGE
- 6 : VEHICLE_NAME
- 7 : AREA_NAME
- 8 : VEHICLE_CLASS
- 9 : STREET_NAME
- 10 : HELP_TEXT
- 11 : FLOATING_HELP_TEXT_1
- 12 : FLOATING_HELP_TEXT_2
- 13 : CASH_CHANGE
- 14 : RETICLE
- 15 : SUBTITLE_TEXT
- 16 : RADIO_STATIONS
- 17 : SAVING_GAME
- 18 : GAME_STREAM
- 19 : WEAPON_WHEEL
- 20 : WEAPON_WHEEL_STATS
- 21 : HUD_COMPONENTS
- 22 : HUD_WEAPONS

These integers also work for the [`HIDE_HUD_COMPONENT_THIS_FRAME`](#_0x6806C51AD12B83B8) native, but instead hides the HUD component.



pub fn show_hud_component_this_frame_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B4DF1FA60C0E664u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B4DF1FA60C0E664u64;
        
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
pub fn show_hud_component_this_frame_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B4DF1FA60C0E664u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B4DF1FA60C0E664u64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x35a3cd97b2c0a6d2_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35A3CD97B2C0A6D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35A3CD97B2C0A6D2u64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x35a3cd97b2c0a6d2_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35A3CD97B2C0A6D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35A3CD97B2C0A6D2u64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// Correct native name lies between SET_BLIP_SPRITE and SET_RADIUS_BLIP_EDGE alphabetically.



pub fn _0x2c9f302398e13141_safe(
        
        
            blip: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C9F302398E13141u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C9F302398E13141u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2c9f302398e13141_raw(
        blip: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C9F302398E13141u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C9F302398E13141u64;

        invoke_raw_typed!(
            hash,
                blip, 
                p1
        )
    }
}

/// ```c
enum eTextFonts
{
    FONT_STANDARD = 0,
    FONT_CURSIVE = 1,
    FONT_ROCKSTAR_TAG = 2,
    FONT_LEADERBOAR0D = 3,
    FONT_CONDENSED = 4,
    FONT_STYLE_FIXED_WIDTH_NUMBERS = 5,
    FONT_CONDENSED_NOT_GAMERNAME = 6,
    FONT_STYLE_PRICEDOWN = 7,
    FONT_STYLE_TAXI = 8,
}
```



pub fn set_text_font_safe(
        
        
            fontType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66E0276CC5F6B9DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66E0276CC5F6B9DAu64;
        
        let result = invoke_raw!(
            hash,
                fontType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_text_font_raw(
        fontType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66E0276CC5F6B9DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66E0276CC5F6B9DAu64;

        invoke_raw_typed!(
            hash,
                fontType
        )
    }
}

/// Toggles if the text input box can be opened with [`DISPLAY_ONSCREEN_KEYBOARD`](#_0x00DC833F2568DBF6).



pub fn set_text_input_box_enabled_safe(
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1185A8087587322Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1185A8087587322Cu64;
        
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
pub fn set_text_input_box_enabled_raw(
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1185A8087587322Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1185A8087587322Cu64;

        invoke_raw_typed!(
            hash,
                state
        )
    }
}

/// Changes the current frontend menu to the desired frontend menu version.



pub fn restart_frontend_menu_safe(
        
        
            menuHash: 
        , 
        
        
            highlightedTab: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10706DC6AD2D49C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10706DC6AD2D49C0u64;
        
        let result = invoke_raw!(
            hash,
                menuHash, 
                highlightedTab
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn restart_frontend_menu_raw(
        menuHash: , 
        highlightedTab: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10706DC6AD2D49C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10706DC6AD2D49C0u64;

        invoke_raw_typed!(
            hash,
                menuHash, 
                highlightedTab
        )
    }
}

/// ## Return value



pub fn get_waypoint_blip_enum_id_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x186E5D252FA50E7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x186E5D252FA50E7Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_waypoint_blip_enum_id_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x186E5D252FA50E7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x186E5D252FA50E7Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_pausemap_in_interior_mode_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9049FE339D5F6F6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9049FE339D5F6F6Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_pausemap_in_interior_mode_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9049FE339D5F6F6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9049FE339D5F6F6Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Size range : 0F to 1.0F
p0 is unknown and doesn't seem to have an effect, yet in the game scripts it changes to 1.0F sometimes.
```



pub fn set_text_scale_safe(
        
        
            scale: 
        , 
        
        
            size: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07C837F9A01C34C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07C837F9A01C34C9u64;
        
        let result = invoke_raw!(
            hash,
                scale, 
                size
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_text_scale_raw(
        scale: , 
        size: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07C837F9A01C34C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07C837F9A01C34C9u64;

        invoke_raw_typed!(
            hash,
                scale, 
                size
        )
    }
}

/// ## Parameters
*



pub fn set_floating_help_text_world_position_safe(
        
        
            hudIndex: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x784BA7E0ECEB4178u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x784BA7E0ECEB4178u64;
        
        let result = invoke_raw!(
            hash,
                hudIndex, 
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
pub fn set_floating_help_text_world_position_raw(
        hudIndex: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x784BA7E0ECEB4178u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x784BA7E0ECEB4178u64;

        invoke_raw_typed!(
            hash,
                hudIndex, 
                x, 
                y, 
                z
        )
    }
}

/// ```
World to relative screen coords
this world to screen will keep the text on screen. it will keep it in the screen pos
```



pub fn get_hud_screen_position_from_world_position_safe(
        
        
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
        let hash = 0xF9904D11F1ACBEC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9904D11F1ACBEC3u64;
        
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
pub fn get_hud_screen_position_from_world_position_raw(
        worldX: , 
        worldY: , 
        worldZ: , 
        screenX: , 
        screenY: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9904D11F1ACBEC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9904D11F1ACBEC3u64;

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



pub fn pause_menu_is_context_active_safe(
        
        
            contextHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84698AB38D0C6636u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84698AB38D0C6636u64;
        
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
pub fn pause_menu_is_context_active_raw(
        contextHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84698AB38D0C6636u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84698AB38D0C6636u64;

        invoke_raw_typed!(
            hash,
                contextHash
        )
    }
}

/// ## Parameters
*



pub fn is_named_rendertarget_linked_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x113750538FA31298u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x113750538FA31298u64;
        
        let result = invoke_raw!(
            hash,
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_named_rendertarget_linked_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x113750538FA31298u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x113750538FA31298u64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// ```
This gets the height of the FONT and not the total text. You need to get the number of lines your text uses, and get the height of a newline (I'm using a smaller value) to get the total text height.
Old name: _GET_TEXT_SCALE_HEIGHT
```



pub fn get_rendered_character_height_safe(
        
        
            size: 
        , 
        
        
            font: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB88A37483346780u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB88A37483346780u64;
        
        let result = invoke_raw!(
            hash,
                size, 
                font
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_rendered_character_height_raw(
        size: , 
        font: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB88A37483346780u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB88A37483346780u64;

        invoke_raw_typed!(
            hash,
                size, 
                font
        )
    }
}

/// As per the name, this native creates a "fake" gamer tag that is attached to a specific ped.
Unlike "real" gamer tags, you cannot set the crew colour of these gamer tags.

To create gamer tags for actual players and for more gamer tag information, see [CREATE_MP_GAMER_TAG_WITH_CREW_COLOR](#_0x6DD05E9D83EFA4C9).



pub fn create_fake_mp_gamer_tag_safe(
        
        
            ped: 
        , 
        
        
            username: 
        , 
        
        
            crewIsPrivate: 
        , 
        
        
            crewIsRockstar: 
        , 
        
        
            crewName: 
        , 
        
        
            crewRank: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFEFE3321A3F5015u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFEFE3321A3F5015u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                username, 
                crewIsPrivate, 
                crewIsRockstar, 
                crewName, 
                crewRank
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_fake_mp_gamer_tag_raw(
        ped: , 
        username: , 
        crewIsPrivate: , 
        crewIsRockstar: , 
        crewName: , 
        crewRank: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFEFE3321A3F5015u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFEFE3321A3F5015u64;

        invoke_raw_typed!(
            hash,
                ped, 
                username, 
                crewIsPrivate, 
                crewIsRockstar, 
                crewName, 
                crewRank
        )
    }
}

/// Not much is known so far on what it does _exactly_.
All I know for sure is that it draws the specified hole ID on the pause menu map as well as on the mini-map/radar. This native also seems to change some other things related to the pause menu map's behaviour, for example: you can no longer set waypoints, the pause menu map starts up in a 'zoomed in' state. This native does not need to be executed every tick.

You need to center the minimap manually as well as change/lock it's zoom and angle in order for it to appear correctly on the minimap.
You'll also need to use the `GOLF` scaleform in order to get the correct minmap border to show up.

Use [`SET_MINIMAP_GOLF_COURSE_OFF()`](#_0x35EDD5B2E3FF01C0) to reset the map when you no longer want to display any golf holes (you still need to unlock zoom, position and angle of the radar manually after calling this).



pub fn set_minimap_golf_course_safe(
        
        
            hole: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71BDB63DBAF8DA59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71BDB63DBAF8DA59u64;
        
        let result = invoke_raw!(
            hash,
                hole
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_minimap_golf_course_raw(
        hole: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71BDB63DBAF8DA59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71BDB63DBAF8DA59u64;

        invoke_raw_typed!(
            hash,
                hole
        )
    }
}

/// ## Parameters
*



pub fn _0xe67c6dfd386ea5e7_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE67C6DFD386EA5E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE67C6DFD386EA5E7u64;
        
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
pub fn _0xe67c6dfd386ea5e7_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE67C6DFD386EA5E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE67C6DFD386EA5E7u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Sets the mouse pointer to be active on current frame.



pub fn set_mouse_cursor_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAE7CE1D63167423u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAE7CE1D63167423u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mouse_cursor_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAE7CE1D63167423u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAE7CE1D63167423u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _CLEAR_RACE_GALLERY_BLIPS native function



pub fn _clear_race_gallery_blips_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2708FC083123F9FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2708FC083123F9FFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _clear_race_gallery_blips_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2708FC083123F9FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2708FC083123F9FFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _0x55f5a5f07134de60_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55F5A5F07134DE60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55F5A5F07134DE60u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x55f5a5f07134de60_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55F5A5F07134DE60u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55F5A5F07134DE60u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Toggles a cyan outline around the blip.

Color can be changed with [`SET_BLIP_SECONDARY_COLOUR`](#_0x14892474891E09EB). Enabling this circle will override the "crew" and "friend" half-circles (see [`SHOW_CREW_INDICATOR_ON_BLIP`](#_0xDCFB5D4DB8BF367E) and [`SHOW_FRIEND_INDICATOR_ON_BLIP`](#_0x23C3EB807312F01A)).



pub fn show_outline_indicator_on_blip_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB81656BC81FE24D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB81656BC81FE24D1u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn show_outline_indicator_on_blip_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB81656BC81FE24D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB81656BC81FE24D1u64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn end_text_command_thefeed_post_ticker_forced_safe(
        
        
            blink: 
        , 
        
        
            bHasTokens: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44FA03975424A0EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44FA03975424A0EEu64;
        
        let result = invoke_raw!(
            hash,
                blink, 
                bHasTokens
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_ticker_forced_raw(
        blink: , 
        bHasTokens: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44FA03975424A0EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44FA03975424A0EEu64;

        invoke_raw_typed!(
            hash,
                blink, 
                bHasTokens
        )
    }
}

/// This function can show pictures of every texture that can be requested by REQUEST_STREAMED_TEXTURE_DICT.
Needs more research.
Only one type of usage in the scripts:
HUD::_C6F580E4C94926AC("CHAR_ACTING_UP", "CHAR_ACTING_UP", 0, 0, "DI_FEED_CHAR", a_0);



pub fn _end_text_command_thefeed_post_messagetext_gxt_entry_safe(
        
        
            txdName: 
        , 
        
        
            textureName: 
        , 
        
        
            flash: 
        , 
        
        
            iconType: 
        , 
        
        
            sender: 
        , 
        
        
            subject: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6F580E4C94926ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6F580E4C94926ACu64;
        
        let result = invoke_raw!(
            hash,
                txdName, 
                textureName, 
                flash, 
                iconType, 
                sender, 
                subject
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _end_text_command_thefeed_post_messagetext_gxt_entry_raw(
        txdName: , 
        textureName: , 
        flash: , 
        iconType: , 
        sender: , 
        subject: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6F580E4C94926ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6F580E4C94926ACu64;

        invoke_raw_typed!(
            hash,
                txdName, 
                textureName, 
                flash, 
                iconType, 
                sender, 
                subject
        )
    }
}

/// ```
The following were found in the decompiled script files:  
STRING, TWOSTRINGS, NUMBER, PERCENTAGE, FO_TWO_NUM, ESMINDOLLA, ESDOLLA, MTPHPER_XPNO, AHD_DIST, CMOD_STAT_0, CMOD_STAT_1, CMOD_STAT_2, CMOD_STAT_3, DFLT_MNU_OPT, F3A_TRAFDEST, ES_HELP_SOC3  
ESDOLLA   
ESMINDOLLA - cash (negative)  
Used to be known as _SET_TEXT_ENTRY  
```



pub fn begin_text_command_display_text_safe(
        
        
            text: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25FBB336DF1804CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25FBB336DF1804CBu64;
        
        let result = invoke_raw!(
            hash,
                text
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn begin_text_command_display_text_raw(
        text: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25FBB336DF1804CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25FBB336DF1804CBu64;

        invoke_raw_typed!(
            hash,
                text
        )
    }
}

/// ```
Doesn't actually return anything.
```



pub fn force_sonar_blips_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1121BFA1A1A522A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1121BFA1A1A522A8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_sonar_blips_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1121BFA1A1A522A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1121BFA1A1A522A8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Draws the subtitle at middle center of the screen.  
int duration = time in milliseconds to show text on screen before disappearing  
drawImmediately = If true, the text will be drawn immediately, if false, the text will be drawn after the previous subtitle has finished  
Used to be known as _DRAW_SUBTITLE_TIMED  
```



pub fn end_text_command_print_safe(
        
        
            duration: 
        , 
        
        
            drawImmediately: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D77056A530643F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D77056A530643F6u64;
        
        let result = invoke_raw!(
            hash,
                duration, 
                drawImmediately
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_print_raw(
        duration: , 
        drawImmediately: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D77056A530643F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D77056A530643F6u64;

        invoke_raw_typed!(
            hash,
                duration, 
                drawImmediately
        )
    }
}

/// Certain characters like `<` will have to be escaped using html entities (e.g. `&lt;`), otherwise the text wont display properly.



pub fn add_text_component_substring_keyboard_display_safe(
        
        
            string: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F68520888E69014u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F68520888E69014u64;
        
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
pub fn add_text_component_substring_keyboard_display_raw(
        string: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F68520888E69014u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F68520888E69014u64;

        invoke_raw_typed!(
            hash,
                string
        )
    }
}

/// ## Parameters
*



pub fn display_area_name_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x276B6CE369C33678u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x276B6CE369C33678u64;
        
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
pub fn display_area_name_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x276B6CE369C33678u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x276B6CE369C33678u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Sets a loading icon in the pause menu.



pub fn pause_menu_set_busy_spinner_safe(
        
        
            bVisible: 
        , 
        
        
            iColumnID: 
        , 
        
        
            iSpinnerIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC78E239AC5B2DDB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC78E239AC5B2DDB9u64;
        
        let result = invoke_raw!(
            hash,
                bVisible, 
                iColumnID, 
                iSpinnerIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn pause_menu_set_busy_spinner_raw(
        bVisible: , 
        iColumnID: , 
        iSpinnerIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC78E239AC5B2DDB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC78E239AC5B2DDB9u64;

        invoke_raw_typed!(
            hash,
                bVisible, 
                iColumnID, 
                iSpinnerIndex
        )
    }
}

/// HIDE_HELP_TEXT_THIS_FRAME native function



pub fn hide_help_text_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD46923FC481CA285u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD46923FC481CA285u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hide_help_text_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD46923FC481CA285u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD46923FC481CA285u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
See this topic for more details : gtaforums.com/topic/717612-v-scriptnative-documentation-and-research/page-35?p=1069477935  
```



pub fn set_blip_priority_safe(
        
        
            blip: 
        , 
        
        
            priority: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE9FC9EF6A9FAC79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE9FC9EF6A9FAC79u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                priority
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_priority_raw(
        blip: , 
        priority: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE9FC9EF6A9FAC79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE9FC9EF6A9FAC79u64;

        invoke_raw_typed!(
            hash,
                blip, 
                priority
        )
    }
}

/// ## Parameters
*



pub fn set_blip_flashes_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB14552383D39CE3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB14552383D39CE3Eu64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_flashes_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB14552383D39CE3Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB14552383D39CE3Eu64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_minimap_block_waypoint_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58FADDED207897DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58FADDED207897DCu64;
        
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
pub fn set_minimap_block_waypoint_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58FADDED207897DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58FADDED207897DCu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Toggles the light state for the pause menu ped in frontend menus.

This is used by R* in combination with [`SET_PAUSE_MENU_PED_SLEEP_STATE`](#_0xECF128344E9FF9F1) to toggle the "offline" or "online" state in the "friends" tab of the pause menu in GTA Online.

Example:
On: ![lights on](https://vespura.com/hi/i/2019-04-01_16-09_540ee_1015.png)
Off: ![lights off](https://vespura.com/hi/i/2019-04-01_16-10_8b5e7_1016.png)



pub fn set_pause_menu_ped_lighting_safe(
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3CA6050692BC61B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3CA6050692BC61B0u64;
        
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
pub fn set_pause_menu_ped_lighting_raw(
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3CA6050692BC61B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3CA6050692BC61B0u64;

        invoke_raw_typed!(
            hash,
                state
        )
    }
}

/// ## Parameters
*



pub fn reset_global_actionscript_flag_safe(
        
        
            flagIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB99C4E4D9499DF29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB99C4E4D9499DF29u64;
        
        let result = invoke_raw!(
            hash,
                flagIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_global_actionscript_flag_raw(
        flagIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB99C4E4D9499DF29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB99C4E4D9499DF29u64;

        invoke_raw_typed!(
            hash,
                flagIndex
        )
    }
}

/// ```
Displays cash change notifications on HUD.  
```



pub fn change_fake_mp_cash_safe(
        
        
            cash: 
        , 
        
        
            bank: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0772DF77852C2E30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0772DF77852C2E30u64;
        
        let result = invoke_raw!(
            hash,
                cash, 
                bank
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn change_fake_mp_cash_raw(
        cash: , 
        bank: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0772DF77852C2E30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0772DF77852C2E30u64;

        invoke_raw_typed!(
            hash,
                cash, 
                bank
        )
    }
}

/// ## Parameters
*



pub fn _get_pause_menu_selection_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36C1451A88A09630u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36C1451A88A09630u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_pause_menu_selection_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36C1451A88A09630u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36C1451A88A09630u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
returns a notification handle, prints out a notification like below:
type range: 0 - 2
if you set type to 1, button accepts "~INPUT_SOMETHING~"
example:
HUD::_0xDD6CB2CCE7C2735C(1, "~INPUT_TALK~", "Who you trynna get crazy with, ese? Don't you know I'm LOCO?!");
- imgur.com/UPy0Ial
Examples from the scripts:
l_D1[1/*1*/]=HUD::_DD6CB2CCE7C2735C(1,"~INPUT_REPLAY_START_STOP_RECORDING~","");
l_D1[2/*1*/]=HUD::_DD6CB2CCE7C2735C(1,"~INPUT_SAVE_REPLAY_CLIP~","");
l_D1[1/*1*/]=HUD::_DD6CB2CCE7C2735C(1,"~INPUT_REPLAY_START_STOP_RECORDING~","");
l_D1[2/*1*/]=HUD::_DD6CB2CCE7C2735C(1,"~INPUT_REPLAY_START_STOP_RECORDING_SECONDARY~","");
```



pub fn _end_text_command_thefeed_post_replay_input_safe(
        
        
            type: 
        , 
        
        
            button: 
        , 
        
        
            text: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD6CB2CCE7C2735Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD6CB2CCE7C2735Cu64;
        
        let result = invoke_raw!(
            hash,
                type, 
                button, 
                text
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _end_text_command_thefeed_post_replay_input_raw(
        type: , 
        button: , 
        text: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD6CB2CCE7C2735Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD6CB2CCE7C2735Cu64;

        invoke_raw_typed!(
            hash,
                type, 
                button, 
                text
        )
    }
}

/// Create a blip that by default is red (enemy), you can use [SET_BLIP_AS_FRIENDLY](#_0xC6F43D0E) to make it blue (friend).  
Can be used for objects, vehicles and peds.

Example of enemy:
![enemy](https://i.imgur.com/fl78svv.png)
Example of friend:
![friend](https://i.imgur.com/Q16ho5d.png)



pub fn add_blip_for_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CDE92C702A8FCE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CDE92C702A8FCE7u64;
        
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
pub fn add_blip_for_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CDE92C702A8FCE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CDE92C702A8FCE7u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ```
if (HUD::GET_CURRENT_FRONTEND_MENU_VERSION() == joaat("fe_menu_version_empty_no_background"))
```



pub fn get_current_frontend_menu_version_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2309595AD6145265u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2309595AD6145265u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_current_frontend_menu_version_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2309595AD6145265u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2309595AD6145265u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn busyspinner_is_on_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD422FCC5F239A915u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD422FCC5F239A915u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn busyspinner_is_on_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD422FCC5F239A915u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD422FCC5F239A915u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Returns whether a specific help message is being displayed or not.

```c
enum HudIndexes {
    HELP_TEXT = 0,
    FLOATING_HELP_TEXT_1 = 1,
    FLOATING_HELP_TEXT_2 = 2,
}
```



pub fn end_text_command_is_this_help_message_being_displayed_safe(
        
        
            hudIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10BDDBFC529428DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10BDDBFC529428DDu64;
        
        let result = invoke_raw!(
            hash,
                hudIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_is_this_help_message_being_displayed_raw(
        hudIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10BDDBFC529428DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10BDDBFC529428DDu64;

        invoke_raw_typed!(
            hash,
                hudIndex
        )
    }
}

/// ```
Enables loading screen tips to be be shown (`_0x15CFA549788D35EF` and `_0x488043841BBE156F`), blocks other kinds of notifications from being displayed (at least from current script). Call `0xADED7F5748ACAFE6` to display those again.
```



pub fn thefeed_comment_teleport_pool_on_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56C8B608CFD49854u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56C8B608CFD49854u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_comment_teleport_pool_on_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56C8B608CFD49854u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56C8B608CFD49854u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Create a blip with a radius for the specified coordinates (it doesnt create the blip sprite, so you need to use [AddBlipCoords](#_0xC6F43D0E))

Example image:
![example](https://i.imgur.com/9hQl3DB.png)



pub fn add_blip_for_radius_safe(
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46818D79B1F7499Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46818D79B1F7499Au64;
        
        let result = invoke_raw!(
            hash,
                posX, 
                posY, 
                posZ, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_blip_for_radius_raw(
        posX: , 
        posY: , 
        posZ: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46818D79B1F7499Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46818D79B1F7499Au64;

        invoke_raw_typed!(
            hash,
                posX, 
                posY, 
                posZ, 
                radius
        )
    }
}

/// ```
FORCE_*
```



pub fn _0xba8d65c1c65702e5_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA8D65C1C65702E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA8D65C1C65702E5u64;
        
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
pub fn _0xba8d65c1c65702e5_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA8D65C1C65702E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA8D65C1C65702E5u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_minimap_fow_coordinate_is_revealed_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E31B91145873922u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E31B91145873922u64;
        
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
pub fn get_minimap_fow_coordinate_is_revealed_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E31B91145873922u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E31B91145873922u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ## Return value



pub fn get_current_webpage_id_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01A358D9128B7A86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01A358D9128B7A86u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_current_webpage_id_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01A358D9128B7A86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01A358D9128B7A86u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xa238192f33110615_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA238192F33110615u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA238192F33110615u64;
        
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
pub fn _0xa238192f33110615_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA238192F33110615u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA238192F33110615u64;

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



pub fn has_additional_text_loaded_safe(
        
        
            slot: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02245FE4BED318B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02245FE4BED318B8u64;
        
        let result = invoke_raw!(
            hash,
                slot
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_additional_text_loaded_raw(
        slot: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02245FE4BED318B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02245FE4BED318B8u64;

        invoke_raw_typed!(
            hash,
                slot
        )
    }
}

/// ```
SET_F*
```



pub fn _0x2790f4b17d098e26_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2790F4B17D098E26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2790F4B17D098E26u64;
        
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
pub fn _0x2790f4b17d098e26_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2790F4B17D098E26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2790F4B17D098E26u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn trigger_sonar_blip_safe(
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            radius: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72DD432F3CDFC0EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72DD432F3CDFC0EEu64;
        
        let result = invoke_raw!(
            hash,
                posX, 
                posY, 
                posZ, 
                radius, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn trigger_sonar_blip_raw(
        posX: , 
        posY: , 
        posZ: , 
        radius: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72DD432F3CDFC0EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72DD432F3CDFC0EEu64;

        invoke_raw_typed!(
            hash,
                posX, 
                posY, 
                posZ, 
                radius, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn release_named_rendertarget_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE9F6FFE837354DD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE9F6FFE837354DD4u64;
        
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
pub fn release_named_rendertarget_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE9F6FFE837354DD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE9F6FFE837354DD4u64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ## Parameters
*



pub fn _0xc594b315edf2d4af_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC594B315EDF2D4AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC594B315EDF2D4AFu64;
        
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
pub fn _0xc594b315edf2d4af_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC594B315EDF2D4AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC594B315EDF2D4AFu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Gets hud color RGBA parameter values by passing a hud color index (hudColorIndex).

HUD colors can be found [here](https://docs.fivem.net/docs/game-references/hud-colors/)



pub fn get_hud_colour_safe(
        
        
            hudColorIndex: 
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
        let hash = 0x7C9C91AB74A0360Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C9C91AB74A0360Fu64;
        
        let result = invoke_raw!(
            hash,
                hudColorIndex, 
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
pub fn get_hud_colour_raw(
        hudColorIndex: , 
        r: , 
        g: , 
        b: , 
        a: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C9C91AB74A0360Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C9C91AB74A0360Fu64;

        invoke_raw_typed!(
            hash,
                hudColorIndex, 
                r, 
                g, 
                b, 
                a
        )
    }
}

/// ```
p1 is either 1 or 2 in the PC scripts.  
```


This native is used to "give"/duplicate a player ped to a frontend menu as configured via the `ACTIVATE_FRONTEND_MENU` native, you first must utilize the [CLONE_PED](#_0xEF29A16337FACADB) to clone said ped.



pub fn give_ped_to_pause_menu_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC0BFBDC3BE00E14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC0BFBDC3BE00E14u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn give_ped_to_pause_menu_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC0BFBDC3BE00E14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC0BFBDC3BE00E14u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
if "flag" is true, the AI blip will always be displayed for the specified ped, if it has an AI blip  
If "flag" is false, the AI blip will only be displayed when the player is in combat with the specified ped, if it has an AI blip  
```



pub fn set_ped_ai_blip_forced_on_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C4BBF625CA98C4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C4BBF625CA98C4Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_ai_blip_forced_on_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C4BBF625CA98C4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C4BBF625CA98C4Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_blip_coords_safe(
        
        
            blip: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE2AF67E9D9AF65Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE2AF67E9D9AF65Du64;
        
        let result = invoke_raw!(
            hash,
                blip, 
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
pub fn set_blip_coords_raw(
        blip: , 
        posX: , 
        posY: , 
        posZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE2AF67E9D9AF65Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE2AF67E9D9AF65Du64;

        invoke_raw_typed!(
            hash,
                blip, 
                posX, 
                posY, 
                posZ
        )
    }
}

/// Toggles whether or not name labels are shown on the expanded minimap next to player blips, like in GTA:O.
Doesn't need to be called every frame.
Preview: https://i.imgur.com/DfqKWfJ.png
Make sure to call SET_BLIP_CATEGORY with index 7 for this to work on the desired blip.



pub fn display_player_name_tags_on_blips_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82CEDC33687E1F50u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82CEDC33687E1F50u64;
        
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
pub fn display_player_name_tags_on_blips_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82CEDC33687E1F50u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82CEDC33687E1F50u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// SUPPRESS_FRONTEND_RENDERING_THIS_FRAME native function



pub fn suppress_frontend_rendering_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA751764F0821256u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA751764F0821256u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn suppress_frontend_rendering_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA751764F0821256u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA751764F0821256u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p0 was always 0xAE2602A3.
```



pub fn get_menu_ped_bool_stat_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x052991E59076E4E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x052991E59076E4E4u64;
        
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
pub fn get_menu_ped_bool_stat_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x052991E59076E4E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x052991E59076E4E4u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_mp_gamer_tag_name_safe(
        
        
            gamerTagId: 
        , 
        
        
            string: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEA2B8283BAA3944u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEA2B8283BAA3944u64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId, 
                string
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mp_gamer_tag_name_raw(
        gamerTagId: , 
        string: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEA2B8283BAA3944u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEA2B8283BAA3944u64;

        invoke_raw_typed!(
            hash,
                gamerTagId, 
                string
        )
    }
}

/// ```
Add a BLIP_GALLERY at the specific coordinate. Used in fm_maintain_transition_players to display race track points.
```



pub fn _race_gallery_add_blip_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x551DF99658DB6EE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x551DF99658DB6EE8u64;
        
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
pub fn _race_gallery_add_blip_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x551DF99658DB6EE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x551DF99658DB6EE8u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// This function hides various HUD (Heads-up Display) components.

Listed below are the integers and the corresponding HUD component.
- 1 : WANTED_STARS
- 2 : WEAPON_ICON
- 3 : CASH
- 4 : MP_CASH
- 5 : MP_MESSAGE
- 6 : VEHICLE_NAME
- 7 : AREA_NAME
- 8 : VEHICLE_CLASS
- 9 : STREET_NAME
- 10 : HELP_TEXT
- 11 : FLOATING_HELP_TEXT_1
- 12 : FLOATING_HELP_TEXT_2
- 13 : CASH_CHANGE
- 14 : RETICLE
- 15 : SUBTITLE_TEXT
- 16 : RADIO_STATIONS
- 17 : SAVING_GAME
- 18 : GAME_STREAM
- 19 : WEAPON_WHEEL
- 20 : WEAPON_WHEEL_STATS
- 21 : HUD_COMPONENTS
- 22 : HUD_WEAPONS

These integers also work for the [`SHOW_HUD_COMPONENT_THIS_FRAME`](#_0x0B4DF1FA60C0E664) native, but instead shows the HUD Component.



pub fn hide_hud_component_this_frame_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6806C51AD12B83B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6806C51AD12B83B8u64;
        
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
pub fn hide_hud_component_this_frame_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6806C51AD12B83B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6806C51AD12B83B8u64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ```
UI::_817B86108EB94E51(1, &g_189F36._f10CD1[0/*16*/], &g_189F36._f10CD1[1/*16*/], &g_189F36._f10CD1[2/*16*/], &g_189F36._f10CD1[3/*16*/], &g_189F36._f10CD1[4/*16*/], &g_189F36._f10CD1[5/*16*/], &g_189F36._f10CD1[6/*16*/], &g_189F36._f10CD1[7/*16*/]);  
```



pub fn _0x817b86108eb94e51_safe(
        
        
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
        let hash = 0x817B86108EB94E51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x817B86108EB94E51u64;
        
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
pub fn _0x817b86108eb94e51_raw(
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
        let hash = 0x817B86108EB94E51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x817B86108EB94E51u64;

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

/// ## Parameters
*



pub fn flag_player_context_in_tournament_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEF214315D276FD1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEF214315D276FD1u64;
        
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
pub fn flag_player_context_in_tournament_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEF214315D276FD1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEF214315D276FD1u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Sets whether or not the specified blip should only be displayed when nearby, or on the minimap.



pub fn set_blip_as_short_range_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE8BE4FE60E27B72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE8BE4FE60E27B72u64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_as_short_range_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE8BE4FE60E27B72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE8BE4FE60E27B72u64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_blip_secondary_colour_safe(
        
        
            blip: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14892474891E09EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14892474891E09EBu64;
        
        let result = invoke_raw!(
            hash,
                blip
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_blip_secondary_colour_raw(
        blip: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14892474891E09EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14892474891E09EBu64;

        invoke_raw_typed!(
            hash,
                blip
        )
    }
}

/// ## Parameters
*



pub fn flash_wanted_display_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA18AFB39081B6A1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA18AFB39081B6A1Fu64;
        
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
pub fn flash_wanted_display_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA18AFB39081B6A1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA18AFB39081B6A1Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Added Parameter 9: int hudColour
NativeDB Introduced: v1290
```

Sets the 'data' for a cone for a blip

See [here](https://docs.fivem.net/docs/game-references/hud-colors/) for the hud colours for the cone

Name in scripts: `SETUP_FAKE_CONE_DATA()`

For people who don't know how to convert degrees to radians and the other way around:

```
degrees = radians * 180 / pi
radians = degrees * pi / 180
```



pub fn _0xf83d0febe75e62c9_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF83D0FEBE75E62C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF83D0FEBE75E62C9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf83d0febe75e62c9_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF83D0FEBE75E62C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF83D0FEBE75E62C9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xd1942374085c8469_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1942374085C8469u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1942374085C8469u64;
        
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
pub fn _0xd1942374085c8469_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1942374085C8469u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1942374085C8469u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// THEFEED_SPS_EXTEND_WIDESCREEN_ON native function



pub fn thefeed_sps_extend_widescreen_on_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4438C0564490E63u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4438C0564490E63u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_sps_extend_widescreen_on_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4438C0564490E63u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4438C0564490E63u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// If mouse is hovering on a slot, it returns uniqueid of that slot, else it returns -1.



pub fn _pause_menu_get_unique_id_of_mouse_hovered_slot_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13C4B962653A5280u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13C4B962653A5280u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _pause_menu_get_unique_id_of_mouse_hovered_slot_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13C4B962653A5280u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13C4B962653A5280u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
You can only use text entries. No custom text.  
C# Example :  
Function.Call(Hash._SET_WARNING_MESSAGE_2, "HUD_QUIT", "HUD_CGIGNORE", 2, "HUD_CGINVITE", 0, -1, 0, 0, 1);  
you can recreate this easily with scaleforms



pub fn set_warning_message_with_header_safe(
        
        
            titleMsg: 
        , 
        
        
            entryLine1: 
        , 
        
        
            flags: 
        , 
        
        
            promptMsg: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            background: 
        , 
        
        
            p7: 
        , 
        
        
            showBg: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC38CC1E35B6A5D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC38CC1E35B6A5D7u64;
        
        let result = invoke_raw!(
            hash,
                titleMsg, 
                entryLine1, 
                flags, 
                promptMsg, 
                p4, 
                p5, 
                background, 
                p7, 
                showBg
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_warning_message_with_header_raw(
        titleMsg: , 
        entryLine1: , 
        flags: , 
        promptMsg: , 
        p4: , 
        p5: , 
        background: , 
        p7: , 
        showBg: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC38CC1E35B6A5D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC38CC1E35B6A5D7u64;

        invoke_raw_typed!(
            hash,
                titleMsg, 
                entryLine1, 
                flags, 
                promptMsg, 
                p4, 
                p5, 
                background, 
                p7, 
                showBg
        )
    }
}

/// ```
Appears to return whether the player is using the pause menu store. Can't be sure though.  
```



pub fn _0x2f057596f2bd0061_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F057596F2BD0061u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F057596F2BD0061u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x2f057596f2bd0061_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F057596F2BD0061u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F057596F2BD0061u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Makes a blip go small when off the minimap.
SET_BLIP_AS_*
```



pub fn _set_blip_shrink_safe(
        
        
            blip: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B6D467DAB714E8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B6D467DAB714E8Du64;
        
        let result = invoke_raw!(
            hash,
                blip, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_blip_shrink_raw(
        blip: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B6D467DAB714E8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B6D467DAB714E8Du64;

        invoke_raw_typed!(
            hash,
                blip, 
                toggle
        )
    }
}

/// THEFEED_PAUSE native function



pub fn thefeed_pause_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDB423997FA30340u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDB423997FA30340u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn thefeed_pause_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDB423997FA30340u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDB423997FA30340u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn clear_floating_help_safe(
        
        
            hudIndex: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50085246ABD3FEFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50085246ABD3FEFAu64;
        
        let result = invoke_raw!(
            hash,
                hudIndex, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_floating_help_raw(
        hudIndex: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50085246ABD3FEFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50085246ABD3FEFAu64;

        invoke_raw_typed!(
            hash,
                hudIndex, 
                p1
        )
    }
}

/// SET_TEXT_OUTLINE native function



pub fn set_text_outline_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2513DFB0FB8400FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2513DFB0FB8400FEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_text_outline_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2513DFB0FB8400FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2513DFB0FB8400FEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
zoomLevel ranges from 0 to 200  
```



pub fn set_radar_zoom_safe(
        
        
            zoomLevel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x096EF57A0C999BBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x096EF57A0C999BBAu64;
        
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
pub fn set_radar_zoom_raw(
        zoomLevel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x096EF57A0C999BBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x096EF57A0C999BBAu64;

        invoke_raw_typed!(
            hash,
                zoomLevel
        )
    }
}

/// _SHOW_SIGNIN_UI native function



pub fn _show_signin_ui_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60E892BA4F5BDCA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60E892BA4F5BDCA4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _show_signin_ui_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60E892BA4F5BDCA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60E892BA4F5BDCA4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_waypoint_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DD1F58F493F1DA5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DD1F58F493F1DA5u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_waypoint_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DD1F58F493F1DA5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DD1F58F493F1DA5u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Manually sets the player health value for a gamer tag, using the maximum health to calculate what percentage of the bar should be filled.
Has no effect unless [_SET_MP_GAMER_TAG_DISABLE_PLAYER_HEALTH_SYNC](#_0xD29EC58C2F6B5014) has been called prior to disable synchronisation with the attached ped.



pub fn _set_mp_gamer_tag_override_player_health_safe(
        
        
            gamerTagId: 
        , 
        
        
            health: 
        , 
        
        
            maximumHealth: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1563FE35E9928E67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1563FE35E9928E67u64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId, 
                health, 
                maximumHealth
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_mp_gamer_tag_override_player_health_raw(
        gamerTagId: , 
        health: , 
        maximumHealth: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1563FE35E9928E67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1563FE35E9928E67u64;

        invoke_raw_typed!(
            hash,
                gamerTagId, 
                health, 
                maximumHealth
        )
    }
}

/// ```c
enum eMpGamerTagComponent
{
	MP_TAG_GAMER_NAME = 0,
	MP_TAG_CREW_TAG = 1,
	MP_TAG_HEALTH_ARMOUR = 2,
	MP_TAG_BIG_TEXT = 3,
	MP_TAG_AUDIO_ICON = 4,
	MP_TAG_USING_MENU = 5,
	MP_TAG_PASSIVE_MODE = 6,
	MP_TAG_WANTED_STARS = 7,
	MP_TAG_DRIVER = 8,
	MP_TAG_CO_DRIVER = 9,
	MP_TAG_TAGGED = 10,
	MP_TAG_GAMER_NAME_NEARBY = 11,
	MP_TAG_ARROW = 12,
	MP_TAG_PACKAGES = 13,
	MP_TAG_INV_IF_PED_FOLLOWING = 14,
	MP_TAG_RANK_TEXT = 15,
	MP_TAG_TYPING = 16,
	MP_TAG_BAG_LARGE = 17,
	MP_TAG_ARROW = 18,
	MP_TAG_GANG_CEO = 19,
	MP_TAG_GANG_BIKER = 20,
	MP_TAG_BIKER_ARROW = 21,
	MP_TAG_MC_ROLE_PRESIDENT = 22,
	MP_TAG_MC_ROLE_VICE_PRESIDENT = 23,
	MP_TAG_MC_ROLE_ROAD_CAPTAIN = 24,
	MP_TAG_MC_ROLE_SARGEANT = 25,
	MP_TAG_MC_ROLE_ENFORCER = 26,
	MP_TAG_MC_ROLE_PROSPECT = 27,
	MP_TAG_TRANSMITTER = 28,
	MP_TAG_BOMB = 29
};
```

```
NativeDB Added Parameter 4: Any p3
```



pub fn set_mp_gamer_tag_visibility_safe(
        
        
            gamerTagId: 
        , 
        
        
            component: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63BB75ABEDC1F6A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63BB75ABEDC1F6A0u64;
        
        let result = invoke_raw!(
            hash,
                gamerTagId, 
                component, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mp_gamer_tag_visibility_raw(
        gamerTagId: , 
        component: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63BB75ABEDC1F6A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63BB75ABEDC1F6A0u64;

        invoke_raw_typed!(
            hash,
                gamerTagId, 
                component, 
                toggle
        )
    }
}

/// ```
BEGIN_TEXT_COMMAND_*
Example:
_BEGIN_TEXT_COMMAND_GET_WIDTH("NUMBER");
ADD_TEXT_COMPONENT_FLOAT(69.420f, 2);
float width = _END_TEXT_COMMAND_GET_WIDTH(1);
```



pub fn _begin_text_command_get_width_safe(
        
        
            text: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54CE8AC98E120CABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54CE8AC98E120CABu64;
        
        let result = invoke_raw!(
            hash,
                text
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _begin_text_command_get_width_raw(
        text: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54CE8AC98E120CABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54CE8AC98E120CABu64;

        invoke_raw_typed!(
            hash,
                text
        )
    }
}

/// ## Parameters
*



pub fn end_text_command_thefeed_post_unlock_safe(
        
        
            chTitle: 
        , 
        
        
            iconType: 
        , 
        
        
            chSubtitle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33EE12743CCD6343u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33EE12743CCD6343u64;
        
        let result = invoke_raw!(
            hash,
                chTitle, 
                iconType, 
                chSubtitle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn end_text_command_thefeed_post_unlock_raw(
        chTitle: , 
        iconType: , 
        chSubtitle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33EE12743CCD6343u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33EE12743CCD6343u64;

        invoke_raw_typed!(
            hash,
                chTitle, 
                iconType, 
                chSubtitle
        )
    }
}

