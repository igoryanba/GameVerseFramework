//! PED native functions
//! 
//! Functions for the ped category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn is_ped_model_safe(
        
        
            ped: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9D55B1A358A5BF7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9D55B1A358A5BF7u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_model_raw(
        ped: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9D55B1A358A5BF7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9D55B1A358A5BF7u64;

        invoke_raw_typed!(
            hash,
                ped, 
                modelHash
        )
    }
}

/// ## Return value



pub fn is_pedheadshot_img_upload_available_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBB376779A760AA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBB376779A760AA8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_pedheadshot_img_upload_available_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBB376779A760AA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBB376779A760AA8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
returns whether or not a ped is visible within your FOV, not this check auto's to false after a certain distance.  
Target needs to be tracked.. won't work otherwise.  
```



pub fn is_tracked_ped_visible_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x91C8E617F64188ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x91C8E617F64188ACu64;
        
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
pub fn is_tracked_ped_visible_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x91C8E617F64188ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x91C8E617F64188ACu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0xb282749d5e028163_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB282749D5E028163u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB282749D5E028163u64;
        
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
pub fn _0xb282749d5e028163_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB282749D5E028163u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB282749D5E028163u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_ped_defensive_area_direction_safe(
        
        
            ped: 
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
        let hash = 0x413C6C763A4AFFADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x413C6C763A4AFFADu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn set_ped_defensive_area_direction_raw(
        ped: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x413C6C763A4AFFADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x413C6C763A4AFFADu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn is_ped_getting_into_a_vehicle_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB062B2B5722478Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB062B2B5722478Eu64;
        
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
pub fn is_ped_getting_into_a_vehicle_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB062B2B5722478Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB062B2B5722478Eu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
SET_PED_ALLOW*
toggle was always false except in one instance (b678).
The one time this is set to true seems to do with when you fail the mission.
```



pub fn _0xf2bebcdfafdaa19e_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2BEBCDFAFDAA19Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2BEBCDFAFDAA19Eu64;
        
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
pub fn _0xf2bebcdfafdaa19e_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2BEBCDFAFDAA19Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2BEBCDFAFDAA19Eu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Ped no longer takes critical damage modifiers if set to FALSE.

Example: Headshotting a player no longer one shots them. Instead they will take the same damage as a torso shot.



pub fn set_ped_suffers_critical_hits_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBD76F2359F190ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBD76F2359F190ACu64;
        
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
pub fn set_ped_suffers_critical_hits_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBD76F2359F190ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBD76F2359F190ACu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
Function just returns 0  
void __fastcall ped__get_mount(NativeContext *a1)  
{  
  NativeContext *v1; // rbx@1  
  v1 = a1;  
  GetAddressOfPedFromScriptHandle(a1->Args->Arg1);  
  v1->Returns->Item1= 0;  
}  
```



pub fn get_mount_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7E11B8DCBED1058u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7E11B8DCBED1058u64;
        
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
pub fn get_mount_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7E11B8DCBED1058u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7E11B8DCBED1058u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_should_play_immediate_scenario_exit_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1C03A5352243A30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1C03A5352243A30u64;
        
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
pub fn set_ped_should_play_immediate_scenario_exit_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1C03A5352243A30u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1C03A5352243A30u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_visual_field_max_angle_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70793BDCA1E854D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70793BDCA1E854D4u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_visual_field_max_angle_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70793BDCA1E854D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70793BDCA1E854D4u64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// ## Parameters
*



pub fn _get_ped_current_movement_speed_safe(
        
        
            ped: 
        , 
        
        
            speedX: 
        , 
        
        
            speedY: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF60165E1D2C5370Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF60165E1D2C5370Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                speedX, 
                speedY
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_ped_current_movement_speed_raw(
        ped: , 
        speedX: , 
        speedY: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF60165E1D2C5370Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF60165E1D2C5370Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                speedX, 
                speedY
        )
    }
}

/// ## Parameters
*



pub fn add_ped_decoration_from_hashes_in_corona_safe(
        
        
            ped: 
        , 
        
        
            collection: 
        , 
        
        
            overlay: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5619BFA07CFD7833u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5619BFA07CFD7833u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                collection, 
                overlay
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_ped_decoration_from_hashes_in_corona_raw(
        ped: , 
        collection: , 
        overlay: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5619BFA07CFD7833u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5619BFA07CFD7833u64;

        invoke_raw_typed!(
            hash,
                ped, 
                collection, 
                overlay
        )
    }
}

/// ```
Only called once in the scripts:
if (sub_1abd() && (!PED::_A3F3564A5B3646C0(l_8C))) {
    if (sub_52e3("RESNA_CELLR", 0)) {
        PED::SET_PED_CAN_PLAY_GESTURE_ANIMS(l_8C, 1);
        PED::SET_PED_CAN_PLAY_AMBIENT_ANIMS(l_8C, 1);
        PED::SET_PED_CAN_PLAY_VISEME_ANIMS(l_8C, 1, 0);
        l_184 += 1;
    }
}
Checks something related to the mobile phone task.
IS_*
```



pub fn _0xa3f3564a5b3646c0_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3F3564A5B3646C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3F3564A5B3646C0u64;
        
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
pub fn _0xa3f3564a5b3646c0_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3F3564A5B3646C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3F3564A5B3646C0u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_taking_off_helmet_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14590DDBEDB1EC85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14590DDBEDB1EC85u64;
        
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
pub fn is_ped_taking_off_helmet_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14590DDBEDB1EC85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14590DDBEDB1EC85u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _set_ped_can_play_injured_anims_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33A60D8BDD6E508Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33A60D8BDD6E508Cu64;
        
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
pub fn _set_ped_can_play_injured_anims_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33A60D8BDD6E508Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33A60D8BDD6E508Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
NativeDB Added Parameter 5: Any p4
```



pub fn set_ped_vehicle_forced_seat_usage_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            seatIndex: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x952F06BEECD775CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x952F06BEECD775CCu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                seatIndex, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_vehicle_forced_seat_usage_raw(
        ped: , 
        vehicle: , 
        seatIndex: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x952F06BEECD775CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x952F06BEECD775CCu64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                seatIndex, 
                flags
        )
    }
}

/// ## Parameters
*



pub fn get_player_ped_is_following_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A3975DEA89F9A17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A3975DEA89F9A17u64;
        
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
pub fn get_player_ped_is_following_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A3975DEA89F9A17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A3975DEA89F9A17u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Checks if the specified unknown flag is set in the ped's model.  
The engine itself seems to exclusively check for flags 1 and 4 (Might be inlined code of the check that checks for other flags).  
Game scripts exclusively check for flags 1 and 4.  
```



pub fn _0x46b05bcae43856b0_safe(
        
        
            ped: 
        , 
        
        
            flag: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46B05BCAE43856B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46B05BCAE43856B0u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                flag
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x46b05bcae43856b0_raw(
        ped: , 
        flag: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46B05BCAE43856B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46B05BCAE43856B0u64;

        invoke_raw_typed!(
            hash,
                ped, 
                flag
        )
    }
}

/// ## Parameters
*



pub fn set_ped_steers_around_peds_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46F2193B3AD1D891u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46F2193B3AD1D891u64;
        
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
pub fn set_ped_steers_around_peds_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46F2193B3AD1D891u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46F2193B3AD1D891u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
Returns the Entity (Ped, Vehicle, or ?Object?) that killed the 'ped'  
Is best to check if the Ped is dead before asking for its killer.  
```



pub fn get_ped_source_of_death_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93C8B64DEB84728Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93C8B64DEB84728Cu64;
        
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
pub fn get_ped_source_of_death_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93C8B64DEB84728Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93C8B64DEB84728Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_bounds_orientation_safe(
        
        
            ped: 
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
        let hash = 0x4F5F651ACCC9C4CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F5F651ACCC9C4CFu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn set_ped_bounds_orientation_raw(
        ped: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F5F651ACCC9C4CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F5F651ACCC9C4CFu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5
        )
    }
}

/// ```
shootRate 0-1000  
```



pub fn set_ped_shoot_rate_safe(
        
        
            ped: 
        , 
        
        
            shootRate: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x614DA022990752DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x614DA022990752DCu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                shootRate
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_shoot_rate_raw(
        ped: , 
        shootRate: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x614DA022990752DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x614DA022990752DCu64;

        invoke_raw_typed!(
            hash,
                ped, 
                shootRate
        )
    }
}

/// ```
100 would equal attack  
less then 50ish would mean run away  
Only the values 0, 1 and 2 occur in the decompiled scripts. Most likely refers directly to the values also described in combatbehaviour.meta:  
0: CA_Poor  
1: CA_Average  
2: CA_Professional  
Tested this and got the same results as the first explanation here. Could not find any difference between 0, 1 and 2.  
```



pub fn set_ped_combat_ability_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7622C0D36B2FDA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7622C0D36B2FDA8u64;
        
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
pub fn set_ped_combat_ability_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7622C0D36B2FDA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7622C0D36B2FDA8u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_ped_into_vehicle_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            seatIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF75B0D629E1C063Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF75B0D629E1C063Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                seatIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_into_vehicle_raw(
        ped: , 
        vehicle: , 
        seatIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF75B0D629E1C063Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF75B0D629E1C063Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                seatIndex
        )
    }
}

/// ## Parameters
*



pub fn does_group_exist_safe(
        
        
            groupId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C6B0C22F9F40BBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C6B0C22F9F40BBEu64;
        
        let result = invoke_raw!(
            hash,
                groupId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_group_exist_raw(
        groupId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C6B0C22F9F40BBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C6B0C22F9F40BBEu64;

        invoke_raw_typed!(
            hash,
                groupId
        )
    }
}

/// ## Return value



pub fn spawnpoints_is_search_failed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF445DE8DA80A1792u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF445DE8DA80A1792u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn spawnpoints_is_search_failed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF445DE8DA80A1792u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF445DE8DA80A1792u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Clipsets:
"facials@gen_female@base"
"facials@gen_male@base"
"facials@p_m_zero@base"

Typically followed with [SET_FACIAL_IDLE_ANIM_OVERRIDE](#_0xFFC24B988B938B38):
"mood_drunk_1"
"mood_stressed_1"
"mood_happy_1"
"mood_talking_1"

```
NativeDB Introduced: v1493
```



pub fn _set_facial_clipset_override_safe(
        
        
            ped: 
        , 
        
        
            animDict: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5687C7F05B39E401u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5687C7F05B39E401u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                animDict
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_facial_clipset_override_raw(
        ped: , 
        animDict: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5687C7F05B39E401u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5687C7F05B39E401u64;

        invoke_raw_typed!(
            hash,
                ped, 
                animDict
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_play_gesture_anims_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAF20C5432058024u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAF20C5432058024u64;
        
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
pub fn set_ped_can_play_gesture_anims_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAF20C5432058024u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAF20C5432058024u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_be_targeted_when_injured_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x638C03B0F9878F57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x638C03B0F9878F57u64;
        
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
pub fn set_ped_can_be_targeted_when_injured_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x638C03B0F9878F57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x638C03B0F9878F57u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// Allows marine animals to survive outside of water (R* is using it for sharks).

```
NativeDB Introduced: v3407
```



pub fn _set_ped_survives_being_out_of_water_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x100CD221F572F6E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x100CD221F572F6E1u64;
        
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
pub fn _set_ped_survives_being_out_of_water_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x100CD221F572F6E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x100CD221F572F6E1u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_ped_max_health_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5F6378C4F3419D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5F6378C4F3419D3u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_max_health_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5F6378C4F3419D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5F6378C4F3419D3u64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// ## Parameters
*



pub fn _0x2f3c3d9f50681de4_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F3C3D9F50681DE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F3C3D9F50681DE4u64;
        
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
pub fn _0x2f3c3d9f50681de4_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F3C3D9F50681DE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F3C3D9F50681DE4u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn can_ped_see_hated_ped_safe(
        
        
            ped1: 
        , 
        
        
            ped2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CD5A433374D4CFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CD5A433374D4CFBu64;
        
        let result = invoke_raw!(
            hash,
                ped1, 
                ped2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_ped_see_hated_ped_raw(
        ped1: , 
        ped2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6CD5A433374D4CFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6CD5A433374D4CFBu64;

        invoke_raw_typed!(
            hash,
                ped1, 
                ped2
        )
    }
}

/// ## Parameters
*



pub fn _is_ped_helmet_unk_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9496CE47546DB2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9496CE47546DB2Cu64;
        
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
pub fn _is_ped_helmet_unk_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9496CE47546DB2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9496CE47546DB2Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_defensive_sphere_attached_to_vehicle_safe(
        
        
            ped: 
        , 
        
        
            target: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
        , 
        
        
            radius: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4723DB6E736CCFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4723DB6E736CCFFu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                target, 
                xOffset, 
                yOffset, 
                zOffset, 
                radius, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_defensive_sphere_attached_to_vehicle_raw(
        ped: , 
        target: , 
        xOffset: , 
        yOffset: , 
        zOffset: , 
        radius: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4723DB6E736CCFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4723DB6E736CCFFu64;

        invoke_raw_typed!(
            hash,
                ped, 
                target, 
                xOffset, 
                yOffset, 
                zOffset, 
                radius, 
                p6
        )
    }
}

/// Applies blood damage to a ped with specific parameters for zone, UV offsets, rotation, scale, and initial aging.

```
NativeDB Introduced: v323
```



pub fn apply_ped_blood_specific_safe(
        
        
            ped: 
        , 
        
        
            component: 
        , 
        
        
            u: 
        , 
        
        
            v: 
        , 
        
        
            rotation: 
        , 
        
        
            scale: 
        , 
        
        
            forcedFrame: 
        , 
        
        
            preAge: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF0D582CBF2D9B0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF0D582CBF2D9B0Fu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                component, 
                u, 
                v, 
                rotation, 
                scale, 
                forcedFrame, 
                preAge
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn apply_ped_blood_specific_raw(
        ped: , 
        component: , 
        u: , 
        v: , 
        rotation: , 
        scale: , 
        forcedFrame: , 
        preAge: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF0D582CBF2D9B0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF0D582CBF2D9B0Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                component, 
                u, 
                v, 
                rotation, 
                scale, 
                forcedFrame, 
                preAge
        )
    }
}

/// [Animations list](https://alexguirre.github.io/animations-list/)



pub fn get_anim_initial_offset_position_safe(
        
        
            animDict: 
        , 
        
        
            animName: 
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
        
        
            p8: 
        , 
        
        
            p9: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE22B26DD764C040u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE22B26DD764C040u64;
        
        let result = invoke_raw!(
            hash,
                animDict, 
                animName, 
                x, 
                y, 
                z, 
                xRot, 
                yRot, 
                zRot, 
                p8, 
                p9
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_anim_initial_offset_position_raw(
        animDict: , 
        animName: , 
        x: , 
        y: , 
        z: , 
        xRot: , 
        yRot: , 
        zRot: , 
        p8: , 
        p9: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE22B26DD764C040u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE22B26DD764C040u64;

        invoke_raw_typed!(
            hash,
                animDict, 
                animName, 
                x, 
                y, 
                z, 
                xRot, 
                yRot, 
                zRot, 
                p8, 
                p9
        )
    }
}

/// This native is used to set prop variation on a ped. Components, drawables and textures IDs are related to the ped model.



pub fn set_ped_prop_index_safe(
        
        
            ped: 
        , 
        
        
            componentId: 
        , 
        
        
            drawableId: 
        , 
        
        
            textureId: 
        , 
        
        
            attach: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93376B65A266EB5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93376B65A266EB5Fu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                componentId, 
                drawableId, 
                textureId, 
                attach
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_prop_index_raw(
        ped: , 
        componentId: , 
        drawableId: , 
        textureId: , 
        attach: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93376B65A266EB5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93376B65A266EB5Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                componentId, 
                drawableId, 
                textureId, 
                attach
        )
    }
}

/// ## Parameters
*



pub fn get_ped_texture_variation_safe(
        
        
            ped: 
        , 
        
        
            componentId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04A355E041E004E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04A355E041E004E6u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                componentId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_texture_variation_raw(
        ped: , 
        componentId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04A355E041E004E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04A355E041E004E6u64;

        invoke_raw_typed!(
            hash,
                ped, 
                componentId
        )
    }
}

/// ```
Sets the range at which members will automatically leave the group.  
```



pub fn set_group_separation_range_safe(
        
        
            groupHandle: 
        , 
        
        
            separationRange: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4102C7858CFEE4E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4102C7858CFEE4E4u64;
        
        let result = invoke_raw!(
            hash,
                groupHandle, 
                separationRange
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_group_separation_range_raw(
        groupHandle: , 
        separationRange: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4102C7858CFEE4E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4102C7858CFEE4E4u64;

        invoke_raw_typed!(
            hash,
                groupHandle, 
                separationRange
        )
    }
}

/// ## Parameters
*



pub fn is_ped_heading_towards_position_safe(
        
        
            ped: 
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
        let hash = 0xFCF37A457CB96DC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCF37A457CB96DC0u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn is_ped_heading_towards_position_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFCF37A457CB96DC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFCF37A457CB96DC0u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                p4
        )
    }
}

/// ```
The pointer is to a padded struct that matches the arguments to SET_PED_HEAD_BLEND_DATA(...). There are 4 bytes of padding after each field.  
pass this struct in the second parameter   
typedef struct  
{  
        int shapeFirst, shapeSecond, shapeThird;   
        int skinFirst, skinSecond, skinThird;   
	float shapeMix, skinMix, thirdMix;  
} headBlendData;  
```



pub fn get_ped_head_blend_data_safe(
        
        
            ped: 
        , 
        
        
            headBlendData: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2746BD9D88C5C5D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2746BD9D88C5C5D0u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                headBlendData
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_head_blend_data_raw(
        ped: , 
        headBlendData: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2746BD9D88C5C5D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2746BD9D88C5C5D0u64;

        invoke_raw_typed!(
            hash,
                ped, 
                headBlendData
        )
    }
}

/// ## Parameters
*



pub fn set_ped_increased_avoidance_radius_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x570389D1C3DE3C6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x570389D1C3DE3C6Bu64;
        
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
pub fn set_ped_increased_avoidance_radius_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x570389D1C3DE3C6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x570389D1C3DE3C6Bu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn drop_ambient_prop_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFF4710E2A0A6C12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFF4710E2A0A6C12u64;
        
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
pub fn drop_ambient_prop_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFF4710E2A0A6C12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFF4710E2A0A6C12u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_ped_max_health_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4700A416E8324EF3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4700A416E8324EF3u64;
        
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
pub fn get_ped_max_health_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4700A416E8324EF3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4700A416E8324EF3u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
vb.net
Dim ped_handle As Integer
                    With Game.Player.Character
                        Dim pos As Vector3 = .Position + .ForwardVector * 3
                        ped_handle = Native.Function.Call(Of Integer)(Hash.CREATE_RANDOM_PED, pos.X, pos.Y, pos.Z)
                    End With
Creates a Ped at the specified location, returns the Ped Handle.
Ped will not act until SET_PED_AS_NO_LONGER_NEEDED is called.
```



pub fn create_random_ped_safe(
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4AC7D0CF06BFE8Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4AC7D0CF06BFE8Fu64;
        
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
pub fn create_random_ped_raw(
        posX: , 
        posY: , 
        posZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4AC7D0CF06BFE8Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4AC7D0CF06BFE8Fu64;

        invoke_raw_typed!(
            hash,
                posX, 
                posY, 
                posZ
        )
    }
}

/// ```
Enable/disable ped shadow (ambient occlusion). https://gfycat.com/thankfulesteemedgecko
```



pub fn set_ped_ao_blob_rendering_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B5AA717A181FB4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B5AA717A181FB4Cu64;
        
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
pub fn set_ped_ao_blob_rendering_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B5AA717A181FB4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B5AA717A181FB4Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_ped_dies_in_sinking_vehicle_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD718A22995E2B4BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD718A22995E2B4BCu64;
        
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
pub fn set_ped_dies_in_sinking_vehicle_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD718A22995E2B4BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD718A22995E2B4BCu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// Preview: https://gfycat.com/MaleRareAmazonparrot



pub fn set_head_blend_palette_color_safe(
        
        
            ped: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        , 
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC9682B8951C5229u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC9682B8951C5229u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                r, 
                g, 
                b, 
                id
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_head_blend_palette_color_raw(
        ped: , 
        r: , 
        g: , 
        b: , 
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC9682B8951C5229u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC9682B8951C5229u64;

        invoke_raw_typed!(
            hash,
                ped, 
                r, 
                g, 
                b, 
                id
        )
    }
}

/// ## Parameters
*



pub fn was_ped_knocked_out_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61767F73EACEED21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61767F73EACEED21u64;
        
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
pub fn was_ped_knocked_out_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61767F73EACEED21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61767F73EACEED21u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_being_jacked_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A497FE2DF198913u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A497FE2DF198913u64;
        
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
pub fn is_ped_being_jacked_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A497FE2DF198913u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A497FE2DF198913u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Verifies whether ped was eliminated through stealth.



pub fn was_ped_killed_by_stealth_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9800AA1A771B000u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9800AA1A771B000u64;
        
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
pub fn was_ped_killed_by_stealth_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9800AA1A771B000u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9800AA1A771B000u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_motion_blur_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A986918B102B448u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A986918B102B448u64;
        
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
pub fn set_ped_motion_blur_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A986918B102B448u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A986918B102B448u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
Deletes the specified ped, then sets the handle pointed to by the pointer to NULL.  
```



pub fn delete_ped_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9614299DCB53E54Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9614299DCB53E54Bu64;
        
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
pub fn delete_ped_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9614299DCB53E54Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9614299DCB53E54Bu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_ped_prop_index_safe(
        
        
            ped: 
        , 
        
        
            componentId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x898CC20EA75BACD8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x898CC20EA75BACD8u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                componentId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_prop_index_raw(
        ped: , 
        componentId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x898CC20EA75BACD8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x898CC20EA75BACD8u64;

        invoke_raw_typed!(
            hash,
                ped, 
                componentId
        )
    }
}

/// ```
Returns true if the ped passed through the parenthesis is wearing a helmet.  
```



pub fn is_ped_wearing_helmet_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF33BDFE19B309B19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF33BDFE19B309B19u64;
        
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
pub fn is_ped_wearing_helmet_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF33BDFE19B309B19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF33BDFE19B309B19u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
IS_PED_*

Returns true if the ped is currently opening a door (CTaskOpenDoor).
```



