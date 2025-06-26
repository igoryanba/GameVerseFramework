//! PAD native functions
//! 
//! Functions for the pad category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn get_control_group_instructional_button_safe(
        
        
            padIndex: 
        , 
        
        
            controlGroup: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80C2FD58D720C801u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80C2FD58D720C801u64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                controlGroup, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_control_group_instructional_button_raw(
        padIndex: , 
        controlGroup: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80C2FD58D720C801u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80C2FD58D720C801u64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                controlGroup, 
                p2
        )
    }
}

/// ```
Returns profile setting 225.
```



pub fn get_is_using_alternate_driveby_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F70731BACCFBB96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F70731BACCFBB96u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_is_using_alternate_driveby_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F70731BACCFBB96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F70731BACCFBB96u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x5b73c77d9eb66e24_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B73C77D9EB66E24u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B73C77D9EB66E24u64;
        
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
pub fn _0x5b73c77d9eb66e24_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B73C77D9EB66E24u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B73C77D9EB66E24u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Hardcoded to return false.



pub fn _0x23f09eadc01449d6_safe(
        
        
            padIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23F09EADC01449D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23F09EADC01449D6u64;
        
        let result = invoke_raw!(
            hash,
                padIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x23f09eadc01449d6_raw(
        padIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23F09EADC01449D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23F09EADC01449D6u64;

        invoke_raw_typed!(
            hash,
                padIndex
        )
    }
}

/// ## Parameters
*



pub fn get_control_normal_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC3C9B8D5327B563u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC3C9B8D5327B563u64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_control_normal_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC3C9B8D5327B563u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC3C9B8D5327B563u64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// ## Parameters
*



pub fn _0xa0cefcea390aab9b_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0CEFCEA390AAB9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0CEFCEA390AAB9Bu64;
        
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
pub fn _0xa0cefcea390aab9b_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0CEFCEA390AAB9Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0CEFCEA390AAB9Bu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _is_using_keyboard_safe(
        
        
            padIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA571D46727E2B718u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA571D46727E2B718u64;
        
        let result = invoke_raw!(
            hash,
                padIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_using_keyboard_raw(
        padIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA571D46727E2B718u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA571D46727E2B718u64;

        invoke_raw_typed!(
            hash,
                padIndex
        )
    }
}

/// ## Parameters
*



pub fn _0xf239400e16c23e08_safe(
        
        
            padIndex: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF239400E16C23E08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF239400E16C23E08u64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf239400e16c23e08_raw(
        padIndex: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF239400E16C23E08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF239400E16C23E08u64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_input_exclusive_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDE476E5EE29EDB1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDE476E5EE29EDB1u64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_input_exclusive_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDE476E5EE29EDB1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDE476E5EE29EDB1u64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// ```
