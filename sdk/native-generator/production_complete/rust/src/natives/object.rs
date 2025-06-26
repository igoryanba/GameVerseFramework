//! OBJECT native functions
//! 
//! Functions for the object category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
Spawns one or more money pickups.  
x: The X-component of the world position to spawn the money pickups at.  
y: The Y-component of the world position to spawn the money pickups at.  
z: The Z-component of the world position to spawn the money pickups at.  
value: The combined value of the pickups (in dollars).  
amount: The number of pickups to spawn.  
model: The model to use, or 0 for default money model.  
Example:  
CREATE_MONEY_PICKUPS(x, y, z, 1000, 3, 0x684a97ae);  
Spawns 3 spray cans that'll collectively give $1000 when picked up. (Three spray cans, each giving $334, $334, $332 = $1000).  
==============================================  
Max is 2000 in MP. So if you put the amount to 20, but the value to $400,000 eg. They will only be able to pickup 20 - $2,000 bags. So, $40,000  
```



pub fn create_money_pickups_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            value: 
        , 
        
        
            amount: 
        , 
        
        
            model: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0589B5E791CE9B2Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0589B5E791CE9B2Bu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                value, 
                amount, 
                model
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_money_pickups_raw(
        x: , 
        y: , 
        z: , 
        value: , 
        amount: , 
        model: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0589B5E791CE9B2Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0589B5E791CE9B2Bu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                value, 
                amount, 
                model
        )
    }
}

/// ## Parameters
*



pub fn is_garage_empty_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90E47239EA1980B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90E47239EA1980B8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_garage_empty_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x90E47239EA1980B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x90E47239EA1980B8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _0x834344a414c7c85d_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x834344A414C7C85Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x834344A414C7C85Du64;
        
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
pub fn _0x834344a414c7c85d_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x834344A414C7C85Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x834344A414C7C85Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
Defines the state of a destructible object.
Use the GET_RAYFIRE_MAP_OBJECT native to find an object's handle with its name / coords.
State 2 == object just spawned
State 4 == Beginning of the animation
State 6 == Start animation
State 9 == End of the animation
```



