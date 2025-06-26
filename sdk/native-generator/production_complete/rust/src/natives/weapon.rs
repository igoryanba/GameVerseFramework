//! WEAPON native functions
//! 
//! Functions for the weapon category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn remove_weapon_component_from_weapon_object_safe(
        
        
            weaponObject: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7D82B0D66777611u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7D82B0D66777611u64;
        
        let result = invoke_raw!(
            hash,
                weaponObject
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_weapon_component_from_weapon_object_raw(
        weaponObject: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF7D82B0D66777611u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF7D82B0D66777611u64;

        invoke_raw_typed!(
            hash,
                weaponObject
        )
    }
}

/// If `explode` true, then removal is done through exploding the projectile. Basically the same as EXPLODE_PROJECTILES but without defining the owner ped.



pub fn remove_all_projectiles_of_type_safe(
        
        
            weaponHash: 
        , 
        
        
            explode: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC52E0F37E446528u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC52E0F37E446528u64;
        
        let result = invoke_raw!(
            hash,
                weaponHash, 
                explode
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_all_projectiles_of_type_raw(
        weaponHash: , 
        explode: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC52E0F37E446528u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC52E0F37E446528u64;

        invoke_raw_typed!(
            hash,
                weaponHash, 
                explode
        )
    }
}

/// ```
WEAPON::GET_AMMO_IN_PED_WEAPON(PLAYER::PLAYER_PED_ID(), a_0)  
From decompiled scripts  
Returns total ammo in weapon  
GTALua Example :  
natives.WEAPON.GET_AMMO_IN_PED_WEAPON(plyPed, WeaponHash)  
```



pub fn get_ammo_in_ped_weapon_safe(
        
        
            ped: 
        , 
        
        
            weaponhash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x015A522136D7F951u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x015A522136D7F951u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponhash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ammo_in_ped_weapon_raw(
        ped: , 
        weaponhash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x015A522136D7F951u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x015A522136D7F951u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponhash
        )
    }
}

/// Checks if the ped is currently equipped with a weapon matching a bit specified using a bitwise-or in typeFlags.

| Bit value | Effect            |
|-----------|-------------------|
| 1         | Melee weapons     |
| 2         | Explosive weapons |
| 4         | Any other weapons |

Not specifying any bit will lead to the native *always* returning 'false', and for example specifying '4 | 2' will check for any weapon except fists and melee weapons.



pub fn is_ped_armed_safe(
        
        
            ped: 
        , 
        
        
            typeFlags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x475768A975D5AD17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x475768A975D5AD17u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                typeFlags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_armed_raw(
        ped: , 
        typeFlags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x475768A975D5AD17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x475768A975D5AD17u64;

        invoke_raw_typed!(
            hash,
                ped, 
                typeFlags
        )
    }
}

/// List of all available loadouts:
```
LOADOUT_DEFAULT
LOADOUT_ANIMAL
LOADOUT_COUGAR
LOADOUT_HILLBILLY
LOADOUT_CULT
LOADOUT_CHEAT_0
LOADOUT_CHEAT_1
LOADOUT_GUARD
LOADOUT_NETWORK_BOT
LOADOUT_LOST
LOADOUT_LOST_L1
LOADOUT_LOST_L2
LOADOUT_LOST_L3
LOADOUT_MEXICAN
LOADOUT_MEXICAN_L1
LOADOUT_MEXICAN_L2
LOADOUT_MEXICAN_L3
LOADOUT_FAMILY
LOADOUT_ASIAN
LOADOUT_SECUR
LOADOUT_POLICE_GUARD
LOADOUT_COP
LOADOUT_COP_L1
LOADOUT_COP_L2
LOADOUT_COP_L3
LOADOUT_SWAT
LOADOUT_SWAT_NO_LASER
LOADOUT_COP_SHOTGUN
LOADOUT_FIREMAN
LOADOUT_COP_HELI
LOADOUT_COP_BOAT
LOADOUT_ARMY
LOADOUT_ANIMAL_RETRIEVER
LOADOUT_SMALL_DOG
LOADOUT_TIGER_SHARK
LOADOUT_HAMMERHEAD_SHARK
LOADOUT_KILLER_WHALE
LOADOUT_BOAR
LOADOUT_PIG
LOADOUT_COYOTE
LOADOUT_DEER
LOADOUT_HEN
LOADOUT_RABBIT
LOADOUT_CAT
LOADOUT_COW
```



pub fn _give_loadout_to_ped_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68F8BE6AF5CDF8A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68F8BE6AF5CDF8A6u64;
        
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
pub fn _give_loadout_to_ped_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x68F8BE6AF5CDF8A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x68F8BE6AF5CDF8A6u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_flash_light_on_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B7620C47217126Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B7620C47217126Cu64;
        
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
pub fn is_flash_light_on_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4B7620C47217126Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4B7620C47217126Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
NativeDB Introduced: v3407
```



pub fn _get_vehicle_weapon_reload_time_safe(
        
        
            vehicle: 
        , 
        
        
            seat: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0AD348FFD7A6868u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0AD348FFD7A6868u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                seat
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_vehicle_weapon_reload_time_raw(
        vehicle: , 
        seat: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0AD348FFD7A6868u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0AD348FFD7A6868u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                seat
        )
    }
}

/// ## Parameters
*