formerly called _GET_CONTROL_ACTION_NAME incorrectly  
p2 appears to always be true.  
p2 is unused variable in function.  
EG:  
GET_CONTROL_INSTRUCTIONAL_BUTTON (2, 201, 1) /*INPUT_FRONTEND_ACCEPT (e.g. Enter button)*/  
GET_CONTROL_INSTRUCTIONAL_BUTTON (2, 202, 1) /*INPUT_FRONTEND_CANCEL (e.g. ESC button)*/  
GET_CONTROL_INSTRUCTIONAL_BUTTON (2, 51, 1) /*INPUT_CONTEXT (e.g. E button)*/  
gtaforums.com/topic/819070-c-draw-instructional-buttons-scaleform-movie/#entry1068197378  
0, 1 and 2 used in the scripts. 0 is by far the most common of them.  
```



pub fn get_control_instructional_button_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0499D7B09FC9B407u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0499D7B09FC9B407u64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_control_instructional_button_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0499D7B09FC9B407u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0499D7B09FC9B407u64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// [Control values and meaning](https://docs.fivem.net/docs/game-references/controls/#controls)

Example: `CONTROLS::DISABLE_CONTROL_ACTION(2, 19, true)` disables the switching UI from appearing both when using a keyboard and Xbox 360 controller. Needs to be executed each frame.
Control group 1 and 0 gives the same results as 2. Same results for all players.



pub fn disable_control_action_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        , 
        
        
            disable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE99B66D079CF6BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE99B66D079CF6BCu64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control, 
                disable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_control_action_raw(
        padIndex: , 
        control: , 
        disable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE99B66D079CF6BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE99B66D079CF6BCu64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control, 
                disable
        )
    }
}

/// ## Parameters
*



pub fn _set_control_light_effect_color_safe(
        
        
            padIndex: 
        , 
        
        
            red: 
        , 
        
        
            green: 
        , 
        
        
            blue: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8290252FFF36ACB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8290252FFF36ACB5u64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
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
pub fn _set_control_light_effect_color_raw(
        padIndex: , 
        red: , 
        green: , 
        blue: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8290252FFF36ACB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8290252FFF36ACB5u64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                red, 
                green, 
                blue
        )
    }
}

/// ## Parameters
*



pub fn _0x6cd79468a1e595c6_safe(
        
        
            padIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CD79468A1E595C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CD79468A1E595C6u64;
        
        let result = invoke_raw!(
            hash,
                padIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6cd79468a1e595c6_raw(
        padIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CD79468A1E595C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CD79468A1E595C6u64;

        invoke_raw_typed!(
            hash,
                padIndex
        )
    }
}

/// ## Parameters
*



pub fn set_playerpad_shakes_when_controller_disabled_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x798FDEB5B1575088u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x798FDEB5B1575088u64;
        
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
pub fn set_playerpad_shakes_when_controller_disabled_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x798FDEB5B1575088u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x798FDEB5B1575088u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Returns whether a control is currently _not_ pressed.



pub fn is_control_released_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x648EE3E7F38877DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x648EE3E7F38877DDu64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_control_released_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x648EE3E7F38877DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x648EE3E7F38877DDu64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// ```
Used with IS_LOOK_INVERTED() and negates its affect.
--
Not sure how the person above got that description, but here's an actual example:
if (PAD::_GET_LAST_INPUT_METHOD(2)) {
    if (a_5) {
        if (PAD::IS_LOOK_INVERTED()) {
            a_3 *= -1;
        }
        if (PAD::_E1615EC03B3BB4FD()) {
            a_3 *= -1;
        }
    }
}
```



pub fn _0xe1615ec03b3bb4fd_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1615EC03B3BB4FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1615EC03B3BB4FDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xe1615ec03b3bb4fd_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1615EC03B3BB4FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1615EC03B3BB4FDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_disabled_control_normal_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11E65974A982637Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11E65974A982637Cu64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_disabled_control_normal_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11E65974A982637Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11E65974A982637Cu64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// ## Parameters
*



pub fn is_control_enabled_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CEA6BFDF248E5D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CEA6BFDF248E5D9u64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_control_enabled_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CEA6BFDF248E5D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CEA6BFDF248E5D9u64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// ## Parameters
*



pub fn is_disabled_control_just_released_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x305C8DCD79DA8B0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x305C8DCD79DA8B0Fu64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_disabled_control_just_released_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x305C8DCD79DA8B0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x305C8DCD79DA8B0Fu64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// ```c
enum ePadType {
  PLAYER_CONTROL = 0,
  CAMERA_CONTROL = 1,
  FRONTEND_CONTRO = 2
};
```



pub fn enable_all_control_actions_safe(
        
        
            padIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5FFE9B05F199DE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5FFE9B05F199DE7u64;
        
        let result = invoke_raw!(
            hash,
                padIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn enable_all_control_actions_raw(
        padIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5FFE9B05F199DE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5FFE9B05F199DE7u64;

        invoke_raw_typed!(
            hash,
                padIndex
        )
    }
}

/// ```
S*
```



pub fn _reset_input_mapping_scheme_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x643ED62D5EA3BEBDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x643ED62D5EA3BEBDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _reset_input_mapping_scheme_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x643ED62D5EA3BEBDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x643ED62D5EA3BEBDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn disable_all_control_actions_safe(
        
        
            padIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F4B6931816E599Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F4B6931816E599Bu64;
        
        let result = invoke_raw!(
            hash,
                padIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_all_control_actions_raw(
        padIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F4B6931816E599Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F4B6931816E599Bu64;

        invoke_raw_typed!(
            hash,
                padIndex
        )
    }
}

/// ## Parameters
*



pub fn is_disabled_control_pressed_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2587F8CBBD87B1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2587F8CBBD87B1Du64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_disabled_control_pressed_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2587F8CBBD87B1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2587F8CBBD87B1Du64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// ## Parameters
*



pub fn stop_pad_shake_safe(
        
        
            padIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38C16A305E8CDC8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38C16A305E8CDC8Du64;
        
        let result = invoke_raw!(
            hash,
                padIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_pad_shake_raw(
        padIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38C16A305E8CDC8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38C16A305E8CDC8Du64;

        invoke_raw_typed!(
            hash,
                padIndex
        )
    }
}

/// Returns whether a control was newly released since the last check.



pub fn is_control_just_released_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50F940259D3841E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50F940259D3841E6u64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_control_just_released_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50F940259D3841E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50F940259D3841E6u64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// Returns whether a control was newly pressed since the last check.



pub fn is_control_just_pressed_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x580417101DDB492Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x580417101DDB492Fu64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_control_just_pressed_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x580417101DDB492Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x580417101DDB492Fu64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// ## Parameters
*



pub fn _set_cursor_location_safe(
        
        
            x: 
        , 
        
        
            y: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC695459D4D0E219u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC695459D4D0E219u64;
        
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
pub fn _set_cursor_location_raw(
        x: , 
        y: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC695459D4D0E219u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC695459D4D0E219u64;

        invoke_raw_typed!(
            hash,
                x, 
                y
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x14d29bb12d47f68c_safe(
        
        
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
        let hash = 0x14D29BB12D47F68Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14D29BB12D47F68Cu64;
        
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
pub fn _0x14d29bb12d47f68c_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14D29BB12D47F68Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14D29BB12D47F68Cu64;

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
Seems to return values between -1 and 1 for controls like gas and steering.
```



