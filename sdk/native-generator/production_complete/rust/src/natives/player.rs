//! PLAYER native functions
//! 
//! Functions for the player category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn _special_ability_activate_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x821FDC827D6F4090u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x821FDC827D6F4090u64;
        
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
pub fn _special_ability_activate_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x821FDC827D6F4090u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x821FDC827D6F4090u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ### Warning
This native will return `0` if the last vehicle the player was in was destroyed.



pub fn get_players_last_vehicle_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6997A7EB3F5C8C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6997A7EB3F5C8C0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_players_last_vehicle_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6997A7EB3F5C8C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6997A7EB3F5C8C0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Gets the ped for a specified player index.



pub fn get_player_ped_safe(
        
        
            playerId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43A66C31C68491C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43A66C31C68491C0u64;
        
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
pub fn get_player_ped_raw(
        playerId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43A66C31C68491C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43A66C31C68491C0u64;

        invoke_raw_typed!(
            hash,
                playerId
        )
    }
}

/// ## Parameters
*



pub fn _0x6e4361ff3e8cd7ca_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E4361FF3E8CD7CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E4361FF3E8CD7CAu64;
        
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
pub fn _0x6e4361ff3e8cd7ca_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E4361FF3E8CD7CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E4361FF3E8CD7CAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_player_noise_multiplier_safe(
        
        
            player: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB89EF50FF25FCE9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB89EF50FF25FCE9u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_noise_multiplier_raw(
        player: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB89EF50FF25FCE9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB89EF50FF25FCE9u64;

        invoke_raw_typed!(
            hash,
                player, 
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn _set_player_fall_distance_safe(
        
        
            player: 
        , 
        
        
            distance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFD79FA81DFBA9CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFD79FA81DFBA9CBu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                distance
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_player_fall_distance_raw(
        player: , 
        distance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFD79FA81DFBA9CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFD79FA81DFBA9CBu64;

        invoke_raw_typed!(
            hash,
                player, 
                distance
        )
    }
}

/// ```
Achievements from 0-57
more achievements came with update 1.29 (freemode events update), I'd say that they now go to 60, but I'll need to check.
```



pub fn give_achievement_to_player_safe(
        
        
            achievement: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBEC7076D64130195u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBEC7076D64130195u64;
        
        let result = invoke_raw!(
            hash,
                achievement
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn give_achievement_to_player_raw(
        achievement: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBEC7076D64130195u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBEC7076D64130195u64;

        invoke_raw_typed!(
            hash,
                achievement
        )
    }
}

/// ```
This can be between 1.0f - 14.9f   
You can change the max in IDA from 15.0. I say 15.0 as the function blrs if what you input is greater than or equal to 15.0 hence why it's 14.9 max default.  
On PC the multiplier can be between 0.0f and 50.0f (inclusive).  
```



pub fn set_air_drag_multiplier_for_players_vehicle_safe(
        
        
            player: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA7DC8329F0A1E9Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA7DC8329F0A1E9Eu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_air_drag_multiplier_for_players_vehicle_raw(
        player: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCA7DC8329F0A1E9Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCA7DC8329F0A1E9Eu64;

        invoke_raw_typed!(
            hash,
                player, 
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn set_player_bluetooth_state_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DC40A8869C22141u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DC40A8869C22141u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_bluetooth_state_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DC40A8869C22141u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DC40A8869C22141u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_player_weapon_defense_modifier_safe(
        
        
            player: 
        , 
        
        
            modifier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D83BC011CA14A3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D83BC011CA14A3Cu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                modifier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_weapon_defense_modifier_raw(
        player: , 
        modifier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D83BC011CA14A3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D83BC011CA14A3Cu64;

        invoke_raw_typed!(
            hash,
                player, 
                modifier
        )
    }
}

/// ```
Default is 100. Use player id and not ped id. For instance: PLAYER::SET_PLAYER_MAX_ARMOUR(PLAYER::PLAYER_ID(), 100); // main_persistent.ct4  
```



pub fn set_player_max_armour_safe(
        
        
            player: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77DFCCF5948B8C71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77DFCCF5948B8C71u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_max_armour_raw(
        player: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77DFCCF5948B8C71u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77DFCCF5948B8C71u64;

        invoke_raw_typed!(
            hash,
                player, 
                value
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn special_ability_unlock_safe(
        
        
            playerModel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF145F3BE2EFA9A3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF145F3BE2EFA9A3Bu64;
        
        let result = invoke_raw!(
            hash,
                playerModel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn special_ability_unlock_raw(
        playerModel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF145F3BE2EFA9A3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF145F3BE2EFA9A3Bu64;

        invoke_raw_typed!(
            hash,
                playerModel
        )
    }
}

/// ## Parameters
*



pub fn is_player_targetting_entity_safe(
        
        
            player: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7912F7FC4F6264B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7912F7FC4F6264B6u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_player_targetting_entity_raw(
        player: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7912F7FC4F6264B6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7912F7FC4F6264B6u64;

        invoke_raw_typed!(
            hash,
                player, 
                entity
        )
    }
}

/// Sets whether the player is able to do drive-bys in vehicle (shooting & aiming in vehicles), this also includes middle finger taunts.

This is a toggle, it does not have to be ran every frame.



pub fn set_player_can_do_drive_by_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E8834B52EC20C77u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E8834B52EC20C77u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_can_do_drive_by_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E8834B52EC20C77u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E8834B52EC20C77u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_disable_ambient_melee_move_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E8AABFA40A84F8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E8AABFA40A84F8Cu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_disable_ambient_melee_move_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E8AABFA40A84F8Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E8AABFA40A84F8Cu64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ```
Gets a value indicating whether the specified player is currently aiming freely.  
```



pub fn is_player_free_aiming_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E397FD2ECD37C87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E397FD2ECD37C87u64;
        
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
pub fn is_player_free_aiming_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E397FD2ECD37C87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E397FD2ECD37C87u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Tints:  
None = -1,  
Rainbow = 0,  
Red = 1,  
SeasideStripes = 2,  
WidowMaker = 3,  
Patriot = 4,  
Blue = 5,  
Black = 6,  
Hornet = 7,  
AirFocce = 8,  
Desert = 9,  
Shadow = 10,  
HighAltitude = 11,  
Airbone = 12,  
Sunrise = 13,  
```



pub fn get_player_reserve_parachute_tint_index_safe(
        
        
            player: 
        , 
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5A016BC3C09CF40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5A016BC3C09CF40u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_player_reserve_parachute_tint_index_raw(
        player: , 
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5A016BC3C09CF40u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5A016BC3C09CF40u64;

        invoke_raw_typed!(
            hash,
                player, 
                index
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn special_ability_lock_safe(
        
        
            playerModel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A09D0D590A47D13u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A09D0D590A47D13u64;
        
        let result = invoke_raw!(
            hash,
                playerModel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn special_ability_lock_raw(
        playerModel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6A09D0D590A47D13u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6A09D0D590A47D13u64;

        invoke_raw_typed!(
            hash,
                playerModel
        )
    }
}

/// ```
Returns the group ID the player is member of.  
```



pub fn get_player_group_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D127585F77030AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D127585F77030AFu64;
        
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
pub fn get_player_group_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D127585F77030AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D127585F77030AFu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn get_time_since_player_drove_on_pavement_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD559D2BE9E37853Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD559D2BE9E37853Bu64;
        
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
pub fn get_time_since_player_drove_on_pavement_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD559D2BE9E37853Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD559D2BE9E37853Bu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// Returns the players name from a specified player index



pub fn get_player_name_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D0DE6A7B5DA71F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D0DE6A7B5DA71F8u64;
        
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
pub fn get_player_name_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D0DE6A7B5DA71F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D0DE6A7B5DA71F8u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn _0x7e07c78925d5fd96_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E07C78925D5FD96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E07C78925D5FD96u64;
        
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
pub fn _0x7e07c78925d5fd96_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7E07C78925D5FD96u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7E07C78925D5FD96u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```c
enum eViolationType {
  // Checks if the player is driving on pedestrians walk ways
  VT_PAVED_PEDESTRIAN_AREAS = 0,
  // Checks if the player is running through red lights
  // This takes some time to return true.
  VT_RUNNING_REDS = 1,
  // checks if the player is driving on the wrong side of the road
  VT_AGAINST_TRAFFIC = 2
};
```

Used solely in "Al Di Napoli" with type 2 for a voiceline.



pub fn _is_player_driving_dangerously_safe(
        
        
            player: 
        , 
        
        
            type: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF10B44FD479D69F3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF10B44FD479D69F3u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                type
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_player_driving_dangerously_raw(
        player: , 
        type: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF10B44FD479D69F3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF10B44FD479D69F3u64;

        invoke_raw_typed!(
            hash,
                player, 
                type
        )
    }
}

/// ## Parameters
*



pub fn _set_player_invincible_keep_ragdoll_enabled_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BC97F4F4BB3C04Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BC97F4F4BB3C04Bu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_player_invincible_keep_ragdoll_enabled_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BC97F4F4BB3C04Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BC97F4F4BB3C04Bu64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ```
Returns the Player's Invincible status.  
This function will always return false if 0x733A643B5B0C53C1 is used to set the invincibility status. To always get the correct result, use this:  
	bool IsPlayerInvincible(Player player)  
	{  
auto addr = getScriptHandleBaseAddress(GET_PLAYER_PED(player));	  
if (addr)  
{  
	DWORD flag = *(DWORD *)(addr + 0x188);  
	return ((flag & (1 << 8)) != 0) || ((flag & (1 << 9)) != 0);  
}  
return false;  
	}  
============================================================  
This has bothered me for too long, whoever may come across this, where did anyone ever come up with this made up hash? 0x733A643B5B0C53C1 I've looked all over old hash list, and this nativedb I can not find that PC hash anywhere. What native name is it now or was it?  
```



pub fn get_player_invincible_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB721981B2B939E07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB721981B2B939E07u64;
        
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
pub fn get_player_invincible_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB721981B2B939E07u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB721981B2B939E07u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
NativeDB Added Parameter 3: BOOL p2
```



pub fn set_player_melee_weapon_damage_modifier_safe(
        
        
            player: 
        , 
        
        
            modifier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A3DC7ECCC321032u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A3DC7ECCC321032u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                modifier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_melee_weapon_damage_modifier_raw(
        player: , 
        modifier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A3DC7ECCC321032u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A3DC7ECCC321032u64;

        invoke_raw_typed!(
            hash,
                player, 
                modifier
        )
    }
}

/// ## Parameters
*



pub fn set_wanted_level_multiplier_safe(
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x020E5F00CDA207BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x020E5F00CDA207BAu64;
        
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
pub fn set_wanted_level_multiplier_raw(
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x020E5F00CDA207BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x020E5F00CDA207BAu64;

        invoke_raw_typed!(
            hash,
                multiplier
        )
    }
}

/// ```
Does exactly the same thing as PLAYER_ID()  
```



pub fn network_player_id_to_int_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE68096F9F37341Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE68096F9F37341Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn network_player_id_to_int_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE68096F9F37341Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE68096F9F37341Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns the time since the character was arrested in (ms) milliseconds.  
example  
var time = Function.call<int>(Hash.GET_TIME_SINCE_LAST_ARREST();  
UI.DrawSubtitle(time.ToString());  
if player has not been arrested, the int returned will be -1.  
```



pub fn get_time_since_last_arrest_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5063F92F07C2A316u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5063F92F07C2A316u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_time_since_last_arrest_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5063F92F07C2A316u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5063F92F07C2A316u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ASSISTED_MOVEMENT_CLOSE_ROUTE native function



pub fn assisted_movement_close_route_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAEBF081FFC0A0E5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAEBF081FFC0A0E5Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn assisted_movement_close_route_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAEBF081FFC0A0E5Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAEBF081FFC0A0E5Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_player_max_armour_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92659B4CE1863CB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92659B4CE1863CB3u64;
        
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
pub fn get_player_max_armour_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92659B4CE1863CB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92659B4CE1863CB3u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn get_player_wanted_centre_position_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C92BA89F1AF26F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C92BA89F1AF26F8u64;
        
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
pub fn get_player_wanted_centre_position_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0C92BA89F1AF26F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0C92BA89F1AF26F8u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ADD_*

```
NativeDB Introduced: v1868
```



pub fn _0x9097eb6d4bb9a12a_safe(
        
        
            player: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9097EB6D4BB9A12Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9097EB6D4BB9A12Au64;
        
        let result = invoke_raw!(
            hash,
                player, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x9097eb6d4bb9a12a_raw(
        player: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9097EB6D4BB9A12Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9097EB6D4BB9A12Au64;

        invoke_raw_typed!(
            hash,
                player, 
                entity
        )
    }
}

/// ```
Returns true if an unk value is greater than 0.0f  
```



pub fn is_player_battle_aware_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38D28DA81E4E9BF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38D28DA81E4E9BF9u64;
        
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
pub fn is_player_battle_aware_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x38D28DA81E4E9BF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x38D28DA81E4E9BF9u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn is_player_targetting_anything_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78CFE51896B6B8A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78CFE51896B6B8A4u64;
        
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
pub fn is_player_targetting_anything_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78CFE51896B6B8A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78CFE51896B6B8A4u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn clear_player_parachute_pack_model_override_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10C54E4389C12B42u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10C54E4389C12B42u64;
        
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
pub fn clear_player_parachute_pack_model_override_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x10C54E4389C12B42u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x10C54E4389C12B42u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn get_player_parachute_smoke_trail_color_safe(
        
        
            player: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF56DBABD3CD4887u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF56DBABD3CD4887u64;
        
        let result = invoke_raw!(
            hash,
                player, 
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
pub fn get_player_parachute_smoke_trail_color_raw(
        player: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEF56DBABD3CD4887u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEF56DBABD3CD4887u64;

        invoke_raw_typed!(
            hash,
                player, 
                r, 
                g, 
                b
        )
    }
}

/// ## Parameters
*



pub fn get_player_has_reserve_parachute_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DDFE2FF727F3CA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DDFE2FF727F3CA3u64;
        
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
pub fn get_player_has_reserve_parachute_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DDFE2FF727F3CA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DDFE2FF727F3CA3u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn get_time_since_player_hit_ped_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE36A25322DC35F42u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE36A25322DC35F42u64;
        
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
pub fn get_time_since_player_hit_ped_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE36A25322DC35F42u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE36A25322DC35F42u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// Returns the player index for the local player.



pub fn player_id_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F8644AF03D0E0D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F8644AF03D0E0D6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn player_id_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4F8644AF03D0E0D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4F8644AF03D0E0D6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Returns TRUE if it found an entity in your crosshair within range of your weapon. Assigns the handle of the target to the *entity that you pass it.  
Returns false if no entity found.  
```



pub fn get_entity_player_is_free_aiming_at_safe(
        
        
            player: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2975C866E6713290u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2975C866E6713290u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_entity_player_is_free_aiming_at_raw(
        player: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2975C866E6713290u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2975C866E6713290u64;

        invoke_raw_typed!(
            hash,
                player, 
                entity
        )
    }
}

/// ## Parameters
*



pub fn has_player_damaged_at_least_one_ped_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20CE80B0C2BF4ACCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20CE80B0C2BF4ACCu64;
        
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
pub fn has_player_damaged_at_least_one_ped_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20CE80B0C2BF4ACCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20CE80B0C2BF4ACCu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Return value



pub fn get_cause_of_most_recent_force_cleanup_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A41CF4674A12272u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A41CF4674A12272u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cause_of_most_recent_force_cleanup_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A41CF4674A12272u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A41CF4674A12272u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Establishes a reset flag to prevent the player from entering any vehicle. Not that this native must be called every frame.



pub fn set_player_may_not_enter_any_vehicle_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DE37BBF9E9CC14Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DE37BBF9E9CC14Au64;
        
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
pub fn set_player_may_not_enter_any_vehicle_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DE37BBF9E9CC14Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DE37BBF9E9CC14Au64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Affects the range of auto aim target.  
```



pub fn set_player_lockon_range_override_safe(
        
        
            player: 
        , 
        
        
            range: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29961D490E5814FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29961D490E5814FDu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                range
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_lockon_range_override_raw(
        player: , 
        range: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x29961D490E5814FDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x29961D490E5814FDu64;

        invoke_raw_typed!(
            hash,
                player, 
                range
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _clear_player_reserve_parachute_model_override_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x290D248E25815AE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x290D248E25815AE8u64;
        
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
pub fn _clear_player_reserve_parachute_model_override_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x290D248E25815AE8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x290D248E25815AE8u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// Make the player impervious to all forms of damage.



pub fn set_player_invincible_safe(
        
        
            player: 
        , 
        
        
            bInvincible: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x239528EACDC3E7DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x239528EACDC3E7DEu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                bInvincible
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_invincible_raw(
        player: , 
        bInvincible: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x239528EACDC3E7DEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x239528EACDC3E7DEu64;

        invoke_raw_typed!(
            hash,
                player, 
                bInvincible
        )
    }
}

/// ```
6 matches across 4 scripts. 5 occurrences were 240. The other was 255.  
```



pub fn set_player_cloth_lock_counter_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14D913B777DFF5DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14D913B777DFF5DAu64;
        
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
pub fn set_player_cloth_lock_counter_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14D913B777DFF5DAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14D913B777DFF5DAu64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ```
Also known as _RECHARGE_SPECIAL_ABILITY
```

```
NativeDB Added Parameter 3: Any p2
```



pub fn special_ability_fill_meter_safe(
        
        
            player: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DACA8DDC6FD4980u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DACA8DDC6FD4980u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn special_ability_fill_meter_raw(
        player: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3DACA8DDC6FD4980u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3DACA8DDC6FD4980u64;

        invoke_raw_typed!(
            hash,
                player, 
                p1
        )
    }
}

/// ```
Seems to only appear in scripts used in Singleplayer.  
Always used like this in scripts  
PLAYER::_BC9490CA15AEA8FB(PLAYER::PLAYER_ID());  
```



pub fn _0xbc9490ca15aea8fb_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC9490CA15AEA8FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC9490CA15AEA8FBu64;
        
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
pub fn _0xbc9490ca15aea8fb_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC9490CA15AEA8FBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC9490CA15AEA8FBu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn clear_player_parachute_model_override_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8753997EB5F6EE3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8753997EB5F6EE3Fu64;
        
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
pub fn clear_player_parachute_model_override_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8753997EB5F6EE3Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8753997EB5F6EE3Fu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// For Steam.
Does nothing and always returns false in the retail version of the game.



pub fn _set_achievement_progress_safe(
        
        
            achievement: 
        , 
        
        
            progress: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2AFFFDABBDC2C5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2AFFFDABBDC2C5Cu64;
        
        let result = invoke_raw!(
            hash,
                achievement, 
                progress
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_achievement_progress_raw(
        achievement: , 
        progress: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC2AFFFDABBDC2C5Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC2AFFFDABBDC2C5Cu64;

        invoke_raw_typed!(
            hash,
                achievement, 
                progress
        )
    }
}

/// ## Parameters
*



pub fn report_police_spotted_player_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC64D2C53493ED12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC64D2C53493ED12u64;
        
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
pub fn report_police_spotted_player_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC64D2C53493ED12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC64D2C53493ED12u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// REMOVE_*

```
NativeDB Introduced: v1868
```



pub fn _0x9f260bfb59adbca3_safe(
        
        
            player: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F260BFB59ADBCA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F260BFB59ADBCA3u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x9f260bfb59adbca3_raw(
        player: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F260BFB59ADBCA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F260BFB59ADBCA3u64;

        invoke_raw_typed!(
            hash,
                player, 
                entity
        )
    }
}

/// ```
NativeDB Introduced: v323
```



pub fn reset_world_boundary_for_player_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA1DF03D5A315F4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA1DF03D5A315F4Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn reset_world_boundary_for_player_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA1DF03D5A315F4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA1DF03D5A315F4Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Sets whether this player can be hassled by gangs.  
```



pub fn set_player_can_be_hassled_by_gangs_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5E460AD7020A246u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5E460AD7020A246u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_can_be_hassled_by_gangs_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD5E460AD7020A246u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD5E460AD7020A246u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ```
PLAYER::REPORT_CRIME(PLAYER::PLAYER_ID(), 37, PLAYER::GET_WANTED_LEVEL_THRESHOLD(1));  
From am_armybase.ysc.c4:  
PLAYER::REPORT_CRIME(PLAYER::PLAYER_ID(4), 36, PLAYER::GET_WANTED_LEVEL_THRESHOLD(4));



pub fn report_crime_safe(
        
        
            player: 
        , 
        
        
            crimeType: 
        , 
        
        
            wantedLvlThresh: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE9B09589827545E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE9B09589827545E7u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                crimeType, 
                wantedLvlThresh
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn report_crime_raw(
        player: , 
        crimeType: , 
        wantedLvlThresh: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE9B09589827545E7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE9B09589827545E7u64;

        invoke_raw_typed!(
            hash,
                player, 
                crimeType, 
                wantedLvlThresh
        )
    }
}

/// ```
example:  
PLAYER::SET_PLAYER_PARACHUTE_MODEL_OVERRIDE(PLAYER::PLAYER_ID(), 0x73268708);  
```



pub fn set_player_parachute_model_override_safe(
        
        
            player: 
        , 
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x977DB4641F6FC3DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x977DB4641F6FC3DBu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_parachute_model_override_raw(
        player: , 
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x977DB4641F6FC3DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x977DB4641F6FC3DBu64;

        invoke_raw_typed!(
            hash,
                player, 
                model
        )
    }
}

/// ## Parameters
*



pub fn can_ped_hear_player_safe(
        
        
            player: 
        , 
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF297383AA91DCA29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF297383AA91DCA29u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                ped
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_ped_hear_player_raw(
        player: , 
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF297383AA91DCA29u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF297383AA91DCA29u64;

        invoke_raw_typed!(
            hash,
                player, 
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_player_has_reserve_parachute_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DDAB28D31FAC363u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DDAB28D31FAC363u64;
        
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
pub fn set_player_has_reserve_parachute_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7DDAB28D31FAC363u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7DDAB28D31FAC363u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn get_player_sprint_stamina_remaining_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F9F16F8E65A7ED7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F9F16F8E65A7ED7u64;
        
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
pub fn get_player_sprint_stamina_remaining_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3F9F16F8E65A7ED7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3F9F16F8E65A7ED7u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn _special_ability_deplete_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17F7471EACA78290u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17F7471EACA78290u64;
        
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
pub fn _special_ability_deplete_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x17F7471EACA78290u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x17F7471EACA78290u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_everyone_ignore_player_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EEDA153AD141BA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EEDA153AD141BA4u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_everyone_ignore_player_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8EEDA153AD141BA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8EEDA153AD141BA4u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _set_player_reserve_parachute_model_override_safe(
        
        
            player: 
        , 
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0764486AEDE748DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0764486AEDE748DBu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_player_reserve_parachute_model_override_raw(
        player: , 
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0764486AEDE748DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0764486AEDE748DBu64;

        invoke_raw_typed!(
            hash,
                player, 
                model
        )
    }
}

/// ```
Remnant from GTA IV. Does nothing in GTA V.
```



pub fn get_wanted_level_radius_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x085DEB493BE80812u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x085DEB493BE80812u64;
        
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
pub fn get_wanted_level_radius_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x085DEB493BE80812u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x085DEB493BE80812u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn set_player_stealth_perception_modifier_safe(
        
        
            player: 
        , 
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E9021C1FCDD507Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E9021C1FCDD507Au64;
        
        let result = invoke_raw!(
            hash,
                player, 
                value
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_stealth_perception_modifier_raw(
        player: , 
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4E9021C1FCDD507Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4E9021C1FCDD507Au64;

        invoke_raw_typed!(
            hash,
                player, 
                value
        )
    }
}

/// Set the player's current team.



pub fn set_player_team_safe(
        
        
            player: 
        , 
        
        
            team: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0299FA38396A4940u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0299FA38396A4940u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                team
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_team_raw(
        player: , 
        team: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0299FA38396A4940u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0299FA38396A4940u64;

        invoke_raw_typed!(
            hash,
                player, 
                team
        )
    }
}

/// ```
p1 was always true.
```

```
NativeDB Added Parameter 3: Any p2
```



pub fn special_ability_deplete_meter_safe(
        
        
            player: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D506DBBBC51E64Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D506DBBBC51E64Bu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn special_ability_deplete_meter_raw(
        player: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1D506DBBBC51E64Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1D506DBBBC51E64Bu64;

        invoke_raw_typed!(
            hash,
                player, 
                p1
        )
    }
}

/// Set the model for a specific Player. Note that this will destroy the current Ped for the Player and create a new one, any reference to the old ped will be invalid after calling this.

As per usual, make sure to request the model first and wait until it has loaded.



pub fn set_player_model_safe(
        
        
            player: 
        , 
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00A1CADD00108836u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00A1CADD00108836u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_model_raw(
        player: , 
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x00A1CADD00108836u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x00A1CADD00108836u64;

        invoke_raw_typed!(
            hash,
                player, 
                model
        )
    }
}

/// ## Parameters
*



pub fn get_player_wanted_level_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE28E54788CE8F12Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE28E54788CE8F12Du64;
        
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
pub fn get_player_wanted_level_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE28E54788CE8F12Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE28E54788CE8F12Du64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
This has been found in use in the decompiled files.  
```



pub fn _0xad73ce5a09e42d12_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD73CE5A09E42D12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD73CE5A09E42D12u64;
        
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
pub fn _0xad73ce5a09e42d12_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD73CE5A09E42D12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD73CE5A09E42D12u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
NativeDB Added Parameter 3: Any p2
```



pub fn _set_special_ability_safe(
        
        
            player: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB214D570EAD7F81Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB214D570EAD7F81Au64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_special_ability_raw(
        player: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB214D570EAD7F81Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB214D570EAD7F81Au64;

        invoke_raw_typed!(
            hash,
                player, 
                p1
        )
    }
}

/// ```
Sets your targeting mode.
0 = Assisted Aim - Full
1 = Assisted Aim - Partial
2 = Free Aim - Assisted
3 = Free Aim
```



pub fn set_player_targeting_mode_safe(
        
        
            targetMode: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1906895227793F3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1906895227793F3u64;
        
        let result = invoke_raw!(
            hash,
                targetMode
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_targeting_mode_raw(
        targetMode: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1906895227793F3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1906895227793F3u64;

        invoke_raw_typed!(
            hash,
                targetMode
        )
    }
}

/// ```
Used with radios:
void sub_cf383(auto _a0) {
    if ((a_0)==1) {
        if (MISC::IS_BIT_SET((g_240005._f1), 3)) {
            PLAYER::_2F7CEB6520288061(0);
            AUDIO::SET_AUDIO_FLAG("AllowRadioDuringSwitch", 0);
            AUDIO::SET_MOBILE_PHONE_RADIO_STATE(0);
            AUDIO::SET_AUDIO_FLAG("MobileRadioInGame", 0);
        }
        sub_cf3f6(1);
    } else {
        if (MISC::IS_BIT_SET((g_240005._f1), 3)) {
            PLAYER::_2F7CEB6520288061(1);
            AUDIO::SET_AUDIO_FLAG("AllowRadioDuringSwitch", 1);
            AUDIO::SET_MOBILE_PHONE_RADIO_STATE(1);
            AUDIO::SET_AUDIO_FLAG("MobileRadioInGame", 1);
        }
        sub_cf3f6(0);
    }
}
SET_PLAYER_S*
```



pub fn _0x2f7ceb6520288061_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F7CEB6520288061u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F7CEB6520288061u64;
        
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
pub fn _0x2f7ceb6520288061_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F7CEB6520288061u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F7CEB6520288061u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Multiplier goes up to 1.49 any value above will be completely overruled by the game and the multiplier will not take effect, this can be edited in memory however.  
Just call it one time, it is not required to be called once every tick.  
Note: At least the IDA method if you change the max float multiplier from 1.5 it will change it for both this and SWIM above. I say 1.5 as the function blrs if what you input is greater than or equal to 1.5 hence why it's 1.49 max default.  
It is not possible to "decrease" speed. Anything below 1 will be ignored.  
```



pub fn set_run_sprint_multiplier_for_player_safe(
        
        
            player: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6DB47AA77FD94E09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6DB47AA77FD94E09u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_run_sprint_multiplier_for_player_raw(
        player: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6DB47AA77FD94E09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6DB47AA77FD94E09u64;

        invoke_raw_typed!(
            hash,
                player, 
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn are_player_flashing_stars_about_to_drop_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFAF86043E5874E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFAF86043E5874E9u64;
        
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
pub fn are_player_flashing_stars_about_to_drop_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFAF86043E5874E9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFAF86043E5874E9u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn reset_player_input_gait_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19531C47A2ABD691u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19531C47A2ABD691u64;
        
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
pub fn reset_player_input_gait_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x19531C47A2ABD691u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x19531C47A2ABD691u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// Seems to lock the underwater timer of the specified player. Set `percentage` to `50.0` will reduce the value of [GET_PLAYER_UNDERWATER_TIME_REMAINING](#_0xA1FCF8E6AF40B731) to 5.0.

If you want to increase the underwater time for ped, use [SET_PED_MAX_TIME_UNDERWATER](#_0x6BA428C528D9E522) instead.

Using this native after [SET_PED_MAX_TIME_UNDERWATER](#_0x6BA428C528D9E522)



pub fn _set_player_underwater_time_remaining_safe(
        
        
            player: 
        , 
        
        
            percentage: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0D3E4F7AAFB7E78u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0D3E4F7AAFB7E78u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                percentage
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_player_underwater_time_remaining_raw(
        player: , 
        percentage: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0D3E4F7AAFB7E78u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0D3E4F7AAFB7E78u64;

        invoke_raw_typed!(
            hash,
                player, 
                percentage
        )
    }
}

/// ## Parameters
*



pub fn get_player_current_stealth_noise_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F395D61F3A1F877u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F395D61F3A1F877u64;
        
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
pub fn get_player_current_stealth_noise_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F395D61F3A1F877u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F395D61F3A1F877u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn reset_player_stamina_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6F312FCCE9C1DFEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6F312FCCE9C1DFEu64;
        
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
pub fn reset_player_stamina_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA6F312FCCE9C1DFEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA6F312FCCE9C1DFEu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn _0x55fcc0c390620314_safe(
        
        
            player1: 
        , 
        
        
            player2: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55FCC0C390620314u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55FCC0C390620314u64;
        
        let result = invoke_raw!(
            hash,
                player1, 
                player2, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x55fcc0c390620314_raw(
        player1: , 
        player2: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x55FCC0C390620314u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x55FCC0C390620314u64;

        invoke_raw_typed!(
            hash,
                player1, 
                player2, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_ignore_low_priority_shocking_events_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x596976B02B6B5700u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x596976B02B6B5700u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ignore_low_priority_shocking_events_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x596976B02B6B5700u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x596976B02B6B5700u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ```
this function is hard-coded to always return 0.  
```



pub fn is_player_logging_in_np_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74556E1420867ECAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74556E1420867ECAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_player_logging_in_np_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x74556E1420867ECAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x74556E1420867ECAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Values around 1.0f to 2.0f used in game scripts.  
```



pub fn set_player_sneaking_noise_multiplier_safe(
        
        
            player: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2C1A29588A9F47Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2C1A29588A9F47Cu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_sneaking_noise_multiplier_raw(
        player: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2C1A29588A9F47Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2C1A29588A9F47Cu64;

        invoke_raw_typed!(
            hash,
                player, 
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn set_player_parachute_pack_model_override_safe(
        
        
            player: 
        , 
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC80A4C2F18A2B64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC80A4C2F18A2B64u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_parachute_pack_model_override_raw(
        player: , 
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC80A4C2F18A2B64u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC80A4C2F18A2B64u64;

        invoke_raw_typed!(
            hash,
                player, 
                model
        )
    }
}

/// ## Parameters
*



pub fn _update_player_teleport_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE23D5873C2394C61u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE23D5873C2394C61u64;
        
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
pub fn _update_player_teleport_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE23D5873C2394C61u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE23D5873C2394C61u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Appears only 3 times in the scripts, more specifically in michael1.ysc
-
This can be used to prevent dying if you are "out of the world"
```



pub fn extend_world_boundary_for_player_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5006D96C995A5827u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5006D96C995A5827u64;
        
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
pub fn extend_world_boundary_for_player_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5006D96C995A5827u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5006D96C995A5827u64;

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



pub fn change_player_ped_safe(
        
        
            player: 
        , 
        
        
            ped: 
        , 
        
        
            b2: 
        , 
        
        
            resetDamage: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x048189FAC643DEEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x048189FAC643DEEEu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                ped, 
                b2, 
                resetDamage
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn change_player_ped_raw(
        player: , 
        ped: , 
        b2: , 
        resetDamage: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x048189FAC643DEEEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x048189FAC643DEEEu64;

        invoke_raw_typed!(
            hash,
                player, 
                ped, 
                b2, 
                resetDamage
        )
    }
}

/// ```
Tints:  
None = -1,  
Rainbow = 0,  
Red = 1,  
SeasideStripes = 2,  
WidowMaker = 3,  
Patriot = 4,  
Blue = 5,  
Black = 6,  
Hornet = 7,  
AirFocce = 8,  
Desert = 9,  
Shadow = 10,  
HighAltitude = 11,  
Airbone = 12,  
Sunrise = 13,  
```



pub fn set_player_reserve_parachute_tint_index_safe(
        
        
            player: 
        , 
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF04C87F5DC1DF38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF04C87F5DC1DF38u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_reserve_parachute_tint_index_raw(
        player: , 
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAF04C87F5DC1DF38u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAF04C87F5DC1DF38u64;

        invoke_raw_typed!(
            hash,
                player, 
                index
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _get_player_parachute_model_override_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC219887CA3E65C41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC219887CA3E65C41u64;
        
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
pub fn _get_player_parachute_model_override_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC219887CA3E65C41u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC219887CA3E65C41u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
This executes at the same as speed as PLAYER::SET_PLAYER_WANTED_LEVEL(player, 0, false);  
PLAYER::GET_PLAYER_WANTED_LEVEL(player); executes in less than half the time. Which means that it's worth first checking if the wanted level needs to be cleared before clearing. However, this is mostly about good code practice and can important in other situations. The difference in time in this example is negligible.  
```



pub fn clear_player_wanted_level_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB302540597885499u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB302540597885499u64;
        
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
pub fn clear_player_wanted_level_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB302540597885499u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB302540597885499u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn is_special_ability_unlocked_safe(
        
        
            playerModel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6017F6A6CDFA694u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6017F6A6CDFA694u64;
        
        let result = invoke_raw!(
            hash,
                playerModel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_special_ability_unlocked_raw(
        playerModel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6017F6A6CDFA694u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6017F6A6CDFA694u64;

        invoke_raw_typed!(
            hash,
                playerModel
        )
    }
}

/// ASSISTED_MOVEMENT_FLUSH_ROUTE native function



pub fn assisted_movement_flush_route_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8621390F0CDCFE1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8621390F0CDCFE1Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn assisted_movement_flush_route_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8621390F0CDCFE1Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8621390F0CDCFE1Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// This is to make the player walk without accepting input.

Call this native every frame so you can control the direction of your ped.



pub fn simulate_player_input_gait_safe(
        
        
            player: 
        , 
        
        
            amount: 
        , 
        
        
            gaitType: 
        , 
        
        
            rotationSpeed: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x477D5D63E63ECA5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x477D5D63E63ECA5Du64;
        
        let result = invoke_raw!(
            hash,
                player, 
                amount, 
                gaitType, 
                rotationSpeed, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn simulate_player_input_gait_raw(
        player: , 
        amount: , 
        gaitType: , 
        rotationSpeed: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x477D5D63E63ECA5Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x477D5D63E63ECA5Du64;

        invoke_raw_typed!(
            hash,
                player, 
                amount, 
                gaitType, 
                rotationSpeed, 
                p4, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn _has_player_been_shot_by_cop_safe(
        
        
            player: 
        , 
        
        
            ms: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC0753C9CA14B506u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC0753C9CA14B506u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                ms, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _has_player_been_shot_by_cop_raw(
        player: , 
        ms: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC0753C9CA14B506u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC0753C9CA14B506u64;

        invoke_raw_typed!(
            hash,
                player, 
                ms, 
                p2
        )
    }
}

/// ```
p1 appears as 5, 10, 15, 25, or 30. p2 is always true.
```

```
NativeDB Added Parameter 4: Any p3
```



pub fn special_ability_charge_absolute_safe(
        
        
            player: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7B0870EB531D08Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7B0870EB531D08Du64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn special_ability_charge_absolute_raw(
        player: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7B0870EB531D08Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7B0870EB531D08Du64;

        invoke_raw_typed!(
            hash,
                player, 
                p1, 
                p2
        )
    }
}

/// ```
Max value is 1.0  
```



pub fn set_wanted_level_difficulty_safe(
        
        
            player: 
        , 
        
        
            difficulty: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B0BB33B04405E7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B0BB33B04405E7Au64;
        
        let result = invoke_raw!(
            hash,
                player, 
                difficulty
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_wanted_level_difficulty_raw(
        player: , 
        difficulty: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B0BB33B04405E7Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B0BB33B04405E7Au64;

        invoke_raw_typed!(
            hash,
                player, 
                difficulty
        )
    }
}

/// ## Parameters
*



pub fn set_dispatch_cops_for_player_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB172424876553F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB172424876553F4u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_dispatch_cops_for_player_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB172424876553F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB172424876553F4u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0xfac75988a7d078d3_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAC75988A7D078D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAC75988A7D078D3u64;
        
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
pub fn _0xfac75988a7d078d3_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFAC75988A7D078D3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFAC75988A7D078D3u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn get_time_since_player_drove_against_traffic_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB89591E290D9182u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB89591E290D9182u64;
        
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
pub fn get_time_since_player_drove_against_traffic_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB89591E290D9182u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB89591E290D9182u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// The native ensures the 'modifier' parameter is 0.1 or greater.



pub fn set_player_weapon_damage_modifier_safe(
        
        
            player: 
        , 
        
        
            modifier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE07B9F7817AADA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE07B9F7817AADA3u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                modifier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_weapon_damage_modifier_raw(
        player: , 
        modifier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCE07B9F7817AADA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCE07B9F7817AADA3u64;

        invoke_raw_typed!(
            hash,
                player, 
                modifier
        )
    }
}

/// ## Parameters
*



pub fn set_special_ability_multiplier_safe(
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA49C426ED0CA4AB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA49C426ED0CA4AB7u64;
        
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
pub fn set_special_ability_multiplier_raw(
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA49C426ED0CA4AB7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA49C426ED0CA4AB7u64;

        invoke_raw_typed!(
            hash,
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn _0x9edd76e87d5d51ba_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9EDD76E87D5D51BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9EDD76E87D5D51BAu64;
        
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
pub fn _0x9edd76e87d5d51ba_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9EDD76E87D5D51BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9EDD76E87D5D51BAu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn get_is_player_driving_on_highway_safe(
        
        
            playerId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FC472C501CCADB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FC472C501CCADB3u64;
        
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
pub fn get_is_player_driving_on_highway_raw(
        playerId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FC472C501CCADB3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FC472C501CCADB3u64;

        invoke_raw_typed!(
            hash,
                playerId
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x237440e46d918649_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x237440E46D918649u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x237440E46D918649u64;
        
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
pub fn _0x237440e46d918649_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x237440E46D918649u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x237440E46D918649u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
modifier's min value is 0.1
```



pub fn set_player_melee_weapon_defense_modifier_safe(
        
        
            player: 
        , 
        
        
            modifier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE540335B4ABC4E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE540335B4ABC4E2u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                modifier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_melee_weapon_defense_modifier_raw(
        player: , 
        modifier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAE540335B4ABC4E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAE540335B4ABC4E2u64;

        invoke_raw_typed!(
            hash,
                player, 
                modifier
        )
    }
}

/// ## Parameters
*



pub fn are_player_stars_greyed_out_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A6EB355EE14A2DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A6EB355EE14A2DBu64;
        
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
pub fn are_player_stars_greyed_out_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A6EB355EE14A2DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A6EB355EE14A2DBu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// Returns the entity handle for the local player ped. Note that this entity handle will change after using commands such as SET\_PLAYER\_MODEL.



pub fn player_ped_id_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD80958FC74E988A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD80958FC74E988A6u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn player_ped_id_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD80958FC74E988A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD80958FC74E988A6u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
PLAYER::FORCE_CLEANUP_FOR_ALL_THREADS_WITH_THIS_NAME("pb_prostitute", 1); // Found in decompilation  
```



pub fn force_cleanup_for_all_threads_with_this_name_safe(
        
        
            name: 
        , 
        
        
            cleanupFlags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C68DDDDF0097317u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C68DDDDF0097317u64;
        
        let result = invoke_raw!(
            hash,
                name, 
                cleanupFlags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_cleanup_for_all_threads_with_this_name_raw(
        name: , 
        cleanupFlags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C68DDDDF0097317u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C68DDDDF0097317u64;

        invoke_raw_typed!(
            hash,
                name, 
                cleanupFlags
        )
    }
}

/// ## Parameters
*



pub fn start_firing_amnesty_safe(
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF9BD71691857E48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF9BD71691857E48u64;
        
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
pub fn start_firing_amnesty_raw(
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF9BD71691857E48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF9BD71691857E48u64;

        invoke_raw_typed!(
            hash,
                duration
        )
    }
}

/// ## Parameters
*



pub fn can_player_start_mission_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE7465A27D403C06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE7465A27D403C06u64;
        
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
pub fn can_player_start_mission_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE7465A27D403C06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE7465A27D403C06u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Returns the time since the character died in (ms) milliseconds.  
example  
var time = Function.call<int>(Hash.GET_TIME_SINCE_LAST_DEATH();  
UI.DrawSubtitle(time.ToString());  
if player has not died, the int returned will be -1.  
```



pub fn get_time_since_last_death_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7034807558DDFCAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7034807558DDFCAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_time_since_last_death_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7034807558DDFCAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7034807558DDFCAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_player_leave_ped_behind_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF300C7649724A0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF300C7649724A0Bu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_leave_ped_behind_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFF300C7649724A0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFF300C7649724A0Bu64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn has_achievement_been_passed_safe(
        
        
            achievement: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x867365E111A3B6EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x867365E111A3B6EBu64;
        
        let result = invoke_raw!(
            hash,
                achievement
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_achievement_been_passed_raw(
        achievement: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x867365E111A3B6EBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x867365E111A3B6EBu64;

        invoke_raw_typed!(
            hash,
                achievement
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn is_special_ability_enabled_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1D200FE26AEF3CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1D200FE26AEF3CBu64;
        
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
pub fn is_special_ability_enabled_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1D200FE26AEF3CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1D200FE26AEF3CBu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn has_player_been_spotted_in_stolen_vehicle_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD705740BB0A1CF4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD705740BB0A1CF4Cu64;
        
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
pub fn has_player_been_spotted_in_stolen_vehicle_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD705740BB0A1CF4Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD705740BB0A1CF4Cu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _get_player_reserve_parachute_model_override_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37FAAA68DCA9D08Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37FAAA68DCA9D08Du64;
        
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
pub fn _get_player_reserve_parachute_model_override_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37FAAA68DCA9D08Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37FAAA68DCA9D08Du64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn is_player_dead_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x424D4687FA1E5652u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x424D4687FA1E5652u64;
        
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
pub fn is_player_dead_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x424D4687FA1E5652u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x424D4687FA1E5652u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
2 occurrences in agency_heist3a. p1 was 0.7f then 0.4f.  
```



pub fn _0xdd2620b7b9d16ff1_safe(
        
        
            player: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD2620B7B9D16FF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD2620B7B9D16FF1u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xdd2620b7b9d16ff1_raw(
        player: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDD2620B7B9D16FF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDD2620B7B9D16FF1u64;

        invoke_raw_typed!(
            hash,
                player, 
                p1
        )
    }
}

/// ```
Simply returns whatever is passed to it (Regardless of whether the handle is valid or not).  
```



pub fn int_to_playerindex_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41BD2A6B006AF756u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41BD2A6B006AF756u64;
        
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
pub fn int_to_playerindex_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41BD2A6B006AF756u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41BD2A6B006AF756u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Parameters
*



pub fn set_max_wanted_level_safe(
        
        
            maxWantedLevel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA5F02DB48D704B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA5F02DB48D704B9u64;
        
        let result = invoke_raw!(
            hash,
                maxWantedLevel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_max_wanted_level_raw(
        maxWantedLevel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA5F02DB48D704B9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA5F02DB48D704B9u64;

        invoke_raw_typed!(
            hash,
                maxWantedLevel
        )
    }
}

/// ## Parameters
*



pub fn clear_player_parachute_variation_override_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F4CC924CF8C7B21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F4CC924CF8C7B21u64;
        
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
pub fn clear_player_parachute_variation_override_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0F4CC924CF8C7B21u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0F4CC924CF8C7B21u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn has_player_left_the_world_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD55DDFB47991A294u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD55DDFB47991A294u64;
        
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
pub fn has_player_left_the_world_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD55DDFB47991A294u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD55DDFB47991A294u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn get_player_rgb_colour_safe(
        
        
            player: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE902EF951DCE178Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE902EF951DCE178Fu64;
        
        let result = invoke_raw!(
            hash,
                player, 
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
pub fn get_player_rgb_colour_raw(
        player: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE902EF951DCE178Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE902EF951DCE178Fu64;

        invoke_raw_typed!(
            hash,
                player, 
                r, 
                g, 
                b
        )
    }
}

/// ```
Gets the number of players in the current session.
If not multiplayer, always returns 1.
```



pub fn get_number_of_players_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x407C7F91DDB46C16u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x407C7F91DDB46C16u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_number_of_players_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x407C7F91DDB46C16u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x407C7F91DDB46C16u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x2f41a3bae005e5fa_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F41A3BAE005E5FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F41A3BAE005E5FAu64;
        
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
pub fn _0x2f41a3bae005e5fa_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F41A3BAE005E5FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F41A3BAE005E5FAu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Name between DISABLE_ALL_CONTROL_ACTIONS and DISABLE_CONTROL_ACTION
```



pub fn _0x5501b7a5cdb79d37_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5501B7A5CDB79D37u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5501B7A5CDB79D37u64;
        
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
pub fn _0x5501b7a5cdb79d37_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5501B7A5CDB79D37u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5501B7A5CDB79D37u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn is_special_ability_meter_full_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x05A1FE504B7F2587u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x05A1FE504B7F2587u64;
        
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
pub fn is_special_ability_meter_full_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x05A1FE504B7F2587u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x05A1FE504B7F2587u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// Always returns false.

```
NativeDB Introduced: v1868
```



pub fn _0xdcc07526b8ec45af_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCC07526B8EC45AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCC07526B8EC45AFu64;
        
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
pub fn _0xdcc07526b8ec45af_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCC07526B8EC45AFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCC07526B8EC45AFu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn set_player_wanted_level_safe(
        
        
            player: 
        , 
        
        
            wantedLevel: 
        , 
        
        
            delayedResponse: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39FF19C64EF7DA5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39FF19C64EF7DA5Bu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                wantedLevel, 
                delayedResponse
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_wanted_level_raw(
        player: , 
        wantedLevel: , 
        delayedResponse: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39FF19C64EF7DA5Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39FF19C64EF7DA5Bu64;

        invoke_raw_typed!(
            hash,
                player, 
                wantedLevel, 
                delayedResponse
        )
    }
}

/// ```
Return true while player is being arrested / busted.  
If atArresting is set to 1, this function will return 1 when player is being arrested (while player is putting his hand up, but still have control)  
If atArresting is set to 0, this function will return 1 only when the busted screen is shown.  
```



pub fn is_player_being_arrested_safe(
        
        
            player: 
        , 
        
        
            atArresting: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x388A47C51ABDAC8Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x388A47C51ABDAC8Eu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                atArresting
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_player_being_arrested_raw(
        player: , 
        atArresting: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x388A47C51ABDAC8Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x388A47C51ABDAC8Eu64;

        invoke_raw_typed!(
            hash,
                player, 
                atArresting
        )
    }
}

/// ## Parameters
*



pub fn set_auto_give_parachute_when_enter_plane_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F343285A00B4BB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F343285A00B4BB6u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_auto_give_parachute_when_enter_plane_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F343285A00B4BB6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F343285A00B4BB6u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ```
Returns true when the player is not able to control the cam i.e. when running a benchmark test, switching the player or viewing a cutscene.  
Note: I am not 100% sure if the native actually checks if the cam control is disabled but it seems promising.  
```



pub fn _is_player_cam_control_disabled_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C814D2FB49F40C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C814D2FB49F40C0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_player_cam_control_disabled_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C814D2FB49F40C0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C814D2FB49F40C0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Purpose of the BOOL currently unknown.  
Both, true and false, work  
```



pub fn display_system_signin_ui_safe(
        
        
            unk: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94DD7888C10A979Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94DD7888C10A979Eu64;
        
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
pub fn display_system_signin_ui_raw(
        unk: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x94DD7888C10A979Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x94DD7888C10A979Eu64;

        invoke_raw_typed!(
            hash,
                unk
        )
    }
}

/// ```
Only 1 match. ob_sofa_michael.  
PLAYER::PLAYER_ATTACH_VIRTUAL_BOUND(-804.5928f, 173.1801f, 71.68436f, 0f, 0f, 0.590625f, 1f, 0.7f);1.0.335.2, 1.0.350.1/2, 1.0.372.2, 1.0.393.2, 1.0.393.4, 1.0.463.1;  
```



pub fn player_attach_virtual_bound_safe(
        
        
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
        let hash = 0xED51733DC73AED51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED51733DC73AED51u64;
        
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
pub fn player_attach_virtual_bound_raw(
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
        let hash = 0xED51733DC73AED51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED51733DC73AED51u64;

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

/// ```
normalizedValue is from 0.0 - 1.0
p2 is always 1
```

```
NativeDB Added Parameter 4: Any p3
```



pub fn special_ability_charge_normalized_safe(
        
        
            player: 
        , 
        
        
            normalizedValue: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0696A65F009EE18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0696A65F009EE18u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                normalizedValue, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn special_ability_charge_normalized_raw(
        player: , 
        normalizedValue: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0696A65F009EE18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0696A65F009EE18u64;

        invoke_raw_typed!(
            hash,
                player, 
                normalizedValue, 
                p2
        )
    }
}

/// ```
Disables something. Used only once in R* scripts (freemode.ysc).
DISABLE_PLAYER_*
```



pub fn _0xb885852c39cc265d_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB885852C39CC265Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB885852C39CC265Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb885852c39cc265d_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB885852C39CC265Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB885852C39CC265Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Tints:  
None = -1,  
Rainbow = 0,  
Red = 1,  
SeasideStripes = 2,  
WidowMaker = 3,  
Patriot = 4,  
Blue = 5,  
Black = 6,  
Hornet = 7,  
AirFocce = 8,  
Desert = 9,  
Shadow = 10,  
HighAltitude = 11,  
Airbone = 12,  
Sunrise = 13,  
```



pub fn get_player_parachute_tint_index_safe(
        
        
            player: 
        , 
        
        
            tintIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75D3F7A1B0D9B145u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75D3F7A1B0D9B145u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                tintIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_player_parachute_tint_index_raw(
        player: , 
        tintIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75D3F7A1B0D9B145u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75D3F7A1B0D9B145u64;

        invoke_raw_typed!(
            hash,
                player, 
                tintIndex
        )
    }
}

/// ```
This has been found in use in the decompiled files.  
```



pub fn _0x4669b3ed80f24b4e_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4669B3ED80F24B4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4669B3ED80F24B4Eu64;
        
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
pub fn _0x4669b3ed80f24b4e_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4669B3ED80F24B4Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4669B3ED80F24B4Eu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
modifier's min value is 0.1
```



pub fn set_player_vehicle_defense_modifier_safe(
        
        
            player: 
        , 
        
        
            modifier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C60E6EFDAFF2462u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C60E6EFDAFF2462u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                modifier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_vehicle_defense_modifier_raw(
        player: , 
        modifier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C60E6EFDAFF2462u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C60E6EFDAFF2462u64;

        invoke_raw_typed!(
            hash,
                player, 
                modifier
        )
    }
}

/// ## Parameters
*



pub fn _0x36f1b38855f2a8df_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36F1B38855F2A8DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36F1B38855F2A8DFu64;
        
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
pub fn _0x36f1b38855f2a8df_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36F1B38855F2A8DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36F1B38855F2A8DFu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Used to toggle the square up aim.
```



pub fn set_player_lockon_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C8B2F450EE4328Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C8B2F450EE4328Eu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_lockon_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C8B2F450EE4328Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C8B2F450EE4328Eu64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// Disables vehicle rewards for the current frame.



pub fn disable_player_vehicle_rewards_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC142BE3BB9CE125Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC142BE3BB9CE125Fu64;
        
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
pub fn disable_player_vehicle_rewards_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC142BE3BB9CE125Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC142BE3BB9CE125Fu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn is_player_free_for_ambient_task_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCCFD3F106C36AB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCCFD3F106C36AB4u64;
        
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
pub fn is_player_free_for_ambient_task_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCCFD3F106C36AB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCCFD3F106C36AB4u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn is_player_pressing_horn_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA1E2BF8B10598F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA1E2BF8B10598F9u64;
        
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
pub fn is_player_pressing_horn_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFA1E2BF8B10598F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFA1E2BF8B10598F9u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn special_ability_deactivate_fast_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CB5CE07A3968D5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CB5CE07A3968D5Au64;
        
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
pub fn special_ability_deactivate_fast_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9CB5CE07A3968D5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9CB5CE07A3968D5Au64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Drft  
```



pub fn get_wanted_level_threshold_safe(
        
        
            wantedLevel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDD179EAF45B556Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDD179EAF45B556Cu64;
        
        let result = invoke_raw!(
            hash,
                wantedLevel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_wanted_level_threshold_raw(
        wantedLevel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDD179EAF45B556Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDD179EAF45B556Cu64;

        invoke_raw_typed!(
            hash,
                wantedLevel
        )
    }
}

/// ## Parameters
*



pub fn _0xd821056b9acf8052_safe(
        
        
            player: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD821056B9ACF8052u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD821056B9ACF8052u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xd821056b9acf8052_raw(
        player: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD821056B9ACF8052u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD821056B9ACF8052u64;

        invoke_raw_typed!(
            hash,
                player, 
                p1
        )
    }
}

/// ```
Sets whether this player can take cover.
```



pub fn set_player_can_use_cover_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD465A8599DFF6814u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD465A8599DFF6814u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_can_use_cover_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD465A8599DFF6814u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD465A8599DFF6814u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ```
Returns profile setting 243.
GET_*
```



pub fn _0xcb645e85e97ea48b_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB645E85E97EA48Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB645E85E97EA48Bu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xcb645e85e97ea48b_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCB645E85E97EA48Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCB645E85E97EA48Bu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Gets a value indicating whether the specified player is currently aiming freely at the specified entity.  
```



pub fn is_player_free_aiming_at_entity_safe(
        
        
            player: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C06B5C839B38F7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C06B5C839B38F7Bu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_player_free_aiming_at_entity_raw(
        player: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C06B5C839B38F7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C06B5C839B38F7Bu64;

        invoke_raw_typed!(
            hash,
                player, 
                entity
        )
    }
}

/// ```
2 matches in 1 script - am_hold_up
Used in multiplayer scripts?
```



pub fn _0x0032a6dba562c518_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0032A6DBA562C518u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0032A6DBA562C518u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x0032a6dba562c518_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0032A6DBA562C518u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0032A6DBA562C518u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_player_simulate_aiming_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC54C95DA968EC5B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC54C95DA968EC5B5u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_simulate_aiming_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC54C95DA968EC5B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC54C95DA968EC5B5u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _set_player_weapon_defense_modifier_2_safe(
        
        
            player: 
        , 
        
        
            modifier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCFDE9EDE4CF27DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCFDE9EDE4CF27DCu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                modifier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_player_weapon_defense_modifier_2_raw(
        player: , 
        modifier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCFDE9EDE4CF27DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCFDE9EDE4CF27DCu64;

        invoke_raw_typed!(
            hash,
                player, 
                modifier
        )
    }
}

/// ```
Returns the same as PLAYER_ID and NETWORK_PLAYER_ID_TO_INT  
```



pub fn get_player_index_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5EDC40EF369B48Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5EDC40EF369B48Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_player_index_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA5EDC40EF369B48Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA5EDC40EF369B48Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_player_can_leave_parachute_smoke_trail_safe(
        
        
            player: 
        , 
        
        
            enabled: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF401B182DBA8AF53u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF401B182DBA8AF53u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                enabled
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_can_leave_parachute_smoke_trail_raw(
        player: , 
        enabled: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF401B182DBA8AF53u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF401B182DBA8AF53u64;

        invoke_raw_typed!(
            hash,
                player, 
                enabled
        )
    }
}

/// ## Parameters
*



pub fn _0x2382ab11450ae7ba_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2382AB11450AE7BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2382AB11450AE7BAu64;
        
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
pub fn _0x2382ab11450ae7ba_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2382AB11450AE7BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2382AB11450AE7BAu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Found in "director_mode", "fm_bj_race_controler", "fm_deathmatch_controler", "fm_impromptu_dm_controler", "fm_race_controler", "gb_deathmatch".  
```



pub fn _0xcac57395b151135f_safe(
        
        
            player: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCAC57395B151135Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCAC57395B151135Fu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xcac57395b151135f_raw(
        player: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCAC57395B151135Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCAC57395B151135Fu64;

        invoke_raw_typed!(
            hash,
                player, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_all_random_peds_flee_this_frame_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x471D2FF42A94B4F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x471D2FF42A94B4F2u64;
        
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
pub fn set_all_random_peds_flee_this_frame_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x471D2FF42A94B4F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x471D2FF42A94B4F2u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn set_player_parachute_smoke_trail_color_safe(
        
        
            player: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8217FD371A4625CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8217FD371A4625CFu64;
        
        let result = invoke_raw!(
            hash,
                player, 
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
pub fn set_player_parachute_smoke_trail_color_raw(
        player: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8217FD371A4625CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8217FD371A4625CFu64;

        invoke_raw_typed!(
            hash,
                player, 
                r, 
                g, 
                b
        )
    }
}

/// ## Parameters
*



pub fn is_player_ready_for_cutscene_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x908CBECC2CAA3690u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x908CBECC2CAA3690u64;
        
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
pub fn is_player_ready_for_cutscene_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x908CBECC2CAA3690u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x908CBECC2CAA3690u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn set_player_forced_zoom_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75E7D505F2B15902u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75E7D505F2B15902u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_forced_zoom_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75E7D505F2B15902u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75E7D505F2B15902u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// Teleports the player to the given coordinates.

If findCollisionLand is true it will try to find the Z value for you, this however has a timeout of 100 frames.

When trying to find the Z value the native will take longer the higher the difference from the given Z to the ground, this combined with the timeout can cause the teleport to just teleport to the given Z value, so try to estimate the z value, so don't just pass in 1000.0.

Also if you're in a vehicle and teleportWithVehicle is true it will not find the Z value for you.



pub fn start_player_teleport_safe(
        
        
            player: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            heading: 
        , 
        
        
            teleportWithVehicle: 
        , 
        
        
            findCollisionLand: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD15F075A4DA0FDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD15F075A4DA0FDEu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                x, 
                y, 
                z, 
                heading, 
                teleportWithVehicle, 
                findCollisionLand, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_player_teleport_raw(
        player: , 
        x: , 
        y: , 
        z: , 
        heading: , 
        teleportWithVehicle: , 
        findCollisionLand: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAD15F075A4DA0FDEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAD15F075A4DA0FDEu64;

        invoke_raw_typed!(
            hash,
                player, 
                x, 
                y, 
                z, 
                heading, 
                teleportWithVehicle, 
                findCollisionLand, 
                p7
        )
    }
}

/// ## Parameters
*



pub fn set_player_forced_aim_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0FEE4F80AC44A726u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0FEE4F80AC44A726u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_forced_aim_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0FEE4F80AC44A726u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0FEE4F80AC44A726u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ```
Simply returns whatever is passed to it (Regardless of whether the handle is valid or not).



pub fn int_to_participantindex_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9EC6603812C24710u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9EC6603812C24710u64;
        
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
pub fn int_to_participantindex_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9EC6603812C24710u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9EC6603812C24710u64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _get_number_of_players_in_team_safe(
        
        
            team: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1FC200409F10E6F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1FC200409F10E6F1u64;
        
        let result = invoke_raw!(
            hash,
                team
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_number_of_players_in_team_raw(
        team: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1FC200409F10E6F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1FC200409F10E6F1u64;

        invoke_raw_typed!(
            hash,
                team
        )
    }
}

/// ```
Checks whether the specified player has a Ped, the Ped is not dead, is not injured and is not arrested.  
```



pub fn is_player_playing_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E9564D8246B909Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E9564D8246B909Au64;
        
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
pub fn is_player_playing_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E9564D8246B909Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E9564D8246B909Au64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
PLAYER::0xBF6993C7(rPtr((&l_122) + 71)); // Found in decompilation



pub fn _0xb45eff719d8427a6_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB45EFF719D8427A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB45EFF719D8427A6u64;
        
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
pub fn _0xb45eff719d8427a6_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB45EFF719D8427A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB45EFF719D8427A6u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Returns profile setting 237.
GET_*
```



pub fn _0xb9cf1f793a9f1bf1_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9CF1F793A9F1BF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9CF1F793A9F1BF1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb9cf1f793a9f1bf1_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9CF1F793A9F1BF1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9CF1F793A9F1BF1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x31e90b8873a4cd3b_safe(
        
        
            player: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31E90B8873A4CD3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31E90B8873A4CD3Bu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x31e90b8873a4cd3b_raw(
        player: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31E90B8873A4CD3Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31E90B8873A4CD3Bu64;

        invoke_raw_typed!(
            hash,
                player, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn reset_player_arrest_state_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D03E13C460760D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D03E13C460760D6u64;
        
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
pub fn reset_player_arrest_state_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D03E13C460760D6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D03E13C460760D6u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn set_auto_give_scuba_gear_when_exit_vehicle_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2B315B6689D537Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2B315B6689D537Du64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_auto_give_scuba_gear_when_exit_vehicle_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD2B315B6689D537Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD2B315B6689D537Du64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// Adds a percentage to a players stamina



pub fn restore_player_stamina_safe(
        
        
            player: 
        , 
        
        
            percentage: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA352C1B864CAFD33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA352C1B864CAFD33u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                percentage
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn restore_player_stamina_raw(
        player: , 
        percentage: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA352C1B864CAFD33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA352C1B864CAFD33u64;

        invoke_raw_typed!(
            hash,
                player, 
                percentage
        )
    }
}

/// ```
Returns true if the player is riding a train.  
```



pub fn is_player_riding_train_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4EC12697209F2196u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4EC12697209F2196u64;
        
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
pub fn is_player_riding_train_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4EC12697209F2196u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4EC12697209F2196u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
# Predominant call signatures  
PLAYER::SET_PLAYER_WANTED_CENTRE_POSITION(PLAYER::PLAYER_ID(), ENTITY::GET_ENTITY_COORDS(PLAYER::PLAYER_PED_ID(), 1));  
# Parameter value ranges  
P0: PLAYER::PLAYER_ID()  
P1: ENTITY::GET_ENTITY_COORDS(PLAYER::PLAYER_PED_ID(), 1)  
P2: Not set by any call  
```



pub fn set_player_wanted_centre_position_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x520E541A97A13354u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x520E541A97A13354u64;
        
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
pub fn set_player_wanted_centre_position_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x520E541A97A13354u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x520E541A97A13354u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn force_cleanup_for_thread_with_this_id_safe(
        
        
            id: 
        , 
        
        
            cleanupFlags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF745B37630DF176Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF745B37630DF176Bu64;
        
        let result = invoke_raw!(
            hash,
                id, 
                cleanupFlags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_cleanup_for_thread_with_this_id_raw(
        id: , 
        cleanupFlags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF745B37630DF176Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF745B37630DF176Bu64;

        invoke_raw_typed!(
            hash,
                id, 
                cleanupFlags
        )
    }
}

/// ```
Gets the player's team.  
Does nothing in singleplayer.  
```



pub fn get_player_team_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37039302F4E0A008u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37039302F4E0A008u64;
        
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
pub fn get_player_team_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x37039302F4E0A008u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x37039302F4E0A008u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn get_player_underwater_time_remaining_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1FCF8E6AF40B731u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1FCF8E6AF40B731u64;
        
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
pub fn get_player_underwater_time_remaining_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1FCF8E6AF40B731u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1FCF8E6AF40B731u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0x70a382adec069dd3_safe(
        
        
            coordX: 
        , 
        
        
            coordY: 
        , 
        
        
            coordZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70A382ADEC069DD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70A382ADEC069DD3u64;
        
        let result = invoke_raw!(
            hash,
                coordX, 
                coordY, 
                coordZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x70a382adec069dd3_raw(
        coordX: , 
        coordY: , 
        coordZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x70A382ADEC069DD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x70A382ADEC069DD3u64;

        invoke_raw_typed!(
            hash,
                coordX, 
                coordY, 
                coordZ
        )
    }
}

/// ```
For Steam.
Always returns 0 in retail version of the game.
```



pub fn _get_achievement_progress_safe(
        
        
            achievement: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C186837D0619335u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C186837D0619335u64;
        
        let result = invoke_raw!(
            hash,
                achievement
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_achievement_progress_raw(
        achievement: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C186837D0619335u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C186837D0619335u64;

        invoke_raw_typed!(
            hash,
                achievement
        )
    }
}

/// ```
NativeDB Introduced: v2060
```



pub fn _set_wanted_level_hidden_evasion_time_safe(
        
        
            player: 
        , 
        
        
            wantedLevel: 
        , 
        
        
            lossTime: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49B856B1360C47C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49B856B1360C47C7u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                wantedLevel, 
                lossTime
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_wanted_level_hidden_evasion_time_raw(
        player: , 
        wantedLevel: , 
        lossTime: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49B856B1360C47C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49B856B1360C47C7u64;

        invoke_raw_typed!(
            hash,
                player, 
                wantedLevel, 
                lossTime
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn special_ability_reset_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x375F0E738F861A94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x375F0E738F861A94u64;
        
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
pub fn special_ability_reset_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x375F0E738F861A94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x375F0E738F861A94u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn get_time_since_player_hit_vehicle_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D35ECF3A81A0EE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D35ECF3A81A0EE0u64;
        
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
pub fn get_time_since_player_hit_vehicle_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D35ECF3A81A0EE0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D35ECF3A81A0EE0u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Return value



pub fn is_player_teleport_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02B15662D7F8886Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02B15662D7F8886Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_player_teleport_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x02B15662D7F8886Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x02B15662D7F8886Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Gets the maximum wanted level the player can get.  
Ranges from 0 to 5.  
```



pub fn get_max_wanted_level_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x462E0DB9B137DC5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x462E0DB9B137DC5Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_max_wanted_level_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x462E0DB9B137DC5Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x462E0DB9B137DC5Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
modifier's min value is 0.1
```



pub fn set_player_vehicle_damage_modifier_safe(
        
        
            player: 
        , 
        
        
            modifier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA50E117CDDF82F0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA50E117CDDF82F0Cu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                modifier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_vehicle_damage_modifier_raw(
        player: , 
        modifier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA50E117CDDF82F0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA50E117CDDF82F0Cu64;

        invoke_raw_typed!(
            hash,
                player, 
                modifier
        )
    }
}

/// ## Parameters
*



pub fn get_player_parachute_pack_tint_index_safe(
        
        
            player: 
        , 
        
        
            tintIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E9C742F340CE5A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E9C742F340CE5A2u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                tintIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_player_parachute_pack_tint_index_raw(
        player: , 
        tintIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E9C742F340CE5A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E9C742F340CE5A2u64;

        invoke_raw_typed!(
            hash,
                player, 
                tintIndex
        )
    }
}

/// ## Parameters
*



pub fn is_player_bluetooth_enable_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65FAEE425DE637B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65FAEE425DE637B0u64;
        
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
pub fn is_player_bluetooth_enable_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65FAEE425DE637B0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65FAEE425DE637B0u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn reset_wanted_level_difficulty_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9D0DD990DC141DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9D0DD990DC141DDu64;
        
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
pub fn reset_wanted_level_difficulty_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB9D0DD990DC141DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB9D0DD990DC141DDu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
IS_*
```



pub fn _0x690a61a6d13583f6_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x690A61A6D13583F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x690A61A6D13583F6u64;
        
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
pub fn _0x690a61a6d13583f6_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x690A61A6D13583F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x690A61A6D13583F6u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn clear_player_has_damaged_at_least_one_non_animal_ped_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AACB96203D11A31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AACB96203D11A31u64;
        
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
pub fn clear_player_has_damaged_at_least_one_non_animal_ped_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AACB96203D11A31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AACB96203D11A31u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn remove_player_helmet_safe(
        
        
            player: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3AC26D3CC576528u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3AC26D3CC576528u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_player_helmet_raw(
        player: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF3AC26D3CC576528u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF3AC26D3CC576528u64;

        invoke_raw_typed!(
            hash,
                player, 
                p2
        )
    }
}

/// ```
If toggle is set to false:
 The police won't be shown on the (mini)map
If toggle is set to true:
 The police will be shown on the (mini)map
```



pub fn set_police_radar_blips_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43286D561B72B8BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43286D561B72B8BFu64;
        
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
pub fn set_police_radar_blips_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43286D561B72B8BFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43286D561B72B8BFu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_player_cloth_pin_frames_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x749FADDF97DFE930u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x749FADDF97DFE930u64;
        
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
pub fn set_player_cloth_pin_frames_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x749FADDF97DFE930u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x749FADDF97DFE930u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Every occurrence of p1 & p2 were both true.
```

```
NativeDB Added Parameter 4: Any p3
```



pub fn special_ability_charge_small_safe(
        
        
            player: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E7B9B683481687Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E7B9B683481687Du64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn special_ability_charge_small_raw(
        player: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E7B9B683481687Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E7B9B683481687Du64;

        invoke_raw_typed!(
            hash,
                player, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn is_player_wanted_level_greater_safe(
        
        
            player: 
        , 
        
        
            wantedLevel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x238DB2A2C23EE9EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x238DB2A2C23EE9EFu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                wantedLevel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_player_wanted_level_greater_raw(
        player: , 
        wantedLevel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x238DB2A2C23EE9EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x238DB2A2C23EE9EFu64;

        invoke_raw_typed!(
            hash,
                player, 
                wantedLevel
        )
    }
}

/// Inhibits the player from using any method of combat including melee and firearms.  
NOTE: Only disables the firing for one frame



pub fn disable_player_firing_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E6CC07646BBEAB8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E6CC07646BBEAB8u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_player_firing_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5E6CC07646BBEAB8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5E6CC07646BBEAB8u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ## Return value



pub fn is_system_ui_being_displayed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D511E3867C87139u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D511E3867C87139u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_system_ui_being_displayed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5D511E3867C87139u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5D511E3867C87139u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// This multiplier is reset to `1.0` every time the player ped is changed, often times via [`SET_PLAYER_MODEL`](#_0x00A1CADD00108836) or [`CHANGE_PLAYER_PED`](#_0x048189FAC643DEEE).



pub fn set_player_health_recharge_multiplier_safe(
        
        
            player: 
        , 
        
        
            regenRate: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DB660B38DD98A31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DB660B38DD98A31u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                regenRate
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_health_recharge_multiplier_raw(
        player: , 
        regenRate: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DB660B38DD98A31u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DB660B38DD98A31u64;

        invoke_raw_typed!(
            hash,
                player, 
                regenRate
        )
    }
}

/// ```
NativeDB Introduced: v2060
```



pub fn _0x823ec8e82ba45986_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x823EC8E82BA45986u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x823EC8E82BA45986u64;
        
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
pub fn _0x823ec8e82ba45986_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x823EC8E82BA45986u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x823EC8E82BA45986u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
used with 1,2,8,64,128 in the scripts  
```



pub fn force_cleanup_safe(
        
        
            cleanupFlags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC8983F38F78ED51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC8983F38F78ED51u64;
        
        let result = invoke_raw!(
            hash,
                cleanupFlags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn force_cleanup_raw(
        cleanupFlags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC8983F38F78ED51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC8983F38F78ED51u64;

        invoke_raw_typed!(
            hash,
                cleanupFlags
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn special_ability_charge_on_mission_failed_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9A763D8FE87436Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9A763D8FE87436Au64;
        
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
pub fn special_ability_charge_on_mission_failed_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC9A763D8FE87436Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC9A763D8FE87436Au64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Assigns the handle of locked-on melee target to *entity that you pass it.  
Returns false if no entity found.  
```



pub fn get_player_target_entity_safe(
        
        
            player: 
        , 
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13EDE1A5DBF797C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13EDE1A5DBF797C9u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                entity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_player_target_entity_raw(
        player: , 
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x13EDE1A5DBF797C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x13EDE1A5DBF797C9u64;

        invoke_raw_typed!(
            hash,
                player, 
                entity
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0x7148e0f43d11f0d9_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7148E0F43D11F0D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7148E0F43D11F0D9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x7148e0f43d11f0d9_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7148E0F43D11F0D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7148E0F43D11F0D9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn is_player_script_control_on_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A876A65283DD7D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A876A65283DD7D7u64;
        
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
pub fn is_player_script_control_on_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A876A65283DD7D7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A876A65283DD7D7u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
tints 0- 13
0 - unkown
1 - unkown
2 - unkown
3 - unkown
4 - unkown
```



pub fn set_player_parachute_pack_tint_index_safe(
        
        
            player: 
        , 
        
        
            tintIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93B0FB27C9A04060u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93B0FB27C9A04060u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                tintIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_parachute_pack_tint_index_raw(
        player: , 
        tintIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x93B0FB27C9A04060u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x93B0FB27C9A04060u64;

        invoke_raw_typed!(
            hash,
                player, 
                tintIndex
        )
    }
}

/// ```
Returns TRUE if the player ('s ped) is climbing at the moment.  
```



pub fn is_player_climbing_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95E8F73DC65EFB9Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95E8F73DC65EFB9Cu64;
        
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
pub fn is_player_climbing_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x95E8F73DC65EFB9Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x95E8F73DC65EFB9Cu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn get_player_sprint_time_remaining_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1885BC9B108B4C99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1885BC9B108B4C99u64;
        
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
pub fn get_player_sprint_time_remaining_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1885BC9B108B4C99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1885BC9B108B4C99u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
p1 appears to always be 1 (only comes up twice)
```

```
NativeDB Added Parameter 3: Any p2
```



pub fn special_ability_charge_continuous_safe(
        
        
            player: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED481732DFF7E997u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED481732DFF7E997u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn special_ability_charge_continuous_raw(
        player: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xED481732DFF7E997u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xED481732DFF7E997u64;

        invoke_raw_typed!(
            hash,
                player, 
                p2
        )
    }
}

/// ```
2 matches. p1 was always true.
```

```
NativeDB Added Parameter 4: Any p3
```



pub fn special_ability_charge_large_safe(
        
        
            player: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF733F45FA4497D93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF733F45FA4497D93u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn special_ability_charge_large_raw(
        player: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF733F45FA4497D93u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF733F45FA4497D93u64;

        invoke_raw_typed!(
            hash,
                player, 
                p1, 
                p2
        )
    }
}

/// ```
1.0.335.2, 1.0.350.1/2, 1.0.372.2, 1.0.393.2, 1.0.393.4, 1.0.463.1;  
```



pub fn player_detach_virtual_bound_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DD5897E2FA6E7C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DD5897E2FA6E7C9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn player_detach_virtual_bound_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1DD5897E2FA6E7C9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1DD5897E2FA6E7C9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_player_sprint_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA01B8075D8B92DF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA01B8075D8B92DF4u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_sprint_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA01B8075D8B92DF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA01B8075D8B92DF4u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ```
Flags:
SPC_AMBIENT_SCRIPT = (1 << 1),
SPC_CLEAR_TASKS = (1 << 2),
SPC_REMOVE_FIRES = (1 << 3),
SPC_REMOVE_EXPLOSIONS = (1 << 4),
SPC_REMOVE_PROJECTILES = (1 << 5),
SPC_DEACTIVATE_GADGETS = (1 << 6),
SPC_REENABLE_CONTROL_ON_DEATH = (1 << 7),
SPC_LEAVE_CAMERA_CONTROL_ON = (1 << 8),
SPC_ALLOW_PLAYER_DAMAGE = (1 << 9),
SPC_DONT_STOP_OTHER_CARS_AROUND_PLAYER = (1 << 10),
SPC_PREVENT_EVERYBODY_BACKOFF = (1 << 11),
SPC_ALLOW_PAD_SHAKE = (1 << 12)
See: https://alloc8or.re/gta5/doc/enums/eSetPlayerControlFlag.txt
```



pub fn set_player_control_safe(
        
        
            player: 
        , 
        
        
            bHasControl: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D32347D6D4C40A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D32347D6D4C40A2u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                bHasControl, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_control_raw(
        player: , 
        bHasControl: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D32347D6D4C40A2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D32347D6D4C40A2u64;

        invoke_raw_typed!(
            hash,
                player, 
                bHasControl, 
                flags
        )
    }
}

/// ```
The player will be ignored by the police if toggle is set to true  
```



pub fn set_police_ignore_player_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32C62AA929C2DA6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32C62AA929C2DA6Au64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_police_ignore_player_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x32C62AA929C2DA6Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x32C62AA929C2DA6Au64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_player_target_level_safe(
        
        
            targetLevel: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5702B917B99DB1CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5702B917B99DB1CDu64;
        
        let result = invoke_raw!(
            hash,
                targetLevel
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_target_level_raw(
        targetLevel: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5702B917B99DB1CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5702B917B99DB1CDu64;

        invoke_raw_typed!(
            hash,
                targetLevel
        )
    }
}

/// ```
Only 1 match. Both p1 & p2 were true.
```

```
NativeDB Added Parameter 4: Any p3
```



pub fn special_ability_charge_medium_safe(
        
        
            player: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF113E3AA9BC54613u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF113E3AA9BC54613u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn special_ability_charge_medium_raw(
        player: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF113E3AA9BC54613u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF113E3AA9BC54613u64;

        invoke_raw_typed!(
            hash,
                player, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn set_player_force_skip_aim_intro_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7651BC64AE59E128u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7651BC64AE59E128u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_force_skip_aim_intro_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7651BC64AE59E128u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7651BC64AE59E128u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn is_special_ability_active_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E5F7FC85D854E15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E5F7FC85D854E15u64;
        
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
pub fn is_special_ability_active_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3E5F7FC85D854E15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3E5F7FC85D854E15u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn has_force_cleanup_occurred_safe(
        
        
            cleanupFlags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC968670BFACE42D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC968670BFACE42D9u64;
        
        let result = invoke_raw!(
            hash,
                cleanupFlags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_force_cleanup_occurred_raw(
        cleanupFlags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC968670BFACE42D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC968670BFACE42D9u64;

        invoke_raw_typed!(
            hash,
                cleanupFlags
        )
    }
}

/// ## Parameters
*



pub fn give_player_ragdoll_control_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C49C870E66F0A28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C49C870E66F0A28u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn give_player_ragdoll_control_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3C49C870E66F0A28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3C49C870E66F0A28u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ```
Can the player control himself, used to disable controls for player for things like a cutscene.



pub fn is_player_control_on_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49C32D60007AFA47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49C32D60007AFA47u64;
        
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
pub fn is_player_control_on_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49C32D60007AFA47u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49C32D60007AFA47u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Swim speed multiplier.  
Multiplier goes up to 1.49  
Just call it one time, it is not required to be called once every tick. - Note copied from below native.  
Note: At least the IDA method if you change the max float multiplier from 1.5 it will change it for both this and RUN_SPRINT below. I say 1.5 as the function blrs if what you input is greater than or equal to 1.5 hence why it's 1.49 max default.  
```



pub fn set_swim_multiplier_for_player_safe(
        
        
            player: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA91C6F0FF7D16A13u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA91C6F0FF7D16A13u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_swim_multiplier_for_player_raw(
        player: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA91C6F0FF7D16A13u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA91C6F0FF7D16A13u64;

        invoke_raw_typed!(
            hash,
                player, 
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn _set_player_health_recharge_limit_safe(
        
        
            player: 
        , 
        
        
            limit: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC388A0F065F5BC34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC388A0F065F5BC34u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                limit
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_player_health_recharge_limit_raw(
        player: , 
        limit: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC388A0F065F5BC34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC388A0F065F5BC34u64;

        invoke_raw_typed!(
            hash,
                player, 
                limit
        )
    }
}

/// Sets whether all random peds will run away from the player if they are agitated (threatened) (bool=true), or if they will stand their ground (bool=false).



pub fn set_all_random_peds_flee_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x056E0FE8534C2949u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x056E0FE8534C2949u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_all_random_peds_flee_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x056E0FE8534C2949u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x056E0FE8534C2949u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ```
- This is called after SET_ALL_RANDOM_PEDS_FLEE_THIS_FRAME
```



pub fn _0xc3376f42b1faccc6_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3376F42B1FACCC6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3376F42B1FACCC6u64;
        
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
pub fn _0xc3376f42b1faccc6_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3376F42B1FACCC6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3376F42B1FACCC6u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Every occurrence was either 0 or 2.  
```



pub fn set_player_cloth_package_index_safe(
        
        
            index: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F7BBA2EA6372500u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F7BBA2EA6372500u64;
        
        let result = invoke_raw!(
            hash,
                index
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_cloth_package_index_raw(
        index: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9F7BBA2EA6372500u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9F7BBA2EA6372500u64;

        invoke_raw_typed!(
            hash,
                index
        )
    }
}

/// ## Parameters
*



pub fn has_player_damaged_at_least_one_non_animal_ped_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4B90F367BD81752u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4B90F367BD81752u64;
        
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
pub fn has_player_damaged_at_least_one_non_animal_ped_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4B90F367BD81752u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4B90F367BD81752u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn set_player_wanted_level_no_drop_safe(
        
        
            player: 
        , 
        
        
            wantedLevel: 
        , 
        
        
            delayedResponse: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x340E61DE7F471565u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x340E61DE7F471565u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                wantedLevel, 
                delayedResponse
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_wanted_level_no_drop_raw(
        player: , 
        wantedLevel: , 
        delayedResponse: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x340E61DE7F471565u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x340E61DE7F471565u64;

        invoke_raw_typed!(
            hash,
                player, 
                wantedLevel, 
                delayedResponse
        )
    }
}

/// ```
example:  
flags: 0-6  
PLAYER::SET_PLAYER_RESET_FLAG_PREFER_REAR_SEATS(PLAYER::PLAYER_ID(), 6);  
wouldnt the flag be the seatIndex?  
```



pub fn set_player_reset_flag_prefer_rear_seats_safe(
        
        
            player: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11D5F725F0E780E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11D5F725F0E780E0u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_reset_flag_prefer_rear_seats_raw(
        player: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11D5F725F0E780E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11D5F725F0E780E0u64;

        invoke_raw_typed!(
            hash,
                player, 
                flags
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x7bae68775557ae0b_safe(
        
        
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
        let hash = 0x7BAE68775557AE0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7BAE68775557AE0Bu64;
        
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
pub fn _0x7bae68775557ae0b_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7BAE68775557AE0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7BAE68775557AE0Bu64;

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

/// ```
Forces any pending wanted level to be applied to the specified player immediately.  
Call SET_PLAYER_WANTED_LEVEL with the desired wanted level, followed by SET_PLAYER_WANTED_LEVEL_NOW.  
Second parameter is unknown (always false).  
```



pub fn set_player_wanted_level_now_safe(
        
        
            player: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0A7D1E497FFCD6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0A7D1E497FFCD6Fu64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_wanted_level_now_raw(
        player: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE0A7D1E497FFCD6Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE0A7D1E497FFCD6Fu64;

        invoke_raw_typed!(
            hash,
                player, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _get_wanted_level_parole_duration_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA72200F51875FEA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA72200F51875FEA4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_wanted_level_parole_duration_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA72200F51875FEA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA72200F51875FEA4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
SET_PLAYER_MAX_*
```



pub fn _0x8d768602adef2245_safe(
        
        
            player: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D768602ADEF2245u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D768602ADEF2245u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8d768602adef2245_raw(
        player: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D768602ADEF2245u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D768602ADEF2245u64;

        invoke_raw_typed!(
            hash,
                player, 
                p1
        )
    }
}

/// ```
p1 was always 5.  
p4 was always false.  
```



pub fn set_player_parachute_variation_override_safe(
        
        
            player: 
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
        let hash = 0xD9284A8C0D48352Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9284A8C0D48352Cu64;
        
        let result = invoke_raw!(
            hash,
                player, 
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
pub fn set_player_parachute_variation_override_raw(
        player: , 
        p1: , 
        p2: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9284A8C0D48352Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9284A8C0D48352Cu64;

        invoke_raw_typed!(
            hash,
                player, 
                p1, 
                p2, 
                p3, 
                p4
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn _0xffee8fa29ab9a18e_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFFEE8FA29AB9A18Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFFEE8FA29AB9A18Eu64;
        
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
pub fn _0xffee8fa29ab9a18e_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFFEE8FA29AB9A18Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFFEE8FA29AB9A18Eu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn clear_player_has_damaged_at_least_one_ped_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0B67A4DE6AB5F98u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0B67A4DE6AB5F98u64;
        
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
pub fn clear_player_has_damaged_at_least_one_ped_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0B67A4DE6AB5F98u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0B67A4DE6AB5F98u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _set_player_homing_rocket_disabled_safe(
        
        
            player: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE4EBDD2593BA844u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE4EBDD2593BA844u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_player_homing_rocket_disabled_raw(
        player: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE4EBDD2593BA844u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE4EBDD2593BA844u64;

        invoke_raw_typed!(
            hash,
                player, 
                p1
        )
    }
}

/// ```
NativeDB Added Parameter 3: Any p2
```



pub fn enable_special_ability_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x181EC197DAEFE121u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x181EC197DAEFE121u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn enable_special_ability_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x181EC197DAEFE121u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x181EC197DAEFE121u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// Suppresses a crime for a given player for this frame only.



pub fn suppress_crime_this_frame_safe(
        
        
            player: 
        , 
        
        
            crimeType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A987297ED8BD838u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A987297ED8BD838u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                crimeType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn suppress_crime_this_frame_raw(
        player: , 
        crimeType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A987297ED8BD838u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A987297ED8BD838u64;

        invoke_raw_typed!(
            hash,
                player, 
                crimeType
        )
    }
}

/// ## Parameters
*



pub fn _get_player_health_recharge_limit_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BC515BAE4AAF8FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BC515BAE4AAF8FFu64;
        
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
pub fn _get_player_health_recharge_limit_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8BC515BAE4AAF8FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8BC515BAE4AAF8FFu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// It returns true if the player is online, suggesting they are also logged in locally. Note that this is an alias for `NETWORK_IS_SIGNED_ONLINE`.



pub fn is_player_online_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF25D331DC2627BBCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF25D331DC2627BBCu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_player_online_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF25D331DC2627BBCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF25D331DC2627BBCu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn special_ability_deactivate_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6A953C6D1492057u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6A953C6D1492057u64;
        
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
pub fn special_ability_deactivate_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6A953C6D1492057u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6A953C6D1492057u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ## Parameters
*



pub fn _0xde45d1a1ef45ee61_safe(
        
        
            player: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE45D1A1EF45EE61u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE45D1A1EF45EE61u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xde45d1a1ef45ee61_raw(
        player: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE45D1A1EF45EE61u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE45D1A1EF45EE61u64;

        invoke_raw_typed!(
            hash,
                player, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_player_fake_wanted_level_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56105E599CAB0EFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56105E599CAB0EFAu64;
        
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
pub fn get_player_fake_wanted_level_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x56105E599CAB0EFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x56105E599CAB0EFAu64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Disables the player's teleportation  
```



pub fn stop_player_teleport_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC449EDED9D73009Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC449EDED9D73009Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_player_teleport_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC449EDED9D73009Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC449EDED9D73009Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Does the same like PLAYER::GET_PLAYER_PED
```



pub fn get_player_ped_script_index_safe(
        
        
            player: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50FAC3A3E030A6E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50FAC3A3E030A6E1u64;
        
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
pub fn get_player_ped_script_index_raw(
        player: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50FAC3A3E030A6E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50FAC3A3E030A6E1u64;

        invoke_raw_typed!(
            hash,
                player
        )
    }
}

/// ```
Tints:  
None = -1,  
Rainbow = 0,  
Red = 1,  
SeasideStripes = 2,  
WidowMaker = 3,  
Patriot = 4,  
Blue = 5,  
Black = 6,  
Hornet = 7,  
AirFocce = 8,  
Desert = 9,  
Shadow = 10,  
HighAltitude = 11,  
Airbone = 12,  
Sunrise = 13,  
```



pub fn set_player_parachute_tint_index_safe(
        
        
            player: 
        , 
        
        
            tintIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3D0E54541D9A5E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3D0E54541D9A5E5u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                tintIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_parachute_tint_index_raw(
        player: , 
        tintIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA3D0E54541D9A5E5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA3D0E54541D9A5E5u64;

        invoke_raw_typed!(
            hash,
                player, 
                tintIndex
        )
    }
}

/// Limit the player to only enter this vehicle. Note set vehicle to false if you want them to access any vehicle.



pub fn set_player_may_only_enter_this_vehicle_safe(
        
        
            player: 
        , 
        
        
            vehicle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8026FF78F208978Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8026FF78F208978Au64;
        
        let result = invoke_raw!(
            hash,
                player, 
                vehicle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_player_may_only_enter_this_vehicle_raw(
        player: , 
        vehicle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8026FF78F208978Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8026FF78F208978Au64;

        invoke_raw_typed!(
            hash,
                player, 
                vehicle
        )
    }
}