pub fn _set_ped_weapon_livery_color_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FE5633880ECD8EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FE5633880ECD8EDu64;
        
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
pub fn _set_ped_weapon_livery_color_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9FE5633880ECD8EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9FE5633880ECD8EDu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
SET_WEAPON_OBJECT_*
```



pub fn _0x977ca98939e82e4b_safe(
        
        
            weaponObject: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x977CA98939E82E4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x977CA98939E82E4Bu64;
        
        let result = invoke_raw!(
            hash,
                weaponObject, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x977ca98939e82e4b_raw(
        weaponObject: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x977CA98939E82E4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x977CA98939E82E4Bu64;

        invoke_raw_typed!(
            hash,
                weaponObject, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _remove_air_defense_zone_safe(
        
        
            zoneId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0ABF535877897560u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0ABF535877897560u64;
        
        let result = invoke_raw!(
            hash,
                zoneId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _remove_air_defense_zone_raw(
        zoneId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0ABF535877897560u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0ABF535877897560u64;

        invoke_raw_typed!(
            hash,
                zoneId
        )
    }
}

/// ## Parameters
*



pub fn is_weapon_valid_safe(
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x937C71165CF334B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x937C71165CF334B3u64;
        
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
pub fn is_weapon_valid_raw(
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x937C71165CF334B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x937C71165CF334B3u64;

        invoke_raw_typed!(
            hash,
                weaponHash
        )
    }
}

/// ## Parameters
*



pub fn is_ped_weapon_ready_to_shoot_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB80CA294F2F26749u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB80CA294F2F26749u64;
        
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
pub fn is_ped_weapon_ready_to_shoot_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB80CA294F2F26749u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB80CA294F2F26749u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_ammo_in_clip_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            ammo: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E1202248937775Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E1202248937775Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                ammo
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ammo_in_clip_raw(
        ped: , 
        weaponHash: , 
        ammo: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E1202248937775Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E1202248937775Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                ammo
        )
    }
}

/// ## Parameters
*



pub fn get_current_ped_vehicle_weapon_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1017582BCD3832DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1017582BCD3832DCu64;
        
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
pub fn get_current_ped_vehicle_weapon_raw(
        ped: , 
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1017582BCD3832DCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1017582BCD3832DCu64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash
        )
    }
}

/// ## Parameters
*



pub fn has_weapon_asset_loaded_safe(
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36E353271F0E90EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36E353271F0E90EEu64;
        
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
pub fn has_weapon_asset_loaded_raw(
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36E353271F0E90EEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36E353271F0E90EEu64;

        invoke_raw_typed!(
            hash,
                weaponHash
        )
    }
}

/// Enables/disables flashlight on ped's weapon.

```
NativeDB Introduced: v2060
```



pub fn _set_flash_light_enabled_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x988DB6FE9B3AC000u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x988DB6FE9B3AC000u64;
        
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
pub fn _set_flash_light_enabled_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x988DB6FE9B3AC000u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x988DB6FE9B3AC000u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_lockon_distance_of_current_ped_weapon_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x840F03E9041E2C9Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x840F03E9041E2C9Cu64;
        
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
pub fn get_lockon_distance_of_current_ped_weapon_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x840F03E9041E2C9Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x840F03E9041E2C9Cu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _get_weapon_component_variant_extra_component_count_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6558AC7C17BFEF58u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6558AC7C17BFEF58u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_weapon_component_variant_extra_component_count_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6558AC7C17BFEF58u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6558AC7C17BFEF58u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn set_flash_light_fade_distance_safe(
        
        
            distance: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEA66DAD478CD39Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEA66DAD478CD39Bu64;
        
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
pub fn set_flash_light_fade_distance_raw(
        distance: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCEA66DAD478CD39Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCEA66DAD478CD39Bu64;

        invoke_raw_typed!(
            hash,
                distance
        )
    }
}

/// ```
NativeDB Added Parameter 4: BOOL p3
```



pub fn set_ped_ammo_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            ammo: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14E56BC5B5DB6A19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14E56BC5B5DB6A19u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                ammo
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_ammo_raw(
        ped: , 
        weaponHash: , 
        ammo: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14E56BC5B5DB6A19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14E56BC5B5DB6A19u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                ammo
        )
    }
}

/// ## Parameters
*



pub fn _get_ped_weapon_livery_color_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0A60040BE558F2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0A60040BE558F2Du64;
        
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
pub fn _get_ped_weapon_livery_color_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0A60040BE558F2Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0A60040BE558F2Du64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn _get_weapon_object_livery_color_safe(
        
        
            weaponObject: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3EA4FEABF41464Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3EA4FEABF41464Bu64;
        
        let result = invoke_raw!(
            hash,
                weaponObject
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_weapon_object_livery_color_raw(
        weaponObject: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3EA4FEABF41464Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3EA4FEABF41464Bu64;

        invoke_raw_typed!(
            hash,
                weaponObject
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _set_weapon_explosion_radius_multiplier_safe(
        
        
            weaponHash: 
        , 
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AE5AC8B852D642Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AE5AC8B852D642Cu64;
        
        let result = invoke_raw!(
            hash,
                weaponHash, 
                multiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_weapon_explosion_radius_multiplier_raw(
        weaponHash: , 
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4AE5AC8B852D642Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4AE5AC8B852D642Cu64;

        invoke_raw_typed!(
            hash,
                weaponHash, 
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn give_weapon_component_to_ped_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            componentHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD966D51AA5B28BB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD966D51AA5B28BB9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                componentHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn give_weapon_component_to_ped_raw(
        ped: , 
        weaponHash: , 
        componentHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD966D51AA5B28BB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD966D51AA5B28BB9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                componentHash
        )
    }
}

/// ```
Has 5 parameters since latest patches.  
```



pub fn set_ped_current_weapon_visible_safe(
        
        
            ped: 
        , 
        
        
            visible: 
        , 
        
        
            deselectWeapon: 
        , 
        
        
            p3: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0725A4CCFDED9A70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0725A4CCFDED9A70u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                visible, 
                deselectWeapon, 
                p3, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_current_weapon_visible_raw(
        ped: , 
        visible: , 
        deselectWeapon: , 
        p3: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0725A4CCFDED9A70u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0725A4CCFDED9A70u64;

        invoke_raw_typed!(
            hash,
                ped, 
                visible, 
                deselectWeapon, 
                p3, 
                p4
        )
    }
}

/// ## Parameters
*



pub fn set_ped_ammo_to_drop_safe(
        
        
            ped: 
        , 
        
        
            ammo: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4EFEF9440A5B0EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4EFEF9440A5B0EFu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                ammo
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_ammo_to_drop_raw(
        ped: , 
        ammo: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA4EFEF9440A5B0EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA4EFEF9440A5B0EFu64;

        invoke_raw_typed!(
            hash,
                ped, 
                ammo
        )
    }
}

/// ```
addonHash:
(use WEAPON::GET_WEAPON_COMPONENT_TYPE_MODEL() to get hash value)
${component_at_ar_flsh}, ${component_at_ar_supp}, ${component_at_pi_flsh}, ${component_at_scope_large}, ${component_at_ar_supp_02}
```



pub fn give_weapon_component_to_weapon_object_safe(
        
        
            weaponObject: 
        , 
        
        
            addonHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33E179436C0B31DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33E179436C0B31DBu64;
        
        let result = invoke_raw!(
            hash,
                weaponObject, 
                addonHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn give_weapon_component_to_weapon_object_raw(
        weaponObject: , 
        addonHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x33E179436C0B31DBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x33E179436C0B31DBu64;

        invoke_raw_typed!(
            hash,
                weaponObject, 
                addonHash
        )
    }
}

/// ## Parameters
*



pub fn _0xe4dcec7fd5b739a5_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4DCEC7FD5B739A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4DCEC7FD5B739A5u64;
        
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
pub fn _0xe4dcec7fd5b739a5_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE4DCEC7FD5B739A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE4DCEC7FD5B739A5u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Returns the base/default ammo type of the specified ped's specified weapon.

Use GET_PED_AMMO_TYPE_FROM_WEAPON if you want current ammo type (like AMMO_MG_INCENDIARY/AMMO_MG_TRACER while using MkII magazines) and use this if you want base ammo type. (AMMO_MG)
```



pub fn _get_ped_ammo_type_from_weapon_2_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF489B44DD5AF4BD9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF489B44DD5AF4BD9u64;
        
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
pub fn _get_ped_ammo_type_from_weapon_2_raw(
        ped: , 
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF489B44DD5AF4BD9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF489B44DD5AF4BD9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash
        )
    }
}

/// ## Parameters
*



pub fn set_ped_drops_inventory_weapon_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
        , 
        
        
            ammoCount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x208A1888007FC0E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x208A1888007FC0E6u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                xOffset, 
                yOffset, 
                zOffset, 
                ammoCount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_drops_inventory_weapon_raw(
        ped: , 
        weaponHash: , 
        xOffset: , 
        yOffset: , 
        zOffset: , 
        ammoCount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x208A1888007FC0E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x208A1888007FC0E6u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                xOffset, 
                yOffset, 
                zOffset, 
                ammoCount
        )
    }
}

/// ## Parameters
*