pub fn _is_ped_opening_a_door_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26AF0E8E30BD2A2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26AF0E8E30BD2A2Cu64;
        
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
pub fn _is_ped_opening_a_door_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26AF0E8E30BD2A2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26AF0E8E30BD2A2Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ai_melee_weapon_damage_modifier_safe(
        
        
            modifier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66460DEDDD417254u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66460DEDDD417254u64;
        
        let result = invoke_raw!(
            hash,
                modifier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ai_melee_weapon_damage_modifier_raw(
        modifier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66460DEDDD417254u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66460DEDDD417254u64;

        invoke_raw_typed!(
            hash,
                modifier
        )
    }
}

/// ## Parameters
*



pub fn set_ped_stay_in_vehicle_when_jacked_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDF4079F9D54C9A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDF4079F9D54C9A1u64;
        
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
pub fn set_ped_stay_in_vehicle_when_jacked_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDF4079F9D54C9A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDF4079F9D54C9A1u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _is_ped_shader_effect_valid_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x81AA517FBBA05D39u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x81AA517FBBA05D39u64;
        
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
pub fn _is_ped_shader_effect_valid_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x81AA517FBBA05D39u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x81AA517FBBA05D39u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Same as SET_PED_ARMOUR, but ADDS 'amount' to the armor the Ped already has.  
```



pub fn add_armour_to_ped_safe(
        
        
            ped: 
        , 
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BA652A0CD14DF2Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BA652A0CD14DF2Fu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                amount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_armour_to_ped_raw(
        ped: , 
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BA652A0CD14DF2Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BA652A0CD14DF2Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                amount
        )
    }
}

/// ```
p1: Only "CODE_HUMAN_STAND_COWER" found in the b617d scripts.  
```



pub fn set_ped_cower_hash_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA549131166868ED3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA549131166868ED3u64;
        
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
pub fn set_ped_cower_hash_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA549131166868ED3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA549131166868ED3u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
It will revive/cure the injured ped. The condition is ped must not be dead.  
Upon setting and converting the health int, found, if health falls below 5, the ped will lay on the ground in pain(Maximum default health is 100).  
This function is well suited there.  
```



pub fn revive_injured_ped_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D8ACD8388CD99CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D8ACD8388CD99CEu64;
        
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
pub fn revive_injured_ped_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D8ACD8388CD99CEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D8ACD8388CD99CEu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
NOTE: Debugging functions are not present in the retail version of the game.  
*untested but char *name could also be a hash for a localized string  
```



pub fn set_ped_name_debug_safe(
        
        
            ped: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98EFA132A4117BE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98EFA132A4117BE1u64;
        
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
pub fn set_ped_name_debug_raw(
        ped: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x98EFA132A4117BE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x98EFA132A4117BE1u64;

        invoke_raw_typed!(
            hash,
                ped, 
                name
        )
    }
}

/// ## Parameters
*



pub fn set_ped_as_group_leader_safe(
        
        
            ped: 
        , 
        
        
            groupId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A7819605465FBCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A7819605465FBCEu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                groupId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_as_group_leader_raw(
        ped: , 
        groupId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A7819605465FBCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A7819605465FBCEu64;

        invoke_raw_typed!(
            hash,
                ped, 
                groupId
        )
    }
}

/// Creates a copy of the passed ped, optionally setting it as local and/or shallow-copying the head blend data.



pub fn clone_ped_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF29A16337FACADBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF29A16337FACADBu64;
        
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
pub fn clone_ped_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF29A16337FACADBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF29A16337FACADBu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn request_ped_visibility_tracking_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D7A2E43E74E2EB8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D7A2E43E74E2EB8u64;
        
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
pub fn request_ped_visibility_tracking_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D7A2E43E74E2EB8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D7A2E43E74E2EB8u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _0x061cb768363d6424_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x061CB768363D6424u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x061CB768363D6424u64;
        
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
pub fn _0x061cb768363d6424_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x061CB768363D6424u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x061CB768363D6424u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
The function specifically verifies the value is equal to, or less than 1.0f. If it is greater than 1.0f, the function does nothing at all.  
```



pub fn set_driver_ability_safe(
        
        
            driver: 
        , 
        
        
            ability: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB195FFA8042FC5C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB195FFA8042FC5C3u64;
        
        let result = invoke_raw!(
            hash,
                driver, 
                ability
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_driver_ability_raw(
        driver: , 
        ability: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB195FFA8042FC5C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB195FFA8042FC5C3u64;

        invoke_raw_typed!(
            hash,
                driver, 
                ability
        )
    }
}

/// ## Parameters
*



pub fn is_ped_swimming_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DE327631295B4C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DE327631295B4C2u64;
        
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
pub fn is_ped_swimming_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DE327631295B4C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DE327631295B4C2u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Turns the desired ped into a cop. If you use this on the player ped, you will become almost invisible to cops dispatched for you. You will also report your own crimes, get a generic cop voice, get a cop-vision-cone on the radar, and you will be unable to shoot at other cops. SWAT and Army will still shoot at you. Toggling ped as "false" has no effect; you must change p0's ped model to disable the effect.  
```



pub fn set_ped_as_cop_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB03C38DD3FB7FFDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB03C38DD3FB7FFDu64;
        
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
pub fn set_ped_as_cop_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB03C38DD3FB7FFDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB03C38DD3FB7FFDu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_ped_armour_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9483AF821605B1D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9483AF821605B1D8u64;
        
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
pub fn get_ped_armour_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9483AF821605B1D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9483AF821605B1D8u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn spawnpoints_get_search_result_safe(
        
        
            randomInt: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x280C7E3AC7F56E90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x280C7E3AC7F56E90u64;
        
        let result = invoke_raw!(
            hash,
                randomInt, 
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
pub fn spawnpoints_get_search_result_raw(
        randomInt: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x280C7E3AC7F56E90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x280C7E3AC7F56E90u64;

        invoke_raw_typed!(
            hash,
                randomInt, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn set_ped_lod_multiplier_safe(
        
        
            ped: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC2C5C242AAC342Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC2C5C242AAC342Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_lod_multiplier_raw(
        ped: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC2C5C242AAC342Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC2C5C242AAC342Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                multiplier
        )
    }
}

/// REMOVE_SCENARIO_BLOCKING_AREAS native function



pub fn remove_scenario_blocking_areas_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD37401D78A929A49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD37401D78A929A49u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_scenario_blocking_areas_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD37401D78A929A49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD37401D78A929A49u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// When this ped receives its next script task, they will exit from their scenario using the normal scenario exit.
Exiting the scenario may take several frames while the ped is playing the exit animation.
If the ped is not currently using a scenario at the time of the command or 0,0,0 is specified as the reaction position,
then the ped will by default attempt to direct their exit forwards.



pub fn _set_ped_should_play_directed_scenario_exit_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC6935EBE0847B90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC6935EBE0847B90u64;
        
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
pub fn _set_ped_should_play_directed_scenario_exit_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC6935EBE0847B90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC6935EBE0847B90u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_in_any_taxi_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E575D6A898AB852u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E575D6A898AB852u64;
        
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
pub fn is_ped_in_any_taxi_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E575D6A898AB852u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E575D6A898AB852u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn clear_ped_drive_by_clipset_override_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AFE3690D7E0B5ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AFE3690D7E0B5ACu64;
        
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
pub fn clear_ped_drive_by_clipset_override_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AFE3690D7E0B5ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AFE3690D7E0B5ACu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_seat_ped_is_trying_to_enter_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F4C85ACD641BCD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F4C85ACD641BCD2u64;
        
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
pub fn get_seat_ped_is_trying_to_enter_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F4C85ACD641BCD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F4C85ACD641BCD2u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
It simply makes the said ped to cower behind cover object(wall, desk, car)  
Peds flee attributes must be set to not to flee, first. Else, most of the peds, will just flee from gunshot sounds or any other panic situations.  
```



pub fn set_ped_can_cower_in_cover_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB7553CDCEF4A735u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB7553CDCEF4A735u64;
        
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
pub fn set_ped_can_cower_in_cover_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB7553CDCEF4A735u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB7553CDCEF4A735u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_ped_palette_variation_safe(
        
        
            ped: 
        , 
        
        
            componentId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3DD5F2A84B42281u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3DD5F2A84B42281u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                componentId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_palette_variation_raw(
        ped: , 
        componentId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3DD5F2A84B42281u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3DD5F2A84B42281u64;

        invoke_raw_typed!(
            hash,
                ped, 
                componentId
        )
    }
}

/// ```
PED::SET_PED_IN_VEHICLE_CONTEXT(l_128, MISC::GET_HASH_KEY("MINI_PROSTITUTE_LOW_PASSENGER"));
PED::SET_PED_IN_VEHICLE_CONTEXT(l_128, MISC::GET_HASH_KEY("MINI_PROSTITUTE_LOW_RESTRICTED_PASSENGER"));
PED::SET_PED_IN_VEHICLE_CONTEXT(l_3212, MISC::GET_HASH_KEY("MISS_FAMILY1_JIMMY_SIT"));
PED::SET_PED_IN_VEHICLE_CONTEXT(l_3212, MISC::GET_HASH_KEY("MISS_FAMILY1_JIMMY_SIT_REAR"));
PED::SET_PED_IN_VEHICLE_CONTEXT(l_95, MISC::GET_HASH_KEY("MISS_FAMILY2_JIMMY_BICYCLE"));
PED::SET_PED_IN_VEHICLE_CONTEXT(num3, MISC::GET_HASH_KEY("MISSFBI2_MICHAEL_DRIVEBY"));
PED::SET_PED_IN_VEHICLE_CONTEXT(PLAYER::PLAYER_PED_ID(), MISC::GET_HASH_KEY("MISS_ARMENIAN3_FRANKLIN_TENSE"));
PED::SET_PED_IN_VEHICLE_CONTEXT(PLAYER::PLAYER_PED_ID(), MISC::GET_HASH_KEY("MISSFBI5_TREVOR_DRIVING"));
```



pub fn set_ped_in_vehicle_context_safe(
        
        
            ped: 
        , 
        
        
            context: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x530071295899A8C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x530071295899A8C6u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                context
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_in_vehicle_context_raw(
        ped: , 
        context: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x530071295899A8C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x530071295899A8C6u64;

        invoke_raw_typed!(
            hash,
                ped, 
                context
        )
    }
}

/// ## Parameters
*



pub fn get_ped_reset_flag_safe(
        
        
            ped: 
        , 
        
        
            flagId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF9E59B1B1FBF2A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF9E59B1B1FBF2A0u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                flagId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_reset_flag_raw(
        ped: , 
        flagId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF9E59B1B1FBF2A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF9E59B1B1FBF2A0u64;

        invoke_raw_typed!(
            hash,
                ped, 
                flagId
        )
    }
}

/// ## Parameters
*



pub fn has_ped_preload_variation_data_finished_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66680A92700F43DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66680A92700F43DFu64;
        
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
pub fn has_ped_preload_variation_data_finished_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66680A92700F43DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66680A92700F43DFu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
bit 15 (0x8000) = force cower
```



pub fn set_ped_flee_attributes_safe(
        
        
            ped: 
        , 
        
        
            attributeFlags: 
        , 
        
        
            enable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70A2D1137C8ED7C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70A2D1137C8ED7C9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                attributeFlags, 
                enable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_flee_attributes_raw(
        ped: , 
        attributeFlags: , 
        enable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70A2D1137C8ED7C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70A2D1137C8ED7C9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                attributeFlags, 
                enable
        )
    }
}

/// ```
Used with freemode (online) characters.
```



pub fn get_ped_head_overlay_num_safe(
        
        
            overlayID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF1CE768BB43480Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF1CE768BB43480Eu64;
        
        let result = invoke_raw!(
            hash,
                overlayID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_head_overlay_num_raw(
        overlayID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF1CE768BB43480Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF1CE768BB43480Eu64;

        invoke_raw_typed!(
            hash,
                overlayID
        )
    }
}

/// ## Parameters
*



pub fn _0x425aecf167663f48_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x425AECF167663F48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x425AECF167663F48u64;
        
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
pub fn _0x425aecf167663f48_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x425AECF167663F48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x425AECF167663F48u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn clear_ped_alternate_walk_anim_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8844BBFCE30AA9E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8844BBFCE30AA9E9u64;
        
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
pub fn clear_ped_alternate_walk_anim_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8844BBFCE30AA9E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8844BBFCE30AA9E9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// PED::SET_PED_RESET_FLAG(PLAYER::PLAYER_PED_ID(), 240, 1);
Known values:



pub fn set_ped_reset_flag_safe(
        
        
            ped: 
        , 
        
        
            flagId: 
        , 
        
        
            doReset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1E8A365BF3B29F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1E8A365BF3B29F2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                flagId, 
                doReset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_reset_flag_raw(
        ped: , 
        flagId: , 
        doReset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1E8A365BF3B29F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1E8A365BF3B29F2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                flagId, 
                doReset
        )
    }
}

/// ```
This is the SET_CHAR_DUCKING from GTA IV, that makes Peds duck. This function does nothing in GTA V. It cannot set the ped as ducking in vehicles, and IS_PED_DUCKING will always return false.  
```



pub fn set_ped_ducking_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x030983CA930B692Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x030983CA930B692Du64;
        
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
pub fn set_ped_ducking_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x030983CA930B692Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x030983CA930B692Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Return value



pub fn has_pedheadshot_img_upload_succeeded_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8A169E666CBC541u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8A169E666CBC541u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_pedheadshot_img_upload_succeeded_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8A169E666CBC541u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8A169E666CBC541u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn spawnpoints_start_search_in_angled_area_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        , 
        
        
            interiorFlags: 
        , 
        
        
            scale: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2AFF10216DEFA2Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2AFF10216DEFA2Fu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                p3, 
                p4, 
                p5, 
                p6, 
                interiorFlags, 
                scale, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn spawnpoints_start_search_in_angled_area_raw(
        x: , 
        y: , 
        z: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        interiorFlags: , 
        scale: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2AFF10216DEFA2Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2AFF10216DEFA2Fu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                p3, 
                p4, 
                p5, 
                p6, 
                interiorFlags, 
                scale, 
                duration
        )
    }
}

/// ```
enable or disable the gravity of a ped  
Examples:  
PED::SET_PED_GRAVITY(PLAYER::PLAYER_PED_ID(), 0x00000001);  
PED::SET_PED_GRAVITY(Local_289[iVar0 /*20*/], 0x00000001);  
```



pub fn set_ped_gravity_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FF447B6B6AD960Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FF447B6B6AD960Au64;
        
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
pub fn set_ped_gravity_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FF447B6B6AD960Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FF447B6B6AD960Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x0b3e35ac043707d9_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B3E35AC043707D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B3E35AC043707D9u64;
        
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
pub fn _0x0b3e35ac043707d9_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B3E35AC043707D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B3E35AC043707D9u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_ai_weapon_damage_modifier_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B1E2A40A65B8521u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B1E2A40A65B8521u64;
        
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
pub fn set_ai_weapon_damage_modifier_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B1E2A40A65B8521u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B1E2A40A65B8521u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// CLEAR_PED_NON_CREATION_AREA native function



pub fn clear_ped_non_creation_area_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E05208086BA0651u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E05208086BA0651u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_ped_non_creation_area_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E05208086BA0651u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E05208086BA0651u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_synchronized_scene_phase_safe(
        
        
            sceneID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4A310B1D7FA73CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4A310B1D7FA73CCu64;
        
        let result = invoke_raw!(
            hash,
                sceneID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_synchronized_scene_phase_raw(
        sceneID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4A310B1D7FA73CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4A310B1D7FA73CCu64;

        invoke_raw_typed!(
            hash,
                sceneID
        )
    }
}

/// Sets the various freemode face features, e.g. nose length, chin shape.



pub fn _set_ped_face_feature_safe(
        
        
            ped: 
        , 
        
        
            index: 
        , 
        
        
            scale: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71A5C1DBA060049Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71A5C1DBA060049Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                index, 
                scale
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_ped_face_feature_raw(
        ped: , 
        index: , 
        scale: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71A5C1DBA060049Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71A5C1DBA060049Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                index, 
                scale
        )
    }
}

/// ## Parameters
*



pub fn remove_scenario_blocking_area_safe(
        
        
            scenarioBlockingIndex: 
        , 
        
        
            bNetwork: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31D16B74C6E29D66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31D16B74C6E29D66u64;
        
        let result = invoke_raw!(
            hash,
                scenarioBlockingIndex, 
                bNetwork
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_scenario_blocking_area_raw(
        scenarioBlockingIndex: , 
        bNetwork: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31D16B74C6E29D66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31D16B74C6E29D66u64;

        invoke_raw_typed!(
            hash,
                scenarioBlockingIndex, 
                bNetwork
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _clear_facial_clipset_override_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x637822DC2AFEEBF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x637822DC2AFEEBF8u64;
        
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
pub fn _clear_facial_clipset_override_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x637822DC2AFEEBF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x637822DC2AFEEBF8u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0xc30bdaee47256c13_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC30BDAEE47256C13u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC30BDAEE47256C13u64;
        
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
pub fn _0xc30bdaee47256c13_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC30BDAEE47256C13u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC30BDAEE47256C13u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x820e9892a77e97cd_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x820E9892A77E97CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x820E9892A77E97CDu64;
        
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
pub fn _0x820e9892a77e97cd_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x820E9892A77E97CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x820E9892A77E97CDu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// These combat attributes seem to be the same as the BehaviourFlags from combatbehaviour.meta.

So far, these are the equivalents found:

```c
enum eCombatAttribute
{
  CA_INVALID = -1,	
  // AI will only use cover if this is set
  CA_USE_COVER = 0,
  // AI will only use vehicles if this is set
  CA_USE_VEHICLE = 1,
  // AI will only driveby from a vehicle if this is set
  CA_DO_DRIVEBYS = 2,
  // Will be forced to stay in a ny vehicel if this isn't set
  CA_LEAVE_VEHICLES = 3,
  // This ped can make decisions on whether to strafe or not based on distance to destination, recent bullet events, etc.
  CA_CAN_USE_DYNAMIC_STRAFE_DECISIONS	= 4,
  // Ped will always fight upon getting threat response task
  CA_ALWAYS_FIGHT = 5,
  // If in combat and in a vehicle, the ped will flee rather than attacking
  CA_FLEE_WHILST_IN_VEHICLE = 6,
  // If in combat and chasing in a vehicle, the ped will keep a distance behind rather than ramming
  CA_JUST_FOLLOW_VEHICLE = 7,
  // Deprecated
  CA_PLAY_REACTION_ANIMS = 8,
  // Peds will scan for and react to dead peds found
  CA_WILL_SCAN_FOR_DEAD_PEDS = 9,
  // Deprecated
  CA_IS_A_GUARD = 10,
  // The ped will seek cover only 
  CA_JUST_SEEK_COVER = 11,
  // Ped will only blind fire when in cover
  CA_BLIND_FIRE_IN_COVER = 12,
  // Ped may advance
  CA_AGGRESSIVE = 13,
  // Ped can investigate events such as distant gunfire, footsteps, explosions etc
  CA_CAN_INVESTIGATE = 14,
  // Ped can use a radio to call for backup (happens after a reaction)
  CA_CAN_USE_RADIO = 15,
  // Deprecated
  CA_CAN_CAPTURE_ENEMY_PEDS = 16,
  // Ped will always flee upon getting threat response task
  CA_ALWAYS_FLEE = 17,
  // Ped can do unarmed taunts in vehicle
  CA_CAN_TAUNT_IN_VEHICLE = 20,
  // Ped will be able to chase their targets if both are on foot and the target is running away
  CA_CAN_CHASE_TARGET_ON_FOOT = 21,
  // Ped can drag injured peds to safety
  CA_WILL_DRAG_INJURED_PEDS_TO_SAFETY = 22,
  // Ped will require LOS to the target it is aiming at before shooting
  CA_REQUIRES_LOS_TO_SHOOT = 23,
  // Ped is allowed to use proximity based fire rate (increasing fire rate at closer distances)
  CA_USE_PROXIMITY_FIRING_RATE = 24,
  // Normally peds can switch briefly to a secondary target in combat, setting this will prevent that
  CA_DISABLE_SECONDARY_TARGET = 25,
  // This will disable the flinching combat entry reactions for peds, instead only playing the turn and aim anims
  CA_DISABLE_ENTRY_REACTIONS = 26,
  // Force ped to be 100% accurate in all situations (added by Jay Reinebold)
  CA_PERFECT_ACCURACY = 27,
  // If we don't have cover and can't see our target it's possible we will advance, even if the target is in cover
  CA_CAN_USE_FRUSTRATED_ADVANCE	= 28,
  // This will have the ped move to defensive areas and within attack windows before performing the cover search
  CA_MOVE_TO_LOCATION_BEFORE_COVER_SEARCH = 29,
  // Allow shooting of our weapon even if we don't have LOS (this isn't X-ray vision as it only affects weapon firing)
  CA_CAN_SHOOT_WITHOUT_LOS = 30,
  // Ped will try to maintain a min distance to the target, even if using defensive areas (currently only for cover finding + usage) 
  CA_MAINTAIN_MIN_DISTANCE_TO_TARGET = 31,
  // Allows ped to use steamed variations of peeking anims
  CA_CAN_USE_PEEKING_VARIATIONS	= 34,
  // Disables pinned down behaviors
  CA_DISABLE_PINNED_DOWN = 35,
  // Disables pinning down others
  CA_DISABLE_PIN_DOWN_OTHERS = 36,
  // When defensive area is reached the area is cleared and the ped is set to use defensive combat movement
  CA_OPEN_COMBAT_WHEN_DEFENSIVE_AREA_IS_REACHED = 37,
  // Disables bullet reactions
  CA_DISABLE_BULLET_REACTIONS = 38,
  // Allows ped to bust the player
  CA_CAN_BUST = 39,
  // This ped is ignored by other peds when wanted
  CA_IGNORED_BY_OTHER_PEDS_WHEN_WANTED = 40,
  // Ped is allowed to "jack" vehicles when needing to chase a target in combat
  CA_CAN_COMMANDEER_VEHICLES = 41,
  // Ped is allowed to flank
  CA_CAN_FLANK = 42,
  // Ped will switch to advance if they can't find cover
  CA_SWITCH_TO_ADVANCE_IF_CANT_FIND_COVER = 43,
  // Ped will switch to defensive if they are in cover
  CA_SWITCH_TO_DEFENSIVE_IF_IN_COVER = 44,
  // Ped will clear their primary defensive area when it is reached
  CA_CLEAR_PRIMARY_DEFENSIVE_AREA_WHEN_REACHED = 45,
  // Ped is allowed to fight armed peds when not armed
  CA_CAN_FIGHT_ARMED_PEDS_WHEN_NOT_ARMED = 46,
  // Ped is not allowed to use tactical points if set to use defensive movement (will only use cover)
  CA_ENABLE_TACTICAL_POINTS_WHEN_DEFENSIVE = 47,
  // Ped cannot adjust cover arcs when testing cover safety (atm done on corner cover points when  ped usingdefensive area + no LOS)
  CA_DISABLE_COVER_ARC_ADJUSTMENTS = 48,
  // Ped may use reduced accuracy with large number of enemies attacking the same local player target
  CA_USE_ENEMY_ACCURACY_SCALING	= 49,
  // Ped is allowed to charge the enemy position
  CA_CAN_CHARGE = 50,
  // When defensive area is reached the area is cleared and the ped is set to use will advance movement
  CA_REMOVE_AREA_SET_WILL_ADVANCE_WHEN_DEFENSIVE_AREA_REACHED = 51,
  // Use the vehicle attack mission during combat (only works on driver)
  CA_USE_VEHICLE_ATTACK = 52,
  // Use the vehicle attack mission during combat if the vehicle has mounted guns (only works on driver)
  CA_USE_VEHICLE_ATTACK_IF_VEHICLE_HAS_MOUNTED_GUNS = 53,
  // Always equip best weapon in combat
  CA_ALWAYS_EQUIP_BEST_WEAPON = 54,
  // Ignores in water at depth visibility check
  CA_CAN_SEE_UNDERWATER_PEDS = 55,
  // Will prevent this ped from aiming at any AI targets that are in helicopters
  CA_DISABLE_AIM_AT_AI_TARGETS_IN_HELIS = 56,
  // Disables peds seeking due to no clear line of sight
  CA_DISABLE_SEEK_DUE_TO_LINE_OF_SIGHT = 57,
  // To be used when releasing missions peds if we don't want them fleeing from combat (mission peds already prevent flee)
  CA_DISABLE_FLEE_FROM_COMBAT = 58,
  // Disables target changes during vehicle pursuit
  CA_DISABLE_TARGET_CHANGES_DURING_VEHICLE_PURSUIT = 59,
  // Ped may throw a smoke grenade at player loitering in combat
  CA_CAN_THROW_SMOKE_GRENADE = 60,
  // Will clear a set defensive area if that area cannot be reached
  CA_CLEAR_AREA_SET_DEFENSIVE_IF_DEFENSIVE_CANNOT_BE_REACHED = 62,
  // Disable block from pursue during vehicle chases
  CA_DISABLE_BLOCK_FROM_PURSUE_DURING_VEHICLE_CHASE = 64,
  // Disable spin out during vehicle chases
  CA_DISABLE_SPIN_OUT_DURING_VEHICLE_CHASE = 65,
  // Disable cruise in front during block during vehicle chases
  CA_DISABLE_CRUISE_IN_FRONT_DURING_BLOCK_DURING_VEHICLE_CHASE = 66,
  // Makes it more likely that the ped will continue targeting a target with blocked los for a few seconds
  CA_CAN_IGNORE_BLOCKED_LOS_WEIGHTING = 67,
  // Disables the react to buddy shot behaviour.
  CA_DISABLE_REACT_TO_BUDDY_SHOT = 68,
  // Prefer pathing using navmesh over road nodes
  CA_PREFER_NAVMESH_DURING_VEHICLE_CHASE = 69,
  // Ignore road edges when avoiding
  CA_ALLOWED_TO_AVOID_OFFROAD_DURING_VEHICLE_CHASE = 70,
  // Permits ped to charge a target outside the assigned defensive area.
  CA_PERMIT_CHARGE_BEYOND_DEFENSIVE_AREA = 71,
  // This ped will switch to an RPG if target is in a vehicle, otherwise will use alternate weapon.
  CA_USE_ROCKETS_AGAINST_VEHICLES_ONLY = 72,
  // Disables peds moving to a tactical point without clear los
  CA_DISABLE_TACTICAL_POINTS_WITHOUT_CLEAR_LOS = 73,
  // Disables pull alongside during vehicle chase
  CA_DISABLE_PULL_ALONGSIDE_DURING_VEHICLE_CHASE = 74,
  // If set on a ped, they will not flee when all random peds flee is set to TRUE (they are still able to flee due to other reasons)
  CA_DISABLE_ALL_RANDOMS_FLEE = 78,
  // This ped will send out a script DeadPedSeenEvent when they see a dead ped
  CA_WILL_GENERATE_DEAD_PED_SEEN_SCRIPT_EVENTS = 79,
  // This will use the receiving peds sense range rather than the range supplied to the communicate event
  CA_USE_MAX_SENSE_RANGE_WHEN_RECEIVING_EVENTS = 80,
  // When aiming from a vehicle the ped will only aim at targets on his side of the vehicle
  CA_RESTRICT_IN_VEHICLE_AIMING_TO_CURRENT_SIDE = 81,
  // LOS to the target is blocked we return to our default position and direction until we have LOS (no aiming)
  CA_USE_DEFAULT_BLOCKED_LOS_POSITION_AND_DIRECTION = 82,
  // LOS to the target is blocked we return to our default position and direction until we have LOS (no aiming)
  CA_REQUIRES_LOS_TO_AIM = 83,
  // Allow vehicles spawned infront of target facing away to enter cruise and wait to block approaching target
  CA_CAN_CRUISE_AND_BLOCK_IN_VEHICLE = 84,
  // Peds flying aircraft will prefer to target other aircraft over entities on the ground
  CA_PREFER_AIR_COMBAT_WHEN_IN_AIRCRAFT = 85,
  //Allow peds flying aircraft to use dog fighting behaviours
  CA_ALLOW_DOG_FIGHTING = 86,
  // This will make the weight of targets who aircraft vehicles be reduced greatly compared to targets on foot or in ground based vehicles
  CA_PREFER_NON_AIRCRAFT_TARGETS = 87,
  //When peds are tasked to go to combat, they keep searching for a known target for a while before forcing an unknown one
  CA_PREFER_KNOWN_TARGETS_WHEN_COMBAT_CLOSEST_TARGET = 88,
  // Only allow mounted weapons to fire if within the correct attack angle (default 25-degree cone). On a flag in order to keep exiting behaviour and only fix in specific cases.
  CA_FORCE_CHECK_ATTACK_ANGLE_FOR_MOUNTED_GUNS = 89,
  // Blocks the firing state for passenger-controlled mounted weapons. Existing flags CA_USE_VEHICLE_ATTACK and CA_USE_VEHICLE_ATTACK_IF_VEHICLE_HAS_MOUNTED_GUNS only work for drivers.
  CA_BLOCK_FIRE_FOR_VEHICLE_PASSENGER_MOUNTED_GUNS = 90 
};
```



pub fn set_ped_combat_attributes_safe(
        
        
            ped: 
        , 
        
        
            attributeIndex: 
        , 
        
        
            enabled: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F7794730795E019u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F7794730795E019u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                attributeIndex, 
                enabled
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_combat_attributes_raw(
        ped: , 
        attributeIndex: , 
        enabled: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F7794730795E019u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F7794730795E019u64;

        invoke_raw_typed!(
            hash,
                ped, 
                attributeIndex, 
                enabled
        )
    }
}

/// ## Parameters
*



pub fn is_ped_responding_to_event_safe(
        
        
            ped: 
        , 
        
        
            event: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x625B774D75C87068u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x625B774D75C87068u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                event
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_responding_to_event_raw(
        ped: , 
        event: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x625B774D75C87068u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x625B774D75C87068u64;

        invoke_raw_typed!(
            hash,
                ped, 
                event
        )
    }
}

/// ## Parameters
*



pub fn get_number_of_ped_drawable_variations_safe(
        
        
            ped: 
        , 
        
        
            componentId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27561561732A7842u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27561561732A7842u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                componentId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_number_of_ped_drawable_variations_raw(
        ped: , 
        componentId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27561561732A7842u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27561561732A7842u64;

        invoke_raw_typed!(
            hash,
                ped, 
                componentId
        )
    }
}

/// ## Parameters
*



pub fn _0x5407b7288d0478b7_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5407B7288D0478B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5407B7288D0478B7u64;
        
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
pub fn _0x5407b7288d0478b7_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5407B7288D0478B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5407B7288D0478B7u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _freeze_ped_camera_rotation_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF287323B0E2C69Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF287323B0E2C69Au64;
        
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
pub fn _freeze_ped_camera_rotation_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF287323B0E2C69Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF287323B0E2C69Au64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_dead_ped_pickup_coords_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD5003B097200F36u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD5003B097200F36u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_dead_ped_pickup_coords_raw(
        ped: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD5003B097200F36u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD5003B097200F36u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2
        )
    }
}

/// ## Return value



pub fn spawnpoints_is_search_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C67506996001F5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C67506996001F5Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn spawnpoints_is_search_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C67506996001F5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C67506996001F5Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _is_ped_hair_color_valid_2_safe(
        
        
            colorId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED6D8E27A43B8CDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED6D8E27A43B8CDEu64;
        
        let result = invoke_raw!(
            hash,
                colorId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_ped_hair_color_valid_2_raw(
        colorId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED6D8E27A43B8CDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED6D8E27A43B8CDEu64;

        invoke_raw_typed!(
            hash,
                colorId
        )
    }
}

/// ## Parameters
*



pub fn is_ped_in_high_cover_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A03BF943D767C93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A03BF943D767C93u64;
        
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
pub fn is_ped_in_high_cover_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A03BF943D767C93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A03BF943D767C93u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_weapon_movement_clipset_safe(
        
        
            ped: 
        , 
        
        
            clipSet: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2622E35B77D3ACA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2622E35B77D3ACA2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                clipSet
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_weapon_movement_clipset_raw(
        ped: , 
        clipSet: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2622E35B77D3ACA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2622E35B77D3ACA2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                clipSet
        )
    }
}

/// ## Parameters
*



pub fn set_create_random_cops_on_scenarios_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x444CB7D7DBE6973Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x444CB7D7DBE6973Du64;
        
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
pub fn set_create_random_cops_on_scenarios_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x444CB7D7DBE6973Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x444CB7D7DBE6973Du64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
_SET_PED_HEAD_* - _SET_PED_HEARING_*

_SET_PED_HEALTH_...
```

```
NativeDB Introduced: v2699
```



pub fn _0xb3352e018d6f89df_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3352E018D6F89DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3352E018D6F89DFu64;
        
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
pub fn _0xb3352e018d6f89df_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3352E018D6F89DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3352E018D6F89DFu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Sets whether a pedestrian should wear a helmet.



pub fn set_ped_helmet_safe(
        
        
            ped: 
        , 
        
        
            bEnable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x560A43136EB58105u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x560A43136EB58105u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                bEnable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_helmet_raw(
        ped: , 
        bEnable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x560A43136EB58105u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x560A43136EB58105u64;

        invoke_raw_typed!(
            hash,
                ped, 
                bEnable
        )
    }
}

/// ```
This native refers to the field of vision the ped has below them, starting at 0 degrees. The angle value should be negative.  
```



pub fn set_ped_visual_field_min_elevation_angle_safe(
        
        
            ped: 
        , 
        
        
            angle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A276EB2C224D70Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A276EB2C224D70Fu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                angle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_visual_field_min_elevation_angle_raw(
        ped: , 
        angle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A276EB2C224D70Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A276EB2C224D70Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                angle
        )
    }
}

/// ```
gtaforums.com/topic/885580-ped-headshotmugshot-txd/  
```



pub fn is_pedheadshot_ready_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7085228842B13A67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7085228842B13A67u64;
        
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
pub fn is_pedheadshot_ready_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7085228842B13A67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7085228842B13A67u64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ## Parameters
*



pub fn set_scripted_anim_seat_offset_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5917BBA32D06C230u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5917BBA32D06C230u64;
        
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
pub fn set_scripted_anim_seat_offset_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5917BBA32D06C230u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5917BBA32D06C230u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
0: Freedom to move
1: Circle Around Leader
2: Alternative Circle Around Leader  
3: Line, with Leader at center  
4: Arrow Formation
5: "V" Formation
6: Line Follow Formation
7: Single Formation
8: Pairwise
```



pub fn set_group_formation_safe(
        
        
            groupId: 
        , 
        
        
            formationType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE2F5FC3AF7E8C1Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE2F5FC3AF7E8C1Eu64;
        
        let result = invoke_raw!(
            hash,
                groupId, 
                formationType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_group_formation_raw(
        groupId: , 
        formationType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE2F5FC3AF7E8C1Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE2F5FC3AF7E8C1Eu64;

        invoke_raw_typed!(
            hash,
                groupId, 
                formationType
        )
    }
}

/// ## Parameters
*



pub fn clear_ped_parachute_pack_variation_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1280804F7CFD2D6Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1280804F7CFD2D6Cu64;
        
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
pub fn clear_ped_parachute_pack_variation_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1280804F7CFD2D6Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1280804F7CFD2D6Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Returns whether the specified ped is reloading.  
```



pub fn is_ped_reloading_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24B100C68C645951u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24B100C68C645951u64;
        
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
pub fn is_ped_reloading_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24B100C68C645951u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24B100C68C645951u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Maximum possible amount of money on MP is 2000. ~JX



pub fn set_ped_money_safe(
        
        
            ped: 
        , 
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9C8960E8684C1B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9C8960E8684C1B5u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                amount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_money_raw(
        ped: , 
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9C8960E8684C1B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9C8960E8684C1B5u64;

        invoke_raw_typed!(
            hash,
                ped, 
                amount
        )
    }
}

/// ## Parameters
*



pub fn create_random_ped_as_driver_safe(
        
        
            vehicle: 
        , 
        
        
            returnHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B62392B474F44A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B62392B474F44A0u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                returnHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_random_ped_as_driver_raw(
        vehicle: , 
        returnHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B62392B474F44A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B62392B474F44A0u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                returnHandle
        )
    }
}

/// ## Parameters
*



pub fn reset_group_formation_default_spacing_safe(
        
        
            groupHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63DAB4CCB3273205u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63DAB4CCB3273205u64;
        
        let result = invoke_raw!(
            hash,
                groupHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_group_formation_default_spacing_raw(
        groupHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63DAB4CCB3273205u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63DAB4CCB3273205u64;

        invoke_raw_typed!(
            hash,
                groupHandle
        )
    }
}

/// ```
damages a ped with the given amount



pub fn apply_damage_to_ped_safe(
        
        
            ped: 
        , 
        
        
            damageAmount: 
        , 
        
        
            armorFirst: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x697157CED63F18D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x697157CED63F18D4u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                damageAmount, 
                armorFirst
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn apply_damage_to_ped_raw(
        ped: , 
        damageAmount: , 
        armorFirst: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x697157CED63F18D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x697157CED63F18D4u64;

        invoke_raw_typed!(
            hash,
                ped, 
                damageAmount, 
                armorFirst
        )
    }
}

/// Console/PC structure definitions and example: pastebin.com/SsFej963

For FiveM/Cfx.Re use-cases refer to: [`GET_GAME_POOL`](#_0x2B9D4F50).



pub fn get_ped_nearby_peds_safe(
        
        
            ped: 
        , 
        
        
            sizeAndPeds: 
        , 
        
        
            ignore: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23F8F5FC7E8C4A6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23F8F5FC7E8C4A6Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                sizeAndPeds, 
                ignore
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_nearby_peds_raw(
        ped: , 
        sizeAndPeds: , 
        ignore: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23F8F5FC7E8C4A6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23F8F5FC7E8C4A6Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                sizeAndPeds, 
                ignore
        )
    }
}

/// ## Parameters
*



pub fn is_ped_ducking_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD125AE748725C6BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD125AE748725C6BCu64;
        
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
pub fn is_ped_ducking_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD125AE748725C6BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD125AE748725C6BCu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// [Animations list](https://alexguirre.github.io/animations-list/)



pub fn is_scripted_scenario_ped_using_conditional_anim_safe(
        
        
            ped: 
        , 
        
        
            animDict: 
        , 
        
        
            anim: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6EC47A344923E1EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6EC47A344923E1EDu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                animDict, 
                anim
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_scripted_scenario_ped_using_conditional_anim_raw(
        ped: , 
        animDict: , 
        anim: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6EC47A344923E1EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6EC47A344923E1EDu64;

        invoke_raw_typed!(
            hash,
                ped, 
                animDict, 
                anim
        )
    }
}

/// ## Parameters
*



pub fn get_ped_as_group_leader_safe(
        
        
            groupID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CCE68DBD5FE93ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CCE68DBD5FE93ECu64;
        
        let result = invoke_raw!(
            hash,
                groupID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_as_group_leader_raw(
        groupID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CCE68DBD5FE93ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CCE68DBD5FE93ECu64;

        invoke_raw_typed!(
            hash,
                groupID
        )
    }
}

/// ## Parameters
*



pub fn reset_ped_weapon_movement_clipset_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97B0DB5B4AA74E77u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97B0DB5B4AA74E77u64;
        
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
pub fn reset_ped_weapon_movement_clipset_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97B0DB5B4AA74E77u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97B0DB5B4AA74E77u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn reset_ped_visible_damage_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3AC1F7B898F30C05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3AC1F7B898F30C05u64;
        
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
pub fn reset_ped_visible_damage_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3AC1F7B898F30C05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3AC1F7B898F30C05u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_dies_instantly_in_water_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEB64139BA29A7CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEB64139BA29A7CFu64;
        
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
pub fn set_ped_dies_instantly_in_water_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEB64139BA29A7CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEB64139BA29A7CFu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**



pub fn _0x1216e0bfa72cc703_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1216E0BFA72CC703u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1216E0BFA72CC703u64;
        
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
pub fn _0x1216e0bfa72cc703_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1216E0BFA72CC703u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1216E0BFA72CC703u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```c
enum eNMFallType {
    TYPE_FROM_HIGH = 0,
    TYPE_OVER_WALL = 1,
    TYPE_DOWN_STAIRS = 2,
    TYPE_DIE_TYPES = 3,
    TYPE_DIE_FROM_HIGH = 4,
    TYPE_DIE_OVER_WALL = 5,
    TYPE_DIE_DOWN_STAIRS = 6
}
```

```
Return variable is never used in R*'s scripts.  
Not sure what p2 does. It seems like it would be a time judging by it's usage in R*'s scripts, but didn't seem to affect anything in my testings.  
x, y, and z are coordinates, most likely to where the ped will fall.  
p7 is probably the force of the fall, but untested, so I left the variable name the same.  
p8 to p13 are always 0f in R*'s scripts.  
(Simplified) Example of the usage of the function from R*'s scripts:  
ped::set_ped_to_ragdoll_with_fall(ped, 1500, 2000, 1, -entity::get_entity_forward_vector(ped), 1f, 0f, 0f, 0f, 0f, 0f, 0f);  
```



pub fn set_ped_to_ragdoll_with_fall_safe(
        
        
            ped: 
        , 
        
        
            minTime: 
        , 
        
        
            maxTime: 
        , 
        
        
            nFallType: 
        , 
        
        
            dirX: 
        , 
        
        
            dirY: 
        , 
        
        
            dirZ: 
        , 
        
        
            fGroundHeight: 
        , 
        
        
            grab1X: 
        , 
        
        
            grab1Y: 
        , 
        
        
            grab1Z: 
        , 
        
        
            grab2X: 
        , 
        
        
            grab2Y: 
        , 
        
        
            grab2Z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD76632D99E4966C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD76632D99E4966C8u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                minTime, 
                maxTime, 
                nFallType, 
                dirX, 
                dirY, 
                dirZ, 
                fGroundHeight, 
                grab1X, 
                grab1Y, 
                grab1Z, 
                grab2X, 
                grab2Y, 
                grab2Z
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_to_ragdoll_with_fall_raw(
        ped: , 
        minTime: , 
        maxTime: , 
        nFallType: , 
        dirX: , 
        dirY: , 
        dirZ: , 
        fGroundHeight: , 
        grab1X: , 
        grab1Y: , 
        grab1Z: , 
        grab2X: , 
        grab2Y: , 
        grab2Z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD76632D99E4966C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD76632D99E4966C8u64;

        invoke_raw_typed!(
            hash,
                ped, 
                minTime, 
                maxTime, 
                nFallType, 
                dirX, 
                dirY, 
                dirZ, 
                fGroundHeight, 
                grab1X, 
                grab1Y, 
                grab1Z, 
                grab2X, 
                grab2Y, 
                grab2Z
        )
    }
}

/// ## Parameters
*



pub fn request_pedheadshot_img_upload_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0DAEF2F545BEE25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0DAEF2F545BEE25u64;
        
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
pub fn request_pedheadshot_img_upload_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0DAEF2F545BEE25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0DAEF2F545BEE25u64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ## Parameters
*



pub fn knock_off_ped_prop_safe(
        
        
            ped: 
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
        let hash = 0x6FD7816A36615F48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FD7816A36615F48u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn knock_off_ped_prop_raw(
        ped: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6FD7816A36615F48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6FD7816A36615F48u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ```
There seem to be 26 flags  
```



pub fn clear_ragdoll_blocking_flags_safe(
        
        
            ped: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD86D101FCFD00A4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD86D101FCFD00A4Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_ragdoll_blocking_flags_raw(
        ped: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD86D101FCFD00A4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD86D101FCFD00A4Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                flags
        )
    }
}

/// ```
Creates a new NaturalMotion message.  
startImmediately: If set to true, the character will perform the message the moment it receives it by GIVE_PED_NM_MESSAGE. If false, the Ped will get the message but won't perform it yet. While it's a boolean value, if negative, the message will not be initialized.  
messageId: The ID of the NaturalMotion message.  
If a message already exists, this function does nothing. A message exists until the point it has been successfully dispatched by GIVE_PED_NM_MESSAGE.  
```



pub fn create_nm_message_safe(
        
        
            startImmediately: 
        , 
        
        
            messageId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x418EF2A1BCE56685u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x418EF2A1BCE56685u64;
        
        let result = invoke_raw!(
            hash,
                startImmediately, 
                messageId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_nm_message_raw(
        startImmediately: , 
        messageId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x418EF2A1BCE56685u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x418EF2A1BCE56685u64;

        invoke_raw_typed!(
            hash,
                startImmediately, 
                messageId
        )
    }
}

/// ```
Ped will no longer get angry when you stay near him.  
```



pub fn remove_ped_defensive_area_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74D4E028107450A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74D4E028107450A9u64;
        
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
pub fn remove_ped_defensive_area_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74D4E028107450A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74D4E028107450A9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x2f074c904d85129e_safe(
        
        
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F074C904D85129Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F074C904D85129Eu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
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
pub fn _0x2f074c904d85129e_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F074C904D85129Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F074C904D85129Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6
        )
    }
}

/// ```
i could be time. Only example in the decompiled scripts uses it as -1.
```



pub fn set_ped_pinned_down_safe(
        
        
            ped: 
        , 
        
        
            pinned: 
        , 
        
        
            i: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAD6D1ACF08F4612u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAD6D1ACF08F4612u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                pinned, 
                i
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_pinned_down_raw(
        ped: , 
        pinned: , 
        i: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAD6D1ACF08F4612u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAD6D1ACF08F4612u64;

        invoke_raw_typed!(
            hash,
                ped, 
                pinned, 
                i
        )
    }
}

/// ## Parameters
*



pub fn is_ped_group_member_safe(
        
        
            ped: 
        , 
        
        
            groupId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BB01E3834671191u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BB01E3834671191u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                groupId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_group_member_raw(
        ped: , 
        groupId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BB01E3834671191u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BB01E3834671191u64;

        invoke_raw_typed!(
            hash,
                ped, 
                groupId
        )
    }
}

/// Remove a helmet from a ped



pub fn remove_ped_helmet_safe(
        
        
            ped: 
        , 
        
        
            instantly: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7B2458D0AD6DED8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7B2458D0AD6DED8u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                instantly
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_ped_helmet_raw(
        ped: , 
        instantly: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7B2458D0AD6DED8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7B2458D0AD6DED8u64;

        invoke_raw_typed!(
            hash,
                ped, 
                instantly
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _is_ped_body_blemish_valid_safe(
        
        
            colorID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x09E7ECA981D9B210u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x09E7ECA981D9B210u64;
        
        let result = invoke_raw!(
            hash,
                colorID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_ped_body_blemish_valid_raw(
        colorID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x09E7ECA981D9B210u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x09E7ECA981D9B210u64;

        invoke_raw_typed!(
            hash,
                colorID
        )
    }
}

/// ```
Prevents the ped from going limp.  
[Example: Can prevent peds from falling when standing on moving vehicles.]  
```



pub fn can_ped_ragdoll_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x128F79EDCECE4FD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x128F79EDCECE4FD5u64;
        
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
pub fn can_ped_ragdoll_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x128F79EDCECE4FD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x128F79EDCECE4FD5u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_smash_glass_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CCE141467FF42A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CCE141467FF42A2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_can_smash_glass_raw(
        ped: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CCE141467FF42A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CCE141467FF42A2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn request_ped_vehicle_visibility_tracking_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BC338A7B21F4608u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BC338A7B21F4608u64;
        
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
pub fn request_ped_vehicle_visibility_tracking_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BC338A7B21F4608u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BC338A7B21F4608u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```c
enum ePedMotionState
{
    MOTIONSTATE_NONE = -294553821, // MotionState_None
    MOTIONSTATE_IDLE = -1871534317, // MotionState_Idle
    MOTIONSTATE_WALK = -668482597, // MotionState_Walk
    MOTIONSTATE_RUN = -530524, // MotionState_Run
    MOTIONSTATE_SPRINT = -1115154469, // MotionState_Sprint
    MOTIONSTATE_CROUCH_IDLE = 1140525470, // MotionState_Crouch_Idle
    MOTIONSTATE_CROUCH_WALK = 147004056, // MotionState_Crouch_Walk
    MOTIONSTATE_CROUCH_RUN = 898879241, // MotionState_Crouch_Run
    MOTIONSTATE_DONOTHING = 247561816, // MotionState_DoNothing
    MOTIONSTATE_ANIMATEDVELOCITY = 1427811395, // MotionState_AnimatedVelocity
    MOTIONSTATE_INVEHICLE = -1797663347, // MotionState_InVehicle
    MOTIONSTATE_AIMING = 1063765679, // MotionState_Aiming
    MOTIONSTATE_DIVING_IDLE = 1212730861, // MotionState_Diving_Idle
    MOTIONSTATE_DIVING_SWIM = -1855028596, // MotionState_Diving_Swim
    MOTIONSTATE_SWIMMING_TREADWATER = -776007225, // MotionState_Swimming_TreadWater
    MOTIONSTATE_DEAD = 230360860, // MotionState_Dead
    MOTIONSTATE_STEALTH_IDLE = 1110276645, // MotionState_Stealth_Idle
    MOTIONSTATE_STEALTH_WALK = 69908130, // MotionState_Stealth_Walk
    MOTIONSTATE_STEALTH_RUN = -83133983, // MotionState_Stealth_Run
    MOTIONSTATE_PARACHUTING = -1161760501, // MotionState_Parachuting
    MOTIONSTATE_ACTIONMODE_IDLE = -633298724, // MotionState_ActionMode_Idle
    MOTIONSTATE_ACTIONMODE_WALK = -762290521, // MotionState_ActionMode_Walk
    MOTIONSTATE_ACTIONMODE_RUN = 834330132, // MotionState_ActionMode_Run
    MOTIONSTATE_JETPACK = 1398696542 // MotionState_Jetpack
}
```



pub fn force_ped_motion_state_safe(
        
        
            ped: 
        , 
        
        
            motionStateHash: 
        , 
        
        
            shouldReset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF28965D04F570DCAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF28965D04F570DCAu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                motionStateHash, 
                shouldReset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_ped_motion_state_raw(
        ped: , 
        motionStateHash: , 
        shouldReset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF28965D04F570DCAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF28965D04F570DCAu64;

        invoke_raw_typed!(
            hash,
                ped, 
                motionStateHash, 
                shouldReset
        )
    }
}

/// ```
Clears the relationship between two groups. This should be called twice (once for each group).  
Relationship types:  
0 = Companion  
1 = Respect  
2 = Like  
3 = Neutral  
4 = Dislike  
5 = Hate  
255 = Pedestrians  
(Credits: Inco)  
Example:  
PED::CLEAR_RELATIONSHIP_BETWEEN_GROUPS(2, l_1017, 0xA49E591C);  
PED::CLEAR_RELATIONSHIP_BETWEEN_GROUPS(2, 0xA49E591C, l_1017);  
```



pub fn clear_relationship_between_groups_safe(
        
        
            relationship: 
        , 
        
        
            group1: 
        , 
        
        
            group2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E29243FB56FC6D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E29243FB56FC6D4u64;
        
        let result = invoke_raw!(
            hash,
                relationship, 
                group1, 
                group2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_relationship_between_groups_raw(
        relationship: , 
        group1: , 
        group2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E29243FB56FC6D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E29243FB56FC6D4u64;

        invoke_raw_typed!(
            hash,
                relationship, 
                group1, 
                group2
        )
    }
}

/// For more info please refer to [this](https://gtaforums.com/topic/858970-all-gtao-face-ids-pedset-ped-head-blend-data-explained) topic.



pub fn set_ped_head_blend_data_safe(
        
        
            ped: 
        , 
        
        
            shapeFirstID: 
        , 
        
        
            shapeSecondID: 
        , 
        
        
            shapeThirdID: 
        , 
        
        
            skinFirstID: 
        , 
        
        
            skinSecondID: 
        , 
        
        
            skinThirdID: 
        , 
        
        
            shapeMix: 
        , 
        
        
            skinMix: 
        , 
        
        
            thirdMix: 
        , 
        
        
            isParent: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9414E18B9434C2FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9414E18B9434C2FEu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                shapeFirstID, 
                shapeSecondID, 
                shapeThirdID, 
                skinFirstID, 
                skinSecondID, 
                skinThirdID, 
                shapeMix, 
                skinMix, 
                thirdMix, 
                isParent
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_head_blend_data_raw(
        ped: , 
        shapeFirstID: , 
        shapeSecondID: , 
        shapeThirdID: , 
        skinFirstID: , 
        skinSecondID: , 
        skinThirdID: , 
        shapeMix: , 
        skinMix: , 
        thirdMix: , 
        isParent: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9414E18B9434C2FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9414E18B9434C2FEu64;

        invoke_raw_typed!(
            hash,
                ped, 
                shapeFirstID, 
                shapeSecondID, 
                shapeThirdID, 
                skinFirstID, 
                skinSecondID, 
                skinThirdID, 
                shapeMix, 
                skinMix, 
                thirdMix, 
                isParent
        )
    }
}

/// Checks if the component variation is valid, this works great for randomizing components using loops.



pub fn is_ped_component_variation_valid_safe(
        
        
            ped: 
        , 
        
        
            componentId: 
        , 
        
        
            drawableId: 
        , 
        
        
            textureId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE825F6B6CEA7671Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE825F6B6CEA7671Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                componentId, 
                drawableId, 
                textureId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_component_variation_valid_raw(
        ped: , 
        componentId: , 
        drawableId: , 
        textureId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE825F6B6CEA7671Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE825F6B6CEA7671Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                componentId, 
                drawableId, 
                textureId
        )
    }
}

/// ## Parameters
*



pub fn get_ped_helmet_stored_hat_prop_index_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x451294E859ECC018u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x451294E859ECC018u64;
        
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
pub fn get_ped_helmet_stored_hat_prop_index_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x451294E859ECC018u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x451294E859ECC018u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Used one time in fmmc_launcher.c instead of CLONE_PED because ?



pub fn _clone_ped_ex_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x668FD40BCBA5DE48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x668FD40BCBA5DE48u64;
        
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
pub fn _clone_ped_ex_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x668FD40BCBA5DE48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x668FD40BCBA5DE48u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Ids
0 - Head
1 - Beard
2 - Hair
3 - Torso
4 - Legs
5 - Hands
6 - Foot
7 - Scarfs/Neck Accessories
8 - Accessories 1
9 - Accessories 2
10- Decals
11 - Auxiliary parts for torso
```



pub fn get_ped_drawable_variation_safe(
        
        
            ped: 
        , 
        
        
            componentId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67F3780DD425D4FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67F3780DD425D4FCu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                componentId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_drawable_variation_raw(
        ped: , 
        componentId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67F3780DD425D4FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67F3780DD425D4FCu64;

        invoke_raw_typed!(
            hash,
                ped, 
                componentId
        )
    }
}

/// ## Parameters
*



pub fn is_ped_running_melee_task_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1871251F3B5ACD7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1871251F3B5ACD7u64;
        
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
pub fn is_ped_running_melee_task_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1871251F3B5ACD7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1871251F3B5ACD7u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Used for freemode (online) characters. 
Called after SET_PED_HEAD_OVERLAY().  
```



pub fn _set_ped_head_overlay_color_safe(
        
        
            ped: 
        , 
        
        
            overlayID: 
        , 
        
        
            colorType: 
        , 
        
        
            colorID: 
        , 
        
        
            secondColorID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x497BF74A7B9CB952u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x497BF74A7B9CB952u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                overlayID, 
                colorType, 
                colorID, 
                secondColorID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_ped_head_overlay_color_raw(
        ped: , 
        overlayID: , 
        colorType: , 
        colorID: , 
        secondColorID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x497BF74A7B9CB952u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x497BF74A7B9CB952u64;

        invoke_raw_typed!(
            hash,
                ped, 
                overlayID, 
                colorType, 
                colorID, 
                secondColorID
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_be_targetted_by_team_safe(
        
        
            ped: 
        , 
        
        
            team: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF1CA77833E58F2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF1CA77833E58F2Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                team, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_can_be_targetted_by_team_raw(
        ped: , 
        team: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF1CA77833E58F2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF1CA77833E58F2Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                team, 
                toggle
        )
    }
}

/// ```
SET_PED_ALLOW*
```



pub fn _0x49e50bdb8ba4dab2_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49E50BDB8BA4DAB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49E50BDB8BA4DAB2u64;
        
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
pub fn _0x49e50bdb8ba4dab2_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49E50BDB8BA4DAB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49E50BDB8BA4DAB2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn release_pedheadshot_img_upload_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D517B27CF6ECD04u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D517B27CF6ECD04u64;
        
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
pub fn release_pedheadshot_img_upload_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D517B27CF6ECD04u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D517B27CF6ECD04u64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ```
Gets a value indicating whether this ped's health is below its fatally injured threshold. The default threshold is 100.  
If the handle is invalid, the function returns true.  
```



pub fn is_ped_fatally_injured_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD839450756ED5A80u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD839450756ED5A80u64;
        
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
pub fn is_ped_fatally_injured_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD839450756ED5A80u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD839450756ED5A80u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Sets Ped Default Clothes  
```



pub fn set_ped_default_component_variation_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45EEE61580806D63u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45EEE61580806D63u64;
        
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
pub fn set_ped_default_component_variation_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45EEE61580806D63u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45EEE61580806D63u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_torso_vehicle_ik_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6647C5F6F5792496u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6647C5F6F5792496u64;
        
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
pub fn set_ped_can_torso_vehicle_ik_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6647C5F6F5792496u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6647C5F6F5792496u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
FIRING_PATTERN_BURST_FIRE = 0xD6FF6D61 ( 1073727030 )  
FIRING_PATTERN_BURST_FIRE_IN_COVER = 0x026321F1 ( 40051185 )  
FIRING_PATTERN_BURST_FIRE_DRIVEBY = 0xD31265F2 ( -753768974 )  
FIRING_PATTERN_FROM_GROUND = 0x2264E5D6 ( 577037782 )  
FIRING_PATTERN_DELAY_FIRE_BY_ONE_SEC = 0x7A845691 ( 2055493265 )  
FIRING_PATTERN_FULL_AUTO = 0xC6EE6B4C ( -957453492 )  
FIRING_PATTERN_SINGLE_SHOT = 0x5D60E4E0 ( 1566631136 )  
FIRING_PATTERN_BURST_FIRE_PISTOL = 0xA018DB8A ( -1608983670 )  
FIRING_PATTERN_BURST_FIRE_SMG = 0xD10DADEE ( 1863348768 )  
FIRING_PATTERN_BURST_FIRE_RIFLE = 0x9C74B406 ( -1670073338 )  
FIRING_PATTERN_BURST_FIRE_MG = 0xB573C5B4 ( -1250703948 )  
FIRING_PATTERN_BURST_FIRE_PUMPSHOTGUN = 0x00BAC39B ( 12239771 )  
FIRING_PATTERN_BURST_FIRE_HELI = 0x914E786F ( -1857128337 )  
FIRING_PATTERN_BURST_FIRE_MICRO = 0x42EF03FD ( 1122960381 )  
FIRING_PATTERN_SHORT_BURSTS = 0x1A92D7DF ( 445831135 )  
FIRING_PATTERN_SLOW_FIRE_TANK = 0xE2CA3A71 ( -490063247 )  
if anyone is interested firing pattern info: pastebin.com/Px036isB  
```



pub fn set_ped_firing_pattern_safe(
        
        
            ped: 
        , 
        
        
            patternHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AC577F5A12AD8A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AC577F5A12AD8A9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                patternHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_firing_pattern_raw(
        ped: , 
        patternHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AC577F5A12AD8A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AC577F5A12AD8A9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                patternHash
        )
    }
}

/// ```
xyz - relative to the world origin.  
```



pub fn is_cop_ped_in_area_3d_safe(
        
        
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16EC4839969F9F5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16EC4839969F9F5Eu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_cop_ped_in_area_3d_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16EC4839969F9F5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16EC4839969F9F5Eu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2
        )
    }
}

/// ```
This only will teleport the ped to the group leader if the group leader teleports (sets coords).  
Only works in singleplayer  
```



pub fn set_ped_can_teleport_to_group_leader_safe(
        
        
            pedHandle: 
        , 
        
        
            groupHandle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E2F4240B3F24647u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E2F4240B3F24647u64;
        
        let result = invoke_raw!(
            hash,
                pedHandle, 
                groupHandle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_can_teleport_to_group_leader_raw(
        pedHandle: , 
        groupHandle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E2F4240B3F24647u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E2F4240B3F24647u64;

        invoke_raw_typed!(
            hash,
                pedHandle, 
                groupHandle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_ped_doing_driveby_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2C086CC1BF8F2BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2C086CC1BF8F2BFu64;
        
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
pub fn is_ped_doing_driveby_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2C086CC1BF8F2BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2C086CC1BF8F2BFu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
FORCE_*
```



pub fn _0xed3c76adfa6d07c4_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED3C76ADFA6D07C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED3C76ADFA6D07C4u64;
        
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
pub fn _0xed3c76adfa6d07c4_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED3C76ADFA6D07C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED3C76ADFA6D07C4u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_strafe_clipset_safe(
        
        
            ped: 
        , 
        
        
            clipSet: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29A28F3F8CF6D854u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29A28F3F8CF6D854u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                clipSet
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_strafe_clipset_raw(
        ped: , 
        clipSet: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29A28F3F8CF6D854u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29A28F3F8CF6D854u64;

        invoke_raw_typed!(
            hash,
                ped, 
                clipSet
        )
    }
}

/// ## Parameters
*



pub fn set_ped_dies_in_vehicle_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A30922C90C9B42Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A30922C90C9B42Cu64;
        
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
pub fn set_ped_dies_in_vehicle_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A30922C90C9B42Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A30922C90C9B42Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_ped_preferred_cover_set_safe(
        
        
            ped: 
        , 
        
        
            itemSet: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8421EB4DA7E391B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8421EB4DA7E391B9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                itemSet
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_preferred_cover_set_raw(
        ped: , 
        itemSet: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8421EB4DA7E391B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8421EB4DA7E391B9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                itemSet
        )
    }
}

/// ```
GET_*
```



pub fn _0x1e77fa7a62ee6c4c_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E77FA7A62EE6C4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E77FA7A62EE6C4Cu64;
        
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
pub fn _0x1e77fa7a62ee6c4c_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E77FA7A62EE6C4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E77FA7A62EE6C4Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn is_ped_in_model_safe(
        
        
            ped: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x796D90EFB19AA332u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x796D90EFB19AA332u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_in_model_raw(
        ped: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x796D90EFB19AA332u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x796D90EFB19AA332u64;

        invoke_raw_typed!(
            hash,
                ped, 
                modelHash
        )
    }
}

/// Examines whether the ped is engaged in combat; when given a target ped index, it confirms if the ped is actively fighting the specified target, returning true if engaged and false if not.



pub fn is_ped_in_combat_safe(
        
        
            ped: 
        , 
        
        
            target: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4859F1FC66A6278Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4859F1FC66A6278Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                target
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_in_combat_raw(
        ped: , 
        target: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4859F1FC66A6278Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4859F1FC66A6278Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                target
        )
    }
}

/// ## Parameters
*



pub fn set_ped_dies_in_water_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56CEF0AC79073BDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56CEF0AC79073BDEu64;
        
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
pub fn set_ped_dies_in_water_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56CEF0AC79073BDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56CEF0AC79073BDEu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0xa9b61a329bfdcbea_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9B61A329BFDCBEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9B61A329BFDCBEAu64;
        
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
pub fn _0xa9b61a329bfdcbea_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA9B61A329BFDCBEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA9B61A329BFDCBEAu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn have_all_streaming_requests_completed_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7350823473013C02u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7350823473013C02u64;
        
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
pub fn have_all_streaming_requests_completed_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7350823473013C02u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7350823473013C02u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_being_stealth_killed_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x863B23EFDE9C5DF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x863B23EFDE9C5DF2u64;
        
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
pub fn is_ped_being_stealth_killed_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x863B23EFDE9C5DF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x863B23EFDE9C5DF2u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
This is only called once in the scripts.
sub_1CD9(&l_49, 0, getElem(3, &l_34, 4), "MICHAEL", 0, 1);
                    sub_1CA8("WORLD_HUMAN_SMOKING", 2);
                    PED::SET_PED_PRIMARY_LOOKAT(getElem(3, &l_34, 4), PLAYER::PLAYER_PED_ID());
```



pub fn set_ped_primary_lookat_safe(
        
        
            ped: 
        , 
        
        
            lookAt: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD17B554996A8D9Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD17B554996A8D9Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                lookAt
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_primary_lookat_raw(
        ped: , 
        lookAt: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD17B554996A8D9Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD17B554996A8D9Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                lookAt
        )
    }
}

/// ## Parameters
*



pub fn set_ped_reserve_parachute_tint_index_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE88DA0751C22A2ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE88DA0751C22A2ADu64;
        
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
pub fn set_ped_reserve_parachute_tint_index_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE88DA0751C22A2ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE88DA0751C22A2ADu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn is_ped_performing_dependent_combo_limit_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBD0EDBA5BE957CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBD0EDBA5BE957CFu64;
        
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
pub fn is_ped_performing_dependent_combo_limit_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBD0EDBA5BE957CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBD0EDBA5BE957CFu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn hide_ped_blood_damage_by_zone_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62AB793144DE75DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62AB793144DE75DCu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hide_ped_blood_damage_by_zone_raw(
        ped: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62AB793144DE75DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62AB793144DE75DCu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2
        )
    }
}