pub fn get_control_unbound_normal_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B84D09CEC5209C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B84D09CEC5209C5u64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_control_unbound_normal_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B84D09CEC5209C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B84D09CEC5209C5u64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// ```
Used in carsteal3 script with p0 = "Carsteal4_spycar".
S*
```



pub fn _switch_to_input_mapping_scheme_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D42B92563939375u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D42B92563939375u64;
        
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
pub fn _switch_to_input_mapping_scheme_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D42B92563939375u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D42B92563939375u64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// Returns whether a control is currently pressed.



pub fn is_control_pressed_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3A21BCD95725A4Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3A21BCD95725A4Au64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_control_pressed_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3A21BCD95725A4Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3A21BCD95725A4Au64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// ## Parameters
*



pub fn get_disabled_control_unbound_normal_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F8A26A890FD62FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F8A26A890FD62FBu64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_disabled_control_unbound_normal_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F8A26A890FD62FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F8A26A890FD62FBu64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// This is for simulating player input.



pub fn _set_control_normal_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        , 
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8A25867FBA3B05Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8A25867FBA3B05Eu64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control, 
                amount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_control_normal_raw(
        padIndex: , 
        control: , 
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8A25867FBA3B05Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8A25867FBA3B05Eu64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control, 
                amount
        )
    }
}

/// ## Parameters
*



pub fn _is_using_keyboard_2_safe(
        
        
            padIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13337B38DB572509u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13337B38DB572509u64;
        
        let result = invoke_raw!(
            hash,
                padIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_using_keyboard_2_raw(
        padIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13337B38DB572509u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13337B38DB572509u64;

        invoke_raw_typed!(
            hash,
                padIndex
        )
    }
}

/// ```
NativeDB Introduced: v1365
```



pub fn _0x25aaa32bdc98f2a3_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25AAA32BDC98F2A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25AAA32BDC98F2A3u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x25aaa32bdc98f2a3_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25AAA32BDC98F2A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25AAA32BDC98F2A3u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Returns the local player's targeting mode. See [`SET_PLAYER_TARGETING_MODE`](#_0xB1906895227793F3).



pub fn get_local_player_aim_state_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB41AFBBBC0A0287u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB41AFBBBC0A0287u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_local_player_aim_state_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB41AFBBBC0A0287u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB41AFBBBC0A0287u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
control values and meaning: https://github.com/scripthookvdotnet/scripthookvdotnet/blob/47f8bb02a50e27d4bb982f43a78091feac59b21c/source/scripting_v3/GTA/Control.cs
and  
https://docs.fivem.net/game-references/controls/
0, 1 and 2 used in the scripts.  
Control values from the decompiled scripts:   
0,1,2,3,4,5,6,8,9,10,11,14,15,16,17,19,21,22,24,25,26,30,31,32,33,34,35,36,  
37,44,46,47,59,60,65,68,69,70,71,72,73,74,75,76,79,80,81,82,86,95,98,99,100  
,101,114,140,141,143,172,173,174,175,176,177,178,179,180,181,187,188,189,19  
0,195,196,197,198,199,201,202,203,204,205,206,207,208,209,210,217,218,219,2  
20,221,225,228,229,230,231,234,235,236,237,238,239,240,241,242,245,246,257,  
261,262,263,264,286,287,288,289,337,338,339,340,341,342,343  
INPUTGROUP_MOVE  
INPUTGROUP_LOOK  
INPUTGROUP_WHEEL  
INPUTGROUP_CELLPHONE_NAVIGATE  
INPUTGROUP_CELLPHONE_NAVIGATE_UD  
INPUTGROUP_CELLPHONE_NAVIGATE_LR  
INPUTGROUP_FRONTEND_DPAD_ALL  
INPUTGROUP_FRONTEND_DPAD_UD  
INPUTGROUP_FRONTEND_DPAD_LR  
INPUTGROUP_FRONTEND_LSTICK_ALL  
INPUTGROUP_FRONTEND_RSTICK_ALL  
INPUTGROUP_FRONTEND_GENERIC_UD  
INPUTGROUP_FRONTEND_GENERIC_LR  
INPUTGROUP_FRONTEND_GENERIC_ALL  
INPUTGROUP_FRONTEND_BUMPERS  
INPUTGROUP_FRONTEND_TRIGGERS  
INPUTGROUP_FRONTEND_STICKS  
INPUTGROUP_SCRIPT_DPAD_ALL  
INPUTGROUP_SCRIPT_DPAD_UD  
INPUTGROUP_SCRIPT_DPAD_LR  
INPUTGROUP_SCRIPT_LSTICK_ALL  
INPUTGROUP_SCRIPT_RSTICK_ALL  
INPUTGROUP_SCRIPT_BUMPERS  
INPUTGROUP_SCRIPT_TRIGGERS  
INPUTGROUP_WEAPON_WHEEL_CYCLE  
INPUTGROUP_FLY  
INPUTGROUP_SUB  
INPUTGROUP_VEH_MOVE_ALL  
INPUTGROUP_CURSOR  
INPUTGROUP_CURSOR_SCROLL  
INPUTGROUP_SNIPER_ZOOM_SECONDARY  
INPUTGROUP_VEH_HYDRAULICS_CONTROL  
Took those in IDA Pro.Not sure in which order they go  
```