pub fn give_weapon_to_ped_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            ammoCount: 
        , 
        
        
            isHidden: 
        , 
        
        
            bForceInHand: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF0FD6E56C964FCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF0FD6E56C964FCBu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                ammoCount, 
                isHidden, 
                bForceInHand
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn give_weapon_to_ped_raw(
        ped: , 
        weaponHash: , 
        ammoCount: , 
        isHidden: , 
        bForceInHand: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBF0FD6E56C964FCBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBF0FD6E56C964FCBu64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                ammoCount, 
                isHidden, 
                bForceInHand
        )
    }
}

/// ```
It determines what weapons caused damage:  
If you want to define only a specific weapon, second parameter=weapon hash code, third parameter=0  
If you want to define any melee weapon, second parameter=0, third parameter=1.  
If you want to identify any weapon (firearms, melee, rockets, etc.), second parameter=0, third parameter=2.  
```



pub fn has_ped_been_damaged_by_weapon_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            weaponType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D343D2219CD027Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D343D2219CD027Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                weaponType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_ped_been_damaged_by_weapon_raw(
        ped: , 
        weaponHash: , 
        weaponType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D343D2219CD027Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D343D2219CD027Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                weaponType
        )
    }
}

/// ```
Enables laser sight on any weapon.  
It doesn't work. Neither on tick nor OnKeyDown  
```



pub fn enable_laser_sight_rendering_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8B46D7727D864AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8B46D7727D864AAu64;
        
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
pub fn enable_laser_sight_rendering_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8B46D7727D864AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8B46D7727D864AAu64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_max_range_of_current_ped_weapon_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x814C9D19DFD69679u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x814C9D19DFD69679u64;
        
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
pub fn get_max_range_of_current_ped_weapon_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x814C9D19DFD69679u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x814C9D19DFD69679u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
p2 is mostly 1 in the scripts.  
```



pub fn get_max_ammo_in_clip_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA38DCFFCEA8962FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA38DCFFCEA8962FAu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_max_ammo_in_clip_raw(
        ped: , 
        weaponHash: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA38DCFFCEA8962FAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA38DCFFCEA8962FAu64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                p2
        )
    }
}

/// ```
NativeDB Introduced: v3407
```



pub fn _set_ammo_in_vehicle_weapon_clip_safe(
        
        
            vehicle: 
        , 
        
        
            seat: 
        , 
        
        
            ammo: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x873906720EE842C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x873906720EE842C3u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                seat, 
                ammo
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_ammo_in_vehicle_weapon_clip_raw(
        vehicle: , 
        seat: , 
        ammo: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x873906720EE842C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x873906720EE842C3u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                seat, 
                ammo
        )
    }
}

/// _REMOVE_ALL_AIR_DEFENSE_ZONES native function



pub fn _remove_all_air_defense_zones_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E45B34ADEBEE48Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E45B34ADEBEE48Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _remove_all_air_defense_zones_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E45B34ADEBEE48Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E45B34ADEBEE48Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn add_ammo_to_ped_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            ammo: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78F0424C34306220u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78F0424C34306220u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                ammo
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_ammo_to_ped_raw(
        ped: , 
        weaponHash: , 
        ammo: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78F0424C34306220u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78F0424C34306220u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                ammo
        )
    }
}

/// ## Parameters
*



pub fn get_selected_ped_weapon_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A6DB4965674D243u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A6DB4965674D243u64;
        
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
pub fn get_selected_ped_weapon_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A6DB4965674D243u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A6DB4965674D243u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
The return value seems to indicate returns true if the hash of the weapon object weapon equals the weapon hash.  
p2 seems to be 1 most of the time; and is not implemented.
```



pub fn get_current_ped_weapon_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A87E44BB9A01D54u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A87E44BB9A01D54u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_current_ped_weapon_raw(
        ped: , 
        weaponHash: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3A87E44BB9A01D54u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3A87E44BB9A01D54u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                p2
        )
    }
}

/// Disables selecting the given weapon. Ped isn't forced to put the gun away. However you can't reselect the weapon if you holster then unholster. Weapon is also grayed out on the weapon wheel.



