//! BRAIN native functions
//! 
//! Functions for the brain category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ```
BRAIN::ADD_SCRIPT_TO_RANDOM_PED("pb_prostitute", ${s_f_y_hooker_01}, 100, 0);
- Nacorpio



pub fn add_script_to_random_ped_safe(
        
        
            name: 
        , 
        
        
            model: 
        , 
        
        
            p2: 
        , 
        
        
            p3: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4EE5367468A65CCCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4EE5367468A65CCCu64;
        
        let result = invoke_raw!(
            hash,
                name, 
                model, 
                p2, 
                p3
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn add_script_to_random_ped_raw(
        name: , 
        model: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4EE5367468A65CCCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4EE5367468A65CCCu64;

        invoke_raw_typed!(
            hash,
                name, 
                model, 
                p2, 
                p3
        )
    }
}

/// ```
Possible values:  
act_cinema  
am_mp_carwash_launch  
am_mp_carwash_control  
am_mp_property_ext  
chop  
fairgroundHub  
launcher_BasejumpHeli  
launcher_BasejumpPack  
launcher_CarWash  
launcher_golf  
launcher_Hunting_Ambient  
launcher_MrsPhilips  
launcher_OffroadRacing  
launcher_pilotschool  
launcher_Racing  
launcher_rampage  
launcher_rampage  
launcher_range  
launcher_stunts  
launcher_stunts  
launcher_tennis  
launcher_Tonya  
launcher_Triathlon  
launcher_Yoga  
ob_mp_bed_low  
ob_mp_bed_med  
```



pub fn _0x6d6840cee8845831_safe(
        
        
            action: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D6840CEE8845831u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D6840CEE8845831u64;
        
        let result = invoke_raw!(
            hash,
                action
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6d6840cee8845831_raw(
        action: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6D6840CEE8845831u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6D6840CEE8845831u64;

        invoke_raw_typed!(
            hash,
                action
        )
    }
}

/// ```
Something like flush_all_scripts   
Most of time comes after NETWORK_END_TUTORIAL_SESSION() or before TERMINATE_THIS_THREAD()  
```



pub fn _0x4d953df78ebf8158_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D953DF78EBF8158u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D953DF78EBF8158u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4d953df78ebf8158_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4D953DF78EBF8158u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4D953DF78EBF8158u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// _0x0B40ED49D7D6FF84 native function



pub fn _0x0b40ed49d7d6ff84_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B40ED49D7D6FF84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B40ED49D7D6FF84u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x0b40ed49d7d6ff84_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0B40ED49D7D6FF84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0B40ED49D7D6FF84u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn is_object_within_brain_activation_range_safe(
        
        
            object: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCBA154209823057u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCBA154209823057u64;
        
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
pub fn is_object_within_brain_activation_range_raw(
        object: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xCCBA154209823057u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xCCBA154209823057u64;

        invoke_raw_typed!(
            hash,
                object
        )
    }
}

/// ## Parameters
*



pub fn register_world_point_script_brain_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3CDC7136613284BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3CDC7136613284BDu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn register_world_point_script_brain_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x3CDC7136613284BDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x3CDC7136613284BDu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Looks like a cousin of above function _6D6840CEE8845831 as it was found among them. Must be similar  
Here are possible values of argument -   
"ob_tv"  
"launcher_Darts"  
```



pub fn _0x6e91b04e08773030_safe(
        
        
            action: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E91B04E08773030u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E91B04E08773030u64;
        
        let result = invoke_raw!(
            hash,
                action
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x6e91b04e08773030_raw(
        action: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x6E91B04E08773030u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x6E91B04E08773030u64;

        invoke_raw_typed!(
            hash,
                action
        )
    }
}

/// ## Parameters
*



pub fn enable_script_brain_set_safe(
        
        
            brainSet: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67AA4D73F0CFA86Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67AA4D73F0CFA86Bu64;
        
        let result = invoke_raw!(
            hash,
                brainSet
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn enable_script_brain_set_raw(
        brainSet: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x67AA4D73F0CFA86Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x67AA4D73F0CFA86Bu64;

        invoke_raw_typed!(
            hash,
                brainSet
        )
    }
}

/// ```
Gets whether the world point the calling script is registered to is within desired range of the player.  
```



pub fn is_world_point_within_brain_activation_range_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5042CC6F5E3D450u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5042CC6F5E3D450u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_world_point_within_brain_activation_range_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC5042CC6F5E3D450u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC5042CC6F5E3D450u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Registers a script for any object with a specific model hash.
BRAIN::REGISTER_OBJECT_SCRIPT_BRAIN("ob_telescope", ${prop_telescope_01}, 100, 4.0, -1, 9);
```



pub fn register_object_script_brain_safe(
        
        
            scriptName: 
        , 
        
        
            modelHash: 
        , 
        
        
            p2: 
        , 
        
        
            activationRange: 
        , 
        
        
            p4: 
        , 
        
        
            p5: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BE84C318BA6EC22u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BE84C318BA6EC22u64;
        
        let result = invoke_raw!(
            hash,
                scriptName, 
                modelHash, 
                p2, 
                activationRange, 
                p4, 
                p5
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn register_object_script_brain_raw(
        scriptName: , 
        modelHash: , 
        p2: , 
        activationRange: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0BE84C318BA6EC22u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0BE84C318BA6EC22u64;

        invoke_raw_typed!(
            hash,
                scriptName, 
                modelHash, 
                p2, 
                activationRange, 
                p4, 
                p5
        )
    }
}

/// ## Parameters
*



pub fn disable_script_brain_set_safe(
        
        
            brainSet: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14D8518E9760F08Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14D8518E9760F08Fu64;
        
        let result = invoke_raw!(
            hash,
                brainSet
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn disable_script_brain_set_raw(
        brainSet: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x14D8518E9760F08Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x14D8518E9760F08Fu64;

        invoke_raw_typed!(
            hash,
                brainSet
        )
    }
}