pub fn enable_control_action_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        , 
        
        
            enable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x351220255D64C155u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x351220255D64C155u64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control, 
                enable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn enable_control_action_raw(
        padIndex: , 
        control: , 
        enable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x351220255D64C155u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x351220255D64C155u64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control, 
                enable
        )
    }
}

/// ## Parameters
*



pub fn is_disabled_control_just_pressed_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x91AEF906BCA88877u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x91AEF906BCA88877u64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_disabled_control_just_pressed_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x91AEF906BCA88877u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x91AEF906BCA88877u64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// ## Parameters
*



pub fn get_control_value_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD95E79E8686D2C27u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD95E79E8686D2C27u64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_control_value_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD95E79E8686D2C27u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD95E79E8686D2C27u64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

/// ```
Returns profile setting 17.
```



pub fn get_allow_movement_while_zoomed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC859E2374407556u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC859E2374407556u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_allow_movement_while_zoomed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC859E2374407556u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC859E2374407556u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xcb0360efefb2580d_safe(
        
        
            padIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB0360EFEFB2580Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB0360EFEFB2580Du64;
        
        let result = invoke_raw!(
            hash,
                padIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xcb0360efefb2580d_raw(
        padIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB0360EFEFB2580Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB0360EFEFB2580Du64;

        invoke_raw_typed!(
            hash,
                padIndex
        )
    }
}

/// ## Return value



pub fn is_look_inverted_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77B612531280010Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77B612531280010Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_look_inverted_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77B612531280010Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77B612531280010Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
p0 always seems to be 0  
duration in milliseconds   
frequency should range from about 10 (slow vibration) to 255 (very fast)  
example:  
SET_PAD_SHAKE(0, 100, 200);  
```



