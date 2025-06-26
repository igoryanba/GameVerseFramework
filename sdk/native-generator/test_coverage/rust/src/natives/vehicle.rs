//! VEHICLE native functions
//! 
//! Functions for the vehicle category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
On accelerating, spins the driven wheels with the others braked, so you don't go anywhere.  
```



pub fn set_vehicle_burnout_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB8794444A7D60FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB8794444A7D60FBu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_burnout_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFB8794444A7D60FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFB8794444A7D60FBu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
NativeDB Added Parameter 1: Vehicle vehicle
NativeDB Added Parameter 2: Any p1
```



pub fn _0xdce97bdf8a0eabc8_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCE97BDF8A0EABC8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCE97BDF8A0EABC8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xdce97bdf8a0eabc8_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCE97BDF8A0EABC8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCE97BDF8A0EABC8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Sets a vehicle on the ground on all wheels.  Returns whether or not the operation was successful.  
```

```
NativeDB Added Parameter 2: float p1
```



pub fn set_vehicle_on_ground_properly_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49733E92263139D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49733E92263139D1u64;
        
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
pub fn set_vehicle_on_ground_properly_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49733E92263139D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49733E92263139D1u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn is_playback_using_ai_going_on_for_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAEA8FD591FAD4106u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAEA8FD591FAD4106u64;
        
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
pub fn is_playback_using_ai_going_on_for_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAEA8FD591FAD4106u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAEA8FD591FAD4106u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```c
enum eVehicleWheelType
{
    VWT_SPORT = 0,
    VWT_MUSCLE = 1,
    VWT_LOWRIDER = 2,
    VWT_SUV = 3,
    VWT_OFFROAD = 4,
    VWT_TUNER = 5,
    VWT_BIKE = 6,
    VWT_HIEND = 7,
    // Benny's Original
    VWT_SUPERMOD1 = 8,
    // Benny's Bespoke
    VWT_SUPERMOD2 = 9,
    // Open Wheel
    VWT_SUPERMOD3 = 10,
    // Street
    VWT_SUPERMOD4 = 11,
    // Track
    VWT_SUPERMOD5 = 12,
};
```



pub fn get_vehicle_wheel_type_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3ED1BFB4BE636DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3ED1BFB4BE636DCu64;
        
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
pub fn get_vehicle_wheel_type_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3ED1BFB4BE636DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3ED1BFB4BE636DCu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_parked_vehicle_density_multiplier_this_frame_safe(
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAE6DCC7EEE3DB1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAE6DCC7EEE3DB1Du64;
        
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
pub fn set_parked_vehicle_density_multiplier_this_frame_raw(
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEAE6DCC7EEE3DB1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEAE6DCC7EEE3DB1Du64;

        invoke_raw_typed!(
            hash,
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn add_vehicle_stuck_check_with_warp_safe(
        
        
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
        let hash = 0x2FA9923062DD396Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FA9923062DD396Cu64;
        
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
pub fn add_vehicle_stuck_check_with_warp_raw(
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
        let hash = 0x2FA9923062DD396Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FA9923062DD396Cu64;

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

/// ## Parameters
*



pub fn get_current_playback_for_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42BC05C27A946054u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42BC05C27A946054u64;
        
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
pub fn get_current_playback_for_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42BC05C27A946054u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42BC05C27A946054u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
From the driver's perspective, is the right headlight broken.  
```



pub fn get_is_right_vehicle_headlight_damaged_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7ECB73355EB2F20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7ECB73355EB2F20u64;
        
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
pub fn get_is_right_vehicle_headlight_damaged_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7ECB73355EB2F20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7ECB73355EB2F20u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn get_total_duration_of_vehicle_recording_safe(
        
        
            recording: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E48D1C262390950u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E48D1C262390950u64;
        
        let result = invoke_raw!(
            hash,
                recording
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_total_duration_of_vehicle_recording_raw(
        recording: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E48D1C262390950u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E48D1C262390950u64;

        invoke_raw_typed!(
            hash,
                recording
        )
    }
}

/// ```
Returns true when in a vehicle, false whilst entering/exiting.  
```



pub fn get_is_vehicle_engine_running_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE31E7DF9B5B132Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE31E7DF9B5B132Eu64;
        
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
pub fn get_is_vehicle_engine_running_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE31E7DF9B5B132Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE31E7DF9B5B132Eu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Sets the specified door index open on the passed vehicle. See [`IS_VEHICLE_DOOR_FULLY_OPEN`](#_0x3E933CFF7B111C22).



pub fn set_vehicle_door_open_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        , 
        
        
            loose: 
        , 
        
        
            openInstantly: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C65DAC73C35C862u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C65DAC73C35C862u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex, 
                loose, 
                openInstantly
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_door_open_raw(
        vehicle: , 
        doorIndex: , 
        loose: , 
        openInstantly: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C65DAC73C35C862u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C65DAC73C35C862u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex, 
                loose, 
                openInstantly
        )
    }
}

/// ```
Gets the trailer of a vehicle and puts it into the trailer parameter.  
```



pub fn get_vehicle_trailer_vehicle_safe(
        
        
            vehicle: 
        , 
        
        
            trailer: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CDD6BADC297830Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CDD6BADC297830Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                trailer
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_trailer_vehicle_raw(
        vehicle: , 
        trailer: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CDD6BADC297830Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CDD6BADC297830Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                trailer
        )
    }
}

/// ## Parameters
*



pub fn _0xe851e480b814d4ba_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE851E480B814D4BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE851E480B814D4BAu64;
        
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
pub fn _0xe851e480b814d4ba_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE851E480B814D4BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE851E480B814D4BAu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _set_vehicle_dashboard_color_safe(
        
        
            vehicle: 
        , 
        
        
            color: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6089CDF6A57F326Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6089CDF6A57F326Cu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                color
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_dashboard_color_raw(
        vehicle: , 
        color: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6089CDF6A57F326Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6089CDF6A57F326Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                color
        )
    }
}

/// ## Parameters
*



pub fn _0xab04325045427aae_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB04325045427AAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB04325045427AAEu64;
        
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
pub fn _0xab04325045427aae_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB04325045427AAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB04325045427AAEu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_playback_speed_safe(
        
        
            vehicle: 
        , 
        
        
            speed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6683AB880E427778u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6683AB880E427778u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                speed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_playback_speed_raw(
        vehicle: , 
        speed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6683AB880E427778u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6683AB880E427778u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                speed
        )
    }
}

/// ## Parameters
*



pub fn attach_vehicle_to_cargobob_safe(
        
        
            cargobob: 
        , 
        
        
            vehicle: 
        , 
        
        
            vehicleBoneIndex: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4127F1D84E347769u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4127F1D84E347769u64;
        
        let result = invoke_raw!(
            hash,
                cargobob, 
                vehicle, 
                vehicleBoneIndex, 
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
pub fn attach_vehicle_to_cargobob_raw(
        cargobob: , 
        vehicle: , 
        vehicleBoneIndex: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4127F1D84E347769u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4127F1D84E347769u64;

        invoke_raw_typed!(
            hash,
                cargobob, 
                vehicle, 
                vehicleBoneIndex, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_layout_hash_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28D37D4F71AC5C58u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28D37D4F71AC5C58u64;
        
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
pub fn get_vehicle_layout_hash_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28D37D4F71AC5C58u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28D37D4F71AC5C58u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_doors_locked_for_player_safe(
        
        
            vehicle: 
        , 
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6AF6CB341349015u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6AF6CB341349015u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                player
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_doors_locked_for_player_raw(
        vehicle: , 
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6AF6CB341349015u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6AF6CB341349015u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                player
        )
    }
}

/// ```
Can be used with GET_TOTAL_DURATION_OF_VEHICLE_RECORDING{_ID} to compute a percentage.
```



pub fn get_time_position_in_recording_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5746F3A7AB7FE544u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5746F3A7AB7FE544u64;
        
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
pub fn get_time_position_in_recording_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5746F3A7AB7FE544u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5746F3A7AB7FE544u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Sets the wanted state of this vehicle.  
```



pub fn set_vehicle_is_wanted_safe(
        
        
            vehicle: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7EC25A3EBEEC726u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7EC25A3EBEEC726u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_is_wanted_raw(
        vehicle: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7EC25A3EBEEC726u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7EC25A3EBEEC726u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                state
        )
    }
}

/// ## Parameters
*



pub fn get_rotation_of_vehicle_recording_id_at_time_safe(
        
        
            id: 
        , 
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0F2103EFAF8CBA7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0F2103EFAF8CBA7u64;
        
        let result = invoke_raw!(
            hash,
                id, 
                time
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_rotation_of_vehicle_recording_id_at_time_raw(
        id: , 
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0F2103EFAF8CBA7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0F2103EFAF8CBA7u64;

        invoke_raw_typed!(
            hash,
                id, 
                time
        )
    }
}

/// Does not work for vehicle of type: CBike, CBmx, CBoat, CTrain, CSubmarine.



pub fn _is_vehicle_parachute_active_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DE51E9C80B116CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DE51E9C80B116CFu64;
        
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
pub fn _is_vehicle_parachute_active_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DE51E9C80B116CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DE51E9C80B116CFu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Returns the text label of a mod type for a given vehicle  
Use _GET_LABEL_TEXT to get the part name in the game's language  
```



pub fn get_mod_text_label_safe(
        
        
            vehicle: 
        , 
        
        
            modType: 
        , 
        
        
            modValue: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8935624F8C5592CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8935624F8C5592CCu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                modType, 
                modValue
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_mod_text_label_raw(
        vehicle: , 
        modType: , 
        modValue: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8935624F8C5592CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8935624F8C5592CCu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                modType, 
                modValue
        )
    }
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)



pub fn is_vehicle_door_damaged_safe(
        
        
            veh: 
        , 
        
        
            doorID: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8E181E559464527u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8E181E559464527u64;
        
        let result = invoke_raw!(
            hash,
                veh, 
                doorID
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_door_damaged_raw(
        veh: , 
        doorID: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8E181E559464527u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8E181E559464527u64;

        invoke_raw_typed!(
            hash,
                veh, 
                doorID
        )
    }
}

/// ```
Retracts the hook on the cargobob.  
Note: after you retract it the natives for dropping the hook no longer work  
```



pub fn remove_pick_up_rope_for_cargobob_safe(
        
        
            cargobob: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9768CF648F54C804u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9768CF648F54C804u64;
        
        let result = invoke_raw!(
            hash,
                cargobob
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_pick_up_rope_for_cargobob_raw(
        cargobob: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9768CF648F54C804u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9768CF648F54C804u64;

        invoke_raw_typed!(
            hash,
                cargobob
        )
    }
}

/// ```
SET_VEHICLE_D*
```



pub fn _set_vehicle_damage_modifier_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E20D2A627011E8Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E20D2A627011E8Eu64;
        
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
pub fn _set_vehicle_damage_modifier_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E20D2A627011E8Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E20D2A627011E8Eu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_mod_kit_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6325D1A044AE510Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6325D1A044AE510Du64;
        
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
pub fn get_vehicle_mod_kit_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6325D1A044AE510Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6325D1A044AE510Du64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Sets the turn signal enabled for a vehicle.  
Set turnSignal to 1 for left light, 0 for right light.  
```



pub fn set_vehicle_indicator_lights_safe(
        
        
            vehicle: 
        , 
        
        
            turnSignal: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5D45264751B7DF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5D45264751B7DF0u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                turnSignal, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_indicator_lights_raw(
        vehicle: , 
        turnSignal: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5D45264751B7DF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5D45264751B7DF0u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                turnSignal, 
                toggle
        )
    }
}

/// ```
From the driver's perspective, is the left headlight broken.  
```



pub fn get_is_left_vehicle_headlight_damaged_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EF77C9ADD3B11A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EF77C9ADD3B11A3u64;
        
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
pub fn get_is_left_vehicle_headlight_damaged_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EF77C9ADD3B11A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EF77C9ADD3B11A3u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn is_any_vehicle_near_point_safe(
        
        
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
        let hash = 0x61E1DD6125A3EEE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61E1DD6125A3EEE6u64;
        
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
pub fn is_any_vehicle_near_point_raw(
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x61E1DD6125A3EEE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x61E1DD6125A3EEE6u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// ## Parameters
*



pub fn _set_heli_tail_rotor_health_safe(
        
        
            vehicle: 
        , 
        
        
            health: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE205F38AAA58E5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE205F38AAA58E5Bu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                health
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_heli_tail_rotor_health_raw(
        vehicle: , 
        health: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE205F38AAA58E5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE205F38AAA58E5Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                health
        )
    }
}

/// ## Parameters
*



pub fn is_seat_warp_only_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7F203E31F96F6A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7F203E31F96F6A1u64;
        
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
pub fn is_seat_warp_only_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7F203E31F96F6A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7F203E31F96F6A1u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Only called once inside main_persitant with the parameters, 0  
```



pub fn set_train_track_spawn_frequency_safe(
        
        
            trackIndex: 
        , 
        
        
            frequency: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21973BBF8D17EDFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21973BBF8D17EDFAu64;
        
        let result = invoke_raw!(
            hash,
                trackIndex, 
                frequency
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_train_track_spawn_frequency_raw(
        trackIndex: , 
        frequency: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21973BBF8D17EDFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21973BBF8D17EDFAu64;

        invoke_raw_typed!(
            hash,
                trackIndex, 
                frequency
        )
    }
}

/// ```
Finds the vehicle that is carrying this entity with a handler frame.
The model of the entity must be prop_contr_03b_ld or the function will return 0.
```



pub fn _find_vehicle_carrying_this_entity_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x375E7FC44F21C8ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x375E7FC44F21C8ABu64;
        
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
pub fn _find_vehicle_carrying_this_entity_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x375E7FC44F21C8ABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x375E7FC44F21C8ABu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// Queries whether the control panels of a plane are intact. This native is used to determine the operational status of a plane's cockpit controls, which can affect the plane's flyability.



pub fn are_plane_control_panels_intact_safe(
        
        
            vehicle: 
        , 
        
        
            checkForZeroHealth: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF78F94D60248C737u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF78F94D60248C737u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                checkForZeroHealth
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn are_plane_control_panels_intact_raw(
        vehicle: , 
        checkForZeroHealth: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF78F94D60248C737u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF78F94D60248C737u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                checkForZeroHealth
        )
    }
}

/// ## Parameters
*



pub fn set_car_boot_open_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC40CBF7B90CA77Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC40CBF7B90CA77Cu64;
        
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
pub fn set_car_boot_open_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC40CBF7B90CA77Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC40CBF7B90CA77Cu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
It switch to highbeam when p1 is set to true.  
```



pub fn set_vehicle_fullbeam_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B7FD87F0DDB421Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B7FD87F0DDB421Eu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_fullbeam_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B7FD87F0DDB421Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B7FD87F0DDB421Eu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_disable_vehicle_petrol_tank_fires_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x465BF26AB9684352u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x465BF26AB9684352u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_disable_vehicle_petrol_tank_fires_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x465BF26AB9684352u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x465BF26AB9684352u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
Returns an int  
Vehicle Classes:  
0: Compacts  
1: Sedans  
2: SUVs  
3: Coupes  
4: Muscle  
5: Sports Classics  
6: Sports  
7: Super  
8: Motorcycles  
9: Off-road  
10: Industrial  
11: Utility  
12: Vans  
13: Cycles  
14: Boats  
15: Helicopters  
16: Planes  
17: Service  
18: Emergency  
19: Military  
20: Commercial  
21: Trains  
22: Open Wheel
char buffer[128];  
std::sprintf(buffer, "VEH_CLASS_%i", VEHICLE::GET_VEHICLE_CLASS(vehicle));  
char* className = UI::_GET_LABEL_TEXT(buffer);  
```



pub fn get_vehicle_class_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29439776AAA00A62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29439776AAA00A62u64;
        
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
pub fn get_vehicle_class_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29439776AAA00A62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29439776AAA00A62u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Usex in decompiled scripts in combination with [`_GET_VEHICLE_SUSPENSION_BOUNDS`](#_0xDF7E3EEB29642C38).

NativeDB Introduced: v1180

```
// Example from fm_bj_race_controller.c
if (!VEHICLE::_0x51F30DB60626A20E(uParam0->f_26, uParam0->f_12.f_3, uParam0->f_12, 2, 1) && !func_282(uParam0->f_6))
{
    VEHICLE::_GET_VEHICLE_SUSPENSION_BOUNDS(*uParam0, &vVar15, &uVar16);
    VEHICLE::_GET_VEHICLE_SUSPENSION_BOUNDS(uParam0->f_26, &vVar17, &uVar18);
    fVar19 = SYSTEM::VDIST2(0f, 0f, vVar15.z, 0f, 0f, vVar17.z);
    uParam0->f_12.f_3.f_2 = (uParam0->f_12.f_3.f_2 + fVar19);
    if (!VEHICLE::_0x51F30DB60626A20E(uParam0->f_26, uParam0->f_12.f_3, uParam0->f_12, 2, 1))
    {
        uParam0->f_12.f_3 = { uParam0->f_6 };
        uParam0->f_12 = { uParam0->f_9 };
    }
}
```



pub fn _0x51f30db60626a20e_safe(
        
        
            vehicle: 
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
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51F30DB60626A20Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51F30DB60626A20Eu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                x, 
                y, 
                z, 
                rotX, 
                rotY, 
                rotZ, 
                p7, 
                p8
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x51f30db60626a20e_raw(
        vehicle: , 
        x: , 
        y: , 
        z: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        p7: , 
        p8: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51F30DB60626A20Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51F30DB60626A20Eu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                x, 
                y, 
                z, 
                rotX, 
                rotY, 
                rotZ, 
                p7, 
                p8
        )
    }
}

/// ## Parameters
*



pub fn _0x9bddc73cc6a115d4_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BDDC73CC6A115D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BDDC73CC6A115D4u64;
        
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
pub fn _0x9bddc73cc6a115d4_raw(
        vehicle: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BDDC73CC6A115D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BDDC73CC6A115D4u64;

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



pub fn is_any_entity_attached_to_handler_frame_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62CA17B74C435651u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62CA17B74C435651u64;
        
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
pub fn is_any_entity_attached_to_handler_frame_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62CA17B74C435651u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62CA17B74C435651u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
returns a string which is the codename of the vehicle's currently selected primary color  
p1 is always 0  
```



pub fn get_vehicle_mod_color_1_name_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB45085B721EFD38Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB45085B721EFD38Cu64;
        
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
pub fn get_vehicle_mod_color_1_name_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB45085B721EFD38Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB45085B721EFD38Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// Last named native above this one is `TRACK_VEHICLE_VISIBILITY` and first named native below is `UNCUFF_PED`. 
Unknown what it does, couldn't find good examples in the decompiled scripts.



pub fn _0x725012a415dba050_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x725012A415DBA050u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x725012A415DBA050u64;
        
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
pub fn _0x725012a415dba050_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x725012A415DBA050u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x725012A415DBA050u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)