pub fn _set_can_ped_equip_weapon_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4771B9AAF4E68E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4771B9AAF4E68E4u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_can_ped_equip_weapon_raw(
        ped: , 
        weaponHash: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4771B9AAF4E68E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4771B9AAF4E68E4u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _get_max_ammo_by_type_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x585847C5E4E11709u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x585847C5E4E11709u64;
        
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
pub fn _get_max_ammo_by_type_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x585847C5E4E11709u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x585847C5E4E11709u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn get_current_ped_weapon_entity_index_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B390A939AF0B5FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B390A939AF0B5FCu64;
        
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
pub fn get_current_ped_weapon_entity_index_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B390A939AF0B5FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B390A939AF0B5FCu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
tintIndex can be the following:  
0 : Default/Black
1 : Green
2 : Gold
3 : Pink
4 : Army
5 : LSPD
6 : Orange
7 : Platinum

tintIndex for MK2 weapons :
0 : Classic Black
1 : Classic Gray
2 : Classic Two-Tone
3 : Classic White
4 : Classic Beige
5 : Classic Green
6 : Classic Blue
7 : Classic Earth
8 : Classic Brown & Black
9 : Red Contrast
10 : Blue Contrast
11 : Yellow Contrast
12 : Orange Contrast
13 : Bold Pink
14 : Bold Purple & Yellow
15 : Bold Orange
16 : Bold Green & Purple
17 : Bold Red Features
18 : Bold Green Features
19 : Bold Cyan Features
20 : Bold Yellow Features
21 : Bold Red & White
22 : Bold Blue & White
23 : Metallic Gold
24 : Metallic Platinum
25 : Metallic Gray & Lilac
26 : Metallic Purple & Lime
27 : Metallic Red
28 : Metallic Green
29 : Metallic Blue
30 : Metallic White & Aqua
31 : Metallic Orange & Yellow
32 : Mettalic Red and Yellow
```



pub fn set_ped_weapon_tint_index_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            tintIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50969B9B89ED5738u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50969B9B89ED5738u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                tintIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_weapon_tint_index_raw(
        ped: , 
        weaponHash: , 
        tintIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50969B9B89ED5738u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50969B9B89ED5738u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                tintIndex
        )
    }
}

/// ## Parameters
*



pub fn set_ped_chance_of_firing_blanks_safe(
        
        
            ped: 
        , 
        
        
            xBias: 
        , 
        
        
            yBias: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8378627201D5497Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8378627201D5497Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                xBias, 
                yBias
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_chance_of_firing_blanks_raw(
        ped: , 
        xBias: , 
        yBias: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8378627201D5497Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8378627201D5497Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                xBias, 
                yBias
        )
    }
}

/// Does the same as [`_SET_CAN_PED_SELECT_WEAPON`](#_0xB4771B9AAF4E68E4) except for all weapons.



pub fn _set_can_ped_equip_all_weapons_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFF296097FF1E509u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFF296097FF1E509u64;
        
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
pub fn _set_can_ped_equip_all_weapons_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFF296097FF1E509u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFF296097FF1E509u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn get_ped_ammo_by_type_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39D22031557946C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39D22031557946C1u64;
        
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
pub fn get_ped_ammo_by_type_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39D22031557946C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39D22031557946C1u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn is_ped_weapon_component_active_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            componentHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D78DE0572D3969Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D78DE0572D3969Eu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                componentHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_ped_weapon_component_active_raw(
        ped: , 
        weaponHash: , 
        componentHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0D78DE0572D3969Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0D78DE0572D3969Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                componentHash
        )
    }
}

/// ## Parameters
*



pub fn has_weapon_got_weapon_component_safe(
        
        
            weapon: 
        , 
        
        
            addonHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76A18844E743BF91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76A18844E743BF91u64;
        
        let result = invoke_raw!(
            hash,
                weapon, 
                addonHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_weapon_got_weapon_component_raw(
        weapon: , 
        addonHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x76A18844E743BF91u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x76A18844E743BF91u64;

        invoke_raw_typed!(
            hash,
                weapon, 
                addonHash
        )
    }
}

/// This native does not return damages of weapons from the melee and explosive group.



pub fn get_weapon_damage_safe(
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3133B907D8B32053u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3133B907D8B32053u64;
        
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
pub fn get_weapon_damage_raw(
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3133B907D8B32053u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3133B907D8B32053u64;

        invoke_raw_typed!(
            hash,
                weaponHash
        )
    }
}

/// ## Parameters
*



pub fn set_ped_drops_weapon_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B7513D9966FBEC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B7513D9966FBEC0u64;
        
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
pub fn set_ped_drops_weapon_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B7513D9966FBEC0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B7513D9966FBEC0u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_current_ped_weapon_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            bForceInHand: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADF692B254977C0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADF692B254977C0Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                bForceInHand
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_current_ped_weapon_raw(
        ped: , 
        weaponHash: , 
        bForceInHand: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADF692B254977C0Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADF692B254977C0Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                bForceInHand
        )
    }
}

/// ```
This native removes a specified weapon from your selected ped.  
Weapon Hashes: pastebin.com/0wwDZgkF  
Example:  
C#:  
Function.Call(Hash.REMOVE_WEAPON_FROM_PED, Game.Player.Character, 0x99B507EA);  
C++:  
WEAPON::REMOVE_WEAPON_FROM_PED(PLAYER::PLAYER_PED_ID(), 0x99B507EA);  
The code above removes the knife from the player.  
```



pub fn remove_weapon_from_ped_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4899CB088EDF59B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4899CB088EDF59B8u64;
        
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
pub fn remove_weapon_from_ped_raw(
        ped: , 
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4899CB088EDF59B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4899CB088EDF59B8u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash
        )
    }
}

/// Start a reload for a vehicle's weapon.

```
NativeDB Introduced: v3407
```



pub fn _trigger_vehicle_weapon_reload_safe(
        
        
            vehicle: 
        , 
        
        
            seat: 
        , 
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B1513F27F279A44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B1513F27F279A44u64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                seat, 
                ped
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _trigger_vehicle_weapon_reload_raw(
        vehicle: , 
        seat: , 
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5B1513F27F279A44u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5B1513F27F279A44u64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                seat, 
                ped
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x24c024ba8379a70a_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24C024BA8379A70Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24C024BA8379A70Au64;
        
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
pub fn _0x24c024ba8379a70a_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x24C024BA8379A70Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x24C024BA8379A70Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn clear_entity_last_weapon_damage_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC678E40BE7C74D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC678E40BE7C74D2u64;
        
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
pub fn clear_entity_last_weapon_damage_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAC678E40BE7C74D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAC678E40BE7C74D2u64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn _get_weapon_component_variant_extra_component_model_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D1CB8DC40208A17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D1CB8DC40208A17u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_weapon_component_variant_extra_component_model_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D1CB8DC40208A17u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D1CB8DC40208A17u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Hides the players weapon during a cutscene.  
```



pub fn hide_ped_weapon_for_scripted_cutscene_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F6981D2253C208Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F6981D2253C208Fu64;
        
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
pub fn hide_ped_weapon_for_scripted_cutscene_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F6981D2253C208Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F6981D2253C208Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn refill_ammo_instantly_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C0D57EA686FAD87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C0D57EA686FAD87u64;
        
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
pub fn refill_ammo_instantly_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C0D57EA686FAD87u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C0D57EA686FAD87u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Returns handle of the projectile.
```



pub fn set_ped_shoot_ordnance_weapon_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4C8D77C80C0421Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4C8D77C80C0421Eu64;
        
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
pub fn set_ped_shoot_ordnance_weapon_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB4C8D77C80C0421Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB4C8D77C80C0421Eu64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn get_weapontype_slot_safe(
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4215460B9B8B7FA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4215460B9B8B7FA0u64;
        
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
pub fn get_weapontype_slot_raw(
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4215460B9B8B7FA0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4215460B9B8B7FA0u64;

        invoke_raw_typed!(
            hash,
                weaponHash
        )
    }
}

/// Does NOT seem to work with HAS_PED_BEEN_DAMAGED_BY_WEAPON. Use CLEAR_ENTITY_LAST_WEAPON_DAMAGE and HAS_ENTITY_BEEN_DAMAGED_BY_WEAPON instead.



pub fn clear_ped_last_weapon_damage_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E98F88A24C5F4B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E98F88A24C5F4B8u64;
        
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
pub fn clear_ped_last_weapon_damage_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0E98F88A24C5F4B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0E98F88A24C5F4B8u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
p1/gadgetHash was always 0xFBAB5776 ("GADGET_PARACHUTE").  
p2 is always true.  
```



pub fn set_ped_gadget_safe(
        
        
            ped: 
        , 
        
        
            gadgetHash: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0D7B1E680ED4A1Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0D7B1E680ED4A1Au64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                gadgetHash, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_gadget_raw(
        ped: , 
        gadgetHash: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD0D7B1E680ED4A1Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD0D7B1E680ED4A1Au64;

        invoke_raw_typed!(
            hash,
                ped, 
                gadgetHash, 
                p2
        )
    }
}

/// ```
Returns the model of any weapon.  
Can also take an ammo hash?  
sub_6663a(&l_115B, WEAPON::GET_WEAPONTYPE_MODEL(${ammo_rpg}));  
```



pub fn get_weapontype_model_safe(
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF46CDC33180FDA94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF46CDC33180FDA94u64;
        
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
pub fn get_weapontype_model_raw(
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF46CDC33180FDA94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF46CDC33180FDA94u64;

        invoke_raw_typed!(
            hash,
                weaponHash
        )
    }
}

/// ## Parameters
*



pub fn _does_air_defense_zone_exist_safe(
        
        
            zoneId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD79A550999D7D4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD79A550999D7D4Fu64;
        
        let result = invoke_raw!(
            hash,
                zoneId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _does_air_defense_zone_exist_raw(
        zoneId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD79A550999D7D4Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD79A550999D7D4Fu64;

        invoke_raw_typed!(
            hash,
                zoneId
        )
    }
}

/// Both coordinates are from objects in the decompiled scripts.

Native related to [_0xECDC202B25E5CF48](#_0xECDC202B25E5CF48) p1 value. The only weapon hash used in the decompiled scripts is weapon_air_defence_gun. These two natives are used by the yacht script, decompiled scripts suggest it and the weapon hash used (valkyrie's rockets) are also used by yachts.



pub fn create_air_defence_sphere_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x91EF34584710BE99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x91EF34584710BE99u64;
        
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
pub fn create_air_defence_sphere_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x91EF34584710BE99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x91EF34584710BE99u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// ```
0=unknown (or incorrect weaponHash)  
1= no damage (flare,snowball, petrolcan)  
2=melee  
3=bullet  
4=force ragdoll fall  
5=explosive (RPG, Railgun, grenade)  
6=fire(molotov)  
8=fall(WEAPON_HELI_CRASH)  
10=electric  
11=barbed wire  
12=extinguisher  
13=gas  
14=water cannon(WEAPON_HIT_BY_WATER_CANNON)  
```



pub fn get_weapon_damage_type_safe(
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BE0BB12D25FB305u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BE0BB12D25FB305u64;
        
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
pub fn get_weapon_damage_type_raw(
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BE0BB12D25FB305u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BE0BB12D25FB305u64;

        invoke_raw_typed!(
            hash,
                weaponHash
        )
    }
}

/// ```
It determines what weapons caused damage:
If you want to define only a specific weapon, second parameter=weapon hash code, third parameter=0
If you want to define any melee weapon, second parameter=0, third parameter=1.
If you want to identify any weapon (firearms, melee, rockets, etc.), second parameter=0, third parameter=2.
```



pub fn has_entity_been_damaged_by_weapon_safe(
        
        
            entity: 
        , 
        
        
            weaponHash: 
        , 
        
        
            weaponType: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x131D401334815E94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x131D401334815E94u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                weaponHash, 
                weaponType
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_entity_been_damaged_by_weapon_raw(
        entity: , 
        weaponHash: , 
        weaponType: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x131D401334815E94u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x131D401334815E94u64;

        invoke_raw_typed!(
            hash,
                entity, 
                weaponHash, 
                weaponType
        )
    }
}

/// ```
Pass ped. Pass address of Vector3.  
The coord will be put into the Vector3.  
The return will determine whether there was a coord found or not.  
```



pub fn get_ped_last_weapon_impact_coord_safe(
        
        
            ped: 
        , 
        
        
            coords: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C4D0409BA1A2BC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C4D0409BA1A2BC2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                coords
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_last_weapon_impact_coord_raw(
        ped: , 
        coords: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6C4D0409BA1A2BC2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6C4D0409BA1A2BC2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                coords
        )
    }
}

/// ## Parameters
*



pub fn create_air_defence_angled_area_safe(
        
        
            srcCoord1X: 
        , 
        
        
            srcCoord1Y: 
        , 
        
        
            srcCoord1Z: 
        , 
        
        
            srcCoord2X: 
        , 
        
        
            srcCoord2Y: 
        , 
        
        
            srcCoord2Z: 
        , 
        
        
            fWidth: 
        , 
        
        
            weaponPositionX: 
        , 
        
        
            weaponPositionY: 
        , 
        
        
            weaponPositionZ: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DA58CDBF6BDBC08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DA58CDBF6BDBC08u64;
        
        let result = invoke_raw!(
            hash,
                srcCoord1X, 
                srcCoord1Y, 
                srcCoord1Z, 
                srcCoord2X, 
                srcCoord2Y, 
                srcCoord2Z, 
                fWidth, 
                weaponPositionX, 
                weaponPositionY, 
                weaponPositionZ
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_air_defence_angled_area_raw(
        srcCoord1X: , 
        srcCoord1Y: , 
        srcCoord1Z: , 
        srcCoord2X: , 
        srcCoord2Y: , 
        srcCoord2Z: , 
        fWidth: , 
        weaponPositionX: , 
        weaponPositionY: , 
        weaponPositionZ: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9DA58CDBF6BDBC08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9DA58CDBF6BDBC08u64;

        invoke_raw_typed!(
            hash,
                srcCoord1X, 
                srcCoord1Y, 
                srcCoord1Z, 
                srcCoord2X, 
                srcCoord2Y, 
                srcCoord2Z, 
                fWidth, 
                weaponPositionX, 
                weaponPositionY, 
                weaponPositionZ
        )
    }
}

/// ```
gadgetHash - was always 0xFBAB5776 ("GADGET_PARACHUTE").  
```



pub fn get_is_ped_gadget_equipped_safe(
        
        
            ped: 
        , 
        
        
            gadgetHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF731332072F5156Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF731332072F5156Cu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                gadgetHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_is_ped_gadget_equipped_raw(
        ped: , 
        gadgetHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF731332072F5156Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF731332072F5156Cu64;

        invoke_raw_typed!(
            hash,
                ped, 
                gadgetHash
        )
    }
}

/// ```
NativeDB Introduced: v3407
```



pub fn _has_weapon_reloading_in_vehicle_safe(
        
        
            vehicle: 
        , 
        
        
            seat: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8062F07153F4446Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8062F07153F4446Fu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                seat
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _has_weapon_reloading_in_vehicle_raw(
        vehicle: , 
        seat: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8062F07153F4446Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8062F07153F4446Fu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                seat
        )
    }
}

/// ```
NativeDB Introduced: v3407
```



pub fn _get_time_before_vehicle_weapon_reload_finishes_safe(
        
        
            vehicle: 
        , 
        
        
            seat: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8C6F4B1CDEB40EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8C6F4B1CDEB40EFu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                seat
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_time_before_vehicle_weapon_reload_finishes_raw(
        vehicle: , 
        seat: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC8C6F4B1CDEB40EFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC8C6F4B1CDEB40EFu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                seat
        )
    }
}

/// ```
Returns the current ammo type of the specified ped's specified weapon.

MkII magazines will change the return value, like Pistol MkII returning AMMO_PISTOL without any components and returning AMMO_PISTOL_TRACER after Tracer Rounds component is attached.

Use 0xF489B44DD5AF4BD9 if you always want AMMO_PISTOL.
```



pub fn get_ped_ammo_type_from_weapon_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FEAD38B326B9F74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FEAD38B326B9F74u64;
        
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
pub fn get_ped_ammo_type_from_weapon_raw(
        ped: , 
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FEAD38B326B9F74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FEAD38B326B9F74u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash
        )
    }
}

/// ## Parameters
*



pub fn _0xa2c9ac24b4061285_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2C9AC24B4061285u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2C9AC24B4061285u64;
        
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
pub fn _0xa2c9ac24b4061285_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2C9AC24B4061285u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2C9AC24B4061285u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Restricts weapon selection when cycling through weapons, to select only vehicle weapons.



pub fn set_ped_cycle_vehicle_weapons_only_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50276EF8172F5F12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50276EF8172F5F12u64;
        
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
pub fn set_ped_cycle_vehicle_weapons_only_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50276EF8172F5F12u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50276EF8172F5F12u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// Create a weapon object that cannot be attached to a ped. If you want to create a weapon object that can be attached to a ped, use [`CREATE_OBJECT`](#_0x509D5878EB39E842) instead.

```
NativeDB Added Parameter 9: BOOL bRegisterAsNetworkObject
NativeDB Added Parameter 10: BOOL bScriptHostObject
```



pub fn create_weapon_object_safe(
        
        
            weaponHash: 
        , 
        
        
            ammoCount: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            bCreateDefaultComponents: 
        , 
        
        
            scale: 
        , 
        
        
            customModelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9541D3CF0D398F36u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9541D3CF0D398F36u64;
        
        let result = invoke_raw!(
            hash,
                weaponHash, 
                ammoCount, 
                x, 
                y, 
                z, 
                bCreateDefaultComponents, 
                scale, 
                customModelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_weapon_object_raw(
        weaponHash: , 
        ammoCount: , 
        x: , 
        y: , 
        z: , 
        bCreateDefaultComponents: , 
        scale: , 
        customModelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9541D3CF0D398F36u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9541D3CF0D398F36u64;

        invoke_raw_typed!(
            hash,
                weaponHash, 
                ammoCount, 
                x, 
                y, 
                z, 
                bCreateDefaultComponents, 
                scale, 
                customModelHash
        )
    }
}

/// ## Parameters
*



pub fn set_ped_infinite_ammo_clip_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x183DADC6AA953186u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x183DADC6AA953186u64;
        
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
pub fn set_ped_infinite_ammo_clip_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x183DADC6AA953186u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x183DADC6AA953186u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn set_ped_drops_weapons_when_dead_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x476AE72C1D19D1A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x476AE72C1D19D1A8u64;
        
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
pub fn set_ped_drops_weapons_when_dead_raw(
        ped: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x476AE72C1D19D1A8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x476AE72C1D19D1A8u64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle
        )
    }
}

/// Changes the selected ped aiming animation style, you can find the list of animations below.

These are stored in the `weaponanimations.meta` file located in `Grand Theft Auto V\update\update.rpf\common\data\ai\weaponanimations.meta`.

For Lua, it's best if you send the animation using [compile-time jenkins](https://cookbook.fivem.net/2019/06/23/lua-support-for-compile-time-jenkins-hashes/) hashes to avoid overhead. An example is shown down below.



pub fn set_weapon_animation_override_safe(
        
        
            ped: 
        , 
        
        
            animStyle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1055AC3A667F09D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1055AC3A667F09D9u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                animStyle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_weapon_animation_override_raw(
        ped: , 
        animStyle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1055AC3A667F09D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1055AC3A667F09D9u64;

        invoke_raw_typed!(
            hash,
                ped, 
                animStyle
        )
    }
}

/// ## Parameters
*



pub fn does_weapon_take_weapon_component_safe(
        
        
            weaponHash: 
        , 
        
        
            componentHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CEE3DF569CECAB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CEE3DF569CECAB0u64;
        
        let result = invoke_raw!(
            hash,
                weaponHash, 
                componentHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_weapon_take_weapon_component_raw(
        weaponHash: , 
        componentHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5CEE3DF569CECAB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5CEE3DF569CECAB0u64;

        invoke_raw_typed!(
            hash,
                weaponHash, 
                componentHash
        )
    }
}

/// ```
Gives a weapon to PED with a delay, example:
WEAPON::GIVE_DELAYED_WEAPON_TO_PED(PED::PLAYER_PED_ID(), MISC::GET_HASH_KEY("WEAPON_PISTOL"), 1000, false)
```



pub fn give_delayed_weapon_to_ped_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            ammoCount: 
        , 
        
        
            bForceInHand: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB282DC6EBD803C75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB282DC6EBD803C75u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                ammoCount, 
                bForceInHand
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn give_delayed_weapon_to_ped_raw(
        ped: , 
        weaponHash: , 
        ammoCount: , 
        bForceInHand: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB282DC6EBD803C75u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB282DC6EBD803C75u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                ammoCount, 
                bForceInHand
        )
    }
}

/// ## Parameters
*



pub fn remove_weapon_asset_safe(
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA08EF13F341C8FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA08EF13F341C8FCu64;
        
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
pub fn remove_weapon_asset_raw(
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA08EF13F341C8FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA08EF13F341C8FCu64;

        invoke_raw_typed!(
            hash,
                weaponHash
        )
    }
}

/// ```
NativeDB Introduced: v3407
```



pub fn _get_ammo_in_vehicle_weapon_clip_safe(
        
        
            vehicle: 
        , 
        
        
            seat: 
        , 
        
        
            ammo: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2857938C5D407AFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2857938C5D407AFAu64;
        
        let result = invoke_raw!(
            hash,
                vehicle, 
                seat, 
                ammo
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_ammo_in_vehicle_weapon_clip_raw(
        vehicle: , 
        seat: , 
        ammo: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2857938C5D407AFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2857938C5D407AFAu64;

        invoke_raw_typed!(
            hash,
                vehicle, 
                seat, 
                ammo
        )
    }
}

/// ## Parameters
*



pub fn has_ped_got_weapon_component_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            componentHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC593212475FAE340u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC593212475FAE340u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                componentHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_ped_got_weapon_component_raw(
        ped: , 
        weaponHash: , 
        componentHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC593212475FAE340u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC593212475FAE340u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                componentHash
        )
    }
}

/// ```
p2 should be FALSE, otherwise it seems to always return FALSE  
Bool does not check if the weapon is current equipped, unfortunately.  
```



pub fn has_ped_got_weapon_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DECB02F88F428BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DECB02F88F428BCu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_ped_got_weapon_raw(
        ped: , 
        weaponHash: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DECB02F88F428BCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DECB02F88F428BCu64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                p2
        )
    }
}

/// Gets and returns the hash of the group of the specified weapon (group names can be found/changed under "Group" in the weapons' meta file).
Note that the group is



pub fn get_weapontype_group_safe(
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3287EE3050FB74Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3287EE3050FB74Cu64;
        
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
pub fn get_weapontype_group_raw(
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC3287EE3050FB74Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC3287EE3050FB74Cu64;

        invoke_raw_typed!(
            hash,
                weaponHash
        )
    }
}

/// ## Parameters
*



pub fn _is_any_air_defense_zone_inside_sphere_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            SphereIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDAB963831DBFD3F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDAB963831DBFD3F4u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                SphereIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _is_any_air_defense_zone_inside_sphere_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        SphereIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDAB963831DBFD3F4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDAB963831DBFD3F4u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                SphereIndex
        )
    }
}

/// Parameter `p1` does not seem to be used or referenced in game binaries.



pub fn remove_all_ped_weapons_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF25DF915FA38C5F3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF25DF915FA38C5F3u64;
        
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
pub fn remove_all_ped_weapons_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF25DF915FA38C5F3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF25DF915FA38C5F3u64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// Forces a ped to reload only if they are able to; if they have a full magazine, they will not reload.



pub fn make_ped_reload_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20AE33F3AC9C0033u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20AE33F3AC9C0033u64;
        
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
pub fn make_ped_reload_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20AE33F3AC9C0033u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20AE33F3AC9C0033u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn remove_weapon_component_from_ped_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            componentHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E8BE90C74FB4C09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E8BE90C74FB4C09u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                componentHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_weapon_component_from_ped_raw(
        ped: , 
        weaponHash: , 
        componentHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E8BE90C74FB4C09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E8BE90C74FB4C09u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                componentHash
        )
    }
}

/// ```
// Returns the size of the default weapon component clip.  
Use it like this:  
char cClipSize[32];  
Hash cur;  
if (WEAPON::GET_CURRENT_PED_WEAPON(playerPed, &cur, 1))  
{  
    if (WEAPON::IS_WEAPON_VALID(cur))  
    {  
        int iClipSize = WEAPON::GET_WEAPON_CLIP_SIZE(cur);  
        sprintf_s(cClipSize, "ClipSize: %.d", iClipSize);  
        vDrawString(cClipSize, 0.5f, 0.5f);  
    }  
}  
```



pub fn get_weapon_clip_size_safe(
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x583BE370B1EC6EB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x583BE370B1EC6EB4u64;
        
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
pub fn get_weapon_clip_size_raw(
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x583BE370B1EC6EB4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x583BE370B1EC6EB4u64;

        invoke_raw_typed!(
            hash,
                weaponHash
        )
    }
}

/// ```
this returns if you can use the weapon while using a parachute  
```



pub fn can_use_weapon_on_parachute_safe(
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC7BE5ABC0879F74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC7BE5ABC0879F74u64;
        
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
pub fn can_use_weapon_on_parachute_raw(
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBC7BE5ABC0879F74u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBC7BE5ABC0879F74u64;

        invoke_raw_typed!(
            hash,
                weaponHash
        )
    }
}

/// ```
Third Parameter = unsure, but pretty sure it is weapon hash  
--> get_hash_key("weapon_stickybomb")  
Fourth Parameter = unsure, almost always -1  
```



pub fn has_vehicle_got_projectile_attached_safe(
        
        
            driver: 
        , 
        
        
            vehicle: 
        , 
        
        
            weaponHash: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x717C8481234E3B88u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x717C8481234E3B88u64;
        
        let result = invoke_raw!(
            hash,
                driver, 
                vehicle, 
                weaponHash, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_vehicle_got_projectile_attached_raw(
        driver: , 
        vehicle: , 
        weaponHash: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x717C8481234E3B88u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x717C8481234E3B88u64;

        invoke_raw_typed!(
            hash,
                driver, 
                vehicle, 
                weaponHash, 
                p3
        )
    }
}

/// ```
Nearly every instance of p1 I found was 31. Nearly every instance of p2 I found was 0.  
REQUEST_WEAPON_ASSET(iLocal_1888, 31, 26);  
```



pub fn request_weapon_asset_safe(
        
        
            weaponHash: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5443438F033E29C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5443438F033E29C3u64;
        
        let result = invoke_raw!(
            hash,
                weaponHash, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_weapon_asset_raw(
        weaponHash: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5443438F033E29C3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5443438F033E29C3u64;

        invoke_raw_typed!(
            hash,
                weaponHash, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn _add_ammo_to_ped_by_type_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2472622CE1F2D45Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2472622CE1F2D45Fu64;
        
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
pub fn _add_ammo_to_ped_by_type_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2472622CE1F2D45Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2472622CE1F2D45Fu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
WEAPON::EXPLODE_PROJECTILES(PLAYER::PLAYER_PED_ID(), func_221(0x00000003), 0x00000001);  
```



pub fn explode_projectiles_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC4BD125DE7611E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC4BD125DE7611E4u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn explode_projectiles_raw(
        ped: , 
        weaponHash: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC4BD125DE7611E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC4BD125DE7611E4u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn set_ped_ammo_by_type_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FD1E1F011E76D7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FD1E1F011E76D7Eu64;
        
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
pub fn set_ped_ammo_by_type_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5FD1E1F011E76D7Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5FD1E1F011E76D7Eu64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ```
Drops the current weapon and returns the object  
Unknown behavior when unarmed.  
```



pub fn get_weapon_object_from_ped_safe(
        
        
            ped: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCAE1DC9A0E22A16Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCAE1DC9A0E22A16Du64;
        
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
pub fn get_weapon_object_from_ped_raw(
        ped: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCAE1DC9A0E22A16Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCAE1DC9A0E22A16Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _0xe6d2cedd370ff98e_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6D2CEDD370FF98Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6D2CEDD370FF98Eu64;
        
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
pub fn _0xe6d2cedd370ff98e_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE6D2CEDD370FF98Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE6D2CEDD370FF98Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _set_weapon_object_livery_color_safe(
        
        
            weaponObject: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DA825A85D0EA6E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DA825A85D0EA6E6u64;
        
        let result = invoke_raw!(
            hash,
                weaponObject
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_weapon_object_livery_color_raw(
        weaponObject: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DA825A85D0EA6E6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DA825A85D0EA6E6u64;

        invoke_raw_typed!(
            hash,
                weaponObject
        )
    }
}

/// ## Parameters
*



pub fn get_weapon_object_tint_index_safe(
        
        
            weapon: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD183314F7CD2E57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD183314F7CD2E57u64;
        
        let result = invoke_raw!(
            hash,
                weapon
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_weapon_object_tint_index_raw(
        weapon: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCD183314F7CD2E57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCD183314F7CD2E57u64;

        invoke_raw_typed!(
            hash,
                weapon
        )
    }
}

/// ## Parameters
*



pub fn _fire_air_defense_weapon_safe(
        
        
            zoneId: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44F1012B69313374u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44F1012B69313374u64;
        
        let result = invoke_raw!(
            hash,
                zoneId, 
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
pub fn _fire_air_defense_weapon_raw(
        zoneId: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x44F1012B69313374u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x44F1012B69313374u64;

        invoke_raw_typed!(
            hash,
                zoneId, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn get_max_ammo_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            ammo: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC16122C7A20C933u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC16122C7A20C933u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                ammo
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_max_ammo_raw(
        ped: , 
        weaponHash: , 
        ammo: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDC16122C7A20C933u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDC16122C7A20C933u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                ammo
        )
    }
}

/// ## Parameters
*



pub fn get_best_ped_weapon_safe(
        
        
            ped: 
        , 
        
        
            ignoreAmmoCount: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8483E98E8B888AE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8483E98E8B888AE2u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                ignoreAmmoCount
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_best_ped_weapon_raw(
        ped: , 
        ignoreAmmoCount: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8483E98E8B888AE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8483E98E8B888AE2u64;

        invoke_raw_typed!(
            hash,
                ped, 
                ignoreAmmoCount
        )
    }
}

/// ## Parameters
*



pub fn _set_player_air_defense_zone_flag_safe(
        
        
            player: 
        , 
        
        
            zoneId: 
        , 
        
        
            enable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xECDC202B25E5CF48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xECDC202B25E5CF48u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                zoneId, 
                enable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_player_air_defense_zone_flag_raw(
        player: , 
        zoneId: , 
        enable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xECDC202B25E5CF48u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xECDC202B25E5CF48u64;

        invoke_raw_typed!(
            hash,
                player, 
                zoneId, 
                enable
        )
    }
}

/// ## Parameters
*



pub fn set_ammo_in_clip_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        , 
        
        
            ammo: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCD2A934D65CB497u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCD2A934D65CB497u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponHash, 
                ammo
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ammo_in_clip_raw(
        ped: , 
        weaponHash: , 
        ammo: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDCD2A934D65CB497u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDCD2A934D65CB497u64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash, 
                ammo
        )
    }
}

/// ## Parameters
*



pub fn get_weapon_component_type_model_safe(
        
        
            componentHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0DB57B41EC1DB083u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0DB57B41EC1DB083u64;
        
        let result = invoke_raw!(
            hash,
                componentHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_weapon_component_type_model_raw(
        componentHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0DB57B41EC1DB083u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0DB57B41EC1DB083u64;

        invoke_raw_typed!(
            hash,
                componentHash
        )
    }
}

/// ```
This native returns a true or false value.  
Ped ped = The ped whose weapon you want to check.  
```



pub fn is_ped_current_weapon_silenced_safe(
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65F0C5AE05943EC7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65F0C5AE05943EC7u64;
        
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
pub fn is_ped_current_weapon_silenced_raw(
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65F0C5AE05943EC7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65F0C5AE05943EC7u64;

        invoke_raw_typed!(
            hash,
                ped
        )
    }
}

/// ## Parameters
*



pub fn get_weapon_tint_count_safe(
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DCF6C5CAB2E9BF7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DCF6C5CAB2E9BF7u64;
        
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
pub fn get_weapon_tint_count_raw(
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5DCF6C5CAB2E9BF7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5DCF6C5CAB2E9BF7u64;

        invoke_raw_typed!(
            hash,
                weaponHash
        )
    }
}

/// ## Parameters
*



pub fn get_weapon_component_hud_stats_safe(
        
        
            componentHash: 
        , 
        
        
            outData: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3CAF387AE12E9F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3CAF387AE12E9F8u64;
        
        let result = invoke_raw!(
            hash,
                componentHash, 
                outData
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_weapon_component_hud_stats_raw(
        componentHash: , 
        outData: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3CAF387AE12E9F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3CAF387AE12E9F8u64;

        invoke_raw_typed!(
            hash,
                componentHash, 
                outData
        )
    }
}

/// ## Parameters
*



pub fn get_ped_weapontype_in_slot_safe(
        
        
            ped: 
        , 
        
        
            weaponSlot: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFFED78E9011134Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFFED78E9011134Du64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                weaponSlot
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_ped_weapontype_in_slot_raw(
        ped: , 
        weaponSlot: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEFFED78E9011134Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEFFED78E9011134Du64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponSlot
        )
    }
}

/// ## Parameters
*



pub fn set_pickup_ammo_amount_scaler_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE620FD3512A04F18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE620FD3512A04F18u64;
        
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
pub fn set_pickup_ammo_amount_scaler_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE620FD3512A04F18u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE620FD3512A04F18u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn get_ped_weapon_tint_index_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B9EEDC07BD06B9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B9EEDC07BD06B9Fu64;
        
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
pub fn get_ped_weapon_tint_index_raw(
        ped: , 
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2B9EEDC07BD06B9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2B9EEDC07BD06B9Fu64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash
        )
    }
}

/// ```
// members should be aligned to 8 bytes by default but it's best to use alignas here, just to be sure  
struct WeaponHudStatsData  
{  
	alignas(8) uint8_t hudDamage; // 0x0000  
	alignas(8) uint8_t hudSpeed; // 0x0008  
	alignas(8) uint8_t hudCapacity; // 0x0010  
	alignas(8) uint8_t hudAccuracy; // 0x0018  
	alignas(8) uint8_t hudRange; // 0x0020  
};  
Usage:  
WeaponHudStatsData data;  
if (GET_WEAPON_HUD_STATS(weaponHash, (Any*)&data))  
{  
    // uint8_t damagePercentage = data.hudDamage etc...  
}  
```



pub fn get_weapon_hud_stats_safe(
        
        
            weaponHash: 
        , 
        
        
            outData: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD92C739EE34C9EBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD92C739EE34C9EBAu64;
        
        let result = invoke_raw!(
            hash,
                weaponHash, 
                outData
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_weapon_hud_stats_raw(
        weaponHash: , 
        outData: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD92C739EE34C9EBAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD92C739EE34C9EBAu64;

        invoke_raw_typed!(
            hash,
                weaponHash, 
                outData
        )
    }
}

/// ## Parameters
*



pub fn set_weapon_object_tint_index_safe(
        
        
            weapon: 
        , 
        
        
            tintIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF827589017D4E4A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF827589017D4E4A9u64;
        
        let result = invoke_raw!(
            hash,
                weapon, 
                tintIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_weapon_object_tint_index_raw(
        weapon: , 
        tintIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF827589017D4E4A9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF827589017D4E4A9u64;

        invoke_raw_typed!(
            hash,
                weapon, 
                tintIndex
        )
    }
}

/// ## Parameters
*



pub fn set_ped_infinite_ammo_safe(
        
        
            ped: 
        , 
        
        
            toggle: 
        , 
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3EDCB0505123623Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3EDCB0505123623Bu64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                toggle, 
                weaponHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_ped_infinite_ammo_raw(
        ped: , 
        toggle: , 
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3EDCB0505123623Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3EDCB0505123623Bu64;

        invoke_raw_typed!(
            hash,
                ped, 
                toggle, 
                weaponHash
        )
    }
}

/// ## Parameters
*



pub fn _get_weapon_time_between_shots_safe(
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x065D2AACAD8CF7A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x065D2AACAD8CF7A4u64;
        
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
pub fn _get_weapon_time_between_shots_raw(
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x065D2AACAD8CF7A4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x065D2AACAD8CF7A4u64;

        invoke_raw_typed!(
            hash,
                weaponHash
        )
    }
}

/// ## Parameters
*



pub fn request_weapon_high_detail_model_safe(
        
        
            weaponObject: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48164DBB970AC3F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48164DBB970AC3F0u64;
        
        let result = invoke_raw!(
            hash,
                weaponObject
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_weapon_high_detail_model_raw(
        weaponObject: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x48164DBB970AC3F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x48164DBB970AC3F0u64;

        invoke_raw_typed!(
            hash,
                weaponObject
        )
    }
}

/// ## Parameters
*



pub fn give_weapon_object_to_ped_safe(
        
        
            weaponObject: 
        , 
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1FA61371AF7C4B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1FA61371AF7C4B7u64;
        
        let result = invoke_raw!(
            hash,
                weaponObject, 
                ped
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn give_weapon_object_to_ped_raw(
        weaponObject: , 
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1FA61371AF7C4B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1FA61371AF7C4B7u64;

        invoke_raw_typed!(
            hash,
                weaponObject, 
                ped
        )
    }
}

/// ## Parameters
*



pub fn set_current_ped_vehicle_weapon_safe(
        
        
            ped: 
        , 
        
        
            weaponHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75C55983C2C39DAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75C55983C2C39DAAu64;
        
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
pub fn set_current_ped_vehicle_weapon_raw(
        ped: , 
        weaponHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x75C55983C2C39DAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x75C55983C2C39DAAu64;

        invoke_raw_typed!(
            hash,
                ped, 
                weaponHash
        )
    }
}

/// Changes the weapon damage output by the given multiplier value.
Does NOT need to be called every frame.



pub fn _set_weapon_damage_modifier_safe(
        
        
            weaponHash: 
        , 
        
        
            damageMultiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4757F00BC6323CFEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4757F00BC6323CFEu64;
        
        let result = invoke_raw!(
            hash,
                weaponHash, 
                damageMultiplier
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_weapon_damage_modifier_raw(
        weaponHash: , 
        damageMultiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4757F00BC6323CFEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4757F00BC6323CFEu64;

        invoke_raw_typed!(
            hash,
                weaponHash, 
                damageMultiplier
        )
    }
}

