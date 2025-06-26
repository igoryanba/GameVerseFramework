//! FIRE native functions
//! 
//! Functions for the fire category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn stop_entity_fire_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F0DD2EBBB651AFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F0DD2EBBB651AFFu64;
        
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
pub fn stop_entity_fire_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F0DD2EBBB651AFFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F0DD2EBBB651AFFu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn is_entity_on_fire_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28D3FED7190D3A0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28D3FED7190D3A0Bu64;
        
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
pub fn is_entity_on_fire_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x28D3FED7190D3A0Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x28D3FED7190D3A0Bu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn add_owned_explosion_safe(
        
        
            ped: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            explosionType: 
        , 
        
        
            damageScale: 
        , 
        
        
            isAudible: 
        , 
        
        
            isInvisible: 
        , 
        
        
            cameraShake: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x172AA1B624FA1013u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x172AA1B624FA1013u64;
        
        let result = invoke_raw!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                explosionType, 
                damageScale, 
                isAudible, 
                isInvisible, 
                cameraShake
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_owned_explosion_raw(
        ped: , 
        x: , 
        y: , 
        z: , 
        explosionType: , 
        damageScale: , 
        isAudible: , 
        isInvisible: , 
        cameraShake: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x172AA1B624FA1013u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x172AA1B624FA1013u64;

        invoke_raw_typed!(
            hash,
                ped, 
                x, 
                y, 
                z, 
                explosionType, 
                damageScale, 
                isAudible, 
                isInvisible, 
                cameraShake
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _get_entity_inside_explosion_sphere_safe(
        
        
            explosionType: 
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
        let hash = 0xB3CD51E3DB86F176u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3CD51E3DB86F176u64;
        
        let result = invoke_raw!(
            hash,
                explosionType, 
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
pub fn _get_entity_inside_explosion_sphere_raw(
        explosionType: , 
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3CD51E3DB86F176u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3CD51E3DB86F176u64;

        invoke_raw_typed!(
            hash,
                explosionType, 
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// ## Parameters
*



pub fn remove_script_fire_safe(
        
        
            fireHandle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FF548385680673Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FF548385680673Fu64;
        
        let result = invoke_raw!(
            hash,
                fireHandle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_script_fire_raw(
        fireHandle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7FF548385680673Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7FF548385680673Fu64;

        invoke_raw_typed!(
            hash,
                fireHandle
        )
    }
}

/// ```
NativeDB Added Parameter 9: BOOL noDamage
```

```
BOOL isAudible = If explosion makes a sound.  
BOOL isInvisible = If the explosion is invisible or not.
BOOL noDamage = false: damage || nodamage = true: no damage
```

```c
enum eExplosionTag
{
	DONTCARE = -1,
	GRENADE = 0,
	GRENADELAUNCHER = 1,
	STICKYBOMB = 2,
	MOLOTOV = 3,
	ROCKET = 4,
	TANKSHELL = 5,
	HI_OCTANE = 6,
	CAR = 7,
	PLANE = 8,
	PETROL_PUMP = 9,
	BIKE = 10,
	DIR_STEAM = 11,
	DIR_FLAME = 12,
	DIR_WATER_HYDRANT = 13,
	DIR_GAS_CANISTER = 14,
	BOAT = 15,
	SHIP_DESTROY = 16,
	TRUCK = 17,
	BULLET = 18,
	SMOKE_GRENADE_LAUNCHER = 19,
	SMOKE_GRENADE = 20,
	BZGAS = 21,
	FLARE = 22,
	GAS_CANISTER = 23,
	EXTINGUISHER = 24,
	PROGRAMMABLEAR = 25,
	TRAIN = 26,
	BARREL = 27,
	PROPANE = 28,
	BLIMP = 29,
	DIR_FLAME_EXPLODE = 30,
	TANKER = 31,
	PLANE_ROCKET = 32,
	VEHICLE_BULLET = 33,
	GAS_TANK = 34,
	BIRD_CRAP = 35,
	RAILGUN = 36,
	BLIMP2 = 37,
	FIREWORK = 38,
	SNOWBALL = 39,
	PROXMINE = 40,
	VALKYRIE_CANNON = 41,
	AIR_DEFENCE = 42,
	PIPEBOMB = 43,
	VEHICLEMINE = 44,
	EXPLOSIVEAMMO = 45,
	APCSHELL = 46,
	BOMB_CLUSTER = 47,
	BOMB_GAS = 48,
	BOMB_INCENDIARY = 49,
	BOMB_STANDARD = 50,
	TORPEDO = 51,
	TORPEDO_UNDERWATER = 52,
	BOMBUSHKA_CANNON = 53,
	BOMB_CLUSTER_SECONDARY = 54,
	HUNTER_BARRAGE = 55,
	HUNTER_CANNON = 56,
	ROGUE_CANNON = 57,
	MINE_UNDERWATER = 58,
	ORBITAL_CANNON = 59,
	BOMB_STANDARD_WIDE = 60,
	EXPLOSIVEAMMO_SHOTGUN = 61,
	OPPRESSOR2_CANNON = 62,
	MORTAR_KINETIC = 63,
	VEHICLEMINE_KINETIC = 64,
	VEHICLEMINE_EMP = 65,
	VEHICLEMINE_SPIKE = 66,
	VEHICLEMINE_SLICK = 67,
	VEHICLEMINE_TAR = 68,
	SCRIPT_DRONE = 69,
	RAYGUN = 70,
	BURIEDMINE = 71,
	SCRIPT_MISSILE = 72,
	RCTANK_ROCKET = 73,
	BOMB_WATER = 74,
	BOMB_WATER_SECONDARY = 75,
	MINE_CNCSPIKE = 76,
	BZGAS_MK2 = 77,
	FLASHGRENADE = 78,
	STUNGRENADE = 79,
	CNC_KINETICRAM = 80,
	SCRIPT_MISSILE_LARGE = 81,
	SUBMARINE_BIG = 82,
	EMPLAUNCHER_EMP = 83
};
```



pub fn add_explosion_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            explosionType: 
        , 
        
        
            damageScale: 
        , 
        
        
            isAudible: 
        , 
        
        
            isInvisible: 
        , 
        
        
            cameraShake: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3AD2BDBAEE269ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3AD2BDBAEE269ACu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                explosionType, 
                damageScale, 
                isAudible, 
                isInvisible, 
                cameraShake
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_explosion_raw(
        x: , 
        y: , 
        z: , 
        explosionType: , 
        damageScale: , 
        isAudible: , 
        isInvisible: , 
        cameraShake: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3AD2BDBAEE269ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3AD2BDBAEE269ACu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                explosionType, 
                damageScale, 
                isAudible, 
                isInvisible, 
                cameraShake
        )
    }
}

/// ```
Returns a handle to the first entity within the a circle spawned inside the 2 points from a radius.
```



pub fn _get_entity_inside_explosion_area_safe(
        
        
            explosionType: 
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
        
        
            radius: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14BA4BA137AF6CECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14BA4BA137AF6CECu64;
        
        let result = invoke_raw!(
            hash,
                explosionType, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                radius
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_entity_inside_explosion_area_raw(
        explosionType: , 
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14BA4BA137AF6CECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14BA4BA137AF6CECu64;

        invoke_raw_typed!(
            hash,
                explosionType, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                radius
        )
    }
}

/// ## Parameters
*



pub fn is_explosion_in_area_safe(
        
        
            explosionType: 
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E2EBA0EE7CED0E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E2EBA0EE7CED0E0u64;
        
        let result = invoke_raw!(
            hash,
                explosionType, 
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
pub fn is_explosion_in_area_raw(
        explosionType: , 
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2E2EBA0EE7CED0E0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2E2EBA0EE7CED0E0u64;

        invoke_raw_typed!(
            hash,
                explosionType, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2
        )
    }
}

/// SET_FIRE_*

```
NativeDB Introduced: v1734
```



pub fn _set_fire_spread_rate_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F390AC4155099BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F390AC4155099BAu64;
        
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
pub fn _set_fire_spread_rate_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8F390AC4155099BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8F390AC4155099BAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
Returns TRUE if it found something. FALSE if not.  
```



pub fn get_closest_fire_pos_safe(
        
        
            outPosition: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x352A9F6BCF90081Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x352A9F6BCF90081Fu64;
        
        let result = invoke_raw!(
            hash,
                outPosition, 
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
pub fn get_closest_fire_pos_raw(
        outPosition: , 
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x352A9F6BCF90081Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x352A9F6BCF90081Fu64;

        invoke_raw_typed!(
            hash,
                outPosition, 
                x, 
                y, 
                z
        )
    }
}

/// ## Parameters
*



pub fn stop_fire_in_range_safe(
        
        
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
        let hash = 0x056A8A219B8E829Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x056A8A219B8E829Fu64;
        
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
pub fn stop_fire_in_range_raw(
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x056A8A219B8E829Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x056A8A219B8E829Fu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// See [`IS_POINT_IN_ANGLED_AREA`](#_0x2A70BAE8883E4C81) for the definition of an angled area.



pub fn is_explosion_in_angled_area_safe(
        
        
            explosionType: 
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
        
        
            width: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA079A6C51525DC4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA079A6C51525DC4Bu64;
        
        let result = invoke_raw!(
            hash,
                explosionType, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_explosion_in_angled_area_raw(
        explosionType: , 
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        width: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA079A6C51525DC4Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA079A6C51525DC4Bu64;

        invoke_raw_typed!(
            hash,
                explosionType, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width
        )
    }
}

/// ## Parameters
*



pub fn start_entity_fire_safe(
        
        
            entity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6A9D9708F6F23DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6A9D9708F6F23DFu64;
        
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
pub fn start_entity_fire_raw(
        entity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6A9D9708F6F23DFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6A9D9708F6F23DFu64;

        invoke_raw_typed!(
            hash,
                entity
        )
    }
}

/// ## Parameters
*



pub fn add_explosion_with_user_vfx_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            explosionType: 
        , 
        
        
            explosionFx: 
        , 
        
        
            damageScale: 
        , 
        
        
            isAudible: 
        , 
        
        
            isInvisible: 
        , 
        
        
            cameraShake: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36DD3FE58B5E5212u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36DD3FE58B5E5212u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                explosionType, 
                explosionFx, 
                damageScale, 
                isAudible, 
                isInvisible, 
                cameraShake
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_explosion_with_user_vfx_raw(
        x: , 
        y: , 
        z: , 
        explosionType: , 
        explosionFx: , 
        damageScale: , 
        isAudible: , 
        isInvisible: , 
        cameraShake: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x36DD3FE58B5E5212u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x36DD3FE58B5E5212u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                explosionType, 
                explosionFx, 
                damageScale, 
                isAudible, 
                isInvisible, 
                cameraShake
        )
    }
}

/// ```
Starts a fire:  
xyz: Location of fire  
maxChildren: The max amount of times a fire can spread to other objects. Must be 25 or less, or the function will do nothing.  
isGasFire: Whether or not the fire is powered by gasoline.  
```



pub fn start_script_fire_safe(
        
        
            X: 
        , 
        
        
            Y: 
        , 
        
        
            Z: 
        , 
        
        
            maxChildren: 
        , 
        
        
            isGasFire: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B83617E04503888u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B83617E04503888u64;
        
        let result = invoke_raw!(
            hash,
                X, 
                Y, 
                Z, 
                maxChildren, 
                isGasFire
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_script_fire_raw(
        X: , 
        Y: , 
        Z: , 
        maxChildren: , 
        isGasFire: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6B83617E04503888u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6B83617E04503888u64;

        invoke_raw_typed!(
            hash,
                X, 
                Y, 
                Z, 
                maxChildren, 
                isGasFire
        )
    }
}

/// ## Parameters
*



pub fn is_explosion_in_sphere_safe(
        
        
            explosionType: 
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
        let hash = 0xAB0F816885B0E483u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB0F816885B0E483u64;
        
        let result = invoke_raw!(
            hash,
                explosionType, 
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
pub fn is_explosion_in_sphere_raw(
        explosionType: , 
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAB0F816885B0E483u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAB0F816885B0E483u64;

        invoke_raw_typed!(
            hash,
                explosionType, 
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// ## Parameters
*



pub fn is_explosion_active_in_area_safe(
        
        
            explosionType: 
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6070104B699B2EF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6070104B699B2EF4u64;
        
        let result = invoke_raw!(
            hash,
                explosionType, 
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
pub fn is_explosion_active_in_area_raw(
        explosionType: , 
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6070104B699B2EF4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6070104B699B2EF4u64;

        invoke_raw_typed!(
            hash,
                explosionType, 
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



pub fn get_number_of_fires_in_range_safe(
        
        
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
        let hash = 0x50CAD495A460B305u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50CAD495A460B305u64;
        
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
pub fn get_number_of_fires_in_range_raw(
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x50CAD495A460B305u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x50CAD495A460B305u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius
        )
    }
}