pub fn set_vehicle_door_latched_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        , 
        
        
            forceClose: 
        , 
        
        
            lock: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5A9653A8D2CAF48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5A9653A8D2CAF48u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex, 
                forceClose, 
                lock, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_door_latched_raw(
        vehicle: , 
        doorIndex: , 
        forceClose: , 
        lock: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5A9653A8D2CAF48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5A9653A8D2CAF48u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex, 
                forceClose, 
                lock, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_number_plate_text_safe(
        
        
            vehicle: 
        , 
        
        
            plateText: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95A88F0B409CDA47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95A88F0B409CDA47u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                plateText
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_number_plate_text_raw(
        vehicle: , 
        plateText: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95A88F0B409CDA47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95A88F0B409CDA47u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                plateText
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_bumper_broken_off_safe(
        
        
            vehicle: 
        , 
        
        
            front: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x468056A6BB6F3846u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x468056A6BB6F3846u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                front
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_bumper_broken_off_raw(
        vehicle: , 
        front: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x468056A6BB6F3846u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x468056A6BB6F3846u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                front
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_flight_nozzle_position_immediate_safe(
        
        
            vehicle: 
        , 
        
        
            angle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AA47FFF660CB932u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AA47FFF660CB932u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                angle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_flight_nozzle_position_immediate_raw(
        vehicle: , 
        angle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9AA47FFF660CB932u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9AA47FFF660CB932u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                angle
        )
    }
}

/// See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).



pub fn roll_up_window_safe(
        
        
            vehicle: 
        , 
        
        
            windowIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x602E548F46E24D59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x602E548F46E24D59u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                windowIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn roll_up_window_raw(
        vehicle: , 
        windowIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x602E548F46E24D59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x602E548F46E24D59u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                windowIndex
        )
    }
}

/// _CLEAR_VEHICLE_PHONE_EXPLOSIVE_DEVICE native function



pub fn _clear_vehicle_phone_explosive_device_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA3F739ABDDCF21Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA3F739ABDDCF21Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _clear_vehicle_phone_explosive_device_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA3F739ABDDCF21Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA3F739ABDDCF21Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_random_boats_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84436EC293B1415Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84436EC293B1415Fu64;
        
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
pub fn set_random_boats_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84436EC293B1415Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84436EC293B1415Fu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _set_vehicle_rocket_boost_active_safe(
        
        
            vehicle: 
        , 
        
        
            active: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x81E1552E35DC3839u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x81E1552E35DC3839u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                active
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_rocket_boost_active_raw(
        vehicle: , 
        active: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x81E1552E35DC3839u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x81E1552E35DC3839u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                active
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0xb68cfaf83a02768d_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB68CFAF83A02768Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB68CFAF83A02768Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb68cfaf83a02768d_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB68CFAF83A02768Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB68CFAF83A02768Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
Deletes a vehicle.  
The vehicle must be a mission entity to delete, so call this before deleting: SET_ENTITY_AS_MISSION_ENTITY(vehicle, true, true);  
eg how to use:  
SET_ENTITY_AS_MISSION_ENTITY(vehicle, true, true);  
DELETE_VEHICLE(&vehicle);  
Deletes the specified vehicle, then sets the handle pointed to by the pointer to NULL.  
```



pub fn delete_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA386986E786A54Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA386986E786A54Fu64;
        
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
pub fn delete_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA386986E786A54Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA386986E786A54Fu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
GET_VEHICLE_MODEL_*
9.8 * thrust if air vehicle, else 0.38 + drive force?
```



pub fn _get_vehicle_model_estimated_agility_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53409B5163D5B846u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53409B5163D5B846u64;
        
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
pub fn _get_vehicle_model_estimated_agility_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53409B5163D5B846u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53409B5163D5B846u64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// Sets whether the vehicle's lights can be broken.

```
NativeDB Introduced: v323
```



pub fn set_vehicle_has_unbreakable_lights_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1AA8A837D2169D94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1AA8A837D2169D94u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_has_unbreakable_lights_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1AA8A837D2169D94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1AA8A837D2169D94u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)



pub fn set_vehicle_door_broken_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        , 
        
        
            deleteDoor: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4D4F6A4AB575A33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4D4F6A4AB575A33u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex, 
                deleteDoor
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_door_broken_raw(
        vehicle: , 
        doorIndex: , 
        deleteDoor: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4D4F6A4AB575A33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4D4F6A4AB575A33u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex, 
                deleteDoor
        )
    }
}

/// ## Parameters
*



pub fn _set_hydraulic_raised_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28B18377EB6E25F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28B18377EB6E25F6u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_hydraulic_raised_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28B18377EB6E25F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28B18377EB6E25F6u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x1312ddd8385aee4e_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1312DDD8385AEE4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1312DDD8385AEE4Eu64;
        
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
pub fn _0x1312ddd8385aee4e_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1312DDD8385AEE4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1312DDD8385AEE4Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn clear_vehicle_route_history_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D6AF961B72728AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D6AF961B72728AEu64;
        
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
pub fn clear_vehicle_route_history_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D6AF961B72728AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D6AF961B72728AEu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn detach_vehicle_from_cargobob_safe(
        
        
            cargobob: 
        , 
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E21D3DF1051399Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E21D3DF1051399Du64;
        
        let result = invoke_raw!(
            hash,
                cargobob, 
                vehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn detach_vehicle_from_cargobob_raw(
        cargobob: , 
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E21D3DF1051399Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E21D3DF1051399Du64;

        invoke_raw_typed!(
            hash,
                cargobob, 
                vehicle
        )
    }
}

/// ```
p1 is always 0  
```



pub fn set_mission_train_as_no_longer_needed_safe(
        
        
            train: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBE7648349B49BE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBE7648349B49BE8u64;
        
        let result = invoke_raw!(
            hash,
                train, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_mission_train_as_no_longer_needed_raw(
        train: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBBE7648349B49BE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBBE7648349B49BE8u64;

        invoke_raw_typed!(
            hash,
                train, 
                p1
        )
    }
}

/// ```
Max value is 32767
```



pub fn set_vehicle_extended_removal_range_safe(
        
        
            vehicle: 
        , 
        
        
            range: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79DF7E806202CE01u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79DF7E806202CE01u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                range
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_extended_removal_range_raw(
        vehicle: , 
        range: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79DF7E806202CE01u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79DF7E806202CE01u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                range
        )
    }
}

/// ```
Setting this to false, makes the specified vehicle to where if you press Y your character doesn't even attempt the animation to enter the vehicle. Hence it's not considered aka ignored.  
```



pub fn set_vehicle_is_considered_by_player_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31B927BBC44156CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31B927BBC44156CDu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_is_considered_by_player_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31B927BBC44156CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31B927BBC44156CDu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Enables spawning random trains on the preset tracks. 

Requires [`SWITCH_TRAIN_TRACK`](#_0xFD813BB7DB977F20) and [`SET_TRAIN_TRACK_SPAWN_FREQUENCY`](#_0x21973BBF8D17EDFA) to be set.



pub fn set_random_trains_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80D9F74197EA47D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80D9F74197EA47D9u64;
        
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
pub fn set_random_trains_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80D9F74197EA47D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80D9F74197EA47D9u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_max_number_of_passengers_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7C4F2C6E744A550u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7C4F2C6E744A550u64;
        
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
pub fn get_vehicle_max_number_of_passengers_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7C4F2C6E744A550u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7C4F2C6E744A550u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
For a full enum, see here : pastebin.com/i2GGAjY0
char buffer[128];
std::sprintf(buffer, "VEH_CLASS_%i", VEHICLE::GET_VEHICLE_CLASS_FROM_NAME (hash));
const char* className = HUD::_GET_LABEL_TEXT(buffer);
```



pub fn get_vehicle_class_from_name_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEDF1C8BD47C2200u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEDF1C8BD47C2200u64;
        
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
pub fn get_vehicle_class_from_name_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDEDF1C8BD47C2200u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDEDF1C8BD47C2200u64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// ## Parameters
*



pub fn _get_is_wheels_lowered_state_active_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DA0DA9CB3F0C8BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DA0DA9CB3F0C8BFu64;
        
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
pub fn _get_is_wheels_lowered_state_active_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DA0DA9CB3F0C8BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DA0DA9CB3F0C8BFu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn remove_vehicle_upsidedown_check_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC53EB42A499A7E90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC53EB42A499A7E90u64;
        
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
pub fn remove_vehicle_upsidedown_check_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC53EB42A499A7E90u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC53EB42A499A7E90u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_window_tint_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EE21293DAD47C95u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EE21293DAD47C95u64;
        
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
pub fn get_vehicle_window_tint_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0EE21293DAD47C95u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0EE21293DAD47C95u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Enables or disables the ability for a helicopter's tail boom to break off.

```
NativeDB Introduced: v323
```



pub fn set_heli_tail_boom_can_break_off_safe(
        
        
            heli: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3EC8BF18AA453FE9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3EC8BF18AA453FE9u64;
        
        let result = invoke_raw!(
            hash,
                heli, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_heli_tail_boom_can_break_off_raw(
        heli: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3EC8BF18AA453FE9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3EC8BF18AA453FE9u64;

        invoke_raw_typed!(
            hash,
                heli, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_undriveable_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8ABA6AF54B942B95u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8ABA6AF54B942B95u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_undriveable_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8ABA6AF54B942B95u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8ABA6AF54B942B95u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Despite its name, this works on Helicopters and Planes.

Sets the speed of the helicopter blades to full speed.

This is equivalent to calling `SetHeliBladesSpeed(vehicleHandle, 1.0);`



pub fn set_heli_blades_full_speed_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA178472EBB8AE60Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA178472EBB8AE60Du64;
        
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
pub fn set_heli_blades_full_speed_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA178472EBB8AE60Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA178472EBB8AE60Du64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Transforms the `stormberg` to its "water vehicle" variant. If the vehicle is already in that state then the vehicle transformation audio will still play, but the vehicle won't change at all.



pub fn transform_to_submarine_safe(
        
        
            vehicle: 
        , 
        
        
            instantly: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE4C854FFDB6EEBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE4C854FFDB6EEBEu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                instantly
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn transform_to_submarine_raw(
        vehicle: , 
        instantly: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE4C854FFDB6EEBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE4C854FFDB6EEBEu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                instantly
        )
    }
}

/// ## Parameters
*



pub fn set_cargobob_pickup_rope_type_safe(
        
        
            vehicle: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D5F65A8F4EBDAB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D5F65A8F4EBDAB5u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cargobob_pickup_rope_type_raw(
        vehicle: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D5F65A8F4EBDAB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D5F65A8F4EBDAB5u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                state
        )
    }
}

/// ```
Adds some kind of shadow to the vehicle.
-1 disables the effect.
DISABLE_*
```



pub fn _set_vehicle_shadow_effect_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0E4BA16D1DB546Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0E4BA16D1DB546Cu64;
        
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
pub fn _set_vehicle_shadow_effect_raw(
        vehicle: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0E4BA16D1DB546Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0E4BA16D1DB546Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1, 
                p2
        )
    }
}

/// Set a specific offset for helis chasing target in combat

```
NativeDB Introduced: v1180
```



pub fn set_heli_combat_offset_safe(
        
        
            vehicle: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A3F820A9A9A9AC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A3F820A9A9A9AC5u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn set_heli_combat_offset_raw(
        vehicle: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A3F820A9A9A9AC5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A3F820A9A9A9AC5u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_custom_primary_colour_safe(
        
        
            vehicle: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB64CF2CCA9D95F52u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB64CF2CCA9D95F52u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn get_vehicle_custom_primary_colour_raw(
        vehicle: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB64CF2CCA9D95F52u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB64CF2CCA9D95F52u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                r, 
                g, 
                b
        )
    }
}

/// Use [_SET_VEHICLE_HEADLIGHTS_COLOUR](#_0xE41033B25D003A07) to set the headlights color for the vehicle.

You must enable xenon headlights for this native to work properly.

```c
enum eHeadlightColors {
    Default = 255,
    White = 0,
    Blue = 1,
    ElectricBlue = 2,
    MintGreen = 3,
    LimeGreen = 4,
    Yellow = 5,
    GoldenShower = 6,
    Orange = 7,
    Red = 8,
    PonyPink = 9,
    HotPink = 10,
    Purple = 11,
    Blacklight = 12
}
```



pub fn _get_vehicle_xenon_lights_color_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DFF319A831E0CDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DFF319A831E0CDBu64;
        
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
pub fn _get_vehicle_xenon_lights_color_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DFF319A831E0CDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DFF319A831E0CDBu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
If set to true, vehicle will not take crash damage, but is still susceptible to damage from bullets and explosives  
```



pub fn set_vehicle_strong_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E8C8727991A8A0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E8C8727991A8A0Bu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_strong_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E8C8727991A8A0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E8C8727991A8A0Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Enables individual propeller on a propeller plane. This native is the inverse of [`DISABLE_INDIVIDUAL_PLANE_PROPELLER`](#_0x500873A45724C863).

```
NativeDB Introduced: v3407
```



pub fn _enable_individual_plane_propeller_safe(
        
        
            plane: 
        , 
        
        
            propeller: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC05D2777F855F44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC05D2777F855F44u64;
        
        let result = invoke_raw!(
            hash,
                plane, 
                propeller
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _enable_individual_plane_propeller_raw(
        plane: , 
        propeller: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC05D2777F855F44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC05D2777F855F44u64;

        invoke_raw_typed!(
            hash,
                plane, 
                propeller
        )
    }
}

/// ```
in the decompiled scripts, seems to be always called on the vehicle right after being attached to a trailer.
```



pub fn set_trailer_legs_raised_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95CF53B3D687F9FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95CF53B3D687F9FAu64;
        
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
pub fn set_trailer_legs_raised_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95CF53B3D687F9FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95CF53B3D687F9FAu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_has_been_driven_flag_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02398B627547189Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02398B627547189Cu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_has_been_driven_flag_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02398B627547189Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02398B627547189Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_gravity_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89F149B6131E57DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89F149B6131E57DAu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_gravity_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89F149B6131E57DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89F149B6131E57DAu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```c
// eVehicleModType values modified to conform to script native reorganization (see 0x140D25327 in 1604).
enum eVehicleModType
{
	VMT_SPOILER = 0,
	VMT_BUMPER_F = 1,
	VMT_BUMPER_R = 2,
	VMT_SKIRT = 3,
	VMT_EXHAUST = 4,
	VMT_CHASSIS = 5,
	VMT_GRILL = 6,
	VMT_BONNET = 7,
	VMT_WING_L = 8,
	VMT_WING_R = 9,
	VMT_ROOF = 10,
	VMT_ENGINE = 11,
	VMT_BRAKES = 12,
	VMT_GEARBOX = 13,
	VMT_HORN = 14,
	VMT_SUSPENSION = 15,
	VMT_ARMOUR = 16,
	VMT_NITROUS = 17,
	VMT_TURBO = 18,
	VMT_SUBWOOFER = 19,
	VMT_TYRE_SMOKE = 20,
	VMT_HYDRAULICS = 21,
	VMT_XENON_LIGHTS = 22,
	VMT_WHEELS = 23,
	VMT_WHEELS_REAR_OR_HYDRAULICS = 24,
	VMT_PLTHOLDER = 25,
	VMT_PLTVANITY = 26,
	VMT_INTERIOR1 = 27,
	VMT_INTERIOR2 = 28,
	VMT_INTERIOR3 = 29,
	VMT_INTERIOR4 = 30,
	VMT_INTERIOR5 = 31,
	VMT_SEATS = 32,
	VMT_STEERING = 33,
	VMT_KNOB = 34,
	VMT_PLAQUE = 35,
	VMT_ICE = 36,
	VMT_TRUNK = 37,
	VMT_HYDRO = 38,
	VMT_ENGINEBAY1 = 39,
	VMT_ENGINEBAY2 = 40,
	VMT_ENGINEBAY3 = 41,
	VMT_CHASSIS2 = 42,
	VMT_CHASSIS3 = 43,
	VMT_CHASSIS4 = 44,
	VMT_CHASSIS5 = 45,
	VMT_DOOR_L = 46,
	VMT_DOOR_R = 47,
	VMT_LIVERY_MOD = 48,
	VMT_LIGHTBAR = 49,
};
```



pub fn set_vehicle_mod_safe(
        
        
            vehicle: 
        , 
        
        
            modType: 
        , 
        
        
            modIndex: 
        , 
        
        
            customTires: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6AF0636DDEDCB6DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6AF0636DDEDCB6DDu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                modType, 
                modIndex, 
                customTires
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_mod_raw(
        vehicle: , 
        modType: , 
        modIndex: , 
        customTires: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6AF0636DDEDCB6DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6AF0636DDEDCB6DDu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                modType, 
                modIndex, 
                customTires
        )
    }
}

/// STOP_ALL_GARAGE_ACTIVITY native function



pub fn stop_all_garage_activity_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F87E938BDF29D66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F87E938BDF29D66u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_all_garage_activity_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F87E938BDF29D66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F87E938BDF29D66u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn explode_vehicle_in_cutscene_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x786A4EB67B01BF0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x786A4EB67B01BF0Bu64;
        
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
pub fn explode_vehicle_in_cutscene_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x786A4EB67B01BF0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x786A4EB67B01BF0Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// Apply damage to vehicle at a location. Location is relative to vehicle model (not world).
Radius of effect damage applied in a sphere at impact location
When `focusOnModel` set to `true`, the damage sphere will travel towards the vehicle from the given point, thus guaranteeing an impact



pub fn set_vehicle_damage_safe(
        
        
            vehicle: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
        , 
        
        
            damage: 
        , 
        
        
            radius: 
        , 
        
        
            focusOnModel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1DD317EA8FD4F29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1DD317EA8FD4F29u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                xOffset, 
                yOffset, 
                zOffset, 
                damage, 
                radius, 
                focusOnModel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_damage_raw(
        vehicle: , 
        xOffset: , 
        yOffset: , 
        zOffset: , 
        damage: , 
        radius: , 
        focusOnModel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1DD317EA8FD4F29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1DD317EA8FD4F29u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                xOffset, 
                yOffset, 
                zOffset, 
                damage, 
                radius, 
                focusOnModel
        )
    }
}

/// ```
To check if the model is an amphibious car, see gtaforums.com/topic/717612-v-scriptnative-documentation-and-research/page-33#entry1069317363 (for build 944 and above only!)  
```



pub fn is_this_model_a_car_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F6DB52EEFC96DF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F6DB52EEFC96DF8u64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_this_model_a_car_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F6DB52EEFC96DF8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F6DB52EEFC96DF8u64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// Sets the anchor state for a boat.

```
NativeDB Introduced: v323
```



pub fn set_boat_anchor_safe(
        
        
            boat: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75DBEC174AEEAD10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75DBEC174AEEAD10u64;
        
        let result = invoke_raw!(
            hash,
                boat, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_boat_anchor_raw(
        boat: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75DBEC174AEEAD10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75DBEC174AEEAD10u64;

        invoke_raw_typed!(
            hash,
                boat, 
                toggle
        )
    }
}

/// ```
Also includes some "turnOffBones" when vehicle mods are installed.
```



pub fn _get_vehicle_number_of_broken_off_bones_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42A4BEB35D372407u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42A4BEB35D372407u64;
        
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
pub fn _get_vehicle_number_of_broken_off_bones_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42A4BEB35D372407u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42A4BEB35D372407u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _set_disable_vehicle_unk_2_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x211E95CE9903940Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x211E95CE9903940Cu64;
        
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
pub fn _set_disable_vehicle_unk_2_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x211E95CE9903940Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x211E95CE9903940Cu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_timed_explosion_safe(
        
        
            vehicle: 
        , 
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E0A74E1002380B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E0A74E1002380B1u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                ped, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_timed_explosion_raw(
        vehicle: , 
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E0A74E1002380B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E0A74E1002380B1u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                ped, 
                toggle
        )
    }
}

/// Gets hash related to task happening with seat index
Native name: GET_I*



pub fn _0xa01bc64dd4bfbbac_safe(
        
        
            vehicle: 
        , 
        
        
            seatIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA01BC64DD4BFBBACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA01BC64DD4BFBBACu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                seatIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa01bc64dd4bfbbac_raw(
        vehicle: , 
        seatIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA01BC64DD4BFBBACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA01BC64DD4BFBBACu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                seatIndex
        )
    }
}

/// ## Parameters
*



pub fn preload_vehicle_mod_safe(
        
        
            p0: 
        , 
        
        
            modType: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x758F49C24925568Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x758F49C24925568Au64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                modType, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn preload_vehicle_mod_raw(
        p0: , 
        modType: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x758F49C24925568Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x758F49C24925568Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                modType, 
                p2
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn disable_individual_plane_propeller_safe(
        
        
            vehicle: 
        , 
        
        
            propeller: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x500873A45724C863u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x500873A45724C863u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                propeller
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_individual_plane_propeller_raw(
        vehicle: , 
        propeller: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x500873A45724C863u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x500873A45724C863u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                propeller
        )
    }
}

/// ## Parameters
*



pub fn set_number_of_parked_vehicles_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCAA15F13EBD417FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCAA15F13EBD417FFu64;
        
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
pub fn set_number_of_parked_vehicles_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCAA15F13EBD417FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCAA15F13EBD417FFu64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Parameters
*



pub fn _0xc0ed6438e6d39ba8_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0ED6438E6D39BA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0ED6438E6D39BA8u64;
        
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
pub fn _0xc0ed6438e6d39ba8_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0ED6438E6D39BA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0ED6438E6D39BA8u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// Checks whether the specified boat vehicle is capsized, meaning it has overturned or is upside down in the water.



pub fn get_is_boat_capsized_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA91D045575699ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA91D045575699ADu64;
        
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
pub fn get_is_boat_capsized_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA91D045575699ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA91D045575699ADu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _get_vehicle_dashboard_color_safe(
        
        
            vehicle: 
        , 
        
        
            color: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7635E80A5C31BFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7635E80A5C31BFFu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                color
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_vehicle_dashboard_color_raw(
        vehicle: , 
        color: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7635E80A5C31BFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7635E80A5C31BFFu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                color
        )
    }
}

/// ## Parameters
*



pub fn get_position_of_vehicle_recording_id_at_time_safe(
        
        
            id: 
        , 
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92523B76657A517Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92523B76657A517Du64;
        
        let result = invoke_raw!(
            hash,
                id, 
                time
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_position_of_vehicle_recording_id_at_time_raw(
        id: , 
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92523B76657A517Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92523B76657A517Du64;

        invoke_raw_typed!(
            hash,
                id, 
                time
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_can_save_in_garage_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x428BACCDF5E26EADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x428BACCDF5E26EADu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_can_save_in_garage_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x428BACCDF5E26EADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x428BACCDF5E26EADu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_class_max_acceleration_safe(
        
        
            vehicleClass: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F83E7E45D9EA7AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F83E7E45D9EA7AEu64;
        
        let result = invoke_raw!(
            hash,
                vehicleClass
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_class_max_acceleration_raw(
        vehicleClass: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F83E7E45D9EA7AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F83E7E45D9EA7AEu64;

        invoke_raw_typed!(
            hash,
                vehicleClass
        )
    }
}

/// Returns the display name/text label (`gameName` in `vehicles.meta`) for the specified vehicle model.



pub fn get_display_name_from_vehicle_model_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB215AAC32D25D019u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB215AAC32D25D019u64;
        
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
pub fn get_display_name_from_vehicle_model_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB215AAC32D25D019u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB215AAC32D25D019u64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// ```
makes the train all jumbled up and derailed as it moves on the tracks (though that wont stop it from its normal operations)  
```



pub fn set_render_train_as_derailed_safe(
        
        
            train: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x317B11A312DF5534u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x317B11A312DF5534u64;
        
        let result = invoke_raw!(
            hash,
                train, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_render_train_as_derailed_raw(
        train: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x317B11A312DF5534u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x317B11A312DF5534u64;

        invoke_raw_typed!(
            hash,
                train, 
                toggle
        )
    }
}

/// Refer to [GET_VEHICLE_WHEEL_TYPE](#_0xB3ED1BFB4BE636DC) for wheel types.



pub fn set_vehicle_wheel_type_safe(
        
        
            vehicle: 
        , 
        
        
            wheelType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x487EB21CC7295BA1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x487EB21CC7295BA1u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                wheelType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_wheel_type_raw(
        vehicle: , 
        wheelType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x487EB21CC7295BA1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x487EB21CC7295BA1u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                wheelType
        )
    }
}

/// ```
NativeDB Introduced: v1365
```



pub fn _set_vehicle_doors_locked_for_unk_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x203B527D1B77904Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x203B527D1B77904Cu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_doors_locked_for_unk_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x203B527D1B77904Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x203B527D1B77904Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_attached_to_trailer_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7CF3C4F9F489F0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7CF3C4F9F489F0Cu64;
        
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
pub fn is_vehicle_attached_to_trailer_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7CF3C4F9F489F0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7CF3C4F9F489F0Cu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_force_hd_vehicle_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97CE68CB032583F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97CE68CB032583F0u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_force_hd_vehicle_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97CE68CB032583F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97CE68CB032583F0u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
Locks the vehicle's steering to the desired angle, explained below.  
Requires to be called onTick. Steering is unlocked the moment the function stops being called on the vehicle.  
Steer bias:  
-1.0 = full right  
0.0 = centered steering  
1.0 = full left  
```



pub fn set_vehicle_steer_bias_safe(
        
        
            vehicle: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42A8EC77D5150CBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42A8EC77D5150CBEu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_steer_bias_raw(
        vehicle: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x42A8EC77D5150CBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x42A8EC77D5150CBEu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                value
        )
    }
}

/// ## Parameters
*



pub fn _set_vehicle_rocket_boost_percentage_safe(
        
        
            vehicle: 
        , 
        
        
            percentage: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEB2DDED3509562Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEB2DDED3509562Eu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                percentage
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_rocket_boost_percentage_raw(
        vehicle: , 
        percentage: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFEB2DDED3509562Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFEB2DDED3509562Eu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                percentage
        )
    }
}

/// ```
SET_VEHICLE_AL*
```



pub fn _set_vehicle_can_be_locked_on_safe(
        
        
            vehicle: 
        , 
        
        
            canBeLockedOn: 
        , 
        
        
            unk: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DDA078D12879EEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DDA078D12879EEEu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                canBeLockedOn, 
                unk
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_can_be_locked_on_raw(
        vehicle: , 
        canBeLockedOn: , 
        unk: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DDA078D12879EEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DDA078D12879EEEu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                canBeLockedOn, 
                unk
        )
    }
}

/// ```
Inverts vehicle's controls. So INPUT_VEH_ACCELERATE will be INPUT_VEH_BRAKE and vise versa (same for A/D controls)
Doesn't work for planes/helis.
```



pub fn _set_vehicle_controls_inverted_safe(
        
        
            vehicle: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B91B229243351A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B91B229243351A8u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_controls_inverted_raw(
        vehicle: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B91B229243351A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B91B229243351A8u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                state
        )
    }
}

/// ```
Only used for wheels(ModType = 23/24) Returns true if the wheels are custom wheels
```



pub fn get_vehicle_mod_variation_safe(
        
        
            vehicle: 
        , 
        
        
            modType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3924ECD70E095DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3924ECD70E095DCu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                modType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_mod_variation_raw(
        vehicle: , 
        modType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3924ECD70E095DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3924ECD70E095DCu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                modType
        )
    }
}

/// ## Parameters
*



pub fn add_vehicle_phone_explosive_device_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99AD4CCCB128CBC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99AD4CCCB128CBC9u64;
        
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
pub fn add_vehicle_phone_explosive_device_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99AD4CCCB128CBC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99AD4CCCB128CBC9u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
0.0 = Lowest 1.0 = Highest. This is best to be used if you wanna pick-up a car since un-realistically on GTA V forklifts can't pick up much of anything due to vehicle mass. If you put this under a car then set it above 0.0 to a 'lifted-value' it will raise the car with no issue lol
```



pub fn set_forklift_fork_height_safe(
        
        
            vehicle: 
        , 
        
        
            height: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37EBBF3117BD6A25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37EBBF3117BD6A25u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                height
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_forklift_fork_height_raw(
        vehicle: , 
        height: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37EBBF3117BD6A25u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37EBBF3117BD6A25u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                height
        )
    }
}

/// ## Parameters
*



pub fn _is_this_model_an_amphibious_car_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x633F6F44A537EBB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x633F6F44A537EBB6u64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_this_model_an_amphibious_car_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x633F6F44A537EBB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x633F6F44A537EBB6u64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// ## Parameters
*



pub fn _0xae3fee8709b39dcb_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE3FEE8709B39DCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE3FEE8709B39DCBu64;
        
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
pub fn _0xae3fee8709b39dcb_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE3FEE8709B39DCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE3FEE8709B39DCBu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
<1.0 - Decreased torque
=1.0 - Default torque
>1.0 - Increased torque
Negative values will cause the vehicle to go backwards instead of forwards while accelerating.
value - is between 0.2 and 1.8 in the decompiled scripts.
This needs to be called every frame to take effect.
```



pub fn set_vehicle_cheat_power_increase_safe(
        
        
            vehicle: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB59E4BD37AE292DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB59E4BD37AE292DBu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_cheat_power_increase_raw(
        vehicle: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB59E4BD37AE292DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB59E4BD37AE292DBu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                value
        )
    }
}

/// ```
Returns max speed (without mods) of the specified vehicle model in m/s.
```



pub fn get_vehicle_model_estimated_max_speed_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF417C2502FFFED43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF417C2502FFFED43u64;
        
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
pub fn get_vehicle_model_estimated_max_speed_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF417C2502FFFED43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF417C2502FFFED43u64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// ## Parameters
*



pub fn _are_plane_wings_intact_safe(
        
        
            plane: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5991A01434CE9677u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5991A01434CE9677u64;
        
        let result = invoke_raw!(
            hash,
                plane
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _are_plane_wings_intact_raw(
        plane: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5991A01434CE9677u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5991A01434CE9677u64;

        invoke_raw_typed!(
            hash,
                plane
        )
    }
}

/// ```
SET_TIME_POSITION_IN_RECORDING can be emulated by: desired_time - GET_TIME_POSITION_IN_RECORDING(vehicle)
```



pub fn skip_time_in_playback_recorded_vehicle_safe(
        
        
            vehicle: 
        , 
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9438F7AD68771A20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9438F7AD68771A20u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                time
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn skip_time_in_playback_recorded_vehicle_raw(
        vehicle: , 
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9438F7AD68771A20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9438F7AD68771A20u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                time
        )
    }
}

/// ```
what does this do?  
```



pub fn _0xcfd778e7904c255e_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFD778E7904C255Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFD778E7904C255Eu64;
        
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
pub fn _0xcfd778e7904c255e_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFD778E7904C255Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFD778E7904C255Eu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _0x9becd4b9fef3f8a6_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BECD4B9FEF3F8A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BECD4B9FEF3F8A6u64;
        
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
pub fn _0x9becd4b9fef3f8a6_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BECD4B9FEF3F8A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BECD4B9FEF3F8A6u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
Tested on the player's current vehicle. Unless you kill the driver, the vehicle doesn't loose control, however, if enabled, explodeOnImpact is still active. The moment you crash, boom.  
```



pub fn set_vehicle_out_of_control_safe(
        
        
            vehicle: 
        , 
        
        
            killDriver: 
        , 
        
        
            explodeOnImpact: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF19D095E42D430CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF19D095E42D430CCu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                killDriver, 
                explodeOnImpact
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_out_of_control_raw(
        vehicle: , 
        killDriver: , 
        explodeOnImpact: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF19D095E42D430CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF19D095E42D430CCu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                killDriver, 
                explodeOnImpact
        )
    }
}

/// Returns the plates a vehicle has.

```c
enum eVehiclePlateType
{
	VPT_FRONT_AND_BACK_PLATES = 0,
	VPT_FRONT_PLATES = 1,
	VPT_BACK_PLATES = 2,
	VPT_NONE = 3,
};
```

Motorcycles with no visible plates will sometimes return a 2 for unknown reasons.



pub fn get_vehicle_plate_type_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CCC9525BF2408E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CCC9525BF2408E0u64;
        
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
pub fn get_vehicle_plate_type_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CCC9525BF2408E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CCC9525BF2408E0u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn is_playback_going_on_for_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C8A4C2C19E68EECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C8A4C2C19E68EECu64;
        
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
pub fn is_playback_going_on_for_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C8A4C2C19E68EECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C8A4C2C19E68EECu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Calling this native will keep a vehicle's engine running after exiting.



pub fn set_vehicle_keep_engine_on_when_abandoned_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8FBC8B1330CA9B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8FBC8B1330CA9B4u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_keep_engine_on_when_abandoned_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8FBC8B1330CA9B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8FBC8B1330CA9B4u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_last_driven_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xACFB2463CC22BED2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xACFB2463CC22BED2u64;
        
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
pub fn set_last_driven_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xACFB2463CC22BED2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xACFB2463CC22BED2u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _get_number_of_vehicle_doors_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92922A607497B14Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92922A607497B14Du64;
        
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
pub fn _get_number_of_vehicle_doors_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92922A607497B14Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92922A607497B14Du64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_model_max_braking_max_mods_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFBA3BA79CFF7EBFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFBA3BA79CFF7EBFu64;
        
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
pub fn get_vehicle_model_max_braking_max_mods_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFBA3BA79CFF7EBFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFBA3BA79CFF7EBFu64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// This native checks if the given vehicle is stopped at a red or amber traffic light junction, provided the driver's personality is set to not run amber lights.



pub fn is_vehicle_stopped_at_traffic_lights_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2959F696AE390A99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2959F696AE390A99u64;
        
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
pub fn is_vehicle_stopped_at_traffic_lights_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2959F696AE390A99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2959F696AE390A99u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_heli_turbulence_scalar_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6F13851780394DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6F13851780394DAu64;
        
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
pub fn set_heli_turbulence_scalar_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6F13851780394DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6F13851780394DAu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// Used to delete mission trains created with [`CREATE_MISSION_TRAIN`](#_0x63C6CCA8E68AE8C8).



pub fn delete_mission_train_safe(
        
        
            train: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B76B14AE875C795u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B76B14AE875C795u64;
        
        let result = invoke_raw!(
            hash,
                train
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn delete_mission_train_raw(
        train: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B76B14AE875C795u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B76B14AE875C795u64;

        invoke_raw_typed!(
            hash,
                train
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_cause_of_destruction_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE495D1EF4C91FD20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE495D1EF4C91FD20u64;
        
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
pub fn get_vehicle_cause_of_destruction_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE495D1EF4C91FD20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE495D1EF4C91FD20u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn clear_vehicle_custom_primary_colour_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55E1D2758F34E437u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55E1D2758F34E437u64;
        
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
pub fn clear_vehicle_custom_primary_colour_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55E1D2758F34E437u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55E1D2758F34E437u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Stops CTaskBringVehicleToHalt
```



pub fn _stop_bring_vehicle_to_halt_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C06330BFDDA182Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C06330BFDDA182Eu64;
        
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
pub fn _stop_bring_vehicle_to_halt_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C06330BFDDA182Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C06330BFDDA182Eu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _set_drift_tyres_enabled_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AC79C98C5C17F05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AC79C98C5C17F05u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_drift_tyres_enabled_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AC79C98C5C17F05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AC79C98C5C17F05u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _is_mission_train_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD464F2E18836BFCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD464F2E18836BFCu64;
        
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
pub fn _is_mission_train_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD464F2E18836BFCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD464F2E18836BFCu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)



pub fn set_vehicle_door_control_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        , 
        
        
            speed: 
        , 
        
        
            angle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2BFA0430F0A0FCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2BFA0430F0A0FCBu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex, 
                speed, 
                angle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_door_control_raw(
        vehicle: , 
        doorIndex: , 
        speed: , 
        angle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2BFA0430F0A0FCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2BFA0430F0A0FCBu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex, 
                speed, 
                angle
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x107a473d7a6647a9_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x107A473D7A6647A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x107A473D7A6647A9u64;
        
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
pub fn _0x107a473d7a6647a9_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x107A473D7A6647A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x107A473D7A6647A9u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn is_heli_landing_area_blocked_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x634148744F385576u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x634148744F385576u64;
        
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
pub fn is_heli_landing_area_blocked_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x634148744F385576u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x634148744F385576u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _0x796a877e459b99ea_safe(
        
        
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
        let hash = 0x796A877E459B99EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x796A877E459B99EAu64;
        
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
pub fn _0x796a877e459b99ea_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x796A877E459B99EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x796A877E459B99EAu64;

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
Allows creation of CEventShockingPlaneFlyby, CEventShockingHelicopterOverhead, and other(?) Shocking events
```



pub fn set_vehicle_generates_engine_shocking_events_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x279D50DE5652D935u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x279D50DE5652D935u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_generates_engine_shocking_events_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x279D50DE5652D935u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x279D50DE5652D935u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _hide_vehicle_tombstone_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE71FB656C600587u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE71FB656C600587u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _hide_vehicle_tombstone_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE71FB656C600587u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE71FB656C600587u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Returns true when the bomb bay doors of this plane are open. False if they're closed.



pub fn _are_bomb_bay_doors_open_safe(
        
        
            aircraft: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0917A423314BBA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0917A423314BBA8u64;
        
        let result = invoke_raw!(
            hash,
                aircraft
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _are_bomb_bay_doors_open_raw(
        aircraft: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0917A423314BBA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0917A423314BBA8u64;

        invoke_raw_typed!(
            hash,
                aircraft
        )
    }
}

/// ## Parameters
*



pub fn _0xbb2333bb87ddd87f_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB2333BB87DDD87Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB2333BB87DDD87Fu64;
        
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
pub fn _0xbb2333bb87ddd87f_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB2333BB87DDD87Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB2333BB87DDD87Fu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _is_ped_exclusive_driver_of_vehicle_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        , 
        
        
            outIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB09D25E77C33EB3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB09D25E77C33EB3Fu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                vehicle, 
                outIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_ped_exclusive_driver_of_vehicle_raw(
        ped: , 
        vehicle: , 
        outIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB09D25E77C33EB3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB09D25E77C33EB3Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle, 
                outIndex
        )
    }
}

/// Enables or disables the opening of a vehicle's rear doors in the event of a sticky bomb explosion. This native is effective for armored vehicles, such as the Stockade (Brinks vehicle), allowing the rear doors to be opened through controlled explosions, which might otherwise remain locked due to the vehicle nature.



pub fn set_open_rear_doors_on_explosion_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B212B26DD3C04DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B212B26DD3C04DFu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_open_rear_doors_on_explosion_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1B212B26DD3C04DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1B212B26DD3C04DFu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_plane_resist_to_explosion_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE16142B94664DEFDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE16142B94664DEFDu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_plane_resist_to_explosion_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE16142B94664DEFDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE16142B94664DEFDu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Retrieves a static value representing the maximum drive force of specific a vehicle, including any vehicle mods. This value does not change dynamically during gameplay. This value provides an approximation and should be considered alongside other performance metrics like top speed for a more comprehensive understanding of the vehicle's capabilities.

```
NativeDB Introduced: v323
```



pub fn get_vehicle_acceleration_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DD35C8D074E57AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DD35C8D074E57AEu64;
        
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
pub fn get_vehicle_acceleration_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DD35C8D074E57AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DD35C8D074E57AEu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Returns a number of available rooftop liveries, or -1 if vehicle has no rooftop liveries available.



pub fn _get_vehicle_roof_livery_count_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5ECB40269053C0D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5ECB40269053C0D4u64;
        
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
pub fn _get_vehicle_roof_livery_count_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5ECB40269053C0D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5ECB40269053C0D4u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Seems related to vehicle health, like the one in IV.  
Max 1000, min 0.  
Vehicle does not necessarily explode or become undrivable at 0.  
```



pub fn get_vehicle_body_health_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF271147EB7B40F12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF271147EB7B40F12u64;
        
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
pub fn get_vehicle_body_health_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF271147EB7B40F12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF271147EB7B40F12u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Related to monster trucks in native scripts.
```

```
NativeDB Introduced: v1604
```



pub fn _set_vehicle_wheels_deal_damage_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2970EAA18FD5E42Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2970EAA18FD5E42Fu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_wheels_deal_damage_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2970EAA18FD5E42Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2970EAA18FD5E42Fu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _set_vehicle_ramp_launch_modifier_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFC13B1CE30D755Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFC13B1CE30D755Du64;
        
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
pub fn _set_vehicle_ramp_launch_modifier_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFC13B1CE30D755Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFC13B1CE30D755Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _set_vehicle_weapons_disabled_safe(
        
        
            vehicle: 
        , 
        
        
            weaponSlot: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x86B4B6212CB8B627u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x86B4B6212CB8B627u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                weaponSlot
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_weapons_disabled_raw(
        vehicle: , 
        weaponSlot: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x86B4B6212CB8B627u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x86B4B6212CB8B627u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                weaponSlot
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_class_max_traction_safe(
        
        
            vehicleClass: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBC86D85C5059461u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBC86D85C5059461u64;
        
        let result = invoke_raw!(
            hash,
                vehicleClass
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_class_max_traction_raw(
        vehicleClass: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBC86D85C5059461u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBC86D85C5059461u64;

        invoke_raw_typed!(
            hash,
                vehicleClass
        )
    }
}

/// ## Parameters
*



pub fn is_this_model_a_heli_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCE4334788AF94EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCE4334788AF94EAu64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_this_model_a_heli_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCE4334788AF94EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCE4334788AF94EAu64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// Similar to [`_SET_AIRCRAFT_BOMB_COUNT`](#_0xF4B2ED59DEB5D774), this sets the amount of countermeasures that are present on this vehicle.

Use [`_GET_AIRCRAFT_COUNTERMEASURE_COUNT`](#_0xF846AA63DF56B804) to get the current amount.



pub fn _set_vehicle_countermeasure_count_safe(
        
        
            aircraft: 
        , 
        
        
            count: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BDA23BF666F0855u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BDA23BF666F0855u64;
        
        let result = invoke_raw!(
            hash,
                aircraft, 
                count
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_countermeasure_count_raw(
        aircraft: , 
        count: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BDA23BF666F0855u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BDA23BF666F0855u64;

        invoke_raw_typed!(
            hash,
                aircraft, 
                count
        )
    }
}

/// ## Parameters
*



pub fn _0x6501129c9e0ffa05_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6501129C9E0FFA05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6501129C9E0FFA05u64;
        
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
pub fn _0x6501129c9e0ffa05_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6501129C9E0FFA05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6501129C9E0FFA05u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn remove_vehicle_stuck_check_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8386BFB614D06749u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8386BFB614D06749u64;
        
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
pub fn remove_vehicle_stuck_check_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8386BFB614D06749u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8386BFB614D06749u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
1000 is max health  
Begins leaking gas at around 650 health  
```



pub fn set_vehicle_petrol_tank_health_safe(
        
        
            vehicle: 
        , 
        
        
            health: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70DB57649FA8D0D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70DB57649FA8D0D8u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                health
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_petrol_tank_health_raw(
        vehicle: , 
        health: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70DB57649FA8D0D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70DB57649FA8D0D8u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                health
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_steering_bias_scalar_safe(
        
        
            vehicle: 
        , 
        
        
            scalar: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9007A2F21DC108D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9007A2F21DC108D4u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                scalar
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_steering_bias_scalar_raw(
        vehicle: , 
        scalar: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9007A2F21DC108D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9007A2F21DC108D4u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                scalar
        )
    }
}

/// Only used with the "akula" in the decompiled native scripts.

```
NativeDB Introduced: v1290
```



pub fn _are_heli_stub_wings_deployed_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAEF12960FA943792u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAEF12960FA943792u64;
        
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
pub fn _are_heli_stub_wings_deployed_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAEF12960FA943792u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAEF12960FA943792u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_high_detail_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F25887F3C104278u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F25887F3C104278u64;
        
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
pub fn is_vehicle_high_detail_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F25887F3C104278u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F25887F3C104278u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Appears to return true if the vehicle has any damage, including cosmetically.
GET_*
```



pub fn _is_vehicle_damaged_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCDC5017D3CE1E9Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCDC5017D3CE1E9Eu64;
        
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
pub fn _is_vehicle_damaged_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCDC5017D3CE1E9Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCDC5017D3CE1E9Eu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn is_big_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F243D3919F442FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F243D3919F442FEu64;
        
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
pub fn is_big_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F243D3919F442FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F243D3919F442FEu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Seat indices range from -1 to [`GET_VEHICLE_MAX_NUMBER_OF_PASSENGERS`](#_0xA7C4F2C6E744A550) minus one.

```c
// CTaskExitVehicleSeat::eSeatPosition - 1
enum eSeatPosition
{
    SF_FrontDriverSide = -1,
    SF_FrontPassengerSide = 0,
    SF_BackDriverSide = 1,
    SF_BackPassengerSide = 2,
    SF_AltFrontDriverSide = 3,
    SF_AltFrontPassengerSide = 4,
    SF_AltBackDriverSide = 5,
    SF_AltBackPassengerSide = 6,
};
```

```
NativeDB Added Parameter 3: BOOL isTaskRunning

isTaskRunning = on true the function returns already false while a task on the target seat is running (TASK_ENTER_VEHICLE/TASK_SHUFFLE_TO_NEXT_VEHICLE_SEAT) - on false only when a ped is finally sitting in the seat.
```



pub fn is_vehicle_seat_free_safe(
        
        
            vehicle: 
        , 
        
        
            seatIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22AC59A870E6A669u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22AC59A870E6A669u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                seatIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_seat_free_raw(
        vehicle: , 
        seatIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22AC59A870E6A669u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22AC59A870E6A669u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                seatIndex
        )
    }
}

/// ```
NativeDB Introduced: v3258
```



pub fn _get_vehicle_drivetrain_type_safe(
        
        
            vehicleModel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1423725069EE1D14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1423725069EE1D14u64;
        
        let result = invoke_raw!(
            hash,
                vehicleModel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_vehicle_drivetrain_type_raw(
        vehicleModel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1423725069EE1D14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1423725069EE1D14u64;

        invoke_raw_typed!(
            hash,
                vehicleModel
        )
    }
}

/// Starts or stops the engine on the specified vehicle.
From what I've tested when I do this to a helicopter the propellers turn off after the engine has started.



pub fn set_vehicle_engine_on_safe(
        
        
            vehicle: 
        , 
        
        
            value: 
        , 
        
        
            instantly: 
        , 
        
        
            disableAutoStart: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2497C4717C8B881Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2497C4717C8B881Eu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                value, 
                instantly, 
                disableAutoStart
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_engine_on_raw(
        vehicle: , 
        value: , 
        instantly: , 
        disableAutoStart: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2497C4717C8B881Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2497C4717C8B881Eu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                value, 
                instantly, 
                disableAutoStart
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0x6a973569ba094650_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A973569BA094650u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A973569BA094650u64;
        
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
pub fn _0x6a973569ba094650_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A973569BA094650u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A973569BA094650u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _set_vehicle_parachute_active_safe(
        
        
            vehicle: 
        , 
        
        
            active: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BFFB028B3DD0A97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BFFB028B3DD0A97u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                active
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_parachute_active_raw(
        vehicle: , 
        active: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BFFB028B3DD0A97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BFFB028B3DD0A97u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                active
        )
    }
}

/// ```
Usually used alongside other vehicle door natives.
```



pub fn _0x3b458ddb57038f08_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B458DDB57038F08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B458DDB57038F08u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x3b458ddb57038f08_raw(
        vehicle: , 
        doorIndex: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B458DDB57038F08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B458DDB57038F08u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex, 
                toggle
        )
    }
}

/// ```
Reduces grip significantly so it's hard to go anywhere.  
```



pub fn set_vehicle_reduce_grip_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x222FF6A823D122E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x222FF6A823D122E2u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_reduce_grip_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x222FF6A823D122E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x222FF6A823D122E2u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0xe5810ac70602f2f5_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE5810AC70602F2F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE5810AC70602F2F5u64;
        
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
pub fn _0xe5810ac70602f2f5_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE5810AC70602F2F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE5810AC70602F2F5u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
parachuteModel = 230075693  
```



pub fn _set_vehicle_parachute_model_safe(
        
        
            vehicle: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D610C6B56031351u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D610C6B56031351u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_parachute_model_raw(
        vehicle: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D610C6B56031351u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D610C6B56031351u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                modelHash
        )
    }
}

/// ## Parameters
*



pub fn _0xab31ef4de6800ce9_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB31EF4DE6800CE9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB31EF4DE6800CE9u64;
        
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
pub fn _0xab31ef4de6800ce9_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB31EF4DE6800CE9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB31EF4DE6800CE9u64;

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



pub fn _get_tyre_health_safe(
        
        
            vehicle: 
        , 
        
        
            wheelIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55EAB010FAEE9380u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55EAB010FAEE9380u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                wheelIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_tyre_health_raw(
        vehicle: , 
        wheelIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55EAB010FAEE9380u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55EAB010FAEE9380u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                wheelIndex
        )
    }
}

/// ```
Hardcoded to not work in multiplayer.  
```



pub fn set_can_respray_vehicle_safe(
        
        
            vehicle: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52BBA29D5EC69356u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52BBA29D5EC69356u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_can_respray_vehicle_raw(
        vehicle: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52BBA29D5EC69356u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52BBA29D5EC69356u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                state
        )
    }
}

/// ```
Returns max number of passengers (including the driver) for the specified vehicle model.
```



pub fn get_vehicle_model_number_of_seats_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2AD93716F184EDA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2AD93716F184EDA4u64;
        
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
pub fn get_vehicle_model_number_of_seats_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2AD93716F184EDA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2AD93716F184EDA4u64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_provides_cover_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AFEEDD9BB2899D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AFEEDD9BB2899D7u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_provides_cover_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AFEEDD9BB2899D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AFEEDD9BB2899D7u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// This native sets whether a specific vehicle influences the player's wanted level when it is involved in an incident that typically triggers a wanted response, such as being marked as a "victim" vehicle.

This is particularly useful when utilizing the wanted system from GTA, and you want to prevent a vehicle from affecting the wanted level when it is stolen. In the decompiled scripts this native is only used to disable the influence of the vehicle on the wanted level.



pub fn set_vehicle_influences_wanted_level_safe(
        
        
            vehicle: 
        , 
        
        
            influenceWantedLevel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AD9E8F87FF7C16Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AD9E8F87FF7C16Fu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                influenceWantedLevel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_influences_wanted_level_raw(
        vehicle: , 
        influenceWantedLevel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0AD9E8F87FF7C16Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0AD9E8F87FF7C16Fu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                influenceWantedLevel
        )
    }
}

/// Gets the amount of bombs that this vehicle has. As far as I know, this does _not_ impact vehicle weapons or the ammo of those weapons in any way, it is just a way to keep track of the amount of bombs in a specific plane. 

In decompiled scripts this is used to check if the vehicle has enough bombs before a bomb can be dropped (bombs are dropped by using [`_SHOOT_SINGLE_BULLET_BETWEEN_COORDS_WITH_EXTRA_PARAMS`](#_0xBFE5756E7407064A)). 

Use [`_SET_AIRCRAFT_BOMB_COUNT`](#_0xF4B2ED59DEB5D774) to set the amount of bombs on that vehicle.



pub fn _get_vehicle_bomb_count_safe(
        
        
            aircraft: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA12BD130D7569A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA12BD130D7569A1u64;
        
        let result = invoke_raw!(
            hash,
                aircraft
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_vehicle_bomb_count_raw(
        aircraft: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEA12BD130D7569A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEA12BD130D7569A1u64;

        invoke_raw_typed!(
            hash,
                aircraft
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _0x59c3757b3b7408e8_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59C3757B3B7408E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59C3757B3B7408E8u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x59c3757b3b7408e8_raw(
        vehicle: , 
        toggle: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59C3757B3B7408E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59C3757B3B7408E8u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_bumper_bouncing_safe(
        
        
            vehicle: 
        , 
        
        
            frontBumper: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27B926779DEB502Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27B926779DEB502Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                frontBumper
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_bumper_bouncing_raw(
        vehicle: , 
        frontBumper: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27B926779DEB502Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27B926779DEB502Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                frontBumper
        )
    }
}

/// Gets the color of the neon lights of the specified vehicle.  

See [`_SET_VEHICLE_NEON_LIGHTS_COLOUR`](#_0x8E0A582209A62695) for more information



pub fn _get_vehicle_neon_lights_colour_safe(
        
        
            vehicle: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7619EEE8C886757Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7619EEE8C886757Fu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn _get_vehicle_neon_lights_colour_raw(
        vehicle: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7619EEE8C886757Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7619EEE8C886757Fu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                r, 
                g, 
                b
        )
    }
}

/// ## Parameters
*



pub fn does_script_vehicle_generator_exist_safe(
        
        
            vehicleGenerator: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6086BC836400876u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6086BC836400876u64;
        
        let result = invoke_raw!(
            hash,
                vehicleGenerator
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_script_vehicle_generator_exist_raw(
        vehicleGenerator: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6086BC836400876u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6086BC836400876u64;

        invoke_raw_typed!(
            hash,
                vehicleGenerator
        )
    }
}

/// ## Parameters
*



pub fn stop_playback_recorded_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54833611C17ABDEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54833611C17ABDEAu64;
        
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
pub fn stop_playback_recorded_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x54833611C17ABDEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54833611C17ABDEAu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
SET_VEHICLE_AL*
```



pub fn _0x7d6f9a3ef26136a0_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D6F9A3EF26136A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D6F9A3EF26136A0u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7d6f9a3ef26136a0_raw(
        vehicle: , 
        toggle: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D6F9A3EF26136A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D6F9A3EF26136A0u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle, 
                p2
        )
    }
}

/// ```
Corrected p1. it's basically the 'carriage/trailer number'. So if the train has 3 trailers you'd call the native once with a var or 3 times with 1, 2, 3.  
```



pub fn get_train_carriage_safe(
        
        
            train: 
        , 
        
        
            trailerNumber: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x08AAFD0814722BC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x08AAFD0814722BC3u64;
        
        let result = invoke_raw!(
            hash,
                train, 
                trailerNumber
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_train_carriage_raw(
        train: , 
        trailerNumber: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x08AAFD0814722BC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x08AAFD0814722BC3u64;

        invoke_raw_typed!(
            hash,
                train, 
                trailerNumber
        )
    }
}

/// ## Parameters
*



pub fn _disable_vehicle_neon_lights_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83F813570FF519DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83F813570FF519DEu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _disable_vehicle_neon_lights_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83F813570FF519DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83F813570FF519DEu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
Returns true if the vehicle has the FLAG_JUMPING_CAR flag set.
```



pub fn _get_can_vehicle_jump_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9078C0C5EF8C19E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9078C0C5EF8C19E9u64;
        
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
pub fn _get_can_vehicle_jump_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9078C0C5EF8C19E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9078C0C5EF8C19E9u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Checks if a boat can be anchored at its present position without possibly intersecting collision later.

```
NativeDB Introduced: v323
```



pub fn can_anchor_boat_here_safe(
        
        
            boat: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26C10ECBDA5D043Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26C10ECBDA5D043Bu64;
        
        let result = invoke_raw!(
            hash,
                boat
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_anchor_boat_here_raw(
        boat: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26C10ECBDA5D043Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26C10ECBDA5D043Bu64;

        invoke_raw_typed!(
            hash,
                boat
        )
    }
}

/// ```
Example usage  
VEHICLE::GET_CLOSEST_VEHICLE(x, y, z, radius, hash, unknown leave at 70)   
x, y, z: Position to get closest vehicle to.  
radius: Max radius to get a vehicle.  
modelHash: Limit to vehicles with this model. 0 for any.  
flags: The bitwise flags altering the function's behaviour.  
Does not return police cars or helicopters.  
It seems to return police cars for me, does not seem to return helicopters, planes or boats for some reason  
Only returns non police cars and motorbikes with the flag set to 70 and modelHash to 0. ModelHash seems to always be 0 when not a modelHash in the scripts, as stated above.   
These flags were found in the b617d scripts: 0,2,4,6,7,23,127,260,2146,2175,12294,16384,16386,20503,32768,67590,67711,98309,100359.  
Converted to binary, each bit probably represents a flag as explained regarding another native here: gtaforums.com/topic/822314-guide-driving-styles  
Conversion of found flags to binary: pastebin.com/kghNFkRi  
At exactly 16384 which is 0100000000000000 in binary and 4000 in hexadecimal only planes are returned.   
It's probably more convenient to use worldGetAllVehicles(int *arr, int arrSize) and check the shortest distance yourself and sort if you want by checking the vehicle type with for example VEHICLE::IS_THIS_MODEL_A_BOAT



pub fn get_closest_vehicle_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            modelHash: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF73EB622C4F1689Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF73EB622C4F1689Bu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                modelHash, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_closest_vehicle_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        modelHash: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF73EB622C4F1689Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF73EB622C4F1689Bu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                modelHash, 
                flags
        )
    }
}

/// ```
NativeDB Introduced: v3407
```

Prevents the plane from exploding when taking body damage if the inflictor is an AI-controlled vehicle. Only works for planes.



pub fn _set_disable_explode_from_body_damage_received_by_ai_vehicle_safe(
        
        
            plane: 
        , 
        
        
            disable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB0B7DF5CB876FF5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB0B7DF5CB876FF5Eu64;
        
        let result = invoke_raw!(
            hash,
                plane, 
                disable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_disable_explode_from_body_damage_received_by_ai_vehicle_raw(
        plane: , 
        disable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB0B7DF5CB876FF5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB0B7DF5CB876FF5Eu64;

        invoke_raw_typed!(
            hash,
                plane, 
                disable
        )
    }
}

/// ## Parameters
*



pub fn _get_vehicle_number_of_broken_bones_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C8CBFE1EA5FC631u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C8CBFE1EA5FC631u64;
        
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
pub fn _get_vehicle_number_of_broken_bones_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C8CBFE1EA5FC631u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C8CBFE1EA5FC631u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_mod_color_2_safe(
        
        
            vehicle: 
        , 
        
        
            paintType: 
        , 
        
        
            color: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x81592BE4E3878728u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x81592BE4E3878728u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                paintType, 
                color
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_mod_color_2_raw(
        vehicle: , 
        paintType: , 
        color: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x81592BE4E3878728u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x81592BE4E3878728u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                paintType, 
                color
        )
    }
}

/// This only works for planes.

Prevents a vehicle from exploding upon sustaining body damage from physical collisions. 

For helicopters, you might want to check [`SET_DISABLE_HELI_EXPLODE_FROM_BODY_DAMAGE`](#_0xEDBC8405B3895CC9) instead.

```
NativeDB Introduced: v1290
```



pub fn set_disable_explode_from_body_damage_on_collision_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26E13D440E7F6064u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26E13D440E7F6064u64;
        
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
pub fn set_disable_explode_from_body_damage_on_collision_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26E13D440E7F6064u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26E13D440E7F6064u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Only ever used once in decompiled scripts:



pub fn _is_vehicle_engine_on_fire_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC69ADF931AAE0C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC69ADF931AAE0C3u64;
        
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
pub fn _is_vehicle_engine_on_fire_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC69ADF931AAE0C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC69ADF931AAE0C3u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _set_vehicle_interior_color_safe(
        
        
            vehicle: 
        , 
        
        
            color: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF40DD601A65F7F19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF40DD601A65F7F19u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                color
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_interior_color_raw(
        vehicle: , 
        color: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF40DD601A65F7F19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF40DD601A65F7F19u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                color
        )
    }
}

/// ## Parameters
*



pub fn set_lights_cutoff_distance_tweak_safe(
        
        
            distance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC3CCA5844452B06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC3CCA5844452B06u64;
        
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
pub fn set_lights_cutoff_distance_tweak_raw(
        distance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC3CCA5844452B06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC3CCA5844452B06u64;

        invoke_raw_typed!(
            hash,
                distance
        )
    }
}

/// Identical to SET_PLAYBACK_TO_USE_AI_TRY_TO_REVERT_BACK_LATER with 0 as arguments for p1 and p3.



pub fn set_playback_to_use_ai_safe(
        
        
            vehicle: 
        , 
        
        
            drivingStyle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA549C3B37EA28131u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA549C3B37EA28131u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                drivingStyle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_playback_to_use_ai_raw(
        vehicle: , 
        drivingStyle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA549C3B37EA28131u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA549C3B37EA28131u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                drivingStyle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_tank_turret_position_safe(
        
        
            vehicle: 
        , 
        
        
            position: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56B94C6D7127DFBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56B94C6D7127DFBAu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                position, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_tank_turret_position_raw(
        vehicle: , 
        position: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56B94C6D7127DFBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56B94C6D7127DFBAu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                position, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_extra_turned_on_safe(
        
        
            vehicle: 
        , 
        
        
            extraId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2E6822DBFD6C8BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2E6822DBFD6C8BDu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                extraId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_extra_turned_on_raw(
        vehicle: , 
        extraId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2E6822DBFD6C8BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2E6822DBFD6C8BDu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                extraId
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_extra_colours_safe(
        
        
            vehicle: 
        , 
        
        
            pearlescentColor: 
        , 
        
        
            wheelColor: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BC4245933A166F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BC4245933A166F7u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                pearlescentColor, 
                wheelColor
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_extra_colours_raw(
        vehicle: , 
        pearlescentColor: , 
        wheelColor: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BC4245933A166F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BC4245933A166F7u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                pearlescentColor, 
                wheelColor
        )
    }
}

/// See [REQUEST_VEHICLE_RECORDING](#_0xAF514CABE74CBF15)



pub fn remove_vehicle_recording_safe(
        
        
            recording: 
        , 
        
        
            script: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1160ACCF98A3FC8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1160ACCF98A3FC8u64;
        
        let result = invoke_raw!(
            hash,
                recording, 
                script
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_vehicle_recording_raw(
        recording: , 
        script: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1160ACCF98A3FC8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1160ACCF98A3FC8u64;

        invoke_raw_typed!(
            hash,
                recording, 
                script
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_stolen_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AF9BD80EEBEB453u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AF9BD80EEBEB453u64;
        
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
pub fn is_vehicle_stolen_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AF9BD80EEBEB453u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AF9BD80EEBEB453u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _0x65b080555ea48149_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65B080555EA48149u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65B080555EA48149u64;
        
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
pub fn _0x65b080555ea48149_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65B080555EA48149u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65B080555EA48149u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Detaches the specified entity currently being carried by a Cargobob.



pub fn detach_entity_from_cargobob_safe(
        
        
            vehicle: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF03011701811146u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF03011701811146u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn detach_entity_from_cargobob_raw(
        vehicle: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF03011701811146u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF03011701811146u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                entity
        )
    }
}

/// ## Parameters
*



pub fn _0xed5ede9e676643c9_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED5EDE9E676643C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED5EDE9E676643C9u64;
        
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
pub fn _0xed5ede9e676643c9_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED5EDE9E676643C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED5EDE9E676643C9u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Sets the arm position of a bulldozer. Position must be a value between 0.0 and 1.0. Ignored when `p2` is set to false, instead incrementing arm position by 0.1 (or 10%).



pub fn set_vehicle_bulldozer_arm_position_safe(
        
        
            vehicle: 
        , 
        
        
            position: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8EBCCC96ADB9FB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8EBCCC96ADB9FB7u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                position, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_bulldozer_arm_position_raw(
        vehicle: , 
        position: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8EBCCC96ADB9FB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8EBCCC96ADB9FB7u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                position, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn open_bomb_bay_doors_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87E7F24270732CB1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87E7F24270732CB1u64;
        
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
pub fn open_bomb_bay_doors_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87E7F24270732CB1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87E7F24270732CB1u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_cargobob_pickup_magnet_pull_strength_safe(
        
        
            cargobob: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED8286F71A819BAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED8286F71A819BAAu64;
        
        let result = invoke_raw!(
            hash,
                cargobob, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cargobob_pickup_magnet_pull_strength_raw(
        cargobob: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED8286F71A819BAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED8286F71A819BAAu64;

        invoke_raw_typed!(
            hash,
                cargobob, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x41290b40fa63e6da_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41290B40FA63E6DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41290B40FA63E6DAu64;
        
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
pub fn _0x41290b40fa63e6da_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41290B40FA63E6DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41290B40FA63E6DAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Returns `nMonetaryValue` from handling.meta for specific model, which is the vehicle's monetary value.
```



pub fn get_vehicle_model_value_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5873C14A52D74236u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5873C14A52D74236u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_model_value_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5873C14A52D74236u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5873C14A52D74236u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _get_hydraulic_wheel_value_safe(
        
        
            vehicle: 
        , 
        
        
            wheelId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BB5CBDDD0F25AE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BB5CBDDD0F25AE3u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                wheelId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_hydraulic_wheel_value_raw(
        vehicle: , 
        wheelId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BB5CBDDD0F25AE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BB5CBDDD0F25AE3u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                wheelId
        )
    }
}

/// See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).



pub fn remove_vehicle_window_safe(
        
        
            vehicle: 
        , 
        
        
            windowIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA711568EEDB43069u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA711568EEDB43069u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                windowIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_vehicle_window_raw(
        vehicle: , 
        windowIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA711568EEDB43069u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA711568EEDB43069u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                windowIndex
        )
    }
}

/// ```
NativeDB Introduced: v2060
```



pub fn _get_tyre_wear_multiplier_safe(
        
        
            vehicle: 
        , 
        
        
            wheelIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E387895952F4F71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E387895952F4F71u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                wheelIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_tyre_wear_multiplier_raw(
        vehicle: , 
        wheelIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E387895952F4F71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E387895952F4F71u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                wheelIndex
        )
    }
}

/// ## Parameters
*



pub fn does_vehicle_have_weapons_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25ECB9F8017D98E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25ECB9F8017D98E0u64;
        
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
pub fn does_vehicle_have_weapons_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25ECB9F8017D98E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25ECB9F8017D98E0u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_lock_on_target_safe(
        
        
            vehicle: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F5EBAB1F260CFCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F5EBAB1F260CFCEu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_lock_on_target_raw(
        vehicle: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F5EBAB1F260CFCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F5EBAB1F260CFCEu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                entity
        )
    }
}

/// ## Parameters
*



pub fn _0xa247f9ef01d8082e_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA247F9EF01D8082Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA247F9EF01D8082Eu64;
        
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
pub fn _0xa247f9ef01d8082e_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA247F9EF01D8082Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA247F9EF01D8082Eu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn get_boat_boom_position_ratio_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6636C535F6CC2725u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6636C535F6CC2725u64;
        
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
pub fn get_boat_boom_position_ratio_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6636C535F6CC2725u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6636C535F6CC2725u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// This native it's a debug native. Won't do anything.



pub fn allow_ambient_vehicles_to_avoid_adverse_conditions_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB264C4D2F2B0A78Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB264C4D2F2B0A78Bu64;
        
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
pub fn allow_ambient_vehicles_to_avoid_adverse_conditions_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB264C4D2F2B0A78Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB264C4D2F2B0A78Bu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _0xd3301660a57c9272_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3301660A57C9272u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3301660A57C9272u64;
        
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
pub fn _0xd3301660a57c9272_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3301660A57C9272u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3301660A57C9272u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Sets some health value. Looks like it's used for helis.
```



pub fn _0x5ee5632f47ae9695_safe(
        
        
            vehicle: 
        , 
        
        
            health: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EE5632F47AE9695u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EE5632F47AE9695u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                health
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5ee5632f47ae9695_raw(
        vehicle: , 
        health: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EE5632F47AE9695u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EE5632F47AE9695u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                health
        )
    }
}

/// Returns whether the vehicle's lights and sirens are on.



pub fn is_vehicle_siren_on_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C9BF537BE2634B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C9BF537BE2634B2u64;
        
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
pub fn is_vehicle_siren_on_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C9BF537BE2634B2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C9BF537BE2634B2u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```c
enum eVehicleWheels
{
	WHEEL_LF = 0, // Vehicle Left front
	WHEEL_RF = 1, // Vehicle Right front
	WHEEL_LM = 2, // Vehicle Left middle
	WHEEL_RM = 3, // Vehicle Right middle
	WHEEL_LR = 4, // Vehicle Left rear
	WHEEL_RR = 5, // Vehicle Right rear
	WHEEL_BF = 6, // Bike front
	WHEEL_BR = 7, // Bike rear
	MAX_WHEELS = 8
};
```



pub fn is_vehicle_tyre_burst_safe(
        
        
            vehicle: 
        , 
        
        
            wheelID: 
        , 
        
        
            isBurstToRim: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA291848A0815CA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA291848A0815CA9u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                wheelID, 
                isBurstToRim
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_tyre_burst_raw(
        vehicle: , 
        wheelID: , 
        isBurstToRim: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA291848A0815CA9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA291848A0815CA9u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                wheelID, 
                isBurstToRim
        )
    }
}

/// ## Parameters
*



pub fn remove_vehicles_from_generators_in_area_safe(
        
        
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
        
        
            unk: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46A1E1A299EC4BBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46A1E1A299EC4BBAu64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                unk
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_vehicles_from_generators_in_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        unk: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46A1E1A299EC4BBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46A1E1A299EC4BBAu64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                unk
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_interiorlight_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC2042F090AF6AD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC2042F090AF6AD3u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_interiorlight_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC2042F090AF6AD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC2042F090AF6AD3u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Specifies an area of interest where cargens will focus on spawning vehicles

You can clear the area of interest with [`CLEAR_VEHICLE_GENERATOR_AREA_OF_INTEREST`](#_0x0A436B8643716D14)



pub fn set_vehicle_generator_area_of_interest_safe(
        
        
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
        let hash = 0x9A75585FB2E54FADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A75585FB2E54FADu64;
        
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
pub fn set_vehicle_generator_area_of_interest_raw(
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A75585FB2E54FADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A75585FB2E54FADu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_door_lock_status_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25BC98A59C2EA962u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25BC98A59C2EA962u64;
        
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
pub fn get_vehicle_door_lock_status_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25BC98A59C2EA962u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25BC98A59C2EA962u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
"To burst tyres VEHICLE::SET_VEHICLE_TYRE_BURST(vehicle, 0, true, 1000.0)  
to burst all tyres type it 8 times where p1 = 0 to 7.  
p3 seems to be how much damage it has taken. 0 doesn't deflate them, 1000 completely deflates them.  
'0 = wheel_lf / bike, plane or jet front  
'1 = wheel_rf  
'2 = wheel_lm / in 6 wheels trailer, plane or jet is first one on left  
'3 = wheel_rm / in 6 wheels trailer, plane or jet is first one on right  
'4 = wheel_lr / bike rear / in 6 wheels trailer, plane or jet is last one on left  
'5 = wheel_rr / in 6 wheels trailer, plane or jet is last one on right  
'45 = 6 wheels trailer mid wheel left  
'47 = 6 wheels trailer mid wheel right  
```



pub fn set_vehicle_tyre_burst_safe(
        
        
            vehicle: 
        , 
        
        
            index: 
        , 
        
        
            onRim: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC6A202EE4960385u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC6A202EE4960385u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                index, 
                onRim, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_tyre_burst_raw(
        vehicle: , 
        index: , 
        onRim: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC6A202EE4960385u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC6A202EE4960385u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                index, 
                onRim, 
                p3
        )
    }
}

/// Detaches the vehicle's windscreen.



pub fn pop_out_vehicle_windscreen_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D645D59FB5F5AD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D645D59FB5F5AD3u64;
        
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
pub fn pop_out_vehicle_windscreen_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D645D59FB5F5AD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D645D59FB5F5AD3u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
NativeDB Introduced: 3095
```

Activates or deactivates the nitrous system in the specified vehicle, based on the boolean value provided.
You can clear the nitrous with [`CLEAR_NITROUS`](#_0xC889AE921400E1ED), if you want to have more control on the nitrous and those settings, use [`SET_OVERRIDE_NITROUS_LEVEL`](#_0xC8E9B6B71B8E660D)



pub fn set_nitrous_is_active_safe(
        
        
            vehicle: 
        , 
        
        
            isActive: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x465EEA70AF251045u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x465EEA70AF251045u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                isActive
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_nitrous_is_active_raw(
        vehicle: , 
        isActive: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x465EEA70AF251045u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x465EEA70AF251045u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                isActive
        )
    }
}

/// ```
Gets a random vehicle in a sphere at the specified position, of the specified radius.  
x: The X-component of the position of the sphere.  
y: The Y-component of the position of the sphere.  
z: The Z-component of the position of the sphere.  
radius: The radius of the sphere. Max is 9999.9004.  
modelHash: The vehicle model to limit the selection to. Pass 0 for any model.  
flags: The bitwise flags that modifies the behaviour of this function.  
```



pub fn get_random_vehicle_in_sphere_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            modelHash: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x386F6CE5BAF6091Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x386F6CE5BAF6091Cu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                modelHash, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_random_vehicle_in_sphere_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        modelHash: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x386F6CE5BAF6091Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x386F6CE5BAF6091Cu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                modelHash, 
                flags
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_doors_locked_for_all_players_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2F80B8D040727CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2F80B8D040727CCu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_doors_locked_for_all_players_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2F80B8D040727CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2F80B8D040727CCu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_kers_allowed_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99C82F8A139F3E4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99C82F8A139F3E4Eu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_kers_allowed_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99C82F8A139F3E4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99C82F8A139F3E4Eu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0xaa653ae61924b0a0_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA653AE61924B0A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA653AE61924B0A0u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xaa653ae61924b0a0_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA653AE61924B0A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA653AE61924B0A0u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Allows locking the hover/non-hover mode of a vehicle, such as the flying mode of the `Deluxo`. In the decompiled scripts, this native is used on `oppressor2` but couldn't get it to work on it.



pub fn set_special_flight_mode_allowed_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1211889DF15A763u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1211889DF15A763u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_special_flight_mode_allowed_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF1211889DF15A763u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF1211889DF15A763u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)



pub fn _get_is_door_valid_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x645F4B6E8499F632u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x645F4B6E8499F632u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_is_door_valid_raw(
        vehicle: , 
        doorIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x645F4B6E8499F632u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x645F4B6E8499F632u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_brake_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4E2FD323574965Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4E2FD323574965Cu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_brake_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4E2FD323574965Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4E2FD323574965Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_can_be_used_by_fleeing_peds_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x300504B23BD3B711u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x300504B23BD3B711u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_can_be_used_by_fleeing_peds_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x300504B23BD3B711u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x300504B23BD3B711u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_plane_landing_gear_intact_safe(
        
        
            plane: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4198AB0022B15F87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4198AB0022B15F87u64;
        
        let result = invoke_raw!(
            hash,
                plane
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_plane_landing_gear_intact_raw(
        plane: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4198AB0022B15F87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4198AB0022B15F87u64;

        invoke_raw_typed!(
            hash,
                plane
        )
    }
}

/// Enable/Disables global slipstream physics



pub fn set_enable_vehicle_slipstreaming_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6C0C80B8C867537u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6C0C80B8C867537u64;
        
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
pub fn set_enable_vehicle_slipstreaming_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6C0C80B8C867537u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6C0C80B8C867537u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Retrieves the manufacturer's name for a specified vehicle.

```
NativeDB Introduced: v1868
```



pub fn get_make_name_from_vehicle_model_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7AF4F159FF99F97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7AF4F159FF99F97u64;
        
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
pub fn get_make_name_from_vehicle_model_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7AF4F159FF99F97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7AF4F159FF99F97u64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// ## Parameters
*



pub fn is_this_model_a_bike_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB50C0B0CEDC6CE84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB50C0B0CEDC6CE84u64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_this_model_a_bike_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB50C0B0CEDC6CE84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB50C0B0CEDC6CE84u64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// Gets the ped in the specified seat of the passed vehicle.

If there is no ped in the seat, and the game considers the vehicle as ambient population, this will create a random occupant ped in the seat, which may be cleaned up by the game fairly soon if not marked as script-owned mission entity.



pub fn get_ped_in_vehicle_seat_safe(
        
        
            vehicle: 
        , 
        
        
            seatIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB40DD2270B65366u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB40DD2270B65366u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                seatIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_in_vehicle_seat_raw(
        vehicle: , 
        seatIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBB40DD2270B65366u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBB40DD2270B65366u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                seatIndex
        )
    }
}

/// ## Parameters
*



pub fn set_all_vehicle_generators_active_in_area_safe(
        
        
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
        
        
            p6: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC12321827687FE4Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC12321827687FE4Du64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                p6, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_all_vehicle_generators_active_in_area_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        p6: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC12321827687FE4Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC12321827687FE4Du64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                p6, 
                p7
        )
    }
}

/// ## Parameters
*



pub fn get_random_vehicle_back_bumper_in_sphere_safe(
        
        
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
        let hash = 0xB50807EABE20A8DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB50807EABE20A8DCu64;
        
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
pub fn get_random_vehicle_back_bumper_in_sphere_raw(
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
        let hash = 0xB50807EABE20A8DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB50807EABE20A8DCu64;

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

/// Checks if a Submarine has any air leaks, when there is more than 4 the player will drown.

```
NativeDB Introduced: v2189
```



pub fn get_submarine_number_of_air_leaks_safe(
        
        
            submarine: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x093D6DDCA5B8FBAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x093D6DDCA5B8FBAEu64;
        
        let result = invoke_raw!(
            hash,
                submarine
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_submarine_number_of_air_leaks_raw(
        submarine: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x093D6DDCA5B8FBAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x093D6DDCA5B8FBAEu64;

        invoke_raw_typed!(
            hash,
                submarine
        )
    }
}

/// ```
in script hook .net   
Vehicle v = ...;  
Function.Call(Hash.TRACK_VEHICLE_VISIBILITY, v.Handle);  
```



pub fn track_vehicle_visibility_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x64473AEFDCF47DCAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x64473AEFDCF47DCAu64;
        
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
pub fn track_vehicle_visibility_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x64473AEFDCF47DCAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x64473AEFDCF47DCAu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
REQUEST_VEHICLE_ASSET(GET_HASH_KEY(cargobob3), 3);  
vehicle found that have asset's:  
cargobob3  
submersible  
blazer  
```



pub fn request_vehicle_asset_safe(
        
        
            vehicleHash: 
        , 
        
        
            vehicleAsset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x81A15811460FAB3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x81A15811460FAB3Au64;
        
        let result = invoke_raw!(
            hash,
                vehicleHash, 
                vehicleAsset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_vehicle_asset_raw(
        vehicleHash: , 
        vehicleAsset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x81A15811460FAB3Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x81A15811460FAB3Au64;

        invoke_raw_typed!(
            hash,
                vehicleHash, 
                vehicleAsset
        )
    }
}

/// ```c
enum eVehiclePlateIndicies {
	SanAndreasCursive = 0,
	SanAndreasBlack = 1,
	SanAndreasBlue = 2,
	SanAndreasPlain = 3,
	SRExcept = 4,
	NorthYankton = 5,
	// All indicies below this require b3095
	ECola = 6,
	LasVenturas = 7,
	LiberyCity = 8,
	LSCarMeet = 9,
	LSPanic = 10,
	LSPounders = 11,
	Sprunk = 12,
}
```



pub fn get_vehicle_number_plate_text_index_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF11BC2DD9A3E7195u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF11BC2DD9A3E7195u64;
        
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
pub fn get_vehicle_number_plate_text_index_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF11BC2DD9A3E7195u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF11BC2DD9A3E7195u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Actually number of color combinations  
```



pub fn get_number_of_vehicle_colours_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B963160CD65D41Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B963160CD65D41Eu64;
        
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
pub fn get_number_of_vehicle_colours_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B963160CD65D41Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B963160CD65D41Eu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn detach_vehicle_from_any_cargobob_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADF7BE450512C12Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADF7BE450512C12Fu64;
        
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
pub fn detach_vehicle_from_any_cargobob_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADF7BE450512C12Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADF7BE450512C12Fu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Makes the vehicle accept no passengers.  
```



pub fn set_vehicle_allow_no_passengers_lockon_safe(
        
        
            veh: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D14D4154BFE7B2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D14D4154BFE7B2Cu64;
        
        let result = invoke_raw!(
            hash,
                veh, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_allow_no_passengers_lockon_raw(
        veh: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D14D4154BFE7B2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D14D4154BFE7B2Cu64;

        invoke_raw_typed!(
            hash,
                veh, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _get_entity_attached_to_cargobob_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99093F60746708CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99093F60746708CAu64;
        
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
pub fn _get_entity_attached_to_cargobob_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99093F60746708CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99093F60746708CAu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Returns max traction of the specified vehicle model.
```



pub fn get_vehicle_model_max_traction_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x539DE94D44FDFD0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x539DE94D44FDFD0Du64;
        
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
pub fn get_vehicle_model_max_traction_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x539DE94D44FDFD0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x539DE94D44FDFD0Du64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// Disables wings for `Deluxo` and `Oppressor MK II`. For the Deluxo, it retracts the wings immediately, preventing flight. For the Oppressor Mk II, the wings retract after landing and take-off is not possible, though it can still glide if launched into the air.



pub fn set_disable_hover_mode_flight_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D55FE374D5FDB91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D55FE374D5FDB91u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_disable_hover_mode_flight_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D55FE374D5FDB91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D55FE374D5FDB91u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// R* used it to "remove" vehicle windows when "nightshark" had some mod, which adding some kind of armored windows. When enabled, you can't break vehicles glass. All your bullets wiil shoot through glass. You also will not able to break the glass with any other way (hitting and etc)



pub fn _set_disable_vehicle_window_collisions_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1087BC8EC540DAEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1087BC8EC540DAEBu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_disable_vehicle_window_collisions_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1087BC8EC540DAEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1087BC8EC540DAEBu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)



pub fn get_ped_using_vehicle_door_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x218297BF0CFD853Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x218297BF0CFD853Bu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_using_vehicle_door_raw(
        vehicle: , 
        doorIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x218297BF0CFD853Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x218297BF0CFD853Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex
        )
    }
}

/// ## Parameters
*



pub fn _get_vehicle_has_parachute_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC9CFF381338CB4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC9CFF381338CB4Fu64;
        
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
pub fn _get_vehicle_has_parachute_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC9CFF381338CB4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC9CFF381338CB4Fu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// For FiveM, use [`GET_GAME_POOL`](#_0x2B9D4F50).



pub fn get_all_vehicles_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B8E1BF04B51F2E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B8E1BF04B51F2E8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_all_vehicles_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B8E1BF04B51F2E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B8E1BF04B51F2E8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```c
enum eWindowId {
	VEH_EXT_WINDOW_LF = 0,
	VEH_EXT_WINDOW_RF = 1,
	VEH_EXT_WINDOW_LR = 2,
	VEH_EXT_WINDOW_RR = 3,
	VEH_EXT_WINDOW_LM = 4,
	VEH_EXT_WINDOW_RM = 5,
	VEH_EXT_WINDSCREEN = 6,
	VEH_EXT_WINDSCREEN_R = 7,
}
```



pub fn is_vehicle_window_intact_safe(
        
        
            vehicle: 
        , 
        
        
            windowIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46E571A0E20D01F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46E571A0E20D01F1u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                windowIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_window_intact_raw(
        vehicle: , 
        windowIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46E571A0E20D01F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46E571A0E20D01F1u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                windowIndex
        )
    }
}

/// Despite its name, this works on Helicopters and Planes.

Sets the speed of the helicopter blades in percentage of the full speed.



pub fn set_heli_blades_speed_safe(
        
        
            vehicle: 
        , 
        
        
            speed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD280B4D7F3ABC4Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD280B4D7F3ABC4Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                speed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_heli_blades_speed_raw(
        vehicle: , 
        speed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD280B4D7F3ABC4Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD280B4D7F3ABC4Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                speed
        )
    }
}

/// Often used in conjunction with: [SET_VEHICLE_REDUCE_GRIP](#_0x222FF6A823D122E2).

```
NativeDB Introduced: v1604
```



pub fn _set_vehicle_reduce_traction_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6DEE944E1EE90CFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6DEE944E1EE90CFBu64;
        
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
pub fn _set_vehicle_reduce_traction_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6DEE944E1EE90CFBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6DEE944E1EE90CFBu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Returns 1000.0 if the function is unable to get the address of the specified vehicle or if it's not a vehicle.  
Minimum: -4000  
Maximum: 1000  
-4000: Engine is destroyed  
0 and below: Engine catches fire and health rapidly declines  
300: Engine is smoking and losing functionality  
1000: Engine is perfect  
```



pub fn get_vehicle_engine_health_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC45D23BAF168AAB8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC45D23BAF168AAB8u64;
        
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
pub fn get_vehicle_engine_health_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC45D23BAF168AAB8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC45D23BAF168AAB8u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Returns index of the current vehicle's rooftop livery.
A getter for [_SET_VEHICLE_ROOF_LIVERY](#_0xA6D3A8750DC73270).



pub fn _get_vehicle_roof_livery_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60190048C0764A26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60190048C0764A26u64;
        
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
pub fn _get_vehicle_roof_livery_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60190048C0764A26u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60190048C0764A26u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_can_be_visibly_damaged_safe(
        
        
            vehicle: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C7028F78FFD3681u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C7028F78FFD3681u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_can_be_visibly_damaged_raw(
        vehicle: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C7028F78FFD3681u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C7028F78FFD3681u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                state
        )
    }
}

/// ## Parameters
*



pub fn _set_vehicle_ramp_upwards_launch_motion_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x756AE6E962168A04u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x756AE6E962168A04u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_ramp_upwards_launch_motion_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x756AE6E962168A04u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x756AE6E962168A04u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
Gets the height of the vehicle's suspension.  
The higher the value the lower the suspension. Each 0.002 corresponds with one more level lowered.  
0.000 is the stock suspension.  
0.008 is Ultra Suspension.  
```



pub fn _get_vehicle_suspension_height_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53952FD2BAA19F17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53952FD2BAA19F17u64;
        
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
pub fn _get_vehicle_suspension_height_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53952FD2BAA19F17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53952FD2BAA19F17u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Activate siren on vehicle (Only works if the vehicle has a siren).  
```



pub fn set_vehicle_siren_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4924635A19EB37Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4924635A19EB37Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_siren_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4924635A19EB37Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4924635A19EB37Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Raises the roof on a convertible vehicle, utilizing any available animations for the action. This native is particularly useful for creating a realistic interaction with convertible vehicles by animating the process of raising the roof.

You can check if the vehicle has an convertible roof using [`IS_VEHICLE_A_CONVERTIBLE`](#_0x52F357A30698BCCE).

To lower the convertible roof, you can use [`LOWER_CONVERTIBLE_ROOF`](#_0xDED51F703D0FA83D).

```
NativeDB Introduced: v323
```



pub fn raise_convertible_roof_safe(
        
        
            vehicle: 
        , 
        
        
            instantlyRaise: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F5FB35D7E88FC70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F5FB35D7E88FC70u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                instantlyRaise
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn raise_convertible_roof_raw(
        vehicle: , 
        instantlyRaise: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F5FB35D7E88FC70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F5FB35D7E88FC70u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                instantlyRaise
        )
    }
}

/// ```
Inverse of 0x95CF53B3D687F9FA
```

```
NativeDB Added Parameter 1: Vehicle vehicle
```



pub fn _set_trailer_legs_lowered_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x878C75C09FBDB942u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x878C75C09FBDB942u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_trailer_legs_lowered_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x878C75C09FBDB942u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x878C75C09FBDB942u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_can_leak_petrol_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x192547247864DFDDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x192547247864DFDDu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_can_leak_petrol_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x192547247864DFDDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x192547247864DFDDu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x88bc673ca9e0ae99_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88BC673CA9E0AE99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88BC673CA9E0AE99u64;
        
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
pub fn _0x88bc673ca9e0ae99_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88BC673CA9E0AE99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88BC673CA9E0AE99u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x6eaaefc76acc311f_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6EAAEFC76ACC311Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6EAAEFC76ACC311Fu64;
        
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
pub fn _0x6eaaefc76acc311f_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6EAAEFC76ACC311Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6EAAEFC76ACC311Fu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Returns attached vehicle (Vehicle in parameter must be cargobob)  
```



pub fn get_vehicle_attached_to_cargobob_safe(
        
        
            cargobob: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x873B82D42AC2B9E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x873B82D42AC2B9E5u64;
        
        let result = invoke_raw!(
            hash,
                cargobob
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_attached_to_cargobob_raw(
        cargobob: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x873B82D42AC2B9E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x873B82D42AC2B9E5u64;

        invoke_raw_typed!(
            hash,
                cargobob
        )
    }
}

/// ## Parameters
*



pub fn _get_vehicle_interior_color_safe(
        
        
            vehicle: 
        , 
        
        
            color: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D1464D472D32136u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D1464D472D32136u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                color
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_vehicle_interior_color_raw(
        vehicle: , 
        color: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D1464D472D32136u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D1464D472D32136u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                color
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_disable_towing_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B6747FAA9DB9D6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B6747FAA9DB9D6Bu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_disable_towing_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B6747FAA9DB9D6Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B6747FAA9DB9D6Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0x8f0d5ba1c2cc91d7_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F0D5BA1C2CC91D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F0D5BA1C2CC91D7u64;
        
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
pub fn _0x8f0d5ba1c2cc91d7_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F0D5BA1C2CC91D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F0D5BA1C2CC91D7u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Only used with the "akula" and "annihilator2" in the decompiled native scripts.

```
NativeDB Introduced: v1290
```



pub fn _set_deploy_heli_stub_wings_safe(
        
        
            vehicle: 
        , 
        
        
            deploy: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB251E0B33E58B424u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB251E0B33E58B424u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                deploy, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_deploy_heli_stub_wings_raw(
        vehicle: , 
        deploy: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB251E0B33E58B424u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB251E0B33E58B424u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                deploy, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_is_stolen_safe(
        
        
            vehicle: 
        , 
        
        
            isStolen: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67B2C79AA7FF5738u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67B2C79AA7FF5738u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                isStolen
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_is_stolen_raw(
        vehicle: , 
        isStolen: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67B2C79AA7FF5738u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67B2C79AA7FF5738u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                isStolen
        )
    }
}

/// ```
REQUEST_VEHICLE_*  
```



pub fn _request_vehicle_dashboard_scaleform_movie_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBA3C090E3D74690u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBA3C090E3D74690u64;
        
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
pub fn _request_vehicle_dashboard_scaleform_movie_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBA3C090E3D74690u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBA3C090E3D74690u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Checks the angle of the door mapped from 0.0 - 1.0 where 0.0 is fully closed and 1.0 is fully open.

See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)



pub fn get_vehicle_door_angle_ratio_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE3F9C29F7B32BD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE3F9C29F7B32BD5u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_door_angle_ratio_raw(
        vehicle: , 
        doorIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE3F9C29F7B32BD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE3F9C29F7B32BD5u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex
        )
    }
}

/// Sets the vehicle lights state. Allowing for different lighting modes.

```
NativeDB Introduced: v323
```

```c
enum eVehicleLightSetting {
    // Normal light behavior. Lights cycle through off, then low beams, then high beams.
    // Note: It's affected by day or night; high beams don't exist in daytime.
    NO_VEHICLE_LIGHT_OVERRIDE = 0,
    // Vehicle doesn't have lights, always off.
    FORCE_VEHICLE_LIGHTS_OFF  = 1, 
    // Vehicle has always-on lights.
    // During day: Cycles between low beams and high beams. 
    // At night: Cycles between low beams, low beams, and high beams.
    FORCE_VEHICLE_LIGHTS_ON   = 2,
    // Sets vehicle lights on. Behaves like normal lights (same as 0).
    SET_VEHICLE_LIGHTS_ON     = 3,
    // Sets vehicle lights off. Behaves like normal lights (same as 0).
    SET_VEHICLE_LIGHTS_OFF    = 4 
};
```



pub fn set_vehicle_lights_safe(
        
        
            vehicle: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34E710FF01247C5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34E710FF01247C5Au64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_lights_raw(
        vehicle: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34E710FF01247C5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34E710FF01247C5Au64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                state
        )
    }
}

/// ## Parameters
*



pub fn get_mod_slot_name_safe(
        
        
            vehicle: 
        , 
        
        
            modType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51F0FEB9F6AE98C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51F0FEB9F6AE98C0u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                modType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_mod_slot_name_raw(
        vehicle: , 
        modType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51F0FEB9F6AE98C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51F0FEB9F6AE98C0u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                modType
        )
    }
}

/// ```
paintType:  
0: Normal  
1: Metallic  
2: Pearl  
3: Matte  
4: Metal  
5: Chrome  
color: number of the color.  
p3 seems to always be 0.  
```



pub fn set_vehicle_mod_color_1_safe(
        
        
            vehicle: 
        , 
        
        
            paintType: 
        , 
        
        
            color: 
        , 
        
        
            pearlescentColor: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43FEB945EE7F85B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43FEB945EE7F85B8u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                paintType, 
                color, 
                pearlescentColor
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_mod_color_1_raw(
        vehicle: , 
        paintType: , 
        color: , 
        pearlescentColor: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43FEB945EE7F85B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43FEB945EE7F85B8u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                paintType, 
                color, 
                pearlescentColor
        )
    }
}

/// ```
p1 is always 0 in the scripts.  
p1 = check if vehicle is on fire  
```



pub fn is_vehicle_driveable_safe(
        
        
            vehicle: 
        , 
        
        
            isOnFireCheck: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C241E39B23DF959u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C241E39B23DF959u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                isOnFireCheck
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_driveable_raw(
        vehicle: , 
        isOnFireCheck: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C241E39B23DF959u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C241E39B23DF959u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                isOnFireCheck
        )
    }
}

/// ## Parameters
*



pub fn _0x80e3357fdef45c21_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80E3357FDEF45C21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80E3357FDEF45C21u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x80e3357fdef45c21_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80E3357FDEF45C21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80E3357FDEF45C21u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn does_vehicle_exist_with_decorator_safe(
        
        
            decorator: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x956B409B984D9BF7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x956B409B984D9BF7u64;
        
        let result = invoke_raw!(
            hash,
                decorator
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_vehicle_exist_with_decorator_raw(
        decorator: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x956B409B984D9BF7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x956B409B984D9BF7u64;

        invoke_raw_typed!(
            hash,
                decorator
        )
    }
}

/// ## Parameters
*



pub fn set_disable_pretend_occupants_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25367DE49D64CF16u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25367DE49D64CF16u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_disable_pretend_occupants_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25367DE49D64CF16u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25367DE49D64CF16u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
HAS_*
```



pub fn _has_filled_vehicle_population_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x91D6DD290888CBABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x91D6DD290888CBABu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _has_filled_vehicle_population_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x91D6DD290888CBABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x91D6DD290888CBABu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_has_muted_sirens_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8050E0EB60CF274u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8050E0EB60CF274u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_has_muted_sirens_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD8050E0EB60CF274u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD8050E0EB60CF274u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x3441cad2f2231923_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3441CAD2F2231923u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3441CAD2F2231923u64;
        
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
pub fn _0x3441cad2f2231923_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3441CAD2F2231923u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3441CAD2F2231923u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_automatically_attaches_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BA6F76BC53A1493u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BA6F76BC53A1493u64;
        
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
pub fn set_vehicle_automatically_attaches_raw(
        vehicle: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BA6F76BC53A1493u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BA6F76BC53A1493u64;

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



pub fn set_boat_sinks_when_wrecked_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F719973E1445BA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F719973E1445BA2u64;
        
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
pub fn set_boat_sinks_when_wrecked_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F719973E1445BA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F719973E1445BA2u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Has something to do with trains. Always precedes SET_MISSION_TRAIN_AS_NO_LONGER_NEEDED.  
============================================  
May be true that it can be used with trains not sure, but not specifically for trains. Go find Xbox360 decompiled scripts and search for 'func_1333' in freemode.c it isn't used just for trains. Thanks for the info tho.  
Btw, func_1333 ends up calling this func which uses this native,  
void func_1338(int iParam0)//Position   
{  
	ENTITY::FREEZE_ENTITY_POSITION(iParam0, true);  
	ENTITY::SET_ENTITY_COLLISION(iParam0, false, 0);  
	ENTITY::SET_ENTITY_INVINCIBLE(iParam0, true);  
	VEHICLE::_0xDF594D8D(iParam0, true);  
}  
```



pub fn _set_vehicle_st_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CF38D529D7441D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CF38D529D7441D9u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_st_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1CF38D529D7441D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1CF38D529D7441D9u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Sets the amount of bombs that this vehicle has. As far as I know, this does _not_ impact vehicle weapons or the ammo of those weapons in any way, it is just a way to keep track of the amount of bombs in a specific plane. 

In decompiled scripts this is used to deduct from or add to the count whenever bombs are dropped or purchased/restocked. 

Use [`_GET_AIRCRAFT_BOMB_COUNT`](#_0xEA12BD130D7569A1) to get the amount of bombs on that vehicle.



pub fn _set_vehicle_bomb_count_safe(
        
        
            aircraft: 
        , 
        
        
            bombCount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4B2ED59DEB5D774u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4B2ED59DEB5D774u64;
        
        let result = invoke_raw!(
            hash,
                aircraft, 
                bombCount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_bomb_count_raw(
        aircraft: , 
        bombCount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4B2ED59DEB5D774u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4B2ED59DEB5D774u64;

        invoke_raw_typed!(
            hash,
                aircraft, 
                bombCount
        )
    }
}

/// ## Parameters
*



pub fn set_cargobob_pickup_magnet_strength_safe(
        
        
            cargobob: 
        , 
        
        
            strength: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCBFCD9D1DAC19E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCBFCD9D1DAC19E2u64;
        
        let result = invoke_raw!(
            hash,
                cargobob, 
                strength
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cargobob_pickup_magnet_strength_raw(
        cargobob: , 
        strength: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCBFCD9D1DAC19E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCBFCD9D1DAC19E2u64;

        invoke_raw_typed!(
            hash,
                cargobob, 
                strength
        )
    }
}

/// ## Parameters
*



pub fn get_rotation_of_vehicle_recording_at_time_safe(
        
        
            recording: 
        , 
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2058206FBE79A8ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2058206FBE79A8ADu64;
        
        let result = invoke_raw!(
            hash,
                recording, 
                time
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_rotation_of_vehicle_recording_at_time_raw(
        recording: , 
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2058206FBE79A8ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2058206FBE79A8ADu64;

        invoke_raw_typed!(
            hash,
                recording, 
                time
        )
    }
}

/// ```
-1 = no livery  
```



pub fn get_vehicle_livery_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BB9230590DA5E8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BB9230590DA5E8Au64;
        
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
pub fn get_vehicle_livery_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2BB9230590DA5E8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2BB9230590DA5E8Au64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn pause_playback_recorded_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x632A689BF42301B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x632A689BF42301B1u64;
        
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
pub fn pause_playback_recorded_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x632A689BF42301B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x632A689BF42301B1u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_cargobob_pickup_rope_damping_multiplier_safe(
        
        
            cargobob: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF1182F682F65307u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF1182F682F65307u64;
        
        let result = invoke_raw!(
            hash,
                cargobob
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cargobob_pickup_rope_damping_multiplier_raw(
        cargobob: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF1182F682F65307u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF1182F682F65307u64;

        invoke_raw_typed!(
            hash,
                cargobob
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_weapon_restricted_ammo_safe(
        
        
            vehicle: 
        , 
        
        
            weaponIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8181CE2F25CB9BB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8181CE2F25CB9BB7u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                weaponIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_weapon_restricted_ammo_raw(
        vehicle: , 
        weaponIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8181CE2F25CB9BB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8181CE2F25CB9BB7u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                weaponIndex
        )
    }
}

/// Returns the acceleration of the specified model.



pub fn get_vehicle_model_acceleration_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C044C5C84505B6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C044C5C84505B6Au64;
        
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
pub fn get_vehicle_model_acceleration_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C044C5C84505B6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C044C5C84505B6Au64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// ```
NativeDB Introduced: v3095
```

Enables or disables the use of the vehicle's horn button for activating the nitrous system.



pub fn _set_vehicle_use_horn_button_for_nitrous_safe(
        
        
            vehicle: 
        , 
        
        
            bToggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1980F68872CC2C3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1980F68872CC2C3Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                bToggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_use_horn_button_for_nitrous_raw(
        vehicle: , 
        bToggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1980F68872CC2C3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1980F68872CC2C3Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                bToggle
        )
    }
}

/// ```
Controls how fast the tires wear out.
Default values from Rockstar's Open Wheel Race JSON's:
"owrtss" (Soft): 2.2
"owrtsm" (Medium): 1.7
"owrtsh" (Hard): 1.2
Usable wheels:
0: wheel_lf
1: wheel_rf
2: wheel_lm1
3: wheel_rm1
4: wheel_lr
5: wheel_rr
```

```
NativeDB Introduced: v2060
```



pub fn _set_tyre_softness_multiplier_safe(
        
        
            vehicle: 
        , 
        
        
            wheelIndex: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x392183BB9EA57697u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x392183BB9EA57697u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                wheelIndex, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_tyre_softness_multiplier_raw(
        vehicle: , 
        wheelIndex: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x392183BB9EA57697u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x392183BB9EA57697u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                wheelIndex, 
                multiplier
        )
    }
}

/// ```
vehicle must be a plane
```



pub fn set_vehicle_uses_large_rear_ramp_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCAC66558B944DA67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCAC66558B944DA67u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_uses_large_rear_ramp_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCAC66558B944DA67u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCAC66558B944DA67u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Used to control train speed, can be used to start and stop its movement as well.



pub fn set_train_cruise_speed_safe(
        
        
            train: 
        , 
        
        
            speed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16469284DB8C62B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16469284DB8C62B5u64;
        
        let result = invoke_raw!(
            hash,
                train, 
                speed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_train_cruise_speed_raw(
        train: , 
        speed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16469284DB8C62B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16469284DB8C62B5u64;

        invoke_raw_typed!(
            hash,
                train, 
                speed
        )
    }
}

/// Remove the weird shadow applied by [_SET_VEHICLE_SHADOW_EFFECT](#_0x2A70BAE8883E4C81)



pub fn _remove_vehicle_shadow_effect_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF87D9F2301F7D206u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF87D9F2301F7D206u64;
        
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
pub fn _remove_vehicle_shadow_effect_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF87D9F2301F7D206u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF87D9F2301F7D206u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Returns whether this vehicle is currently disabled by an EMP mine.

NativeDB Introduced: v1604
```



pub fn _get_is_vehicle_emp_disabled_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0506ED94363AD905u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0506ED94363AD905u64;
        
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
pub fn _get_is_vehicle_emp_disabled_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0506ED94363AD905u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0506ED94363AD905u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn is_this_model_a_boat_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45A9187928F4B9E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45A9187928F4B9E3u64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_this_model_a_boat_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45A9187928F4B9E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45A9187928F4B9E3u64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// Sets the boat boom position for the `TR3` trailer.

Ratio value is between `0.0` and `1.0`, where `0.0` is 90 degrees to the left of the boat, and `1.0` is just slightly to the right/back of the boat.

To get the current boom position ratio, use [GET_BOAT_BOOM_POSITION_RATIO](#_0x6636C535F6CC2725).



pub fn _set_boat_boom_position_ratio_safe(
        
        
            vehicle: 
        , 
        
        
            ratio: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF488C566413B4232u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF488C566413B4232u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                ratio
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_boat_boom_position_ratio_raw(
        vehicle: , 
        ratio: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF488C566413B4232u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF488C566413B4232u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                ratio
        )
    }
}

/// ```
indices:  
0 = Left  
1 = Right  
2 = Front  
3 = Back  
```



pub fn _is_vehicle_neon_light_enabled_safe(
        
        
            vehicle: 
        , 
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C4B92553E4766A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C4B92553E4766A5u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_vehicle_neon_light_enabled_raw(
        vehicle: , 
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C4B92553E4766A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C4B92553E4766A5u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                index
        )
    }
}

/// ## Parameters
*



pub fn has_vehicle_recording_been_loaded_safe(
        
        
            recording: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x300D614A4C785FC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x300D614A4C785FC4u64;
        
        let result = invoke_raw!(
            hash,
                recording
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_vehicle_recording_been_loaded_raw(
        recording: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x300D614A4C785FC4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x300D614A4C785FC4u64;

        invoke_raw_typed!(
            hash,
                recording
        )
    }
}

/// Sets the vehicle headlight shadow flags.

```
NativeDB Introduced: v323
```

```c
enum eVehicleHeadlightShadowFlags {
    // Default (Lights can be toggled between off, normal and high beams)
    NO_HEADLIGHT_SHADOWS = 0,
    // Lights Disabled (Lights are fully disabled, cannot be toggled)
    HEADLIGHTS_CAST_DYNAMIC_SHADOWS = 1,
    // Always On (Lights can be toggled between normal and high beams)
    HEADLIGHTS_CAST_STATIC_SHADOWS = 2,
    HEADLIGHTS_CAST_FULL_SHADOWS = 3 
};
```



pub fn set_vehicle_headlight_shadows_safe(
        
        
            vehicle: 
        , 
        
        
            flag: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1FD09E7390A74D54u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1FD09E7390A74D54u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                flag
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_headlight_shadows_raw(
        vehicle: , 
        flag: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1FD09E7390A74D54u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1FD09E7390A74D54u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                flag
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_active_for_ped_navigation_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21115BCD6E44656Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21115BCD6E44656Au64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_active_for_ped_navigation_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21115BCD6E44656Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21115BCD6E44656Au64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_class_max_braking_safe(
        
        
            vehicleClass: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4BF54C16EC8FEC03u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4BF54C16EC8FEC03u64;
        
        let result = invoke_raw!(
            hash,
                vehicleClass
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_class_max_braking_raw(
        vehicleClass: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4BF54C16EC8FEC03u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4BF54C16EC8FEC03u64;

        invoke_raw_typed!(
            hash,
                vehicleClass
        )
    }
}

/// ```
Public Function isVehicleOnAllWheels(vh As Vehicle) As Boolean  
Return Native.Function.Call(Of Boolean)(Hash.IS_VEHICLE_ON_ALL_WHEELS, vh)  
		    End Function  
```



pub fn is_vehicle_on_all_wheels_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB104CD1BABF302E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB104CD1BABF302E2u64;
        
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
pub fn is_vehicle_on_all_wheels_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB104CD1BABF302E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB104CD1BABF302E2u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// This native does no interpolation between pathpoints. The same position will be returned for all times up to the next pathpoint in the recording.

See [`REQUEST_VEHICLE_RECORDING`](#_0xAF514CABE74CBF15).



pub fn get_position_of_vehicle_recording_at_time_safe(
        
        
            recording: 
        , 
        
        
            time: 
        , 
        
        
            script: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD242728AA6F0FBA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD242728AA6F0FBA2u64;
        
        let result = invoke_raw!(
            hash,
                recording, 
                time, 
                script
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_position_of_vehicle_recording_at_time_raw(
        recording: , 
        time: , 
        script: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD242728AA6F0FBA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD242728AA6F0FBA2u64;

        invoke_raw_typed!(
            hash,
                recording, 
                time, 
                script
        )
    }
}

/// ```
Usage:  
public bool isCopInRange(Vector3 Location, float Range)  
        {  
            return Function.Call<bool>(Hash.IS_COP_PED_IN_AREA_3D, Location.X - Range, Location.Y - Range, Location.Z - Range, Location.X + Range, Location.Y + Range, Location.Z + Range);  
        }  
```



pub fn is_cop_vehicle_in_area_3d_safe(
        
        
            x1: 
        , 
        
        
            x2: 
        , 
        
        
            y1: 
        , 
        
        
            y2: 
        , 
        
        
            z1: 
        , 
        
        
            z2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EEF65D5F153E26Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EEF65D5F153E26Au64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                x2, 
                y1, 
                y2, 
                z1, 
                z2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_cop_vehicle_in_area_3d_raw(
        x1: , 
        x2: , 
        y1: , 
        y2: , 
        z1: , 
        z2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EEF65D5F153E26Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EEF65D5F153E26Au64;

        invoke_raw_typed!(
            hash,
                x1, 
                x2, 
                y1, 
                y2, 
                z1, 
                z2
        )
    }
}

/// ## Parameters
*



pub fn set_task_vehicle_goto_plane_min_height_above_terrain_safe(
        
        
            plane: 
        , 
        
        
            height: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB893215D8D4C015Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB893215D8D4C015Bu64;
        
        let result = invoke_raw!(
            hash,
                plane, 
                height
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_task_vehicle_goto_plane_min_height_above_terrain_raw(
        plane: , 
        height: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB893215D8D4C015Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB893215D8D4C015Bu64;

        invoke_raw_typed!(
            hash,
                plane, 
                height
        )
    }
}

/// ## Parameters
*



pub fn set_train_speed_safe(
        
        
            train: 
        , 
        
        
            speed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA0BC91BE0B796E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA0BC91BE0B796E3u64;
        
        let result = invoke_raw!(
            hash,
                train, 
                speed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_train_speed_raw(
        train: , 
        speed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA0BC91BE0B796E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA0BC91BE0B796E3u64;

        invoke_raw_typed!(
            hash,
                train, 
                speed
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_use_player_light_settings_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC45C27EF50F36ADCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC45C27EF50F36ADCu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_use_player_light_settings_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC45C27EF50F36ADCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC45C27EF50F36ADCu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _set_cambered_wheels_disabled_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1201E8A3290A3B98u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1201E8A3290A3B98u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_cambered_wheels_disabled_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1201E8A3290A3B98u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1201E8A3290A3B98u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
Appears to return false if any window is broken.  
```



pub fn are_all_vehicle_windows_intact_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11D862A3E977A9EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11D862A3E977A9EFu64;
        
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
pub fn are_all_vehicle_windows_intact_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11D862A3E977A9EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11D862A3E977A9EFu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _0xd3e51c0ab8c26eee_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3E51C0AB8C26EEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3E51C0AB8C26EEEu64;
        
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
pub fn _0xd3e51c0ab8c26eee_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3E51C0AB8C26EEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3E51C0AB8C26EEEu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x35bb21de06784373_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35BB21DE06784373u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35BB21DE06784373u64;
        
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
pub fn _0x35bb21de06784373_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35BB21DE06784373u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35BB21DE06784373u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Only used in R* Script fm_content_cargo
```

```
NativeDB Introduced: v2699
```



pub fn _0xef9d388f8d377f44_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF9D388F8D377F44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF9D388F8D377F44u64;
        
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
pub fn _0xef9d388f8d377f44_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF9D388F8D377F44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF9D388F8D377F44u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
Only works on bikes, both X and Y work in the -1 - 1 range.
X forces the bike to turn left or right (-1, 1)
Y forces the bike to lean to the left or to the right (-1, 1)
Example with X -1/Y 1
http://i.imgur.com/TgIuAPJ.jpg
```



pub fn set_bike_on_stand_safe(
        
        
            vehicle: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CFA4896C3A53CBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CFA4896C3A53CBBu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                x, 
                y
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_bike_on_stand_raw(
        vehicle: , 
        x: , 
        y: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CFA4896C3A53CBBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CFA4896C3A53CBBu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                x, 
                y
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_has_kers_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50634E348C8D44EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50634E348C8D44EFu64;
        
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
pub fn get_vehicle_has_kers_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50634E348C8D44EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50634E348C8D44EFu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Money pickups are created around cars when they explode. Only works when the vehicle model is a car. A single pickup is between 1 and 18 dollars in size. All car models seem to give the same amount of money.
youtu.be/3arlUxzHl5Y
i.imgur.com/WrNpYFs.jpg
```



pub fn set_vehicle_drops_money_when_blown_up_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x068F64F2470F9656u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x068F64F2470F9656u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_drops_money_when_blown_up_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x068F64F2470F9656u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x068F64F2470F9656u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Toggles whether ambient trains can spawn on the specified track or not.

| trackId | File | Description |
|



pub fn switch_train_track_safe(
        
        
            trackId: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD813BB7DB977F20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD813BB7DB977F20u64;
        
        let result = invoke_raw!(
            hash,
                trackId, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn switch_train_track_raw(
        trackId: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFD813BB7DB977F20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFD813BB7DB977F20u64;

        invoke_raw_typed!(
            hash,
                trackId, 
                state
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_use_cutscene_wheel_compression_safe(
        
        
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
        let hash = 0xE023E8AC4EF7C117u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE023E8AC4EF7C117u64;
        
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
pub fn set_vehicle_use_cutscene_wheel_compression_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE023E8AC4EF7C117u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE023E8AC4EF7C117u64;

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
GET_VEHICLE_MODEL_*
Function pertains only to aviation vehicles.
```



pub fn _get_vehicle_model_max_knots_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6AD107DDC9054CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6AD107DDC9054CCu64;
        
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
pub fn _get_vehicle_model_max_knots_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6AD107DDC9054CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6AD107DDC9054CCu64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// ## Parameters
*



pub fn is_taxi_light_on_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7504C0F113AB50FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7504C0F113AB50FCu64;
        
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
pub fn is_taxi_light_on_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7504C0F113AB50FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7504C0F113AB50FCu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)



pub fn _get_entry_position_of_door_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0572928C0ABFDA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0572928C0ABFDA3u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_entry_position_of_door_raw(
        vehicle: , 
        doorIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0572928C0ABFDA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0572928C0ABFDA3u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex
        )
    }
}

/// ## Parameters
*



pub fn _disable_vehicle_turret_movement_this_frame_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32CAEDF24A583345u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32CAEDF24A583345u64;
        
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
pub fn _disable_vehicle_turret_movement_this_frame_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32CAEDF24A583345u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32CAEDF24A583345u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
seems to make the vehicle stop spawning naturally in traffic. Here's an essential example:  
VEHICLE::SET_VEHICLE_MODEL_IS_SUPPRESSED(GAMEPLAY::GET_HASH_KEY("taco"), true);  
```



pub fn set_vehicle_model_is_suppressed_safe(
        
        
            model: 
        , 
        
        
            suppressed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0FC2D89AC25A5814u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0FC2D89AC25A5814u64;
        
        let result = invoke_raw!(
            hash,
                model, 
                suppressed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_model_is_suppressed_raw(
        model: , 
        suppressed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0FC2D89AC25A5814u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0FC2D89AC25A5814u64;

        invoke_raw_typed!(
            hash,
                model, 
                suppressed
        )
    }
}

/// ## Parameters
*



pub fn set_cargobob_pickup_magnet_reduced_strength_safe(
        
        
            cargobob: 
        , 
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE301BD63E9E13CF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE301BD63E9E13CF0u64;
        
        let result = invoke_raw!(
            hash,
                cargobob, 
                vehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cargobob_pickup_magnet_reduced_strength_raw(
        cargobob: , 
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE301BD63E9E13CF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE301BD63E9E13CF0u64;

        invoke_raw_typed!(
            hash,
                cargobob, 
                vehicle
        )
    }
}

/// ```
Returns the number of *types* of licence plates, enumerated below in SET_VEHICLE_NUMBER_PLATE_TEXT_INDEX.  
```



pub fn get_number_of_vehicle_number_plates_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C4D6B2644F458CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C4D6B2644F458CBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_number_of_vehicle_number_plates_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C4D6B2644F458CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C4D6B2644F458CBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _set_heli_main_rotor_health_safe(
        
        
            vehicle: 
        , 
        
        
            health: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4056EA1105F5ABD7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4056EA1105F5ABD7u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                health
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_heli_main_rotor_health_raw(
        vehicle: , 
        health: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4056EA1105F5ABD7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4056EA1105F5ABD7u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                health
        )
    }
}

/// ```
p1 can be anywhere from 0 to 3 in the scripts. p2 is generally somewhere in the 1000 to 10000 range.  
```



pub fn is_vehicle_stuck_timer_up_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x679BE1DAF71DA874u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x679BE1DAF71DA874u64;
        
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
pub fn is_vehicle_stuck_timer_up_raw(
        vehicle: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x679BE1DAF71DA874u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x679BE1DAF71DA874u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1, 
                p2
        )
    }
}

/// See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).

This function is coded to not work on vehicles of type: `CBike`, `Bmx`, `CBoat`, `CTrain`, and `CSubmarine`.



pub fn fix_vehicle_window_safe(
        
        
            vehicle: 
        , 
        
        
            windowIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x772282EBEB95E682u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x772282EBEB95E682u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                windowIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn fix_vehicle_window_raw(
        vehicle: , 
        windowIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x772282EBEB95E682u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x772282EBEB95E682u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                windowIndex
        )
    }
}

/// ```c
enum eColourBitField {
    HAS_BODY_COLOUR1 = 1,
    HAS_BODY_COLOUR2 = 2,
    HAS_BODY_COLOUR3 = 4,
    HAS_BODY_COLOUR4 = 8,
    HAS_BODY_COLOUR5 = 16
}
```



pub fn get_vehicle_colours_which_can_be_set_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEBFC7A7EFDC35B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEBFC7A7EFDC35B4u64;
        
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
pub fn get_vehicle_colours_which_can_be_set_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEEBFC7A7EFDC35B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEEBFC7A7EFDC35B4u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _set_vehicle_unk_damage_multiplier_safe(
        
        
            vehicle: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45A561A9421AB6ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45A561A9421AB6ADu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_unk_damage_multiplier_raw(
        vehicle: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45A561A9421AB6ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45A561A9421AB6ADu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn close_bomb_bay_doors_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3556041742A0DC74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3556041742A0DC74u64;
        
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
pub fn close_bomb_bay_doors_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3556041742A0DC74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3556041742A0DC74u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn has_vehicle_asset_loaded_safe(
        
        
            vehicleAsset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1BBE0523B8DB9A21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1BBE0523B8DB9A21u64;
        
        let result = invoke_raw!(
            hash,
                vehicleAsset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_vehicle_asset_loaded_raw(
        vehicleAsset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1BBE0523B8DB9A21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1BBE0523B8DB9A21u64;

        invoke_raw_typed!(
            hash,
                vehicleAsset
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_attached_to_cargobob_safe(
        
        
            cargobob: 
        , 
        
        
            vehicleAttached: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD40148F22E81A1D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD40148F22E81A1D9u64;
        
        let result = invoke_raw!(
            hash,
                cargobob, 
                vehicleAttached
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_attached_to_cargobob_raw(
        cargobob: , 
        vehicleAttached: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD40148F22E81A1D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD40148F22E81A1D9u64;

        invoke_raw_typed!(
            hash,
                cargobob, 
                vehicleAttached
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_inactive_during_playback_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06582AFF74894C75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06582AFF74894C75u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_inactive_during_playback_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06582AFF74894C75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06582AFF74894C75u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
Returns true if vehicle is halted by BRING_VEHICLE_TO_HALT
_IS_VEHICLE_*
```

```
NativeDB Introduced: v1493
```



pub fn _is_vehicle_being_halted_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC69BB1D832A710EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC69BB1D832A710EFu64;
        
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
pub fn _is_vehicle_being_halted_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC69BB1D832A710EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC69BB1D832A710EFu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Examples with a besra:

- [fade value `0.0`](https://i.imgur.com/DXNk63e.jpg)
- [fade value `0.5`](https://i.imgur.com/2Vb35fq.jpg)
- [fade value `1.0`](https://i.imgur.com/aa8cxaD.jpg)

The parameter fade is a value from 0-1, where 0 is fresh paint.



pub fn set_vehicle_enveff_scale_safe(
        
        
            vehicle: 
        , 
        
        
            fade: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3AFDC536C3D01674u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3AFDC536C3D01674u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                fade
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_enveff_scale_raw(
        vehicle: , 
        fade: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3AFDC536C3D01674u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3AFDC536C3D01674u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                fade
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_flight_nozzle_position_safe(
        
        
            aircraft: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA62027C8BDB326Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA62027C8BDB326Eu64;
        
        let result = invoke_raw!(
            hash,
                aircraft
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_flight_nozzle_position_raw(
        aircraft: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA62027C8BDB326Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA62027C8BDB326Eu64;

        invoke_raw_typed!(
            hash,
                aircraft
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_doors_locked_for_non_script_players_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9737A37136F07E75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9737A37136F07E75u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_doors_locked_for_non_script_players_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9737A37136F07E75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9737A37136F07E75u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _is_this_model_an_amphibious_quadbike_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1A9FC1C76A6730Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1A9FC1C76A6730Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_this_model_an_amphibious_quadbike_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1A9FC1C76A6730Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1A9FC1C76A6730Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Disables collision for this vehicle (maybe it also supports other entities, not sure).
Only world/building/fixed world objects will have their collisions disabled, props, peds, or any other entity still collides with the vehicle.

[Example video](https://streamable.com/6n45d5)

Not sure if there is a native (and if so, which one) that resets the collisions.



pub fn _disable_vehicle_world_collision_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75627043C6AA90ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75627043C6AA90ADu64;
        
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
pub fn _disable_vehicle_world_collision_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75627043C6AA90ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75627043C6AA90ADu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_ambient_vehicle_range_multiplier_this_frame_safe(
        
        
            range: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90B6DA738A9A25DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90B6DA738A9A25DAu64;
        
        let result = invoke_raw!(
            hash,
                range
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ambient_vehicle_range_multiplier_this_frame_raw(
        range: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90B6DA738A9A25DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90B6DA738A9A25DAu64;

        invoke_raw_typed!(
            hash,
                range
        )
    }
}

/// ## Parameters
*



pub fn _set_vehicle_receives_ramp_damage_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28D034A93FE31BF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28D034A93FE31BF5u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_receives_ramp_damage_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28D034A93FE31BF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28D034A93FE31BF5u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn clear_vehicle_custom_secondary_colour_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FFBDEEC3E8E2009u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FFBDEEC3E8E2009u64;
        
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
pub fn clear_vehicle_custom_secondary_colour_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FFBDEEC3E8E2009u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FFBDEEC3E8E2009u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Transforms the `stormberg` to its "road vehicle" variant. If the vehicle is already in that state then the vehicle transformation audio will still play, but the vehicle won't change at all.



pub fn transform_to_car_safe(
        
        
            vehicle: 
        , 
        
        
            instantly: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A69FFD1B42BFF9Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A69FFD1B42BFF9Eu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                instantly
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn transform_to_car_raw(
        vehicle: , 
        instantly: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A69FFD1B42BFF9Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A69FFD1B42BFF9Eu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                instantly
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_rudder_broken_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x09606148B6C71DEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x09606148B6C71DEFu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_rudder_broken_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x09606148B6C71DEFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x09606148B6C71DEFu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
Implemented only for Trains.
```

```
NativeDB Introduced: v2372
```



pub fn _network_use_high_precision_vehicle_blending_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC0C1D4922AF9754u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC0C1D4922AF9754u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _network_use_high_precision_vehicle_blending_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEC0C1D4922AF9754u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEC0C1D4922AF9754u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_turret_speed_this_frame_safe(
        
        
            vehicle: 
        , 
        
        
            speed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1093408B4B9D1146u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1093408B4B9D1146u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                speed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_turret_speed_this_frame_raw(
        vehicle: , 
        speed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1093408B4B9D1146u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1093408B4B9D1146u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                speed
        )
    }
}

/// ```
Returns last vehicle that was rammed by the given vehicle using the shunt boost.

NativeDB Introduced: v1604
```



pub fn _get_last_rammed_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04F2FA6E234162F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04F2FA6E234162F7u64;
        
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
pub fn _get_last_rammed_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x04F2FA6E234162F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x04F2FA6E234162F7u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn get_random_vehicle_front_bumper_in_sphere_safe(
        
        
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
        let hash = 0xC5574E0AEB86BA68u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5574E0AEB86BA68u64;
        
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
pub fn get_random_vehicle_front_bumper_in_sphere_raw(
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
        let hash = 0xC5574E0AEB86BA68u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5574E0AEB86BA68u64;

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

/// ## Parameters
*



pub fn _set_vehicle_can_engine_operate_on_fire_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x206BC5DC9D1AC70Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x206BC5DC9D1AC70Au64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_can_engine_operate_on_fire_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x206BC5DC9D1AC70Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x206BC5DC9D1AC70Au64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Retrieves a static value representing the estimated max speed of a specific vehicle, including any vehicle mods. This value does not change dynamically during gameplay.

```
NativeDB Introduced: v323
```



pub fn get_vehicle_estimated_max_speed_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53AF99BAA671CA47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53AF99BAA671CA47u64;
        
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
pub fn get_vehicle_estimated_max_speed_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53AF99BAA671CA47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53AF99BAA671CA47u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_needs_to_be_hotwired_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFBA550EA44404EE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFBA550EA44404EE6u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_needs_to_be_hotwired_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFBA550EA44404EE6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFBA550EA44404EE6u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_class_estimated_max_speed_safe(
        
        
            vehicleClass: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00C09F246ABEDD82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00C09F246ABEDD82u64;
        
        let result = invoke_raw!(
            hash,
                vehicleClass
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_class_estimated_max_speed_raw(
        vehicleClass: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00C09F246ABEDD82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00C09F246ABEDD82u64;

        invoke_raw_typed!(
            hash,
                vehicleClass
        )
    }
}

/// ## Parameters
*



pub fn set_cargobob_pickup_magnet_effect_radius_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA17BAD153B51547Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA17BAD153B51547Eu64;
        
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
pub fn set_cargobob_pickup_magnet_effect_radius_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA17BAD153B51547Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA17BAD153B51547Eu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn is_toggle_mod_on_safe(
        
        
            vehicle: 
        , 
        
        
            modType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84B233A8C8FC8AE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84B233A8C8FC8AE7u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                modType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_toggle_mod_on_raw(
        vehicle: , 
        modType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84B233A8C8FC8AE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84B233A8C8FC8AE7u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                modType
        )
    }
}

/// ## Parameters
*



pub fn reset_vehicle_wheels_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21D2E5662C1F6FEDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21D2E5662C1F6FEDu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_vehicle_wheels_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21D2E5662C1F6FEDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21D2E5662C1F6FEDu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
Controls how much traction the wheel loses.
Default values from Rockstar's Open Wheel Race JSON's:
"owrtds" (Soft): 0.05
"owrtdm" (Medium): 0.45
"owrtdh" (Hard): 0.8
Usable wheels:
0: wheel_lf
1: wheel_rf
2: wheel_lm1
3: wheel_rm1
4: wheel_lr
5: wheel_rr
```

```
NativeDB Introduced: v2060
```



pub fn _set_tyre_traction_loss_multiplier_safe(
        
        
            vehicle: 
        , 
        
        
            wheelIndex: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC970D0E0FC31D768u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC970D0E0FC31D768u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                wheelIndex, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_tyre_traction_loss_multiplier_raw(
        vehicle: , 
        wheelIndex: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC970D0E0FC31D768u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC970D0E0FC31D768u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                wheelIndex, 
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn _0x8533cafde1f0f336_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8533CAFDE1F0F336u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8533CAFDE1F0F336u64;
        
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
pub fn _0x8533cafde1f0f336_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8533CAFDE1F0F336u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8533CAFDE1F0F336u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Stops cargobob from being able to detach the attached vehicle.



pub fn set_cargobob_force_dont_detach_vehicle_safe(
        
        
            cargobob: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x571FEB383F629926u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x571FEB383F629926u64;
        
        let result = invoke_raw!(
            hash,
                cargobob, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cargobob_force_dont_detach_vehicle_raw(
        cargobob: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x571FEB383F629926u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x571FEB383F629926u64;

        invoke_raw_typed!(
            hash,
                cargobob, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn add_road_node_speed_zone_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            speed: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CE544C68FB812A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CE544C68FB812A0u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                speed, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_road_node_speed_zone_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        speed: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2CE544C68FB812A0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2CE544C68FB812A0u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                speed, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_mod_kit_type_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC058F5121E54C32u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC058F5121E54C32u64;
        
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
pub fn get_vehicle_mod_kit_type_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC058F5121E54C32u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC058F5121E54C32u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
p1, p2, p3 are RGB values for color (255,0,0 for Red, ect)  
```



pub fn set_vehicle_custom_secondary_colour_safe(
        
        
            vehicle: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36CED73BFED89754u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36CED73BFED89754u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn set_vehicle_custom_secondary_colour_raw(
        vehicle: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36CED73BFED89754u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36CED73BFED89754u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                r, 
                g, 
                b
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0xa4a9a4c40e615885_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4A9A4C40E615885u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4A9A4C40E615885u64;
        
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
pub fn _0xa4a9a4c40e615885_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4A9A4C40E615885u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4A9A4C40E615885u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x5ba68a0840d546ac_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BA68A0840D546ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BA68A0840D546ACu64;
        
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
pub fn _0x5ba68a0840d546ac_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BA68A0840D546ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BA68A0840D546ACu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Changes the secondary paint type and color  
paintType:  
0: Normal  
1: Metallic  
2: Pearl  
3: Matte  
4: Metal  
5: Chrome  
color: number of the color  
```



pub fn set_vehicle_mod_color_2_safe(
        
        
            vehicle: 
        , 
        
        
            paintType: 
        , 
        
        
            color: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x816562BADFDEC83Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x816562BADFDEC83Eu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                paintType, 
                color
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_mod_color_2_raw(
        vehicle: , 
        paintType: , 
        color: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x816562BADFDEC83Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x816562BADFDEC83Eu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                paintType, 
                color
        )
    }
}

/// ```
min: 1.9f, max: 100.0f
```



pub fn set_pickup_rope_length_for_cargobob_safe(
        
        
            cargobob: 
        , 
        
        
            length1: 
        , 
        
        
            length2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x877C1EAEAC531023u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x877C1EAEAC531023u64;
        
        let result = invoke_raw!(
            hash,
                cargobob, 
                length1, 
                length2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_pickup_rope_length_for_cargobob_raw(
        cargobob: , 
        length1: , 
        length2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x877C1EAEAC531023u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x877C1EAEAC531023u64;

        invoke_raw_typed!(
            hash,
                cargobob, 
                length1, 
                length2
        )
    }
}

/// ## Parameters
*



pub fn release_preload_mods_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x445D79F995508307u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x445D79F995508307u64;
        
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
pub fn release_preload_mods_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x445D79F995508307u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x445D79F995508307u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _0xd4196117af7bb974_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4196117AF7BB974u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4196117AF7BB974u64;
        
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
pub fn _0xd4196117af7bb974_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4196117AF7BB974u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4196117AF7BB974u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn detach_vehicle_from_trailer_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90532EDF0D2BDD86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90532EDF0D2BDD86u64;
        
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
pub fn detach_vehicle_from_trailer_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90532EDF0D2BDD86u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90532EDF0D2BDD86u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _0x51db102f4a3ba5e0_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51DB102F4A3BA5E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51DB102F4A3BA5E0u64;
        
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
pub fn _0x51db102f4a3ba5e0_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51DB102F4A3BA5E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51DB102F4A3BA5E0u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Will disable a plane or a helicopter's need to swerve around object in its heightmap when using TASK_PLANE_MISSION or other AI / Pilot behavior.  Will ensure plane flys directly to it's destination or die trying! This native does NOT need to be called every frame, but instead, just called once on the vehicle (NOT THE PED) you're trying to disable avoidance for!



pub fn _enable_aircraft_obstacle_avoidance_safe(
        
        
            vehicle: 
        , 
        
        
            avoidObstacles: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8AA9180DE2FEDD45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8AA9180DE2FEDD45u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                avoidObstacles
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _enable_aircraft_obstacle_avoidance_raw(
        vehicle: , 
        avoidObstacles: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8AA9180DE2FEDD45u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8AA9180DE2FEDD45u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                avoidObstacles
        )
    }
}

/// ```
Returns true if the vehicle's current speed is less than, or equal to 0.0025f.
For some vehicles it returns true if the current speed is <= 0.00039999999.
```



pub fn is_vehicle_stopped_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5721B434AD84D57Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5721B434AD84D57Au64;
        
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
pub fn is_vehicle_stopped_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5721B434AD84D57Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5721B434AD84D57Au64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Only works during nighttime.
```



pub fn set_vehicle_searchlight_safe(
        
        
            heli: 
        , 
        
        
            toggle: 
        , 
        
        
            canBeUsedByAI: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14E85C5EE7A4D542u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14E85C5EE7A4D542u64;
        
        let result = invoke_raw!(
            hash,
                heli, 
                toggle, 
                canBeUsedByAI
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_searchlight_raw(
        heli: , 
        toggle: , 
        canBeUsedByAI: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14E85C5EE7A4D542u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14E85C5EE7A4D542u64;

        invoke_raw_typed!(
            hash,
                heli, 
                toggle, 
                canBeUsedByAI
        )
    }
}

/// ```
True stops vtols from switching modes. Doesn't stop the sound though.
```

```
NativeDB Introduced: v1290
```



pub fn _set_disable_vehicle_flight_nozzle_position_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE2B43770B655F8Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE2B43770B655F8Fu64;
        
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
pub fn _set_disable_vehicle_flight_nozzle_position_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE2B43770B655F8Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE2B43770B655F8Fu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn is_turret_seat_safe(
        
        
            vehicle: 
        , 
        
        
            seatIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE33FFA906CE74880u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE33FFA906CE74880u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                seatIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_turret_seat_raw(
        vehicle: , 
        seatIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE33FFA906CE74880u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE33FFA906CE74880u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                seatIndex
        )
    }
}

/// Checks if a boat can be anchored at its present position, ignoring any players standing on the boat.

```
NativeDB Introduced: v678
```



pub fn can_anchor_boat_here_ignore_players_safe(
        
        
            boat: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24F4121D07579880u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24F4121D07579880u64;
        
        let result = invoke_raw!(
            hash,
                boat
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_anchor_boat_here_ignore_players_raw(
        boat: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24F4121D07579880u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24F4121D07579880u64;

        invoke_raw_typed!(
            hash,
                boat
        )
    }
}

/// Determines whether the specified vehicle is equipped with a searchlight.

```
NativeDB Introduced: v2189
```



pub fn does_vehicle_have_searchlight_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99015ED7DBEA5113u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99015ED7DBEA5113u64;
        
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
pub fn does_vehicle_have_searchlight_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99015ED7DBEA5113u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99015ED7DBEA5113u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Allows vehicles with the FLAG_JUMPING_CAR flag to jump higher (i.e. Ruiner 2000).



pub fn _set_use_higher_vehicle_jump_force_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF06A16CA55D138D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF06A16CA55D138D8u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_use_higher_vehicle_jump_force_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF06A16CA55D138D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF06A16CA55D138D8u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_num_mod_kits_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33F2E3FE70EAAE1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33F2E3FE70EAAE1Du64;
        
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
pub fn get_num_mod_kits_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33F2E3FE70EAAE1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33F2E3FE70EAAE1Du64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
This fixes the deformation of a vehicle but the vehicle health doesn't improve  
```



pub fn set_vehicle_deformation_fixed_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x953DA1E1B12C0491u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x953DA1E1B12C0491u64;
        
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
pub fn set_vehicle_deformation_fixed_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x953DA1E1B12C0491u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x953DA1E1B12C0491u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// This multiplier has no limit, by default the game has this set to `1.0`.



pub fn set_vehicle_light_multiplier_safe(
        
        
            vehicle: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB385454F8791F57Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB385454F8791F57Cu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_light_multiplier_raw(
        vehicle: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB385454F8791F57Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB385454F8791F57Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                multiplier
        )
    }
}

/// ```
NativeDB Introduced: v3407
```



pub fn _set_plane_avoids_others_safe(
        
        
            plane: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC40CBF7B90CA77Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC40CBF7B90CA77Cu64;
        
        let result = invoke_raw!(
            hash,
                plane, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_plane_avoids_others_raw(
        plane: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC40CBF7B90CA77Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC40CBF7B90CA77Cu64;

        invoke_raw_typed!(
            hash,
                plane, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v3095
```

Recharges the nitrous system of the specified vehicle to its maximum capacity. This action sets the nitrous charge duration to the maximum limit defined by previous settings applied through [`SET_OVERRIDE_NITROUS_LEVEL`](#_0xC8E9B6B71B8E660D).



pub fn fully_charge_nitrous_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A2BCC8C636F9226u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A2BCC8C636F9226u64;
        
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
pub fn fully_charge_nitrous_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A2BCC8C636F9226u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A2BCC8C636F9226u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_doors_locked_for_player_safe(
        
        
            vehicle: 
        , 
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x517AAF684BB50CD1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x517AAF684BB50CD1u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_doors_locked_for_player_raw(
        vehicle: , 
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x517AAF684BB50CD1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x517AAF684BB50CD1u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                player, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x0419b167ee128f33_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0419B167EE128F33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0419B167EE128F33u64;
        
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
pub fn _0x0419b167ee128f33_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0419B167EE128F33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0419B167EE128F33u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Returns a float value between 0.0 and 3.0 related to its slipstream draft (boost/speedup).
GET_VEHICLE_*
```



pub fn _get_vehicle_current_slipstream_draft_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36492C2F0D134C56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36492C2F0D134C56u64;
        
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
pub fn _get_vehicle_current_slipstream_draft_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36492C2F0D134C56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36492C2F0D134C56u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_engine_can_degrade_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x983765856F2564F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x983765856F2564F9u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_engine_can_degrade_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x983765856F2564F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x983765856F2564F9u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// colorPrimary & colorSecondary are the paint indexes for the vehicle.  

For a list of valid paint indexes, view: pastebin.com/pwHci0xK



pub fn set_vehicle_colours_safe(
        
        
            vehicle: 
        , 
        
        
            colorPrimary: 
        , 
        
        
            colorSecondary: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F1D4BE3A7F24601u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F1D4BE3A7F24601u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                colorPrimary, 
                colorSecondary
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_colours_raw(
        vehicle: , 
        colorPrimary: , 
        colorSecondary: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F1D4BE3A7F24601u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F1D4BE3A7F24601u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                colorPrimary, 
                colorSecondary
        )
    }
}

/// Set state to true to extend the wings, false to retract them.



pub fn _set_oppressor_transform_state_safe(
        
        
            vehicle: 
        , 
        
        
            extend: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x544996C0081ABDEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x544996C0081ABDEBu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                extend
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_oppressor_transform_state_raw(
        vehicle: , 
        extend: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x544996C0081ABDEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x544996C0081ABDEBu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                extend
        )
    }
}

/// ```
Returns false if the vehicle has the FLAG_NO_RESPRAY flag set.
```



pub fn is_vehicle_sprayable_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D474C8FAEFF6CDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D474C8FAEFF6CDEu64;
        
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
pub fn is_vehicle_sprayable_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D474C8FAEFF6CDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D474C8FAEFF6CDEu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
If set to TRUE, it seems to suppress door noises and doesn't allow the horn to be continuous.  
```



pub fn _set_vehicle_silent_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D44FCCE98450843u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D44FCCE98450843u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_silent_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D44FCCE98450843u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D44FCCE98450843u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// A getter for [`SET_VEHICLE_DIRT_LEVEL`](#_0x79D3B596FE44EE8B).



pub fn get_vehicle_dirt_level_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F17BC8BA08DA62Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F17BC8BA08DA62Bu64;
        
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
pub fn get_vehicle_dirt_level_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F17BC8BA08DA62Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F17BC8BA08DA62Bu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _0xc4b3347bd68bd609_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4B3347BD68BD609u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4B3347BD68BD609u64;
        
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
pub fn _0xc4b3347bd68bd609_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC4B3347BD68BD609u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC4B3347BD68BD609u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x6ebfb22d646ffc18_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6EBFB22D646FFC18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6EBFB22D646FFC18u64;
        
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
pub fn _0x6ebfb22d646ffc18_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6EBFB22D646FFC18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6EBFB22D646FFC18u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0x5bbcf35bf6e456f7_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BBCF35BF6E456F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BBCF35BF6E456F7u64;
        
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
pub fn _0x5bbcf35bf6e456f7_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5BBCF35BF6E456F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5BBCF35BF6E456F7u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0xe05dd0e9707003a3_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE05DD0E9707003A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE05DD0E9707003A3u64;
        
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
pub fn _0xe05dd0e9707003a3_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE05DD0E9707003A3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE05DD0E9707003A3u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _get_does_vehicle_have_tombstone_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71AFB258CCED3A27u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71AFB258CCED3A27u64;
        
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
pub fn _get_does_vehicle_have_tombstone_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71AFB258CCED3A27u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71AFB258CCED3A27u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_submarine_crush_depths_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        , 
        
        
            depth1: 
        , 
        
        
            depth2: 
        , 
        
        
            depth3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC59872A5134879C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC59872A5134879C7u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle, 
                depth1, 
                depth2, 
                depth3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_submarine_crush_depths_raw(
        vehicle: , 
        toggle: , 
        depth1: , 
        depth2: , 
        depth3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC59872A5134879C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC59872A5134879C7u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle, 
                depth1, 
                depth2, 
                depth3
        )
    }
}

/// ## Parameters
*



pub fn _0x73561d4425a021a2_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73561D4425A021A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73561D4425A021A2u64;
        
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
pub fn _0x73561d4425a021a2_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x73561D4425A021A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x73561D4425A021A2u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
AI abides by the provided driving style (e.g., stopping at red lights or waiting behind traffic) while executing the specificed vehicle recording.

0x1F2E4E06DEA8992B is a related native that deals with the AI physics for such recordings.
```



pub fn start_playback_recorded_vehicle_using_ai_safe(
        
        
            vehicle: 
        , 
        
        
            recording: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29DE5FA52D00428Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29DE5FA52D00428Cu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                recording
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_playback_recorded_vehicle_using_ai_raw(
        vehicle: , 
        recording: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29DE5FA52D00428Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29DE5FA52D00428Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                recording
        )
    }
}

/// ```
Seems to be related to the metal parts, not tyres (like i was expecting lol)  
Must be called every tick.  
```



pub fn set_vehicle_friction_override_safe(
        
        
            vehicle: 
        , 
        
        
            friction: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1837AF7C627009BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1837AF7C627009BAu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                friction
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_friction_override_raw(
        vehicle: , 
        friction: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1837AF7C627009BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1837AF7C627009BAu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                friction
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_lights_state_safe(
        
        
            vehicle: 
        , 
        
        
            lightsOn: 
        , 
        
        
            highbeamsOn: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB91B4C20085BD12Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB91B4C20085BD12Fu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                lightsOn, 
                highbeamsOn
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_lights_state_raw(
        vehicle: , 
        lightsOn: , 
        highbeamsOn: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB91B4C20085BD12Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB91B4C20085BD12Fu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                lightsOn, 
                highbeamsOn
        )
    }
}

/// ## Parameters
*



pub fn remove_vehicle_combat_avoidance_area_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE30524E1871F481Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE30524E1871F481Du64;
        
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
pub fn remove_vehicle_combat_avoidance_area_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE30524E1871F481Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE30524E1871F481Du64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_far_draw_vehicles_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26324F33423F3CC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26324F33423F3CC3u64;
        
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
pub fn set_far_draw_vehicles_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26324F33423F3CC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26324F33423F3CC3u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_visible_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA0A52D24FB98293u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA0A52D24FB98293u64;
        
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
pub fn is_vehicle_visible_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA0A52D24FB98293u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA0A52D24FB98293u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_force_afterburner_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB055A34527CB8FD7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB055A34527CB8FD7u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_force_afterburner_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB055A34527CB8FD7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB055A34527CB8FD7u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_entity_attached_to_handler_frame_safe(
        
        
            vehicle: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57715966069157ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57715966069157ADu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_entity_attached_to_handler_frame_raw(
        vehicle: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57715966069157ADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57715966069157ADu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                entity
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn get_heli_tail_rotor_health_safe(
        
        
            heli: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE8CE82A4219AC8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE8CE82A4219AC8Cu64;
        
        let result = invoke_raw!(
            hash,
                heli
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_heli_tail_rotor_health_raw(
        heli: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE8CE82A4219AC8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE8CE82A4219AC8Cu64;

        invoke_raw_typed!(
            hash,
                heli
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_can_deform_wheels_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CDDA42F9E360CA6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CDDA42F9E360CA6u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_can_deform_wheels_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0CDDA42F9E360CA6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0CDDA42F9E360CA6u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Enables or disables the convertible roof on vehicles that support old-style GTA IV roofs, which are not animated. Setting `toggle` to true will apply the roof to the vehicle, and setting it to false will remove the roof, assuming the vehicle has versions with and without a roof.

If you want to lock or unlock the roof mechanism, use [`SET_CONVERTIBLE_ROOF_LATCH_STATE`](#_0x1A78AD3D8240536F).

You can check if the vehicle has a roof with [`DOES_VEHICLE_HAVE_ROOF`](#_0x8AC862B0B32C5B80).

```
NativeDB Introduced: v323
```



pub fn set_convertible_roof_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF39C4F538B5124C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF39C4F538B5124C2u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_convertible_roof_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF39C4F538B5124C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF39C4F538B5124C2u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
Related to locking the vehicle or something similar.  
In the decompiled scripts, its always called after  
VEHICLE::_SET_EXCLUSIVE_DRIVER(a_0, 0, 0);  
VEHICLE::SET_VEHICLE_DOORS_LOCKED_FOR_ALL_PLAYERS(a_0, 1);  
VEHICLE::SET_VEHICLE_DOORS_LOCKED_FOR_PLAYER(a_0, PLAYER::PLAYER_ID(), 0);  
```



pub fn _0xdbc631f109350b8c_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBC631F109350B8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBC631F109350B8Cu64;
        
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
pub fn _0xdbc631f109350b8c_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDBC631F109350B8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDBC631F109350B8Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
Returns true if the vehicle has the FLAG_ALLOWS_RAPPEL flag set.
```



pub fn _does_vehicle_allow_rappel_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E417C547182C84Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E417C547182C84Du64;
        
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
pub fn _does_vehicle_allow_rappel_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E417C547182C84Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E417C547182C84Du64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Set modKit to 0 if you plan to call SET_VEHICLE_MOD. That's what the game does. Most body modifications through SET_VEHICLE_MOD will not take effect until this is set to 0.
```



pub fn set_vehicle_mod_kit_safe(
        
        
            vehicle: 
        , 
        
        
            modKit: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F2AA07F00B3217Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F2AA07F00B3217Au64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                modKit
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_mod_kit_raw(
        vehicle: , 
        modKit: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F2AA07F00B3217Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F2AA07F00B3217Au64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                modKit
        )
    }
}

/// ## Parameters
*



pub fn _0x7bbe7ff626a591fe_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7BBE7FF626A591FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7BBE7FF626A591FEu64;
        
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
pub fn _0x7bbe7ff626a591fe_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7BBE7FF626A591FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7BBE7FF626A591FEu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_colour_combination_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A842D197F845D56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A842D197F845D56u64;
        
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
pub fn get_vehicle_colour_combination_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A842D197F845D56u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A842D197F845D56u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _0xb9562064627ff9db_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9562064627FF9DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9562064627FF9DBu64;
        
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
pub fn _0xb9562064627ff9db_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9562064627FF9DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9562064627FF9DBu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_max_traction_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA132FB5370554DB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA132FB5370554DB0u64;
        
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
pub fn get_vehicle_max_traction_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA132FB5370554DB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA132FB5370554DB0u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _get_has_retractable_wheels_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCA174A42133F08Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCA174A42133F08Cu64;
        
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
pub fn _get_has_retractable_wheels_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCA174A42133F08Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCA174A42133F08Cu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Determines whether the specified Cargobob can pick up a given entity.



pub fn can_cargobob_pick_up_entity_safe(
        
        
            cargobob: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C1D8B3B19E517CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C1D8B3B19E517CCu64;
        
        let result = invoke_raw!(
            hash,
                cargobob, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_cargobob_pick_up_entity_raw(
        cargobob: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C1D8B3B19E517CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C1D8B3B19E517CCu64;

        invoke_raw_typed!(
            hash,
                cargobob, 
                entity
        )
    }
}

/// ```
Won't attract or magnetize to any helicopters or planes of course, but that's common sense.  
```



pub fn set_cargobob_pickup_magnet_active_safe(
        
        
            cargobob: 
        , 
        
        
            isActive: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A665550F8DA349Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A665550F8DA349Bu64;
        
        let result = invoke_raw!(
            hash,
                cargobob, 
                isActive
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cargobob_pickup_magnet_active_raw(
        cargobob: , 
        isActive: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A665550F8DA349Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A665550F8DA349Bu64;

        invoke_raw_typed!(
            hash,
                cargobob, 
                isActive
        )
    }
}

/// ```
Closes all doors of a vehicle:  
```



pub fn set_vehicle_doors_shut_safe(
        
        
            vehicle: 
        , 
        
        
            closeInstantly: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x781B3D62BB013EF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x781B3D62BB013EF5u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                closeInstantly
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_doors_shut_raw(
        vehicle: , 
        closeInstantly: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x781B3D62BB013EF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x781B3D62BB013EF5u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                closeInstantly
        )
    }
}

/// See [`SET_VEHICLE_CUSTOM_PRIMARY_COLOUR`](#_0x7141766F91D15BEA) and [`SET_VEHICLE_CUSTOM_SECONDARY_COLOUR`](#_0x36CED73BFED89754).



pub fn get_vehicle_color_safe(
        
        
            vehicle: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3CC740D36221548u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3CC740D36221548u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn get_vehicle_color_raw(
        vehicle: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3CC740D36221548u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3CC740D36221548u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                r, 
                g, 
                b
        )
    }
}

/// ## Parameters
*



pub fn _0x9f3f689b814f2599_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F3F689B814F2599u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F3F689B814F2599u64;
        
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
pub fn _0x9f3f689b814f2599_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F3F689B814F2599u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F3F689B814F2599u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_alarm_activated_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4319E335B71FFF34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4319E335B71FFF34u64;
        
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
pub fn is_vehicle_alarm_activated_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4319E335B71FFF34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4319E335B71FFF34u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Disables the additional physics forces applied to BMX bikes that enable them to perform tricks.

```
NativeDB Introduced: v463
```



pub fn set_disable_bmx_extra_trick_forces_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26D99D5A82FD18E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26D99D5A82FD18E8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_disable_bmx_extra_trick_forces_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x26D99D5A82FD18E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x26D99D5A82FD18E8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_mission_train_coords_safe(
        
        
            train: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x591CA673AA6AB736u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x591CA673AA6AB736u64;
        
        let result = invoke_raw!(
            hash,
                train, 
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
pub fn set_mission_train_coords_raw(
        train: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x591CA673AA6AB736u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x591CA673AA6AB736u64;

        invoke_raw_typed!(
            hash,
                train, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_use_alternate_handling_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D97D1E3A70A649Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D97D1E3A70A649Fu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_use_alternate_handling_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D97D1E3A70A649Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D97D1E3A70A649Fu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_entity_attached_to_tow_truck_safe(
        
        
            towTruck: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFEA18DCF10F8F75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFEA18DCF10F8F75u64;
        
        let result = invoke_raw!(
            hash,
                towTruck
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_entity_attached_to_tow_truck_raw(
        towTruck: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFEA18DCF10F8F75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFEA18DCF10F8F75u64;

        invoke_raw_typed!(
            hash,
                towTruck
        )
    }
}

/// ## Parameters
*



pub fn _set_boat_is_sinking_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD32E46AA95C1DD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD32E46AA95C1DD2u64;
        
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
pub fn _set_boat_is_sinking_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBD32E46AA95C1DD2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBD32E46AA95C1DD2u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn detach_vehicle_from_any_tow_truck_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0E9CE05A1E68CD8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0E9CE05A1E68CD8u64;
        
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
pub fn detach_vehicle_from_any_tow_truck_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0E9CE05A1E68CD8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0E9CE05A1E68CD8u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
This has not yet been tested - it's just an assumption of what the types could be.  
```



pub fn set_vehicle_can_be_targetted_safe(
        
        
            vehicle: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3750146A28097A82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3750146A28097A82u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_can_be_targetted_raw(
        vehicle: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3750146A28097A82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3750146A28097A82u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                state
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_model_safe(
        
        
            vehicle: 
        , 
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x423E8DE37D934D89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x423E8DE37D934D89u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_model_raw(
        vehicle: , 
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x423E8DE37D934D89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x423E8DE37D934D89u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                model
        )
    }
}

/// ## Parameters
*



pub fn _0xa7dcdf4ded40a8f4_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7DCDF4DED40A8F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7DCDF4DED40A8F4u64;
        
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
pub fn _0xa7dcdf4ded40a8f4_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA7DCDF4DED40A8F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA7DCDF4DED40A8F4u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
Does nothing. It's a nullsub.

NativeDB Introduced: v1604
```



pub fn _0x36de109527a2c0c4_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36DE109527A2C0C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36DE109527A2C0C4u64;
        
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
pub fn _0x36de109527a2c0c4_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36DE109527A2C0C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36DE109527A2C0C4u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _is_vehicle_slipstream_leader_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48C633E94A8142A7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48C633E94A8142A7u64;
        
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
pub fn _is_vehicle_slipstream_leader_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48C633E94A8142A7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48C633E94A8142A7u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
tyreIndex = 0 to 4 on normal vehicles  
'0 = wheel_lf / bike, plane or jet front  
'1 = wheel_rf  
'2 = wheel_lm / in 6 wheels trailer, plane or jet is first one on left  
'3 = wheel_rm / in 6 wheels trailer, plane or jet is first one on right  
'4 = wheel_lr / bike rear / in 6 wheels trailer, plane or jet is last one on left  
'5 = wheel_rr / in 6 wheels trailer, plane or jet is last one on right  
'45 = 6 wheels trailer mid wheel left  
'47 = 6 wheels trailer mid wheel right  
```



pub fn set_vehicle_tyre_fixed_safe(
        
        
            vehicle: 
        , 
        
        
            tyreIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E13FC662B882D1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E13FC662B882D1Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                tyreIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_tyre_fixed_raw(
        vehicle: , 
        tyreIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E13FC662B882D1Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E13FC662B882D1Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                tyreIndex
        )
    }
}

/// ## Parameters
*



pub fn _raise_retractable_wheels_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF660602546D27BA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF660602546D27BA8u64;
        
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
pub fn _raise_retractable_wheels_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF660602546D27BA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF660602546D27BA8u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Returns the convertible state of the specified vehicle.



```c
enum eRoofState {
    RAISED = 0,
    LOWERING = 1,
    LOWERED = 2,
    RAISING = 3,
    CLOSING_BOOT = 4,
    ROOF_STUCK_RAISED = 5,
    ROOF_STUCK_LOWERED = 6
}
```



pub fn get_convertible_roof_state_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8C397922FC03F41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8C397922FC03F41u64;
        
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
pub fn get_convertible_roof_state_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8C397922FC03F41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8C397922FC03F41u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Checks if the vehicle is electric.

```
NativeDB Introduced: v3258
```



pub fn _get_is_vehicle_electric_safe(
        
        
            vehicleModel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1FCB07FE230B6639u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1FCB07FE230B6639u64;
        
        let result = invoke_raw!(
            hash,
                vehicleModel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_is_vehicle_electric_raw(
        vehicleModel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1FCB07FE230B6639u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1FCB07FE230B6639u64;

        invoke_raw_typed!(
            hash,
                vehicleModel
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_stuck_on_roof_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB497F06B288DCFDFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB497F06B288DCFDFu64;
        
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
pub fn is_vehicle_stuck_on_roof_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB497F06B288DCFDFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB497F06B288DCFDFu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Sets the distance from the player at which anchored boats switch between high and low LOD (Level of Detail) buoyancy mode.

```
NativeDB Introduced: v323
```



pub fn set_boat_low_lod_anchor_distance_safe(
        
        
            boat: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE842A9398079BD82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE842A9398079BD82u64;
        
        let result = invoke_raw!(
            hash,
                boat, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_boat_low_lod_anchor_distance_raw(
        boat: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE842A9398079BD82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE842A9398079BD82u64;

        invoke_raw_typed!(
            hash,
                boat, 
                value
        )
    }
}

/// ## Parameters
*



pub fn _0x5845066d8a1ea7f7_safe(
        
        
            vehicle: 
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
        let hash = 0x5845066D8A1EA7F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5845066D8A1EA7F7u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn _0x5845066d8a1ea7f7_raw(
        vehicle: , 
        x: , 
        y: , 
        z: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5845066D8A1EA7F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5845066D8A1EA7F7u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                x, 
                y, 
                z, 
                p4
        )
    }
}

/// Lowers the vehicle's stance. Only works for vehicles that support this feature.

```
NativeDB Introduced: v2372
```



pub fn _set_reduce_drift_vehicle_suspension_safe(
        
        
            vehicle: 
        , 
        
        
            enable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A375167F5782A65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A375167F5782A65u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                enable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_reduce_drift_vehicle_suspension_raw(
        vehicle: , 
        enable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A375167F5782A65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A375167F5782A65u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                enable
        )
    }
}

/// Sets the dirt level of the passed vehicle.



pub fn set_vehicle_dirt_level_safe(
        
        
            vehicle: 
        , 
        
        
            dirtLevel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79D3B596FE44EE8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79D3B596FE44EE8Bu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                dirtLevel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_dirt_level_raw(
        vehicle: , 
        dirtLevel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x79D3B596FE44EE8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x79D3B596FE44EE8Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                dirtLevel
        )
    }
}

/// Drops the Hook/Magnet on a cargobob  

```c
enum eCargobobHook  
{  
	CARGOBOB_HOOK = 0,  
	CARGOBOB_MAGNET = 1,  
};  
```



pub fn create_pick_up_rope_for_cargobob_safe(
        
        
            cargobob: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7BEB0C7A235F6F3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7BEB0C7A235F6F3Bu64;
        
        let result = invoke_raw!(
            hash,
                cargobob, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_pick_up_rope_for_cargobob_raw(
        cargobob: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7BEB0C7A235F6F3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7BEB0C7A235F6F3Bu64;

        invoke_raw_typed!(
            hash,
                cargobob, 
                state
        )
    }
}

/// ```
how does this work?  
```



pub fn disable_vehicle_weapon_safe(
        
        
            disabled: 
        , 
        
        
            weaponHash: 
        , 
        
        
            vehicle: 
        , 
        
        
            owner: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4FC6A6F67D8D856u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4FC6A6F67D8D856u64;
        
        let result = invoke_raw!(
            hash,
                disabled, 
                weaponHash, 
                vehicle, 
                owner
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_vehicle_weapon_raw(
        disabled: , 
        weaponHash: , 
        vehicle: , 
        owner: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF4FC6A6F67D8D856u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF4FC6A6F67D8D856u64;

        invoke_raw_typed!(
            hash,
                disabled, 
                weaponHash, 
                vehicle, 
                owner
        )
    }
}

/// ## Parameters
*



pub fn _0x56eb5e94318d3fb6_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56EB5E94318D3FB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56EB5E94318D3FB6u64;
        
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
pub fn _0x56eb5e94318d3fb6_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56EB5E94318D3FB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56EB5E94318D3FB6u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// Forces a submarine to maintain neutral buoyancy for a specified duration, preventing it from rising when unoccupied or without a driver.

```
NativeDB Introduced: v2189
```



pub fn force_submarine_neurtal_buoyancy_safe(
        
        
            submarine: 
        , 
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC67DB108A9ADE3BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC67DB108A9ADE3BEu64;
        
        let result = invoke_raw!(
            hash,
                submarine, 
                time
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_submarine_neurtal_buoyancy_raw(
        submarine: , 
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC67DB108A9ADE3BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC67DB108A9ADE3BEu64;

        invoke_raw_typed!(
            hash,
                submarine, 
                time
        )
    }
}

/// ```
Returns the license plate text from a vehicle.  8 chars maximum.  
```



pub fn get_vehicle_number_plate_text_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CE1CCB9B293020Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CE1CCB9B293020Eu64;
        
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
pub fn get_vehicle_number_plate_text_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7CE1CCB9B293020Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7CE1CCB9B293020Eu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Returns true only when the hook is active, will return false if the magnet is active  
```



pub fn does_cargobob_have_pick_up_rope_safe(
        
        
            cargobob: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1821D91AD4B56108u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1821D91AD4B56108u64;
        
        let result = invoke_raw!(
            hash,
                cargobob
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_cargobob_have_pick_up_rope_raw(
        cargobob: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1821D91AD4B56108u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1821D91AD4B56108u64;

        invoke_raw_typed!(
            hash,
                cargobob
        )
    }
}

/// ```
SET_VEHICLE_LI*
```



pub fn _0xc50ce861b55eab8b_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC50CE861B55EAB8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC50CE861B55EAB8Bu64;
        
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
pub fn _0xc50ce861b55eab8b_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC50CE861B55EAB8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC50CE861B55EAB8Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// Locks the doors of a specified vehicle to a defined lock state, affecting how players and NPCs can interact with the vehicle.

```
NativeDB Introduced: v323
```

```c
enum eVehicleLockState {
    // No specific lock state, vehicle behaves according to the game's default settings.
    VEHICLELOCK_NONE = 0,
    // Vehicle is fully unlocked, allowing free entry by players and NPCs.
    VEHICLELOCK_UNLOCKED = 1,
    // Vehicle is locked, preventing entry by players and NPCs.
    VEHICLELOCK_LOCKED = 2,
    // Vehicle locks out only players, allowing NPCs to enter.
    VEHICLELOCK_LOCKOUT_PLAYER_ONLY = 3,
    // Vehicle is locked once a player enters, preventing others from entering.
    VEHICLELOCK_LOCKED_PLAYER_INSIDE = 4,
    // Vehicle starts in a locked state, but may be unlocked through game events.
    VEHICLELOCK_LOCKED_INITIALLY = 5,
    // Forces the vehicle's doors to shut and lock.
    VEHICLELOCK_FORCE_SHUT_DOORS = 6,
    // Vehicle is locked but can still be damaged.
    VEHICLELOCK_LOCKED_BUT_CAN_BE_DAMAGED = 7,
    // Vehicle is locked, but its trunk/boot remains unlocked.
    VEHICLELOCK_LOCKED_BUT_BOOT_UNLOCKED = 8,
    // Vehicle is locked and does not allow passengers, except for the driver.
    VEHICLELOCK_LOCKED_NO_PASSENGERS = 9,
    // Vehicle is completely locked, preventing entry entirely, even if previously inside.
    VEHICLELOCK_CANNOT_ENTER = 10 
};

```



pub fn set_vehicle_doors_locked_safe(
        
        
            vehicle: 
        , 
        
        
            doorLockStatus: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB664292EAECF7FA6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB664292EAECF7FA6u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorLockStatus
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_doors_locked_raw(
        vehicle: , 
        doorLockStatus: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB664292EAECF7FA6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB664292EAECF7FA6u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorLockStatus
        )
    }
}

/// Disables turret movement when called in a loop. You can still fire and aim. You cannot shoot backwards though.

```
NativeDB Introduced: v1365
```



pub fn _set_disable_turret_movement_this_frame_safe(
        
        
            vehicle: 
        , 
        
        
            turretIdx: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE615BB7A7752C76Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE615BB7A7752C76Au64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                turretIdx
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_disable_turret_movement_this_frame_raw(
        vehicle: , 
        turretIdx: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE615BB7A7752C76Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE615BB7A7752C76Au64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                turretIdx
        )
    }
}

/// ```
colorIndex = 0 - 7
```



pub fn _set_vehicle_parachute_texture_variation_safe(
        
        
            vehicle: 
        , 
        
        
            textureVariation: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA74AD2439468C883u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA74AD2439468C883u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                textureVariation
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_parachute_texture_variation_raw(
        vehicle: , 
        textureVariation: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA74AD2439468C883u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA74AD2439468C883u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                textureVariation
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_brake_lights_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92B35082E0B42F66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92B35082E0B42F66u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_brake_lights_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92B35082E0B42F66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92B35082E0B42F66u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_this_model_a_quadbike_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39DAC362EE65FA28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39DAC362EE65FA28u64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_this_model_a_quadbike_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39DAC362EE65FA28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39DAC362EE65FA28u64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// Gets the position of the cargobob hook, in world coords.



pub fn _get_cargobob_hook_position_safe(
        
        
            cargobob: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCBDB9B923CACC92Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCBDB9B923CACC92Du64;
        
        let result = invoke_raw!(
            hash,
                cargobob
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_cargobob_hook_position_raw(
        cargobob: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCBDB9B923CACC92Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCBDB9B923CACC92Du64;

        invoke_raw_typed!(
            hash,
                cargobob
        )
    }
}

/// ## Parameters
*



pub fn stabilise_entity_attached_to_heli_safe(
        
        
            vehicle: 
        , 
        
        
            entity: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x374706271354CB18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x374706271354CB18u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                entity, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stabilise_entity_attached_to_heli_raw(
        vehicle: , 
        entity: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x374706271354CB18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x374706271354CB18u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                entity, 
                p2
        )
    }
}

/// ```
Check if Vehicle Secondary is avaliable for customize  
```



pub fn get_is_vehicle_secondary_colour_custom_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x910A32E7AAD2656Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x910A32E7AAD2656Cu64;
        
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
pub fn get_is_vehicle_secondary_colour_custom_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x910A32E7AAD2656Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x910A32E7AAD2656Cu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Check if a entry point for a certain seat is clear, useful for checking if a vehicle seat is accesible.
If you park your vehicle near a wall and the ped cannot enter/exit this side, the return value toggles from true (not blocked) to false (blocked).

Keep in mind, with checkSide set to true, that only certain vehicles have entry points on both sides for the same seat, like motorcycles, most normal vehicles don't have this and if the native doesn't find a entry point with the given parameters it will always return false. So for most normal usecases leaving checkSide set to false would result in the expected behavior.



pub fn is_entry_point_for_seat_clear_safe(
        
        
            ped: 
        , 
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x639431E895B9AA57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x639431E895B9AA57u64;
        
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
pub fn is_entry_point_for_seat_clear_raw(
        ped: , 
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x639431E895B9AA57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x639431E895B9AA57u64;

        invoke_raw_typed!(
            hash,
                ped, 
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn get_last_ped_in_vehicle_seat_safe(
        
        
            vehicle: 
        , 
        
        
            seatIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83F969AA1EE2A664u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83F969AA1EE2A664u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                seatIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_last_ped_in_vehicle_seat_raw(
        vehicle: , 
        seatIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x83F969AA1EE2A664u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x83F969AA1EE2A664u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                seatIndex
        )
    }
}

/// ## Parameters
*



pub fn _0xd565f438137f0e10_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD565F438137F0E10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD565F438137F0E10u64;
        
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
pub fn _0xd565f438137f0e10_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD565F438137F0E10u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD565F438137F0E10u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn remove_vehicle_asset_safe(
        
        
            vehicleAsset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xACE699C71AB9DEB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xACE699C71AB9DEB5u64;
        
        let result = invoke_raw!(
            hash,
                vehicleAsset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_vehicle_asset_raw(
        vehicleAsset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xACE699C71AB9DEB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xACE699C71AB9DEB5u64;

        invoke_raw_typed!(
            hash,
                vehicleAsset
        )
    }
}

/// ## Parameters
*



pub fn set_disable_vehicle_engine_fires_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x91A0BD635321F145u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x91A0BD635321F145u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_disable_vehicle_engine_fires_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x91A0BD635321F145u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x91A0BD635321F145u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// See [`REQUEST_VEHICLE_RECORDING`](#_0xAF514CABE74CBF15).



pub fn get_vehicle_recording_id_safe(
        
        
            recording: 
        , 
        
        
            script: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21543C612379DB3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21543C612379DB3Cu64;
        
        let result = invoke_raw!(
            hash,
                recording, 
                script
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_recording_id_raw(
        recording: , 
        script: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x21543C612379DB3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x21543C612379DB3Cu64;

        invoke_raw_typed!(
            hash,
                recording, 
                script
        )
    }
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)



pub fn _set_vehicle_door_can_break_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        , 
        
        
            isBreakable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FA133A4A9D37ED8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FA133A4A9D37ED8u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex, 
                isBreakable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_door_can_break_raw(
        vehicle: , 
        doorIndex: , 
        isBreakable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FA133A4A9D37ED8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FA133A4A9D37ED8u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex, 
                isBreakable
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _set_vehicle_neon_lights_color_2_safe(
        
        
            vehicle: 
        , 
        
        
            color: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB93B2867F7B479D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB93B2867F7B479D1u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                color
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_neon_lights_color_2_raw(
        vehicle: , 
        color: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB93B2867F7B479D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB93B2867F7B479D1u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                color
        )
    }
}

/// ```
Returns false if every seat is occupied.  
```



pub fn are_any_vehicle_seats_free_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D34FC3BC4ADB780u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D34FC3BC4ADB780u64;
        
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
pub fn are_any_vehicle_seats_free_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D34FC3BC4ADB780u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D34FC3BC4ADB780u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Lowers the roof on a convertible vehicle, utilizing any available animations for the action. This native is particularly useful for creating a realistic interaction with convertible vehicles by animating the process of lowering the roof.

You can check if the vehicle has an convertible roof using [`IS_VEHICLE_A_CONVERTIBLE`](#_0x52F357A30698BCCE).

```
NativeDB Introduced: v323
```



pub fn lower_convertible_roof_safe(
        
        
            vehicle: 
        , 
        
        
            instantlyLower: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDED51F703D0FA83Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDED51F703D0FA83Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                instantlyLower
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn lower_convertible_roof_raw(
        vehicle: , 
        instantlyLower: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDED51F703D0FA83Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDED51F703D0FA83Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                instantlyLower
        )
    }
}

/// This native is used to simulate a high-speed impact for a vehicle when it collides with a breakable object (frag). It's particularly useful in scripted sequences where a vehicle is required to break through a barrier but might not actually be moving at a sufficient speed to do so realistically. Note that this setting is temporary and will reset after one frame, so it needs to be called every frame for a lasting effect.



pub fn set_vehicle_act_as_if_high_speed_for_frag_smashing_safe(
        
        
            vehicle: 
        , 
        
        
            actHighSpeed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F9FB66F3A3842D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F9FB66F3A3842D2u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                actHighSpeed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_act_as_if_high_speed_for_frag_smashing_raw(
        vehicle: , 
        actHighSpeed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F9FB66F3A3842D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F9FB66F3A3842D2u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                actHighSpeed
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_mod_color_1_safe(
        
        
            vehicle: 
        , 
        
        
            paintType: 
        , 
        
        
            color: 
        , 
        
        
            pearlescentColor: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8D65CA700C9A693u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8D65CA700C9A693u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                paintType, 
                color, 
                pearlescentColor
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_mod_color_1_raw(
        vehicle: , 
        paintType: , 
        color: , 
        pearlescentColor: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8D65CA700C9A693u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8D65CA700C9A693u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                paintType, 
                color, 
                pearlescentColor
        )
    }
}

/// ```
p1, p2, p3 are RGB values for color (255,0,0 for Red, ect)  
```



pub fn set_vehicle_custom_primary_colour_safe(
        
        
            vehicle: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7141766F91D15BEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7141766F91D15BEAu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn set_vehicle_custom_primary_colour_raw(
        vehicle: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7141766F91D15BEAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7141766F91D15BEAu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                r, 
                g, 
                b
        )
    }
}

/// Sets the specified door index shut on the passed vehicle.

```c
enum eDoorId
{
	VEH_EXT_DOOR_DSIDE_F = 0,
	VEH_EXT_DOOR_DSIDE_R = 1,
	VEH_EXT_DOOR_PSIDE_F = 2,
	VEH_EXT_DOOR_PSIDE_R = 3,
	VEH_EXT_BONNET = 4,
	VEH_EXT_BOOT = 5,
	// 0x872E72B8 = 0xFFFFFFFF,
}
```



pub fn set_vehicle_door_shut_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        , 
        
        
            closeInstantly: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93D9BD300D7789E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93D9BD300D7789E5u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex, 
                closeInstantly
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_door_shut_raw(
        vehicle: , 
        doorIndex: , 
        closeInstantly: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93D9BD300D7789E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93D9BD300D7789E5u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex, 
                closeInstantly
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_doors_locked_for_team_safe(
        
        
            vehicle: 
        , 
        
        
            team: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB81F6D4A8F5EEBA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB81F6D4A8F5EEBA8u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                team, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_doors_locked_for_team_raw(
        vehicle: , 
        team: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB81F6D4A8F5EEBA8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB81F6D4A8F5EEBA8u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                team, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_cargobob_pickup_magnet_pull_rope_length_safe(
        
        
            cargobob: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D8EAC07506291FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D8EAC07506291FBu64;
        
        let result = invoke_raw!(
            hash,
                cargobob, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cargobob_pickup_magnet_pull_rope_length_raw(
        cargobob: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D8EAC07506291FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D8EAC07506291FBu64;

        invoke_raw_typed!(
            hash,
                cargobob, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0xaf60e6a2936f982a_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF60E6A2936F982Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF60E6A2936F982Au64;
        
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
pub fn _0xaf60e6a2936f982a_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF60E6A2936F982Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF60E6A2936F982Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn is_this_model_a_train_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB935175B22E822Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB935175B22E822Bu64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_this_model_a_train_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB935175B22E822Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB935175B22E822Bu64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// Used to set the tornado custom (convertible) rooftop livery.

Livery value that works for tornado custom is between 0 and 9 from what i can tell. Maybe 0-8 even.

Might work on other custom vehicles but im not sure what those might be, only confirmed it working with the tornado custom.



pub fn _set_vehicle_roof_livery_safe(
        
        
            vehicle: 
        , 
        
        
            livery: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6D3A8750DC73270u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6D3A8750DC73270u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                livery
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_roof_livery_raw(
        vehicle: , 
        livery: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6D3A8750DC73270u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6D3A8750DC73270u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                livery
        )
    }
}

/// This native makes the vehicle stop immediately, as it happens when we enter a multiplayer garage.



pub fn bring_vehicle_to_halt_safe(
        
        
            vehicle: 
        , 
        
        
            distance: 
        , 
        
        
            duration: 
        , 
        
        
            bControlVerticalVelocity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x260BE8F09E326A20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x260BE8F09E326A20u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                distance, 
                duration, 
                bControlVerticalVelocity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn bring_vehicle_to_halt_raw(
        vehicle: , 
        distance: , 
        duration: , 
        bControlVerticalVelocity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x260BE8F09E326A20u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x260BE8F09E326A20u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                distance, 
                duration, 
                bControlVerticalVelocity
        )
    }
}

/// ```
NativeDB Introduced: v1493
```



pub fn _0x407dc5e97db1a4d3_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x407DC5E97DB1A4D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x407DC5E97DB1A4D3u64;
        
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
pub fn _0x407dc5e97db1a4d3_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x407DC5E97DB1A4D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x407DC5E97DB1A4D3u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn is_this_model_a_bicycle_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF94DD42F63BDED2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF94DD42F63BDED2u64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_this_model_a_bicycle_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF94DD42F63BDED2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF94DD42F63BDED2u64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// ```
NativeDB Introduced: v2060
```



pub fn _0xf8b49f5ba7f850e7_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8B49F5BA7F850E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8B49F5BA7F850E7u64;
        
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
pub fn _0xf8b49f5ba7f850e7_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF8B49F5BA7F850E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF8B49F5BA7F850E7u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
flags requires further research, e.g., 0x4/0x8 are related to the AI driving task and 0x20 is internally set and interacts with dynamic entity components.
time, often zero and capped at 500, is related to SET_PLAYBACK_TO_USE_AI_TRY_TO_REVERT_BACK_LATER
```



pub fn start_playback_recorded_vehicle_with_flags_safe(
        
        
            vehicle: 
        , 
        
        
            recording: 
        , 
        
        
            script: 
        , 
        
        
            flags: 
        , 
        
        
            time: 
        , 
        
        
            drivingStyle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D80FD645D4DA346u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D80FD645D4DA346u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                recording, 
                script, 
                flags, 
                time, 
                drivingStyle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_playback_recorded_vehicle_with_flags_raw(
        vehicle: , 
        recording: , 
        script: , 
        flags: , 
        time: , 
        drivingStyle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D80FD645D4DA346u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D80FD645D4DA346u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                recording, 
                script, 
                flags, 
                time, 
                drivingStyle
        )
    }
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)



pub fn get_vehicle_individual_door_lock_status_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA4AC3EAAE46EC7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA4AC3EAAE46EC7Bu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_individual_door_lock_status_raw(
        vehicle: , 
        doorIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA4AC3EAAE46EC7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA4AC3EAAE46EC7Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex
        )
    }
}

/// ## Parameters
*



pub fn _set_vehicle_handling_hash_for_ai_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10655FAB9915623Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10655FAB9915623Du64;
        
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
pub fn _set_vehicle_handling_hash_for_ai_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10655FAB9915623Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10655FAB9915623Du64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Sets the color of the neon lights on the specified vehicle.

RGB values and colour names taken from the decompiled scripts:

| Colour         |  R  |  G  |  B  |
|---------------|:---:|:---:|:---:|
| White         | 222 | 222 | 255 |
| Blue          | 2   | 21  | 255 |
| Electric Blue | 3   | 83  | 255 |
| Mint Green    | 0   | 255 | 140 |
| Lime Green    | 94  | 255 | 1   |
| Yellow        | 255 | 255 | 0   |
| Golden Shower | 255 | 150 | 0   |
| Orange        | 255 | 62  | 0   |
| Red           | 255 | 1   | 1   |
| Pony Pink     | 255 | 50  | 100 |
| Hot Pink      | 255 | 5   | 190 |
| Purple        | 35  | 1   | 255 |
| Blacklight    | 15  | 3   | 255 |



pub fn _set_vehicle_neon_lights_colour_safe(
        
        
            vehicle: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E0A582209A62695u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E0A582209A62695u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn _set_vehicle_neon_lights_colour_raw(
        vehicle: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8E0A582209A62695u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8E0A582209A62695u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                r, 
                g, 
                b
        )
    }
}

/// ```
NativeDB Introduced: v3407
```



pub fn set_plane_control_sections_should_break_off_from_explosions_safe(
        
        
            plane: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD8A2D3337F04196u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD8A2D3337F04196u64;
        
        let result = invoke_raw!(
            hash,
                plane, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_plane_control_sections_should_break_off_from_explosions_raw(
        plane: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD8A2D3337F04196u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD8A2D3337F04196u64;

        invoke_raw_typed!(
            hash,
                plane, 
                toggle
        )
    }
}

/// Sets whether a boat should remain anchored even when a player is driving it.



pub fn set_boat_remains_anchored_while_player_is_driver_safe(
        
        
            boat: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3EBAAE484798530u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3EBAAE484798530u64;
        
        let result = invoke_raw!(
            hash,
                boat, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_boat_remains_anchored_while_player_is_driver_raw(
        boat: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3EBAAE484798530u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3EBAAE484798530u64;

        invoke_raw_typed!(
            hash,
                boat, 
                toggle
        )
    }
}

/// ```
GET_H*

NativeDB Introduced: v1604
```



pub fn _0xe8718faf591fd224_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8718FAF591FD224u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8718FAF591FD224u64;
        
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
pub fn _0xe8718faf591fd224_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE8718FAF591FD224u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE8718FAF591FD224u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _attach_container_to_handler_frame_safe(
        
        
            handler: 
        , 
        
        
            container: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A98C2ECF57FA5D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A98C2ECF57FA5D4u64;
        
        let result = invoke_raw!(
            hash,
                handler, 
                container
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _attach_container_to_handler_frame_raw(
        handler: , 
        container: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A98C2ECF57FA5D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A98C2ECF57FA5D4u64;

        invoke_raw_typed!(
            hash,
                handler, 
                container
        )
    }
}

/// ```
SCALE: Setting the speed to 30 would result in a speed of roughly 60mph, according to speedometer.  
Speed is in meters per second  
You can convert meters/s to mph here:  
http://www.calculateme.com/Speed/MetersperSecond/ToMilesperHour.htm  
```



pub fn set_vehicle_forward_speed_safe(
        
        
            vehicle: 
        , 
        
        
            speed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB54A438726D25D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB54A438726D25D5u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                speed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_forward_speed_raw(
        vehicle: , 
        speed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB54A438726D25D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB54A438726D25D5u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                speed
        )
    }
}

/// ```
Sets vehicle wheel hydraulic states transition. Known states:
0 - reset
1 - raise wheel (uses value arg, works just like _SET_VEHICLE_HYDRAULIC_WHEEL_VALUE)
2 - jump using wheel
```



pub fn _set_hydraulic_wheel_state_transition_safe(
        
        
            vehicle: 
        , 
        
        
            wheelId: 
        , 
        
        
            state: 
        , 
        
        
            value: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC24075310A8B9CD1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC24075310A8B9CD1u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                wheelId, 
                state, 
                value, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_hydraulic_wheel_state_transition_raw(
        vehicle: , 
        wheelId: , 
        state: , 
        value: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC24075310A8B9CD1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC24075310A8B9CD1u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                wheelId, 
                state, 
                value, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn are_plane_propellers_intact_safe(
        
        
            plane: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x755D6D5267CBBD7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x755D6D5267CBBD7Eu64;
        
        let result = invoke_raw!(
            hash,
                plane
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn are_plane_propellers_intact_raw(
        plane: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x755D6D5267CBBD7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x755D6D5267CBBD7Eu64;

        invoke_raw_typed!(
            hash,
                plane
        )
    }
}

/// This native is used to latch or unlatch the convertible roof of a vehicle. It allows for direct control over the roof's mechanism without actually opening or closing the roof. This can be useful for scenarios where you need to prepare a vehicle's roof to be opened or closed by another action or to ensure it remains fixed in its current state regardless of other interactions.

```
NativeDB Introduced: v323
```



pub fn set_convertible_roof_latch_state_safe(
        
        
            vehicle: 
        , 
        
        
            bLatched: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A78AD3D8240536Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A78AD3D8240536Fu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                bLatched
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_convertible_roof_latch_state_raw(
        vehicle: , 
        bLatched: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A78AD3D8240536Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A78AD3D8240536Fu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                bLatched
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _set_tyre_health_safe(
        
        
            vehicle: 
        , 
        
        
            wheelIndex: 
        , 
        
        
            health: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74C68EF97645E79Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74C68EF97645E79Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                wheelIndex, 
                health
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_tyre_health_raw(
        vehicle: , 
        wheelIndex: , 
        health: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74C68EF97645E79Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74C68EF97645E79Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                wheelIndex, 
                health
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_colours_safe(
        
        
            vehicle: 
        , 
        
        
            colorPrimary: 
        , 
        
        
            colorSecondary: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA19435F193E081ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA19435F193E081ACu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                colorPrimary, 
                colorSecondary
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_colours_raw(
        vehicle: , 
        colorPrimary: , 
        colorSecondary: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA19435F193E081ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA19435F193E081ACu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                colorPrimary, 
                colorSecondary
        )
    }
}

/// ```
NativeDB Introduced: v3407
```



pub fn _is_vehicle_on_boost_pad_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC40CBF7B90CA77Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC40CBF7B90CA77Cu64;
        
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
pub fn _is_vehicle_on_boost_pad_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC40CBF7B90CA77Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC40CBF7B90CA77Cu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _get_has_rocket_boost_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36D782F68B309BDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36D782F68B309BDAu64;
        
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
pub fn _get_has_rocket_boost_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36D782F68B309BDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36D782F68B309BDAu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Time is number of milliseconds before reverting, zero for indefinitely.
```



pub fn set_playback_to_use_ai_try_to_revert_back_later_safe(
        
        
            vehicle: 
        , 
        
        
            time: 
        , 
        
        
            drivingStyle: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E63860BBB190730u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E63860BBB190730u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                time, 
                drivingStyle, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_playback_to_use_ai_try_to_revert_back_later_raw(
        vehicle: , 
        time: , 
        drivingStyle: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E63860BBB190730u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E63860BBB190730u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                time, 
                drivingStyle, 
                p3
        )
    }
}

/// Determines if the submarine is operating below its designated crush depth.

```
NativeDB Introduced: v2189
```



pub fn get_submarine_is_under_design_depth_safe(
        
        
            submarine: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E71D0B300B7AA79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E71D0B300B7AA79u64;
        
        let result = invoke_raw!(
            hash,
                submarine
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_submarine_is_under_design_depth_raw(
        submarine: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E71D0B300B7AA79u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E71D0B300B7AA79u64;

        invoke_raw_typed!(
            hash,
                submarine
        )
    }
}

/// ```
Note: only some vehicle have extras  
extra ids are from 1 - 9 depending on the vehicle



pub fn set_vehicle_extra_safe(
        
        
            vehicle: 
        , 
        
        
            extraId: 
        , 
        
        
            disable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EE3A3C5E4A40CC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EE3A3C5E4A40CC9u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                extraId, 
                disable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_extra_raw(
        vehicle: , 
        extraId: , 
        disable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7EE3A3C5E4A40CC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7EE3A3C5E4A40CC9u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                extraId, 
                disable
        )
    }
}

/// ## Parameters
*



pub fn set_trailer_inverse_mass_scale_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A8F319B392E7B3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A8F319B392E7B3Fu64;
        
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
pub fn set_trailer_inverse_mass_scale_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A8F319B392E7B3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A8F319B392E7B3Fu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// Incorrectly named `SET_VEHICLE_EXCLUSIVE_DRIVER`; likely `SET_VEHICLE_ALLOW_*`.

Toggles a flag related to `SET_VEHICLE_EXCLUSIVE_DRIVER`, however, doesn't enable that feature (or trigger script events related to it).

See [`_SET_VEHICLE_EXCLUSIVE_DRIVER_2`](#_0xB5C51B5502E85E83).

```
NativeDB Removed Parameter 2: int index
```



pub fn set_vehicle_exclusive_driver_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41062318F23ED854u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41062318F23ED854u64;
        
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
pub fn set_vehicle_exclusive_driver_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41062318F23ED854u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41062318F23ED854u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Toggles specific flag on turret
```

```
NativeDB Introduced: v1290
```



pub fn _set_vehicle_turret_unk_safe(
        
        
            vehicle: 
        , 
        
        
            index: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC60060EB0D8AC7B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC60060EB0D8AC7B1u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                index, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_turret_unk_raw(
        vehicle: , 
        index: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC60060EB0D8AC7B1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC60060EB0D8AC7B1u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                index, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn has_preload_mods_finished_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06F43E5175EB6D96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06F43E5175EB6D96u64;
        
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
pub fn has_preload_mods_finished_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06F43E5175EB6D96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06F43E5175EB6D96u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn _0x737e398138550fff_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x737E398138550FFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x737E398138550FFFu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x737e398138550fff_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x737E398138550FFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x737E398138550FFFu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Similar to [`_GET_AIRCRAFT_BOMB_COUNT`](#_0xEA12BD130D7569A1), this gets the amount of countermeasures that are present on this vehicle.

Use [`_SET_AIRCRAFT_COUNTERMEASURE_COUNT`](#_0x9BDA23BF666F0855) to set the current amount.



pub fn _get_vehicle_countermeasure_count_safe(
        
        
            aircraft: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF846AA63DF56B804u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF846AA63DF56B804u64;
        
        let result = invoke_raw!(
            hash,
                aircraft
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_vehicle_countermeasure_count_raw(
        aircraft: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF846AA63DF56B804u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF846AA63DF56B804u64;

        invoke_raw_typed!(
            hash,
                aircraft
        )
    }
}

/// _0xF25E02CB9C5818F8 native function



pub fn _0xf25e02cb9c5818f8_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF25E02CB9C5818F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF25E02CB9C5818F8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xf25e02cb9c5818f8_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF25E02CB9C5818F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF25E02CB9C5818F8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
if true, axles won't bend.  
```



pub fn set_vehicle_has_strong_axles_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92F0CF722BC4202Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92F0CF722BC4202Fu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_has_strong_axles_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92F0CF722BC4202Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92F0CF722BC4202Fu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _0x430a7631a84c9be7_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x430A7631A84C9BE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x430A7631A84C9BE7u64;
        
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
pub fn _0x430a7631a84c9be7_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x430A7631A84C9BE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x430A7631A84C9BE7u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// SET_ALL_VEHICLE_GENERATORS_ACTIVE native function



pub fn set_all_vehicle_generators_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34AD89078831A4BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34AD89078831A4BCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_all_vehicle_generators_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x34AD89078831A4BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x34AD89078831A4BCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
The only example I can find of this function in the scripts, is this:  
struct _s = VEHICLE::GET_VEHICLE_DEFORMATION_AT_POS(rPtr((A_0) + 4), 1.21f, 6.15f, 0.3f);



pub fn get_vehicle_deformation_at_pos_safe(
        
        
            vehicle: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4EC6CFBC7B2E9536u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4EC6CFBC7B2E9536u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn get_vehicle_deformation_at_pos_raw(
        vehicle: , 
        offsetX: , 
        offsetY: , 
        offsetZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4EC6CFBC7B2E9536u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4EC6CFBC7B2E9536u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                offsetX, 
                offsetY, 
                offsetZ
        )
    }
}

/// DETONATE_VEHICLE_PHONE_EXPLOSIVE_DEVICE native function



pub fn detonate_vehicle_phone_explosive_device_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF49CF0270307CBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF49CF0270307CBEu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn detonate_vehicle_phone_explosive_device_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF49CF0270307CBEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF49CF0270307CBEu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0x9849de24fcf23ccc_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9849DE24FCF23CCCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9849DE24FCF23CCCu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x9849de24fcf23ccc_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9849DE24FCF23CCCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9849DE24FCF23CCCu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
returns a string which is the codename of the vehicle's currently selected secondary color  
```



pub fn get_vehicle_mod_color_2_name_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4967A516ED23A5A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4967A516ED23A5A1u64;
        
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
pub fn get_vehicle_mod_color_2_name_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4967A516ED23A5A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4967A516ED23A5A1u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Returns whether the outrigger legs are deployed for the vehicle.
The Chernobog is one of the few vehicles with outrigger legs.

```
NativeDB Introduced: v1290
```



pub fn _are_outrigger_legs_deployed_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A9128352EAC9E85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A9128352EAC9E85u64;
        
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
pub fn _are_outrigger_legs_deployed_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A9128352EAC9E85u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A9128352EAC9E85u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
NativeDB Introduced: 3095
```

Determines if the nitrous is currently activated in the specified vehicle.



pub fn is_nitrous_active_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x491E822B2C464FE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x491E822B2C464FE4u64;
        
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
pub fn is_nitrous_active_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x491E822B2C464FE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x491E822B2C464FE4u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Sets the neon lights of the specified vehicle on/off.  
Indices:  
0 = Left  
1 = Right  
2 = Front  
3 = Back  
```



pub fn _set_vehicle_neon_light_enabled_safe(
        
        
            vehicle: 
        , 
        
        
            index: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2AA720E4287BF269u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2AA720E4287BF269u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                index, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_neon_light_enabled_raw(
        vehicle: , 
        index: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2AA720E4287BF269u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2AA720E4287BF269u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                index, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0x065d03a9d6b2c6b5_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x065D03A9D6B2C6B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x065D03A9D6B2C6B5u64;
        
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
pub fn _0x065d03a9d6b2c6b5_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x065D03A9D6B2C6B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x065D03A9D6B2C6B5u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Returns true only when the magnet is active, will return false if the hook is active  
```



pub fn does_cargobob_have_pickup_magnet_safe(
        
        
            cargobob: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E08BF5B3722BAC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E08BF5B3722BAC9u64;
        
        let result = invoke_raw!(
            hash,
                cargobob
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_cargobob_have_pickup_magnet_raw(
        cargobob: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E08BF5B3722BAC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E08BF5B3722BAC9u64;

        invoke_raw_typed!(
            hash,
                cargobob
        )
    }
}

/// ```
Scripts verify that towTruck is the first parameter, not the second.  
```



pub fn is_vehicle_attached_to_tow_truck_safe(
        
        
            towTruck: 
        , 
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x146DF9EC4C4B9FD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x146DF9EC4C4B9FD4u64;
        
        let result = invoke_raw!(
            hash,
                towTruck, 
                vehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_attached_to_tow_truck_raw(
        towTruck: , 
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x146DF9EC4C4B9FD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x146DF9EC4C4B9FD4u64;

        invoke_raw_typed!(
            hash,
                towTruck, 
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_players_last_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCDF8BAF56C87B6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCDF8BAF56C87B6Au64;
        
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
pub fn set_players_last_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCDF8BAF56C87B6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCDF8BAF56C87B6Au64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
The inner function has a switch on the second parameter. It's the stuck timer index.  
Here's some pseudo code I wrote for the inner function:  
void __fastcall NATIVE_RESET_VEHICLE_STUCK_TIMER_INNER(CUnknown* unknownClassInVehicle, int timerIndex)  
{  
	switch (timerIndex)  
	{  
	case 0:  
unknownClassInVehicle->FirstStuckTimer = (WORD)0u;  
	case 1:  
unknownClassInVehicle->SecondStuckTimer = (WORD)0u;  
	case 2:  
unknownClassInVehicle->ThirdStuckTimer = (WORD)0u;  
	case 3:  
unknownClassInVehicle->FourthStuckTimer = (WORD)0u;  
	case 4:  
unknownClassInVehicle->FirstStuckTimer = (WORD)0u;  
unknownClassInVehicle->SecondStuckTimer = (WORD)0u;  
unknownClassInVehicle->ThirdStuckTimer = (WORD)0u;  
unknownClassInVehicle->FourthStuckTimer = (WORD)0u;  
break;  
	};  
}  
```



pub fn reset_vehicle_stuck_timer_safe(
        
        
            vehicle: 
        , 
        
        
            nullAttributes: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7591B0065AFAA7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7591B0065AFAA7Au64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                nullAttributes
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_vehicle_stuck_timer_raw(
        vehicle: , 
        nullAttributes: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD7591B0065AFAA7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD7591B0065AFAA7Au64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                nullAttributes
        )
    }
}

/// ## Parameters
*



pub fn _0xf051d9bfb6ba39c0_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF051D9BFB6BA39C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF051D9BFB6BA39C0u64;
        
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
pub fn _0xf051d9bfb6ba39c0_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF051D9BFB6BA39C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF051D9BFB6BA39C0u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn attach_vehicle_to_trailer_safe(
        
        
            vehicle: 
        , 
        
        
            trailer: 
        , 
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C7D42D58F770B54u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C7D42D58F770B54u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                trailer, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn attach_vehicle_to_trailer_raw(
        vehicle: , 
        trailer: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C7D42D58F770B54u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C7D42D58F770B54u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                trailer, 
                radius
        )
    }
}

/// Sets whether the trailer can attach to vehicles



pub fn set_trailer_attachment_enabled_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FA2494B47FDD009u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FA2494B47FDD009u64;
        
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
pub fn set_trailer_attachment_enabled_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FA2494B47FDD009u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FA2494B47FDD009u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _0x66e3aaface2d1eb8_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66E3AAFACE2D1EB8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66E3AAFACE2D1EB8u64;
        
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
pub fn _0x66e3aaface2d1eb8_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66E3AAFACE2D1EB8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66E3AAFACE2D1EB8u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// This native sets the turbulence multiplier. It only works for planes.
0.0 = no turbulence at all.
1.0 = heavy turbulence.

Works by just calling it once, does not need to be called every tick.



pub fn set_plane_turbulence_multiplier_safe(
        
        
            vehicle: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD2D28A1AFDFF131u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD2D28A1AFDFF131u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_plane_turbulence_multiplier_raw(
        vehicle: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD2D28A1AFDFF131u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD2D28A1AFDFF131u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                multiplier
        )
    }
}

/// Changes the key used to transform a vehicle into submarine mode. When set to true, the transformation key switches from the default raise/lower convertible roof key (usually 'H') to the special vehicle transformation key (usually 'X').

```
NativeDB Introduced: v1365
```



pub fn set_transform_to_submarine_uses_alternate_input_safe(
        
        
            vehicle: 
        , 
        
        
            useAlternateInput: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41B9FB92EDED32A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41B9FB92EDED32A6u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                useAlternateInput
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_transform_to_submarine_uses_alternate_input_raw(
        vehicle: , 
        useAlternateInput: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41B9FB92EDED32A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41B9FB92EDED32A6u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                useAlternateInput
        )
    }
}

/// ## Parameters
*



pub fn have_vehicle_mods_streamed_in_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A83F5F9963775EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A83F5F9963775EFu64;
        
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
pub fn have_vehicle_mods_streamed_in_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A83F5F9963775EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A83F5F9963775EFu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn detach_container_from_handler_frame_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C0043FDFF6436BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C0043FDFF6436BCu64;
        
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
pub fn detach_container_from_handler_frame_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C0043FDFF6436BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C0043FDFF6436BCu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn remove_vehicle_mod_safe(
        
        
            vehicle: 
        , 
        
        
            modType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92D619E420858204u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92D619E420858204u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                modType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_vehicle_mod_raw(
        vehicle: , 
        modType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92D619E420858204u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92D619E420858204u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                modType
        )
    }
}

/// ```
Outputs 2 Vector3's.
Scripts check if out2.x - out1.x > something.x
Could be suspension related, as in max suspension height and min suspension height, considering the natives location.
```



pub fn _get_vehicle_suspension_bounds_safe(
        
        
            vehicle: 
        , 
        
        
            out1: 
        , 
        
        
            out2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF7E3EEB29642C38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF7E3EEB29642C38u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                out1, 
                out2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_vehicle_suspension_bounds_raw(
        vehicle: , 
        out1: , 
        out2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF7E3EEB29642C38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF7E3EEB29642C38u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                out1, 
                out2
        )
    }
}

/// See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).



pub fn roll_down_window_safe(
        
        
            vehicle: 
        , 
        
        
            windowIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AD9E6CE657D69E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AD9E6CE657D69E3u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                windowIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn roll_down_window_raw(
        vehicle: , 
        windowIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7AD9E6CE657D69E3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7AD9E6CE657D69E3u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                windowIndex
        )
    }
}

/// ## Parameters
*



pub fn _set_vehicle_ramp_sideways_launch_motion_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1BBAC99C0BC53656u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1BBAC99C0BC53656u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_ramp_sideways_launch_motion_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1BBAC99C0BC53656u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1BBAC99C0BC53656u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_can_leak_oil_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51BB2D88D31A914Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51BB2D88D31A914Bu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_can_leak_oil_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x51BB2D88D31A914Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x51BB2D88D31A914Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
formerly known as _GET_VEHICLE_PAINT_FADE
The result is a value from 0-1, where 0 is fresh paint.
```



pub fn get_vehicle_enveff_scale_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA82819CAC9C4C403u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA82819CAC9C4C403u64;
        
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
pub fn get_vehicle_enveff_scale_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA82819CAC9C4C403u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA82819CAC9C4C403u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0x8664170ef165c4a6_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8664170EF165C4A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8664170EF165C4A6u64;
        
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
pub fn _0x8664170ef165c4a6_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8664170EF165C4A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8664170EF165C4A6u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn disable_plane_aileron_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23428FC53C60919Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23428FC53C60919Cu64;
        
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
pub fn disable_plane_aileron_raw(
        vehicle: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x23428FC53C60919Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x23428FC53C60919Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1, 
                p2
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _0x2310a8f9421ebf43_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2310A8F9421EBF43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2310A8F9421EBF43u64;
        
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
pub fn _0x2310a8f9421ebf43_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2310A8F9421EBF43u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2310A8F9421EBF43u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v3407
```



pub fn _set_vehicle_explosive_damage_scale_safe(
        
        
            vehicle: 
        , 
        
        
            scale: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84D7FFD223CAAFFDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84D7FFD223CAAFFDu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                scale
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_explosive_damage_scale_raw(
        vehicle: , 
        scale: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84D7FFD223CAAFFDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84D7FFD223CAAFFDu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                scale
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_can_break_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59BF8C3D52C92F66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59BF8C3D52C92F66u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_can_break_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x59BF8C3D52C92F66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x59BF8C3D52C92F66u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
Stops the cargobob from being able to attach any vehicle
```

```
NativeDB Introduced: v1180
```



pub fn _set_cargobob_hook_can_attach_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94A68DA412C4007Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94A68DA412C4007Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_cargobob_hook_can_attach_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94A68DA412C4007Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94A68DA412C4007Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn add_vehicle_upsidedown_check_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB72E26D81006005Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB72E26D81006005Bu64;
        
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
pub fn add_vehicle_upsidedown_check_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB72E26D81006005Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB72E26D81006005Bu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_siren_audio_on_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5CC40FBCB586380u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5CC40FBCB586380u64;
        
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
pub fn is_vehicle_siren_audio_on_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5CC40FBCB586380u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5CC40FBCB586380u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn force_submarine_surface_mode_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33506883545AC0DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33506883545AC0DFu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_submarine_surface_mode_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33506883545AC0DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33506883545AC0DFu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_random_vehicle_density_multiplier_this_frame_safe(
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3B3359379FE77D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3B3359379FE77D3u64;
        
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
pub fn set_random_vehicle_density_multiplier_this_frame_raw(
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3B3359379FE77D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3B3359379FE77D3u64;

        invoke_raw_typed!(
            hash,
                multiplier
        )
    }
}

/// Vehicle must be a plane.
Native name is between SET_VEHICLE_BRAKE_LIGHTS and SET_VEHICLE_BULLDOZER_ARM_POSITION alphabetically.



pub fn _0xc361aa040d6637a8_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC361AA040D6637A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC361AA040D6637A8u64;
        
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
pub fn _0xc361aa040d6637a8_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC361AA040D6637A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC361AA040D6637A8u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_wheels_can_break_safe(
        
        
            vehicle: 
        , 
        
        
            enabled: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29B18B4FD460CA8Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29B18B4FD460CA8Fu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                enabled
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_wheels_can_break_raw(
        vehicle: , 
        enabled: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29B18B4FD460CA8Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29B18B4FD460CA8Fu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                enabled
        )
    }
}

/// ## Parameters
*



pub fn is_vehicle_mod_gen9_exclusive_safe(
        
        
            vehicle: 
        , 
        
        
            modType: 
        , 
        
        
            modIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00834EAC4A96E010u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00834EAC4A96E010u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                modType, 
                modIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_mod_gen9_exclusive_raw(
        vehicle: , 
        modType: , 
        modIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00834EAC4A96E010u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00834EAC4A96E010u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                modType, 
                modIndex
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x8821196d91fa2de5_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8821196D91FA2DE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8821196D91FA2DE5u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8821196d91fa2de5_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8821196D91FA2DE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8821196D91FA2DE5u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
SET_VEHICLE_*
```



pub fn _0x76d26a22750e849e_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76D26A22750E849Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76D26A22750E849Eu64;
        
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
pub fn _0x76d26a22750e849e_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76D26A22750E849Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76D26A22750E849Eu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn attach_entity_to_cargobob_safe(
        
        
            vehicle: 
        , 
        
        
            entity: 
        , 
        
        
            p2: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1DD82F3CCF9A01Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1DD82F3CCF9A01Eu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                entity, 
                p2, 
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
pub fn attach_entity_to_cargobob_raw(
        vehicle: , 
        entity: , 
        p2: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1DD82F3CCF9A01Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1DD82F3CCF9A01Eu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                entity, 
                p2, 
                x, 
                y, 
                z
        )
    }
}

/// ```
Not present in the retail version! It's just a nullsub.  
p0 always true (except in one case)  
p1 a random vehicle hash loaded in memory  
successIndicator: 0 if success, -1 if failed
```



pub fn get_random_vehicle_model_in_memory_safe(
        
        
            p0: 
        , 
        
        
            modelHash: 
        , 
        
        
            successIndicator: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x055BF0AC0C34F4FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x055BF0AC0C34F4FDu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                modelHash, 
                successIndicator
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_random_vehicle_model_in_memory_raw(
        p0: , 
        modelHash: , 
        successIndicator: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x055BF0AC0C34F4FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x055BF0AC0C34F4FDu64;

        invoke_raw_typed!(
            hash,
                p0, 
                modelHash, 
                successIndicator
        )
    }
}

/// Paint index goes from 0 to 12.

You can find the list of colors and ids here: [_GET_VEHICLE_HEADLIGHTS_COLOUR](#_0x3DFF319A831E0CDB)



pub fn _set_vehicle_xenon_lights_color_safe(
        
        
            vehicle: 
        , 
        
        
            color: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE41033B25D003A07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE41033B25D003A07u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                color
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_xenon_lights_color_raw(
        vehicle: , 
        color: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE41033B25D003A07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE41033B25D003A07u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                color
        )
    }
}

/// ## Parameters
*



pub fn set_all_low_priority_vehicle_generators_active_safe(
        
        
            active: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x608207E7A8FB787Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x608207E7A8FB787Cu64;
        
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
pub fn set_all_low_priority_vehicle_generators_active_raw(
        active: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x608207E7A8FB787Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x608207E7A8FB787Cu64;

        invoke_raw_typed!(
            hash,
                active
        )
    }
}

/// ```
Request the vehicle recording defined by the lowercase format string "%s%03d.yvr". For example, REQUEST_VEHICLE_RECORDING(1, "FBIs1UBER") corresponds to fbis1uber001.yvr.
For all vehicle recording/playback natives, "script" is a common prefix that usually corresponds to the script/mission the recording is used in, "recording" is its int suffix, and "id" (e.g., in native GET_TOTAL_DURATION_OF_VEHICLE_RECORDING_ID) corresponds to a unique identifier within the recording streaming module.
Note that only 24 recordings (hardcoded in multiple places) can ever active at a given time before clobbering begins.
```



pub fn request_vehicle_recording_safe(
        
        
            recording: 
        , 
        
        
            script: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF514CABE74CBF15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF514CABE74CBF15u64;
        
        let result = invoke_raw!(
            hash,
                recording, 
                script
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_vehicle_recording_raw(
        recording: , 
        script: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF514CABE74CBF15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF514CABE74CBF15u64;

        invoke_raw_typed!(
            hash,
                recording, 
                script
        )
    }
}

/// This method is utilized solely for debugging purposes and is functional only in debug builds of the game. Please note that its functionality may not be available in the retail version.



pub fn set_vehicle_name_debug_safe(
        
        
            vehicle: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFDF984E2C22B94Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFDF984E2C22B94Fu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_name_debug_raw(
        vehicle: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFDF984E2C22B94Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFDF984E2C22B94Fu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                name
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _set_disable_vehicle_unk_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x143921E45EC44D62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x143921E45EC44D62u64;
        
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
pub fn _set_disable_vehicle_unk_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x143921E45EC44D62u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x143921E45EC44D62u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Possibly: Returns whether the searchlight (found on police vehicles) is toggled on.  
```



pub fn is_vehicle_searchlight_on_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0F97FCE55094987u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0F97FCE55094987u64;
        
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
pub fn is_vehicle_searchlight_on_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0F97FCE55094987u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0F97FCE55094987u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
They use the same color indexs as SET_VEHICLE_COLOURS.  
```



pub fn set_vehicle_extra_colours_safe(
        
        
            vehicle: 
        , 
        
        
            pearlescentColor: 
        , 
        
        
            wheelColor: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2036F561ADD12E33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2036F561ADD12E33u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                pearlescentColor, 
                wheelColor
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_extra_colours_raw(
        vehicle: , 
        pearlescentColor: , 
        wheelColor: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2036F561ADD12E33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2036F561ADD12E33u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                pearlescentColor, 
                wheelColor
        )
    }
}

/// ## Parameters
*



pub fn add_vehicle_combat_angled_avoidance_area_safe(
        
        
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
        let hash = 0x54B0F614960F4A5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54B0F614960F4A5Fu64;
        
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
pub fn add_vehicle_combat_angled_avoidance_area_raw(
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
        let hash = 0x54B0F614960F4A5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x54B0F614960F4A5Fu64;

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

/// Train models must be [requested](#_0x963D27A58DF860AC) before use. See trains.xml (located in `Grand Theft Auto V\update\update.rpf\common\data\levels\gta5\trains.xml`) for freight and metro variations.

Model names to request can be found by searching `model_name` in the file.

The `Lua` usage example provided down below has been provided in such way so users can test each and every train variation.



pub fn create_mission_train_safe(
        
        
            variation: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            direction: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63C6CCA8E68AE8C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63C6CCA8E68AE8C8u64;
        
        let result = invoke_raw!(
            hash,
                variation, 
                x, 
                y, 
                z, 
                direction
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_mission_train_raw(
        variation: , 
        x: , 
        y: , 
        z: , 
        direction: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63C6CCA8E68AE8C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63C6CCA8E68AE8C8u64;

        invoke_raw_typed!(
            hash,
                variation, 
                x, 
                y, 
                z, 
                direction
        )
    }
}

/// ```
Creates a script vehicle generator at the given coordinates. Most parameters after the model hash are unknown.  
Parameters:  
a/w/s - Generator position  
heading - Generator heading  
p4 - Unknown (always 5.0)  
p5 - Unknown (always 3.0)  
modelHash - Vehicle model hash  
p7/8/9/10 - Unknown (always -1)  
p11 - Unknown (usually TRUE, only one instance of FALSE)  
p12/13 - Unknown (always FALSE)  
p14 - Unknown (usally FALSE, only two instances of TRUE)  
p15 - Unknown (always TRUE)  
p16 - Unknown (always -1)  
Vector3 coords = GET_ENTITY_COORDS(PLAYER_PED_ID(), 0);	CREATE_SCRIPT_VEHICLE_GENERATOR(coords.x, coords.y, coords.z, 1.0f, 5.0f, 3.0f, GET_HASH_KEY("adder"), -1. -1, -1, -1, -1, true, false, false, false, true, -1);  
```



pub fn create_script_vehicle_generator_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            heading: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        , 
        
        
            modelHash: 
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
        , 
        
        
            p14: 
        , 
        
        
            p15: 
        , 
        
        
            p16: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DEF883114668116u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DEF883114668116u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                heading, 
                p4, 
                p5, 
                modelHash, 
                p7, 
                p8, 
                p9, 
                p10, 
                p11, 
                p12, 
                p13, 
                p14, 
                p15, 
                p16
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_script_vehicle_generator_raw(
        x: , 
        y: , 
        z: , 
        heading: , 
        p4: , 
        p5: , 
        modelHash: , 
        p7: , 
        p8: , 
        p9: , 
        p10: , 
        p11: , 
        p12: , 
        p13: , 
        p14: , 
        p15: , 
        p16: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DEF883114668116u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DEF883114668116u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                heading, 
                p4, 
                p5, 
                modelHash, 
                p7, 
                p8, 
                p9, 
                p10, 
                p11, 
                p12, 
                p13, 
                p14, 
                p15, 
                p16
        )
    }
}

/// ## Parameters
*



pub fn _is_vehicle_rocket_boost_active_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D34E80EED4AE3BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D34E80EED4AE3BEu64;
        
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
pub fn _is_vehicle_rocket_boost_active_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3D34E80EED4AE3BEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3D34E80EED4AE3BEu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Affects the playback speed of the submarine car conversion animations. Does not affect hardcoded animations such as the wheels being retracted. In decompiled scripts the only value used for transformRate is 2.5.



pub fn set_transform_rate_for_animation_safe(
        
        
            vehicle: 
        , 
        
        
            transformRate: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x498218259FB7C72Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x498218259FB7C72Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                transformRate
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_transform_rate_for_animation_raw(
        vehicle: , 
        transformRate: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x498218259FB7C72Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x498218259FB7C72Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                transformRate
        )
    }
}

/// ```
Allows you to toggle bulletproof tires.  
```



pub fn set_vehicle_tyres_can_burst_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB9DC3C7D8596C46u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB9DC3C7D8596C46u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_tyres_can_burst_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB9DC3C7D8596C46u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB9DC3C7D8596C46u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// This native allows opening or closing the wings of the Deluxo/Oppressor. For the Deluxo, wing deployment depends on sufficient altitude.



pub fn set_hover_mode_wing_ratio_safe(
        
        
            vehicle: 
        , 
        
        
            ratio: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70A252F60A3E036Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70A252F60A3E036Bu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                ratio
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_hover_mode_wing_ratio_raw(
        vehicle: , 
        ratio: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70A252F60A3E036Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70A252F60A3E036Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                ratio
        )
    }
}

/// ```
HookOffset defines where the hook is attached. leave at 0 for default attachment.
```



pub fn attach_vehicle_to_tow_truck_safe(
        
        
            towTruck: 
        , 
        
        
            vehicle: 
        , 
        
        
            rear: 
        , 
        
        
            hookOffsetX: 
        , 
        
        
            hookOffsetY: 
        , 
        
        
            hookOffsetZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29A16F8D621C4508u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29A16F8D621C4508u64;
        
        let result = invoke_raw!(
            hash,
                towTruck, 
                vehicle, 
                rear, 
                hookOffsetX, 
                hookOffsetY, 
                hookOffsetZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn attach_vehicle_to_tow_truck_raw(
        towTruck: , 
        vehicle: , 
        rear: , 
        hookOffsetX: , 
        hookOffsetY: , 
        hookOffsetZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29A16F8D621C4508u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29A16F8D621C4508u64;

        invoke_raw_typed!(
            hash,
                towTruck, 
                vehicle, 
                rear, 
                hookOffsetX, 
                hookOffsetY, 
                hookOffsetZ
        )
    }
}

/// ```
Often called after START_PLAYBACK_RECORDED_VEHICLE and SKIP_TIME_IN_PLAYBACK_RECORDED_VEHICLE; similar in use to FORCE_ENTITY_AI_AND_ANIMATION_UPDATE.
```



pub fn force_playback_recorded_vehicle_update_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F2E4E06DEA8992Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F2E4E06DEA8992Bu64;
        
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
pub fn force_playback_recorded_vehicle_update_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F2E4E06DEA8992Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F2E4E06DEA8992Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x4ad280eb48b2d8e6_safe(
        
        
            vehicle: 
        , 
        
        
            togle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AD280EB48B2D8E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AD280EB48B2D8E6u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                togle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4ad280eb48b2d8e6_raw(
        vehicle: , 
        togle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AD280EB48B2D8E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AD280EB48B2D8E6u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                togle
        )
    }
}

/// ## Parameters
*



pub fn attach_vehicle_on_to_trailer_safe(
        
        
            vehicle: 
        , 
        
        
            trailer: 
        , 
        
        
            offsetX: 
        , 
        
        
            offsetY: 
        , 
        
        
            offsetZ: 
        , 
        
        
            coordsX: 
        , 
        
        
            coordsY: 
        , 
        
        
            coordsZ: 
        , 
        
        
            rotationX: 
        , 
        
        
            rotationY: 
        , 
        
        
            rotationZ: 
        , 
        
        
            disableColls: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16B5E274BDE402F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16B5E274BDE402F8u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                trailer, 
                offsetX, 
                offsetY, 
                offsetZ, 
                coordsX, 
                coordsY, 
                coordsZ, 
                rotationX, 
                rotationY, 
                rotationZ, 
                disableColls
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn attach_vehicle_on_to_trailer_raw(
        vehicle: , 
        trailer: , 
        offsetX: , 
        offsetY: , 
        offsetZ: , 
        coordsX: , 
        coordsY: , 
        coordsZ: , 
        rotationX: , 
        rotationY: , 
        rotationZ: , 
        disableColls: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x16B5E274BDE402F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x16B5E274BDE402F8u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                trailer, 
                offsetX, 
                offsetY, 
                offsetZ, 
                coordsX, 
                coordsY, 
                coordsZ, 
                rotationX, 
                rotationY, 
                rotationZ, 
                disableColls
        )
    }
}

/// INSTANTLY_FILL_VEHICLE_POPULATION native function



pub fn instantly_fill_vehicle_population_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48ADC8A773564670u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48ADC8A773564670u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn instantly_fill_vehicle_population_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48ADC8A773564670u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48ADC8A773564670u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _eject_jb700_roof_safe(
        
        
            vehicle: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE38CB9D7D39FDBCCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE38CB9D7D39FDBCCu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn _eject_jb700_roof_raw(
        vehicle: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE38CB9D7D39FDBCCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE38CB9D7D39FDBCCu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn _set_vehicle_rocket_boost_refill_time_safe(
        
        
            vehicle: 
        , 
        
        
            time: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE00F2AB100B76E89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE00F2AB100B76E89u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                time
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_rocket_boost_refill_time_raw(
        vehicle: , 
        time: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE00F2AB100B76E89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE00F2AB100B76E89u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                time
        )
    }
}

/// ```
Max 1000.
At -100 both helicopter rotors will stall.
```



pub fn get_heli_tail_boom_health_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC51915D27E4A5F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC51915D27E4A5F7u64;
        
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
pub fn get_heli_tail_boom_health_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC51915D27E4A5F7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC51915D27E4A5F7u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn is_any_ped_rappelling_from_heli_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x291E373D483E7EE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x291E373D483E7EE7u64;
        
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
pub fn is_any_ped_rappelling_from_heli_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x291E373D483E7EE7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x291E373D483E7EE7u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Removes the cargen area of interest and resumes normal cargen spawning.

You can set the area of interest with [`SET_VEHICLE_GENERATOR_AREA_OF_INTEREST`](#_0x9A75585FB2E54FAD)



pub fn clear_vehicle_generator_area_of_interest_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A436B8643716D14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A436B8643716D14u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_vehicle_generator_area_of_interest_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A436B8643716D14u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A436B8643716D14u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_is_vehicle_primary_colour_custom_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF095C0405307B21Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF095C0405307B21Bu64;
        
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
pub fn get_is_vehicle_primary_colour_custom_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF095C0405307B21Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF095C0405307B21Bu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Sets the tire smoke's color of this vehicle.  
vehicle: The vehicle that is the target of this method.  
r: The red level in the RGB color code.  
g: The green level in the RGB color code.  
b: The blue level in the RGB color code.  
Note:  
setting r,g,b to 0 will give the car independance day tyre smoke  
```



pub fn set_vehicle_tyre_smoke_color_safe(
        
        
            vehicle: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5BA80F839791C0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5BA80F839791C0Fu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn set_vehicle_tyre_smoke_color_raw(
        vehicle: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5BA80F839791C0Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5BA80F839791C0Fu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                r, 
                g, 
                b
        )
    }
}

/// ## Parameters
*



pub fn _0xa4822f1cf23f4810_safe(
        
        
            outVec: 
        , 
        
        
            p1: 
        , 
        
        
            outVec1: 
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
        let hash = 0xA4822F1CF23F4810u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4822F1CF23F4810u64;
        
        let result = invoke_raw!(
            hash,
                outVec, 
                p1, 
                outVec1, 
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
pub fn _0xa4822f1cf23f4810_raw(
        outVec: , 
        p1: , 
        outVec1: , 
        p3: , 
        p4: , 
        p5: , 
        p6: , 
        p7: , 
        p8: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4822F1CF23F4810u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4822F1CF23F4810u64;

        invoke_raw_typed!(
            hash,
                outVec, 
                p1, 
                outVec1, 
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
NativeDB Introduced: v1180
```



pub fn _0x97841634ef7df1d6_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97841634EF7DF1D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97841634EF7DF1D6u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x97841634ef7df1d6_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x97841634EF7DF1D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x97841634EF7DF1D6u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
Toggles to render distant vehicles. They may not be vehicles but images to look like vehicles.  
```



pub fn set_distant_cars_enabled_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF796359A959DF65Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF796359A959DF65Du64;
        
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
pub fn set_distant_cars_enabled_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF796359A959DF65Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF796359A959DF65Du64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```c
enum WindowTints  
{  
	WINDOWTINT_NONE = 0,
	WINDOWTINT_PURE_BLACK = 1,
	WINDOWTINT_DARKSMOKE = 2,
	WINDOWTINT_LIGHTSMOKE = 3,
	WINDOWTINT_STOCK = 4,
	WINDOWTINT_LIMO = 5,
	WINDOWTINT_GREEN = 6
};  
```



pub fn set_vehicle_window_tint_safe(
        
        
            vehicle: 
        , 
        
        
            tint: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57C51E6BAD752696u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57C51E6BAD752696u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                tint
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_window_tint_raw(
        vehicle: , 
        tint: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57C51E6BAD752696u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57C51E6BAD752696u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                tint
        )
    }
}

/// Smashes a vehicles window. See eWindowId declared in [`IS_VEHICLE_WINDOW_INTACT`](#_0x46E571A0E20D01F1).



pub fn smash_vehicle_window_safe(
        
        
            vehicle: 
        , 
        
        
            windowIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E5B5E4D2CCD2259u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E5B5E4D2CCD2259u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                windowIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn smash_vehicle_window_raw(
        vehicle: , 
        windowIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9E5B5E4D2CCD2259u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9E5B5E4D2CCD2259u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                windowIndex
        )
    }
}

/// ## Parameters
*



pub fn _0xf3b0e0aed097a3f5_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3B0E0AED097A3F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3B0E0AED097A3F5u64;
        
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
pub fn _0xf3b0e0aed097a3f5_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3B0E0AED097A3F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3B0E0AED097A3F5u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_mod_safe(
        
        
            vehicle: 
        , 
        
        
            modType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x772960298DA26FDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x772960298DA26FDBu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                modType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_mod_raw(
        vehicle: , 
        modType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x772960298DA26FDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x772960298DA26FDBu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                modType
        )
    }
}

/// ```
Works only on vehicles that support hydraulic.
```



pub fn _set_hydraulic_wheel_value_safe(
        
        
            vehicle: 
        , 
        
        
            wheelId: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84EA99C62CB3EF0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84EA99C62CB3EF0Cu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                wheelId, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_hydraulic_wheel_value_raw(
        vehicle: , 
        wheelId: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84EA99C62CB3EF0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84EA99C62CB3EF0Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                wheelId, 
                value
        )
    }
}

/// ## Parameters
*



pub fn skip_to_end_and_stop_playback_recorded_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB8E2EDA0C0A5883u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB8E2EDA0C0A5883u64;
        
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
pub fn skip_to_end_and_stop_playback_recorded_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB8E2EDA0C0A5883u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB8E2EDA0C0A5883u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_disable_random_trains_this_frame_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4B8E3D1917BC86Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4B8E3D1917BC86Bu64;
        
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
pub fn set_disable_random_trains_this_frame_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4B8E3D1917BC86Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4B8E3D1917BC86Bu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
NativeDB Introduced: 3095
```

Retrieves the remaining duration of nitrous boost available for the specified vehicle.



pub fn _get_remaining_nitrous_duration_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBEC4B8653462450Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBEC4B8653462450Eu64;
        
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
pub fn _get_remaining_nitrous_duration_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBEC4B8653462450Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBEC4B8653462450Eu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
SET_*
```



pub fn _0x428ad3e26c8d9eb0_safe(
        
        
            vehicle: 
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
        let hash = 0x428AD3E26C8D9EB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x428AD3E26C8D9EB0u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn _0x428ad3e26c8d9eb0_raw(
        vehicle: , 
        x: , 
        y: , 
        z: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x428AD3E26C8D9EB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x428AD3E26C8D9EB0u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                x, 
                y, 
                z, 
                p4
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn is_vehicle_in_submarine_mode_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA77DC70BD689A1E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA77DC70BD689A1E5u64;
        
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
pub fn is_vehicle_in_submarine_mode_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA77DC70BD689A1E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA77DC70BD689A1E5u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
1000 is max health  
Begins leaking gas at around 650 health  
```



pub fn get_vehicle_petrol_tank_health_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D5DABE888D2D074u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D5DABE888D2D074u64;
        
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
pub fn get_vehicle_petrol_tank_health_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7D5DABE888D2D074u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7D5DABE888D2D074u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _0x5e569ec46ec21cae_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E569EC46EC21CAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E569EC46EC21CAEu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5e569ec46ec21cae_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E569EC46EC21CAEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E569EC46EC21CAEu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
NativeDB Added Parameter 2: float maxEngineHealth
NativeDB Added Parameter 3: float maxPetrolTankHealth
NativeDB Added Parameter 4: float maxBodyHealth
NativeDB Added Parameter 5: float maxMainRotorHealth
NativeDB Added Parameter 6: float maxTailRotorHealth
NativeDB Added Parameter 7: float maxUnkHealth
```



pub fn get_vehicle_health_percentage_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8EF61207C2393A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8EF61207C2393A9u64;
        
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
pub fn get_vehicle_health_percentage_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8EF61207C2393A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8EF61207C2393A9u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Sounds the horn for the specified vehicle. Note that if a player is in the vehicle, it will only sound briefly.



pub fn start_vehicle_horn_safe(
        
        
            vehicle: 
        , 
        
        
            duration: 
        , 
        
        
            mode: 
        , 
        
        
            forever: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C8C6504B5B63D2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C8C6504B5B63D2Cu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                duration, 
                mode, 
                forever
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_vehicle_horn_raw(
        vehicle: , 
        duration: , 
        mode: , 
        forever: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C8C6504B5B63D2Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C8C6504B5B63D2Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                duration, 
                mode, 
                forever
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_handbrake_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x684785568EF26A22u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x684785568EF26A22u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_handbrake_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x684785568EF26A22u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x684785568EF26A22u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
SET_VEHICLE_W* (next character is either H or I)
```



pub fn _0x2c4a1590abf43e8b_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C4A1590ABF43E8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C4A1590ABF43E8Bu64;
        
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
pub fn _0x2c4a1590abf43e8b_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2C4A1590ABF43E8Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2C4A1590ABF43E8Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
Maximum amount of vehicles with vehicle stuck check appears to be 16.  
```



pub fn does_vehicle_have_stuck_vehicle_check_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57E4C39DE5EE8470u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57E4C39DE5EE8470u64;
        
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
pub fn does_vehicle_have_stuck_vehicle_check_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x57E4C39DE5EE8470u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x57E4C39DE5EE8470u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Adjusts the scale of damage applied to a specified section of a plane.
In the decompiled scripts the `damageScale` is always set to `0f` (maybe to disable damages on the specified section)

```c
enum ePlaneDamageSection {
    WING_L = 0,
    WING_R = 1,
    TAIL = 2,
    ENGINE_L = 3,
    ENGINE_R = 4,
    ELEVATOR_L = 5,
    ELEVATOR_R = 6,
    AILERON_L = 7,
    AILERON_R = 8,
    RUDDER = 9,
    RUDDER_2 = 10,
    AIRBRAKE_L = 11,
    AIRBRAKE_R = 12
}
```

```
NativeDB Introduced: v1290
```



pub fn set_plane_section_damage_scale_safe(
        
        
            vehicle: 
        , 
        
        
            damageSection: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BBB9A7A8FFE931Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BBB9A7A8FFE931Bu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                damageSection
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_plane_section_damage_scale_raw(
        vehicle: , 
        damageSection: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BBB9A7A8FFE931Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BBB9A7A8FFE931Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                damageSection
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_individual_doors_locked_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        , 
        
        
            doorLockStatus: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE70724027F85BCDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE70724027F85BCDu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex, 
                doorLockStatus
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_individual_doors_locked_raw(
        vehicle: , 
        doorIndex: , 
        doorLockStatus: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE70724027F85BCDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE70724027F85BCDu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex, 
                doorLockStatus
        )
    }
}

/// ## Parameters
*



pub fn _get_boat_boom_position_ratio_3_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F3B4D4E43177236u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F3B4D4E43177236u64;
        
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
pub fn _get_boat_boom_position_ratio_3_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F3B4D4E43177236u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F3B4D4E43177236u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// Please refer to [`GET_VEHICLE_NUMBER_PLATE_TEXT_INDEX`](#_0xF11BC2DD9A3E7195) for plate indicies.



pub fn set_vehicle_number_plate_text_index_safe(
        
        
            vehicle: 
        , 
        
        
            plateIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9088EB5A43FFB0A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9088EB5A43FFB0A1u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                plateIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_number_plate_text_index_raw(
        vehicle: , 
        plateIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9088EB5A43FFB0A1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9088EB5A43FFB0A1u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                plateIndex
        )
    }
}

/// ## Parameters
*



pub fn is_heli_part_broken_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC74B4BE25EB6C8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC74B4BE25EB6C8Au64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn is_heli_part_broken_raw(
        vehicle: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC74B4BE25EB6C8Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC74B4BE25EB6C8Au64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1, 
                p2, 
                p3
        )
    }
}

/// Prevents a specified entity from being detached from a Cargobob, even in the event of collisions.



pub fn set_cargobob_exclude_from_pickup_entity_safe(
        
        
            cargobob: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F34B0626C594380u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F34B0626C594380u64;
        
        let result = invoke_raw!(
            hash,
                cargobob, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cargobob_exclude_from_pickup_entity_raw(
        cargobob: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1F34B0626C594380u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1F34B0626C594380u64;

        invoke_raw_typed!(
            hash,
                cargobob, 
                entity
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _does_vehicle_have_landing_gear_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE43701C36CAFF1A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE43701C36CAFF1A4u64;
        
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
pub fn _does_vehicle_have_landing_gear_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE43701C36CAFF1A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE43701C36CAFF1A4u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Checks via CVehicleModelInfo  
```



pub fn does_extra_exist_safe(
        
        
            vehicle: 
        , 
        
        
            extraId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1262D55792428154u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1262D55792428154u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                extraId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_extra_exist_raw(
        vehicle: , 
        extraId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1262D55792428154u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1262D55792428154u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                extraId
        )
    }
}

/// ```
Something to do with "high speed bump severity"?  
if (!sub_87a46("SET_CAR_HIGH_SPEED_BUMP_SEVERITY_MULTIPLIER")) {  
    VEHICLE::_84FD40F56075E816(0.0);  
    sub_8795b("SET_CAR_HIGH_SPEED_BUMP_SEVERITY_MULTIPLIER", 1);  
}  
```



pub fn _set_car_high_speed_bump_severity_multiplier_safe(
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84FD40F56075E816u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84FD40F56075E816u64;
        
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
pub fn _set_car_high_speed_bump_severity_multiplier_raw(
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x84FD40F56075E816u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x84FD40F56075E816u64;

        invoke_raw_typed!(
            hash,
                multiplier
        )
    }
}

/// ```
First two parameters swapped. Scripts verify that towTruck is the first parameter, not the second.  
```



pub fn detach_vehicle_from_tow_truck_safe(
        
        
            towTruck: 
        , 
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2DB6B6708350ED8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2DB6B6708350ED8u64;
        
        let result = invoke_raw!(
            hash,
                towTruck, 
                vehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn detach_vehicle_from_tow_truck_raw(
        towTruck: , 
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2DB6B6708350ED8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2DB6B6708350ED8u64;

        invoke_raw_typed!(
            hash,
                towTruck, 
                vehicle
        )
    }
}

/// p3 is some flag related to 'trailers' (invokes CVehicle::GetTrailer).

See [`REQUEST_VEHICLE_RECORDING`](#_0xAF514CABE74CBF15).



pub fn start_playback_recorded_vehicle_safe(
        
        
            vehicle: 
        , 
        
        
            recording: 
        , 
        
        
            script: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F878F92B3A7A071u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F878F92B3A7A071u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                recording, 
                script, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_playback_recorded_vehicle_raw(
        vehicle: , 
        recording: , 
        script: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F878F92B3A7A071u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F878F92B3A7A071u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                recording, 
                script, 
                p3
        )
    }
}

/// ```
1000 is max health
Begins leaking gas at around 650 health
-999.90002441406 appears to be minimum health, although nothing special occurs <- false statement



pub fn set_vehicle_engine_health_safe(
        
        
            vehicle: 
        , 
        
        
            health: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45F6D8EEF34ABEF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45F6D8EEF34ABEF1u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                health
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_engine_health_raw(
        vehicle: , 
        health: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x45F6D8EEF34ABEF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x45F6D8EEF34ABEF1u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                health
        )
    }
}

/// ## Parameters
*



pub fn get_total_duration_of_vehicle_recording_id_safe(
        
        
            id: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x102D125411A7B6E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x102D125411A7B6E6u64;
        
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
pub fn get_total_duration_of_vehicle_recording_id_raw(
        id: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x102D125411A7B6E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x102D125411A7B6E6u64;

        invoke_raw_typed!(
            hash,
                id
        )
    }
}

/// ```
Checks if model is a boat, then checks for FLAG_IS_JETSKI.
```



pub fn _is_this_model_a_jetski_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9537097412CF75FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9537097412CF75FEu64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_this_model_a_jetski_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9537097412CF75FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9537097412CF75FEu64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// ```
A vehicle recording playback flag only used in jewelry_heist
```



pub fn _0x063ae2b2cc273588_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x063AE2B2CC273588u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x063AE2B2CC273588u64;
        
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
pub fn _0x063ae2b2cc273588_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x063AE2B2CC273588u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x063AE2B2CC273588u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_active_during_playback_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFFCEF48E511DB48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFFCEF48E511DB48u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_active_during_playback_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDFFCEF48E511DB48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDFFCEF48E511DB48u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _0x8235f1bead557629_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8235F1BEAD557629u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8235F1BEAD557629u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8235f1bead557629_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8235F1BEAD557629u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8235F1BEAD557629u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _get_is_vehicle_shunt_boost_active_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2459F72C14E2E8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2459F72C14E2E8Du64;
        
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
pub fn _get_is_vehicle_shunt_boost_active_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2459F72C14E2E8Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2459F72C14E2E8Du64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Distance traveled in the vehicles current recording.
```



pub fn get_position_in_recording_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DACD605FC681475u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DACD605FC681475u64;
        
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
pub fn get_position_in_recording_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2DACD605FC681475u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2DACD605FC681475u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _set_hydraulic_wheel_state_safe(
        
        
            vehicle: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EA86DF356801C7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EA86DF356801C7Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_hydraulic_wheel_state_raw(
        vehicle: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EA86DF356801C7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EA86DF356801C7Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                state
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_wheels_can_break_off_when_blow_up_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA37B9A517B133349u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA37B9A517B133349u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_wheels_can_break_off_when_blow_up_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA37B9A517B133349u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA37B9A517B133349u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_cargobob_pickup_magnet_falloff_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x685D5561680D088Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x685D5561680D088Bu64;
        
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
pub fn set_cargobob_pickup_magnet_falloff_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x685D5561680D088Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x685D5561680D088Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_alarm_safe(
        
        
            vehicle: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCDE5E70C1DDB954Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCDE5E70C1DDB954Cu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_alarm_raw(
        vehicle: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCDE5E70C1DDB954Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCDE5E70C1DDB954Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                state
        )
    }
}

/// This works on helicopters and planes.

Prevents a helicopter from exploding due to relatively minor body damage. 

```
NativeDB Introduced: v1103
```



pub fn set_disable_heli_explode_from_body_damage_safe(
        
        
            helicopter: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDBC8405B3895CC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDBC8405B3895CC9u64;
        
        let result = invoke_raw!(
            hash,
                helicopter
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_disable_heli_explode_from_body_damage_raw(
        helicopter: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDBC8405B3895CC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDBC8405B3895CC9u64;

        invoke_raw_typed!(
            hash,
                helicopter
        )
    }
}

/// ```
CLEAR_VEHICLE_*
```



pub fn _0x4419966c9936071a_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4419966C9936071Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4419966C9936071Au64;
        
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
pub fn _0x4419966c9936071a_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4419966C9936071Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4419966C9936071Au64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_tyres_can_burst_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x678B9BB8C3F58FEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x678B9BB8C3F58FEBu64;
        
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
pub fn get_vehicle_tyres_can_burst_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x678B9BB8C3F58FEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x678B9BB8C3F58FEBu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Sets a vehicle to be strongly resistant to explosions. p0 is the vehicle; set p1 to false to toggle the effect on/off.  
```



pub fn set_vehicle_explodes_on_high_explosion_damage_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71B0892EC081D60Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71B0892EC081D60Au64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_explodes_on_high_explosion_damage_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71B0892EC081D60Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71B0892EC081D60Au64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// According to decompiled scripts this should work with the `deluxo` and `oppressor2` vehicles.
Does nothing when used on `oppressor2`.

For the deluxo:
- Set `state` to `0.0`: Fully transform to a 'road' vehicle (non-hover mode).
- Set `state` to `1.0`: Fully transform to a 'flying' vehicle (hover mode).

If you set it to something like 0.5, then something [weird happens](https://streamable.com/p6wmr), you end up in some 50% hover mode, 50% not hover mode.

This doesn't need to be called every tick, just once and the vehicle will transform to that state at the usual transform speed. It'll just stop transforming when it reaches the state you provided.

Once this native is used then players will just be able to hit the vehicle transform key to toggle the transformation cycle; it won't block users from using the key.



pub fn set_special_flight_mode_target_ratio_safe(
        
        
            vehicle: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x438B3D7CA026FE91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x438B3D7CA026FE91u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_special_flight_mode_target_ratio_raw(
        vehicle: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x438B3D7CA026FE91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x438B3D7CA026FE91u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                state
        )
    }
}

/// ## Parameters
*



pub fn _0x99cad8e7afdb60fa_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99CAD8E7AFDB60FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99CAD8E7AFDB60FAu64;
        
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
pub fn _0x99cad8e7afdb60fa_raw(
        vehicle: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99CAD8E7AFDB60FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99CAD8E7AFDB60FAu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1, 
                p2
        )
    }
}

/// Copies sourceVehicle's damage (broken bumpers, broken lights, etc.) to targetVehicle.



pub fn copy_vehicle_damages_safe(
        
        
            sourceVehicle: 
        , 
        
        
            targetVehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE44A982368A4AF23u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE44A982368A4AF23u64;
        
        let result = invoke_raw!(
            hash,
                sourceVehicle, 
                targetVehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn copy_vehicle_damages_raw(
        sourceVehicle: , 
        targetVehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE44A982368A4AF23u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE44A982368A4AF23u64;

        invoke_raw_typed!(
            hash,
                sourceVehicle, 
                targetVehicle
        )
    }
}

/// See eDoorId declared in [`SET_VEHICLE_DOOR_SHUT`](#_0x93D9BD300D7789E5)



pub fn is_vehicle_door_fully_open_safe(
        
        
            vehicle: 
        , 
        
        
            doorIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E933CFF7B111C22u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E933CFF7B111C22u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                doorIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_door_fully_open_raw(
        vehicle: , 
        doorIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E933CFF7B111C22u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E933CFF7B111C22u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                doorIndex
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _set_random_boats_in_mp_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA5E12F728DB30CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA5E12F728DB30CAu64;
        
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
pub fn _set_random_boats_in_mp_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA5E12F728DB30CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA5E12F728DB30CAu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_mod_modifier_value_safe(
        
        
            vehicle: 
        , 
        
        
            modType: 
        , 
        
        
            modIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90A38E9838E0A8C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90A38E9838E0A8C1u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                modType, 
                modIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_mod_modifier_value_raw(
        vehicle: , 
        modType: , 
        modIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90A38E9838E0A8C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90A38E9838E0A8C1u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                modType, 
                modIndex
        )
    }
}

/// Used alongside [`SET_SPECIAL_FLIGHT_MODE_TARGET_RATIO`](#_0x438B3D7CA026FE91), this function initiates hover transformation for vehicles with a hover mode, like the `Deluxo`, based on a specified ratio (0.0 to 1.0). Incorrect values can glitch the vehicle. Without pairing, vehicles revert to car mode. Ineffective on the `oppressor2`



pub fn set_special_flight_mode_ratio_safe(
        
        
            vehicle: 
        , 
        
        
            ratio: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD138FA15C9776837u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD138FA15C9776837u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                ratio
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_special_flight_mode_ratio_raw(
        vehicle: , 
        ratio: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD138FA15C9776837u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD138FA15C9776837u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                ratio
        )
    }
}

/// ```
p1 (toggle) was always 1 (true) except in one case in the b678 scripts.  
```



pub fn set_vehicle_is_racing_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07116E24E9D1929Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07116E24E9D1929Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_is_racing_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x07116E24E9D1929Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x07116E24E9D1929Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _lower_retractable_wheels_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5335BE58C083E74Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5335BE58C083E74Eu64;
        
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
pub fn _lower_retractable_wheels_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5335BE58C083E74Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5335BE58C083E74Eu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Native is significantly more complicated than simply generating a random vector & length.

The 'point' is either 400.0 or 250.0 units away from the Ped's current coordinates; and paths into functions like rage::grcViewport___IsSphereVisible.

```
NativeDB Introduced: v1290
```



pub fn _find_random_point_in_space_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DC9675797123522u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DC9675797123522u64;
        
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
pub fn _find_random_point_in_space_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DC9675797123522u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DC9675797123522u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Previously named GET_VEHICLE_DEFORMATION_GET_TREE (hash collision)
from Decrypted Scripts I found
VEHICLE::SET_VEHICLE_CEILING_HEIGHT(l_BD9[2/*2*/], 420.0);
```



pub fn set_vehicle_ceiling_height_safe(
        
        
            vehicle: 
        , 
        
        
            height: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA46413066687A328u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA46413066687A328u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                height
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_ceiling_height_raw(
        vehicle: , 
        height: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA46413066687A328u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA46413066687A328u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                height
        )
    }
}

/// ## Parameters
*



pub fn _0x870b8b7a766615c8_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x870B8B7A766615C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x870B8B7A766615C8u64;
        
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
pub fn _0x870b8b7a766615c8_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x870B8B7A766615C8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x870B8B7A766615C8u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
Returns how many possible mods a vehicle has for a given mod type  
```



pub fn get_num_vehicle_mods_safe(
        
        
            vehicle: 
        , 
        
        
            modType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE38E9162A2500646u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE38E9162A2500646u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                modType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_num_vehicle_mods_raw(
        vehicle: , 
        modType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE38E9162A2500646u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE38E9162A2500646u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                modType
        )
    }
}

/// CLEAR_LAST_DRIVEN_VEHICLE native function



pub fn clear_last_driven_vehicle_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE01903C47C7AC89Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE01903C47C7AC89Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_last_driven_vehicle_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE01903C47C7AC89Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE01903C47C7AC89Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Works for vehicles with a retractable landing gear  
landing gear states:  
0: Deployed  
1: Closing  
2: Opening  
3: Retracted  
```



pub fn control_landing_gear_safe(
        
        
            vehicle: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFC8BE9A5E1FE575u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFC8BE9A5E1FE575u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn control_landing_gear_raw(
        vehicle: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCFC8BE9A5E1FE575u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCFC8BE9A5E1FE575u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                state
        )
    }
}

/// ## Parameters
*



pub fn can_shuffle_seat_safe(
        
        
            vehicle: 
        , 
        
        
            seatIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30785D90C956BF35u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30785D90C956BF35u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                seatIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_shuffle_seat_raw(
        vehicle: , 
        seatIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30785D90C956BF35u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30785D90C956BF35u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                seatIndex
        )
    }
}

/// ## Parameters
*



pub fn _0x182f266c2d9e2beb_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x182F266C2D9E2BEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x182F266C2D9E2BEBu64;
        
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
pub fn _0x182f266c2d9e2beb_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x182F266C2D9E2BEBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x182F266C2D9E2BEBu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
Returns max braking of the specified vehicle model.
```



pub fn get_vehicle_model_max_braking_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC53FD41B4ED944Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC53FD41B4ED944Cu64;
        
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
pub fn get_vehicle_model_max_braking_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC53FD41B4ED944Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC53FD41B4ED944Cu64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// ```
NativeDB Introduced: v1365
```



pub fn _0x887fa38787de8c72_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x887FA38787DE8C72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x887FA38787DE8C72u64;
        
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
pub fn _0x887fa38787de8c72_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x887FA38787DE8C72u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x887FA38787DE8C72u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Determines if a vehicle is a convertible with an animatable roof. This native checks if the specified vehicle model features a convertible roof that can be lowered or raised through an animation.

```
NativeDB Introduced: v323
```



pub fn is_vehicle_a_convertible_safe(
        
        
            vehicle: 
        , 
        
        
            checkRoofExtras: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52F357A30698BCCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52F357A30698BCCEu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                checkRoofExtras
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_a_convertible_raw(
        vehicle: , 
        checkRoofExtras: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52F357A30698BCCEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52F357A30698BCCEu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                checkRoofExtras
        )
    }
}

/// Determines whether a specific vehicle is equipped with a roof.

```
NativeDB Introduced: v323
```



pub fn does_vehicle_have_roof_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8AC862B0B32C5B80u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8AC862B0B32C5B80u64;
        
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
pub fn does_vehicle_have_roof_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8AC862B0B32C5B80u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8AC862B0B32C5B80u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _is_vehicle_weapon_disabled_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x563B65A643ED072Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x563B65A643ED072Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_vehicle_weapon_disabled_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x563B65A643ED072Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x563B65A643ED072Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x35e0654f4bad7971_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35E0654F4BAD7971u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35E0654F4BAD7971u64;
        
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
pub fn _0x35e0654f4bad7971_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35E0654F4BAD7971u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35E0654F4BAD7971u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Does nothing. It's a nullsub.

NativeDB Introduced: v1604
```



pub fn _0x99a05839c46ce316_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99A05839C46CE316u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99A05839C46CE316u64;
        
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
pub fn _0x99a05839c46ce316_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x99A05839C46CE316u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x99A05839C46CE316u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
Roll down all the windows of the vehicle passed through the first parameter.  
```



pub fn roll_down_windows_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85796B0549DDE156u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85796B0549DDE156u64;
        
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
pub fn roll_down_windows_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85796B0549DDE156u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85796B0549DDE156u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Same call as VEHICLE::_0x0F3B4D4E43177236
```



pub fn _get_boat_boom_position_ratio_2_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1F981A6F74F0C23u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1F981A6F74F0C23u64;
        
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
pub fn _get_boat_boom_position_ratio_2_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC1F981A6F74F0C23u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC1F981A6F74F0C23u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x9640e30a7f395e4b_safe(
        
        
            vehicle: 
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
        let hash = 0x9640E30A7F395E4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9640E30A7F395E4Bu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn _0x9640e30a7f395e4b_raw(
        vehicle: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9640E30A7F395E4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9640E30A7F395E4Bu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// Sets flag on vehicle that changes behaviour in relation to when player gets wanted level



pub fn set_police_focus_will_track_vehicle_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E74E62E0A97E901u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E74E62E0A97E901u64;
        
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
pub fn set_police_focus_will_track_vehicle_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E74E62E0A97E901u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E74E62E0A97E901u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
Commands the driver of an armed vehicle (p0) to shoot its weapon at a target (p1). p3, p4 and p5 are the coordinates of the target. Example:  
WEAPON::SET_CURRENT_PED_VEHICLE_WEAPON(pilot,GAMEPLAY::GET_HASH_KEY("VEHICLE_WEAPON_PLANE_ROCKET"));VEHICLE::SET_VEHICLE_SHOOT_AT_TARGET(pilot, target, targPos.x, targPos.y, targPos.z);  
```



pub fn set_vehicle_shoot_at_target_safe(
        
        
            driver: 
        , 
        
        
            entity: 
        , 
        
        
            xTarget: 
        , 
        
        
            yTarget: 
        , 
        
        
            zTarget: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74CD9A9327A282EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74CD9A9327A282EAu64;
        
        let result = invoke_raw!(
            hash,
                driver, 
                entity, 
                xTarget, 
                yTarget, 
                zTarget
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_shoot_at_target_raw(
        driver: , 
        entity: , 
        xTarget: , 
        yTarget: , 
        zTarget: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74CD9A9327A282EAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74CD9A9327A282EAu64;

        invoke_raw_typed!(
            hash,
                driver, 
                entity, 
                xTarget, 
                yTarget, 
                zTarget
        )
    }
}

/// ```
Landing gear states:  
0: Deployed  
1: Closing (Retracting)
3: Opening (Deploying)
4: Retracted  
5: Broken
```

Landing gear state 2 is never used.



pub fn get_landing_gear_state_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B0F3DCA3DB0F4CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B0F3DCA3DB0F4CDu64;
        
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
pub fn get_landing_gear_state_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B0F3DCA3DB0F4CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B0F3DCA3DB0F4CDu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x0205f5365292d2eb_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0205F5365292D2EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0205F5365292D2EBu64;
        
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
pub fn _0x0205f5365292d2eb_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0205F5365292D2EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0205F5365292D2EBu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// Checks if a boat is currently anchored.

This native is a getter for [SET_BOAT_ANCHOR](#_0x75DBEC174AEEAD10).


```
NativeDB Introduced: v573
```



pub fn is_boat_anchored_safe(
        
        
            boat: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB0AD1238A709B1A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB0AD1238A709B1A2u64;
        
        let result = invoke_raw!(
            hash,
                boat
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_boat_anchored_raw(
        boat: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB0AD1238A709B1A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB0AD1238A709B1A2u64;

        invoke_raw_typed!(
            hash,
                boat
        )
    }
}

/// ```
Gets the number of passengers, NOT including the driver. Use IS_VEHICLE_SEAT_FREE(Vehicle, -1) to also check for the driver  
```



pub fn get_vehicle_number_of_passengers_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24CB2137731FFE89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24CB2137731FFE89u64;
        
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
pub fn get_vehicle_number_of_passengers_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24CB2137731FFE89u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24CB2137731FFE89u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Works just like SET_VEHICLE_ENGINE_HEALTH, didn't saw any difference. But this native works only for planes.
```



pub fn _set_plane_engine_health_safe(
        
        
            vehicle: 
        , 
        
        
            health: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A86A0475B6A1434u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A86A0475B6A1434u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                health
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_plane_engine_health_raw(
        vehicle: , 
        health: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A86A0475B6A1434u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A86A0475B6A1434u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                health
        )
    }
}

/// ## Parameters
*



pub fn request_vehicle_high_detail_model_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6E9FDCB2C76785Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6E9FDCB2C76785Eu64;
        
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
pub fn request_vehicle_high_detail_model_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6E9FDCB2C76785Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6E9FDCB2C76785Eu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Return value



pub fn get_num_vehicle_window_tints_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D1224004B3A6707u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D1224004B3A6707u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_num_vehicle_window_tints_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D1224004B3A6707u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D1224004B3A6707u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn modify_vehicle_top_speed_safe(
        
        
            vehicle: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93A3996368C94158u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93A3996368C94158u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn modify_vehicle_top_speed_raw(
        vehicle: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93A3996368C94158u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93A3996368C94158u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                value
        )
    }
}

/// ```
NativeDB Introduced: v1604
NativeDB Added Parameter 2 (2060): float durationMod : A multiplier applied to the default nitrous duration (default is 3 seconds). 
NativeDB Added Parameter 3 (2060): float power : A multiplier applied to the default nitrous power multiplier (default is 3).
NativeDB Added Parameter 4 (2060): float rechargeTime : A multiplier applied to the default nitrous recharge rate.
NativeDB Added Parameter 5 (2060): BOOL disableSound : A boolean to disable the default nitrous sound when the nitrous is active.
```

Overrides the default settings of a vehicle's nitrous system, allowing custom control over its performance characteristics.



pub fn set_override_nitrous_level_safe(
        
        
            vehicle: 
        , 
        
        
            override: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8E9B6B71B8E660Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8E9B6B71B8E660Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                override
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_override_nitrous_level_raw(
        vehicle: , 
        override: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8E9B6B71B8E660Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8E9B6B71B8E660Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                override
        )
    }
}

/// ```
value between 0.0 and 1.0  
```



pub fn _set_helicopter_roll_pitch_yaw_mult_safe(
        
        
            helicopter: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E0859B530A365CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E0859B530A365CCu64;
        
        let result = invoke_raw!(
            hash,
                helicopter, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_helicopter_roll_pitch_yaw_mult_raw(
        helicopter: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E0859B530A365CCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E0859B530A365CCu64;

        invoke_raw_typed!(
            hash,
                helicopter, 
                multiplier
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _set_tyre_wear_multiplier_safe(
        
        
            vehicle: 
        , 
        
        
            wheelIndex: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01894E2EDE923CA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01894E2EDE923CA2u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                wheelIndex, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_tyre_wear_multiplier_raw(
        vehicle: , 
        wheelIndex: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x01894E2EDE923CA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x01894E2EDE923CA2u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                wheelIndex, 
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_class_max_agility_safe(
        
        
            vehicleClass: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F930AD022D6DE3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F930AD022D6DE3Bu64;
        
        let result = invoke_raw!(
            hash,
                vehicleClass
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_class_max_agility_raw(
        vehicleClass: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F930AD022D6DE3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F930AD022D6DE3Bu64;

        invoke_raw_typed!(
            hash,
                vehicleClass
        )
    }
}

/// ## Parameters
*



pub fn _0x78ceee41f49f421f_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78CEEE41F49F421Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78CEEE41F49F421Fu64;
        
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
pub fn _0x78ceee41f49f421f_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78CEEE41F49F421Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78CEEE41F49F421Fu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_lod_multiplier_safe(
        
        
            vehicle: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93AE6A61BE015BF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93AE6A61BE015BF1u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_lod_multiplier_raw(
        vehicle: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93AE6A61BE015BF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93AE6A61BE015BF1u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                multiplier
        )
    }
}

/// Sets a limited number of ammo for a particular vehicle weapon index on a vehicle.  

Information about weapon indexes can be found in `handling.meta`. 

In the example given below, `uWeaponHash` defines weapon hashes for the vehicle. Each `<Item>` corresponds to an index starting from `0`.

```
<uWeaponHash>
    <Item>VEHICLE_WEAPON_PLAYER_BUZZARD</Item>  <!-- Index: 0 -->
    <Item>VEHICLE_WEAPON_SPACE_ROCKET</Item>    <!-- Index: 1 -->
    <Item>VEHICLE_WEAPON_SEARCHLIGHT</Item>     <!-- Index: 2 -->
</uWeaponHash>
```



pub fn set_vehicle_weapon_restricted_ammo_safe(
        
        
            vehicle: 
        , 
        
        
            weaponIndex: 
        , 
        
        
            ammoCount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44CD1F493DB2A0A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44CD1F493DB2A0A6u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                weaponIndex, 
                ammoCount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_weapon_restricted_ammo_raw(
        vehicle: , 
        weaponIndex: , 
        ammoCount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44CD1F493DB2A0A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44CD1F493DB2A0A6u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                weaponIndex, 
                ammoCount
        )
    }
}

/// ```
paintType:
0: Normal
1: Metallic
2: Pearl
3: Matte
4: Metal
5: Chrome
```



pub fn get_num_mod_colors_safe(
        
        
            paintType: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA551BE18C11A476Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA551BE18C11A476Du64;
        
        let result = invoke_raw!(
            hash,
                paintType, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_num_mod_colors_raw(
        paintType: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA551BE18C11A476Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA551BE18C11A476Du64;

        invoke_raw_typed!(
            hash,
                paintType, 
                p1
        )
    }
}

/// ```
Checks if vehicle tyre at index exists. Also returns false if tyre was removed.
```

```
NativeDB Introduced: v1493
```



pub fn _does_vehicle_tyre_exist_safe(
        
        
            vehicle: 
        , 
        
        
            tyreIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x534E36D4DB9ECC5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x534E36D4DB9ECC5Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                tyreIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _does_vehicle_tyre_exist_raw(
        vehicle: , 
        tyreIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x534E36D4DB9ECC5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x534E36D4DB9ECC5Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                tyreIndex
        )
    }
}

/// ```
Returns a value depending on the lock-on state of vehicle weapons.
0: not locked on
1: locking on
2: locked on
```



pub fn get_vehicle_homing_lockon_state_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6B0E8CFC3633BF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6B0E8CFC3633BF0u64;
        
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
pub fn get_vehicle_homing_lockon_state_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6B0E8CFC3633BF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6B0E8CFC3633BF0u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// DELETE_ALL_TRAINS native function



pub fn delete_all_trains_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x736A718577F39C7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x736A718577F39C7Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn delete_all_trains_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x736A718577F39C7Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x736A718577F39C7Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_custom_secondary_colour_safe(
        
        
            vehicle: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8389CD56CA8072DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8389CD56CA8072DCu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn get_vehicle_custom_secondary_colour_raw(
        vehicle: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8389CD56CA8072DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8389CD56CA8072DCu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                r, 
                g, 
                b
        )
    }
}

/// ## Parameters
*



pub fn remove_vehicle_high_detail_model_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00689CDE5F7C6787u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00689CDE5F7C6787u64;
        
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
pub fn remove_vehicle_high_detail_model_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00689CDE5F7C6787u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00689CDE5F7C6787u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Makes a helicopter resistant to multiple explosions. When enabled, helicopters can survive two or more explosions.

```
NativeDB Introduced: 2545
```



pub fn set_heli_resist_to_explosion_safe(
        
        
            helicopter: 
        , 
        
        
            bResistToExplosion: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8074CC1886802912u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8074CC1886802912u64;
        
        let result = invoke_raw!(
            hash,
                helicopter, 
                bResistToExplosion
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_heli_resist_to_explosion_raw(
        helicopter: , 
        bResistToExplosion: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8074CC1886802912u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8074CC1886802912u64;

        invoke_raw_typed!(
            hash,
                helicopter, 
                bResistToExplosion
        )
    }
}

/// ## Parameters
*



pub fn _0x4d9d109f63fee1d4_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D9D109F63FEE1D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D9D109F63FEE1D4u64;
        
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
pub fn _0x4d9d109f63fee1d4_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D9D109F63FEE1D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D9D109F63FEE1D4u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Returns whether the specified vehicle is currently in a burnout.  
vb.net  
Public Function isVehicleInBurnout(vh As Vehicle) As Boolean  
        Return Native.Function.Call(Of Boolean)(Hash.IS_VEHICLE_IN_BURNOUT, vh)  
    End Function  
```



pub fn is_vehicle_in_burnout_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1297A88E081430EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1297A88E081430EBu64;
        
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
pub fn is_vehicle_in_burnout_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1297A88E081430EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1297A88E081430EBu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Returns -1 if the vehicle has no livery  
```



pub fn get_vehicle_livery_count_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87B63E25A529D526u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87B63E25A529D526u64;
        
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
pub fn get_vehicle_livery_count_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x87B63E25A529D526u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x87B63E25A529D526u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Second Param = LiveryIndex  
example   
int count = VEHICLE::GET_VEHICLE_LIVERY_COUNT(veh);  
for (int i = 0; i < count; i++)    
	{  
char* LiveryName = VEHICLE::GET_LIVERY_NAME(veh, i);  
	}  
this example will work fine to fetch all names   
for example for Sanchez we get   
SANC_LV1  
SANC_LV2  
SANC_LV3  
SANC_LV4  
SANC_LV5  
Use _GET_LABEL_TEXT, to get the localized livery name.  
```

NOTE: You may need to set the vehicle's modKit to 0 by using this function [SET_VEHICLE_MOD_KIT](#_0x1F2AA07F00B3217A) before getting the name, otherwise this native may return NULL.



pub fn get_livery_name_safe(
        
        
            vehicle: 
        , 
        
        
            liveryIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4C7A93837C91A1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4C7A93837C91A1Fu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                liveryIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_livery_name_raw(
        vehicle: , 
        liveryIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4C7A93837C91A1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4C7A93837C91A1Fu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                liveryIndex
        )
    }
}

/// ## Parameters
*



pub fn start_vehicle_alarm_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8FF7AB45305C345u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8FF7AB45305C345u64;
        
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
pub fn start_vehicle_alarm_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB8FF7AB45305C345u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB8FF7AB45305C345u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_tyre_smoke_color_safe(
        
        
            vehicle: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB635392A4938B3C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB635392A4938B3C3u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
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
pub fn get_vehicle_tyre_smoke_color_raw(
        vehicle: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB635392A4938B3C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB635392A4938B3C3u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                r, 
                g, 
                b
        )
    }
}

/// **Usage:**

- Use this native inside a looped function.
- Values:
  - `0.0` = no vehicles on streets
  - `1.0` = normal vehicles on streets

`1.0` Seems to be the maximum.



pub fn set_vehicle_density_multiplier_this_frame_safe(
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x245A6883D966D537u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x245A6883D966D537u64;
        
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
pub fn set_vehicle_density_multiplier_this_frame_raw(
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x245A6883D966D537u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x245A6883D966D537u64;

        invoke_raw_typed!(
            hash,
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn unpause_playback_recorded_vehicle_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8879EE09268305D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8879EE09268305D5u64;
        
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
pub fn unpause_playback_recorded_vehicle_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8879EE09268305D5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8879EE09268305D5u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Enables or disables a vehicle mod by index (`modType`) for a given vehicle.  

`eVehicleModType` enum, used for `modType` index can be found under [`SET_VEHICLE_MOD`](#_0x6AF0636DDEDCB6DD).



pub fn toggle_vehicle_mod_safe(
        
        
            vehicle: 
        , 
        
        
            modType: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A1F4F37F95BAD08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A1F4F37F95BAD08u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                modType, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn toggle_vehicle_mod_raw(
        vehicle: , 
        modType: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A1F4F37F95BAD08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A1F4F37F95BAD08u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                modType, 
                toggle
        )
    }
}

/// To reset the max speed, set the `speed` value to `0.0` or lower.



pub fn _set_vehicle_max_speed_safe(
        
        
            vehicle: 
        , 
        
        
            speed: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAA045B4E42F3C06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAA045B4E42F3C06u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                speed
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_max_speed_raw(
        vehicle: , 
        speed: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBAA045B4E42F3C06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBAA045B4E42F3C06u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                speed
        )
    }
}

/// ## Parameters
*



pub fn _0x2311dd7159f00582_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2311DD7159F00582u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2311DD7159F00582u64;
        
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
pub fn _0x2311dd7159f00582_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2311DD7159F00582u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2311DD7159F00582u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ```
This is not tested - it's just an assumption.  
Doesn't seem to work.  I'll try with an int instead. --JT  
Read the scripts, im dumpass.   
Doesn't work at all, wether with an bool neither an int  
                            if (!VEHICLE::IS_TAXI_LIGHT_ON(l_115)) {  
                                VEHICLE::SET_TAXI_LIGHTS(l_115, 1);  
                            }  
```



pub fn set_taxi_lights_safe(
        
        
            vehicle: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x598803E85E8448D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x598803E85E8448D9u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_taxi_lights_raw(
        vehicle: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x598803E85E8448D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x598803E85E8448D9u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                state
        )
    }
}

/// Fix a given vehicle.
If the vehicle's engine's broken then you cannot fix it with this native.



pub fn set_vehicle_fixed_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x115722B1B9C14C1Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x115722B1B9C14C1Cu64;
        
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
pub fn set_vehicle_fixed_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x115722B1B9C14C1Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x115722B1B9C14C1Cu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_livery_safe(
        
        
            vehicle: 
        , 
        
        
            livery: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60BF608F1B8CD1B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60BF608F1B8CD1B6u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                livery
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_livery_raw(
        vehicle: , 
        livery: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x60BF608F1B8CD1B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x60BF608F1B8CD1B6u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                livery
        )
    }
}

/// ```
Explodes a selected vehicle.  
Vehicle vehicle = Vehicle you want to explode.  
BOOL isAudible = If explosion makes a sound.  
BOOL isInvisible = If the explosion is invisible or not.  
First BOOL does not give any visual explosion, the vehicle just falls apart completely but slowly and starts to burn.  
```



pub fn explode_vehicle_safe(
        
        
            vehicle: 
        , 
        
        
            isAudible: 
        , 
        
        
            isInvisible: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA71116ADF5B514Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA71116ADF5B514Cu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                isAudible, 
                isInvisible
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn explode_vehicle_raw(
        vehicle: , 
        isAudible: , 
        isInvisible: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA71116ADF5B514Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA71116ADF5B514Cu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                isAudible, 
                isInvisible
        )
    }
}

/// ## Parameters
*



pub fn _0xfaf2a78061fd9ef4_safe(
        
        
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
        let hash = 0xFAF2A78061FD9EF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAF2A78061FD9EF4u64;
        
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
pub fn _0xfaf2a78061fd9ef4_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAF2A78061FD9EF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAF2A78061FD9EF4u64;

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



pub fn is_this_model_a_plane_safe(
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0948AB42D7BA0DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0948AB42D7BA0DEu64;
        
        let result = invoke_raw!(
            hash,
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_this_model_a_plane_raw(
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0948AB42D7BA0DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0948AB42D7BA0DEu64;

        invoke_raw_typed!(
            hash,
                model
        )
    }
}

/// ## Parameters
*



pub fn set_garbage_trucks_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2AFD795EEAC8D30Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2AFD795EEAC8D30Du64;
        
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
pub fn set_garbage_trucks_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2AFD795EEAC8D30Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2AFD795EEAC8D30Du64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ```
p2 often set to 1000.0 in the decompiled scripts.  
```



pub fn set_vehicle_body_health_safe(
        
        
            vehicle: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB77D05AC8C78AADBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB77D05AC8C78AADBu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_body_health_raw(
        vehicle: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB77D05AC8C78AADBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB77D05AC8C78AADBu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                value
        )
    }
}

/// ## Return value



pub fn has_vehicle_phone_explosive_device_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6ADAABD3068C5235u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6ADAABD3068C5235u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_vehicle_phone_explosive_device_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6ADAABD3068C5235u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6ADAABD3068C5235u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _is_handler_frame_above_container_safe(
        
        
            handler: 
        , 
        
        
            container: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89D630CF5EA96D23u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89D630CF5EA96D23u64;
        
        let result = invoke_raw!(
            hash,
                handler, 
                container
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_handler_frame_above_container_raw(
        handler: , 
        container: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x89D630CF5EA96D23u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x89D630CF5EA96D23u64;

        invoke_raw_typed!(
            hash,
                handler, 
                container
        )
    }
}

/// ## Parameters
*



pub fn set_cargobob_pickup_magnet_reduced_falloff_safe(
        
        
            cargobob: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66979ACF5102FD2Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66979ACF5102FD2Fu64;
        
        let result = invoke_raw!(
            hash,
                cargobob, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cargobob_pickup_magnet_reduced_falloff_raw(
        cargobob: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66979ACF5102FD2Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66979ACF5102FD2Fu64;

        invoke_raw_typed!(
            hash,
                cargobob, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn remove_road_node_speed_zone_safe(
        
        
            speedzone: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1033371FC8E842A7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1033371FC8E842A7u64;
        
        let result = invoke_raw!(
            hash,
                speedzone
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_road_node_speed_zone_raw(
        speedzone: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1033371FC8E842A7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1033371FC8E842A7u64;

        invoke_raw_typed!(
            hash,
                speedzone
        )
    }
}

/// Creates a vehicle with the specified model at the specified position. This vehicle will initially be owned by the creating
script as a mission entity, and the model should be loaded already (e.g. using REQUEST_MODEL).

```
NativeDB Added Parameter 8: BOOL p7
```



pub fn create_vehicle_safe(
        
        
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
        
        
            netMissionEntity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF35D0D2583051B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF35D0D2583051B0u64;
        
        let result = invoke_raw!(
            hash,
                modelHash, 
                x, 
                y, 
                z, 
                heading, 
                isNetwork, 
                netMissionEntity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_vehicle_raw(
        modelHash: , 
        x: , 
        y: , 
        z: , 
        heading: , 
        isNetwork: , 
        netMissionEntity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF35D0D2583051B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF35D0D2583051B0u64;

        invoke_raw_typed!(
            hash,
                modelHash, 
                x, 
                y, 
                z, 
                heading, 
                isNetwork, 
                netMissionEntity
        )
    }
}

/// ```
Only called once in the decompiled scripts. Presumably activates the specified generator.  
```



pub fn set_script_vehicle_generator_safe(
        
        
            vehicleGenerator: 
        , 
        
        
            enabled: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9D620E0AC6DC4B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9D620E0AC6DC4B0u64;
        
        let result = invoke_raw!(
            hash,
                vehicleGenerator, 
                enabled
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_script_vehicle_generator_raw(
        vehicleGenerator: , 
        enabled: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9D620E0AC6DC4B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9D620E0AC6DC4B0u64;

        invoke_raw_typed!(
            hash,
                vehicleGenerator, 
                enabled
        )
    }
}

/// ## Parameters
*



pub fn _0x9d30687c57baa0bb_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D30687C57BAA0BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D30687C57BAA0BBu64;
        
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
pub fn _0x9d30687c57baa0bb_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9D30687C57BAA0BBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9D30687C57BAA0BBu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Can be used for IS_DLC_VEHICLE_MOD and _0xC098810437312FFF
```



pub fn get_vehicle_mod_identifier_hash_safe(
        
        
            vehicle: 
        , 
        
        
            modType: 
        , 
        
        
            modIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4593CF82AA179706u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4593CF82AA179706u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                modType, 
                modIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_vehicle_mod_identifier_hash_raw(
        vehicle: , 
        modType: , 
        modIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4593CF82AA179706u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4593CF82AA179706u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                modType, 
                modIndex
        )
    }
}

/// ```
Sets how much the crane on the tow truck is raised, where 0.0 is fully lowered and 1.0 is fully raised.  
```



pub fn set_vehicle_tow_truck_arm_position_safe(
        
        
            vehicle: 
        , 
        
        
            position: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE54B92A344583CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE54B92A344583CAu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                position
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_tow_truck_arm_position_raw(
        vehicle: , 
        position: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFE54B92A344583CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFE54B92A344583CAu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                position
        )
    }
}

/// ```
RESET_*

Resets the effect of 0x428AD3E26C8D9EB0
```



pub fn _0xe2f53f172b45ede1_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2F53F172B45EDE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2F53F172B45EDE1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xe2f53f172b45ede1_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE2F53F172B45EDE1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE2F53F172B45EDE1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Sets whether a boat should remain in the non-physical, low LOD anchor mode even when a player is driving it.



pub fn set_force_low_lod_anchor_mode_safe(
        
        
            boat: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB28B1FE5BFADD7F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB28B1FE5BFADD7F5u64;
        
        let result = invoke_raw!(
            hash,
                boat, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_force_low_lod_anchor_mode_raw(
        boat: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB28B1FE5BFADD7F5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB28B1FE5BFADD7F5u64;

        invoke_raw_typed!(
            hash,
                boat, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_vehicle_max_braking_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD7E85FC227197C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD7E85FC227197C4u64;
        
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
pub fn get_vehicle_max_braking_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD7E85FC227197C4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD7E85FC227197C4u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_disable_vehicle_petrol_tank_damage_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37C8252A7C92D017u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37C8252A7C92D017u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_disable_vehicle_petrol_tank_damage_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37C8252A7C92D017u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37C8252A7C92D017u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// Sets the selected vehicle's colors to their default value (specific variant specified using the colorCombination parameter).

Range of possible values for colorCombination is currently unknown, I couldn't find where these values are stored either (Disquse's guess was vehicles.meta but I haven't seen it in there.)



pub fn set_vehicle_colour_combination_safe(
        
        
            vehicle: 
        , 
        
        
            colorCombination: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33E8CD3322E2FE31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33E8CD3322E2FE31u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                colorCombination
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_colour_combination_raw(
        vehicle: , 
        colorCombination: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33E8CD3322E2FE31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33E8CD3322E2FE31u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                colorCombination
        )
    }
}

/// Removes a scripted vehicle generator.



pub fn delete_script_vehicle_generator_safe(
        
        
            vehicleGenerator: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22102C9ABFCF125Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22102C9ABFCF125Du64;
        
        let result = invoke_raw!(
            hash,
                vehicleGenerator
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn delete_script_vehicle_generator_raw(
        vehicleGenerator: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x22102C9ABFCF125Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x22102C9ABFCF125Du64;

        invoke_raw_typed!(
            hash,
                vehicleGenerator
        )
    }
}

/// ## Parameters
*



pub fn _0x72beccf4b829522e_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72BECCF4B829522Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72BECCF4B829522Eu64;
        
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
pub fn _0x72beccf4b829522e_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x72BECCF4B829522Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x72BECCF4B829522Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_has_been_owned_by_player_safe(
        
        
            vehicle: 
        , 
        
        
            owned: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B5F9D2AF1F1722Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B5F9D2AF1F1722Du64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                owned
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_has_been_owned_by_player_raw(
        vehicle: , 
        owned: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B5F9D2AF1F1722Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B5F9D2AF1F1722Du64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                owned
        )
    }
}

/// ## Parameters
*



pub fn _0xcf9159024555488c_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF9159024555488Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF9159024555488Cu64;
        
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
pub fn _0xcf9159024555488c_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF9159024555488Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF9159024555488Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: 3095
```

Resets or clears the nitrous system for a specified vehicle. You can check if a vehicle has nitrous with [`IS_NITROUS_ACTIVE`](#_0x491E822B2C464FE4)



pub fn clear_nitrous_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC889AE921400E1EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC889AE921400E1EDu64;
        
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
pub fn clear_nitrous_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC889AE921400E1EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC889AE921400E1EDu64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// Returns whether the specified vehicle is designated as a mercenary vehicle



pub fn get_vehicle_is_mercenary_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4C4642CB7F50B5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4C4642CB7F50B5Du64;
        
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
pub fn get_vehicle_is_mercenary_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4C4642CB7F50B5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4C4642CB7F50B5Du64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn _0xbe5c1255a1830ff5_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE5C1255A1830FF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE5C1255A1830FF5u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xbe5c1255a1830ff5_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBE5C1255A1830FF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBE5C1255A1830FF5u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_boat_disable_avoidance_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A6A279F3AA4FD70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A6A279F3AA4FD70u64;
        
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
pub fn set_boat_disable_avoidance_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A6A279F3AA4FD70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A6A279F3AA4FD70u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x0581730ab9380412_safe(
        
        
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
        let hash = 0x0581730AB9380412u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0581730AB9380412u64;
        
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
pub fn _0x0581730ab9380412_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0581730AB9380412u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0581730AB9380412u64;

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



pub fn _set_plane_propellers_health_safe(
        
        
            plane: 
        , 
        
        
            health: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C815EB175086F84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C815EB175086F84u64;
        
        let result = invoke_raw!(
            hash,
                plane, 
                health
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_plane_propellers_health_raw(
        plane: , 
        health: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C815EB175086F84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C815EB175086F84u64;

        invoke_raw_typed!(
            hash,
                plane, 
                health
        )
    }
}

/// ```
Does nothing. It's a nullsub.

NativeDB Introduced: v1604
```



pub fn _0x82e0ac411e41a5b4_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82E0AC411E41A5B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82E0AC411E41A5B4u64;
        
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
pub fn _0x82e0ac411e41a5b4_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x82E0AC411E41A5B4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x82E0AC411E41A5B4u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Return value



pub fn get_last_driven_vehicle_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2D06FAEDE65B577u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2D06FAEDE65B577u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_last_driven_vehicle_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2D06FAEDE65B577u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2D06FAEDE65B577u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// The



pub fn _set_vehicle_exclusive_driver_2_safe(
        
        
            vehicle: 
        , 
        
        
            ped: 
        , 
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5C51B5502E85E83u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5C51B5502E85E83u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                ped, 
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_vehicle_exclusive_driver_2_raw(
        vehicle: , 
        ped: , 
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5C51B5502E85E83u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5C51B5502E85E83u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                ped, 
                index
        )
    }
}

/// Retrieves the agility for a specific boat model, including any vehicle mods. Unlike other vehicles where Rockstar Games typically assess performance based on traction, boats use agility as a measure. This static value is distinct from the traction metrics used for other vehicle types.

```
NativeDB Introduced: v323
```



pub fn get_boat_vehicle_model_agility_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AA3F878A178C4FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AA3F878A178C4FCu64;
        
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
pub fn get_boat_vehicle_model_agility_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5AA3F878A178C4FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5AA3F878A178C4FCu64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// ## Parameters
*



pub fn _set_disable_superdummy_mode_safe(
        
        
            vehicle: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB088E9A47AE6EDD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB088E9A47AE6EDD5u64;
        
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
pub fn _set_disable_superdummy_mode_raw(
        vehicle: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB088E9A47AE6EDD5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB088E9A47AE6EDD5u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _get_vehicle_can_activate_parachute_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA916396DF4154EE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA916396DF4154EE3u64;
        
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
pub fn _get_vehicle_can_activate_parachute_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA916396DF4154EE3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA916396DF4154EE3u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
Max 1000.  
At 0 the main rotor will stall.  
```



pub fn get_heli_main_rotor_health_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4CB7541F413D2C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4CB7541F413D2C5u64;
        
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
pub fn get_heli_main_rotor_health_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4CB7541F413D2C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4CB7541F413D2C5u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

/// ```
SET_C*
```



pub fn _0xb2e0c0d6922d31f2_safe(
        
        
            vehicle: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2E0C0D6922D31F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2E0C0D6922D31F2u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb2e0c0d6922d31f2_raw(
        vehicle: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2E0C0D6922D31F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2E0C0D6922D31F2u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                toggle
        )
    }
}

/// ```
garageName example "Michael - Beverly Hills"
```



pub fn is_vehicle_in_garage_area_safe(
        
        
            garageName: 
        , 
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEE4490CD57BB3C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEE4490CD57BB3C2u64;
        
        let result = invoke_raw!(
            hash,
                garageName, 
                vehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_vehicle_in_garage_area_raw(
        garageName: , 
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEE4490CD57BB3C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEE4490CD57BB3C2u64;

        invoke_raw_typed!(
            hash,
                garageName, 
                vehicle
        )
    }
}

/// ## Parameters
*



pub fn set_vehicle_flight_nozzle_position_safe(
        
        
            vehicle: 
        , 
        
        
            angleRatio: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30D779DE7C4F6DD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30D779DE7C4F6DD3u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                angleRatio
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_vehicle_flight_nozzle_position_raw(
        vehicle: , 
        angleRatio: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x30D779DE7C4F6DD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x30D779DE7C4F6DD3u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                angleRatio
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _get_drift_tyres_enabled_safe(
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F5A72430E78C8D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F5A72430E78C8D3u64;
        
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
pub fn _get_drift_tyres_enabled_raw(
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F5A72430E78C8D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F5A72430E78C8D3u64;

        invoke_raw_typed!(
            hash,
                vehicle
        )
    }
}