/// This native sets a scuba mask for freemode models and an oxygen bottle for player_* models. It works on freemode and player_* models.



pub fn _set_ped_scuba_gear_variation_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36C6984C3ED0C911u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36C6984C3ED0C911u64;
        
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
pub fn _set_ped_scuba_gear_variation_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36C6984C3ED0C911u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36C6984C3ED0C911u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_ped_enveff_scale_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C14D30395A51A3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C14D30395A51A3Cu64;
        
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
pub fn get_ped_enveff_scale_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C14D30395A51A3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C14D30395A51A3Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_synchronized_scene_looped_safe(
        
        
            sceneID: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9A897A4C6C2974Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9A897A4C6C2974Fu64;
        
        let result = invoke_raw!(
            hash,
                sceneID, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_synchronized_scene_looped_raw(
        sceneID: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9A897A4C6C2974Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9A897A4C6C2974Fu64;

        invoke_raw_typed!(
            hash,
                sceneID, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0xad27d957598e49e9_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD27D957598E49E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD27D957598E49E9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xad27d957598e49e9_raw(
        ped: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD27D957598E49E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD27D957598E49E9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn _0xceda60a74219d064_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEDA60A74219D064u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEDA60A74219D064u64;
        
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
pub fn _0xceda60a74219d064_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEDA60A74219D064u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEDA60A74219D064u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn is_ped_shooting_in_area_safe(
        
        
            ped: 
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E9DFE24AC1E58EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E9DFE24AC1E58EFu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                p7, 
                p8
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_shooting_in_area_raw(
        ped: , 
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        p7: , 
        p8: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E9DFE24AC1E58EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E9DFE24AC1E58EFu64;

        invoke_raw_typed!(
            hash,
                ped, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                p7, 
                p8
        )
    }
}

/// Determines if a ped is dead. Contrary to what the name might suggest, it does not always detect when a ped is in the 'dying' phase (transitioning to death). The exception is when `checkMeleeDeathFlags` is set to `true`, which then includes peds in the midst of melee takedown moves as being in a dying state, even if the death task has not yet started.

```
NativeDB Introduced: v323
```



pub fn is_ped_dead_or_dying_safe(
        
        
            ped: 
        , 
        
        
            checkMeleeDeathFlags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3317DEDB88C95038u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3317DEDB88C95038u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                checkMeleeDeathFlags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_dead_or_dying_raw(
        ped: , 
        checkMeleeDeathFlags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3317DEDB88C95038u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3317DEDB88C95038u64;

        invoke_raw_typed!(
            hash,
                ped, 
                checkMeleeDeathFlags
        )
    }
}

/// ## Parameters
*



pub fn set_ped_blocks_pathing_when_dead_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x576594E8D64375E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x576594E8D64375E2u64;
        
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
pub fn set_ped_blocks_pathing_when_dead_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x576594E8D64375E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x576594E8D64375E2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_ped_parachute_tint_index_safe(
        
        
            ped: 
        , 
        
        
            outTintIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAF5F7E5AE7C6C9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAF5F7E5AE7C6C9Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                outTintIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_parachute_tint_index_raw(
        ped: , 
        outTintIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAF5F7E5AE7C6C9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAF5F7E5AE7C6C9Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                outTintIndex
        )
    }
}

/// ```
PED::REGISTER_TARGET(l_216, PLAYER::PLAYER_PED_ID()); from re_prisonbreak.txt.  
l_216 = RECSBRobber1  
```



pub fn register_target_safe(
        
        
            ped: 
        , 
        
        
            target: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F25D9AEFA34FBA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F25D9AEFA34FBA2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                target
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn register_target_raw(
        ped: , 
        target: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F25D9AEFA34FBA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F25D9AEFA34FBA2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                target
        )
    }
}

/// ## Parameters
*



pub fn disable_ped_heatscale_override_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x600048C60D5C2C51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x600048C60D5C2C51u64;
        
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
pub fn disable_ped_heatscale_override_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x600048C60D5C2C51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x600048C60D5C2C51u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Gives the ped a helmet. Can be removed by invoking [`REMOVE_PED_HELMET`](#_0xA7B2458D0AD6DED8).

```c
enum ePedCompFlags {
  PV_FLAG_NONE                  = 0, // 0
  PV_FLAG_BULKY                 = 1, // 1<<0
  PV_FLAG_JOB                   = 2, // 1<<1
  PV_FLAG_SUNNY                 = 4, // 1<<2
  PV_FLAG_WET                   = 8, // 1<<3
  PV_FLAG_COLD                  = 16, // 1<<4
  PV_FLAG_NOT_IN_CAR            = 32, // 1<<5
  PV_FLAG_BIKE_ONLY             = 64, // 1<<6
  PV_FLAG_NOT_INDOORS           = 128, // 1<<7
  PV_FLAG_FIRE_RETARDENT        = 256, // 1<<8
  PV_FLAG_ARMOURED              = 512, // 1<<9
  PV_FLAG_LIGHTLY_ARMOURED      = 1024, // 1<<10
  PV_FLAG_HIGH_DETAIL           = 2048, // 1<<11
  PV_FLAG_DEFAULT_HELMET        = 4096, // 1<<12
  PV_FLAG_RANDOM_HELMET         = 8192, // 1<<13
  PV_FLAG_SCRIPT_HELMET         = 16384, // 1<<14
  PV_FLAG_FLIGHT_HELMET         = 32768, // 1<<15
  PV_FLAG_HIDE_IN_FIRST_PERSON  = 65536, // 1<<16
  PV_FLAG_USE_PHYSICS_HAT_2     = 131072, // 1<<17
  PV_FLAG_PILOT_HELMET          = 262144 // 1<<18
};
```



pub fn give_ped_helmet_safe(
        
        
            ped: 
        , 
        
        
            cannotRemove: 
        , 
        
        
            helmetFlag: 
        , 
        
        
            textureIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54C7C4A94367717Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54C7C4A94367717Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                cannotRemove, 
                helmetFlag, 
                textureIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn give_ped_helmet_raw(
        ped: , 
        cannotRemove: , 
        helmetFlag: , 
        textureIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54C7C4A94367717Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54C7C4A94367717Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                cannotRemove, 
                helmetFlag, 
                textureIndex
        )
    }
}

/// Input: Makeup color index, value between 0 and 63 (inclusive).
Output: RGB values for the makeup color specified in the input.

This is used with the makeup color swatches scaleform.

Use [`_0x4852FC386E2E1BB5`](#_0x4852FC386E2E1BB5) to get the hair colors.



pub fn _get_ped_makeup_rgb_color_safe(
        
        
            makeupColorIndex: 
        , 
        
        
            outR: 
        , 
        
        
            outG: 
        , 
        
        
            outB: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x013E5CFC38CD5387u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x013E5CFC38CD5387u64;
        
        let result = invoke_raw!(
            hash,
                makeupColorIndex, 
                outR, 
                outG, 
                outB
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_ped_makeup_rgb_color_raw(
        makeupColorIndex: , 
        outR: , 
        outG: , 
        outB: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x013E5CFC38CD5387u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x013E5CFC38CD5387u64;

        invoke_raw_typed!(
            hash,
                makeupColorIndex, 
                outR, 
                outG, 
                outB
        )
    }
}

/// ```c
enum ePedDecorationZone
{
	ZONE_TORSO = 0,
	ZONE_HEAD = 1,
	ZONE_LEFT_ARM = 2,
	ZONE_RIGHT_ARM = 3,
	ZONE_LEFT_LEG = 4,
	ZONE_RIGHT_LEG = 5,
	ZONE_UNKNOWN = 6,
	ZONE_NONE = 7
};
```



pub fn get_ped_decoration_zone_from_hashes_safe(
        
        
            collection: 
        , 
        
        
            overlay: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FD452BFBE7A7A8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FD452BFBE7A7A8Bu64;
        
        let result = invoke_raw!(
            hash,
                collection, 
                overlay
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_decoration_zone_from_hashes_raw(
        collection: , 
        overlay: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FD452BFBE7A7A8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FD452BFBE7A7A8Bu64;

        invoke_raw_typed!(
            hash,
                collection, 
                overlay
        )
    }
}

/// ## Parameters
*



pub fn set_ped_coords_no_gang_safe(
        
        
            ped: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87052FE446E07247u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87052FE446E07247u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn set_ped_coords_no_gang_raw(
        ped: , 
        posX: , 
        posY: , 
        posZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87052FE446E07247u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87052FE446E07247u64;

        invoke_raw_typed!(
            hash,
                ped, 
                posX, 
                posY, 
                posZ
        )
    }
}

/// ## Parameters
*



pub fn set_ped_heatscale_override_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1F6EBF9A3D55538u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1F6EBF9A3D55538u64;
        
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
pub fn set_ped_heatscale_override_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1F6EBF9A3D55538u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1F6EBF9A3D55538u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_steers_around_vehicles_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB6FB9D48DDE23ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB6FB9D48DDE23ECu64;
        
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
pub fn set_ped_steers_around_vehicles_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB6FB9D48DDE23ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB6FB9D48DDE23ECu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
Setting ped to true allows the ped to shoot "friendlies".  
p2 set to true when toggle is also true seams to make peds permanently unable to aim at, even if you set p2 back to false.  
p1 = false & p2 = false for unable to aim at.  
p1 = true & p2 = false for able to aim at.  
```



pub fn set_can_attack_friendly_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3B1CB349FF9C75Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3B1CB349FF9C75Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                toggle, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_can_attack_friendly_raw(
        ped: , 
        toggle: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3B1CB349FF9C75Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3B1CB349FF9C75Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_switch_weapon_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED7F7EFE9FABF340u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED7F7EFE9FABF340u64;
        
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
pub fn set_ped_can_switch_weapon_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED7F7EFE9FABF340u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED7F7EFE9FABF340u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
Gets a value indicating whether the specified ped is in the specified vehicle.  
If 'atGetIn' is false, the function will not return true until the ped is sitting in the vehicle and is about to close the door. If it's true, the function returns true the moment the ped starts to get onto the seat (after opening the door). Eg. if false, and the ped is getting into a submersible, the function will not return true until the ped has descended down into the submersible and gotten into the seat, while if it's true, it'll return true the moment the hatch has been opened and the ped is about to descend into the submersible.  
```



pub fn is_ped_in_vehicle_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            atGetIn: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3EE4A07279BB9DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3EE4A07279BB9DBu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                atGetIn
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_in_vehicle_raw(
        ped: , 
        vehicle: , 
        atGetIn: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3EE4A07279BB9DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3EE4A07279BB9DBu64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                atGetIn
        )
    }
}

/// ## Parameters
*



pub fn is_ped_diving_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5527B8246FEF9B11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5527B8246FEF9B11u64;
        
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
pub fn is_ped_diving_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5527B8246FEF9B11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5527B8246FEF9B11u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```c
enum ePedBoneId : uint16_t
{
    SKEL_ROOT = 0x0,
    SKEL_Pelvis = 0x2E28,
    SKEL_L_Thigh = 0xE39F,
    SKEL_L_Calf = 0xF9BB,
    SKEL_L_Foot = 0x3779,
    SKEL_L_Toe0 = 0x83C,
    EO_L_Foot = 0x84C5,
    EO_L_Toe = 0x68BD,
    IK_L_Foot = 0xFEDD,
    PH_L_Foot = 0xE175,
    MH_L_Knee = 0xB3FE,
    SKEL_R_Thigh = 0xCA72,
    SKEL_R_Calf = 0x9000,
    SKEL_R_Foot = 0xCC4D,
    SKEL_R_Toe0 = 0x512D,
    EO_R_Foot = 0x1096,
    EO_R_Toe = 0x7163,
    IK_R_Foot = 0x8AAE,
    PH_R_Foot = 0x60E6,
    MH_R_Knee = 0x3FCF,
    RB_L_ThighRoll = 0x5C57,
    RB_R_ThighRoll = 0x192A,
    SKEL_Spine_Root = 0xE0FD,
    SKEL_Spine0 = 0x5C01,
    SKEL_Spine1 = 0x60F0,
    SKEL_Spine2 = 0x60F1,
    SKEL_Spine3 = 0x60F2,
    SKEL_L_Clavicle = 0xFCD9,
    SKEL_L_UpperArm = 0xB1C5,
    SKEL_L_Forearm = 0xEEEB,
    SKEL_L_Hand = 0x49D9,
    SKEL_L_Finger00 = 0x67F2,
    SKEL_L_Finger01 = 0xFF9,
    SKEL_L_Finger02 = 0xFFA,
    SKEL_L_Finger10 = 0x67F3,
    SKEL_L_Finger11 = 0x1049,
    SKEL_L_Finger12 = 0x104A,
    SKEL_L_Finger20 = 0x67F4,
    SKEL_L_Finger21 = 0x1059,
    SKEL_L_Finger22 = 0x105A,
    SKEL_L_Finger30 = 0x67F5,
    SKEL_L_Finger31 = 0x1029,
    SKEL_L_Finger32 = 0x102A,
    SKEL_L_Finger40 = 0x67F6,
    SKEL_L_Finger41 = 0x1039,
    SKEL_L_Finger42 = 0x103A,
    PH_L_Hand = 0xEB95,
    IK_L_Hand = 0x8CBD,
    RB_L_ForeArmRoll = 0xEE4F,
    RB_L_ArmRoll = 0x1470,
    MH_L_Elbow = 0x58B7,
    SKEL_R_Clavicle = 0x29D2,
    SKEL_R_UpperArm = 0x9D4D,
    SKEL_R_Forearm = 0x6E5C,
    SKEL_R_Hand = 0xDEAD,
    SKEL_R_Finger00 = 0xE5F2,
    SKEL_R_Finger01 = 0xFA10,
    SKEL_R_Finger02 = 0xFA11,
    SKEL_R_Finger10 = 0xE5F3,
    SKEL_R_Finger11 = 0xFA60,
    SKEL_R_Finger12 = 0xFA61,
    SKEL_R_Finger20 = 0xE5F4,
    SKEL_R_Finger21 = 0xFA70,
    SKEL_R_Finger22 = 0xFA71,
    SKEL_R_Finger30 = 0xE5F5,
    SKEL_R_Finger31 = 0xFA40,
    SKEL_R_Finger32 = 0xFA41,
    SKEL_R_Finger40 = 0xE5F6,
    SKEL_R_Finger41 = 0xFA50,
    SKEL_R_Finger42 = 0xFA51,
    PH_R_Hand = 0x6F06,
    IK_R_Hand = 0x188E,
    RB_R_ForeArmRoll = 0xAB22,
    RB_R_ArmRoll = 0x90FF,
    MH_R_Elbow = 0xBB0,
    SKEL_Neck_1 = 0x9995,
    SKEL_Head = 0x796E,
    IK_Head = 0x322C,
    FACIAL_facialRoot = 0xFE2C,
    FB_L_Brow_Out_000 = 0xE3DB,
    FB_L_Lid_Upper_000 = 0xB2B6,
    FB_L_Eye_000 = 0x62AC,
    FB_L_CheekBone_000 = 0x542E,
    FB_L_Lip_Corner_000 = 0x74AC,
    FB_R_Lid_Upper_000 = 0xAA10,
    FB_R_Eye_000 = 0x6B52,
    FB_R_CheekBone_000 = 0x4B88,
    FB_R_Brow_Out_000 = 0x54C,
    FB_R_Lip_Corner_000 = 0x2BA6,
    FB_Brow_Centre_000 = 0x9149,
    FB_UpperLipRoot_000 = 0x4ED2,
    FB_UpperLip_000 = 0xF18F,
    FB_L_Lip_Top_000 = 0x4F37,
    FB_R_Lip_Top_000 = 0x4537,
    FB_Jaw_000 = 0xB4A0,
    FB_LowerLipRoot_000 = 0x4324,
    FB_LowerLip_000 = 0x508F,
    FB_L_Lip_Bot_000 = 0xB93B,
    FB_R_Lip_Bot_000 = 0xC33B,
    FB_Tongue_000 = 0xB987,
    RB_Neck_1 = 0x8B93,
    SPR_L_Breast = 0xFC8E,
    SPR_R_Breast = 0x885F,
    IK_Root = 0xDD1C,
    SKEL_Neck_2 = 0x5FD4,
    SKEL_Pelvis1 = 0xD003,
    SKEL_PelvisRoot = 0x45FC,
    SKEL_SADDLE = 0x9524,
    MH_L_CalfBack = 0x1013,
    MH_L_ThighBack = 0x600D,
    SM_L_Skirt = 0xC419,
    MH_R_CalfBack = 0xB013,
    MH_R_ThighBack = 0x51A3,
    SM_R_Skirt = 0x7712,
    SM_M_BackSkirtRoll = 0xDBB,
    SM_L_BackSkirtRoll = 0x40B2,
    SM_R_BackSkirtRoll = 0xC141,
    SM_M_FrontSkirtRoll = 0xCDBB,
    SM_L_FrontSkirtRoll = 0x9B69,
    SM_R_FrontSkirtRoll = 0x86F1,
    SM_CockNBalls_ROOT = 0xC67D,
    SM_CockNBalls = 0x9D34,
    MH_L_Finger00 = 0x8C63,
    MH_L_FingerBulge00 = 0x5FB8,
    MH_L_Finger10 = 0x8C53,
    MH_L_FingerTop00 = 0xA244,
    MH_L_HandSide = 0xC78A,
    MH_Watch = 0x2738,
    MH_L_Sleeve = 0x933C,
    MH_R_Finger00 = 0x2C63,
    MH_R_FingerBulge00 = 0x69B8,
    MH_R_Finger10 = 0x2C53,
    MH_R_FingerTop00 = 0xEF4B,
    MH_R_HandSide = 0x68FB,
    MH_R_Sleeve = 0x92DC,
    FACIAL_jaw = 0xB21,
    FACIAL_underChin = 0x8A95,
    FACIAL_L_underChin = 0x234E,
    FACIAL_chin = 0xB578,
    FACIAL_chinSkinBottom = 0x98BC,
    FACIAL_L_chinSkinBottom = 0x3E8F,
    FACIAL_R_chinSkinBottom = 0x9E8F,
    FACIAL_tongueA = 0x4A7C,
    FACIAL_tongueB = 0x4A7D,
    FACIAL_tongueC = 0x4A7E,
    FACIAL_tongueD = 0x4A7F,
    FACIAL_tongueE = 0x4A80,
    FACIAL_L_tongueE = 0x35F2,
    FACIAL_R_tongueE = 0x2FF2,
    FACIAL_L_tongueD = 0x35F1,
    FACIAL_R_tongueD = 0x2FF1,
    FACIAL_L_tongueC = 0x35F0,
    FACIAL_R_tongueC = 0x2FF0,
    FACIAL_L_tongueB = 0x35EF,
    FACIAL_R_tongueB = 0x2FEF,
    FACIAL_L_tongueA = 0x35EE,
    FACIAL_R_tongueA = 0x2FEE,
    FACIAL_chinSkinTop = 0x7226,
    FACIAL_L_chinSkinTop = 0x3EB3,
    FACIAL_chinSkinMid = 0x899A,
    FACIAL_L_chinSkinMid = 0x4427,
    FACIAL_L_chinSide = 0x4A5E,
    FACIAL_R_chinSkinMid = 0xF5AF,
    FACIAL_R_chinSkinTop = 0xF03B,
    FACIAL_R_chinSide = 0xAA5E,
    FACIAL_R_underChin = 0x2BF4,
    FACIAL_L_lipLowerSDK = 0xB9E1,
    FACIAL_L_lipLowerAnalog = 0x244A,
    FACIAL_L_lipLowerThicknessV = 0xC749,
    FACIAL_L_lipLowerThicknessH = 0xC67B,
    FACIAL_lipLowerSDK = 0x7285,
    FACIAL_lipLowerAnalog = 0xD97B,
    FACIAL_lipLowerThicknessV = 0xC5BB,
    FACIAL_lipLowerThicknessH = 0xC5ED,
    FACIAL_R_lipLowerSDK = 0xA034,
    FACIAL_R_lipLowerAnalog = 0xC2D9,
    FACIAL_R_lipLowerThicknessV = 0xC6E9,
    FACIAL_R_lipLowerThicknessH = 0xC6DB,
    FACIAL_nose = 0x20F1,
    FACIAL_L_nostril = 0x7322,
    FACIAL_L_nostrilThickness = 0xC15F,
    FACIAL_noseLower = 0xE05A,
    FACIAL_L_noseLowerThickness = 0x79D5,
    FACIAL_R_noseLowerThickness = 0x7975,
    FACIAL_noseTip = 0x6A60,
    FACIAL_R_nostril = 0x7922,
    FACIAL_R_nostrilThickness = 0x36FF,
    FACIAL_noseUpper = 0xA04F,
    FACIAL_L_noseUpper = 0x1FB8,
    FACIAL_noseBridge = 0x9BA3,
    FACIAL_L_nasolabialFurrow = 0x5ACA,
    FACIAL_L_nasolabialBulge = 0xCD78,
    FACIAL_L_cheekLower = 0x6907,
    FACIAL_L_cheekLowerBulge1 = 0xE3FB,
    FACIAL_L_cheekLowerBulge2 = 0xE3FC,
    FACIAL_L_cheekInner = 0xE7AB,
    FACIAL_L_cheekOuter = 0x8161,
    FACIAL_L_eyesackLower = 0x771B,
    FACIAL_L_eyeball = 0x1744,
    FACIAL_L_eyelidLower = 0x998C,
    FACIAL_L_eyelidLowerOuterSDK = 0xFE4C,
    FACIAL_L_eyelidLowerOuterAnalog = 0xB9AA,
    FACIAL_L_eyelashLowerOuter = 0xD7F6,
    FACIAL_L_eyelidLowerInnerSDK = 0xF151,
    FACIAL_L_eyelidLowerInnerAnalog = 0x8242,
    FACIAL_L_eyelashLowerInner = 0x4CCF,
    FACIAL_L_eyelidUpper = 0x97C1,
    FACIAL_L_eyelidUpperOuterSDK = 0xAF15,
    FACIAL_L_eyelidUpperOuterAnalog = 0x67FA,
    FACIAL_L_eyelashUpperOuter = 0x27B7,
    FACIAL_L_eyelidUpperInnerSDK = 0xD341,
    FACIAL_L_eyelidUpperInnerAnalog = 0xF092,
    FACIAL_L_eyelashUpperInner = 0x9B1F,
    FACIAL_L_eyesackUpperOuterBulge = 0xA559,
    FACIAL_L_eyesackUpperInnerBulge = 0x2F2A,
    FACIAL_L_eyesackUpperOuterFurrow = 0xC597,
    FACIAL_L_eyesackUpperInnerFurrow = 0x52A7,
    FACIAL_forehead = 0x9218,
    FACIAL_L_foreheadInner = 0x843,
    FACIAL_L_foreheadInnerBulge = 0x767C,
    FACIAL_L_foreheadOuter = 0x8DCB,
    FACIAL_skull = 0x4221,
    FACIAL_foreheadUpper = 0xF7D6,
    FACIAL_L_foreheadUpperInner = 0xCF13,
    FACIAL_L_foreheadUpperOuter = 0x509B,
    FACIAL_R_foreheadUpperInner = 0xCEF3,
    FACIAL_R_foreheadUpperOuter = 0x507B,
    FACIAL_L_temple = 0xAF79,
    FACIAL_L_ear = 0x19DD,
    FACIAL_L_earLower = 0x6031,
    FACIAL_L_masseter = 0x2810,
    FACIAL_L_jawRecess = 0x9C7A,
    FACIAL_L_cheekOuterSkin = 0x14A5,
    FACIAL_R_cheekLower = 0xF367,
    FACIAL_R_cheekLowerBulge1 = 0x599B,
    FACIAL_R_cheekLowerBulge2 = 0x599C,
    FACIAL_R_masseter = 0x810,
    FACIAL_R_jawRecess = 0x93D4,
    FACIAL_R_ear = 0x1137,
    FACIAL_R_earLower = 0x8031,
    FACIAL_R_eyesackLower = 0x777B,
    FACIAL_R_nasolabialBulge = 0xD61E,
    FACIAL_R_cheekOuter = 0xD32,
    FACIAL_R_cheekInner = 0x737C,
    FACIAL_R_noseUpper = 0x1CD6,
    FACIAL_R_foreheadInner = 0xE43,
    FACIAL_R_foreheadInnerBulge = 0x769C,
    FACIAL_R_foreheadOuter = 0x8FCB,
    FACIAL_R_cheekOuterSkin = 0xB334,
    FACIAL_R_eyesackUpperInnerFurrow = 0x9FAE,
    FACIAL_R_eyesackUpperOuterFurrow = 0x140F,
    FACIAL_R_eyesackUpperInnerBulge = 0xA359,
    FACIAL_R_eyesackUpperOuterBulge = 0x1AF9,
    FACIAL_R_nasolabialFurrow = 0x2CAA,
    FACIAL_R_temple = 0xAF19,
    FACIAL_R_eyeball = 0x1944,
    FACIAL_R_eyelidUpper = 0x7E14,
    FACIAL_R_eyelidUpperOuterSDK = 0xB115,
    FACIAL_R_eyelidUpperOuterAnalog = 0xF25A,
    FACIAL_R_eyelashUpperOuter = 0xE0A,
    FACIAL_R_eyelidUpperInnerSDK = 0xD541,
    FACIAL_R_eyelidUpperInnerAnalog = 0x7C63,
    FACIAL_R_eyelashUpperInner = 0x8172,
    FACIAL_R_eyelidLower = 0x7FDF,
    FACIAL_R_eyelidLowerOuterSDK = 0x1BD,
    FACIAL_R_eyelidLowerOuterAnalog = 0x457B,
    FACIAL_R_eyelashLowerOuter = 0xBE49,
    FACIAL_R_eyelidLowerInnerSDK = 0xF351,
    FACIAL_R_eyelidLowerInnerAnalog = 0xE13,
    FACIAL_R_eyelashLowerInner = 0x3322,
    FACIAL_L_lipUpperSDK = 0x8F30,
    FACIAL_L_lipUpperAnalog = 0xB1CF,
    FACIAL_L_lipUpperThicknessH = 0x37CE,
    FACIAL_L_lipUpperThicknessV = 0x38BC,
    FACIAL_lipUpperSDK = 0x1774,
    FACIAL_lipUpperAnalog = 0xE064,
    FACIAL_lipUpperThicknessH = 0x7993,
    FACIAL_lipUpperThicknessV = 0x7981,
    FACIAL_L_lipCornerSDK = 0xB1C,
    FACIAL_L_lipCornerAnalog = 0xE568,
    FACIAL_L_lipCornerThicknessUpper = 0x7BC,
    FACIAL_L_lipCornerThicknessLower = 0xDD42,
    FACIAL_R_lipUpperSDK = 0x7583,
    FACIAL_R_lipUpperAnalog = 0x51CF,
    FACIAL_R_lipUpperThicknessH = 0x382E,
    FACIAL_R_lipUpperThicknessV = 0x385C,
    FACIAL_R_lipCornerSDK = 0xB3C,
    FACIAL_R_lipCornerAnalog = 0xEE0E,
    FACIAL_R_lipCornerThicknessUpper = 0x54C3,
    FACIAL_R_lipCornerThicknessLower = 0x2BBA,
    MH_MulletRoot = 0x3E73,
    MH_MulletScaler = 0xA1C2,
    MH_Hair_Scale = 0xC664,
    MH_Hair_Crown = 0x1675,
    SM_Torch = 0x8D6,
    FX_Light = 0x8959,
    FX_Light_Scale = 0x5038,
    FX_Light_Switch = 0xE18E,
    BagRoot = 0xAD09,
    BagPivotROOT = 0xB836,
    BagPivot = 0x4D11,
    BagBody = 0xAB6D,
    BagBone_R = 0x937,
    BagBone_L = 0x991,
    SM_LifeSaver_Front = 0x9420,
    SM_R_Pouches_ROOT = 0x2962,
    SM_R_Pouches = 0x4141,
    SM_L_Pouches_ROOT = 0x2A02,
    SM_L_Pouches = 0x4B41,
    SM_Suit_Back_Flapper = 0xDA2D,
    SPR_CopRadio = 0x8245,
    SM_LifeSaver_Back = 0x2127,
    MH_BlushSlider = 0xA0CE,
    SKEL_Tail_01 = 0x347,
    SKEL_Tail_02 = 0x348,
    MH_L_Concertina_B = 0xC988,
    MH_L_Concertina_A = 0xC987,
    MH_R_Concertina_B = 0xC8E8,
    MH_R_Concertina_A = 0xC8E7,
    MH_L_ShoulderBladeRoot = 0x8711,
    MH_L_ShoulderBlade = 0x4EAF,
    MH_R_ShoulderBladeRoot = 0x3A0A,
    MH_R_ShoulderBlade = 0x54AF,
    FB_R_Ear_000 = 0x6CDF,
    SPR_R_Ear = 0x63B6,
    FB_L_Ear_000 = 0x6439,
    SPR_L_Ear = 0x5B10,
    FB_TongueA_000 = 0x4206,
    FB_TongueB_000 = 0x4207,
    FB_TongueC_000 = 0x4208,
    SKEL_L_Toe1 = 0x1D6B,
    SKEL_R_Toe1 = 0xB23F,
    SKEL_Tail_03 = 0x349,
    SKEL_Tail_04 = 0x34A,
    SKEL_Tail_05 = 0x34B,
    SPR_Gonads_ROOT = 0xBFDE,
    SPR_Gonads = 0x1C00,
    FB_L_Brow_Out_001 = 0xE3DB,
    FB_L_Lid_Upper_001 = 0xB2B6,
    FB_L_Eye_001 = 0x62AC,
    FB_L_CheekBone_001 = 0x542E,
    FB_L_Lip_Corner_001 = 0x74AC,
    FB_R_Lid_Upper_001 = 0xAA10,
    FB_R_Eye_001 = 0x6B52,
    FB_R_CheekBone_001 = 0x4B88,
    FB_R_Brow_Out_001 = 0x54C,
    FB_R_Lip_Corner_001 = 0x2BA6,
    FB_Brow_Centre_001 = 0x9149,
    FB_UpperLipRoot_001 = 0x4ED2,
    FB_UpperLip_001 = 0xF18F,
    FB_L_Lip_Top_001 = 0x4F37,
    FB_R_Lip_Top_001 = 0x4537,
    FB_Jaw_001 = 0xB4A0,
    FB_LowerLipRoot_001 = 0x4324,
    FB_LowerLip_001 = 0x508F,
    FB_L_Lip_Bot_001 = 0xB93B,
    FB_R_Lip_Bot_001 = 0xC33B,
    FB_Tongue_001 = 0xB987
}; 
```



pub fn get_ped_bone_index_safe(
        
        
            ped: 
        , 
        
        
            boneId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F428D08BE5AAE31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F428D08BE5AAE31u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                boneId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_bone_index_raw(
        ped: , 
        boneId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F428D08BE5AAE31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F428D08BE5AAE31u64;

        invoke_raw_typed!(
            hash,
                ped, 
                boneId
        )
    }
}

/// ## Parameters
*



pub fn set_create_random_cops_not_on_scenarios_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A4986851C4EF6E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A4986851C4EF6E7u64;
        
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
pub fn set_create_random_cops_not_on_scenarios_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A4986851C4EF6E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A4986851C4EF6E7u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Min: 0.00  
Max: 10.00  
Can be used in combo with fast run cheat.  
When value is set to 10.00:  
Sprinting without fast run cheat: 66 m/s  
Sprinting with fast run cheat: 77 m/s  
Needs to be looped!  
Note: According to IDA for the Xbox360 xex, when they check bgt they seem to have the min to 0.0f, but the max set to 1.15f not 10.0f.  
```



pub fn set_ped_move_rate_override_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x085BF80FA50A39D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x085BF80FA50A39D1u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_move_rate_override_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x085BF80FA50A39D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x085BF80FA50A39D1u64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// ## Parameters
*



pub fn get_ped_combat_movement_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEA92412FCAEB3F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEA92412FCAEB3F5u64;
        
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
pub fn get_ped_combat_movement_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEA92412FCAEB3F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEA92412FCAEB3F5u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _0xea9960d07dadcf10_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA9960D07DADCF10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA9960D07DADCF10u64;
        
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
pub fn _0xea9960d07dadcf10_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA9960D07DADCF10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA9960D07DADCF10u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_torso_react_ik_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5846EDB26A98A24u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5846EDB26A98A24u64;
        
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
pub fn set_ped_can_torso_react_ik_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF5846EDB26A98A24u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF5846EDB26A98A24u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn get_ped_time_of_death_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E98817B311AE98Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E98817B311AE98Au64;
        
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
pub fn get_ped_time_of_death_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E98817B311AE98Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E98817B311AE98Au64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Input: Haircolor index, value between 0 and 63 (inclusive).
Output: RGB values for the haircolor specified in the input.

This is used with the hair color swatches scaleform.

Use [`_0x013E5CFC38CD5387`](#_0x013E5CFC38CD5387) to get the makeup colors.



pub fn _get_ped_hair_rgb_color_safe(
        
        
            hairColorIndex: 
        , 
        
        
            outR: 
        , 
        
        
            outG: 
        , 
        
        
            outB: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4852FC386E2E1BB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4852FC386E2E1BB5u64;
        
        let result = invoke_raw!(
            hash,
                hairColorIndex, 
                outR, 
                outG, 
                outB
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_ped_hair_rgb_color_raw(
        hairColorIndex: , 
        outR: , 
        outG: , 
        outB: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4852FC386E2E1BB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4852FC386E2E1BB5u64;

        invoke_raw_typed!(
            hash,
                hairColorIndex, 
                outR, 
                outG, 
                outB
        )
    }
}

/// ## Parameters
*



pub fn is_ped_tracked_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C5E1F087CD10BB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C5E1F087CD10BB7u64;
        
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
pub fn is_ped_tracked_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C5E1F087CD10BB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C5E1F087CD10BB7u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
works with TASK::TASK_SET_BLOCKING_OF_NON_TEMPORARY_EVENTS to make a ped completely oblivious to all events going on around him
```



pub fn set_blocking_of_non_temporary_events_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F8AA94D6D97DBF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F8AA94D6D97DBF4u64;
        
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
pub fn set_blocking_of_non_temporary_events_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F8AA94D6D97DBF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F8AA94D6D97DBF4u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x9911f4a24485f653_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9911F4A24485F653u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9911F4A24485F653u64;
        
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
pub fn _0x9911f4a24485f653_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9911F4A24485F653u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9911F4A24485F653u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
state: https://alloc8or.re/gta5/doc/enums/eKnockOffVehicle.txt
```



pub fn set_ped_can_be_knocked_off_vehicle_safe(
        
        
            ped: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A6535691B477C48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A6535691B477C48u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_can_be_knocked_off_vehicle_raw(
        ped: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A6535691B477C48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A6535691B477C48u64;

        invoke_raw_typed!(
            hash,
                ped, 
                state
        )
    }
}

/// ## Parameters
*



pub fn is_ped_running_ragdoll_task_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3B6097CC25AA69Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3B6097CC25AA69Eu64;
        
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
pub fn is_ped_running_ragdoll_task_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3B6097CC25AA69Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3B6097CC25AA69Eu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_ped_is_trying_to_enter_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x814FA8BE5449445Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x814FA8BE5449445Du64;
        
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
pub fn get_vehicle_ped_is_trying_to_enter_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x814FA8BE5449445Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x814FA8BE5449445Du64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_synchronized_scene_rate_safe(
        
        
            sceneID: 
        , 
        
        
            rate: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6C49F8A5E295A5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6C49F8A5E295A5Du64;
        
        let result = invoke_raw!(
            hash,
                sceneID, 
                rate
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_synchronized_scene_rate_raw(
        sceneID: , 
        rate: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6C49F8A5E295A5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6C49F8A5E295A5Du64;

        invoke_raw_typed!(
            hash,
                sceneID, 
                rate
        )
    }
}

/// ## Parameters
*



pub fn _clone_ped_to_target_ex_safe(
        
        
            ped: 
        , 
        
        
            targetPed: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x148B08C2D2ACB884u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x148B08C2D2ACB884u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                targetPed, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _clone_ped_to_target_ex_raw(
        ped: , 
        targetPed: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x148B08C2D2ACB884u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x148B08C2D2ACB884u64;

        invoke_raw_typed!(
            hash,
                ped, 
                targetPed, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn _is_ped_lipstick_color_valid_2_safe(
        
        
            colorId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E802F11FBE27674u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E802F11FBE27674u64;
        
        let result = invoke_raw!(
            hash,
                colorId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_ped_lipstick_color_valid_2_raw(
        colorId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E802F11FBE27674u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E802F11FBE27674u64;

        invoke_raw_typed!(
            hash,
                colorId
        )
    }
}

/// ## Parameters
*



pub fn get_peds_jacker_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B128DC36C1E04CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B128DC36C1E04CFu64;
        
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
pub fn get_peds_jacker_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B128DC36C1E04CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B128DC36C1E04CFu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _does_scenario_blocking_area_exist_safe(
        
        
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A24B067D175A7BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A24B067D175A7BDu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _does_scenario_blocking_area_exist_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A24B067D175A7BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A24B067D175A7BDu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2
        )
    }
}

/// Fires a weapon at a coordinate using a ped.



pub fn set_ped_shoots_at_coord_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96A05E4FB321B1BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96A05E4FB321B1BAu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_shoots_at_coord_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96A05E4FB321B1BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96A05E4FB321B1BAu64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_facial_idle_anim_override_safe(
        
        
            ped: 
        , 
        
        
            animName: 
        , 
        
        
            animDict: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFFC24B988B938B38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFFC24B988B938B38u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                animName, 
                animDict
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_facial_idle_anim_override_raw(
        ped: , 
        animName: , 
        animDict: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFFC24B988B938B38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFFC24B988B938B38u64;

        invoke_raw_typed!(
            hash,
                ped, 
                animName, 
                animDict
        )
    }
}

/// ```
Gets a random ped in the x/y/zRadius near the x/y/z coordinates passed.   
Ped Types:  
Any = -1  
Player = 1  
Male = 4   
Female = 5   
Cop = 6  
Human = 26  
SWAT = 27   
Animal = 28  
Army = 29  
```



pub fn get_random_ped_at_coord_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            xRadius: 
        , 
        
        
            yRadius: 
        , 
        
        
            zRadius: 
        , 
        
        
            pedType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x876046A8E3A4B71Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x876046A8E3A4B71Cu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                xRadius, 
                yRadius, 
                zRadius, 
                pedType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_random_ped_at_coord_raw(
        x: , 
        y: , 
        z: , 
        xRadius: , 
        yRadius: , 
        zRadius: , 
        pedType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x876046A8E3A4B71Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x876046A8E3A4B71Cu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                xRadius, 
                yRadius, 
                zRadius, 
                pedType
        )
    }
}

/// ## Parameters
*



pub fn set_ped_should_play_flee_scenario_exit_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEED8FAFEC331A70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEED8FAFEC331A70u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn set_ped_should_play_flee_scenario_exit_raw(
        ped: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEED8FAFEC331A70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEED8FAFEC331A70u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn is_ped_on_foot_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01FEE67DB37F59B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01FEE67DB37F59B2u64;
        
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
pub fn is_ped_on_foot_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01FEE67DB37F59B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01FEE67DB37F59B2u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Applies lethal damage (FLT_MAX) to the `SKEL_Head` bone of the specified ped using the weapon passed, leading to the
ped's untimely demise.

The naming of the native is a legacy leftover (formerly EXPLODE_CHAR_HEAD in GTA3) as in the early 3D GTA games, lethal
damage to a ped head would 'explode' it.

Do note that this native function does not work in multiplayer/network environment.



pub fn explode_ped_head_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D05CED3A38D0F3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D05CED3A38D0F3Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn explode_ped_head_raw(
        ped: , 
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D05CED3A38D0F3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D05CED3A38D0F3Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash
        )
    }
}

/// ```
It clears the wetness of the selected Ped/Player. Clothes have to be wet to notice the difference.  
```



pub fn clear_ped_wetness_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C720776DAA43E7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C720776DAA43E7Eu64;
        
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
pub fn clear_ped_wetness_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C720776DAA43E7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C720776DAA43E7Eu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Gets the closest ped in a radius.  
Ped Types:  
Any ped = -1  
Player = 1  
Male = 4   
Female = 5   
Cop = 6  
Human = 26  
SWAT = 27   
Animal = 28  
Army = 29



pub fn get_closest_ped_safe(
        
        
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
        
        
            outPed: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        , 
        
        
            pedType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC33AB876A77F8164u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC33AB876A77F8164u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                p4, 
                p5, 
                outPed, 
                p7, 
                p8, 
                pedType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_closest_ped_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        p4: , 
        p5: , 
        outPed: , 
        p7: , 
        p8: , 
        pedType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC33AB876A77F8164u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC33AB876A77F8164u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                p4, 
                p5, 
                outPed, 
                p7, 
                p8, 
                pedType
        )
    }
}

/// ## Parameters
*



pub fn remove_ped_preferred_cover_set_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDDB234CF74073D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDDB234CF74073D9u64;
        
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
pub fn remove_ped_preferred_cover_set_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDDB234CF74073D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDDB234CF74073D9u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _0xfd325494792302d7_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD325494792302D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD325494792302D7u64;
        
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
pub fn _0xfd325494792302d7_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD325494792302D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD325494792302D7u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
Returns whether the entity is in stealth mode  
```



pub fn get_ped_stealth_movement_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C2AC9CA66575FBFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C2AC9CA66575FBFu64;
        
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
pub fn get_ped_stealth_movement_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C2AC9CA66575FBFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C2AC9CA66575FBFu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _0xa52d5247a4227e14_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA52D5247A4227E14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA52D5247A4227E14u64;
        
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
pub fn _0xa52d5247a4227e14_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA52D5247A4227E14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA52D5247A4227E14u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
-1: no landing  
0: landing on both feet  
1: stumbling  
2: rolling  
3: ragdoll  
```



pub fn get_ped_parachute_landing_type_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B9F1FC6AE8166C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B9F1FC6AE8166C0u64;
        
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
pub fn get_ped_parachute_landing_type_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B9F1FC6AE8166C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B9F1FC6AE8166C0u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
angle is ped's view cone  
```



pub fn is_ped_facing_ped_safe(
        
        
            ped: 
        , 
        
        
            otherPed: 
        , 
        
        
            angle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD71649DB0A545AA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD71649DB0A545AA3u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                otherPed, 
                angle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_facing_ped_raw(
        ped: , 
        otherPed: , 
        angle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD71649DB0A545AA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD71649DB0A545AA3u64;

        invoke_raw_typed!(
            hash,
                ped, 
                otherPed, 
                angle
        )
    }
}

/// ## Parameters
*



pub fn is_ped_planting_bomb_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC70B5FAE151982D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC70B5FAE151982D8u64;
        
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
pub fn is_ped_planting_bomb_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC70B5FAE151982D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC70B5FAE151982D8u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// INSTANTLY_FILL_PED_POPULATION native function



pub fn instantly_fill_ped_population_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4759CC730F947C81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4759CC730F947C81u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn instantly_fill_ped_population_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4759CC730F947C81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4759CC730F947C81u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 3: BOOL p2
```



pub fn set_ped_helmet_prop_index_safe(
        
        
            ped: 
        , 
        
        
            propIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26D83693ED99291Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26D83693ED99291Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                propIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_helmet_prop_index_raw(
        ped: , 
        propIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26D83693ED99291Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26D83693ED99291Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                propIndex
        )
    }
}

/// ```
Enables diving motion when underwater.  
```



pub fn set_enable_scuba_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF99F62004024D506u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF99F62004024D506u64;
        
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
pub fn set_enable_scuba_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF99F62004024D506u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF99F62004024D506u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _is_ped_hair_color_valid_safe(
        
        
            colorID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0D36E5D9E99CC21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0D36E5D9E99CC21u64;
        
        let result = invoke_raw!(
            hash,
                colorID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_ped_hair_color_valid_raw(
        colorID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0D36E5D9E99CC21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0D36E5D9E99CC21u64;

        invoke_raw_typed!(
            hash,
                colorID
        )
    }
}

/// RESET_AI_WEAPON_DAMAGE_MODIFIER native function



pub fn reset_ai_weapon_damage_modifier_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA16670E7BA4743Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA16670E7BA4743Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_ai_weapon_damage_modifier_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA16670E7BA4743Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA16670E7BA4743Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Likely a char, if that overlay is not set, e.i. "None" option, returns 255;
This might be the once removed native GET_PED_HEAD_OVERLAY.
```



pub fn _get_ped_head_overlay_value_safe(
        
        
            ped: 
        , 
        
        
            overlayID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA60EF3B6461A4D43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA60EF3B6461A4D43u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                overlayID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_ped_head_overlay_value_raw(
        ped: , 
        overlayID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA60EF3B6461A4D43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA60EF3B6461A4D43u64;

        invoke_raw_typed!(
            hash,
                ped, 
                overlayID
        )
    }
}

/// ```
range 0.0f - 1.0f  
```



pub fn set_driver_aggressiveness_safe(
        
        
            driver: 
        , 
        
        
            aggressiveness: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA731F608CA104E3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA731F608CA104E3Cu64;
        
        let result = invoke_raw!(
            hash,
                driver, 
                aggressiveness
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_driver_aggressiveness_raw(
        driver: , 
        aggressiveness: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA731F608CA104E3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA731F608CA104E3Cu64;

        invoke_raw_typed!(
            hash,
                driver, 
                aggressiveness
        )
    }
}

/// ## Parameters
*



pub fn set_ped_seeing_range_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF29CF591C4BF6CEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF29CF591C4BF6CEEu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_seeing_range_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF29CF591C4BF6CEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF29CF591C4BF6CEEu64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// ## Parameters
*



pub fn set_ped_allow_vehicles_override_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C028C636A414ED9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C028C636A414ED9u64;
        
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
pub fn set_ped_allow_vehicles_override_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C028C636A414ED9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C028C636A414ED9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
Despite this function's name, it simply returns whether the specified handle is a Ped.  
```



pub fn was_ped_skeleton_updated_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11B499C1E0FF8559u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11B499C1E0FF8559u64;
        
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
pub fn was_ped_skeleton_updated_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11B499C1E0FF8559u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11B499C1E0FF8559u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Gets the relationship between two peds. This should be called twice (once for each ped).  
Relationship types:  
0 = Companion  
1 = Respect  
2 = Like  
3 = Neutral  
4 = Dislike  
5 = Hate  
255 = Pedestrians  
(Credits: Inco)  
Example:  
PED::GET_RELATIONSHIP_BETWEEN_PEDS(2, l_1017, 0xA49E591C);  
PED::GET_RELATIONSHIP_BETWEEN_PEDS(2, 0xA49E591C, l_1017);  
```



pub fn get_relationship_between_peds_safe(
        
        
            ped1: 
        , 
        
        
            ped2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBA5AD3A0EAF7121u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBA5AD3A0EAF7121u64;
        
        let result = invoke_raw!(
            hash,
                ped1, 
                ped2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_relationship_between_peds_raw(
        ped1: , 
        ped2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEBA5AD3A0EAF7121u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEBA5AD3A0EAF7121u64;

        invoke_raw_typed!(
            hash,
                ped1, 
                ped2
        )
    }
}

/// ```
stance:  
0 = idle  
1 = walk  
2 = running  
p5 = usually set to true  
```

[Animations list](https://alexguirre.github.io/animations-list/)



pub fn set_ped_alternate_movement_anim_safe(
        
        
            ped: 
        , 
        
        
            stance: 
        , 
        
        
            animDictionary: 
        , 
        
        
            animationName: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90A43CC281FFAB46u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90A43CC281FFAB46u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                stance, 
                animDictionary, 
                animationName, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_alternate_movement_anim_raw(
        ped: , 
        stance: , 
        animDictionary: , 
        animationName: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90A43CC281FFAB46u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90A43CC281FFAB46u64;

        invoke_raw_typed!(
            hash,
                ped, 
                stance, 
                animDictionary, 
                animationName, 
                p4, 
                p5
        )
    }
}

/// ```
This native refers to the field of vision the ped has above them, starting at 0 degrees. 90f would let the ped see enemies directly above of them.  
```



pub fn set_ped_visual_field_max_elevation_angle_safe(
        
        
            ped: 
        , 
        
        
            angle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78D0B67629D75856u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78D0B67629D75856u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                angle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_visual_field_max_elevation_angle_raw(
        ped: , 
        angle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78D0B67629D75856u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78D0B67629D75856u64;

        invoke_raw_typed!(
            hash,
                ped, 
                angle
        )
    }
}

/// ## Parameters
*



pub fn spawnpoints_get_search_result_flags_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB782F8238512BAD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB782F8238512BAD5u64;
        
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
pub fn spawnpoints_get_search_result_flags_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB782F8238512BAD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB782F8238512BAD5u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn clear_ped_env_dirt_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6585D955A68452A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6585D955A68452A5u64;
        
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
pub fn clear_ped_env_dirt_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6585D955A68452A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6585D955A68452A5u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
p2 is usually -1 in the scripts. action is either 0 or "DEFAULT_ACTION".  
```



pub fn set_ped_using_action_mode_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            action: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD75ACCF5E0FB5367u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD75ACCF5E0FB5367u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                p2, 
                action
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_using_action_mode_raw(
        ped: , 
        p1: , 
        p2: , 
        action: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD75ACCF5E0FB5367u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD75ACCF5E0FB5367u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2, 
                action
        )
    }
}

/// Verifies whether a ped is firing within a specific area.



pub fn is_any_ped_shooting_in_area_safe(
        
        
            minX: 
        , 
        
        
            minY: 
        , 
        
        
            minZ: 
        , 
        
        
            maxX: 
        , 
        
        
            maxY: 
        , 
        
        
            maxZ: 
        , 
        
        
            bHighlightArea: 
        , 
        
        
            bDo3DCheck: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0D3D71EA1086C55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0D3D71EA1086C55u64;
        
        let result = invoke_raw!(
            hash,
                minX, 
                minY, 
                minZ, 
                maxX, 
                maxY, 
                maxZ, 
                bHighlightArea, 
                bDo3DCheck
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_any_ped_shooting_in_area_raw(
        minX: , 
        minY: , 
        minZ: , 
        maxX: , 
        maxY: , 
        maxZ: , 
        bHighlightArea: , 
        bDo3DCheck: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0D3D71EA1086C55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0D3D71EA1086C55u64;

        invoke_raw_typed!(
            hash,
                minX, 
                minY, 
                minZ, 
                maxX, 
                maxY, 
                maxZ, 
                bHighlightArea, 
                bDo3DCheck
        )
    }
}

/// ```
In agency_heist3b.c4, its like this 90% of the time:  
PED::_110F526AB784111F(ped, 0.099);  
PED::SET_PED_ENVEFF_SCALE(ped, 1.0);  
PED::_D69411AA0CEBF9E9(ped, 87, 81, 68);  
PED::SET_ENABLE_PED_ENVEFF_SCALE(ped, 1);  
and its like this 10% of the time:  
PED::_110F526AB784111F(ped, 0.2);  
PED::SET_PED_ENVEFF_SCALE(ped, 0.65);  
PED::_D69411AA0CEBF9E9(ped, 74, 69, 60);  
PED::SET_ENABLE_PED_ENVEFF_SCALE(ped, 1);  
```



pub fn _0x110f526ab784111f_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x110F526AB784111Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x110F526AB784111Fu64;
        
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
pub fn _0x110f526ab784111f_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x110F526AB784111Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x110F526AB784111Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
OverlayID ranges from 0 to 12, index from 0 to _GET_NUM_OVERLAY_VALUES(overlayID)-1, and opacity from 0.0 to 1.0.   
overlayID       Part                  Index, to disable  
0               Blemishes             0 - 23, 255  
1               Facial Hair           0 - 28, 255  
2               Eyebrows              0 - 33, 255  
3               Ageing                0 - 14, 255  
4               Makeup                0 - 74, 255  
5               Blush                 0 - 6, 255  
6               Complexion            0 - 11, 255  
7               Sun Damage            0 - 10, 255  
8               Lipstick              0 - 9, 255  
9               Moles/Freckles        0 - 17, 255  
10              Chest Hair            0 - 16, 255  
11              Body Blemishes        0 - 11, 255  
12              Add Body Blemishes    0 - 1, 255  
```



pub fn set_ped_head_overlay_safe(
        
        
            ped: 
        , 
        
        
            overlayID: 
        , 
        
        
            index: 
        , 
        
        
            opacity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48F44967FA05CC1Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48F44967FA05CC1Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                overlayID, 
                index, 
                opacity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_head_overlay_raw(
        ped: , 
        overlayID: , 
        index: , 
        opacity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48F44967FA05CC1Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48F44967FA05CC1Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                overlayID, 
                index, 
                opacity
        )
    }
}

/// ```
Scripts use 0.2, 0.5 and 1.0. Value must be >= 0.0 && <= 1.0
```



pub fn set_driver_racing_modifier_safe(
        
        
            driver: 
        , 
        
        
            modifier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDED5AF5A0EA4B297u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDED5AF5A0EA4B297u64;
        
        let result = invoke_raw!(
            hash,
                driver, 
                modifier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_driver_racing_modifier_raw(
        driver: , 
        modifier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDED5AF5A0EA4B297u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDED5AF5A0EA4B297u64;

        invoke_raw_typed!(
            hash,
                driver, 
                modifier
        )
    }
}

/// ## Parameters
*



pub fn _0xe906ec930f5fe7c8_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE906EC930F5FE7C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE906EC930F5FE7C8u64;
        
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
pub fn _0xe906ec930f5fe7c8_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE906EC930F5FE7C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE906EC930F5FE7C8u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x2dfc81c9b9608549_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DFC81C9B9608549u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DFC81C9B9608549u64;
        
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
pub fn _0x2dfc81c9b9608549_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DFC81C9B9608549u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DFC81C9B9608549u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Return value



pub fn has_pedheadshot_img_upload_failed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x876928DDDFCCC9CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x876928DDDFCCC9CDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_pedheadshot_img_upload_failed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x876928DDDFCCC9CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x876928DDDFCCC9CDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn has_ped_preload_prop_data_finished_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x784002A632822099u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x784002A632822099u64;
        
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
pub fn has_ped_preload_prop_data_finished_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x784002A632822099u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x784002A632822099u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_on_any_bike_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94495889E22C6479u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94495889E22C6479u64;
        
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
pub fn is_ped_on_any_bike_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94495889E22C6479u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94495889E22C6479u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Same function call as PED::GET_MOUNT, aka just returns 0  
```



pub fn is_ped_on_mount_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x460BC76A0E10655Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x460BC76A0E10655Eu64;
        
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
pub fn is_ped_on_mount_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x460BC76A0E10655Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x460BC76A0E10655Eu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn create_parachute_bag_object_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C4F3BF23B6237DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C4F3BF23B6237DBu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_parachute_bag_object_raw(
        ped: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C4F3BF23B6237DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C4F3BF23B6237DBu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn remove_action_mode_asset_safe(
        
        
            asset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13E940F88470FA51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13E940F88470FA51u64;
        
        let result = invoke_raw!(
            hash,
                asset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_action_mode_asset_raw(
        asset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13E940F88470FA51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13E940F88470FA51u64;

        invoke_raw_typed!(
            hash,
                asset
        )
    }
}

/// ```
Type equals 0 for male non-dlc, 1 for female non-dlc, 2 for male dlc, and 3 for female dlc.
Used when calling SET_PED_HEAD_BLEND_DATA.
```



pub fn get_ped_head_blend_first_index_safe(
        
        
            type: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68D353AB88B97E0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68D353AB88B97E0Cu64;
        
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
pub fn get_ped_head_blend_first_index_raw(
        type: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68D353AB88B97E0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68D353AB88B97E0Cu64;

        invoke_raw_typed!(
            hash,
                type
        )
    }
}

/// Clears the blood on a ped.



pub fn clear_ped_blood_damage_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8FE22675A5A45817u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8FE22675A5A45817u64;
        
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
pub fn clear_ped_blood_damage_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8FE22675A5A45817u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8FE22675A5A45817u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn clear_ped_decorations_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E5173C163976E38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E5173C163976E38u64;
        
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
pub fn clear_ped_decorations_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E5173C163976E38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E5173C163976E38u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_cloth_prone_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82A3D6D9CC2CB8E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82A3D6D9CC2CB8E3u64;
        
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
pub fn set_ped_cloth_prone_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82A3D6D9CC2CB8E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82A3D6D9CC2CB8E3u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_ped_helmet_flag_safe(
        
        
            ped: 
        , 
        
        
            helmetFlag: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0E78D5C2CE3EB25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0E78D5C2CE3EB25u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                helmetFlag
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_helmet_flag_raw(
        ped: , 
        helmetFlag: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0E78D5C2CE3EB25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0E78D5C2CE3EB25u64;

        invoke_raw_typed!(
            hash,
                ped, 
                helmetFlag
        )
    }
}

/// ```
scar
blushing
cs_flush_anger
cs_flush_anger_face
bruise
bruise_large
herpes
ArmorBullet
basic_dirt_cloth
basic_dirt_skin
cs_trev1_dirt
```



pub fn apply_ped_damage_decal_safe(
        
        
            ped: 
        , 
        
        
            damageZone: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            heading: 
        , 
        
        
            scale: 
        , 
        
        
            alpha: 
        , 
        
        
            variation: 
        , 
        
        
            fadeIn: 
        , 
        
        
            decalName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x397C38AA7B4A5F83u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x397C38AA7B4A5F83u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                damageZone, 
                xOffset, 
                yOffset, 
                heading, 
                scale, 
                alpha, 
                variation, 
                fadeIn, 
                decalName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn apply_ped_damage_decal_raw(
        ped: , 
        damageZone: , 
        xOffset: , 
        yOffset: , 
        heading: , 
        scale: , 
        alpha: , 
        variation: , 
        fadeIn: , 
        decalName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x397C38AA7B4A5F83u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x397C38AA7B4A5F83u64;

        invoke_raw_typed!(
            hash,
                ped, 
                damageZone, 
                xOffset, 
                yOffset, 
                heading, 
                scale, 
                alpha, 
                variation, 
                fadeIn, 
                decalName
        )
    }
}

/// Sets the palette index of a ped's phone.

| Value | Color      |
| :---: | :-----:    |
|  `0`  | Light Blue |
|  `1`  | Green      |
|  `2`  | Red        |
|  `3`  | Orange     |
|  `4`  | Grey       |
|  `5`  | Purple     |
|  `6`  | Pink       |

```
NativeDB Introduced: v323
```



pub fn set_ped_phone_palette_idx_safe(
        
        
            ped: 
        , 
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83A169EABCDB10A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83A169EABCDB10A2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_phone_palette_idx_raw(
        ped: , 
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83A169EABCDB10A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83A169EABCDB10A2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                index
        )
    }
}

/// ```c
// Potential names and hash collisions included as comments
enum ePedConfigFlags {
	CPED_CONFIG_FLAG_CreatedByFactory = 0,
	CPED_CONFIG_FLAG_CanBeShotInVehicle = 1,
	CPED_CONFIG_FLAG_NoCriticalHits = 2,
	CPED_CONFIG_FLAG_DrownsInWater = 3,
	CPED_CONFIG_FLAG_DrownsInSinkingVehicle = 4,
	CPED_CONFIG_FLAG_DiesInstantlyWhenSwimming = 5,
	CPED_CONFIG_FLAG_HasBulletProofVest = 6,
	CPED_CONFIG_FLAG_UpperBodyDamageAnimsOnly = 7,
	CPED_CONFIG_FLAG_NeverFallOffSkis = 8,
	CPED_CONFIG_FLAG_NeverEverTargetThisPed = 9,
	CPED_CONFIG_FLAG_ThisPedIsATargetPriority = 10,
	CPED_CONFIG_FLAG_TargettableWithNoLos = 11,
	CPED_CONFIG_FLAG_DoesntListenToPlayerGroupCommands = 12,
	CPED_CONFIG_FLAG_NeverLeavesGroup = 13,
	CPED_CONFIG_FLAG_DoesntDropWeaponsWhenDead = 14,
	CPED_CONFIG_FLAG_SetDelayedWeaponAsCurrent = 15,
	CPED_CONFIG_FLAG_KeepTasksAfterCleanUp = 16,
	CPED_CONFIG_FLAG_BlockNonTemporaryEvents = 17,
	CPED_CONFIG_FLAG_HasAScriptBrain = 18,
	CPED_CONFIG_FLAG_WaitingForScriptBrainToLoad = 19,
	CPED_CONFIG_FLAG_AllowMedicsToReviveMe = 20,
	CPED_CONFIG_FLAG_MoneyHasBeenGivenByScript = 21,
	CPED_CONFIG_FLAG_NotAllowedToCrouch = 22,
	CPED_CONFIG_FLAG_DeathPickupsPersist = 23,
	CPED_CONFIG_FLAG_IgnoreSeenMelee = 24,
	CPED_CONFIG_FLAG_ForceDieIfInjured = 25,
	CPED_CONFIG_FLAG_DontDragMeOutCar = 26,
	CPED_CONFIG_FLAG_StayInCarOnJack = 27,
	CPED_CONFIG_FLAG_ForceDieInCar = 28,
	CPED_CONFIG_FLAG_GetOutUndriveableVehicle = 29,
	CPED_CONFIG_FLAG_WillRemainOnBoatAfterMissionEnds = 30,
	CPED_CONFIG_FLAG_DontStoreAsPersistent = 31,
	CPED_CONFIG_FLAG_WillFlyThroughWindscreen = 32,
	CPED_CONFIG_FLAG_DieWhenRagdoll = 33,
	CPED_CONFIG_FLAG_HasHelmet = 34,
	CPED_CONFIG_FLAG_UseHelmet = 35,
	CPED_CONFIG_FLAG_DontTakeOffHelmet = 36,
	CPED_CONFIG_FLAG_HideInCutscene = 37,
	CPED_CONFIG_FLAG_PedIsEnemyToPlayer = 38,
	CPED_CONFIG_FLAG_DisableEvasiveDives = 39,
	CPED_CONFIG_FLAG_PedGeneratesDeadBodyEvents = 40,
	CPED_CONFIG_FLAG_DontAttackPlayerWithoutWantedLevel = 41,
	CPED_CONFIG_FLAG_DontInfluenceWantedLevel = 42,
	CPED_CONFIG_FLAG_DisablePlayerLockon = 43,
	CPED_CONFIG_FLAG_DisableLockonToRandomPeds = 44,
	CPED_CONFIG_FLAG_AllowLockonToFriendlyPlayers = 45,
	_0xDB115BFA = 46,
	CPED_CONFIG_FLAG_PedBeingDeleted = 47,
	CPED_CONFIG_FLAG_BlockWeaponSwitching = 48,
	CPED_CONFIG_FLAG_BlockGroupPedAimedAtResponse = 49,
	CPED_CONFIG_FLAG_WillFollowLeaderAnyMeans = 50,
	CPED_CONFIG_FLAG_BlippedByScript = 51,
	CPED_CONFIG_FLAG_DrawRadarVisualField = 52,
	CPED_CONFIG_FLAG_StopWeaponFiringOnImpact = 53,
	CPED_CONFIG_FLAG_DissableAutoFallOffTests = 54,
	CPED_CONFIG_FLAG_SteerAroundDeadBodies = 55,
	CPED_CONFIG_FLAG_ConstrainToNavMesh = 56,
	CPED_CONFIG_FLAG_SyncingAnimatedProps = 57,
	CPED_CONFIG_FLAG_IsFiring = 58,
	CPED_CONFIG_FLAG_WasFiring = 59,
	CPED_CONFIG_FLAG_IsStanding = 60,
	CPED_CONFIG_FLAG_WasStanding = 61,
	CPED_CONFIG_FLAG_InVehicle = 62,
	CPED_CONFIG_FLAG_OnMount = 63,
	CPED_CONFIG_FLAG_AttachedToVehicle = 64,
	CPED_CONFIG_FLAG_IsSwimming = 65,
	CPED_CONFIG_FLAG_WasSwimming = 66,
	CPED_CONFIG_FLAG_IsSkiing = 67,
	CPED_CONFIG_FLAG_IsSitting = 68,
	CPED_CONFIG_FLAG_KilledByStealth = 69,
	CPED_CONFIG_FLAG_KilledByTakedown = 70,
	CPED_CONFIG_FLAG_Knockedout = 71,
	CPED_CONFIG_FLAG_ClearRadarBlipOnDeath = 72,
	CPED_CONFIG_FLAG_JustGotOffTrain = 73,
	CPED_CONFIG_FLAG_JustGotOnTrain = 74,
	CPED_CONFIG_FLAG_UsingCoverPoint = 75,
	CPED_CONFIG_FLAG_IsInTheAir = 76,
	CPED_CONFIG_FLAG_KnockedUpIntoAir = 77,
	CPED_CONFIG_FLAG_IsAimingGun = 78,
	CPED_CONFIG_FLAG_HasJustLeftCar = 79,
	CPED_CONFIG_FLAG_TargetWhenInjuredAllowed = 80,
	CPED_CONFIG_FLAG_CurrLeftFootCollNM = 81,
	CPED_CONFIG_FLAG_PrevLeftFootCollNM = 82,
	CPED_CONFIG_FLAG_CurrRightFootCollNM = 83,
	CPED_CONFIG_FLAG_PrevRightFootCollNM = 84,
	CPED_CONFIG_FLAG_HasBeenBumpedInCar = 85,
	CPED_CONFIG_FLAG_InWaterTaskQuitToClimbLadder = 86,
	CPED_CONFIG_FLAG_NMTwoHandedWeaponBothHandsConstrained = 87,
	CPED_CONFIG_FLAG_CreatedBloodPoolTimer = 88,
	CPED_CONFIG_FLAG_DontActivateRagdollFromAnyPedImpact = 89,
	CPED_CONFIG_FLAG_GroupPedFailedToEnterCover = 90,
	CPED_CONFIG_FLAG_AlreadyChattedOnPhone = 91,
	CPED_CONFIG_FLAG_AlreadyReactedToPedOnRoof = 92,
	CPED_CONFIG_FLAG_ForcePedLoadCover = 93,
	CPED_CONFIG_FLAG_BlockCoweringInCover = 94,
	CPED_CONFIG_FLAG_BlockPeekingInCover = 95,
	CPED_CONFIG_FLAG_JustLeftCarNotCheckedForDoors = 96,
	CPED_CONFIG_FLAG_VaultFromCover = 97,
	CPED_CONFIG_FLAG_AutoConversationLookAts = 98,
	CPED_CONFIG_FLAG_UsingCrouchedPedCapsule = 99,
	CPED_CONFIG_FLAG_HasDeadPedBeenReported = 100,
	CPED_CONFIG_FLAG_ForcedAim = 101,
	CPED_CONFIG_FLAG_SteersAroundPeds = 102,
	CPED_CONFIG_FLAG_SteersAroundObjects = 103,
	CPED_CONFIG_FLAG_OpenDoorArmIK = 104,
	CPED_CONFIG_FLAG_ForceReload = 105,
	CPED_CONFIG_FLAG_DontActivateRagdollFromVehicleImpact = 106,
	CPED_CONFIG_FLAG_DontActivateRagdollFromBulletImpact = 107,
	CPED_CONFIG_FLAG_DontActivateRagdollFromExplosions = 108,
	CPED_CONFIG_FLAG_DontActivateRagdollFromFire = 109,
	CPED_CONFIG_FLAG_DontActivateRagdollFromElectrocution = 110,
	CPED_CONFIG_FLAG_IsBeingDraggedToSafety = 111,
	CPED_CONFIG_FLAG_HasBeenDraggedToSafety = 112,
	CPED_CONFIG_FLAG_KeepWeaponHolsteredUnlessFired = 113,
	CPED_CONFIG_FLAG_ForceScriptControlledKnockout = 114,
	CPED_CONFIG_FLAG_FallOutOfVehicleWhenKilled = 115,
	CPED_CONFIG_FLAG_GetOutBurningVehicle = 116,
	CPED_CONFIG_FLAG_BumpedByPlayer = 117,
	CPED_CONFIG_FLAG_RunFromFiresAndExplosions = 118,
	CPED_CONFIG_FLAG_TreatAsPlayerDuringTargeting = 119,
	CPED_CONFIG_FLAG_IsHandCuffed = 120,
	CPED_CONFIG_FLAG_IsAnkleCuffed = 121,
	CPED_CONFIG_FLAG_DisableMelee = 122,
	CPED_CONFIG_FLAG_DisableUnarmedDrivebys = 123,
	CPED_CONFIG_FLAG_JustGetsPulledOutWhenElectrocuted = 124,
	CPED_CONFIG_FLAG_UNUSED_REPLACE_ME = 125,
	CPED_CONFIG_FLAG_WillNotHotwireLawEnforcementVehicle = 126,
	CPED_CONFIG_FLAG_WillCommandeerRatherThanJack = 127,
	CPED_CONFIG_FLAG_CanBeAgitated = 128,
	CPED_CONFIG_FLAG_ForcePedToFaceLeftInCover = 129,
	CPED_CONFIG_FLAG_ForcePedToFaceRightInCover = 130,
	CPED_CONFIG_FLAG_BlockPedFromTurningInCover = 131,
	CPED_CONFIG_FLAG_KeepRelationshipGroupAfterCleanUp = 132,
	CPED_CONFIG_FLAG_ForcePedToBeDragged = 133,
	CPED_CONFIG_FLAG_PreventPedFromReactingToBeingJacked = 134,
	CPED_CONFIG_FLAG_IsScuba = 135,
	CPED_CONFIG_FLAG_WillArrestRatherThanJack = 136,
	CPED_CONFIG_FLAG_RemoveDeadExtraFarAway = 137,
	CPED_CONFIG_FLAG_RidingTrain = 138,
	CPED_CONFIG_FLAG_ArrestResult = 139,
	CPED_CONFIG_FLAG_CanAttackFriendly = 140,
	CPED_CONFIG_FLAG_WillJackAnyPlayer = 141,
	CPED_CONFIG_FLAG_BumpedByPlayerVehicle = 142,
	CPED_CONFIG_FLAG_DodgedPlayerVehicle = 143,
	CPED_CONFIG_FLAG_WillJackWantedPlayersRatherThanStealCar = 144,
	CPED_CONFIG_FLAG_NoCopWantedAggro = 145,
	CPED_CONFIG_FLAG_DisableLadderClimbing = 146,
	CPED_CONFIG_FLAG_StairsDetected = 147,
	CPED_CONFIG_FLAG_SlopeDetected = 148,
	CPED_CONFIG_FLAG_HelmetHasBeenShot = 149,
	CPED_CONFIG_FLAG_CowerInsteadOfFlee = 150,
	CPED_CONFIG_FLAG_CanActivateRagdollWhenVehicleUpsideDown = 151,
	CPED_CONFIG_FLAG_AlwaysRespondToCriesForHelp = 152,
	CPED_CONFIG_FLAG_DisableBloodPoolCreation = 153,
	CPED_CONFIG_FLAG_ShouldFixIfNoCollision = 154,
	CPED_CONFIG_FLAG_CanPerformArrest = 155,
	CPED_CONFIG_FLAG_CanPerformUncuff = 156,
	CPED_CONFIG_FLAG_CanBeArrested = 157,
	CPED_CONFIG_FLAG_MoverConstrictedByOpposingCollisions = 158,
	CPED_CONFIG_FLAG_PlayerPreferFrontSeatMP = 159,
	CPED_CONFIG_FLAG_DontActivateRagdollFromImpactObject = 160,
	CPED_CONFIG_FLAG_DontActivateRagdollFromMelee = 161,
	CPED_CONFIG_FLAG_DontActivateRagdollFromWaterJet = 162,
	CPED_CONFIG_FLAG_DontActivateRagdollFromDrowning = 163,
	CPED_CONFIG_FLAG_DontActivateRagdollFromFalling = 164,
	CPED_CONFIG_FLAG_DontActivateRagdollFromRubberBullet = 165,
	CPED_CONFIG_FLAG_IsInjured = 166,
	CPED_CONFIG_FLAG_DontEnterVehiclesInPlayersGroup = 167,
	CPED_CONFIG_FLAG_SwimmingTasksRunning = 168,
	CPED_CONFIG_FLAG_PreventAllMeleeTaunts = 169,
	CPED_CONFIG_FLAG_ForceDirectEntry = 170,
	CPED_CONFIG_FLAG_AlwaysSeeApproachingVehicles = 171,
	CPED_CONFIG_FLAG_CanDiveAwayFromApproachingVehicles = 172,
	CPED_CONFIG_FLAG_AllowPlayerToInterruptVehicleEntryExit = 173,
	CPED_CONFIG_FLAG_OnlyAttackLawIfPlayerIsWanted = 174,
	CPED_CONFIG_FLAG_PlayerInContactWithKinematicPed = 175,
	CPED_CONFIG_FLAG_PlayerInContactWithSomethingOtherThanKinematicPed = 176,
	CPED_CONFIG_FLAG_PedsJackingMeDontGetIn = 177,
	CPED_CONFIG_FLAG_AdditionalRappellingPed = 178,
	CPED_CONFIG_FLAG_PedIgnoresAnimInterruptEvents = 179,
	CPED_CONFIG_FLAG_IsInCustody = 180,
	CPED_CONFIG_FLAG_ForceStandardBumpReactionThresholds = 181,
	CPED_CONFIG_FLAG_LawWillOnlyAttackIfPlayerIsWanted = 182,
	CPED_CONFIG_FLAG_IsAgitated = 183,
	CPED_CONFIG_FLAG_PreventAutoShuffleToDriversSeat = 184,
	CPED_CONFIG_FLAG_UseKinematicModeWhenStationary = 185,
	CPED_CONFIG_FLAG_EnableWeaponBlocking = 186,
	CPED_CONFIG_FLAG_HasHurtStarted = 187,
	CPED_CONFIG_FLAG_DisableHurt = 188,
	CPED_CONFIG_FLAG_PlayerIsWeird = 189,
	CPED_CONFIG_FLAG_PedHadPhoneConversation = 190,
	CPED_CONFIG_FLAG_BeganCrossingRoad = 191,
	CPED_CONFIG_FLAG_WarpIntoLeadersVehicle = 192,
	CPED_CONFIG_FLAG_DoNothingWhenOnFootByDefault = 193,
	CPED_CONFIG_FLAG_UsingScenario = 194,
	CPED_CONFIG_FLAG_VisibleOnScreen = 195,
	CPED_CONFIG_FLAG_DontCollideWithKinematic = 196,
	CPED_CONFIG_FLAG_ActivateOnSwitchFromLowPhysicsLod = 197,
	CPED_CONFIG_FLAG_DontActivateRagdollOnPedCollisionWhenDead = 198,
	CPED_CONFIG_FLAG_DontActivateRagdollOnVehicleCollisionWhenDead = 199,
	CPED_CONFIG_FLAG_HasBeenInArmedCombat = 200,
	CPED_CONFIG_FLAG_UseDiminishingAmmoRate = 201,
	CPED_CONFIG_FLAG_Avoidance_Ignore_All = 202,
	CPED_CONFIG_FLAG_Avoidance_Ignored_by_All = 203,
	CPED_CONFIG_FLAG_Avoidance_Ignore_Group1 = 204,
	CPED_CONFIG_FLAG_Avoidance_Member_of_Group1 = 205,
	CPED_CONFIG_FLAG_ForcedToUseSpecificGroupSeatIndex = 206,
	CPED_CONFIG_FLAG_LowPhysicsLodMayPlaceOnNavMesh = 207,
	CPED_CONFIG_FLAG_DisableExplosionReactions = 208,
	CPED_CONFIG_FLAG_DodgedPlayer = 209,
	CPED_CONFIG_FLAG_WaitingForPlayerControlInterrupt = 210,
	CPED_CONFIG_FLAG_ForcedToStayInCover = 211,
	CPED_CONFIG_FLAG_GeneratesSoundEvents = 212,
	CPED_CONFIG_FLAG_ListensToSoundEvents = 213,
	CPED_CONFIG_FLAG_AllowToBeTargetedInAVehicle = 214,
	CPED_CONFIG_FLAG_WaitForDirectEntryPointToBeFreeWhenExiting = 215,
	CPED_CONFIG_FLAG_OnlyRequireOnePressToExitVehicle = 216,
	CPED_CONFIG_FLAG_ForceExitToSkyDive = 217,
	CPED_CONFIG_FLAG_SteersAroundVehicles = 218,
	CPED_CONFIG_FLAG_AllowPedInVehiclesOverrideTaskFlags = 219,
	CPED_CONFIG_FLAG_DontEnterLeadersVehicle = 220,
	CPED_CONFIG_FLAG_DisableExitToSkyDive = 221,
	CPED_CONFIG_FLAG_ScriptHasDisabledCollision = 222,
	CPED_CONFIG_FLAG_UseAmbientModelScaling = 223,
	CPED_CONFIG_FLAG_DontWatchFirstOnNextHurryAway = 224,
	CPED_CONFIG_FLAG_DisablePotentialToBeWalkedIntoResponse = 225,
	CPED_CONFIG_FLAG_DisablePedAvoidance = 226,
	CPED_CONFIG_FLAG_ForceRagdollUponDeath = 227,
	CPED_CONFIG_FLAG_CanLosePropsOnDamage = 228,
	CPED_CONFIG_FLAG_DisablePanicInVehicle = 229,
	CPED_CONFIG_FLAG_AllowedToDetachTrailer = 230,
	CPED_CONFIG_FLAG_HasShotBeenReactedToFromFront = 231,
	CPED_CONFIG_FLAG_HasShotBeenReactedToFromBack = 232,
	CPED_CONFIG_FLAG_HasShotBeenReactedToFromLeft = 233,
	CPED_CONFIG_FLAG_HasShotBeenReactedToFromRight = 234,
	CPED_CONFIG_FLAG_AllowBlockDeadPedRagdollActivation = 235,
	CPED_CONFIG_FLAG_IsHoldingProp = 236,
	CPED_CONFIG_FLAG_BlocksPathingWhenDead = 237,
	CPED_CONFIG_FLAG_ForcePlayNormalScenarioExitOnNextScriptCommand = 238,
	CPED_CONFIG_FLAG_ForcePlayImmediateScenarioExitOnNextScriptCommand = 239,
	CPED_CONFIG_FLAG_ForceSkinCharacterCloth = 240,
	CPED_CONFIG_FLAG_LeaveEngineOnWhenExitingVehicles = 241,
	CPED_CONFIG_FLAG_PhoneDisableTextingAnimations = 242,
	CPED_CONFIG_FLAG_PhoneDisableTalkingAnimations = 243,
	CPED_CONFIG_FLAG_PhoneDisableCameraAnimations = 244,
	CPED_CONFIG_FLAG_DisableBlindFiringInShotReactions = 245,
	CPED_CONFIG_FLAG_AllowNearbyCoverUsage = 246,
	CPED_CONFIG_FLAG_InStrafeTransition = 247,
	CPED_CONFIG_FLAG_CanPlayInCarIdles = 248,
	CPED_CONFIG_FLAG_CanAttackNonWantedPlayerAsLaw = 249,
	CPED_CONFIG_FLAG_WillTakeDamageWhenVehicleCrashes = 250,
	CPED_CONFIG_FLAG_AICanDrivePlayerAsRearPassenger = 251,
	CPED_CONFIG_FLAG_PlayerCanJackFriendlyPlayers = 252,
	CPED_CONFIG_FLAG_OnStairs = 253,
	CPED_CONFIG_FLAG_SimulatingAiming = 254,
	CPED_CONFIG_FLAG_AIDriverAllowFriendlyPassengerSeatEntry = 255,
	CPED_CONFIG_FLAG_ParentCarIsBeingRemoved = 256,
	CPED_CONFIG_FLAG_AllowMissionPedToUseInjuredMovement = 257,
	CPED_CONFIG_FLAG_CanLoseHelmetOnDamage = 258,
	CPED_CONFIG_FLAG_NeverDoScenarioExitProbeChecks = 259,
	CPED_CONFIG_FLAG_SuppressLowLODRagdollSwitchWhenCorpseSettles = 260,
	CPED_CONFIG_FLAG_PreventUsingLowerPrioritySeats = 261,
	CPED_CONFIG_FLAG_JustLeftVehicleNeedsReset = 262,
	CPED_CONFIG_FLAG_TeleportIfCantReachPlayer = 263,
	CPED_CONFIG_FLAG_PedsInVehiclePositionNeedsReset = 264,
	CPED_CONFIG_FLAG_PedsFullyInSeat = 265,
	CPED_CONFIG_FLAG_AllowPlayerLockOnIfFriendly = 266,
	CPED_CONFIG_FLAG_UseCameraHeadingForDesiredDirectionLockOnTest = 267,
	CPED_CONFIG_FLAG_TeleportToLeaderVehicle = 268,
	CPED_CONFIG_FLAG_Avoidance_Ignore_WeirdPedBuffer = 269,
	CPED_CONFIG_FLAG_OnStairSlope = 270,
	CPED_CONFIG_FLAG_HasPlayedNMGetup = 271,
	CPED_CONFIG_FLAG_DontBlipCop = 272,
	CPED_CONFIG_FLAG_SpawnedAtExtendedRangeScenario = 273,
	CPED_CONFIG_FLAG_WalkAlongsideLeaderWhenClose = 274,
	CPED_CONFIG_FLAG_KillWhenTrapped = 275,
	CPED_CONFIG_FLAG_EdgeDetected = 276,
	CPED_CONFIG_FLAG_AlwaysWakeUpPhysicsOfIntersectedPeds = 277,
	CPED_CONFIG_FLAG_EquippedAmbientLoadOutWeapon = 278,
	CPED_CONFIG_FLAG_AvoidTearGas = 279,
	CPED_CONFIG_FLAG_StoppedSpeechUponFreezing = 280,
	CPED_CONFIG_FLAG_DisableGoToWritheWhenInjured = 281,
	CPED_CONFIG_FLAG_OnlyUseForcedSeatWhenEnteringHeliInGroup = 282,
	CPED_CONFIG_FLAG_ThrownFromVehicleDueToExhaustion = 283,
	CPED_CONFIG_FLAG_UpdateEnclosedSearchRegion = 284,
	CPED_CONFIG_FLAG_DisableWeirdPedEvents = 285,
	CPED_CONFIG_FLAG_ShouldChargeNow = 286,
	CPED_CONFIG_FLAG_RagdollingOnBoat = 287,
	CPED_CONFIG_FLAG_HasBrandishedWeapon = 288,
	CPED_CONFIG_FLAG_AllowMinorReactionsAsMissionPed = 289,
	CPED_CONFIG_FLAG_BlockDeadBodyShockingEventsWhenDead = 290,
	CPED_CONFIG_FLAG_PedHasBeenSeen = 291,
	CPED_CONFIG_FLAG_PedIsInReusePool = 292,
	CPED_CONFIG_FLAG_PedWasReused = 293,
	CPED_CONFIG_FLAG_DisableShockingEvents = 294,
	CPED_CONFIG_FLAG_MovedUsingLowLodPhysicsSinceLastActive = 295,
	CPED_CONFIG_FLAG_NeverReactToPedOnRoof = 296,
	CPED_CONFIG_FLAG_ForcePlayFleeScenarioExitOnNextScriptCommand = 297,
	CPED_CONFIG_FLAG_JustBumpedIntoVehicle = 298,
	CPED_CONFIG_FLAG_DisableShockingDrivingOnPavementEvents = 299,
	CPED_CONFIG_FLAG_ShouldThrowSmokeNow = 300,
	CPED_CONFIG_FLAG_DisablePedConstraints = 301,
	CPED_CONFIG_FLAG_ForceInitialPeekInCover = 302,
	CPED_CONFIG_FLAG_CreatedByDispatch = 303,
	CPED_CONFIG_FLAG_PointGunLeftHandSupporting = 304,
	CPED_CONFIG_FLAG_DisableJumpingFromVehiclesAfterLeader = 305,
	CPED_CONFIG_FLAG_DontActivateRagdollFromPlayerPedImpact = 306,
	CPED_CONFIG_FLAG_DontActivateRagdollFromAiRagdollImpact = 307,
	CPED_CONFIG_FLAG_DontActivateRagdollFromPlayerRagdollImpact = 308,
	CPED_CONFIG_FLAG_DisableQuadrupedSpring = 309,
	CPED_CONFIG_FLAG_IsInCluster = 310,
	CPED_CONFIG_FLAG_ShoutToGroupOnPlayerMelee = 311,
	CPED_CONFIG_FLAG_IgnoredByAutoOpenDoors = 312,
	CPED_CONFIG_FLAG_PreferInjuredGetup = 313,
	CPED_CONFIG_FLAG_ForceIgnoreMeleeActiveCombatant = 314,
	CPED_CONFIG_FLAG_CheckLoSForSoundEvents = 315,
	CPED_CONFIG_FLAG_JackedAbandonedCar = 316,
	CPED_CONFIG_FLAG_CanSayFollowedByPlayerAudio = 317,
	CPED_CONFIG_FLAG_ActivateRagdollFromMinorPlayerContact = 318,
	CPED_CONFIG_FLAG_HasPortablePickupAttached = 319,
	CPED_CONFIG_FLAG_ForcePoseCharacterCloth = 320,
	CPED_CONFIG_FLAG_HasClothCollisionBounds = 321,
	CPED_CONFIG_FLAG_HasHighHeels = 322,
	CPED_CONFIG_FLAG_TreatAsAmbientPedForDriverLockOn = 323,
	CPED_CONFIG_FLAG_DontBehaveLikeLaw = 324,
	CPED_CONFIG_FLAG_SpawnedAtScenario = 325,
	CPED_CONFIG_FLAG_DisablePoliceInvestigatingBody = 326,
	CPED_CONFIG_FLAG_DisableWritheShootFromGround = 327,
	CPED_CONFIG_FLAG_LowerPriorityOfWarpSeats = 328,
	CPED_CONFIG_FLAG_DisableTalkTo = 329,
	CPED_CONFIG_FLAG_DontBlip = 330,
	CPED_CONFIG_FLAG_IsSwitchingWeapon = 331,
	CPED_CONFIG_FLAG_IgnoreLegIkRestrictions = 332,
	CPED_CONFIG_FLAG_ScriptForceNoTimesliceIntelligenceUpdate = 333,
	CPED_CONFIG_FLAG_JackedOutOfMyVehicle = 334,
	CPED_CONFIG_FLAG_WentIntoCombatAfterBeingJacked = 335,
	CPED_CONFIG_FLAG_DontActivateRagdollForVehicleGrab = 336,
	CPED_CONFIG_FLAG_ForcePackageCharacterCloth = 337,
	CPED_CONFIG_FLAG_DontRemoveWithValidOrder = 338,
	CPED_CONFIG_FLAG_AllowTaskDoNothingTimeslicing = 339,
	CPED_CONFIG_FLAG_ForcedToStayInCoverDueToPlayerSwitch = 340,
	CPED_CONFIG_FLAG_ForceProneCharacterCloth = 341,
	CPED_CONFIG_FLAG_NotAllowedToJackAnyPlayers = 342,
	CPED_CONFIG_FLAG_InToStrafeTransition = 343,
	CPED_CONFIG_FLAG_KilledByStandardMelee = 344,
	CPED_CONFIG_FLAG_AlwaysLeaveTrainUponArrival = 345,
	CPED_CONFIG_FLAG_ForcePlayDirectedNormalScenarioExitOnNextScriptCommand = 346,
	CPED_CONFIG_FLAG_OnlyWritheFromWeaponDamage = 347,
	CPED_CONFIG_FLAG_UseSloMoBloodVfx = 348,
	CPED_CONFIG_FLAG_EquipJetpack = 349,
	CPED_CONFIG_FLAG_PreventDraggedOutOfCarThreatResponse = 350,
	CPED_CONFIG_FLAG_ScriptHasCompletelyDisabledCollision = 351,
	CPED_CONFIG_FLAG_NeverDoScenarioNavChecks = 352,
	CPED_CONFIG_FLAG_ForceSynchronousScenarioExitChecking = 353,
	CPED_CONFIG_FLAG_ThrowingGrenadeWhileAiming = 354,
	CPED_CONFIG_FLAG_HeadbobToRadioEnabled = 355,
	CPED_CONFIG_FLAG_ForceDeepSurfaceCheck = 356,
	CPED_CONFIG_FLAG_DisableDeepSurfaceAnims = 357,
	CPED_CONFIG_FLAG_DontBlipNotSynced = 358,
	CPED_CONFIG_FLAG_IsDuckingInVehicle = 359,
	CPED_CONFIG_FLAG_PreventAutoShuffleToTurretSeat = 360,
	CPED_CONFIG_FLAG_DisableEventInteriorStatusCheck = 361,
	CPED_CONFIG_FLAG_HasReserveParachute = 362,
	CPED_CONFIG_FLAG_UseReserveParachute = 363,
	CPED_CONFIG_FLAG_TreatDislikeAsHateWhenInCombat = 364,
	CPED_CONFIG_FLAG_OnlyUpdateTargetWantedIfSeen = 365,
	CPED_CONFIG_FLAG_AllowAutoShuffleToDriversSeat = 366,
	CPED_CONFIG_FLAG_DontActivateRagdollFromSmokeGrenade = 367,
	CPED_CONFIG_FLAG_LinkMBRToOwnerOnChain = 368,
	CPED_CONFIG_FLAG_AmbientFriendBumpedByPlayer = 369,
	CPED_CONFIG_FLAG_AmbientFriendBumpedByPlayerVehicle = 370,
	CPED_CONFIG_FLAG_InFPSUnholsterTransition = 371,
	CPED_CONFIG_FLAG_PreventReactingToSilencedCloneBullets = 372,
	CPED_CONFIG_FLAG_DisableInjuredCryForHelpEvents = 373,
	CPED_CONFIG_FLAG_NeverLeaveTrain = 374,
	CPED_CONFIG_FLAG_DontDropJetpackOnDeath = 375,
	CPED_CONFIG_FLAG_UseFPSUnholsterTransitionDuringCombatRoll = 376,
	CPED_CONFIG_FLAG_ExitingFPSCombatRoll = 377,
	CPED_CONFIG_FLAG_ScriptHasControlOfPlayer = 378,
	CPED_CONFIG_FLAG_PlayFPSIdleFidgetsForProjectile = 379,
	CPED_CONFIG_FLAG_DisableAutoEquipHelmetsInBikes = 380,
	CPED_CONFIG_FLAG_DisableAutoEquipHelmetsInAircraft = 381,
	CPED_CONFIG_FLAG_WasPlayingFPSGetup = 382,
	CPED_CONFIG_FLAG_WasPlayingFPSMeleeActionResult = 383,
	CPED_CONFIG_FLAG_PreferNoPriorityRemoval = 384,
	CPED_CONFIG_FLAG_FPSFidgetsAbortedOnFire = 385,
	CPED_CONFIG_FLAG_ForceFPSIKWithUpperBodyAnim = 386,
	CPED_CONFIG_FLAG_SwitchingCharactersInFirstPerson = 387,
	CPED_CONFIG_FLAG_IsClimbingLadder = 388,
	CPED_CONFIG_FLAG_HasBareFeet = 389,
	CPED_CONFIG_FLAG_UNUSED_REPLACE_ME_2 = 390,
	CPED_CONFIG_FLAG_GoOnWithoutVehicleIfItIsUnableToGetBackToRoad = 391,
	CPED_CONFIG_FLAG_BlockDroppingHealthSnacksOnDeath = 392,
	CPED_CONFIG_FLAG_ResetLastVehicleOnVehicleExit = 393,
	CPED_CONFIG_FLAG_ForceThreatResponseToNonFriendToFriendMeleeActions = 394,
	CPED_CONFIG_FLAG_DontRespondToRandomPedsDamage = 395,
	CPED_CONFIG_FLAG_AllowContinuousThreatResponseWantedLevelUpdates = 396,
	CPED_CONFIG_FLAG_KeepTargetLossResponseOnCleanup = 397,
	CPED_CONFIG_FLAG_PlayersDontDragMeOutOfCar = 398,
	CPED_CONFIG_FLAG_BroadcastRepondedToThreatWhenGoingToPointShooting = 399,
	CPED_CONFIG_FLAG_IgnorePedTypeForIsFriendlyWith = 400,
	CPED_CONFIG_FLAG_TreatNonFriendlyAsHateWhenInCombat = 401,
	CPED_CONFIG_FLAG_DontLeaveVehicleIfLeaderNotInVehicle = 402,
	CPED_CONFIG_FLAG_ChangeFromPermanentToAmbientPopTypeOnMigration = 403,
	CPED_CONFIG_FLAG_AllowMeleeReactionIfMeleeProofIsOn = 404,
	CPED_CONFIG_FLAG_UsingLowriderLeans = 405,
	CPED_CONFIG_FLAG_UsingAlternateLowriderLeans = 406,
	CPED_CONFIG_FLAG_UseNormalExplosionDamageWhenBlownUpInVehicle = 407,
	CPED_CONFIG_FLAG_DisableHomingMissileLockForVehiclePedInside = 408,
	CPED_CONFIG_FLAG_DisableTakeOffScubaGear = 409,
	CPED_CONFIG_FLAG_IgnoreMeleeFistWeaponDamageMult = 410,
	CPED_CONFIG_FLAG_LawPedsCanFleeFromNonWantedPlayer = 411,
	CPED_CONFIG_FLAG_ForceBlipSecurityPedsIfPlayerIsWanted = 412,
	CPED_CONFIG_FLAG_IsHolsteringWeapon = 413,
	CPED_CONFIG_FLAG_UseGoToPointForScenarioNavigation = 414,
	CPED_CONFIG_FLAG_DontClearLocalPassengersWantedLevel = 415,
	CPED_CONFIG_FLAG_BlockAutoSwapOnWeaponPickups = 416,
	CPED_CONFIG_FLAG_ThisPedIsATargetPriorityForAI = 417,
	CPED_CONFIG_FLAG_IsSwitchingHelmetVisor = 418,
	CPED_CONFIG_FLAG_ForceHelmetVisorSwitch = 419,
	CPED_CONFIG_FLAG_IsPerformingVehicleMelee = 420,
	CPED_CONFIG_FLAG_UseOverrideFootstepPtFx = 421,
	CPED_CONFIG_FLAG_DisableVehicleCombat = 422,
	CPED_CONFIG_FLAG_TreatAsFriendlyForTargetingAndDamage = 423,
	CPED_CONFIG_FLAG_AllowBikeAlternateAnimations = 424,
	CPED_CONFIG_FLAG_TreatAsFriendlyForTargetingAndDamageNonSynced = 425,
	CPED_CONFIG_FLAG_UseLockpickVehicleEntryAnimations = 426,
	CPED_CONFIG_FLAG_IgnoreInteriorCheckForSprinting = 427,
	CPED_CONFIG_FLAG_SwatHeliSpawnWithinLastSpottedLocation = 428,
	CPED_CONFIG_FLAG_DisableStartEngine = 429,
	CPED_CONFIG_FLAG_IgnoreBeingOnFire = 430,
	CPED_CONFIG_FLAG_DisableTurretOrRearSeatPreference = 431,
	CPED_CONFIG_FLAG_DisableWantedHelicopterSpawning = 432,
	CPED_CONFIG_FLAG_UseTargetPerceptionForCreatingAimedAtEvents = 433,
	CPED_CONFIG_FLAG_DisableHomingMissileLockon = 434,
	CPED_CONFIG_FLAG_ForceIgnoreMaxMeleeActiveSupportCombatants = 435,
	CPED_CONFIG_FLAG_StayInDefensiveAreaWhenInVehicle = 436,
	CPED_CONFIG_FLAG_DontShoutTargetPosition = 437,
	CPED_CONFIG_FLAG_DisableHelmetArmor = 438,
	CPED_CONFIG_FLAG_CreatedByConcealedPlayer = 439,
	CPED_CONFIG_FLAG_PermanentlyDisablePotentialToBeWalkedIntoResponse = 440,
	CPED_CONFIG_FLAG_PreventVehExitDueToInvalidWeapon = 441,
	CPED_CONFIG_FLAG_IgnoreNetSessionFriendlyFireCheckForAllowDamage = 442,
	CPED_CONFIG_FLAG_DontLeaveCombatIfTargetPlayerIsAttackedByPolice = 443,
	CPED_CONFIG_FLAG_CheckLockedBeforeWarp = 444,
	CPED_CONFIG_FLAG_DontShuffleInVehicleToMakeRoom = 445,
	CPED_CONFIG_FLAG_GiveWeaponOnGetup = 446,
	CPED_CONFIG_FLAG_DontHitVehicleWithProjectiles = 447,
	CPED_CONFIG_FLAG_DisableForcedEntryForOpenVehiclesFromTryLockedDoor = 448,
	CPED_CONFIG_FLAG_FiresDummyRockets = 449,
	CPED_CONFIG_FLAG_PedIsArresting = 450,
	CPED_CONFIG_FLAG_IsDecoyPed = 451,
	CPED_CONFIG_FLAG_HasEstablishedDecoy = 452,
	CPED_CONFIG_FLAG_BlockDispatchedHelicoptersFromLanding = 453,
	CPED_CONFIG_FLAG_DontCryForHelpOnStun = 454,
	CPED_CONFIG_FLAG_HitByTranqWeapon = 455,
	CPED_CONFIG_FLAG_CanBeIncapacitated = 456,
	CPED_CONFIG_FLAG_ForcedAimFromArrest = 457,
	CPED_CONFIG_FLAG_DontChangeTargetFromMelee = 458,
	_0x4376ABF2 = 459,
	CPED_CONFIG_FLAG_RagdollFloatsIndefinitely = 460,
	CPED_CONFIG_FLAG_BlockElectricWeaponDamage = 461,
	_0x262A3B8E = 462,
	_0x1AA79A25 = 463,
}
```



pub fn set_ped_config_flag_safe(
        
        
            ped: 
        , 
        
        
            flagId: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1913FE4CBF41C463u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1913FE4CBF41C463u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                flagId, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_config_flag_raw(
        ped: , 
        flagId: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1913FE4CBF41C463u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1913FE4CBF41C463u64;

        invoke_raw_typed!(
            hash,
                ped, 
                flagId, 
                value
        )
    }
}

/// ```
Sends the message that was created by a call to CREATE_NM_MESSAGE to the specified Ped.  
If a message hasn't been created already, this function does nothing.  
If the Ped is not ragdolled with Euphoria enabled, this function does nothing.  
The following call can be used to ragdoll the Ped with Euphoria enabled: SET_PED_TO_RAGDOLL(ped, 4000, 5000, 1, 1, 1, 0);  
Call order:  
SET_PED_TO_RAGDOLL  
CREATE_NM_MESSAGE  
GIVE_PED_NM_MESSAGE  
Multiple messages can be chained. Eg. to make the ped stagger and swing his arms around, the following calls can be made:  
SET_PED_TO_RAGDOLL(ped, 4000, 5000, 1, 1, 1, 0);  
CREATE_NM_MESSAGE(true, 0); // stopAllBehaviours - Stop all other behaviours, in case the Ped is already doing some Euphoria stuff.  
GIVE_PED_NM_MESSAGE(ped); // Dispatch message to Ped.  
CREATE_NM_MESSAGE(true, 1151); // staggerFall - Attempt to walk while falling.  
GIVE_PED_NM_MESSAGE(ped); // Dispatch message to Ped.  
CREATE_NM_MESSAGE(true, 372); // armsWindmill - Swing arms around.  
GIVE_PED_NM_MESSAGE(ped); // Dispatch message to Ped.  
```



pub fn give_ped_nm_message_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB158DFCCC56E5C5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB158DFCCC56E5C5Bu64;
        
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
pub fn give_ped_nm_message_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB158DFCCC56E5C5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB158DFCCC56E5C5Bu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Something related to the environmental effects natives.
In the "agency_heist3b" script, p1 - p3 are always under 100 - usually they are {87, 81, 68}. If SET_PED_ENVEFF_SCALE is set to 0.65 (instead of the usual 1.0), they use {74, 69, 60}
```



pub fn set_ped_enveff_color_modulator_safe(
        
        
            ped: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD69411AA0CEBF9E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD69411AA0CEBF9E9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn set_ped_enveff_color_modulator_raw(
        ped: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD69411AA0CEBF9E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD69411AA0CEBF9E9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                r, 
                g, 
                b
        )
    }
}

/// Removes the scubagear (for mp male: component id: 8, drawableId: 123, textureId: any) from peds. Does not play the 'remove scuba gear' animation, but instantly removes it.



pub fn clear_ped_scuba_gear_variation_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB50EB4CCB29704ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB50EB4CCB29704ACu64;
        
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
pub fn clear_ped_scuba_gear_variation_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB50EB4CCB29704ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB50EB4CCB29704ACu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn remove_stealth_mode_asset_safe(
        
        
            asset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9219857D21F0E842u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9219857D21F0E842u64;
        
        let result = invoke_raw!(
            hash,
                asset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_stealth_mode_asset_raw(
        asset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9219857D21F0E842u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9219857D21F0E842u64;

        invoke_raw_typed!(
            hash,
                asset
        )
    }
}

/// ## Parameters
*



pub fn is_ped_falling_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB92A102F1C4DFA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB92A102F1C4DFA3u64;
        
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
pub fn is_ped_falling_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB92A102F1C4DFA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB92A102F1C4DFA3u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn release_ped_preload_prop_data_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF79F9DEF0AADE61Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF79F9DEF0AADE61Au64;
        
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
pub fn release_ped_preload_prop_data_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF79F9DEF0AADE61Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF79F9DEF0AADE61Au64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
CLEAR_PED_*
```



pub fn _clear_ped_cover_clipset_override_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC79196DCB36F6121u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC79196DCB36F6121u64;
        
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
pub fn _clear_ped_cover_clipset_override_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC79196DCB36F6121u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC79196DCB36F6121u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_parachute_tint_index_safe(
        
        
            ped: 
        , 
        
        
            tintIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x333FC8DB079B7186u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x333FC8DB079B7186u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                tintIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_parachute_tint_index_raw(
        ped: , 
        tintIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x333FC8DB079B7186u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x333FC8DB079B7186u64;

        invoke_raw_typed!(
            hash,
                ped, 
                tintIndex
        )
    }
}

/// [Animations list](https://alexguirre.github.io/animations-list/)



pub fn get_anim_initial_offset_rotation_safe(
        
        
            animDict: 
        , 
        
        
            animName: 
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
        
        
            p8: 
        , 
        
        
            p9: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B805E6046EE9E47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B805E6046EE9E47u64;
        
        let result = invoke_raw!(
            hash,
                animDict, 
                animName, 
                x, 
                y, 
                z, 
                xRot, 
                yRot, 
                zRot, 
                p8, 
                p9
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_anim_initial_offset_rotation_raw(
        animDict: , 
        animName: , 
        x: , 
        y: , 
        z: , 
        xRot: , 
        yRot: , 
        zRot: , 
        p8: , 
        p9: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B805E6046EE9E47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B805E6046EE9E47u64;

        invoke_raw_typed!(
            hash,
                animDict, 
                animName, 
                x, 
                y, 
                z, 
                xRot, 
                yRot, 
                zRot, 
                p8, 
                p9
        )
    }
}

/// ## Parameters
*



pub fn is_conversation_ped_dead_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0A0AEC214B1FABAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0A0AEC214B1FABAu64;
        
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
pub fn is_conversation_ped_dead_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0A0AEC214B1FABAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0A0AEC214B1FABAu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_model_is_suppressed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE163A4BCE4DE6F11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE163A4BCE4DE6F11u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_model_is_suppressed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE163A4BCE4DE6F11u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE163A4BCE4DE6F11u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn stop_ped_weapon_firing_when_dropped_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC158D28142A34608u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC158D28142A34608u64;
        
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
pub fn stop_ped_weapon_firing_when_dropped_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC158D28142A34608u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC158D28142A34608u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_use_auto_conversation_lookat_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC4686EC06434678u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC4686EC06434678u64;
        
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
pub fn set_ped_can_use_auto_conversation_lookat_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC4686EC06434678u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC4686EC06434678u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
Only appears in lamar1 script.  
```



pub fn _0x1a330d297aac6bc1_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A330D297AAC6BC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A330D297AAC6BC1u64;
        
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
pub fn _0x1a330d297aac6bc1_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A330D297AAC6BC1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A330D297AAC6BC1u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn is_ped_in_group_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5891CAC5D4ACFF74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5891CAC5D4ACFF74u64;
        
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
pub fn is_ped_in_group_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5891CAC5D4ACFF74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5891CAC5D4ACFF74u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Sets the IK target for a given IK part belonging to the ped.



pub fn set_ik_target_safe(
        
        
            ped: 
        , 
        
        
            ikIndex: 
        , 
        
        
            entityLookAt: 
        , 
        
        
            boneLookAt: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        , 
        
        
            ikTargetFlags: 
        , 
        
        
            blendInDuration: 
        , 
        
        
            blendOutDuration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC32779C16FCEECD9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC32779C16FCEECD9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                ikIndex, 
                entityLookAt, 
                boneLookAt, 
                offsetX, 
                offsetY, 
                offsetZ, 
                ikTargetFlags, 
                blendInDuration, 
                blendOutDuration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ik_target_raw(
        ped: , 
        ikIndex: , 
        entityLookAt: , 
        boneLookAt: , 
        offsetX: , 
        offsetY: , 
        offsetZ: , 
        ikTargetFlags: , 
        blendInDuration: , 
        blendOutDuration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC32779C16FCEECD9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC32779C16FCEECD9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                ikIndex, 
                entityLookAt, 
                boneLookAt, 
                offsetX, 
                offsetY, 
                offsetZ, 
                ikTargetFlags, 
                blendInDuration, 
                blendOutDuration
        )
    }
}

/// ## Parameters
*



pub fn set_ped_never_leaves_group_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DBFC55D5C9BB447u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DBFC55D5C9BB447u64;
        
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
pub fn set_ped_never_leaves_group_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DBFC55D5C9BB447u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DBFC55D5C9BB447u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_evasive_dive_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B7A646C242A7059u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B7A646C242A7059u64;
        
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
pub fn set_ped_can_evasive_dive_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B7A646C242A7059u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B7A646C242A7059u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn create_ped_inside_vehicle_safe(
        
        
            vehicle: 
        , 
        
        
            pedType: 
        , 
        
        
            modelHash: 
        , 
        
        
            seat: 
        , 
        
        
            isNetwork: 
        , 
        
        
            bScriptHostPed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DD959874C1FD534u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DD959874C1FD534u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                pedType, 
                modelHash, 
                seat, 
                isNetwork, 
                bScriptHostPed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_ped_inside_vehicle_raw(
        vehicle: , 
        pedType: , 
        modelHash: , 
        seat: , 
        isNetwork: , 
        bScriptHostPed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DD959874C1FD534u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DD959874C1FD534u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                pedType, 
                modelHash, 
                seat, 
                isNetwork, 
                bScriptHostPed
        )
    }
}

/// ## Parameters
*



pub fn is_ped_on_specific_vehicle_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC5F66E459AF3BB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC5F66E459AF3BB2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_on_specific_vehicle_raw(
        ped: , 
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC5F66E459AF3BB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC5F66E459AF3BB2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn is_ped_hanging_on_to_vehicle_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C86D8AEF8254B78u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C86D8AEF8254B78u64;
        
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
pub fn is_ped_hanging_on_to_vehicle_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C86D8AEF8254B78u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C86D8AEF8254B78u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// **Usage:** Call this native every frame



pub fn set_ped_density_multiplier_this_frame_safe(
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95E3D6257B166CF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95E3D6257B166CF2u64;
        
        let result = invoke_raw!(
            hash,
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_density_multiplier_this_frame_raw(
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95E3D6257B166CF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95E3D6257B166CF2u64;

        invoke_raw_typed!(
            hash,
                multiplier
        )
    }
}

/// ```
Based on TASK_COMBAT_HATED_TARGETS_AROUND_PED, the parameters are likely similar (PedHandle, and area to attack in).  
```



pub fn register_hated_targets_around_ped_safe(
        
        
            ped: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9222F300BF8354FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9222F300BF8354FEu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn register_hated_targets_around_ped_raw(
        ped: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9222F300BF8354FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9222F300BF8354FEu64;

        invoke_raw_typed!(
            hash,
                ped, 
                radius
        )
    }
}

/// ## Parameters
*



pub fn set_ped_preload_prop_data_safe(
        
        
            ped: 
        , 
        
        
            componentId: 
        , 
        
        
            drawableId: 
        , 
        
        
            textureId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B16A3BFF1FBCE49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B16A3BFF1FBCE49u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                componentId, 
                drawableId, 
                textureId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_preload_prop_data_raw(
        ped: , 
        componentId: , 
        drawableId: , 
        textureId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B16A3BFF1FBCE49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B16A3BFF1FBCE49u64;

        invoke_raw_typed!(
            hash,
                ped, 
                componentId, 
                drawableId, 
                textureId
        )
    }
}

/// ## Parameters
*



pub fn set_scripted_conversion_coord_this_frame_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5086C7843552CF85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5086C7843552CF85u64;
        
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
pub fn set_scripted_conversion_coord_this_frame_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5086C7843552CF85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5086C7843552CF85u64;

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



pub fn get_synchronized_scene_rate_safe(
        
        
            sceneID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD80932D577274D40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD80932D577274D40u64;
        
        let result = invoke_raw!(
            hash,
                sceneID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_synchronized_scene_rate_raw(
        sceneID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD80932D577274D40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD80932D577274D40u64;

        invoke_raw_typed!(
            hash,
                sceneID
        )
    }
}

/// ```
Values look to be between 0.0 and 1.0  
From decompiled scripts: 0.0, 0.6, 0.65, 0.8, 1.0  
You are correct, just looked in IDA it breaks from the function if it's less than 0.0f or greater than 1.0f.  
```



pub fn set_ped_enveff_scale_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF29516833893561u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF29516833893561u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_enveff_scale_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF29516833893561u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF29516833893561u64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// ## Parameters
*



pub fn get_ped_money_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F69145BBA87BAE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F69145BBA87BAE7u64;
        
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
pub fn get_ped_money_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F69145BBA87BAE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F69145BBA87BAE7u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// STOP_ANY_PED_MODEL_BEING_SUPPRESSED native function



pub fn stop_any_ped_model_being_suppressed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB47BD05FA66B40CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB47BD05FA66B40CFu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_any_ped_model_being_suppressed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB47BD05FA66B40CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB47BD05FA66B40CFu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2699
```



pub fn _get_ped_dies_in_water_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65671A4FB8218930u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65671A4FB8218930u64;
        
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
pub fn _get_ped_dies_in_water_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65671A4FB8218930u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65671A4FB8218930u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
CLEAR_PED_*
```



pub fn _0x80054d7fcc70eec6_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80054D7FCC70EEC6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80054D7FCC70EEC6u64;
        
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
pub fn _0x80054d7fcc70eec6_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80054D7FCC70EEC6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80054D7FCC70EEC6u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_to_load_cover_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x332B562EEDA62399u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x332B562EEDA62399u64;
        
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
pub fn set_ped_to_load_cover_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x332B562EEDA62399u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x332B562EEDA62399u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_ped_headtracking_entity_safe(
        
        
            ped: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x813A0A7C9D2E831Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x813A0A7C9D2E831Fu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_headtracking_entity_raw(
        ped: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x813A0A7C9D2E831Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x813A0A7C9D2E831Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                entity
        )
    }
}

/// ```
Gets ID of vehicle player using. It means it can get ID at any interaction with vehicle. Enter\exit for example. And that means it is faster than GET_VEHICLE_PED_IS_IN but less safe.  
```



pub fn get_vehicle_ped_is_using_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6094AD011A2EA87Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6094AD011A2EA87Du64;
        
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
pub fn get_vehicle_ped_is_using_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6094AD011A2EA87Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6094AD011A2EA87Du64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
If the ped handle passed through the parenthesis is in a ragdoll state this will return true.  
```



pub fn is_ped_ragdoll_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47E4E977581C5B55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47E4E977581C5B55u64;
        
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
pub fn is_ped_ragdoll_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x47E4E977581C5B55u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x47E4E977581C5B55u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Resets the value for the last vehicle driven by the Ped.  
```



pub fn reset_ped_last_vehicle_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB8DE8CF6A8DD8BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB8DE8CF6A8DD8BBu64;
        
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
pub fn reset_ped_last_vehicle_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB8DE8CF6A8DD8BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB8DE8CF6A8DD8BBu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_should_play_normal_scenario_exit_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3A9299C4F2ADB98u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3A9299C4F2ADB98u64;
        
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
pub fn set_ped_should_play_normal_scenario_exit_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3A9299C4F2ADB98u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3A9299C4F2ADB98u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// RESET_AI_MELEE_WEAPON_DAMAGE_MODIFIER native function



pub fn reset_ai_melee_weapon_damage_modifier_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46E56A7CD1D63C3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46E56A7CD1D63C3Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_ai_melee_weapon_damage_modifier_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46E56A7CD1D63C3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46E56A7CD1D63C3Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns size of array, passed into the second variable.  
See below for usage information.  
This function actually requires a struct, where the first value is the maximum number of elements to return.  Here is a sample of how I was able to get it to work correctly, without yet knowing the struct format.  
//Setup the array  
	const int numElements = 10;  
	const int arrSize = numElements * 2 + 2;  
	Any veh[arrSize];  
	//0 index is the size of the array  
	veh[0] = numElements;  
	int count = PED::GET_PED_NEARBY_VEHICLES(PLAYER::PLAYER_PED_ID(), veh);  
	if (veh != NULL)  
	{  
//Simple loop to go through results  
for (int i = 0; i < count; i++)  
{  
	int offsettedID = i * 2 + 2;  
	//Make sure it exists  
	if (veh[offsettedID] != NULL && ENTITY::DOES_ENTITY_EXIST(veh[offsettedID]))  
	{  
//Do something  
	}  
}  
	}    
Here's the right way to do it (console and pc):  
pastebin.com/SsFej963  
```



pub fn get_ped_nearby_vehicles_safe(
        
        
            ped: 
        , 
        
        
            sizeAndVehs: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFF869CBFA210D82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFF869CBFA210D82u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                sizeAndVehs
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_nearby_vehicles_raw(
        ped: , 
        sizeAndVehs: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFF869CBFA210D82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFF869CBFA210D82u64;

        invoke_raw_typed!(
            hash,
                ped, 
                sizeAndVehs
        )
    }
}

/// ```
p1 is always 0 in R* scripts; and a quick disassembly seems to indicate that p1 is unused.  
```



pub fn set_ped_random_component_variation_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8A9481A01E63C28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8A9481A01E63C28u64;
        
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
pub fn set_ped_random_component_variation_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8A9481A01E63C28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8A9481A01E63C28u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// See [`SET_PED_HEAD_BLEND_DATA`](#_0x9414E18B9434C2FE)



pub fn update_ped_head_blend_data_safe(
        
        
            ped: 
        , 
        
        
            shapeMix: 
        , 
        
        
            skinMix: 
        , 
        
        
            thirdMix: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x723538F61C647C5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x723538F61C647C5Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                shapeMix, 
                skinMix, 
                thirdMix
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn update_ped_head_blend_data_raw(
        ped: , 
        shapeMix: , 
        skinMix: , 
        thirdMix: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x723538F61C647C5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x723538F61C647C5Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                shapeMix, 
                skinMix, 
                thirdMix
        )
    }
}

/// ## Parameters
*



pub fn set_ped_to_inform_respected_friends_safe(
        
        
            ped: 
        , 
        
        
            radius: 
        , 
        
        
            maxFriends: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x112942C6E708F70Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x112942C6E708F70Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                radius, 
                maxFriends
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_to_inform_respected_friends_raw(
        ped: , 
        radius: , 
        maxFriends: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x112942C6E708F70Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x112942C6E708F70Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                radius, 
                maxFriends
        )
    }
}

/// ```
teleports ped to coords along with the vehicle ped is in  
```



pub fn set_ped_coords_keep_vehicle_safe(
        
        
            ped: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AFEFF481A85AB2Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AFEFF481A85AB2Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn set_ped_coords_keep_vehicle_raw(
        ped: , 
        posX: , 
        posY: , 
        posZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AFEFF481A85AB2Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AFEFF481A85AB2Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                posX, 
                posY, 
                posZ
        )
    }
}

/// Retrieves the vehicle the specified ped is currently in, or the last vehicle they were in.



pub fn get_vehicle_ped_is_in_safe(
        
        
            ped: 
        , 
        
        
            lastVehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A9112A0FE9A4713u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A9112A0FE9A4713u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                lastVehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_ped_is_in_raw(
        ped: , 
        lastVehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A9112A0FE9A4713u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A9112A0FE9A4713u64;

        invoke_raw_typed!(
            hash,
                ped, 
                lastVehicle
        )
    }
}

/// ## Parameters
*



pub fn _0x9e30e91fb03a2caf_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E30E91FB03A2CAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E30E91FB03A2CAFu64;
        
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
pub fn _0x9e30e91fb03a2caf_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E30E91FB03A2CAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E30E91FB03A2CAFu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_ped_generates_dead_body_events_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FB17BA2E7DECA5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FB17BA2E7DECA5Bu64;
        
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
pub fn set_ped_generates_dead_body_events_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FB17BA2E7DECA5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FB17BA2E7DECA5Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_play_ambient_base_anims_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EB0585D15254740u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EB0585D15254740u64;
        
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
pub fn set_ped_can_play_ambient_base_anims_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EB0585D15254740u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EB0585D15254740u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x711794453cfd692b_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x711794453CFD692Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x711794453CFD692Bu64;
        
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
pub fn _0x711794453cfd692b_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x711794453CFD692Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x711794453CFD692Bu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_leg_ik_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73518ECE2485412Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73518ECE2485412Bu64;
        
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
pub fn set_ped_can_leg_ik_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73518ECE2485412Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73518ECE2485412Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_synchronized_scene_looped_safe(
        
        
            sceneID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62522002E0C391BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62522002E0C391BAu64;
        
        let result = invoke_raw!(
            hash,
                sceneID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_synchronized_scene_looped_raw(
        sceneID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62522002E0C391BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62522002E0C391BAu64;

        invoke_raw_typed!(
            hash,
                sceneID
        )
    }
}

/// ```
name: "MP_FEMALE_ACTION" found multiple times in the b617d scripts.
```



pub fn set_movement_mode_override_safe(
        
        
            ped: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x781DE8FA214E87D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x781DE8FA214E87D2u64;
        
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
pub fn set_movement_mode_override_raw(
        ped: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x781DE8FA214E87D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x781DE8FA214E87D2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                name
        )
    }
}

/// ## Parameters
*



pub fn set_ped_as_group_member_safe(
        
        
            ped: 
        , 
        
        
            groupId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F3480FE65DB31B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F3480FE65DB31B5u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                groupId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_as_group_member_raw(
        ped: , 
        groupId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F3480FE65DB31B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F3480FE65DB31B5u64;

        invoke_raw_typed!(
            hash,
                ped, 
                groupId
        )
    }
}

/// ## Parameters
*



pub fn get_number_of_ped_texture_variations_safe(
        
        
            ped: 
        , 
        
        
            componentId: 
        , 
        
        
            drawableId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F7156A3142A6BADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F7156A3142A6BADu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                componentId, 
                drawableId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_number_of_ped_texture_variations_raw(
        ped: , 
        componentId: , 
        drawableId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F7156A3142A6BADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F7156A3142A6BADu64;

        invoke_raw_typed!(
            hash,
                ped, 
                componentId, 
                drawableId
        )
    }
}

/// ## Parameters
*



pub fn _0x412f1364fa066cfb_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x412F1364FA066CFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x412F1364FA066CFBu64;
        
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
pub fn _0x412f1364fa066cfb_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x412F1364FA066CFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x412F1364FA066CFBu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Overrides the ped's collision capsule radius for the current tick.  
Must be called every tick to be effective.  
Setting this to 0.001 will allow warping through some objects.  
```



pub fn set_ped_capsule_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x364DF566EC833DE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x364DF566EC833DE2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_capsule_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x364DF566EC833DE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x364DF566EC833DE2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// ## Parameters
*



pub fn clear_ped_decorations_leave_scars_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3B27E70CEAB9F0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3B27E70CEAB9F0Cu64;
        
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
pub fn clear_ped_decorations_leave_scars_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3B27E70CEAB9F0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3B27E70CEAB9F0Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _0x3e9679c1dfcf422c_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E9679C1DFCF422Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E9679C1DFCF422Cu64;
        
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
pub fn _0x3e9679c1dfcf422c_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E9679C1DFCF422Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E9679C1DFCF422Cu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Detect if ped is in any vehicle  
[True/False]  
```



pub fn is_ped_sitting_in_any_vehicle_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x826AA586EDB9FEF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x826AA586EDB9FEF8u64;
        
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
pub fn is_ped_sitting_in_any_vehicle_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x826AA586EDB9FEF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x826AA586EDB9FEF8u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_ped_defensive_area_position_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C06B8786DD94CD1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C06B8786DD94CD1u64;
        
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
pub fn get_ped_defensive_area_position_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C06B8786DD94CD1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C06B8786DD94CD1u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// Creates a ped (biped character, pedestrian, actor) with the specified model at the specified position and heading.
This ped will initially be owned by the creating script as a mission entity, and the model should be loaded already
(e.g. using REQUEST_MODEL).



pub fn create_ped_safe(
        
        
            pedType: 
        , 
        
        
            modelHash: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            heading: 
        , 
        
        
            isNetwork: 
        , 
        
        
            bScriptHostPed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD49F9B0955C367DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD49F9B0955C367DEu64;
        
        let result = invoke_raw!(
            hash,
                pedType, 
                modelHash, 
                x, 
                y, 
                z, 
                heading, 
                isNetwork, 
                bScriptHostPed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_ped_raw(
        pedType: , 
        modelHash: , 
        x: , 
        y: , 
        z: , 
        heading: , 
        isNetwork: , 
        bScriptHostPed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD49F9B0955C367DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD49F9B0955C367DEu64;

        invoke_raw_typed!(
            hash,
                pedType, 
                modelHash, 
                x, 
                y, 
                z, 
                heading, 
                isNetwork, 
                bScriptHostPed
        )
    }
}

/// Prevents ambient peds from dropping their weapons for the current frame.

```
NativeDB Introduced: v3258
```



pub fn _set_block_ambient_peds_from_dropping_weapons_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC73EFFC5E043A8BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC73EFFC5E043A8BAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_block_ambient_peds_from_dropping_weapons_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC73EFFC5E043A8BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC73EFFC5E043A8BAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
gtaforums.com/topic/885580-ped-headshotmugshot-txd/  
```



pub fn is_pedheadshot_valid_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0A9668F158129A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0A9668F158129A2u64;
        
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
pub fn is_pedheadshot_valid_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0A9668F158129A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0A9668F158129A2u64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ## Parameters
*



pub fn is_ped_performing_stealth_kill_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD4CCDBCC59941B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD4CCDBCC59941B7u64;
        
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
pub fn is_ped_performing_stealth_kill_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD4CCDBCC59941B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD4CCDBCC59941B7u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Used with [SET_ENABLE_HANDCUFFS](#_0xDF1AF8B5D56542FA) in decompiled scripts. From my observations, I have noticed that while being ragdolled you are not able to get up but you can still run. Your legs can also bend.



pub fn set_enable_bound_ankles_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC52E0F855C58FC2Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC52E0F855C58FC2Eu64;
        
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
pub fn set_enable_bound_ankles_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC52E0F855C58FC2Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC52E0F855C58FC2Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
From the scripts:  
PED::SET_PED_GESTURE_GROUP(PLAYER::PLAYER_PED_ID(),  
"ANIM_GROUP_GESTURE_MISS_FRA0");  
PED::SET_PED_GESTURE_GROUP(PLAYER::PLAYER_PED_ID(),  
"ANIM_GROUP_GESTURE_MISS_DocksSetup1");  
```



pub fn set_ped_gesture_group_safe(
        
        
            ped: 
        , 
        
        
            animGroupGesture: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDF803377F94AAA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDF803377F94AAA8u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                animGroupGesture
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_gesture_group_raw(
        ped: , 
        animGroupGesture: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDDF803377F94AAA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDDF803377F94AAA8u64;

        invoke_raw_typed!(
            hash,
                ped, 
                animGroupGesture
        )
    }
}

/// ```
gtaforums.com/topic/885580-ped-headshotmugshot-txd/  
```



pub fn register_pedheadshot_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4462658788425076u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4462658788425076u64;
        
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
pub fn register_pedheadshot_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4462658788425076u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4462658788425076u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _create_synchronized_scene_2_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62EC273D00187DCAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62EC273D00187DCAu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _create_synchronized_scene_2_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62EC273D00187DCAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62EC273D00187DCAu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                object
        )
    }
}

/// ## Parameters
*



pub fn set_group_formation_spacing_safe(
        
        
            groupId: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D9D45004C28C916u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D9D45004C28C916u64;
        
        let result = invoke_raw!(
            hash,
                groupId, 
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
pub fn set_group_formation_spacing_raw(
        groupId: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D9D45004C28C916u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D9D45004C28C916u64;

        invoke_raw_typed!(
            hash,
                groupId, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ```
p1 is always 0  
```



pub fn is_ped_being_stunned_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4FBACCE3B4138EE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4FBACCE3B4138EE8u64;
        
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
pub fn is_ped_being_stunned_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4FBACCE3B4138EE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4FBACCE3B4138EE8u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _get_ped_event_data_safe(
        
        
            ped: 
        , 
        
        
            eventType: 
        , 
        
        
            outData: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA656A3BB01BDEA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA656A3BB01BDEA3u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                eventType, 
                outData
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_ped_event_data_raw(
        ped: , 
        eventType: , 
        outData: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA656A3BB01BDEA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA656A3BB01BDEA3u64;

        invoke_raw_typed!(
            hash,
                ped, 
                eventType, 
                outData
        )
    }
}

/// ## Parameters
*



pub fn _0x06087579e7aa85a9_safe(
        
        
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
        let hash = 0x06087579E7AA85A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06087579E7AA85A9u64;
        
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
pub fn _0x06087579e7aa85a9_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06087579E7AA85A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06087579E7AA85A9u64;

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



pub fn set_ped_can_ragdoll_from_player_impact_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF993EE5E90ABA25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF993EE5E90ABA25u64;
        
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
pub fn set_ped_can_ragdoll_from_player_impact_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF993EE5E90ABA25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF993EE5E90ABA25u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
Returns true/false if the ped is/isn't male.  
```



pub fn is_ped_male_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D9F5FAA7488BA46u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D9F5FAA7488BA46u64;
        
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
pub fn is_ped_male_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D9F5FAA7488BA46u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D9F5FAA7488BA46u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn apply_ped_blood_damage_by_zone_safe(
        
        
            ped: 
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
        let hash = 0x816F6981C60BF53Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x816F6981C60BF53Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn apply_ped_blood_damage_by_zone_raw(
        ped: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x816F6981C60BF53Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x816F6981C60BF53Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ```
combined with PED::SET_PED_WETNESS_HEIGHT(), this native makes the ped drenched in water up to the height specified in the other function  
```



pub fn set_ped_wetness_enabled_this_frame_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5485E4907B53019u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5485E4907B53019u64;
        
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
pub fn set_ped_wetness_enabled_this_frame_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5485E4907B53019u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5485E4907B53019u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _is_scuba_gear_light_enabled_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88274C11CF0D866Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88274C11CF0D866Du64;
        
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
pub fn _is_scuba_gear_light_enabled_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88274C11CF0D866Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88274C11CF0D866Du64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _0x336b3d200ab007cb_safe(
        
        
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
        let hash = 0x336B3D200AB007CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x336B3D200AB007CBu64;
        
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
pub fn _0x336b3d200ab007cb_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x336B3D200AB007CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x336B3D200AB007CBu64;

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
Gets the position of the specified bone of the specified ped.  
ped: The ped to get the position of a bone from.  
boneId: The ID of the bone to get the position from. This is NOT the index.  
offsetX: The X-component of the offset to add to the position relative to the bone's rotation.  
offsetY: The Y-component of the offset to add to the position relative to the bone's rotation.  
offsetZ: The Z-component of the offset to add to the position relative to the bone's rotation.  
```



pub fn get_ped_bone_coords_safe(
        
        
            ped: 
        , 
        
        
            boneId: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17C07FC640E86B4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17C07FC640E86B4Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                boneId, 
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
pub fn get_ped_bone_coords_raw(
        ped: , 
        boneId: , 
        offsetX: , 
        offsetY: , 
        offsetZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17C07FC640E86B4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17C07FC640E86B4Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                boneId, 
                offsetX, 
                offsetY, 
                offsetZ
        )
    }
}

/// ## Parameters
*



pub fn has_stealth_mode_asset_loaded_safe(
        
        
            asset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE977FC5B08AF3441u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE977FC5B08AF3441u64;
        
        let result = invoke_raw!(
            hash,
                asset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_stealth_mode_asset_loaded_raw(
        asset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE977FC5B08AF3441u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE977FC5B08AF3441u64;

        invoke_raw_typed!(
            hash,
                asset
        )
    }
}

/// [Animations list](https://alexguirre.github.io/animations-list/)



pub fn set_ped_alternate_walk_anim_safe(
        
        
            ped: 
        , 
        
        
            animDict: 
        , 
        
        
            animName: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C60394CB4F75E9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C60394CB4F75E9Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                animDict, 
                animName, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_alternate_walk_anim_raw(
        ped: , 
        animDict: , 
        animName: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C60394CB4F75E9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C60394CB4F75E9Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                animDict, 
                animName, 
                p3, 
                p4
        )
    }
}

/// ```
Judging purely from a quick disassembly, if the ped is in a vehicle, the ped will be deleted immediately. If not, it'll be marked as no longer needed. 
```



pub fn remove_ped_elegantly_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC6D445B994DF95Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC6D445B994DF95Eu64;
        
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
pub fn remove_ped_elegantly_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC6D445B994DF95Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC6D445B994DF95Eu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// [`SET_VEHICLE_STEER_BIAS`](#_0x42A8EC77D5150CBE) for peds, e.g., `_SET_PED_STEER_BIAS`.



pub fn _0x288df530c92dad6f_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x288DF530C92DAD6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x288DF530C92DAD6Fu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x288df530c92dad6f_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x288DF530C92DAD6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x288DF530C92DAD6Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// ## Parameters
*



pub fn reset_ped_in_vehicle_context_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22EF8FF8778030EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22EF8FF8778030EBu64;
        
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
pub fn reset_ped_in_vehicle_context_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22EF8FF8778030EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22EF8FF8778030EBu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn can_ped_in_combat_see_target_safe(
        
        
            ped: 
        , 
        
        
            target: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAD42DE3610D0721u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAD42DE3610D0721u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                target
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_ped_in_combat_see_target_raw(
        ped: , 
        target: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAD42DE3610D0721u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAD42DE3610D0721u64;

        invoke_raw_typed!(
            hash,
                ped, 
                target
        )
    }
}

/// Kicks the ped from the current vehicle and keeps the rendering-focus on this ped (also disables its collision). If doing this for your player ped, you'll still be able to drive the vehicle.  
Only to be used in very specific situations where the ped needs to be inside the car still but not attached.



pub fn special_function_do_not_use_safe(
        
        
            ped: 
        , 
        
        
            noCollisionUntilClear: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9ACF4A08098EA25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9ACF4A08098EA25u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                noCollisionUntilClear
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn special_function_do_not_use_raw(
        ped: , 
        noCollisionUntilClear: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9ACF4A08098EA25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9ACF4A08098EA25u64;

        invoke_raw_typed!(
            hash,
                ped, 
                noCollisionUntilClear
        )
    }
}

/// ## Parameters
*



pub fn reset_ped_strafe_clipset_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20510814175EA477u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20510814175EA477u64;
        
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
pub fn reset_ped_strafe_clipset_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20510814175EA477u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20510814175EA477u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Return value



pub fn can_create_random_cops_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EE2CAFF7F17770Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EE2CAFF7F17770Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_create_random_cops_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EE2CAFF7F17770Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EE2CAFF7F17770Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_ped_allowed_to_duck_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA1F1B7BE1A8766Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA1F1B7BE1A8766Fu64;
        
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
pub fn set_ped_allowed_to_duck_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA1F1B7BE1A8766Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA1F1B7BE1A8766Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
REQUEST_*
```



pub fn _0xcd018c591f94cb43_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD018C591F94CB43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD018C591F94CB43u64;
        
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
pub fn _0xcd018c591f94cb43_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD018C591F94CB43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD018C591F94CB43u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// Used for freemode (online) characters.  

Indices:
  1. black
  2. very light blue/green
  3. dark blue
  4. brown
  5. darker brown
  6. light brown
  7. blue
  8. light blue
  9. pink
  10. yellow
  11. purple
  12. black
  13. dark green
  14. light brown
  15. yellow/black pattern
  16. light colored spiral pattern
  17. shiny red
  18. shiny half blue/half red
  19. half black/half light blue
  20. white/red perimter
  21. green snake
  22. red snake
  23. dark blue snake
  24. dark yellow
  25. bright yellow
  26. all black
  28. red small pupil
  29. devil blue/black
  30. white small pupil
  31. glossed over



pub fn _set_ped_eye_color_safe(
        
        
            ped: 
        , 
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50B56988B170AFDFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50B56988B170AFDFu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_ped_eye_color_raw(
        ped: , 
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50B56988B170AFDFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50B56988B170AFDFu64;

        invoke_raw_typed!(
            hash,
                ped, 
                index
        )
    }
}

/// ## Parameters
*



pub fn _0xc2ee020f5fb4db53_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2EE020F5FB4DB53u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2EE020F5FB4DB53u64;
        
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
pub fn _0xc2ee020f5fb4db53_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2EE020F5FB4DB53u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2EE020F5FB4DB53u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_be_targetted_by_player_safe(
        
        
            ped: 
        , 
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66B57B72E0836A76u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66B57B72E0836A76u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_can_be_targetted_by_player_raw(
        ped: , 
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66B57B72E0836A76u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66B57B72E0836A76u64;

        invoke_raw_typed!(
            hash,
                ped, 
                player, 
                toggle
        )
    }
}

/// ```
SET_PED_*  
Has most likely to do with some shooting attributes as it sets the float which is in the same range as shootRate.  
```



pub fn _0xec4b4b3b9908052a_safe(
        
        
            ped: 
        , 
        
        
            unk: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC4B4B3B9908052Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC4B4B3B9908052Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                unk
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xec4b4b3b9908052a_raw(
        ped: , 
        unk: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC4B4B3B9908052Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC4B4B3B9908052Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                unk
        )
    }
}

/// ## Return value



pub fn can_create_random_driver_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8EB95E5B4E56978u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8EB95E5B4E56978u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_create_random_driver_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8EB95E5B4E56978u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8EB95E5B4E56978u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// SPAWNPOINTS_CANCEL_SEARCH native function



pub fn spawnpoints_cancel_search_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEE4A5459472A9F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEE4A5459472A9F8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn spawnpoints_cancel_search_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEE4A5459472A9F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEE4A5459472A9F8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_ped_dies_when_injured_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BA7919BED300023u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BA7919BED300023u64;
        
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
pub fn set_ped_dies_when_injured_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BA7919BED300023u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BA7919BED300023u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
Somehow related to changing ped's clothes.  
```



pub fn clear_ped_blood_damage_by_zone_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56E3B78C5408D9F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56E3B78C5408D9F4u64;
        
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
pub fn clear_ped_blood_damage_by_zone_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56E3B78C5408D9F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56E3B78C5408D9F4u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn is_ped_in_any_boat_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E0E1C2B4F6CB339u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E0E1C2B4F6CB339u64;
        
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
pub fn is_ped_in_any_boat_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E0E1C2B4F6CB339u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E0E1C2B4F6CB339u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn clear_ped_stored_hat_prop_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x687C0B594907D2E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x687C0B594907D2E8u64;
        
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
pub fn clear_ped_stored_hat_prop_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x687C0B594907D2E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x687C0B594907D2E8u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Gets a value indicating whether the specified ped is on top of any vehicle.  
Return 1 when ped is on vehicle.  
Return 0 when ped is not on a vehicle.  
```



pub fn is_ped_on_vehicle_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67722AEB798E5FABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67722AEB798E5FABu64;
        
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
pub fn is_ped_on_vehicle_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67722AEB798E5FABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67722AEB798E5FABu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_desired_heading_safe(
        
        
            ped: 
        , 
        
        
            heading: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA5A7ECE2AA8FE70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA5A7ECE2AA8FE70u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                heading
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_desired_heading_raw(
        ped: , 
        heading: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA5A7ECE2AA8FE70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA5A7ECE2AA8FE70u64;

        invoke_raw_typed!(
            hash,
                ped, 
                heading
        )
    }
}

/// ## Return value



pub fn can_create_random_bike_rider_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEACEEDA81751915Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEACEEDA81751915Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_create_random_bike_rider_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEACEEDA81751915Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEACEEDA81751915Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Presumably returns the Entity that the Ped is currently diving out of the way of.
var num3;
    if (PED::IS_PED_EVASIVE_DIVING(A_0, &num3) != 0)
        if (ENTITY::IS_ENTITY_A_VEHICLE(num3) != 0)
```



pub fn is_ped_evasive_diving_safe(
        
        
            ped: 
        , 
        
        
            evadingEntity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x414641C26E105898u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x414641C26E105898u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                evadingEntity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_evasive_diving_raw(
        ped: , 
        evadingEntity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x414641C26E105898u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x414641C26E105898u64;

        invoke_raw_typed!(
            hash,
                ped, 
                evadingEntity
        )
    }
}

/// ## Parameters
*



pub fn disable_head_blend_palette_color_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA21C118553BBDF02u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA21C118553BBDF02u64;
        
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
pub fn disable_head_blend_palette_color_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA21C118553BBDF02u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA21C118553BBDF02u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Can't select void. This function returns nothing. The hash of the created relationship group is output in the second parameter.  
```



pub fn add_relationship_group_safe(
        
        
            name: 
        , 
        
        
            groupHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF372BC22FCB88606u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF372BC22FCB88606u64;
        
        let result = invoke_raw!(
            hash,
                name, 
                groupHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_relationship_group_raw(
        name: , 
        groupHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF372BC22FCB88606u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF372BC22FCB88606u64;

        invoke_raw_typed!(
            hash,
                name, 
                groupHash
        )
    }
}

/// ## Parameters
*



pub fn is_ped_going_into_cover_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F65DBC537E59AD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F65DBC537E59AD5u64;
        
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
pub fn is_ped_going_into_cover_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F65DBC537E59AD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F65DBC537E59AD5u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn release_ped_preload_variation_data_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AAB586FFEC0FD96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AAB586FFEC0FD96u64;
        
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
pub fn release_ped_preload_variation_data_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AAB586FFEC0FD96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AAB586FFEC0FD96u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// See [`TASK_START_SCENARIO_IN_PLACE`](#_0x142A02425FF02BD9) for a list of scenarios.



pub fn is_ped_using_scenario_safe(
        
        
            ped: 
        , 
        
        
            scenario: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1BF094736DD62C2Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1BF094736DD62C2Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                scenario
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_using_scenario_raw(
        ped: , 
        scenario: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1BF094736DD62C2Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1BF094736DD62C2Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                scenario
        )
    }
}

/// ```
The distance between these points, is the diagonal of a box (remember it's 3D).  
```



pub fn set_ped_non_creation_area_safe(
        
        
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE01041D559983EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE01041D559983EAu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_non_creation_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE01041D559983EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE01041D559983EAu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2
        )
    }
}

/// ## Parameters
*



pub fn set_ped_max_move_blend_ratio_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x433083750C5E064Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x433083750C5E064Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_max_move_blend_ratio_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x433083750C5E064Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x433083750C5E064Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// Returns whether the specified ped is in any vehicle. If `atGetIn` is set to true, also returns true if the ped is
currently in the process of entering a vehicle (a specific stage check for `CTaskEnterVehicle`).



pub fn is_ped_in_any_vehicle_safe(
        
        
            ped: 
        , 
        
        
            atGetIn: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x997ABD671D25CA0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x997ABD671D25CA0Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                atGetIn
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_in_any_vehicle_raw(
        ped: , 
        atGetIn: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x997ABD671D25CA0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x997ABD671D25CA0Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                atGetIn
        )
    }
}

/// ```
NativeDB Introduced: v3407
```



pub fn is_ped_in_sphere_area_of_any_enemy_peds_safe(
        
        
            ped: 
        , 
        
        
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
        let hash = 0x082D79E15302F0C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x082D79E15302F0C2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn is_ped_in_sphere_area_of_any_enemy_peds_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        range: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x082D79E15302F0C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x082D79E15302F0C2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                range
        )
    }
}

/// ## Parameters
*



pub fn set_synchronized_scene_phase_safe(
        
        
            sceneID: 
        , 
        
        
            phase: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x734292F4F0ABF6D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x734292F4F0ABF6D0u64;
        
        let result = invoke_raw!(
            hash,
                sceneID, 
                phase
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_synchronized_scene_phase_raw(
        sceneID: , 
        phase: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x734292F4F0ABF6D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x734292F4F0ABF6D0u64;

        invoke_raw_typed!(
            hash,
                sceneID, 
                phase
        )
    }
}

/// ```
0 - Stationary (Will just stand in place)  
1 - Defensive (Will try to find cover and very likely to blind fire)  
2 - Offensive (Will attempt to charge at enemy but take cover as well)  
3 - Suicidal Offensive (Will try to flank enemy in a suicidal attack)  
```



pub fn set_ped_combat_movement_safe(
        
        
            ped: 
        , 
        
        
            combatMovement: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D9CA1009AFBD057u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D9CA1009AFBD057u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                combatMovement
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_combat_movement_raw(
        ped: , 
        combatMovement: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D9CA1009AFBD057u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D9CA1009AFBD057u64;

        invoke_raw_typed!(
            hash,
                ped, 
                combatMovement
        )
    }
}

/// ## Parameters
*



pub fn is_ped_in_any_train_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F972C1AB75A1ED0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F972C1AB75A1ED0u64;
        
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
pub fn is_ped_in_any_train_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F972C1AB75A1ED0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F972C1AB75A1ED0u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
GET_*
```



pub fn _0x511f1a683387c7e2_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x511F1A683387C7E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x511F1A683387C7E2u64;
        
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
pub fn _0x511f1a683387c7e2_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x511F1A683387C7E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x511F1A683387C7E2u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn has_action_mode_asset_loaded_safe(
        
        
            asset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4B5F4BF2CB24E65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4B5F4BF2CB24E65u64;
        
        let result = invoke_raw!(
            hash,
                asset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_action_mode_asset_loaded_raw(
        asset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4B5F4BF2CB24E65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4B5F4BF2CB24E65u64;

        invoke_raw_typed!(
            hash,
                asset
        )
    }
}

/// ## Parameters
*



pub fn spawnpoints_start_search_safe(
        
        
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
        
        
            interiorFlags: 
        , 
        
        
            scale: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DF9038C90AD5264u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DF9038C90AD5264u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                interiorFlags, 
                scale, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn spawnpoints_start_search_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        interiorFlags: , 
        scale: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DF9038C90AD5264u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DF9038C90AD5264u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                p4, 
                interiorFlags, 
                scale, 
                duration
        )
    }
}

/// ## Parameters
*



pub fn get_ped_helmet_stored_hat_tex_index_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D728C1E12BF5518u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D728C1E12BF5518u64;
        
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
pub fn get_ped_helmet_stored_hat_tex_index_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D728C1E12BF5518u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D728C1E12BF5518u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Works for both player and peds, but some flags don't seem to work for the player (1, for example)  
1 - Blocks ragdolling when shot.  
2 - Blocks ragdolling when hit by a vehicle. The ped still might play a falling animation.  
4 - Blocks ragdolling when set on fire.



pub fn set_ragdoll_blocking_flags_safe(
        
        
            ped: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26695EC767728D84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26695EC767728D84u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ragdoll_blocking_flags_raw(
        ped: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26695EC767728D84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26695EC767728D84u64;

        invoke_raw_typed!(
            hash,
                ped, 
                flags
        )
    }
}

/// ## Parameters
*



pub fn set_ped_relationship_group_hash_safe(
        
        
            ped: 
        , 
        
        
            hash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC80A74AC829DDD92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC80A74AC829DDD92u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                hash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_relationship_group_hash_raw(
        ped: , 
        hash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC80A74AC829DDD92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC80A74AC829DDD92u64;

        invoke_raw_typed!(
            hash,
                ped, 
                hash
        )
    }
}

/// ## Parameters
*



pub fn set_force_footstep_update_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x129466ED55140F8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x129466ED55140F8Du64;
        
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
pub fn set_force_footstep_update_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x129466ED55140F8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x129466ED55140F8Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
Sets a value indicating whether scenario peds should be returned by the next call to a command that returns peds. Eg. GET_CLOSEST_PED.  
```



pub fn set_scenario_peds_to_be_returned_by_next_command_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14F19A8782C8071Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14F19A8782C8071Eu64;
        
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
pub fn set_scenario_peds_to_be_returned_by_next_command_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14F19A8782C8071Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14F19A8782C8071Eu64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ```
Returns true if the given ped has a valid pointer to CPlayerInfo in its CPed class. That's all.
```



pub fn is_ped_a_player_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12534C348C6CB68Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12534C348C6CB68Bu64;
        
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
pub fn is_ped_a_player_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x12534C348C6CB68Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x12534C348C6CB68Bu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
p2 usually 0  
```



pub fn set_ped_can_play_viseme_anims_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF833DDBA3B104D43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF833DDBA3B104D43u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                toggle, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_can_play_viseme_anims_raw(
        ped: , 
        toggle: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF833DDBA3B104D43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF833DDBA3B104D43u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_be_shot_in_vehicle_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7EF1BA83230BA07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7EF1BA83230BA07u64;
        
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
pub fn set_ped_can_be_shot_in_vehicle_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7EF1BA83230BA07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7EF1BA83230BA07u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn has_ped_head_blend_finished_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x654CD0A825161131u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x654CD0A825161131u64;
        
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
pub fn has_ped_head_blend_finished_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x654CD0A825161131u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x654CD0A825161131u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Returns whether the specified ped is shooting.  
```



pub fn is_ped_shooting_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34616828CD07F1A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34616828CD07F1A1u64;
        
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
pub fn is_ped_shooting_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34616828CD07F1A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34616828CD07F1A1u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Set the maximum time a ped can stay underwater. Maximum seems to be 50 seconds.



pub fn set_ped_max_time_underwater_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BA428C528D9E522u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BA428C528D9E522u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_max_time_underwater_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BA428C528D9E522u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BA428C528D9E522u64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_arm_ik_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C3B4D6D13B4C841u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C3B4D6D13B4C841u64;
        
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
pub fn set_ped_can_arm_ik_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C3B4D6D13B4C841u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C3B4D6D13B4C841u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_ped_as_enemy_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02A0C9720B854BFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02A0C9720B854BFAu64;
        
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
pub fn set_ped_as_enemy_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02A0C9720B854BFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02A0C9720B854BFAu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_ped_defensive_area_attached_to_ped_safe(
        
        
            ped: 
        , 
        
        
            attachPed: 
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
        let hash = 0x4EF47FE21698A8B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4EF47FE21698A8B6u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                attachPed, 
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
pub fn set_ped_defensive_area_attached_to_ped_raw(
        ped: , 
        attachPed: , 
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
        let hash = 0x4EF47FE21698A8B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4EF47FE21698A8B6u64;

        invoke_raw_typed!(
            hash,
                ped, 
                attachPed, 
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



pub fn _0x9a77dfd295e29b09_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A77DFD295E29B09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A77DFD295E29B09u64;
        
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
pub fn _0x9a77dfd295e29b09_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A77DFD295E29B09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A77DFD295E29B09u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _set_relationship_group_dont_affect_wanted_level_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5615E0C5EB2BC6E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5615E0C5EB2BC6E2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_relationship_group_dont_affect_wanted_level_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5615E0C5EB2BC6E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5615E0C5EB2BC6E2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _block_ped_dead_body_shocking_events_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE43A13C9E4CCCBCFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE43A13C9E4CCCBCFu64;
        
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
pub fn _block_ped_dead_body_shocking_events_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE43A13C9E4CCCBCFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE43A13C9E4CCCBCFu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_combat_float_safe(
        
        
            ped: 
        , 
        
        
            combatType: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF41B4B141ED981Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF41B4B141ED981Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                combatType, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_combat_float_raw(
        ped: , 
        combatType: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF41B4B141ED981Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF41B4B141ED981Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                combatType, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn _0x0f62619393661d6e_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F62619393661D6Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F62619393661D6Eu64;
        
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
pub fn _0x0f62619393661d6e_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F62619393661D6Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F62619393661D6Eu64;

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



pub fn set_ped_move_anims_blend_out_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E8C908F41584ECDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E8C908F41584ECDu64;
        
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
pub fn set_ped_move_anims_blend_out_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E8C908F41584ECDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E8C908F41584ECDu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// If the ped is attempting to enter a locked vehicle.



pub fn is_ped_trying_to_enter_a_locked_vehicle_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44D28D5DDFE5F68Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44D28D5DDFE5F68Cu64;
        
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
pub fn is_ped_trying_to_enter_a_locked_vehicle_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44D28D5DDFE5F68Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44D28D5DDFE5F68Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _0x9c6a6c19b6c0c496_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C6A6C19B6C0C496u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C6A6C19B6C0C496u64;
        
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
pub fn _0x9c6a6c19b6c0c496_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C6A6C19B6C0C496u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C6A6C19B6C0C496u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Sets the tint index for the hair on the specified ped.

```
NativeDB Introduced: v323
```



pub fn set_ped_hair_tint_safe(
        
        
            ped: 
        , 
        
        
            colorID: 
        , 
        
        
            highlightColorID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4CFFC65454C93A49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4CFFC65454C93A49u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                colorID, 
                highlightColorID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_hair_tint_raw(
        ped: , 
        colorID: , 
        highlightColorID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4CFFC65454C93A49u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4CFFC65454C93A49u64;

        invoke_raw_typed!(
            hash,
                ped, 
                colorID, 
                highlightColorID
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _0xdfe68c4b787e1bfb_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFE68C4B787E1BFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFE68C4B787E1BFBu64;
        
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
pub fn _0xdfe68c4b787e1bfb_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFE68C4B787E1BFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFE68C4B787E1BFBu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Sets the IsHandCuffed (120) config flag on the ped. This blocks the ped from switching weapons (with the exception of switching to `weapon_unarmed`), makes the ped ragdoll on getting punched and forces a different get-up animation after ragdolling. The ped can also not vault over or climb on top of objects.

Used in combination with [SET_ENABLE_BOUND_ANKLES](#_0xC52E0F855C58FC2E) in decompiled scripts.



pub fn set_enable_handcuffs_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF1AF8B5D56542FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF1AF8B5D56542FAu64;
        
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
pub fn set_enable_handcuffs_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF1AF8B5D56542FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF1AF8B5D56542FAu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_ped_decorations_state_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71EAB450D86954A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71EAB450D86954A1u64;
        
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
pub fn get_ped_decorations_state_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71EAB450D86954A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71EAB450D86954A1u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_enable_weapon_blocking_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97A790315D3831FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97A790315D3831FDu64;
        
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
pub fn set_ped_enable_weapon_blocking_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97A790315D3831FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97A790315D3831FDu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
List of movement clipsets:
Thanks to elsewhat for list.
 "ANIM_GROUP_MOVE_BALLISTIC"
 "ANIM_GROUP_MOVE_LEMAR_ALLEY"
 "clipset@move@trash_fast_turn"
 "FEMALE_FAST_RUNNER"
 "missfbi4prepp1_garbageman"
 "move_characters@franklin@fire"
 "move_characters@Jimmy@slow@"
 "move_characters@michael@fire"
 "move_f@flee@a"
 "move_f@scared"
 "move_f@sexy@a"
 "move_heist_lester"
 "move_injured_generic"
 "move_lester_CaneUp"
 "move_m@bag"
 "MOVE_M@BAIL_BOND_NOT_TAZERED"
 "MOVE_M@BAIL_BOND_TAZERED"
 "move_m@brave"
 "move_m@casual@d"
 "move_m@drunk@moderatedrunk"
 "MOVE_M@DRUNK@MODERATEDRUNK"
 "MOVE_M@DRUNK@MODERATEDRUNK_HEAD_UP"
 "MOVE_M@DRUNK@SLIGHTLYDRUNK"
 "MOVE_M@DRUNK@VERYDRUNK"
 "move_m@fire"
 "move_m@gangster@var_e"
 "move_m@gangster@var_f"
 "move_m@gangster@var_i"
 "move_m@JOG@"
 "MOVE_M@PRISON_GAURD"
 "MOVE_P_M_ONE"
 "MOVE_P_M_ONE_BRIEFCASE"
 "move_p_m_zero_janitor"
 "move_p_m_zero_slow"
 "move_ped_bucket"
 "move_ped_crouched"
 "move_ped_mop"
 "MOVE_M@FEMME@"
 "MOVE_F@FEMME@"
 "MOVE_M@GANGSTER@NG"
 "MOVE_F@GANGSTER@NG"
 "MOVE_M@POSH@"
 "MOVE_F@POSH@"
 "MOVE_M@TOUGH_GUY@"
 "MOVE_F@TOUGH_GUY@"
~ NotCrunchyTaco
```



pub fn set_ped_movement_clipset_safe(
        
        
            ped: 
        , 
        
        
            clipSet: 
        , 
        
        
            transitionSpeed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF8A94EDE7712BEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF8A94EDE7712BEFu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                clipSet, 
                transitionSpeed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_movement_clipset_raw(
        ped: , 
        clipSet: , 
        transitionSpeed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF8A94EDE7712BEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF8A94EDE7712BEFu64;

        invoke_raw_typed!(
            hash,
                ped, 
                clipSet, 
                transitionSpeed
        )
    }
}

/// ```
Min and max are usually 100.0 and 200.0
```



pub fn set_pop_control_sphere_this_frame_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            min: 
        , 
        
        
            max: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8C3BE3EE94CAF2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8C3BE3EE94CAF2Du64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                min, 
                max
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_pop_control_sphere_this_frame_raw(
        x: , 
        y: , 
        z: , 
        min: , 
        max: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8C3BE3EE94CAF2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8C3BE3EE94CAF2Du64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                min, 
                max
        )
    }
}

/// ## Parameters
*



pub fn play_facial_anim_safe(
        
        
            ped: 
        , 
        
        
            animName: 
        , 
        
        
            animDict: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1E65CA8AC9C00EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1E65CA8AC9C00EDu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                animName, 
                animDict
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn play_facial_anim_raw(
        ped: , 
        animName: , 
        animDict: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE1E65CA8AC9C00EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE1E65CA8AC9C00EDu64;

        invoke_raw_typed!(
            hash,
                ped, 
                animName, 
                animDict
        )
    }
}

/// ## Parameters
*



pub fn get_ped_accuracy_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37F4AD56ECBC0CD6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37F4AD56ECBC0CD6u64;
        
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
pub fn get_ped_accuracy_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37F4AD56ECBC0CD6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37F4AD56ECBC0CD6u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn remove_group_safe(
        
        
            groupId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EB2F69076AF7053u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EB2F69076AF7053u64;
        
        let result = invoke_raw!(
            hash,
                groupId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_group_raw(
        groupId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EB2F69076AF7053u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EB2F69076AF7053u64;

        invoke_raw_typed!(
            hash,
                groupId
        )
    }
}

/// ## Parameters
*



pub fn get_melee_target_for_ped_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x18A3E9EE1297FD39u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x18A3E9EE1297FD39u64;
        
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
pub fn get_melee_target_for_ped_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x18A3E9EE1297FD39u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x18A3E9EE1297FD39u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
from fm_mission_controller.c4 (variable names changed for clarity):  
int groupID = PLAYER::GET_PLAYER_GROUP(PLAYER::PLAYER_ID());  
PED::GET_GROUP_SIZE(group, &unused, &groupSize);  
if (groupSize >= 1) {  
. . . . for (int memberNumber = 0; memberNumber < groupSize; memberNumber++) {  
. . . . . . . . Ped ped1 = PED::GET_PED_AS_GROUP_MEMBER(groupID, memberNumber);  
. . . . . . . . //and so on  
```



pub fn get_ped_as_group_member_safe(
        
        
            groupID: 
        , 
        
        
            memberNumber: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51455483CF23ED97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51455483CF23ED97u64;
        
        let result = invoke_raw!(
            hash,
                groupID, 
                memberNumber
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_as_group_member_raw(
        groupID: , 
        memberNumber: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51455483CF23ED97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51455483CF23ED97u64;

        invoke_raw_typed!(
            hash,
                groupID, 
                memberNumber
        )
    }
}

/// ## Parameters
*



pub fn _0x733c87d4ce22bea2_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x733C87D4CE22BEA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x733C87D4CE22BEA2u64;
        
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
pub fn _0x733c87d4ce22bea2_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x733C87D4CE22BEA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x733C87D4CE22BEA2u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_scenario_peds_spawn_in_sphere_area_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            range: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28157D43CF600981u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28157D43CF600981u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                range, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_scenario_peds_spawn_in_sphere_area_raw(
        x: , 
        y: , 
        z: , 
        range: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28157D43CF600981u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28157D43CF600981u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                range, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_peek_in_cover_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC514825C507E3736u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC514825C507E3736u64;
        
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
pub fn set_ped_can_peek_in_cover_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC514825C507E3736u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC514825C507E3736u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_ped_running_mobile_phone_task_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2AFE52F782F25775u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2AFE52F782F25775u64;
        
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
pub fn is_ped_running_mobile_phone_task_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2AFE52F782F25775u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2AFE52F782F25775u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_synchronized_scene_hold_last_frame_safe(
        
        
            sceneID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F2F4F13AC5257EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F2F4F13AC5257EFu64;
        
        let result = invoke_raw!(
            hash,
                sceneID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_synchronized_scene_hold_last_frame_raw(
        sceneID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F2F4F13AC5257EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F2F4F13AC5257EFu64;

        invoke_raw_typed!(
            hash,
                sceneID
        )
    }
}

/// ## Parameters
*



pub fn is_ped_climbing_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53E8CB4F48BFE623u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53E8CB4F48BFE623u64;
        
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
pub fn is_ped_climbing_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53E8CB4F48BFE623u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53E8CB4F48BFE623u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_defensive_sphere_attached_to_ped_safe(
        
        
            ped: 
        , 
        
        
            target: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
        , 
        
        
            radius: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9B8F91AAD3B953Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9B8F91AAD3B953Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                target, 
                xOffset, 
                yOffset, 
                zOffset, 
                radius, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_defensive_sphere_attached_to_ped_raw(
        ped: , 
        target: , 
        xOffset: , 
        yOffset: , 
        zOffset: , 
        radius: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9B8F91AAD3B953Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9B8F91AAD3B953Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                target, 
                xOffset, 
                yOffset, 
                zOffset, 
                radius, 
                p6
        )
    }
}

/// Causes Ped to ragdoll on collision with any object (e.g Running into trashcan). If applied to player you will sometimes trip on the sidewalk.
Needs to be recalled after each ragdoll from a Collision.



pub fn set_ped_ragdoll_on_collision_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0A4F1BBF4FA7497u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0A4F1BBF4FA7497u64;
        
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
pub fn set_ped_ragdoll_on_collision_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0A4F1BBF4FA7497u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0A4F1BBF4FA7497u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
Detect if ped is sitting in the specified vehicle  
[True/False]  
```



pub fn is_ped_sitting_in_vehicle_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA808AA1D79230FC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA808AA1D79230FC2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_sitting_in_vehicle_raw(
        ped: , 
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA808AA1D79230FC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA808AA1D79230FC2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle
        )
    }
}

/// ```
It adds the wetness level to the player clothing/outfit. As if player just got out from water surface.  
```



pub fn set_ped_wetness_height_safe(
        
        
            ped: 
        , 
        
        
            height: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44CB6447D2571AA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44CB6447D2571AA0u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                height
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_wetness_height_raw(
        ped: , 
        height: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44CB6447D2571AA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44CB6447D2571AA0u64;

        invoke_raw_typed!(
            hash,
                ped, 
                height
        )
    }
}

/// ## Parameters
*



pub fn _set_ped_helmet_unk_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F7325574E41B44Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F7325574E41B44Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn _set_ped_helmet_unk_raw(
        ped: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F7325574E41B44Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F7325574E41B44Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn set_ped_steers_around_objects_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1509C089ADC208BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1509C089ADC208BFu64;
        
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
pub fn set_ped_steers_around_objects_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1509C089ADC208BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1509C089ADC208BFu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_jack_target_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5486A79D9FBD342Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5486A79D9FBD342Du64;
        
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
pub fn get_jack_target_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5486A79D9FBD342Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5486A79D9FBD342Du64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_using_action_mode_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00E73468D085F745u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00E73468D085F745u64;
        
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
pub fn is_ped_using_action_mode_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00E73468D085F745u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00E73468D085F745u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _get_ped_task_combat_target_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32C27A11307B01CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32C27A11307B01CCu64;
        
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
pub fn _get_ped_task_combat_target_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32C27A11307B01CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32C27A11307B01CCu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0xc56fbf2f228e1dac_safe(
        
        
            modelHash: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC56FBF2F228E1DACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC56FBF2F228E1DACu64;
        
        let result = invoke_raw!(
            hash,
                modelHash, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc56fbf2f228e1dac_raw(
        modelHash: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC56FBF2F228E1DACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC56FBF2F228E1DACu64;

        invoke_raw_typed!(
            hash,
                modelHash, 
                p1, 
                p2
        )
    }
}

/// ```
p1 may be a BOOL representing whether or not the group even exists  
```



pub fn get_group_size_safe(
        
        
            groupID: 
        , 
        
        
            unknown: 
        , 
        
        
            sizeInMembers: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DE69FE35CA09A45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DE69FE35CA09A45u64;
        
        let result = invoke_raw!(
            hash,
                groupID, 
                unknown, 
                sizeInMembers
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_group_size_raw(
        groupID: , 
        unknown: , 
        sizeInMembers: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DE69FE35CA09A45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DE69FE35CA09A45u64;

        invoke_raw_typed!(
            hash,
                groupID, 
                unknown, 
                sizeInMembers
        )
    }
}

/// ```
Copies ped's components and props to targetPed.
```



pub fn clone_ped_to_target_safe(
        
        
            ped: 
        , 
        
        
            targetPed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE952D6431689AD9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE952D6431689AD9Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                targetPed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clone_ped_to_target_raw(
        ped: , 
        targetPed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE952D6431689AD9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE952D6431689AD9Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                targetPed
        )
    }
}

/// ## Parameters
*



pub fn is_ped_in_cover_facing_left_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x845333B3150583ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x845333B3150583ABu64;
        
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
pub fn is_ped_in_cover_facing_left_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x845333B3150583ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x845333B3150583ABu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_ped_prop_texture_index_safe(
        
        
            ped: 
        , 
        
        
            componentId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE131A28626F81AB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE131A28626F81AB2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                componentId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_prop_texture_index_raw(
        ped: , 
        componentId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE131A28626F81AB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE131A28626F81AB2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                componentId
        )
    }
}

/// ## Parameters
*



pub fn set_synchronized_scene_origin_safe(
        
        
            sceneID: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            roll: 
        , 
        
        
            pitch: 
        , 
        
        
            yaw: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6ACF6B7225801CD7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6ACF6B7225801CD7u64;
        
        let result = invoke_raw!(
            hash,
                sceneID, 
                x, 
                y, 
                z, 
                roll, 
                pitch, 
                yaw, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_synchronized_scene_origin_raw(
        sceneID: , 
        x: , 
        y: , 
        z: , 
        roll: , 
        pitch: , 
        yaw: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6ACF6B7225801CD7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6ACF6B7225801CD7u64;

        invoke_raw_typed!(
            hash,
                sceneID, 
                x, 
                y, 
                z, 
                roll, 
                pitch, 
                yaw, 
                p7
        )
    }
}

/// ## Parameters
*



pub fn remove_ped_from_group_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED74007FFB146BC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED74007FFB146BC2u64;
        
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
pub fn remove_ped_from_group_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED74007FFB146BC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED74007FFB146BC2u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Only 1 and 2 appear in the scripts. combatbehaviour.meta seems to only have TLR_SearchForTarget for all peds, but we don't know if that's 1 or 2.  
```



pub fn set_ped_target_loss_response_safe(
        
        
            ped: 
        , 
        
        
            responseType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0703B9079823DA4Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0703B9079823DA4Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                responseType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_target_loss_response_raw(
        ped: , 
        responseType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0703B9079823DA4Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0703B9079823DA4Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                responseType
        )
    }
}

/// Ped types:

```cpp
enum ePedType
{
	PED_TYPE_PLAYER_0 = 0,
	PED_TYPE_PLAYER_1 = 1,
	PED_TYPE_NETWORK_PLAYER = 2,
	PED_TYPE_PLAYER_2 = 3,
	PED_TYPE_CIVMALE = 4,
	PED_TYPE_CIVFEMALE = 5,
	PED_TYPE_COP = 6,
	PED_TYPE_GANG_ALBANIAN = 7,
	PED_TYPE_GANG_BIKER_1 = 8,
	PED_TYPE_GANG_BIKER_2 = 9,
	PED_TYPE_GANG_ITALIAN = 10,
	PED_TYPE_GANG_RUSSIAN = 11,
	PED_TYPE_GANG_RUSSIAN_2 = 12,
	PED_TYPE_GANG_IRISH = 13,
	PED_TYPE_GANG_JAMAICAN = 14,
	PED_TYPE_GANG_AFRICAN_AMERICAN = 15,
	PED_TYPE_GANG_KOREAN = 16,
	PED_TYPE_GANG_CHINESE_JAPANESE = 17,
	PED_TYPE_GANG_PUERTO_RICAN = 18,
	PED_TYPE_DEALER = 19,
	PED_TYPE_MEDIC = 20,
	PED_TYPE_FIREMAN = 21,
	PED_TYPE_CRIMINAL = 22,
	PED_TYPE_BUM = 23,
	PED_TYPE_PROSTITUTE = 24,
	PED_TYPE_SPECIAL = 25,
	PED_TYPE_MISSION = 26,
	PED_TYPE_SWAT = 27,
	PED_TYPE_ANIMAL = 28,
	PED_TYPE_ARMY = 29
};
```



pub fn get_ped_type_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF059E1E4C01E63Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF059E1E4C01E63Cu64;
        
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
pub fn get_ped_type_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF059E1E4C01E63Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF059E1E4C01E63Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _is_ped_blush_color_valid_safe(
        
        
            colorID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x604E810189EE3A59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x604E810189EE3A59u64;
        
        let result = invoke_raw!(
            hash,
                colorID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_ped_blush_color_valid_raw(
        colorID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x604E810189EE3A59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x604E810189EE3A59u64;

        invoke_raw_typed!(
            hash,
                colorID
        )
    }
}

/// Indicates whether this ped's health is below its injured threshold.
The default threshold is 100, these are stored in the `pedhealth.meta` file located in `common:\data\`



pub fn is_ped_injured_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84A2DD9AC37C35C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84A2DD9AC37C35C1u64;
        
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
pub fn is_ped_injured_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84A2DD9AC37C35C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84A2DD9AC37C35C1u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn clear_all_ped_vehicle_forced_seat_usage_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6CA85E7259CE16Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6CA85E7259CE16Bu64;
        
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
pub fn clear_all_ped_vehicle_forced_seat_usage_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6CA85E7259CE16Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6CA85E7259CE16Bu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Returns the ped's alertness (0-3).  
Values :   
0 : Neutral  
1 : Heard something (gun shot, hit, etc)  
2 : Knows (the origin of the event)  
3 : Fully alerted (is facing the event?)  
If the Ped does not exist, returns -1.  
```



pub fn get_ped_alertness_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6AA118530443FD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6AA118530443FD2u64;
        
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
pub fn get_ped_alertness_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6AA118530443FD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6AA118530443FD2u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Returns true if the ped doesn't do any movement. If the ped is being pushed forwards by using APPLY_FORCE_TO_ENTITY for example, the function returns false.  
```



pub fn is_ped_stopped_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x530944F6F4B8A214u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x530944F6F4B8A214u64;
        
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
pub fn is_ped_stopped_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x530944F6F4B8A214u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x530944F6F4B8A214u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
"IK" stands for "Inverse kinematics." I assume this has something to do with how the ped uses his legs to balance. In the scripts, the second parameter is always an int with a value of 2, 0, or sometimes 1  
```



pub fn set_ped_leg_ik_mode_safe(
        
        
            ped: 
        , 
        
        
            mode: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC396F5B86FF9FEBDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC396F5B86FF9FEBDu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                mode
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_leg_ik_mode_raw(
        ped: , 
        mode: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC396F5B86FF9FEBDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC396F5B86FF9FEBDu64;

        invoke_raw_typed!(
            hash,
                ped, 
                mode
        )
    }
}

/// ```
Returns whether the specified ped is hurt.  
```



pub fn is_ped_hurt_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5983BB449D7FDB12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5983BB449D7FDB12u64;
        
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
pub fn is_ped_hurt_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5983BB449D7FDB12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5983BB449D7FDB12u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn can_knock_ped_off_vehicle_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51AC07A44D4F5B8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51AC07A44D4F5B8Au64;
        
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
pub fn can_knock_ped_off_vehicle_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51AC07A44D4F5B8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51AC07A44D4F5B8Au64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn reset_ped_ragdoll_timer_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FA4664CF62E47E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FA4664CF62E47E8u64;
        
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
pub fn reset_ped_ragdoll_timer_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FA4664CF62E47E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FA4664CF62E47E8u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _does_relationship_group_exist_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC6E3B6BB69501F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC6E3B6BB69501F1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _does_relationship_group_exist_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCC6E3B6BB69501F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCC6E3B6BB69501F1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
value ranges from 0 to 3.  
```



pub fn set_ped_alertness_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBA71115ED9941A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBA71115ED9941A6u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_alertness_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBA71115ED9941A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBA71115ED9941A6u64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// ```
Damage Packs:  
"SCR_TrevorTreeBang"  
"HOSPITAL_0"  
"HOSPITAL_1"  
"HOSPITAL_2"  
"HOSPITAL_3"  
"HOSPITAL_4"  
"HOSPITAL_5"  
"HOSPITAL_6"  
"HOSPITAL_7"  
"HOSPITAL_8"  
"HOSPITAL_9"  
"SCR_Dumpster"  
"BigHitByVehicle"  
"SCR_Finale_Michael_Face"  
"SCR_Franklin_finb"  
"SCR_Finale_Michael"  
"SCR_Franklin_finb2"  
"Explosion_Med"  
"SCR_Torture"  
"SCR_TracySplash"  
"Skin_Melee_0"  
Additional damage packs:  
gist.github.com/alexguirre/f3f47f75ddcf617f416f3c8a55ae2227  
```



pub fn apply_ped_damage_pack_safe(
        
        
            ped: 
        , 
        
        
            damagePack: 
        , 
        
        
            damage: 
        , 
        
        
            mult: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46DF918788CB093Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46DF918788CB093Fu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                damagePack, 
                damage, 
                mult
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn apply_ped_damage_pack_raw(
        ped: , 
        damagePack: , 
        damage: , 
        mult: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46DF918788CB093Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46DF918788CB093Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                damagePack, 
                damage, 
                mult
        )
    }
}

/// ## Parameters
*



pub fn set_ped_helmet_texture_index_safe(
        
        
            ped: 
        , 
        
        
            textureIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1550C4BD22582E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1550C4BD22582E2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                textureIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_helmet_texture_index_raw(
        ped: , 
        textureIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1550C4BD22582E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1550C4BD22582E2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                textureIndex
        )
    }
}

/// ```
SET_PED_*
```



pub fn _0xafc976fd0580c7b3_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFC976FD0580C7B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFC976FD0580C7B3u64;
        
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
pub fn _0xafc976fd0580c7b3_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFC976FD0580C7B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFC976FD0580C7B3u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_number_of_ped_prop_drawable_variations_safe(
        
        
            ped: 
        , 
        
        
            propId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FAF9754E789FB47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FAF9754E789FB47u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                propId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_number_of_ped_prop_drawable_variations_raw(
        ped: , 
        propId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FAF9754E789FB47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FAF9754E789FB47u64;

        invoke_raw_typed!(
            hash,
                ped, 
                propId
        )
    }
}

/// ## Parameters
*



pub fn _register_pedheadshot_3_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA8805A1108A2515u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA8805A1108A2515u64;
        
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
pub fn _register_pedheadshot_3_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA8805A1108A2515u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA8805A1108A2515u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Returns:  
-1: Normal  
0: Wearing parachute on back  
1: Parachute opening  
2: Parachute open  
3: Falling to doom (e.g. after exiting parachute)  
Normal means no parachute?  
```



pub fn get_ped_parachute_state_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79CFD9827CC979B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79CFD9827CC979B6u64;
        
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
pub fn get_ped_parachute_state_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79CFD9827CC979B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79CFD9827CC979B6u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_relationship_group_default_hash_safe(
        
        
            ped: 
        , 
        
        
            hash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADB3F206518799E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADB3F206518799E8u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                hash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_relationship_group_default_hash_raw(
        ped: , 
        hash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADB3F206518799E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADB3F206518799E8u64;

        invoke_raw_typed!(
            hash,
                ped, 
                hash
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_torso_ik_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2B7106D37947CE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2B7106D37947CE0u64;
        
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
pub fn set_ped_can_torso_ik_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2B7106D37947CE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2B7106D37947CE0u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_ped_in_any_police_vehicle_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BD04E29640C9C12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BD04E29640C9C12u64;
        
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
pub fn is_ped_in_any_police_vehicle_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BD04E29640C9C12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BD04E29640C9C12u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Returns the group id of which the specified ped is a member of.  
```



pub fn get_ped_group_index_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF162E133B4E7A675u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF162E133B4E7A675u64;
        
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
pub fn get_ped_group_index_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF162E133B4E7A675u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF162E133B4E7A675u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _is_ped_lipstick_color_valid_safe(
        
        
            colorID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0525A2C2562F3CD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0525A2C2562F3CD4u64;
        
        let result = invoke_raw!(
            hash,
                colorID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_ped_lipstick_color_valid_raw(
        colorID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0525A2C2562F3CD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0525A2C2562F3CD4u64;

        invoke_raw_typed!(
            hash,
                colorID
        )
    }
}

/// ## Parameters
*



pub fn _0x25361a96e0f7e419_safe(
        
        
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
        let hash = 0x25361A96E0F7E419u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25361A96E0F7E419u64;
        
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
pub fn _0x25361a96e0f7e419_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25361A96E0F7E419u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25361A96E0F7E419u64;

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



pub fn _0xa660faf550eb37e5_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA660FAF550EB37E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA660FAF550EB37E5u64;
        
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
pub fn _0xa660faf550eb37e5_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA660FAF550EB37E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA660FAF550EB37E5u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _is_ped_blush_color_valid_2_safe(
        
        
            colorId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF41B5D290C99A3D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF41B5D290C99A3D6u64;
        
        let result = invoke_raw!(
            hash,
                colorId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_ped_blush_color_valid_2_raw(
        colorId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF41B5D290C99A3D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF41B5D290C99A3D6u64;

        invoke_raw_typed!(
            hash,
                colorId
        )
    }
}

/// ## Parameters
*



pub fn set_ped_visual_field_center_angle_safe(
        
        
            ped: 
        , 
        
        
            angle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B6405E8AB34A907u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B6405E8AB34A907u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                angle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_visual_field_center_angle_raw(
        ped: , 
        angle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B6405E8AB34A907u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B6405E8AB34A907u64;

        invoke_raw_typed!(
            hash,
                ped, 
                angle
        )
    }
}

/// ## Parameters
*



pub fn _0x2b694afcf64e6994_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B694AFCF64E6994u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B694AFCF64E6994u64;
        
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
pub fn _0x2b694afcf64e6994_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B694AFCF64E6994u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B694AFCF64E6994u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn is_ped_jacking_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AE4FF911DFB61DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AE4FF911DFB61DAu64;
        
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
pub fn is_ped_jacking_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AE4FF911DFB61DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AE4FF911DFB61DAu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
SET_A*
```



pub fn _0x87ddeb611b329a9c_safe(
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87DDEB611B329A9Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87DDEB611B329A9Cu64;
        
        let result = invoke_raw!(
            hash,
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x87ddeb611b329a9c_raw(
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87DDEB611B329A9Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87DDEB611B329A9Cu64;

        invoke_raw_typed!(
            hash,
                multiplier
        )
    }
}

/// This native sets the glow intensity of illuminated clothing items.

This native does



pub fn _set_ped_emissive_intensity_safe(
        
        
            ped: 
        , 
        
        
            intensity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E90D746056E273Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E90D746056E273Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                intensity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_ped_emissive_intensity_raw(
        ped: , 
        intensity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E90D746056E273Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E90D746056E273Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                intensity
        )
    }
}

/// ## Parameters
*



pub fn is_ped_defensive_area_active_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA63D9FE45412247u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA63D9FE45412247u64;
        
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
pub fn is_ped_defensive_area_active_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA63D9FE45412247u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA63D9FE45412247u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_ped_keep_task_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x971D38760FBC02EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x971D38760FBC02EFu64;
        
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
pub fn set_ped_keep_task_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x971D38760FBC02EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x971D38760FBC02EFu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn clear_all_ped_props_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD8A7537A9B52F06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD8A7537A9B52F06u64;
        
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
pub fn clear_all_ped_props_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD8A7537A9B52F06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD8A7537A9B52F06u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_any_hostile_ped_near_point_safe(
        
        
            ped: 
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
        let hash = 0x68772DB2B2526F9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68772DB2B2526F9Fu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn is_any_hostile_ped_near_point_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68772DB2B2526F9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68772DB2B2526F9Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// ## Parameters
*



pub fn is_ped_jumping_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEDABC5900A0BF97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEDABC5900A0BF97u64;
        
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
pub fn is_ped_jumping_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEDABC5900A0BF97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEDABC5900A0BF97u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_in_any_plane_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FFF4CFC74D8FB80u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FFF4CFC74D8FB80u64;
        
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
pub fn is_ped_in_any_plane_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FFF4CFC74D8FB80u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FFF4CFC74D8FB80u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_id_range_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF107E836A70DCE05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF107E836A70DCE05u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_id_range_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF107E836A70DCE05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF107E836A70DCE05u64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// ## Parameters
*



pub fn set_ped_sphere_defensive_area_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            p5: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D3151A373974804u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D3151A373974804u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                radius, 
                p5, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_sphere_defensive_area_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        radius: , 
        p5: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D3151A373974804u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D3151A373974804u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                radius, 
                p5, 
                p6
        )
    }
}

/// ```
accuracy = 0-100, 100 being perfectly accurate
```



pub fn set_ped_accuracy_safe(
        
        
            ped: 
        , 
        
        
            accuracy: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AEFB85C1D49DEB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AEFB85C1D49DEB6u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                accuracy
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_accuracy_raw(
        ped: , 
        accuracy: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AEFB85C1D49DEB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AEFB85C1D49DEB6u64;

        invoke_raw_typed!(
            hash,
                ped, 
                accuracy
        )
    }
}

/// ```
SET_PED_STE*
```



pub fn _0x2016c603d6b8987c_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2016C603D6B8987Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2016C603D6B8987Cu64;
        
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
pub fn _0x2016c603d6b8987c_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2016C603D6B8987Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2016C603D6B8987Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_be_dragged_out_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1670E958EEE24E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1670E958EEE24E5u64;
        
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
pub fn set_ped_can_be_dragged_out_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1670E958EEE24E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1670E958EEE24E5u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x5b6010b3cbc29095_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B6010B3CBC29095u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B6010B3CBC29095u64;
        
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
pub fn _0x5b6010b3cbc29095_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B6010B3CBC29095u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B6010B3CBC29095u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_ped_max_time_in_water_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43C851690662113Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43C851690662113Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_max_time_in_water_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43C851690662113Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43C851690662113Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// ## Parameters
*



pub fn set_enable_ped_enveff_scale_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2C5AA0C0E8D0F1Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2C5AA0C0E8D0F1Eu64;
        
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
pub fn set_enable_ped_enveff_scale_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2C5AA0C0E8D0F1Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2C5AA0C0E8D0F1Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_create_random_cops_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x102E68B2024D536Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x102E68B2024D536Du64;
        
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
pub fn set_create_random_cops_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x102E68B2024D536Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x102E68B2024D536Du64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn clear_ped_prop_safe(
        
        
            ped: 
        , 
        
        
            propId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0943E5B8E078E76Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0943E5B8E078E76Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                propId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_ped_prop_raw(
        ped: , 
        propId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0943E5B8E078E76Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0943E5B8E078E76Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                propId
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_head_ik_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC11C18092C5530DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC11C18092C5530DCu64;
        
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
pub fn set_ped_can_head_ik_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC11C18092C5530DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC11C18092C5530DCu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_ped_combat_range_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9D9F7F2DB8E2FA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9D9F7F2DB8E2FA0u64;
        
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
pub fn get_ped_combat_range_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9D9F7F2DB8E2FA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9D9F7F2DB8E2FA0u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Found one occurence in re_crashrescue.c4  
PED::APPLY_PED_BLOOD(l_4B, 3, 0.0, 0.0, 0.0, "wound_sheet");  
```



pub fn apply_ped_blood_safe(
        
        
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
        
        
            woundType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83F7E01C7B769A26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83F7E01C7B769A26u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                boneIndex, 
                xRot, 
                yRot, 
                zRot, 
                woundType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn apply_ped_blood_raw(
        ped: , 
        boneIndex: , 
        xRot: , 
        yRot: , 
        zRot: , 
        woundType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83F7E01C7B769A26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83F7E01C7B769A26u64;

        invoke_raw_typed!(
            hash,
                ped, 
                boneIndex, 
                xRot, 
                yRot, 
                zRot, 
                woundType
        )
    }
}

/// ## Parameters
*



pub fn set_ped_random_props_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC44AA05345C992C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC44AA05345C992C6u64;
        
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
pub fn set_ped_random_props_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC44AA05345C992C6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC44AA05345C992C6u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// **This native does absolutely nothing, just a nullsub**

```
Points to the same function as for example GET_RANDOM_VEHICLE_MODEL_IN_MEMORY and it does absolutely nothing.  
```



pub fn set_ped_plays_head_on_horn_anim_when_dies_in_vehicle_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94D94BF1A75AED3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94D94BF1A75AED3Du64;
        
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
pub fn set_ped_plays_head_on_horn_anim_when_dies_in_vehicle_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94D94BF1A75AED3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94D94BF1A75AED3Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_be_targeted_without_los_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4328652AE5769C71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4328652AE5769C71u64;
        
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
pub fn set_ped_can_be_targeted_without_los_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4328652AE5769C71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4328652AE5769C71u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
This function will simply bring the dead person back to life.
Try not to use it alone, since using this function alone, will make peds fall through ground in hell(well for the most of the times).
Instead, before calling this function, you may want to declare the position, where your Resurrected ped to be spawn at.(For instance, Around 2 floats of Player's current position.)
Also, disabling any assigned task immediately helped in the number of scenarios, where If you want peds to perform certain decided tasks.
```



pub fn resurrect_ped_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71BC8E838B9C6035u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71BC8E838B9C6035u64;
        
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
pub fn resurrect_ped_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71BC8E838B9C6035u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71BC8E838B9C6035u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
p0: Ped Handle  
p1: int i | 0 <= i <= 27  
p1 probably refers to the attributes configured in combatbehavior.meta. There are 13. Example:  
<BlindFireChance value="0.1"/>  
<WeaponShootRateModifier value="1.0"/>  
<TimeBetweenBurstsInCover value="1.25"/>  
<BurstDurationInCover value="2.0"/>  
<TimeBetweenPeeks value="10.0"/>  
<WeaponAccuracy value="0.18"/>  
<FightProficiency value="0.8"/>  
<StrafeWhenMovingChance value="1.0"/>  
<WalkWhenStrafingChance value="0.0"/>  
<AttackWindowDistanceForCover value="55.0"/>  
<TimeToInvalidateInjuredTarget value="9.0"/>  
<TriggerChargeTime_Near value="4.0"/>  
<TriggerChargeTime_Far value="10.0"/>



pub fn get_combat_float_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52DFF8A10508090Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52DFF8A10508090Au64;
        
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
pub fn get_combat_float_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52DFF8A10508090Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52DFF8A10508090Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_force_step_type_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            type: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB968B53FC7F916Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB968B53FC7F916Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                type, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_force_step_type_raw(
        ped: , 
        p1: , 
        type: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB968B53FC7F916Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB968B53FC7F916Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                type, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn set_ped_cloth_package_index_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78C4E9961DB3EB5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78C4E9961DB3EB5Bu64;
        
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
pub fn set_ped_cloth_package_index_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78C4E9961DB3EB5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78C4E9961DB3EB5Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
Found in the b617d scripts:
PED::_9DBA107B4937F809(v_7, "trevor_heist_cover_2h");
SET_PED_MO*
```



pub fn _set_ped_cover_clipset_override_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DBA107B4937F809u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DBA107B4937F809u64;
        
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
pub fn _set_ped_cover_clipset_override_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DBA107B4937F809u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DBA107B4937F809u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _get_ped_visual_field_center_angle_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF2C71A32CAD5FBDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF2C71A32CAD5FBDu64;
        
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
pub fn _get_ped_visual_field_center_angle_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF2C71A32CAD5FBDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF2C71A32CAD5FBDu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_fleeing_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBCCE00B381F8482u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBCCE00B381F8482u64;
        
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
pub fn is_ped_fleeing_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBCCE00B381F8482u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBCCE00B381F8482u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _set_enable_scuba_gear_light_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE2476B9EE4A094Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE2476B9EE4A094Fu64;
        
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
pub fn _set_enable_scuba_gear_light_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE2476B9EE4A094Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE2476B9EE4A094Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
gtaforums.com/topic/885580-ped-headshotmugshot-txd/  
```



pub fn get_pedheadshot_txd_string_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB4EACD4AD0A5D6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB4EACD4AD0A5D6Bu64;
        
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
pub fn get_pedheadshot_txd_string_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB4EACD4AD0A5D6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB4EACD4AD0A5D6Bu64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ```
Related to Peds dropping pickup_health_snack; p0 is a value between [0.0, 1.0] that corresponds to drop rate
```



pub fn _0xff4803bc019852d9_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF4803BC019852D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF4803BC019852D9u64;
        
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
pub fn _0xff4803bc019852d9_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF4803BC019852D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF4803BC019852D9u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x2735233a786b1bef_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2735233A786B1BEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2735233A786B1BEFu64;
        
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
pub fn _0x2735233a786b1bef_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2735233A786B1BEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2735233A786B1BEFu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Return value



pub fn spawnpoints_get_num_search_results_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA635C11B8C44AFC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA635C11B8C44AFC2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn spawnpoints_get_num_search_results_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA635C11B8C44AFC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA635C11B8C44AFC2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_ped_min_move_blend_ratio_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01A898D26E2333DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01A898D26E2333DDu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_min_move_blend_ratio_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01A898D26E2333DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01A898D26E2333DDu64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// ## Parameters
*



pub fn set_ped_panic_exit_scenario_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE07FF6495D52E2Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE07FF6495D52E2Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn set_ped_panic_exit_scenario_raw(
        ped: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE07FF6495D52E2Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE07FF6495D52E2Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn set_ped_visual_field_peripheral_range_safe(
        
        
            ped: 
        , 
        
        
            range: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C74B0BC831B753Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C74B0BC831B753Au64;
        
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
pub fn set_ped_visual_field_peripheral_range_raw(
        ped: , 
        range: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C74B0BC831B753Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C74B0BC831B753Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                range
        )
    }
}

/// ## Parameters
*



pub fn is_ped_in_any_sub_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFBFC01CCFB35D99Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFBFC01CCFB35D99Eu64;
        
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
pub fn is_ped_in_any_sub_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFBFC01CCFB35D99Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFBFC01CCFB35D99Eu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// SET_A*

```
NativeDB Introduced: v1734
```



pub fn _0xfab944d4d481accb_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAB944D4D481ACCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAB944D4D481ACCBu64;
        
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
pub fn _0xfab944d4d481accb_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAB944D4D481ACCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAB944D4D481ACCBu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
Sweat is set to 100.0 or 0.0 in the decompiled scripts.  
```



pub fn set_ped_sweat_safe(
        
        
            ped: 
        , 
        
        
            sweat: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27B0405F59637D1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27B0405F59637D1Fu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                sweat
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_sweat_raw(
        ped: , 
        sweat: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27B0405F59637D1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27B0405F59637D1Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                sweat
        )
    }
}

/// ```
p1 is nearly always 0 in the scripts.  
```



pub fn is_ped_in_cover_safe(
        
        
            ped: 
        , 
        
        
            exceptUseWeapon: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60DFD0691A170B88u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60DFD0691A170B88u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                exceptUseWeapon
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_in_cover_raw(
        ped: , 
        exceptUseWeapon: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60DFD0691A170B88u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60DFD0691A170B88u64;

        invoke_raw_typed!(
            hash,
                ped, 
                exceptUseWeapon
        )
    }
}

/// ## Parameters
*



pub fn _is_ped_swapping_weapon_safe(
        
        
            Ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3795688A307E1EB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3795688A307E1EB6u64;
        
        let result = invoke_raw!(
            hash,
                Ped
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_ped_swapping_weapon_raw(
        Ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3795688A307E1EB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3795688A307E1EB6u64;

        invoke_raw_typed!(
            hash,
                Ped
        )
    }
}

/// ## Parameters
*



pub fn force_ped_ai_and_animation_update_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2208438012482A1Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2208438012482A1Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_ped_ai_and_animation_update_raw(
        ped: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2208438012482A1Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2208438012482A1Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn _is_ped_doing_beast_jump_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x451D05012CCEC234u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x451D05012CCEC234u64;
        
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
pub fn _is_ped_doing_beast_jump_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x451D05012CCEC234u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x451D05012CCEC234u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_aiming_from_cover_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3998B1276A3300E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3998B1276A3300E5u64;
        
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
pub fn is_ped_aiming_from_cover_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3998B1276A3300E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3998B1276A3300E5u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_get_out_upside_down_vehicle_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC0ED94165A48BC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC0ED94165A48BC2u64;
        
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
pub fn set_ped_get_out_upside_down_vehicle_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC0ED94165A48BC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC0ED94165A48BC2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ```
REQUEST_*
```



pub fn _0x75ba1cb3b7d40caf_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75BA1CB3B7D40CAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75BA1CB3B7D40CAFu64;
        
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
pub fn _0x75ba1cb3b7d40caf_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75BA1CB3B7D40CAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75BA1CB3B7D40CAFu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn is_ped_jumping_out_of_vehicle_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x433DDFFE2044B636u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x433DDFFE2044B636u64;
        
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
pub fn is_ped_jumping_out_of_vehicle_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x433DDFFE2044B636u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x433DDFFE2044B636u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn request_action_mode_asset_safe(
        
        
            asset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x290E2780BB7AA598u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x290E2780BB7AA598u64;
        
        let result = invoke_raw!(
            hash,
                asset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_action_mode_asset_raw(
        asset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x290E2780BB7AA598u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x290E2780BB7AA598u64;

        invoke_raw_typed!(
            hash,
                asset
        )
    }
}

/// ## Parameters
*



pub fn is_ped_prone_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6A86331A537A7B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6A86331A537A7B9u64;
        
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
pub fn is_ped_prone_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6A86331A537A7B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6A86331A537A7B9u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_in_any_heli_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x298B91AE825E5705u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x298B91AE825E5705u64;
        
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
pub fn is_ped_in_any_heli_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x298B91AE825E5705u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x298B91AE825E5705u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _0xd33daa36272177c4_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD33DAA36272177C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD33DAA36272177C4u64;
        
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
pub fn _0xd33daa36272177c4_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD33DAA36272177C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD33DAA36272177C4u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
p6 always 2 (but it doesnt seem to matter...)  
roll and pitch 0  
yaw to Ped.rotation  
```



pub fn create_synchronized_scene_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            roll: 
        , 
        
        
            pitch: 
        , 
        
        
            yaw: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C18E0F9080ADD73u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C18E0F9080ADD73u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                roll, 
                pitch, 
                yaw, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_synchronized_scene_raw(
        x: , 
        y: , 
        z: , 
        roll: , 
        pitch: , 
        yaw: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C18E0F9080ADD73u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C18E0F9080ADD73u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                roll, 
                pitch, 
                yaw, 
                p6
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_be_targetted_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63F58F7C80513AADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63F58F7C80513AADu64;
        
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
pub fn set_ped_can_be_targetted_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63F58F7C80513AADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63F58F7C80513AADu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn detach_synchronized_scene_safe(
        
        
            sceneID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D38F1F04CBB37EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D38F1F04CBB37EAu64;
        
        let result = invoke_raw!(
            hash,
                sceneID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn detach_synchronized_scene_raw(
        sceneID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D38F1F04CBB37EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D38F1F04CBB37EAu64;

        invoke_raw_typed!(
            hash,
                sceneID
        )
    }
}

/// ## Parameters
*



pub fn set_ped_blend_from_parents_safe(
        
        
            ped: 
        , 
        
        
            father: 
        , 
        
        
            mother: 
        , 
        
        
            fathersSide: 
        , 
        
        
            mothersSide: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x137BBD05230DB22Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x137BBD05230DB22Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                father, 
                mother, 
                fathersSide, 
                mothersSide
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_blend_from_parents_raw(
        ped: , 
        father: , 
        mother: , 
        fathersSide: , 
        mothersSide: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x137BBD05230DB22Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x137BBD05230DB22Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                father, 
                mother, 
                fathersSide, 
                mothersSide
        )
    }
}

/// _0x5A7F62FDA59759BD native function



pub fn _0x5a7f62fda59759bd_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A7F62FDA59759BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A7F62FDA59759BDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5a7f62fda59759bd_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5A7F62FDA59759BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5A7F62FDA59759BDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn is_ped_in_parachute_free_fall_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DCE8BDA0F1C1200u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DCE8BDA0F1C1200u64;
        
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
pub fn is_ped_in_parachute_free_fall_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DCE8BDA0F1C1200u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DCE8BDA0F1C1200u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
p1 is usually 0 in the scripts. action is either 0 or a pointer to "DEFAULT_ACTION".  
```



pub fn set_ped_stealth_movement_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            action: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88CBB5CEB96B7BD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88CBB5CEB96B7BD2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                action
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_stealth_movement_raw(
        ped: , 
        p1: , 
        action: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88CBB5CEB96B7BD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88CBB5CEB96B7BD2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                action
        )
    }
}

/// ```
Creates a new ped group.  
Groups can contain up to 8 peds.  
The parameter is unused.  
Returns a handle to the created group, or 0 if a group couldn't be created.  
```



pub fn create_group_safe(
        
        
            unused: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90370EBE0FEE1A3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90370EBE0FEE1A3Du64;
        
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
pub fn create_group_raw(
        unused: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90370EBE0FEE1A3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90370EBE0FEE1A3Du64;

        invoke_raw_typed!(
            hash,
                unused
        )
    }
}

/// ## Parameters
*



pub fn get_ped_last_damage_bone_safe(
        
        
            ped: 
        , 
        
        
            outBone: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD75960F6BD9EA49Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD75960F6BD9EA49Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                outBone
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_last_damage_bone_raw(
        ped: , 
        outBone: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD75960F6BD9EA49Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD75960F6BD9EA49Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                outBone
        )
    }
}

/// ## Parameters
*



pub fn set_ped_hearing_range_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33A8F7F7D5F7F33Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33A8F7F7D5F7F33Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_hearing_range_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33A8F7F7D5F7F33Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33A8F7F7D5F7F33Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// ```
Returns the hash of the weapon/model/object that killed the ped.  
```



pub fn get_ped_cause_of_death_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16FFE42AB2D2DC59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16FFE42AB2D2DC59u64;
        
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
pub fn get_ped_cause_of_death_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16FFE42AB2D2DC59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16FFE42AB2D2DC59u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
from extreme3.c4
PED::_39D55A620FCB6A3A(PLAYER::PLAYER_PED_ID(), 8, PED::GET_PED_DRAWABLE_VARIATION(PLAYER::PLAYER_PED_ID(), 8), PED::GET_PED_TEXTURE_VARIATION(PLAYER::PLAYER_PED_ID(), 8));
p1 is probably componentId
```



pub fn set_ped_preload_variation_data_safe(
        
        
            ped: 
        , 
        
        
            slot: 
        , 
        
        
            drawableId: 
        , 
        
        
            textureId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39D55A620FCB6A3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39D55A620FCB6A3Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                slot, 
                drawableId, 
                textureId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_preload_variation_data_raw(
        ped: , 
        slot: , 
        drawableId: , 
        textureId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39D55A620FCB6A3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39D55A620FCB6A3Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                slot, 
                drawableId, 
                textureId
        )
    }
}

/// ## Parameters
*



pub fn set_ped_ragdoll_force_fall_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01F6594B923B9251u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01F6594B923B9251u64;
        
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
pub fn set_ped_ragdoll_force_fall_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01F6594B923B9251u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01F6594B923B9251u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Native to check whether [`_SET_PED_SCUBA_GEAR_VARIATION`](#_0x36C6984C3ED0C911) is enabled/actived.



pub fn _0xfec9a3b1820f3331_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEC9A3B1820F3331u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEC9A3B1820F3331u64;
        
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
pub fn _0xfec9a3b1820f3331_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEC9A3B1820F3331u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEC9A3B1820F3331u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn clear_ped_alternate_movement_anim_safe(
        
        
            ped: 
        , 
        
        
            stance: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8D19675ED5FBDCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8D19675ED5FBDCEu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                stance, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_ped_alternate_movement_anim_raw(
        ped: , 
        stance: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8D19675ED5FBDCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8D19675ED5FBDCEu64;

        invoke_raw_typed!(
            hash,
                ped, 
                stance, 
                p2
        )
    }
}

/// This native is used to set component variation on a ped. Components, drawables and textures IDs are related to the ped model.



pub fn set_ped_component_variation_safe(
        
        
            ped: 
        , 
        
        
            componentId: 
        , 
        
        
            drawableId: 
        , 
        
        
            textureId: 
        , 
        
        
            paletteId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x262B14F48D29DE80u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x262B14F48D29DE80u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                componentId, 
                drawableId, 
                textureId, 
                paletteId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_component_variation_raw(
        ped: , 
        componentId: , 
        drawableId: , 
        textureId: , 
        paletteId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x262B14F48D29DE80u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x262B14F48D29DE80u64;

        invoke_raw_typed!(
            hash,
                ped, 
                componentId, 
                drawableId, 
                textureId, 
                paletteId
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_play_ambient_anims_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6373D1349925A70Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6373D1349925A70Eu64;
        
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
pub fn set_ped_can_play_ambient_anims_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6373D1349925A70Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6373D1349925A70Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn take_ownership_of_synchronized_scene_safe(
        
        
            scene: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD9CC7E200A52A6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD9CC7E200A52A6Fu64;
        
        let result = invoke_raw!(
            hash,
                scene
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn take_ownership_of_synchronized_scene_raw(
        scene: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD9CC7E200A52A6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD9CC7E200A52A6Fu64;

        invoke_raw_typed!(
            hash,
                scene
        )
    }
}

/// ## Parameters
*



pub fn clear_facial_idle_anim_override_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x726256CC1EEB182Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x726256CC1EEB182Fu64;
        
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
pub fn clear_facial_idle_anim_override_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x726256CC1EEB182Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x726256CC1EEB182Fu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ambient_peds_drop_money_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B0E6172C9A4D902u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B0E6172C9A4D902u64;
        
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
pub fn set_ambient_peds_drop_money_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B0E6172C9A4D902u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B0E6172C9A4D902u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Gets the relationship between two groups. This should be called twice (once for each group).  
Relationship types:  
0 = Companion  
1 = Respect  
2 = Like  
3 = Neutral  
4 = Dislike  
5 = Hate  
255 = Pedestrians  
Example:  
PED::GET_RELATIONSHIP_BETWEEN_GROUPS(l_1017, 0xA49E591C);  
PED::GET_RELATIONSHIP_BETWEEN_GROUPS(0xA49E591C, l_1017);  
```



pub fn get_relationship_between_groups_safe(
        
        
            group1: 
        , 
        
        
            group2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E6B70061662AE5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E6B70061662AE5Cu64;
        
        let result = invoke_raw!(
            hash,
                group1, 
                group2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_relationship_between_groups_raw(
        group1: , 
        group2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E6B70061662AE5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E6B70061662AE5Cu64;

        invoke_raw_typed!(
            hash,
                group1, 
                group2
        )
    }
}

/// p4/p5: Unusued in TU27



pub fn set_ped_to_ragdoll_safe(
        
        
            ped: 
        , 
        
        
            minTime: 
        , 
        
        
            maxTime: 
        , 
        
        
            ragdollType: 
        , 
        
        
            bAbortIfInjured: 
        , 
        
        
            bAbortIfDead: 
        , 
        
        
            bForceScriptControl: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE99FB955581844Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE99FB955581844Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                minTime, 
                maxTime, 
                ragdollType, 
                bAbortIfInjured, 
                bAbortIfDead, 
                bForceScriptControl
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_to_ragdoll_raw(
        ped: , 
        minTime: , 
        maxTime: , 
        ragdollType: , 
        bAbortIfInjured: , 
        bAbortIfDead: , 
        bForceScriptControl: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE99FB955581844Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE99FB955581844Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                minTime, 
                maxTime, 
                ragdollType, 
                bAbortIfInjured, 
                bAbortIfDead, 
                bForceScriptControl
        )
    }
}

/// ```
gtaforums.com/topic/885580-ped-headshotmugshot-txd/  
```



pub fn unregister_pedheadshot_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96B1361D9B24C2FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96B1361D9B24C2FFu64;
        
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
pub fn unregister_pedheadshot_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96B1361D9B24C2FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96B1361D9B24C2FFu64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ## Parameters
*



pub fn is_ped_performing_melee_action_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCCA191DF9980FD7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCCA191DF9980FD7u64;
        
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
pub fn is_ped_performing_melee_action_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCCA191DF9980FD7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCCA191DF9980FD7u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn remove_relationship_group_safe(
        
        
            groupHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6BA2444AB393DA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6BA2444AB393DA2u64;
        
        let result = invoke_raw!(
            hash,
                groupHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_relationship_group_raw(
        groupHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6BA2444AB393DA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6BA2444AB393DA2u64;

        invoke_raw_typed!(
            hash,
                groupHash
        )
    }
}

/// ```
Sets the armor of the specified ped.  
ped: The Ped to set the armor of.  
amount: A value between 0 and 100 indicating the value to set the Ped's armor to.  
```



pub fn set_ped_armour_safe(
        
        
            ped: 
        , 
        
        
            amount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEA04D83135264CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEA04D83135264CCu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                amount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_armour_raw(
        ped: , 
        amount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEA04D83135264CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEA04D83135264CCu64;

        invoke_raw_typed!(
            hash,
                ped, 
                amount
        )
    }
}

/// Sets an area where scenarios are blocked



pub fn add_scenario_blocking_area_safe(
        
        
            posMinX: 
        , 
        
        
            posMinY: 
        , 
        
        
            posMinZ: 
        , 
        
        
            posMaxX: 
        , 
        
        
            posMaxY: 
        , 
        
        
            posMaxZ: 
        , 
        
        
            network: 
        , 
        
        
            cancelActive: 
        , 
        
        
            blockPeds: 
        , 
        
        
            blockVehicles: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B5C85C612E5256Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B5C85C612E5256Eu64;
        
        let result = invoke_raw!(
            hash,
                posMinX, 
                posMinY, 
                posMinZ, 
                posMaxX, 
                posMaxY, 
                posMaxZ, 
                network, 
                cancelActive, 
                blockPeds, 
                blockVehicles
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_scenario_blocking_area_raw(
        posMinX: , 
        posMinY: , 
        posMinZ: , 
        posMaxX: , 
        posMaxY: , 
        posMaxZ: , 
        network: , 
        cancelActive: , 
        blockPeds: , 
        blockVehicles: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B5C85C612E5256Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B5C85C612E5256Eu64;

        invoke_raw_typed!(
            hash,
                posMinX, 
                posMinY, 
                posMinZ, 
                posMaxX, 
                posMaxY, 
                posMaxZ, 
                network, 
                cancelActive, 
                blockPeds, 
                blockVehicles
        )
    }
}

/// ## Parameters
*



pub fn is_any_ped_near_point_safe(
        
        
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
        let hash = 0x083961498679DC9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x083961498679DC9Fu64;
        
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
pub fn is_any_ped_near_point_raw(
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x083961498679DC9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x083961498679DC9Fu64;

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
GET_*
```



pub fn _0xf033419d1b81fae8_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF033419D1B81FAE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF033419D1B81FAE8u64;
        
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
pub fn _0xf033419d1b81fae8_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF033419D1B81FAE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF033419D1B81FAE8u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Gets the offset the specified ped has moved since the previous tick.  
If worldSpace is false, the returned offset is relative to the ped. That is, if the ped has moved 1 meter right and 5 meters forward, it'll return 1,5,0.  
If worldSpace is true, the returned offset is relative to the world. That is, if the ped has moved 1 meter on the X axis and 5 meters on the Y axis, it'll return 1,5,0.  
```



pub fn get_ped_extracted_displacement_safe(
        
        
            ped: 
        , 
        
        
            worldSpace: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0AF41401ADF87E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0AF41401ADF87E3u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                worldSpace
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_extracted_displacement_raw(
        ped: , 
        worldSpace: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0AF41401ADF87E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0AF41401ADF87E3u64;

        invoke_raw_typed!(
            hash,
                ped, 
                worldSpace
        )
    }
}

/// ## Parameters
*



pub fn set_ped_angled_defensive_area_safe(
        
        
            ped: 
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7F76DF27A5045A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7F76DF27A5045A1u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                p2, 
                p3, 
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
pub fn set_ped_angled_defensive_area_raw(
        ped: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: , 
        p9: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7F76DF27A5045A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7F76DF27A5045A1u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2, 
                p3, 
                p4, 
                p5, 
                p6, 
                p7, 
                p8, 
                p9
        )
    }
}

/// ## Parameters
*



pub fn is_ped_swimming_under_water_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC024869A53992F34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC024869A53992F34u64;
        
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
pub fn is_ped_swimming_under_water_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC024869A53992F34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC024869A53992F34u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn request_stealth_mode_asset_safe(
        
        
            asset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A0A62FCDEE16D4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A0A62FCDEE16D4Fu64;
        
        let result = invoke_raw!(
            hash,
                asset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_stealth_mode_asset_raw(
        asset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A0A62FCDEE16D4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A0A62FCDEE16D4Fu64;

        invoke_raw_typed!(
            hash,
                asset
        )
    }
}

/// ```
Need to check behavior when drawableId = -1
```



pub fn get_number_of_ped_prop_texture_variations_safe(
        
        
            ped: 
        , 
        
        
            propId: 
        , 
        
        
            drawableId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6E7F1CEB523E171u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6E7F1CEB523E171u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                propId, 
                drawableId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_number_of_ped_prop_texture_variations_raw(
        ped: , 
        propId: , 
        drawableId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6E7F1CEB523E171u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6E7F1CEB523E171u64;

        invoke_raw_typed!(
            hash,
                ped, 
                propId, 
                drawableId
        )
    }
}

/// Use [`SetPedIlluminatedClothingGlowIntensity`](#_0x4E90D746056E273D) to set the illuminated clothing glow intensity for a specific ped.



pub fn _get_ped_emissive_intensity_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1461B28A06717D68u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1461B28A06717D68u64;
        
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
pub fn _get_ped_emissive_intensity_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1461B28A06717D68u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1461B28A06717D68u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn was_ped_killed_by_takedown_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F08E26039C7347Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F08E26039C7347Cu64;
        
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
pub fn was_ped_killed_by_takedown_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F08E26039C7347Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F08E26039C7347Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Notes: The function only returns true while the ped is:   
A.) Swinging a random melee attack (including pistol-whipping)  
B.) Reacting to being hit by a melee attack (including pistol-whipping)  
C.) Is locked-on to an enemy (arms up, strafing/skipping in the default fighting-stance, ready to dodge+counter).   
You don't have to be holding the melee-targetting button to be in this stance; you stay in it by default for a few seconds after swinging at someone. If you do a sprinting punch, it returns true for the duration of the punch animation and then returns false again, even if you've punched and made-angry many peds  
```



pub fn is_ped_in_melee_combat_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E209B2C1EAD5159u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E209B2C1EAD5159u64;
        
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
pub fn is_ped_in_melee_combat_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E209B2C1EAD5159u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E209B2C1EAD5159u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn force_ped_to_open_parachute_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16E42E800B472221u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16E42E800B472221u64;
        
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
pub fn force_ped_to_open_parachute_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16E42E800B472221u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16E42E800B472221u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_drive_by_clipset_override_safe(
        
        
            ped: 
        , 
        
        
            clipset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED34AB6C5CB36520u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED34AB6C5CB36520u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                clipset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_drive_by_clipset_override_raw(
        ped: , 
        clipset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED34AB6C5CB36520u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED34AB6C5CB36520u64;

        invoke_raw_typed!(
            hash,
                ped, 
                clipset
        )
    }
}

/// Set the number of scenario peds on the entire map



pub fn set_scenario_ped_density_multiplier_this_frame_safe(
        
        
            interiorMult: 
        , 
        
        
            exteriorMult: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A556143A1C03898u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A556143A1C03898u64;
        
        let result = invoke_raw!(
            hash,
                interiorMult, 
                exteriorMult
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_scenario_ped_density_multiplier_this_frame_raw(
        interiorMult: , 
        exteriorMult: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A556143A1C03898u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A556143A1C03898u64;

        invoke_raw_typed!(
            hash,
                interiorMult, 
                exteriorMult
        )
    }
}

/// ```
p1: from 0 to 5 in the b617d scripts.  
p2: "blushing" and "ALL" found in the b617d scripts.  
```



pub fn clear_ped_damage_decal_by_zone_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x523C79AEEFCC4A2Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x523C79AEEFCC4A2Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_ped_damage_decal_by_zone_raw(
        ped: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x523C79AEEFCC4A2Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x523C79AEEFCC4A2Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2
        )
    }
}

/// ```
GET_TIME_*
```



pub fn _get_time_of_last_ped_weapon_damage_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36B77BB84687C318u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36B77BB84687C318u64;
        
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
pub fn _get_time_of_last_ped_weapon_damage_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36B77BB84687C318u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36B77BB84687C318u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _0xb8b52e498014f5b0_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8B52E498014F5B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8B52E498014F5B0u64;
        
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
pub fn _0xb8b52e498014f5b0_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8B52E498014F5B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8B52E498014F5B0u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Applies an Item from a PedDecorationCollection to a ped. These include tattoos and shirt decals.
collection - PedDecorationCollection filename hash
overlay - Item name hash
Example:
Entry inside "mpbeach_overlays.xml" -
<Item>
  <uvPos x="0.500000" y="0.500000" />
  <scale x="0.600000" y="0.500000" />
  <rotation value="0.000000" />
  <nameHash>FM_Hair_Fuzz</nameHash>
  <txdHash>mp_hair_fuzz</txdHash>
  <txtHash>mp_hair_fuzz</txtHash>
  <zone>ZONE_HEAD</zone>
  <type>TYPE_TATTOO</type>
  <faction>FM</faction>
  <garment>All</garment>
  <gender>GENDER_DONTCARE</gender>
  <award />
  <awardLevel />
</Item>
Code:
PED::_0x5F5D1665E352A839(PLAYER::PLAYER_PED_ID(), MISC::GET_HASH_KEY("mpbeach_overlays"), MISC::GET_HASH_KEY("fm_hair_fuzz"))
```



pub fn add_ped_decoration_from_hashes_safe(
        
        
            ped: 
        , 
        
        
            collection: 
        , 
        
        
            overlay: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F5D1665E352A839u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F5D1665E352A839u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                collection, 
                overlay
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_ped_decoration_from_hashes_raw(
        ped: , 
        collection: , 
        overlay: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F5D1665E352A839u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F5D1665E352A839u64;

        invoke_raw_typed!(
            hash,
                ped, 
                collection, 
                overlay
        )
    }
}

/// ```
Used for freemode (online) characters.  
```



pub fn _get_num_hair_colors_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE5C0CF872C2AD150u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE5C0CF872C2AD150u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_num_hair_colors_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE5C0CF872C2AD150u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE5C0CF872C2AD150u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn reset_ped_movement_clipset_safe(
        
        
            ped: 
        , 
        
        
            transitionSpeed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA74EC0CB0AAEA2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA74EC0CB0AAEA2Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                transitionSpeed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_ped_movement_clipset_raw(
        ped: , 
        transitionSpeed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA74EC0CB0AAEA2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA74EC0CB0AAEA2Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                transitionSpeed
        )
    }
}

/// ## Parameters
*



pub fn _0x03ea03af85a85cb7_safe(
        
        
            ped: 
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
        let hash = 0x03EA03AF85A85CB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03EA03AF85A85CB7u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn _0x03ea03af85a85cb7_raw(
        ped: , 
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
        let hash = 0x03EA03AF85A85CB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03EA03AF85A85CB7u64;

        invoke_raw_typed!(
            hash,
                ped, 
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



pub fn can_create_random_ped_safe(
        
        
            unk: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E8349C08E4B82E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E8349C08E4B82E4u64;
        
        let result = invoke_raw!(
            hash,
                unk
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_create_random_ped_raw(
        unk: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E8349C08E4B82E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E8349C08E4B82E4u64;

        invoke_raw_typed!(
            hash,
                unk
        )
    }
}

/// ## Parameters
*



pub fn set_ped_visual_field_min_angle_safe(
        
        
            ped: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DB492222FB21E26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DB492222FB21E26u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_visual_field_min_angle_raw(
        ped: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DB492222FB21E26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DB492222FB21E26u64;

        invoke_raw_typed!(
            hash,
                ped, 
                value
        )
    }
}

/// Define the scope within which the ped will engage in combat with the target.

```c
enum eCombatRange {
    CR_NEAR = 0, // keeps within 5-15m
    CR_MEDIUM = 1, // keeps within 7-30m
    CR_FAR = 2, // keeps within 15-40m
    CR_VERY_FAR = 3 // keeps within 22-45m
};
```



pub fn set_ped_combat_range_safe(
        
        
            ped: 
        , 
        
        
            range: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C606747B23E497Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C606747B23E497Bu64;
        
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
pub fn set_ped_combat_range_raw(
        ped: , 
        range: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C606747B23E497Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C606747B23E497Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                range
        )
    }
}

/// ## Parameters
*



pub fn get_ped_relationship_group_hash_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DBDD04862D95F04u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DBDD04862D95F04u64;
        
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
pub fn get_ped_relationship_group_hash_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DBDD04862D95F04u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DBDD04862D95F04u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Initial guess of native: `_IS_PED_WEARING_MOTORCYCLE_HELMET`.



pub fn _0xf2385935bffd4d92_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2385935BFFD4D92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2385935BFFD4D92u64;
        
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
pub fn _0xf2385935bffd4d92_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2385935BFFD4D92u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2385935BFFD4D92u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn knock_ped_off_vehicle_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45BBCBA77C29A841u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45BBCBA77C29A841u64;
        
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
pub fn knock_ped_off_vehicle_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45BBCBA77C29A841u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45BBCBA77C29A841u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_vaulting_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x117C70D1F5730B5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x117C70D1F5730B5Eu64;
        
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
pub fn is_ped_vaulting_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x117C70D1F5730B5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x117C70D1F5730B5Eu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn apply_ped_blood_by_zone_safe(
        
        
            ped: 
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
        let hash = 0x3311E47B91EDCBBCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3311E47B91EDCBBCu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
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
pub fn apply_ped_blood_by_zone_raw(
        ped: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3311E47B91EDCBBCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3311E47B91EDCBBCu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// It makes the ped lose (or not lose) their props (like glasses or helmets/hat) when someone punches or pushes the ped.
This is probably what's being used in GTA:O to keep players from knocking other player's hats/glasses off when in combat.



pub fn set_ped_can_lose_props_on_damage_safe(
        
        
            ped: 
        , 
        
        
            loseProps: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE861D0B05C7662B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE861D0B05C7662B8u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                loseProps, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_can_lose_props_on_damage_raw(
        ped: , 
        loseProps: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE861D0B05C7662B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE861D0B05C7662B8u64;

        invoke_raw_typed!(
            hash,
                ped, 
                loseProps, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_ped_is_entering_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF92691AED837A5FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF92691AED837A5FCu64;
        
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
pub fn get_vehicle_ped_is_entering_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF92691AED837A5FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF92691AED837A5FCu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_ped_relationship_group_default_hash_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42FDD0F017B1E38Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42FDD0F017B1E38Eu64;
        
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
pub fn get_ped_relationship_group_default_hash_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42FDD0F017B1E38Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42FDD0F017B1E38Eu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Return value



pub fn _get_num_makeup_colors_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1F7CA1535D22818u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1F7CA1535D22818u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_num_makeup_colors_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD1F7CA1535D22818u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD1F7CA1535D22818u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_ped_group_member_passenger_index_safe(
        
        
            ped: 
        , 
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BDDB8D9EC6BCF3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BDDB8D9EC6BCF3Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_group_member_passenger_index_raw(
        ped: , 
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BDDB8D9EC6BCF3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BDDB8D9EC6BCF3Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                index
        )
    }
}

/// ## Parameters
*



pub fn set_ped_highly_perceptive_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52D59AB61DDC05DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52D59AB61DDC05DDu64;
        
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
pub fn set_ped_highly_perceptive_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52D59AB61DDC05DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52D59AB61DDC05DDu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// See [`SET_PED_CONFIG_FLAG`](#_0x1913FE4CBF41C463).



pub fn get_ped_config_flag_safe(
        
        
            ped: 
        , 
        
        
            flagId: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EE53118C892B513u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EE53118C892B513u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                flagId, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_config_flag_raw(
        ped: , 
        flagId: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EE53118C892B513u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EE53118C892B513u64;

        invoke_raw_typed!(
            hash,
                ped, 
                flagId, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn set_synchronized_scene_hold_last_frame_safe(
        
        
            sceneID: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x394B9CD12435C981u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x394B9CD12435C981u64;
        
        let result = invoke_raw!(
            hash,
                sceneID, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_synchronized_scene_hold_last_frame_raw(
        sceneID: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x394B9CD12435C981u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x394B9CD12435C981u64;

        invoke_raw_typed!(
            hash,
                sceneID, 
                toggle
        )
    }
}

/// ## Return value



pub fn spawnpoints_is_search_complete_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA586FBEB32A53DBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA586FBEB32A53DBBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn spawnpoints_is_search_complete_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA586FBEB32A53DBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA586FBEB32A53DBBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Sets the relationship between two groups. This should be called twice (once for each group).  
Relationship types:  
0 = Companion  
1 = Respect  
2 = Like  
3 = Neutral  
4 = Dislike  
5 = Hate  
255 = Pedestrians  
Example:  
PED::SET_RELATIONSHIP_BETWEEN_GROUPS(2, l_1017, 0xA49E591C);  
PED::SET_RELATIONSHIP_BETWEEN_GROUPS(2, 0xA49E591C, l_1017);  
```



pub fn set_relationship_between_groups_safe(
        
        
            relationship: 
        , 
        
        
            group1: 
        , 
        
        
            group2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF25EB89375A37ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF25EB89375A37ADu64;
        
        let result = invoke_raw!(
            hash,
                relationship, 
                group1, 
                group2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_relationship_between_groups_raw(
        relationship: , 
        group1: , 
        group2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF25EB89375A37ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF25EB89375A37ADu64;

        invoke_raw_typed!(
            hash,
                relationship, 
                group1, 
                group2
        )
    }
}

/// ## Parameters
*



pub fn attach_synchronized_scene_to_entity_safe(
        
        
            sceneID: 
        , 
        
        
            entity: 
        , 
        
        
            boneIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x272E4723B56A3B96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x272E4723B56A3B96u64;
        
        let result = invoke_raw!(
            hash,
                sceneID, 
                entity, 
                boneIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn attach_synchronized_scene_to_entity_raw(
        sceneID: , 
        entity: , 
        boneIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x272E4723B56A3B96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x272E4723B56A3B96u64;

        invoke_raw_typed!(
            hash,
                sceneID, 
                entity, 
                boneIndex
        )
    }
}

/// ```
Returns true if a synchronized scene is running  
```



pub fn is_synchronized_scene_running_safe(
        
        
            sceneId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25D39B935A038A26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25D39B935A038A26u64;
        
        let result = invoke_raw!(
            hash,
                sceneId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_synchronized_scene_running_raw(
        sceneId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25D39B935A038A26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25D39B935A038A26u64;

        invoke_raw_typed!(
            hash,
                sceneId
        )
    }
}

/// A getter for [`_SET_PED_EYE_COLOR`](#_0x50B56988B170AFDF).



pub fn _get_ped_eye_color_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76BBA2CEE66D47E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76BBA2CEE66D47E9u64;
        
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
pub fn _get_ped_eye_color_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76BBA2CEE66D47E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76BBA2CEE66D47E9u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_using_any_scenario_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57AB4A3080F85143u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57AB4A3080F85143u64;
        
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
pub fn is_ped_using_any_scenario_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57AB4A3080F85143u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57AB4A3080F85143u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn has_ped_received_event_safe(
        
        
            ped: 
        , 
        
        
            eventId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8507BCB710FA6DC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8507BCB710FA6DC0u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                eventId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_ped_received_event_raw(
        ped: , 
        eventId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8507BCB710FA6DC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8507BCB710FA6DC0u64;

        invoke_raw_typed!(
            hash,
                ped, 
                eventId
        )
    }
}

/// ## Parameters
*



pub fn finalize_head_blend_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4668D80430D6C299u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4668D80430D6C299u64;
        
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
pub fn finalize_head_blend_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4668D80430D6C299u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4668D80430D6C299u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_ped_can_ragdoll_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB128377056A54E2Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB128377056A54E2Au64;
        
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
pub fn set_ped_can_ragdoll_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB128377056A54E2Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB128377056A54E2Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn clear_ped_last_damage_bone_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EF6B7AC68E2F01Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EF6B7AC68E2F01Bu64;
        
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
pub fn clear_ped_last_damage_bone_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EF6B7AC68E2F01Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EF6B7AC68E2F01Bu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_ped_ragdoll_bone_index_safe(
        
        
            ped: 
        , 
        
        
            bone: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2057EF813397A772u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2057EF813397A772u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                bone
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_ragdoll_bone_index_raw(
        ped: , 
        bone: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2057EF813397A772u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2057EF813397A772u64;

        invoke_raw_typed!(
            hash,
                ped, 
                bone
        )
    }
}

/// Overwrites the minimum time the ped will stay on the ground for after being stunned. Setting this while the ped is stunned will not alter the duration of the current stun but will still effect future stuns.

Passing -1 into the second parameter `minTimeInMs` will reset the modifier, making it use the weapons original `DamageTime` as the stun duration (see `update/update.rpf/common/data/ai/weapons.meta`)



pub fn set_ped_min_ground_time_for_stungun_safe(
        
        
            ped: 
        , 
        
        
            minTimeInMs: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA0675AB151073FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA0675AB151073FAu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                minTimeInMs
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_min_ground_time_for_stungun_raw(
        ped: , 
        minTimeInMs: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA0675AB151073FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA0675AB151073FAu64;

        invoke_raw_typed!(
            hash,
                ped, 
                minTimeInMs
        )
    }
}

/// ```
Returns true/false if the ped is/isn't humanoid.  
```



pub fn is_ped_human_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB980061DA992779Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB980061DA992779Du64;
        
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
pub fn is_ped_human_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB980061DA992779Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB980061DA992779Du64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Type equals 0 for male non-dlc, 1 for female non-dlc, 2 for male dlc, and 3 for female dlc.  
```



pub fn get_ped_head_blend_num_heads_safe(
        
        
            type: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EF37013A6539C9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EF37013A6539C9Du64;
        
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
pub fn get_ped_head_blend_num_heads_raw(
        type: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EF37013A6539C9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EF37013A6539C9Du64;

        invoke_raw_typed!(
            hash,
                type
        )
    }
}

/// ## Parameters
*



pub fn is_ped_in_flying_vehicle_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9134873537FA419Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9134873537FA419Cu64;
        
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
pub fn is_ped_in_flying_vehicle_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9134873537FA419Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9134873537FA419Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_headtracking_ped_safe(
        
        
            ped1: 
        , 
        
        
            ped2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CD3CB88A7F8850Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CD3CB88A7F8850Du64;
        
        let result = invoke_raw!(
            hash,
                ped1, 
                ped2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_headtracking_ped_raw(
        ped1: , 
        ped2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CD3CB88A7F8850Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CD3CB88A7F8850Du64;

        invoke_raw_typed!(
            hash,
                ped1, 
                ped2
        )
    }
}

/// ## Parameters
*



pub fn _0xaaa6a3698a69e048_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAA6A3698A69E048u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAA6A3698A69E048u64;
        
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
pub fn _0xaaa6a3698a69e048_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAAA6A3698A69E048u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAAA6A3698A69E048u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Similar to REGISTER_PEDHEADSHOT but creates a transparent background instead of black.



pub fn register_pedheadshot_transparent_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x953563CE563143AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x953563CE563143AFu64;
        
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
pub fn register_pedheadshot_transparent_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x953563CE563143AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x953563CE563143AFu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