pub fn set_state_of_rayfire_map_object_safe(
        
        
            object: 
        , 
        
        
            state: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C29F698D404C5E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C29F698D404C5E1u64;
        
        let result = invoke_raw!(
            hash,
                object, 
                state
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_state_of_rayfire_map_object_raw(
        object: , 
        state: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5C29F698D404C5E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5C29F698D404C5E1u64;

        invoke_raw_typed!(
            hash,
                object, 
                state
        )
    }
}

/// ## Parameters
*



pub fn set_local_player_can_collect_portable_pickups_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78857FC65CADB909u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78857FC65CADB909u64;
        
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
pub fn set_local_player_can_collect_portable_pickups_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78857FC65CADB909u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78857FC65CADB909u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn _0x8881c98a31117998_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8881C98A31117998u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8881C98A31117998u64;
        
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
pub fn _0x8881c98a31117998_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8881C98A31117998u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8881C98A31117998u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn get_object_fragment_damage_health_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6FBFD079B8D0596u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6FBFD079B8D0596u64;
        
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
pub fn get_object_fragment_damage_health_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6FBFD079B8D0596u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6FBFD079B8D0596u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1365
```



pub fn is_object_a_portable_pickup_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0378C08504160D0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0378C08504160D0Du64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_object_a_portable_pickup_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0378C08504160D0Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0378C08504160D0Du64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ```
NativeDB Introduced: v1365
```



pub fn _0x63ecf581bc70e363_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63ECF581BC70E363u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63ECF581BC70E363u64;
        
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
pub fn _0x63ecf581bc70e363_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x63ECF581BC70E363u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x63ECF581BC70E363u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn remove_pickup_safe(
        
        
            pickup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3288D8ACAECD2AB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3288D8ACAECD2AB2u64;
        
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
pub fn remove_pickup_raw(
        pickup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3288D8ACAECD2AB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3288D8ACAECD2AB2u64;

        invoke_raw_typed!(
            hash,
                pickup
        )
    }
}

/// ## Parameters
*



pub fn _0x39a5fb7eaf150840_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39A5FB7EAF150840u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39A5FB7EAF150840u64;
        
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
pub fn _0x39a5fb7eaf150840_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x39A5FB7EAF150840u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x39A5FB7EAF150840u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Hardcoded not to work in multiplayer environments.
Native name between `SET_LOCAL_PLAYER_VISIBLE_LOCALLY` & `SET_MAX_WANTED_LEVEL`.

```
OBJECT::_9B12F9A24FABEDB0(${prop_gate_prison_01}, 1845.0, 2605.0, 45.0, 0, 0.0, 50.0, 0);  //door unlocked
OBJECT::_9B12F9A24FABEDB0(${prop_gate_prison_01}, 1845.0, 2605.0, 45.0, 1, 0.0, 50.0, 0);  //door locked
```



pub fn _door_control_safe(
        
        
            modelHash: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            locked: 
        , 
        
        
            xRotMult: 
        , 
        
        
            yRotMult: 
        , 
        
        
            zRotMult: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B12F9A24FABEDB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B12F9A24FABEDB0u64;
        
        let result = invoke_raw!(
            hash,
                modelHash, 
                x, 
                y, 
                z, 
                locked, 
                xRotMult, 
                yRotMult, 
                zRotMult
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _door_control_raw(
        modelHash: , 
        x: , 
        y: , 
        z: , 
        locked: , 
        xRotMult: , 
        yRotMult: , 
        zRotMult: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9B12F9A24FABEDB0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9B12F9A24FABEDB0u64;

        invoke_raw_typed!(
            hash,
                modelHash, 
                x, 
                y, 
                z, 
                locked, 
                xRotMult, 
                yRotMult, 
                zRotMult
        )
    }
}

/// ```
NativeDB Removed Parameter 3: int R
NativeDB Removed Parameter 4: int G
NativeDB Removed Parameter 5: int B
NativeDB Introduced: v757
```



pub fn _0x3b2fd68db5f8331c_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B2FD68DB5F8331Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B2FD68DB5F8331Cu64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x3b2fd68db5f8331c_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3B2FD68DB5F8331Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3B2FD68DB5F8331Cu64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ## Parameters
*



pub fn _0x46f3add1e2d5baf2_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46F3ADD1E2D5BAF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46F3ADD1E2D5BAF2u64;
        
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
pub fn _0x46f3add1e2d5baf2_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46F3ADD1E2D5BAF2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46F3ADD1E2D5BAF2u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Includes networking check: ownership vs. or the door itself



pub fn door_system_set_automatic_rate_safe(
        
        
            doorHash: 
        , 
        
        
            rate: 
        , 
        
        
            requestDoor: 
        , 
        
        
            forceUpdate: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03C27E13B42A0E82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03C27E13B42A0E82u64;
        
        let result = invoke_raw!(
            hash,
                doorHash, 
                rate, 
                requestDoor, 
                forceUpdate
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn door_system_set_automatic_rate_raw(
        doorHash: , 
        rate: , 
        requestDoor: , 
        forceUpdate: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x03C27E13B42A0E82u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x03C27E13B42A0E82u64;

        invoke_raw_typed!(
            hash,
                doorHash, 
                rate, 
                requestDoor, 
                forceUpdate
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0x3bd770d281982db5_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BD770D281982DB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BD770D281982DB5u64;
        
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
pub fn _0x3bd770d281982db5_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3BD770D281982DB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3BD770D281982DB5u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
if (OBJECT::IS_DOOR_REGISTERED_WITH_SYSTEM(doorHash))
{
    OBJECT::REMOVE_DOOR_FROM_SYSTEM(doorHash);
}
```



pub fn is_door_registered_with_system_safe(
        
        
            doorHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC153C43EA202C8C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC153C43EA202C8C1u64;
        
        let result = invoke_raw!(
            hash,
                doorHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_door_registered_with_system_raw(
        doorHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC153C43EA202C8C1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC153C43EA202C8C1u64;

        invoke_raw_typed!(
            hash,
                doorHash
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0xbffe53ae7e67fcdc_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFFE53AE7E67FCDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFFE53AE7E67FCDCu64;
        
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
pub fn _0xbffe53ae7e67fcdc_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFFE53AE7E67FCDCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFFE53AE7E67FCDCu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x826d1ee4d1cafc78_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x826D1EE4D1CAFC78u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x826D1EE4D1CAFC78u64;
        
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
pub fn _0x826d1ee4d1cafc78_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x826D1EE4D1CAFC78u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x826D1EE4D1CAFC78u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Added Parameter 2: Any p1
```



pub fn has_object_been_broken_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8ABFB70C49CC43E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8ABFB70C49CC43E2u64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_object_been_broken_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8ABFB70C49CC43E2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8ABFB70C49CC43E2u64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ## Parameters
*



pub fn set_max_num_portable_pickups_carried_by_player_safe(
        
        
            modelHash: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BF3B3BD47D79C08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BF3B3BD47D79C08u64;
        
        let result = invoke_raw!(
            hash,
                modelHash, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_max_num_portable_pickups_carried_by_player_raw(
        modelHash: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BF3B3BD47D79C08u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BF3B3BD47D79C08u64;

        invoke_raw_typed!(
            hash,
                modelHash, 
                p1
        )
    }
}

/// ```
Sets the ajar angle of a door.
Ranges from -1.0 to 1.0, and 0.0 is closed / default.
```



pub fn door_system_set_open_ratio_safe(
        
        
            doorHash: 
        , 
        
        
            ajar: 
        , 
        
        
            requestDoor: 
        , 
        
        
            forceUpdate: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6E6FBA95C7324ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6E6FBA95C7324ACu64;
        
        let result = invoke_raw!(
            hash,
                doorHash, 
                ajar, 
                requestDoor, 
                forceUpdate
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn door_system_set_open_ratio_raw(
        doorHash: , 
        ajar: , 
        requestDoor: , 
        forceUpdate: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB6E6FBA95C7324ACu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB6E6FBA95C7324ACu64;

        invoke_raw_typed!(
            hash,
                doorHash, 
                ajar, 
                requestDoor, 
                forceUpdate
        )
    }
}

/// ```
Adds an area that seems to be related to pickup physics behavior.
Max amount of areas is 10. Only works in multiplayer.

ADD_*

NativeDB Introduced: v1290
```



pub fn _0xd4a7a435b3710d05_safe(
        
        
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
        let hash = 0xD4A7A435B3710D05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4A7A435B3710D05u64;
        
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
pub fn _0xd4a7a435b3710d05_raw(
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD4A7A435B3710D05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD4A7A435B3710D05u64;

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
Disabling/enabling a player from getting pickups. From the scripts:
OBJECT::_616093EC6B139DD9(PLAYER::PLAYER_ID(), ${pickup_portable_package}, 0);
OBJECT::_616093EC6B139DD9(PLAYER::PLAYER_ID(), ${pickup_portable_package}, 0);
OBJECT::_616093EC6B139DD9(PLAYER::PLAYER_ID(), ${pickup_portable_package}, 1);
OBJECT::_616093EC6B139DD9(PLAYER::PLAYER_ID(), ${pickup_portable_package}, 0);
OBJECT::_616093EC6B139DD9(PLAYER::PLAYER_ID(), ${pickup_armour_standard}, 0);
OBJECT::_616093EC6B139DD9(PLAYER::PLAYER_ID(), ${pickup_armour_standard}, 1);
SET_PLAYER_*
```



pub fn _toggle_use_pickups_for_player_safe(
        
        
            player: 
        , 
        
        
            pickupHash: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x616093EC6B139DD9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x616093EC6B139DD9u64;
        
        let result = invoke_raw!(
            hash,
                player, 
                pickupHash, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _toggle_use_pickups_for_player_raw(
        player: , 
        pickupHash: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x616093EC6B139DD9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x616093EC6B139DD9u64;

        invoke_raw_typed!(
            hash,
                player, 
                pickupHash, 
                toggle
        )
    }
}

/// Creates an ambient pickup given the hash. Pickup hashes can be found [here](https://gist.github.com/4mmonium/1eabfb6b3996e3aa6b9525a3eccf8a0b).



pub fn create_ambient_pickup_safe(
        
        
            pickupHash: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            flags: 
        , 
        
        
            value: 
        , 
        
        
            modelHash: 
        , 
        
        
            returnHandle: 
        , 
        
        
            p8: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x673966A0C0FD7171u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x673966A0C0FD7171u64;
        
        let result = invoke_raw!(
            hash,
                pickupHash, 
                posX, 
                posY, 
                posZ, 
                flags, 
                value, 
                modelHash, 
                returnHandle, 
                p8
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_ambient_pickup_raw(
        pickupHash: , 
        posX: , 
        posY: , 
        posZ: , 
        flags: , 
        value: , 
        modelHash: , 
        returnHandle: , 
        p8: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x673966A0C0FD7171u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x673966A0C0FD7171u64;

        invoke_raw_typed!(
            hash,
                pickupHash, 
                posX, 
                posY, 
                posZ, 
                flags, 
                value, 
                modelHash, 
                returnHandle, 
                p8
        )
    }
}

/// Overrides a flag on the object which determines if the object should be avoided by a vehicle in task: CTaskVehicleGoToPointWithAvoidanceAutomobile.
Tested on vehicles that were created by the vehicle generators.



pub fn set_object_force_vehicles_to_avoid_safe(
        
        
            object: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77F33F2CCF64B3AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77F33F2CCF64B3AAu64;
        
        let result = invoke_raw!(
            hash,
                object, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_object_force_vehicles_to_avoid_raw(
        object: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x77F33F2CCF64B3AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x77F33F2CCF64B3AAu64;

        invoke_raw_typed!(
            hash,
                object, 
                toggle
        )
    }
}

/// ```
`object`: The des-object handle to get the animation progress from.
Return value is a float between 0.0 and 1.0, 0.0 is the beginning of the animation, 1.0 is the end. Value resets to 0.0 instantly after reaching 1.0.
```



pub fn get_rayfire_map_object_anim_phase_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x260EE4FDBDF4DB01u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x260EE4FDBDF4DB01u64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_rayfire_map_object_anim_phase_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x260EE4FDBDF4DB01u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x260EE4FDBDF4DB01u64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ## Parameters
*



pub fn clear_objects_inside_garage_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x190428512B240692u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x190428512B240692u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn clear_objects_inside_garage_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x190428512B240692u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x190428512B240692u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Sets color of embedded light source.
Only appears in am_mp_nightclub.c for the nightclub dancefloor.

Not sure what p1 does, seems to only ever be '1' in scripts.

```
NativeDB Introduced: v1493
```



pub fn _set_object_light_color_safe(
        
        
            object: 
        , 
        
        
            p1: 
        , 
        
        
            r: 
        , 
        
        
            g: 
        , 
        
        
            b: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F048334B4A4E774u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F048334B4A4E774u64;
        
        let result = invoke_raw!(
            hash,
                object, 
                p1, 
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
pub fn _set_object_light_color_raw(
        object: , 
        p1: , 
        r: , 
        g: , 
        b: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5F048334B4A4E774u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5F048334B4A4E774u64;

        invoke_raw_typed!(
            hash,
                object, 
                p1, 
                r, 
                g, 
                b
        )
    }
}

/// ## Parameters
*



pub fn is_door_closed_safe(
        
        
            doorHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC531EE8A1145A149u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC531EE8A1145A149u64;
        
        let result = invoke_raw!(
            hash,
                doorHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_door_closed_raw(
        doorHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC531EE8A1145A149u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC531EE8A1145A149u64;

        invoke_raw_typed!(
            hash,
                doorHash
        )
    }
}

/// Clears the fields sets by [N_0xc7f29ca00f46350e](#_0xC7F29CA00F46350E) (1604 retail: 0x1424A7A10, 0x1424A7A11) and iterates over the global CDoor's bucket-list.

Related to its "Pre-networked state"?



pub fn _0x701fda1e82076ba4_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x701FDA1E82076BA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x701FDA1E82076BA4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x701fda1e82076ba4_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x701FDA1E82076BA4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x701FDA1E82076BA4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _set_texture_variation_of_closest_object_of_type_safe(
        
        
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
        let hash = 0xF12E33034D887F66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF12E33034D887F66u64;
        
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
pub fn _set_texture_variation_of_closest_object_of_type_raw(
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF12E33034D887F66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF12E33034D887F66u64;

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
NativeDB Introduced: v1180
```



pub fn _0x4c134b4df76025d0_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C134B4DF76025D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C134B4DF76025D0u64;
        
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
pub fn _0x4c134b4df76025d0_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C134B4DF76025D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C134B4DF76025D0u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Includes networking check: ownership vs. or the door itself



pub fn door_system_set_hold_open_safe(
        
        
            doorHash: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9B71952F78A2640u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9B71952F78A2640u64;
        
        let result = invoke_raw!(
            hash,
                doorHash, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn door_system_set_hold_open_raw(
        doorHash: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9B71952F78A2640u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9B71952F78A2640u64;

        invoke_raw_typed!(
            hash,
                doorHash, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0xb2d0bde54f0e8e5a_safe(
        
        
            object: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2D0BDE54F0E8E5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2D0BDE54F0E8E5Au64;
        
        let result = invoke_raw!(
            hash,
                object, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb2d0bde54f0e8e5a_raw(
        object: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2D0BDE54F0E8E5Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2D0BDE54F0E8E5Au64;

        invoke_raw_typed!(
            hash,
                object, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_object_a_pickup_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC481C641EBBD27Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC481C641EBBD27Du64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_object_a_pickup_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFC481C641EBBD27Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFC481C641EBBD27Du64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ## Parameters
*



pub fn get_weapon_type_from_pickup_type_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x08F96CA6C551AD51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x08F96CA6C551AD51u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_weapon_type_from_pickup_type_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x08F96CA6C551AD51u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x08F96CA6C551AD51u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x394CD08E31313C28 native function



pub fn _0x394cd08e31313c28_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x394CD08E31313C28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x394CD08E31313C28u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x394cd08e31313c28_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x394CD08E31313C28u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x394CD08E31313C28u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _0xaa059c615de9dd03_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA059C615DE9DD03u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA059C615DE9DD03u64;
        
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
pub fn _0xaa059c615de9dd03_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAA059C615DE9DD03u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAA059C615DE9DD03u64;

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



pub fn _0xadf084fb8f075d06_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADF084FB8F075D06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADF084FB8F075D06u64;
        
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
pub fn _0xadf084fb8f075d06_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADF084FB8F075D06u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADF084FB8F075D06u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn _get_pickup_generation_range_multiplier_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3ECA65C7317F174u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3ECA65C7317F174u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_pickup_generation_range_multiplier_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB3ECA65C7317F174u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB3ECA65C7317F174u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Maximum amount of pickup models that can be disallowed is 30.
SET_LOCAL_PLAYER_*
```



pub fn _set_local_player_can_use_pickups_with_this_model_safe(
        
        
            modelHash: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88EAEC617CD26926u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88EAEC617CD26926u64;
        
        let result = invoke_raw!(
            hash,
                modelHash, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_local_player_can_use_pickups_with_this_model_raw(
        modelHash: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x88EAEC617CD26926u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x88EAEC617CD26926u64;

        invoke_raw_typed!(
            hash,
                modelHash, 
                toggle
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0x734e1714d077da9a_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x734E1714D077DA9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x734E1714D077DA9Au64;
        
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
pub fn _0x734e1714d077da9a_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x734E1714D077DA9Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x734E1714D077DA9Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Lockstates not applied and CNetObjDoor's not created until [DOOR_SYSTEM_GET_IS_PHYSICS_LOADED](#_0xDF97CDD4FC08FD34) returns true.



pub fn door_system_set_door_state_safe(
        
        
            doorHash: 
        , 
        
        
            state: 
        , 
        
        
            requestDoor: 
        , 
        
        
            forceUpdate: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BAB9442830C7F53u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BAB9442830C7F53u64;
        
        let result = invoke_raw!(
            hash,
                doorHash, 
                state, 
                requestDoor, 
                forceUpdate
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn door_system_set_door_state_raw(
        doorHash: , 
        state: , 
        requestDoor: , 
        forceUpdate: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6BAB9442830C7F53u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6BAB9442830C7F53u64;

        invoke_raw_typed!(
            hash,
                doorHash, 
                state, 
                requestDoor, 
                forceUpdate
        )
    }
}

/// p5 only set to true in single player native scripts. Door hashes normally look like `PROP_[int]_DOOR_[int]` for interior doors and `PROP_BUILDING_[int]_DOOR_[int]` exterior doors but you can just make up your own hash if you want.

If scriptDoor is true, register the door on the script handler host (note: there's a hardcap on the number of script IDs that can be added to the system at a given time). If scriptDoor and isLocal are both false, the door is considered to be in a "Persists w/o netobj" state.

A simple "localized" door-system (with hundreds/thousands of doors) can be created by setting p5, p6, and p7 to false and using EventHandlers to synchronize the states to: [DOOR_SYSTEM_SET_DOOR_STATE](#_0x6BAB9442830C7F53), [DOOR_SYSTEM_SET_OPEN_RATIO](#_0xB6E6FBA95C7324AC), [DOOR_SYSTEM_SET_HOLD_OPEN](#_0xD9B71952F78A2640), etc.



pub fn add_door_to_system_safe(
        
        
            doorHash: 
        , 
        
        
            modelHash: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            p5: 
        , 
        
        
            scriptDoor: 
        , 
        
        
            isLocal: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F8838D03D1DC226u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F8838D03D1DC226u64;
        
        let result = invoke_raw!(
            hash,
                doorHash, 
                modelHash, 
                x, 
                y, 
                z, 
                p5, 
                scriptDoor, 
                isLocal
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_door_to_system_raw(
        doorHash: , 
        modelHash: , 
        x: , 
        y: , 
        z: , 
        p5: , 
        scriptDoor: , 
        isLocal: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6F8838D03D1DC226u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6F8838D03D1DC226u64;

        invoke_raw_typed!(
            hash,
                doorHash, 
                modelHash, 
                x, 
                y, 
                z, 
                p5, 
                scriptDoor, 
                isLocal
        )
    }
}

/// ```
Pickup hashes: pastebin.com/8EuSv2r1  
flags:  
8 (1 << 3): place on ground  
512 (1 << 9): spin around  
```



pub fn create_pickup_rotate_safe(
        
        
            pickupHash: 
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
        
        
            flag: 
        , 
        
        
            amount: 
        , 
        
        
            p9: 
        , 
        
        
            p10: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x891804727E0A98B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x891804727E0A98B7u64;
        
        let result = invoke_raw!(
            hash,
                pickupHash, 
                posX, 
                posY, 
                posZ, 
                rotX, 
                rotY, 
                rotZ, 
                flag, 
                amount, 
                p9, 
                p10, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_pickup_rotate_raw(
        pickupHash: , 
        posX: , 
        posY: , 
        posZ: , 
        rotX: , 
        rotY: , 
        rotZ: , 
        flag: , 
        amount: , 
        p9: , 
        p10: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x891804727E0A98B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x891804727E0A98B7u64;

        invoke_raw_typed!(
            hash,
                pickupHash, 
                posX, 
                posY, 
                posZ, 
                rotX, 
                rotY, 
                rotZ, 
                flag, 
                amount, 
                p9, 
                p10, 
                modelHash
        )
    }
}

/// ## Parameters
*



pub fn does_pickup_object_exist_safe(
        
        
            pickupObject: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9EFB6DBF7DAAEA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9EFB6DBF7DAAEA3u64;
        
        let result = invoke_raw!(
            hash,
                pickupObject
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_pickup_object_exist_raw(
        pickupObject: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9EFB6DBF7DAAEA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9EFB6DBF7DAAEA3u64;

        invoke_raw_typed!(
            hash,
                pickupObject
        )
    }
}

/// ## Parameters
*



pub fn has_closest_object_of_type_been_completely_destroyed_safe(
        
        
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
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46494A2475701343u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46494A2475701343u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                modelHash, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_closest_object_of_type_been_completely_destroyed_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        modelHash: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x46494A2475701343u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x46494A2475701343u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                modelHash, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn door_system_set_automatic_distance_safe(
        
        
            doorHash: 
        , 
        
        
            distance: 
        , 
        
        
            requestDoor: 
        , 
        
        
            forceUpdate: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BA001CB45CBF627u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BA001CB45CBF627u64;
        
        let result = invoke_raw!(
            hash,
                doorHash, 
                distance, 
                requestDoor, 
                forceUpdate
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn door_system_set_automatic_distance_raw(
        doorHash: , 
        distance: , 
        requestDoor: , 
        forceUpdate: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9BA001CB45CBF627u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9BA001CB45CBF627u64;

        invoke_raw_typed!(
            hash,
                doorHash, 
                distance, 
                requestDoor, 
                forceUpdate
        )
    }
}

/// ## Parameters
*



pub fn _0xa08fe5e49bdc39dd_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA08FE5E49BDC39DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA08FE5E49BDC39DDu64;
        
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
pub fn _0xa08fe5e49bdc39dd_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA08FE5E49BDC39DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA08FE5E49BDC39DDu64;

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



pub fn is_object_visible_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B32ACE6326A7546u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B32ACE6326A7546u64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_object_visible_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8B32ACE6326A7546u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8B32ACE6326A7546u64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _get_is_arena_prop_physics_disabled_safe(
        
        
            entity: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43C677F1E1158005u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43C677F1E1158005u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_is_arena_prop_physics_disabled_raw(
        entity: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x43C677F1E1158005u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x43C677F1E1158005u64;

        invoke_raw_typed!(
            hash,
                entity, 
                p1
        )
    }
}

/// An



pub fn is_point_in_angled_area_safe(
        
        
            xPos: 
        , 
        
        
            yPos: 
        , 
        
        
            zPos: 
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
        , 
        
        
            p10: 
        , 
        
        
            includez: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A70BAE8883E4C81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A70BAE8883E4C81u64;
        
        let result = invoke_raw!(
            hash,
                xPos, 
                yPos, 
                zPos, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width, 
                p10, 
                includez
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_point_in_angled_area_raw(
        xPos: , 
        yPos: , 
        zPos: , 
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        width: , 
        p10: , 
        includez: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A70BAE8883E4C81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A70BAE8883E4C81u64;

        invoke_raw_typed!(
            hash,
                xPos, 
                yPos, 
                zPos, 
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                width, 
                p10, 
                includez
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _0x2542269291c6ac84_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2542269291C6AC84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2542269291C6AC84u64;
        
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
pub fn _0x2542269291c6ac84_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2542269291C6AC84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2542269291C6AC84u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Pickup hashes can be found [here](https://gist.github.com/4mmonium/1eabfb6b3996e3aa6b9525a3eccf8a0b).



pub fn create_pickup_safe(
        
        
            pickupHash: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            p4: 
        , 
        
        
            value: 
        , 
        
        
            p6: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFBA08C503DD5FA58u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFBA08C503DD5FA58u64;
        
        let result = invoke_raw!(
            hash,
                pickupHash, 
                posX, 
                posY, 
                posZ, 
                p4, 
                value, 
                p6, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_pickup_raw(
        pickupHash: , 
        posX: , 
        posY: , 
        posZ: , 
        p4: , 
        value: , 
        p6: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFBA08C503DD5FA58u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFBA08C503DD5FA58u64;

        invoke_raw_typed!(
            hash,
                pickupHash, 
                posX, 
                posY, 
                posZ, 
                p4, 
                value, 
                p6, 
                modelHash
        )
    }
}

/// ```
Returns true if a destructible object with this handle exists, false otherwise.  
```



pub fn does_rayfire_map_object_exist_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52AF537A0C5B8AADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52AF537A0C5B8AADu64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_rayfire_map_object_exist_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x52AF537A0C5B8AADu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x52AF537A0C5B8AADu64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// Resets and brings back all the children of a fragment based object.
This should be used when attaching or detaching an object from another entity, especially when the object being detached consists of multiple fragments.

Attempting to teleport a fragment-based object using [`SET_ENTITY_COORDS`](#_0x06843DA7060A026B) such as a flag object, will result in it remaining in place and failing to teleport, given the condition mentioned in the preceding statement.

The native should be executed after detaching the object from its parent entity and before calling [`SET_ENTITY_COORDS`](#_0x06843DA7060A026B).

Example given down below.



pub fn fix_object_fragment_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9C1681347C8BD15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9C1681347C8BD15u64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn fix_object_fragment_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9C1681347C8BD15u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9C1681347C8BD15u64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// Search radius: 0.5



pub fn door_system_find_existing_door_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x589F80B325CC82C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x589F80B325CC82C5u64;
        
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
pub fn door_system_find_existing_door_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x589F80B325CC82C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x589F80B325CC82C5u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// Sets the 34th and 35th object flags related to player peds.

```
NativeDB Introduced: v3258
```



pub fn _set_object_targettable_by_player_safe(
        
        
            object: 
        , 
        
        
            setFlag34: 
        , 
        
        
            setFlag35: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB39F03368DB0CAA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB39F03368DB0CAA2u64;
        
        let result = invoke_raw!(
            hash,
                object, 
                setFlag34, 
                setFlag35
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_object_targettable_by_player_raw(
        object: , 
        setFlag34: , 
        setFlag35: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB39F03368DB0CAA2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB39F03368DB0CAA2u64;

        invoke_raw_typed!(
            hash,
                object, 
                setFlag34, 
                setFlag35
        )
    }
}

/// ## Parameters
*



pub fn has_closest_object_of_type_been_broken_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        , 
        
        
            modelHash: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x761B0E69AC4D007Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x761B0E69AC4D007Eu64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                modelHash, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_closest_object_of_type_been_broken_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        modelHash: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x761B0E69AC4D007Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x761B0E69AC4D007Eu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2, 
                p3, 
                modelHash, 
                p5
        )
    }
}

/// ```
Activate the physics to: "xs_prop_arena_{flipper,wall,bollard,turntable,pit}"
```

```
NativeDB Introduced: v1604
```



pub fn _set_enable_arena_prop_physics_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x911024442F4898F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x911024442F4898F0u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                toggle, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_enable_arena_prop_physics_raw(
        entity: , 
        toggle: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x911024442F4898F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x911024442F4898F0u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn _0x8caab2bd3ea58bd4_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8CAAB2BD3EA58BD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8CAAB2BD3EA58BD4u64;
        
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
pub fn _0x8caab2bd3ea58bd4_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8CAAB2BD3EA58BD4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8CAAB2BD3EA58BD4u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0x1c57c94a6446492a_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C57C94A6446492Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C57C94A6446492Au64;
        
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
pub fn _0x1c57c94a6446492a_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C57C94A6446492Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C57C94A6446492Au64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
NativeDB Introduced: v1365
```



pub fn _set_unk_global_bool_related_to_damage_safe(
        
        
            value: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xABDABF4E1EDECBFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xABDABF4E1EDECBFAu64;
        
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
pub fn _set_unk_global_bool_related_to_damage_raw(
        value: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xABDABF4E1EDECBFAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xABDABF4E1EDECBFAu64;

        invoke_raw_typed!(
            hash,
                value
        )
    }
}

/// ## Parameters
*



pub fn track_object_visibility_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB252BC036B525623u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB252BC036B525623u64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn track_object_visibility_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB252BC036B525623u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB252BC036B525623u64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ## Parameters
*



pub fn door_system_get_is_physics_loaded_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF97CDD4FC08FD34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF97CDD4FC08FD34u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn door_system_get_is_physics_loaded_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF97CDD4FC08FD34u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF97CDD4FC08FD34u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```c
enum eObjectPaintVariants
{  
	Pacific = 0,  
	Azure = 1,  
	Nautical = 2,  
	Continental = 3,  
	Battleship = 4,  
	Intrepid = 5,  
	Uniform = 6,  
	Classico = 7,  
	Mediterranean = 8,  
	Command = 9,  
	Mariner = 10,  
	Ruby = 11,  
	Vintage = 12,  
	Pristine = 13,  
	Merchant = 14,  
	Voyager = 15  
};  
```



pub fn _set_object_texture_variation_safe(
        
        
            object: 
        , 
        
        
            textureVariation: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x971DA0055324D033u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x971DA0055324D033u64;
        
        let result = invoke_raw!(
            hash,
                object, 
                textureVariation
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_object_texture_variation_raw(
        object: , 
        textureVariation: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x971DA0055324D033u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x971DA0055324D033u64;

        invoke_raw_typed!(
            hash,
                object, 
                textureVariation
        )
    }
}

/// ```
NativeDB Introduced: v1734
```



pub fn _0xafe24e4d29249e4a_safe(
        
        
            object: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFE24E4D29249E4Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFE24E4D29249E4Au64;
        
        let result = invoke_raw!(
            hash,
                object, 
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
pub fn _0xafe24e4d29249e4a_raw(
        object: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFE24E4D29249E4Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFE24E4D29249E4Au64;

        invoke_raw_typed!(
            hash,
                object, 
                p1, 
                p2, 
                p3
        )
    }
}

/// ## Parameters
*



pub fn prevent_collection_of_portable_pickup_safe(
        
        
            object: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92AEFB5F6E294023u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92AEFB5F6E294023u64;
        
        let result = invoke_raw!(
            hash,
                object, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn prevent_collection_of_portable_pickup_raw(
        object: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x92AEFB5F6E294023u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x92AEFB5F6E294023u64;

        invoke_raw_typed!(
            hash,
                object, 
                p1, 
                p2
        )
    }
}

/// Creates an object (prop) with the specified model centered at the specified position.
This object will initially be owned by the creating script as a mission entity, and the model should be loaded already (e.g. using REQUEST_MODEL).



pub fn create_object_no_offset_safe(
        
        
            modelHash: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            isNetwork: 
        , 
        
        
            netMissionEntity: 
        , 
        
        
            doorFlag: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A294B2138ABB884u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A294B2138ABB884u64;
        
        let result = invoke_raw!(
            hash,
                modelHash, 
                x, 
                y, 
                z, 
                isNetwork, 
                netMissionEntity, 
                doorFlag
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_object_no_offset_raw(
        modelHash: , 
        x: , 
        y: , 
        z: , 
        isNetwork: , 
        netMissionEntity: , 
        doorFlag: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9A294B2138ABB884u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9A294B2138ABB884u64;

        invoke_raw_typed!(
            hash,
                modelHash, 
                x, 
                y, 
                z, 
                isNetwork, 
                netMissionEntity, 
                doorFlag
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0x1a6cbb06e2d0d79d_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A6CBB06E2D0D79Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A6CBB06E2D0D79Du64;
        
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
pub fn _0x1a6cbb06e2d0d79d_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1A6CBB06E2D0D79Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1A6CBB06E2D0D79Du64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_activate_object_physics_as_soon_as_it_is_unfrozen_safe(
        
        
            object: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x406137F8EF90EAF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x406137F8EF90EAF5u64;
        
        let result = invoke_raw!(
            hash,
                object, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_activate_object_physics_as_soon_as_it_is_unfrozen_raw(
        object: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x406137F8EF90EAF5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x406137F8EF90EAF5u64;

        invoke_raw_typed!(
            hash,
                object, 
                toggle
        )
    }
}

/// CDoor and CDoorSystemData still internally allocated (and their associations between doorHash, modelHash, and coordinates).

Only its NetObj removed and flag ``*(v2 + 192) |= 8u`` (1604 retail) toggled.



pub fn remove_door_from_system_safe(
        
        
            doorHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x464D8E1427156FE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x464D8E1427156FE4u64;
        
        let result = invoke_raw!(
            hash,
                doorHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_door_from_system_raw(
        doorHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x464D8E1427156FE4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x464D8E1427156FE4u64;

        invoke_raw_typed!(
            hash,
                doorHash
        )
    }
}

/// Calculates the world coordinates after applying the specified offsets to the given position, relative to a certain heading.
This native is similar to [`GET_OFFSET_FROM_ENTITY_IN_WORLD_COORDS`](#_0x1899F328B0E12848), but uses a world position and heading as the reference point.



pub fn get_offset_from_coord_and_heading_in_world_coords_safe(
        
        
            xPos: 
        , 
        
        
            yPos: 
        , 
        
        
            zPos: 
        , 
        
        
            heading: 
        , 
        
        
            xOffset: 
        , 
        
        
            yOffset: 
        , 
        
        
            zOffset: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x163E252DE035A133u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x163E252DE035A133u64;
        
        let result = invoke_raw!(
            hash,
                xPos, 
                yPos, 
                zPos, 
                heading, 
                xOffset, 
                yOffset, 
                zOffset
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_offset_from_coord_and_heading_in_world_coords_raw(
        xPos: , 
        yPos: , 
        zPos: , 
        heading: , 
        xOffset: , 
        yOffset: , 
        zOffset: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x163E252DE035A133u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x163E252DE035A133u64;

        invoke_raw_typed!(
            hash,
                xPos, 
                yPos, 
                zPos, 
                heading, 
                xOffset, 
                yOffset, 
                zOffset
        )
    }
}

/// Casts a ray downward from the object's position and places the object on the surface it hits (including world surface and objects). Use [`PLACE_OBJECT_ON_GROUND_PROPERLY`](#_0x58A850EAEE20FAA3) to not include objects when determining the surface.



pub fn place_object_on_ground_or_object_properly_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD76EEEF746057FD6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD76EEEF746057FD6u64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn place_object_on_ground_or_object_properly_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD76EEEF746057FD6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD76EEEF746057FD6u64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _0x8dca505a5c196f05_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DCA505A5C196F05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DCA505A5C196F05u64;
        
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
pub fn _0x8dca505a5c196f05_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DCA505A5C196F05u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DCA505A5C196F05u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Sets the intensity of Speed Boost and Slow Down props.

The corresponding values for Speed Boosts in the Creator are:  
Weak: `15`  
Normal: `25`  
Strong: `35`  
Extra Strong: `45`  
Ultra Strong: `100`

For Slow Downs:  
Weak: `44`  
Normal: `30`  
Strong: `16`



pub fn _set_object_stunt_prop_speedup_safe(
        
        
            object: 
        , 
        
        
            intensity: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96EE0EBA0163DF80u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96EE0EBA0163DF80u64;
        
        let result = invoke_raw!(
            hash,
                object, 
                intensity
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_object_stunt_prop_speedup_raw(
        object: , 
        intensity: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x96EE0EBA0163DF80u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x96EE0EBA0163DF80u64;

        invoke_raw_typed!(
            hash,
                object, 
                intensity
        )
    }
}

/// ## Parameters
*



pub fn remove_object_high_detail_model_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A39DB43E47CF3AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A39DB43E47CF3AAu64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_object_high_detail_model_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4A39DB43E47CF3AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4A39DB43E47CF3AAu64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ## Parameters
*



pub fn set_pickup_generation_range_multiplier_safe(
        
        
            multiplier: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x318516E02DE3ECE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x318516E02DE3ECE2u64;
        
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
pub fn set_pickup_generation_range_multiplier_raw(
        multiplier: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x318516E02DE3ECE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x318516E02DE3ECE2u64;

        invoke_raw_typed!(
            hash,
                multiplier
        )
    }
}

/// ## Parameters
*



pub fn is_any_object_near_point_safe(
        
        
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
        let hash = 0x397DC58FF00298D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x397DC58FF00298D1u64;
        
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
pub fn is_any_object_near_point_raw(
        x: , 
        y: , 
        z: , 
        range: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x397DC58FF00298D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x397DC58FF00298D1u64;

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



pub fn _0x762db2d380b48d04_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x762DB2D380B48D04u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x762DB2D380B48D04u64;
        
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
pub fn _0x762db2d380b48d04_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x762DB2D380B48D04u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x762DB2D380B48D04u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_force_object_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF538081986E49E9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF538081986E49E9Du64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_force_object_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF538081986E49E9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF538081986E49E9Du64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// See [`N_0x701fda1e82076ba4`](#_0x701FDA1E82076BA4).



pub fn _0xc7f29ca00f46350e_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7F29CA00F46350Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7F29CA00F46350Eu64;
        
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
pub fn _0xc7f29ca00f46350e_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7F29CA00F46350Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7F29CA00F46350Eu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn has_pickup_been_collected_safe(
        
        
            pickup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80EC48E6679313F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80EC48E6679313F9u64;
        
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
pub fn has_pickup_been_collected_raw(
        pickup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x80EC48E6679313F9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x80EC48E6679313F9u64;

        invoke_raw_typed!(
            hash,
                pickup
        )
    }
}

/// ```
returns pickup hash.
```



pub fn _get_pickup_hash_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EAAD83F8CFB4575u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EAAD83F8CFB4575u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_pickup_hash_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EAAD83F8CFB4575u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EAAD83F8CFB4575u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Pickup hashes can be found [here](https://gist.github.com/4mmonium/1eabfb6b3996e3aa6b9525a3eccf8a0b).



pub fn remove_all_pickups_of_type_safe(
        
        
            pickupHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27F9D613092159CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27F9D613092159CFu64;
        
        let result = invoke_raw!(
            hash,
                pickupHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_all_pickups_of_type_raw(
        pickupHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27F9D613092159CFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27F9D613092159CFu64;

        invoke_raw_typed!(
            hash,
                pickupHash
        )
    }
}

/// ## Parameters
*



pub fn enable_saving_in_garage_safe(
        
        
            garageHash: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2E1A7133DD356A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2E1A7133DD356A6u64;
        
        let result = invoke_raw!(
            hash,
                garageHash, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn enable_saving_in_garage_raw(
        garageHash: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF2E1A7133DD356A6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF2E1A7133DD356A6u64;

        invoke_raw_typed!(
            hash,
                garageHash, 
                toggle
        )
    }
}

/// ```
Hardcoded to not work in multiplayer.  
Used to lock/unlock doors to interior areas of the game.  
(Possible) Door Types:  
pastebin.com/9S2m3qA4  
Heading is either 1, 0 or -1 in the scripts. Means default closed(0) or opened either into(1) or out(-1) of the interior.  
Locked means that the heading is locked.    
p6 is always 0.   
225 door types, model names and coords found in stripclub.c4:  
pastebin.com/gywnbzsH  
get door info: pastebin.com/i14rbekD  
```



pub fn set_state_of_closest_door_of_type_safe(
        
        
            type: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            locked: 
        , 
        
        
            heading: 
        , 
        
        
            p6: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF82D8F1926A02C3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF82D8F1926A02C3Du64;
        
        let result = invoke_raw!(
            hash,
                type, 
                x, 
                y, 
                z, 
                locked, 
                heading, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_state_of_closest_door_of_type_raw(
        type: , 
        x: , 
        y: , 
        z: , 
        locked: , 
        heading: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF82D8F1926A02C3Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF82D8F1926A02C3Du64;

        invoke_raw_typed!(
            hash,
                type, 
                x, 
                y, 
                z, 
                locked, 
                heading, 
                p6
        )
    }
}

/// ## Parameters
*



pub fn is_pickup_weapon_object_valid_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11D1E53A726891FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11D1E53A726891FEu64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_pickup_weapon_object_valid_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x11D1E53A726891FEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x11D1E53A726891FEu64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _0xe05f6aeefeb0bb02_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE05F6AEEFEB0BB02u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE05F6AEEFEB0BB02u64;
        
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
pub fn _0xe05f6aeefeb0bb02_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE05F6AEEFEB0BB02u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE05F6AEEFEB0BB02u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
NativeDB Introduced: v757
```



pub fn _0x8cff648fbd7330f1_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8CFF648FBD7330F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8CFF648FBD7330F1u64;
        
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
pub fn _0x8cff648fbd7330f1_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8CFF648FBD7330F1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8CFF648FBD7330F1u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _door_system_get_automatic_distance_safe(
        
        
            doorHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE851471AEFC3374Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE851471AEFC3374Fu64;
        
        let result = invoke_raw!(
            hash,
                doorHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _door_system_get_automatic_distance_raw(
        doorHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE851471AEFC3374Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE851471AEFC3374Fu64;

        invoke_raw_typed!(
            hash,
                doorHash
        )
    }
}

/// A*

```
NativeDB Introduced: v1734
```



pub fn _0xfdc07c58e8aab715_safe(
        
        
            pickupHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDC07C58E8AAB715u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDC07C58E8AAB715u64;
        
        let result = invoke_raw!(
            hash,
                pickupHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xfdc07c58e8aab715_raw(
        pickupHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xFDC07C58E8AAB715u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xFDC07C58E8AAB715u64;

        invoke_raw_typed!(
            hash,
                pickupHash
        )
    }
}

/// ## Parameters
*



pub fn detach_portable_pickup_from_ped_safe(
        
        
            pickupObject: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF463D1E9A0AECB1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF463D1E9A0AECB1u64;
        
        let result = invoke_raw!(
            hash,
                pickupObject
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn detach_portable_pickup_from_ped_raw(
        pickupObject: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCF463D1E9A0AECB1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCF463D1E9A0AECB1u64;

        invoke_raw_typed!(
            hash,
                pickupObject
        )
    }
}

/// ## Parameters
*



pub fn _0xeb6f1a9b5510a5d2_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB6F1A9B5510A5D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB6F1A9B5510A5D2u64;
        
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
pub fn _0xeb6f1a9b5510a5d2_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEB6F1A9B5510A5D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEB6F1A9B5510A5D2u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Pickup hashes can be found [here](https://gist.github.com/4mmonium/1eabfb6b3996e3aa6b9525a3eccf8a0b).



pub fn does_pickup_of_type_exist_in_area_safe(
        
        
            pickupHash: 
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
        let hash = 0xF9C36251F6E48E33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9C36251F6E48E33u64;
        
        let result = invoke_raw!(
            hash,
                pickupHash, 
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
pub fn does_pickup_of_type_exist_in_area_raw(
        pickupHash: , 
        x: , 
        y: , 
        z: , 
        radius: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF9C36251F6E48E33u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF9C36251F6E48E33u64;

        invoke_raw_typed!(
            hash,
                pickupHash, 
                x, 
                y, 
                z, 
                radius
        )
    }
}

/// ```
rage::phArchetypeDamp
p9: Some phBoundGeometry margin value, limited to (0.0, 0.1) exclusive.
```



pub fn set_object_physics_params_safe(
        
        
            object: 
        , 
        
        
            mass: 
        , 
        
        
            gravityFactor: 
        , 
        
        
            linearC: 
        , 
        
        
            linearV: 
        , 
        
        
            linearV2: 
        , 
        
        
            angularC: 
        , 
        
        
            angularV: 
        , 
        
        
            angularV2: 
        , 
        
        
            p9: 
        , 
        
        
            maxAngSpeed: 
        , 
        
        
            buoyancyFactor: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6DF6E90DE7DF90Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6DF6E90DE7DF90Fu64;
        
        let result = invoke_raw!(
            hash,
                object, 
                mass, 
                gravityFactor, 
                linearC, 
                linearV, 
                linearV2, 
                angularC, 
                angularV, 
                angularV2, 
                p9, 
                maxAngSpeed, 
                buoyancyFactor
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_object_physics_params_raw(
        object: , 
        mass: , 
        gravityFactor: , 
        linearC: , 
        linearV: , 
        linearV2: , 
        angularC: , 
        angularV: , 
        angularV2: , 
        p9: , 
        maxAngSpeed: , 
        buoyancyFactor: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF6DF6E90DE7DF90Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF6DF6E90DE7DF90Fu64;

        invoke_raw_typed!(
            hash,
                object, 
                mass, 
                gravityFactor, 
                linearC, 
                linearV, 
                linearV2, 
                angularC, 
                angularV, 
                angularV2, 
                p9, 
                maxAngSpeed, 
                buoyancyFactor
        )
    }
}

/// ## Parameters
*



pub fn is_object_partially_inside_garage_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0EED5A6BC7B237Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0EED5A6BC7B237Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_object_partially_inside_garage_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF0EED5A6BC7B237Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF0EED5A6BC7B237Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Requires a component_at_*_flsh to be attached to the weapon object
```



pub fn _set_create_weapon_object_light_source_safe(
        
        
            object: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCE595371A5FBAAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCE595371A5FBAAFu64;
        
        let result = invoke_raw!(
            hash,
                object, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_create_weapon_object_light_source_raw(
        object: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBCE595371A5FBAAFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBCE595371A5FBAAFu64;

        invoke_raw_typed!(
            hash,
                object, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _0xdb41d07a45a6d4b7_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB41D07A45A6D4B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB41D07A45A6D4B7u64;
        
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
pub fn _0xdb41d07a45a6d4b7_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDB41D07A45A6D4B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDB41D07A45A6D4B7u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
p5 is usually 0.  
```



pub fn does_object_of_type_exist_at_coords_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            hash: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFA48E2FF417213Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFA48E2FF417213Fu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                hash, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_object_of_type_exist_at_coords_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        hash: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBFA48E2FF417213Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBFA48E2FF417213Fu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                hash, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn _clear_garage_area_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA05194260CDCDF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA05194260CDCDF9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _clear_garage_area_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDA05194260CDCDF9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDA05194260CDCDF9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Creates an object (prop) with the specified model at the specified position, offset on the Z axis by the radius of the object's model.
This object will initially be owned by the creating script as a mission entity, and the model should be loaded already (e.g. using REQUEST_MODEL).



pub fn create_object_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x509D5878EB39E842u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x509D5878EB39E842u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_object_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x509D5878EB39E842u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x509D5878EB39E842u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _create_non_networked_ambient_pickup_safe(
        
        
            pickupHash: 
        , 
        
        
            posX: 
        , 
        
        
            posY: 
        , 
        
        
            posZ: 
        , 
        
        
            flags: 
        , 
        
        
            value: 
        , 
        
        
            modelHash: 
        , 
        
        
            p7: 
        , 
        
        
            p8: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C93764223E29C50u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C93764223E29C50u64;
        
        let result = invoke_raw!(
            hash,
                pickupHash, 
                posX, 
                posY, 
                posZ, 
                flags, 
                value, 
                modelHash, 
                p7, 
                p8
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _create_non_networked_ambient_pickup_raw(
        pickupHash: , 
        posX: , 
        posY: , 
        posZ: , 
        flags: , 
        value: , 
        modelHash: , 
        p7: , 
        p8: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9C93764223E29C50u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9C93764223E29C50u64;

        invoke_raw_typed!(
            hash,
                pickupHash, 
                posX, 
                posY, 
                posZ, 
                flags, 
                value, 
                modelHash, 
                p7, 
                p8
        )
    }
}

/// SET_PICKUP_*

```
NativeDB Introduced: v1734
```



pub fn _0x7813e8b8c4ae4799_safe(
        
        
            pickup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7813E8B8C4AE4799u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7813E8B8C4AE4799u64;
        
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
pub fn _0x7813e8b8c4ae4799_raw(
        pickup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7813E8B8C4AE4799u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7813E8B8C4AE4799u64;

        invoke_raw_typed!(
            hash,
                pickup
        )
    }
}

/// ## Parameters
*



pub fn create_non_networked_portable_pickup_safe(
        
        
            pickupHash: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            placeOnGround: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x125494B98A21AAF7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x125494B98A21AAF7u64;
        
        let result = invoke_raw!(
            hash,
                pickupHash, 
                x, 
                y, 
                z, 
                placeOnGround, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_non_networked_portable_pickup_raw(
        pickupHash: , 
        x: , 
        y: , 
        z: , 
        placeOnGround: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x125494B98A21AAF7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x125494B98A21AAF7u64;

        invoke_raw_typed!(
            hash,
                pickupHash, 
                x, 
                y, 
                z, 
                placeOnGround, 
                modelHash
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x659f9d71f52843f8_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x659F9D71F52843F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x659F9D71F52843F8u64;
        
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
pub fn _0x659f9d71f52843f8_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x659F9D71F52843F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x659F9D71F52843F8u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x62454a641b41f3c5_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62454A641B41F3C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62454A641B41F3C5u64;
        
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
pub fn _0x62454a641b41f3c5_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x62454A641B41F3C5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x62454A641B41F3C5u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
draws circular marker at pos
-1 = none
0 = red
1 = green
2 = blue
3 = green larger
4 = nothing
5 = green small
```



pub fn render_fake_pickup_glow_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            colorIndex: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3430676B11CDF21Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3430676B11CDF21Du64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                colorIndex
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn render_fake_pickup_glow_raw(
        x: , 
        y: , 
        z: , 
        colorIndex: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3430676B11CDF21Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3430676B11CDF21Du64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                colorIndex
        )
    }
}

/// ```
NativeDB Introduced: v2189
```



pub fn _0x31574b1b41268673_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31574B1B41268673u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31574B1B41268673u64;
        
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
pub fn _0x31574b1b41268673_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31574B1B41268673u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31574B1B41268673u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn break_object_fragment_child_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7E4C198B0185900u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7E4C198B0185900u64;
        
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
pub fn break_object_fragment_child_raw(
        p0: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE7E4C198B0185900u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE7E4C198B0185900u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1, 
                p2
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _0xb5b7742424bd4445_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5B7742424BD4445u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5B7742424BD4445u64;
        
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
pub fn _0xb5b7742424bd4445_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB5B7742424BD4445u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB5B7742424BD4445u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x641f272b52e2f0f8_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x641F272B52E2F0F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x641F272B52E2F0F8u64;
        
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
pub fn _0x641f272b52e2f0f8_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x641F272B52E2F0F8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x641F272B52E2F0F8u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Deletes the specified object.



pub fn delete_object_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x539E0AE3E6634B9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x539E0AE3E6634B9Fu64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn delete_object_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x539E0AE3E6634B9Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x539E0AE3E6634B9Fu64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ## Parameters
*



pub fn set_pickup_regeneration_time_safe(
        
        
            pickup: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78015C9B4B3ECC9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78015C9B4B3ECC9Du64;
        
        let result = invoke_raw!(
            hash,
                pickup, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_pickup_regeneration_time_raw(
        pickup: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x78015C9B4B3ECC9Du32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x78015C9B4B3ECC9Du64;

        invoke_raw_typed!(
            hash,
                pickup, 
                duration
        )
    }
}

/// ## Parameters
*



pub fn door_system_get_door_pending_state_safe(
        
        
            doorHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4BC2854478F3A749u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4BC2854478F3A749u64;
        
        let result = invoke_raw!(
            hash,
                doorHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn door_system_get_door_pending_state_raw(
        doorHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4BC2854478F3A749u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4BC2854478F3A749u64;

        invoke_raw_typed!(
            hash,
                doorHash
        )
    }
}

/// ```
NativeDB Introduced: v1868
```



pub fn _0x006e4b040ed37ec3_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x006E4B040ED37EC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x006E4B040ED37EC3u64;
        
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
pub fn _0x006e4b040ed37ec3_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x006E4B040ED37EC3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x006E4B040ED37EC3u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn set_team_pickup_object_safe(
        
        
            object: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53E0DF1A2A3CF0CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53E0DF1A2A3CF0CAu64;
        
        let result = invoke_raw!(
            hash,
                object, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_team_pickup_object_raw(
        object: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x53E0DF1A2A3CF0CAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x53E0DF1A2A3CF0CAu64;

        invoke_raw_typed!(
            hash,
                object, 
                p1, 
                p2
        )
    }
}

/// ## Parameters
*



pub fn door_system_get_door_state_safe(
        
        
            doorHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x160AA1B32F6139B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x160AA1B32F6139B8u64;
        
        let result = invoke_raw!(
            hash,
                doorHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn door_system_get_door_state_raw(
        doorHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x160AA1B32F6139B8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x160AA1B32F6139B8u64;

        invoke_raw_typed!(
            hash,
                doorHash
        )
    }
}

/// ## Parameters
*



pub fn is_player_entirely_inside_garage_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x024A60DEB0EA69F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x024A60DEB0EA69F0u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_player_entirely_inside_garage_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x024A60DEB0EA69F0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x024A60DEB0EA69F0u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Has 8 params in the latest patches.  
isMission - if true doesn't return mission objects  
```



pub fn get_closest_object_of_type_safe(
        
        
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
        
        
            isMission: 
        , 
        
        
            p6: 
        , 
        
        
            p7: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE143FA2249364369u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE143FA2249364369u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                modelHash, 
                isMission, 
                p6, 
                p7
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_closest_object_of_type_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        modelHash: , 
        isMission: , 
        p6: , 
        p7: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE143FA2249364369u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE143FA2249364369u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                modelHash, 
                isMission, 
                p6, 
                p7
        )
    }
}

/// ## Parameters
*



pub fn is_object_near_point_safe(
        
        
            objectHash: 
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
        let hash = 0x8C90FE4B381BA60Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C90FE4B381BA60Au64;
        
        let result = invoke_raw!(
            hash,
                objectHash, 
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
pub fn is_object_near_point_raw(
        objectHash: , 
        x: , 
        y: , 
        z: , 
        range: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8C90FE4B381BA60Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8C90FE4B381BA60Au64;

        invoke_raw_typed!(
            hash,
                objectHash, 
                x, 
                y, 
                z, 
                range
        )
    }
}

/// ```
CLEAR_*
```



pub fn _0xa2c1f5e92afe49ed_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2C1F5E92AFE49EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2C1F5E92AFE49EDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa2c1f5e92afe49ed_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA2C1F5E92AFE49EDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA2C1F5E92AFE49EDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v1180
```



pub fn _0xd05a3241b9a86f19_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD05A3241B9A86F19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD05A3241B9A86F19u64;
        
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
pub fn _0xd05a3241b9a86f19_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD05A3241B9A86F19u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD05A3241B9A86F19u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn get_coords_and_rotation_of_closest_object_of_type_safe(
        
        
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
        
        
            outPosition: 
        , 
        
        
            outRotation: 
        , 
        
        
            rotationOrder: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x163F8B586BC95F2Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x163F8B586BC95F2Au64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                modelHash, 
                outPosition, 
                outRotation, 
                rotationOrder
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_coords_and_rotation_of_closest_object_of_type_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        modelHash: , 
        outPosition: , 
        outRotation: , 
        rotationOrder: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x163F8B586BC95F2Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x163F8B586BC95F2Au64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                modelHash, 
                outPosition, 
                outRotation, 
                rotationOrder
        )
    }
}

/// ## Parameters
*



pub fn get_safe_pickup_coords_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E16BC2503FF1FF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E16BC2503FF1FF0u64;
        
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
pub fn get_safe_pickup_coords_raw(
        x: , 
        y: , 
        z: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E16BC2503FF1FF0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E16BC2503FF1FF0u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z
        )
    }
}

/// Includes networking check: ownership vs. or the door itself



pub fn door_system_set_spring_removed_safe(
        
        
            doorHash: 
        , 
        
        
            removed: 
        , 
        
        
            requestDoor: 
        , 
        
        
            forceUpdate: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC485E07E4F0B7958u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC485E07E4F0B7958u64;
        
        let result = invoke_raw!(
            hash,
                doorHash, 
                removed, 
                requestDoor, 
                forceUpdate
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn door_system_set_spring_removed_raw(
        doorHash: , 
        removed: , 
        requestDoor: , 
        forceUpdate: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC485E07E4F0B7958u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC485E07E4F0B7958u64;

        invoke_raw_typed!(
            hash,
                doorHash, 
                removed, 
                requestDoor, 
                forceUpdate
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _get_pickup_hash_from_weapon_safe(
        
        
            weapon: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6429A016084F1A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6429A016084F1A5u64;
        
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
pub fn _get_pickup_hash_from_weapon_raw(
        weapon: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD6429A016084F1A5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD6429A016084F1A5u64;

        invoke_raw_typed!(
            hash,
                weapon
        )
    }
}

/// ```
Overrides the climbing/blocking flags of the object, used in the native scripts mostly for "prop_dock_bouy_*"
```



pub fn set_object_allow_low_lod_buoyancy_safe(
        
        
            object: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D89D607CB3DD1D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D89D607CB3DD1D2u64;
        
        let result = invoke_raw!(
            hash,
                object, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_object_allow_low_lod_buoyancy_raw(
        object: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D89D607CB3DD1D2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D89D607CB3DD1D2u64;

        invoke_raw_typed!(
            hash,
                object, 
                toggle
        )
    }
}

/// _0x66A49D021870FE88 native function



pub fn _0x66a49d021870fe88_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66A49D021870FE88u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66A49D021870FE88u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x66a49d021870fe88_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x66A49D021870FE88u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x66A49D021870FE88u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _set_pickup_hidden_when_uncollectable_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3ED2B83AB2E82799u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3ED2B83AB2E82799u64;
        
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
pub fn _set_pickup_hidden_when_uncollectable_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3ED2B83AB2E82799u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3ED2B83AB2E82799u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _get_object_texture_variation_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE84EB93729C5F36Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE84EB93729C5F36Au64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_object_texture_variation_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE84EB93729C5F36Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE84EB93729C5F36Au64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ## Parameters
*



pub fn place_object_on_ground_properly_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58A850EAEE20FAA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58A850EAEE20FAA3u64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn place_object_on_ground_properly_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x58A850EAEE20FAA3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x58A850EAEE20FAA3u64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ```
Returns true if the object has finished moving.  
If false, moves the object towards the specified X, Y and Z coordinates with the specified X, Y and Z speed.  
See also: https://gtagmodding.com/opcode-database/opcode/034E/
Has to be looped until it returns true.   
```



pub fn slide_object_safe(
        
        
            object: 
        , 
        
        
            toX: 
        , 
        
        
            toY: 
        , 
        
        
            toZ: 
        , 
        
        
            speedX: 
        , 
        
        
            speedY: 
        , 
        
        
            speedZ: 
        , 
        
        
            collision: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FDFF4107B8C1147u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FDFF4107B8C1147u64;
        
        let result = invoke_raw!(
            hash,
                object, 
                toX, 
                toY, 
                toZ, 
                speedX, 
                speedY, 
                speedZ, 
                collision
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn slide_object_raw(
        object: , 
        toX: , 
        toY: , 
        toZ: , 
        speedX: , 
        speedY: , 
        speedZ: , 
        collision: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2FDFF4107B8C1147u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2FDFF4107B8C1147u64;

        invoke_raw_typed!(
            hash,
                object, 
                toX, 
                toY, 
                toZ, 
                speedX, 
                speedY, 
                speedZ, 
                collision
        )
    }
}

/// ```
NativeDB Introduced: v1604
```



pub fn _set_enable_arena_prop_physics_on_ped_safe(
        
        
            entity: 
        , 
        
        
            toggle: 
        , 
        
        
            p2: 
        , 
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB20834A7DD3D8896u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB20834A7DD3D8896u64;
        
        let result = invoke_raw!(
            hash,
                entity, 
                toggle, 
                p2, 
                ped
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_enable_arena_prop_physics_on_ped_raw(
        entity: , 
        toggle: , 
        p2: , 
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB20834A7DD3D8896u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB20834A7DD3D8896u64;

        invoke_raw_typed!(
            hash,
                entity, 
                toggle, 
                p2, 
                ped
        )
    }
}

/// ## Parameters
*



pub fn attach_portable_pickup_to_ped_safe(
        
        
            pickupObject: 
        , 
        
        
            ped: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DC39368BDD57755u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DC39368BDD57755u64;
        
        let result = invoke_raw!(
            hash,
                pickupObject, 
                ped
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn attach_portable_pickup_to_ped_raw(
        pickupObject: , 
        ped: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8DC39368BDD57755u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8DC39368BDD57755u64;

        invoke_raw_typed!(
            hash,
                pickupObject, 
                ped
        )
    }
}

/// ## Parameters
*



pub fn hide_portable_pickup_when_detached_safe(
        
        
            pickup: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x867458251D47CCB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x867458251D47CCB2u64;
        
        let result = invoke_raw!(
            hash,
                pickup, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn hide_portable_pickup_when_detached_raw(
        pickup: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x867458251D47CCB2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x867458251D47CCB2u64;

        invoke_raw_typed!(
            hash,
                pickup, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn is_any_entity_entirely_inside_garage_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x673ED815D6E323B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x673ED815D6E323B7u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_any_entity_entirely_inside_garage_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x673ED815D6E323B7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x673ED815D6E323B7u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xc6033d32241f6fb5_safe(
        
        
            object: 
        , 
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6033D32241F6FB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6033D32241F6FB5u64;
        
        let result = invoke_raw!(
            hash,
                object, 
                toggle
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xc6033d32241f6fb5_raw(
        object: , 
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC6033D32241F6FB5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC6033D32241F6FB5u64;

        invoke_raw_typed!(
            hash,
                object, 
                toggle
        )
    }
}

/// ## Parameters
*



pub fn _force_pickup_regenerate_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x758A5C1B3B1E1990u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x758A5C1B3B1E1990u64;
        
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
pub fn _force_pickup_regenerate_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x758A5C1B3B1E1990u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x758A5C1B3B1E1990u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn does_pickup_exist_safe(
        
        
            pickup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFC1CA75AD4074D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFC1CA75AD4074D1u64;
        
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
pub fn does_pickup_exist_raw(
        pickup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xAFC1CA75AD4074D1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xAFC1CA75AD4074D1u64;

        invoke_raw_typed!(
            hash,
                pickup
        )
    }
}

/// ## Parameters
*



pub fn _0x0596843b34b95ce5_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0596843B34B95CE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0596843B34B95CE5u64;
        
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
pub fn _0x0596843b34b95ce5_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0596843B34B95CE5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0596843B34B95CE5u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// Some property related to gates. Native name between ``DOOR_SYSTEM_SET_AUTOMATIC_RATE`` and ``DOOR_SYSTEM_SET_DOOR_STATE``.



pub fn _0xa85a21582451e951_safe(
        
        
            doorHash: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA85A21582451E951u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA85A21582451E951u64;
        
        let result = invoke_raw!(
            hash,
                doorHash, 
                p1
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa85a21582451e951_raw(
        doorHash: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA85A21582451E951u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA85A21582451E951u64;

        invoke_raw_typed!(
            hash,
                doorHash, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _set_object_stunt_prop_duration_safe(
        
        
            object: 
        , 
        
        
            duration: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF6CA0330F2E737Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF6CA0330F2E737Bu64;
        
        let result = invoke_raw!(
            hash,
                object, 
                duration
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _set_object_stunt_prop_duration_raw(
        object: , 
        duration: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDF6CA0330F2E737Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDF6CA0330F2E737Bu64;

        invoke_raw_typed!(
            hash,
                object, 
                duration
        )
    }
}

/// ## Parameters
*



pub fn is_object_entirely_inside_garage_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x372EF6699146A1E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x372EF6699146A1E4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_object_entirely_inside_garage_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x372EF6699146A1E4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x372EF6699146A1E4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Clears all areas created by 0xD4A7A435B3710D05

CLEAR_*

NativeDB Introduced: v1290
```



pub fn _0xb7c6d80fb371659a_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7C6D80FB371659Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7C6D80FB371659Au64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xb7c6d80fb371659a_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB7C6D80FB371659Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB7C6D80FB371659Au64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
NativeDB Introduced: v2372
```



pub fn _0x27f248c3febfaad3_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27F248C3FEBFAAD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27F248C3FEBFAAD3u64;
        
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
pub fn _0x27f248c3febfaad3_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x27F248C3FEBFAAD3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x27F248C3FEBFAAD3u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn _0x31f924b53eaddf65_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31F924B53EADDF65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31F924B53EADDF65u64;
        
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
pub fn _0x31f924b53eaddf65_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x31F924B53EADDF65u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x31F924B53EADDF65u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
is this like setting is as no longer needed?  
```



pub fn _mark_object_for_deletion_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADBE4809F19F927Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADBE4809F19F927Au64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _mark_object_for_deletion_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xADBE4809F19F927Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xADBE4809F19F927Au64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ## Parameters
*



pub fn door_system_get_open_ratio_safe(
        
        
            doorHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65499865FCA6E5ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65499865FCA6E5ECu64;
        
        let result = invoke_raw!(
            hash,
                doorHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn door_system_get_open_ratio_raw(
        doorHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x65499865FCA6E5ECu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x65499865FCA6E5ECu64;

        invoke_raw_typed!(
            hash,
                doorHash
        )
    }
}

/// ## Parameters
*



pub fn is_player_partially_inside_garage_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1761DC5D8471CBAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1761DC5D8471CBAAu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_player_partially_inside_garage_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1761DC5D8471CBAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1761DC5D8471CBAAu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// See [`SET_STATE_OF_RAYFIRE_MAP_OBJECT`](#_0x5C29F698D404C5E1) to see the different states

Get a destructible object's state. Substract 1 to get the real state. For example, if the object just spawned (state 2), the native will return 3.



pub fn get_state_of_rayfire_map_object_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x899BA936634A322Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x899BA936634A322Eu64;
        
        let result = invoke_raw!(
            hash,
                object
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_state_of_rayfire_map_object_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x899BA936634A322Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x899BA936634A322Eu64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ## Parameters
*



pub fn _set_pickup_uncollectable_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C1B69FAE509BA97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C1B69FAE509BA97u64;
        
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
pub fn _set_pickup_uncollectable_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C1B69FAE509BA97u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C1B69FAE509BA97u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ```
locked is 0 if no door is found  
locked is 0 if door is unlocked  
locked is 1 if door is found and unlocked.



pub fn get_state_of_closest_door_of_type_safe(
        
        
            type: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            locked: 
        , 
        
        
            heading: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDC1A5B84AEF33FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDC1A5B84AEF33FFu64;
        
        let result = invoke_raw!(
            hash,
                type, 
                x, 
                y, 
                z, 
                locked, 
                heading
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_state_of_closest_door_of_type_raw(
        type: , 
        x: , 
        y: , 
        z: , 
        locked: , 
        heading: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEDC1A5B84AEF33FFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEDC1A5B84AEF33FFu64;

        invoke_raw_typed!(
            hash,
                type, 
                x, 
                y, 
                z, 
                locked, 
                heading
        )
    }
}

/// Pickup hashes can be found [here](https://gist.github.com/4mmonium/1eabfb6b3996e3aa6b9525a3eccf8a0b).



pub fn create_portable_pickup_safe(
        
        
            pickupHash: 
        , 
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            placeOnGround: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2EAF1FDB2FB55698u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2EAF1FDB2FB55698u64;
        
        let result = invoke_raw!(
            hash,
                pickupHash, 
                x, 
                y, 
                z, 
                placeOnGround, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_portable_pickup_raw(
        pickupHash: , 
        x: , 
        y: , 
        z: , 
        placeOnGround: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2EAF1FDB2FB55698u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2EAF1FDB2FB55698u64;

        invoke_raw_typed!(
            hash,
                pickupHash, 
                x, 
                y, 
                z, 
                placeOnGround, 
                modelHash
        )
    }
}

/// ## Parameters
*



pub fn _0x1e3f1b1b891a2aaa_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E3F1B1B891A2AAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E3F1B1B891A2AAAu64;
        
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
pub fn _0x1e3f1b1b891a2aaa_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1E3F1B1B891A2AAAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1E3F1B1B891A2AAAu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn are_entities_entirely_inside_garage_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85B6C850546FDDE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85B6C850546FDDE2u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn are_entities_entirely_inside_garage_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x85B6C850546FDDE2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x85B6C850546FDDE2u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x858ec9fd25de04aa_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x858EC9FD25DE04AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x858EC9FD25DE04AAu64;
        
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
pub fn _0x858ec9fd25de04aa_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x858EC9FD25DE04AAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x858EC9FD25DE04AAu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn set_object_targettable_safe(
        
        
            object: 
        , 
        
        
            targettable: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A7391690F5AFD81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A7391690F5AFD81u64;
        
        let result = invoke_raw!(
            hash,
                object, 
                targettable
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_object_targettable_raw(
        object: , 
        targettable: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8A7391690F5AFD81u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8A7391690F5AFD81u64;

        invoke_raw_typed!(
            hash,
                object, 
                targettable
        )
    }
}

/// ## Parameters
*



pub fn get_pickup_object_safe(
        
        
            pickup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5099BC55630B25AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5099BC55630B25AEu64;
        
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
pub fn get_pickup_object_raw(
        pickup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5099BC55630B25AEu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5099BC55630B25AEu64;

        invoke_raw_typed!(
            hash,
                pickup
        )
    }
}

/// ## Parameters
*



pub fn get_pickup_coords_safe(
        
        
            pickup: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x225B8B35C88029B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x225B8B35C88029B3u64;
        
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
pub fn get_pickup_coords_raw(
        pickup: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x225B8B35C88029B3u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x225B8B35C88029B3u64;

        invoke_raw_typed!(
            hash,
                pickup
        )
    }
}

/// ```
Example:
OBJECT::GET_RAYFIRE_MAP_OBJECT(-809.9619750976562, 170.919, 75.7406997680664, 3.0, "des_tvsmash");
```



pub fn get_rayfire_map_object_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            radius: 
        , 
        
        
            name: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB48FCED898292E52u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB48FCED898292E52u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                name
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_rayfire_map_object_raw(
        x: , 
        y: , 
        z: , 
        radius: , 
        name: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB48FCED898292E52u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB48FCED898292E52u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                radius, 
                name
        )
    }
}

/// ## Parameters
*



pub fn _0xf92099527db8e2a7_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF92099527DB8E2A7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF92099527DB8E2A7u64;
        
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
pub fn _0xf92099527db8e2a7_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xF92099527DB8E2A7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xF92099527DB8E2A7u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