pub fn set_pad_shake_safe(
        
        
            padIndex: 
        , 
        
        
            duration: 
        , 
        
        
            frequency: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48B3886C1358D0D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48B3886C1358D0D5u64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                duration, 
                frequency
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_pad_shake_raw(
        padIndex: , 
        duration: , 
        frequency: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48B3886C1358D0D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48B3886C1358D0D5u64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                duration, 
                frequency
        )
    }
}

/// ```
Same as 0x3D42B92563939375

S*
```



pub fn _switch_to_input_mapping_scheme_2_safe(
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4683149ED1DDE7A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4683149ED1DDE7A1u64;
        
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
pub fn _switch_to_input_mapping_scheme_2_raw(
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4683149ED1DDE7A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4683149ED1DDE7A1u64;

        invoke_raw_typed!(
            hash,
                name
        )
    }
}

/// ```
The number of milliseconds since last padIndex registered pressed
```



pub fn _get_time_since_last_input_safe(
        
        
            padIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7D22F5592AED8BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7D22F5592AED8BAu64;
        
        let result = invoke_raw!(
            hash,
                padIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_time_since_last_input_raw(
        padIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7D22F5592AED8BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7D22F5592AED8BAu64;

        invoke_raw_typed!(
            hash,
                padIndex
        )
    }
}

/// ## Parameters
*



pub fn _disable_input_group_safe(
        
        
            padIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F4724035FDCA1DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F4724035FDCA1DDu64;
        
        let result = invoke_raw!(
            hash,
                padIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _disable_input_group_raw(
        padIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F4724035FDCA1DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F4724035FDCA1DDu64;

        invoke_raw_typed!(
            hash,
                padIndex
        )
    }
}

/// Same behavior as [`GET_LOCAL_PLAYER_AIM_STATE`](#_0xBB41AFBBBC0A0287) but will also return if player using a keyboard.



pub fn get_local_player_gamepad_aim_state_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59B9A7AF4C95133Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59B9A7AF4C95133Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_local_player_gamepad_aim_state_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59B9A7AF4C95133Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59B9A7AF4C95133Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _is_disabled_control_released_safe(
        
        
            padIndex: 
        , 
        
        
            control: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB6C4072E9A32E92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB6C4072E9A32E92u64;
        
        let result = invoke_raw!(
            hash,
                padIndex, 
                control
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_disabled_control_released_raw(
        padIndex: , 
        control: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB6C4072E9A32E92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB6C4072E9A32E92u64;

        invoke_raw_typed!(
            hash,
                padIndex, 
                control
        )
    }
}

