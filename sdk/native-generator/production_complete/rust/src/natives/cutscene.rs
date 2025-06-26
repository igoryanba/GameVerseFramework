//! CUTSCENE native functions
//! 
//! Functions for the cutscene category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// REMOVE_CUTSCENE native function



pub fn remove_cutscene_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x440AF51A3462B86Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x440AF51A3462B86Fu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_cutscene_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x440AF51A3462B86Fu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x440AF51A3462B86Fu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// See [`SET_PED_PROP_INDEX`](#_0x93376B65A266EB5F)



pub fn set_cutscene_ped_prop_variation_safe(
        
        
            cutsceneEntName: 
        , 
        
        
            componentId: 
        , 
        
        
            drawableId: 
        , 
        
        
            textureId: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0546524ADE2E9723u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0546524ADE2E9723u64;
        
        let result = invoke_raw!(
            hash,
                cutsceneEntName, 
                componentId, 
                drawableId, 
                textureId, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cutscene_ped_prop_variation_raw(
        cutsceneEntName: , 
        componentId: , 
        drawableId: , 
        textureId: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0546524ADE2E9723u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0546524ADE2E9723u64;

        invoke_raw_typed!(
            hash,
                cutsceneEntName, 
                componentId, 
                drawableId, 
                textureId, 
                modelHash
        )
    }
}

/// ```
NativeDB Introduced: v1290
```



pub fn _0x4fcd976da686580c_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4FCD976DA686580Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4FCD976DA686580Cu64;
        
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
pub fn _0x4fcd976da686580c_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4FCD976DA686580Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4FCD976DA686580Cu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
flags: Usually 0.
```



pub fn start_cutscene_safe(
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x186D5CB5E7B0FF7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x186D5CB5E7B0FF7Bu64;
        
        let result = invoke_raw!(
            hash,
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_cutscene_raw(
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x186D5CB5E7B0FF7Bu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x186D5CB5E7B0FF7Bu64;

        invoke_raw_typed!(
            hash,
                flags
        )
    }
}

/// Stop cutscene instantly, will dump registered entities right where they were when ran.



pub fn stop_cutscene_immediately_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD220BDD222AC4A1Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD220BDD222AC4A1Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn stop_cutscene_immediately_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD220BDD222AC4A1Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD220BDD222AC4A1Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Only used twice in armenian1.c



pub fn register_synchronised_script_speech_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2131046957F31B04u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2131046957F31B04u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn register_synchronised_script_speech_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2131046957F31B04u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2131046957F31B04u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _get_cut_file_num_sections_safe(
        
        
            cutsceneName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0ABC54DE641DC0FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0ABC54DE641DC0FCu64;
        
        let result = invoke_raw!(
            hash,
                cutsceneName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_cut_file_num_sections_raw(
        cutsceneName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0ABC54DE641DC0FCu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0ABC54DE641DC0FCu64;

        invoke_raw_typed!(
            hash,
                cutsceneName
        )
    }
}

/// See [`SET_PED_COMPONENT_VARIATION`](#_0x262B14F48D29DE80)



pub fn set_cutscene_ped_component_variation_safe(
        
        
            cutsceneEntName: 
        , 
        
        
            componentId: 
        , 
        
        
            drawableId: 
        , 
        
        
            textureId: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA01E7B6DEEFBBC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA01E7B6DEEFBBC9u64;
        
        let result = invoke_raw!(
            hash,
                cutsceneEntName, 
                componentId, 
                drawableId, 
                textureId, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cutscene_ped_component_variation_raw(
        cutsceneEntName: , 
        componentId: , 
        drawableId: , 
        textureId: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xBA01E7B6DEEFBBC9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xBA01E7B6DEEFBBC9u64;

        invoke_raw_typed!(
            hash,
                cutsceneEntName, 
                componentId, 
                drawableId, 
                textureId, 
                modelHash
        )
    }
}

/// ## Return value



pub fn was_cutscene_skipped_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40C8656EDAEDD569u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40C8656EDAEDD569u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn was_cutscene_skipped_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x40C8656EDAEDD569u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x40C8656EDAEDD569u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
This function is hard-coded to always return 1.  
```



pub fn _0x4cebc1ed31e8925e_safe(
        
        
            cutsceneName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4CEBC1ED31E8925Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4CEBC1ED31E8925Eu64;
        
        let result = invoke_raw!(
            hash,
                cutsceneName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x4cebc1ed31e8925e_raw(
        cutsceneName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4CEBC1ED31E8925Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4CEBC1ED31E8925Eu64;

        invoke_raw_typed!(
            hash,
                cutsceneName
        )
    }
}

/// ## Parameters
*



pub fn _0x20746f7b1032a3c7_safe(
        
        
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
        let hash = 0x20746F7B1032A3C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20746F7B1032A3C7u64;
        
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
pub fn _0x20746f7b1032a3c7_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x20746F7B1032A3C7u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x20746F7B1032A3C7u64;

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



pub fn set_cutscene_fade_values_safe(
        
        
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
        let hash = 0x8093F23ABACCC7D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8093F23ABACCC7D4u64;
        
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
pub fn set_cutscene_fade_values_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8093F23ABACCC7D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8093F23ABACCC7D4u64;

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
Simply loads the cutscene and doesn't do extra stuff that REQUEST_CUTSCENE does.
```



pub fn request_cut_file_safe(
        
        
            cutsceneName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06A3524161C502BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06A3524161C502BAu64;
        
        let result = invoke_raw!(
            hash,
                cutsceneName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_cut_file_raw(
        cutsceneName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06A3524161C502BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06A3524161C502BAu64;

        invoke_raw_typed!(
            hash,
                cutsceneName
        )
    }
}

/// ## Parameters
*



pub fn set_cutscene_can_be_skipped_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41FAA8FB2ECE8720u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41FAA8FB2ECE8720u64;
        
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
pub fn set_cutscene_can_be_skipped_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41FAA8FB2ECE8720u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41FAA8FB2ECE8720u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
SET_SCRIPT_*
Sets the cutscene's owning thread ID.
```



pub fn _0x8d9df6eca8768583_safe(
        
        
            threadId: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D9DF6ECA8768583u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D9DF6ECA8768583u64;
        
        let result = invoke_raw!(
            hash,
                threadId
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x8d9df6eca8768583_raw(
        threadId: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x8D9DF6ECA8768583u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x8D9DF6ECA8768583u64;

        invoke_raw_typed!(
            hash,
                threadId
        )
    }
}

/// ## Parameters
*



pub fn get_entity_index_of_registered_entity_safe(
        
        
            cutsceneEntName: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0741A26499654CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0741A26499654CDu64;
        
        let result = invoke_raw!(
            hash,
                cutsceneEntName, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_entity_index_of_registered_entity_raw(
        cutsceneEntName: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC0741A26499654CDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC0741A26499654CDu64;

        invoke_raw_typed!(
            hash,
                cutsceneEntName, 
                modelHash
        )
    }
}

/// ## Parameters
*



pub fn request_cutscene_safe(
        
        
            cutsceneName: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A86743F475D9E09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A86743F475D9E09u64;
        
        let result = invoke_raw!(
            hash,
                cutsceneName, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_cutscene_raw(
        cutsceneName: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A86743F475D9E09u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A86743F475D9E09u64;

        invoke_raw_typed!(
            hash,
                cutsceneName, 
                flags
        )
    }
}

/// HAS_CUTSCENE_CUT_THIS_FRAME native function



pub fn has_cutscene_cut_this_frame_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x708BDD8CD795B043u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x708BDD8CD795B043u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_cutscene_cut_this_frame_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x708BDD8CD795B043u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x708BDD8CD795B043u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0x011883f41211432a_safe(
        
        
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
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x011883F41211432Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x011883F41211432Au64;
        
        let result = invoke_raw!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                p6
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x011883f41211432a_raw(
        x1: , 
        y1: , 
        z1: , 
        x2: , 
        y2: , 
        z2: , 
        p6: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x011883F41211432Au32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x011883F41211432Au64;

        invoke_raw_typed!(
            hash,
                x1, 
                y1, 
                z1, 
                x2, 
                y2, 
                z2, 
                p6
        )
    }
}

/// ## Return value



pub fn _0x583df8e3d4afbd98_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x583DF8E3D4AFBD98u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x583DF8E3D4AFBD98u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x583df8e3d4afbd98_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x583DF8E3D4AFBD98u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x583DF8E3D4AFBD98u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn _0x5edef0cf8c1dab3c_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EDEF0CF8C1DAB3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EDEF0CF8C1DAB3Cu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0x5edef0cf8c1dab3c_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x5EDEF0CF8C1DAB3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x5EDEF0CF8C1DAB3Cu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn does_cutscene_entity_exist_safe(
        
        
            cutsceneEntName: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x499EF20C5DB25C59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x499EF20C5DB25C59u64;
        
        let result = invoke_raw!(
            hash,
                cutsceneEntName, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn does_cutscene_entity_exist_raw(
        cutsceneEntName: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x499EF20C5DB25C59u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x499EF20C5DB25C59u64;

        invoke_raw_typed!(
            hash,
                cutsceneEntName, 
                modelHash
        )
    }
}

/// Whether or not it is safe to run functions on the camera,
as the camera is now no longer being used by the cutscene.



pub fn can_set_exit_state_for_camera_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2CBCD0930DFB420u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2CBCD0930DFB420u64;
        
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
pub fn can_set_exit_state_for_camera_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB2CBCD0930DFB420u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB2CBCD0930DFB420u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Gets the current time of the cutscene.

```
NativeDB Introduced: v3258
```



pub fn get_cutscene_play_time_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x710286BC5EF4D6E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x710286BC5EF4D6E1u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cutscene_play_time_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x710286BC5EF4D6E1u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x710286BC5EF4D6E1u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn is_cutscene_playing_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3C2E180A40F031Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3C2E180A40F031Eu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_cutscene_playing_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD3C2E180A40F031Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD3C2E180A40F031Eu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn is_cutscene_playback_flag_set_safe(
        
        
            flag: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71B74D2AE19338D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71B74D2AE19338D0u64;
        
        let result = invoke_raw!(
            hash,
                flag
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_cutscene_playback_flag_set_raw(
        flag: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x71B74D2AE19338D0u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x71B74D2AE19338D0u64;

        invoke_raw_typed!(
            hash,
                flag
        )
    }
}

/// ## Parameters
*



pub fn has_this_cutscene_loaded_safe(
        
        
            cutsceneName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x228D3D94F8A11C3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x228D3D94F8A11C3Cu64;
        
        let result = invoke_raw!(
            hash,
                cutsceneName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_this_cutscene_loaded_raw(
        cutsceneName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x228D3D94F8A11C3Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x228D3D94F8A11C3Cu64;

        invoke_raw_typed!(
            hash,
                cutsceneName
        )
    }
}

/// ## Parameters
*



pub fn _0x06ee9048fd080382_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06EE9048FD080382u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06EE9048FD080382u64;
        
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
pub fn _0x06ee9048fd080382_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x06EE9048FD080382u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x06EE9048FD080382u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ```
playbackFlags: Which scenes should be played.
Example: 0x105 (bit 0, 2 and 8 set) will enable scene 1, 3 and 9.
```



pub fn request_cutscene_with_playback_list_safe(
        
        
            cutsceneName: 
        , 
        
        
            playbackFlags: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC23DE0E91C30B58Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC23DE0E91C30B58Cu64;
        
        let result = invoke_raw!(
            hash,
                cutsceneName, 
                playbackFlags, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn request_cutscene_with_playback_list_raw(
        cutsceneName: , 
        playbackFlags: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC23DE0E91C30B58Cu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC23DE0E91C30B58Cu64;

        invoke_raw_typed!(
            hash,
                cutsceneName, 
                playbackFlags, 
                flags
        )
    }
}

/// Returns when it is safe to start applying changes to cutscene entities.

Should always be used for applying components.

See [`SET_CUTSCENE_PED_COMPONENT_VARIATION_FROM_PED`](#_0x2A56C06EBEF2B0D9) and [`REGISTER_ENTITY_FOR_CUTSCENE`](#_0xE40C1C56DF95C2E8) for an example.

This will be true before the cutscene is considered loaded



pub fn can_request_assets_for_cutscene_entity_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB56BBBCC2955D9CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB56BBBCC2955D9CBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_request_assets_for_cutscene_entity_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB56BBBCC2955D9CBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB56BBBCC2955D9CBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn _0xe36a98d8ab3d3c66_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE36A98D8AB3D3C66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE36A98D8AB3D3C66u64;
        
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
pub fn _0xe36a98d8ab3d3c66_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE36A98D8AB3D3C66u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE36A98D8AB3D3C66u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Return value



pub fn is_cutscene_active_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x991251AFC3981F84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x991251AFC3981F84u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn is_cutscene_active_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x991251AFC3981F84u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x991251AFC3981F84u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn get_cutscene_section_playing_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49010A6A396553D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49010A6A396553D8u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cutscene_section_playing_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x49010A6A396553D8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x49010A6A396553D8u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
modelHash (p1) was always 0 in R* scripts  
```



pub fn can_set_enter_state_for_registered_entity_safe(
        
        
            cutsceneEntName: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x645D0B458D8E17B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x645D0B458D8E17B5u64;
        
        let result = invoke_raw!(
            hash,
                cutsceneEntName, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_set_enter_state_for_registered_entity_raw(
        cutsceneEntName: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x645D0B458D8E17B5u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x645D0B458D8E17B5u64;

        invoke_raw_typed!(
            hash,
                cutsceneEntName, 
                modelHash
        )
    }
}

/// ```
Simply checks if the cutscene has loaded and doesn't check via CutSceneManager as opposed to HAS_[THIS]_CUTSCENE_LOADED.
```



pub fn has_cut_file_loaded_safe(
        
        
            cutsceneName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1C996C2A744262Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1C996C2A744262Eu64;
        
        let result = invoke_raw!(
            hash,
                cutsceneName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_cut_file_loaded_raw(
        cutsceneName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA1C996C2A744262Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA1C996C2A744262Eu64;

        invoke_raw_typed!(
            hash,
                cutsceneName
        )
    }
}

/// ## Return value



pub fn has_cutscene_loaded_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC59F528E9AB9F339u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC59F528E9AB9F339u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_cutscene_loaded_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC59F528E9AB9F339u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC59F528E9AB9F339u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Return value



pub fn has_cutscene_finished_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C0A893088881D57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C0A893088881D57u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn has_cutscene_finished_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7C0A893088881D57u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7C0A893088881D57u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
SET_VEHICLE_*
```



pub fn _0x7f96f23fa9b73327_safe(
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F96F23FA9B73327u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F96F23FA9B73327u64;
        
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
pub fn _0x7f96f23fa9b73327_raw(
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7F96F23FA9B73327u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7F96F23FA9B73327u64;

        invoke_raw_typed!(
            hash,
                modelHash
        )
    }
}

/// ```
Toggles a value (bool) for cutscenes.
SET_*
```



pub fn _0xc61b86c9f61eb404_safe(
        
        
            toggle: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC61B86C9F61EB404u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC61B86C9F61EB404u64;
        
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
pub fn _0xc61b86c9f61eb404_raw(
        toggle: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC61B86C9F61EB404u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC61B86C9F61EB404u64;

        invoke_raw_typed!(
            hash,
                toggle
        )
    }
}

/// Sets cutscene location, used for multiplayer apartments/businesses.



pub fn set_cutscene_origin_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            heading: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB812B3FD1C01CF27u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB812B3FD1C01CF27u64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                heading, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cutscene_origin_raw(
        x: , 
        y: , 
        z: , 
        heading: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB812B3FD1C01CF27u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB812B3FD1C01CF27u64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                heading, 
                p4
        )
    }
}

/// Gets the total length of the cutscene irrespective of playback list in milliseconds
To account for sections, see [`_GET_CUTSCENE_END_TIME`](#_0x971D7B15BCDBEF99)



pub fn get_cutscene_total_duration_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE53B14A19E480D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE53B14A19E480D4u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cutscene_total_duration_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xEE53B14A19E480D4u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xEE53B14A19E480D4u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Sets the components for a cutscene ped, this will take precendence over the cutscene's component overrides. This does not require the entity be registered.

See



pub fn set_cutscene_ped_component_variation_from_ped_safe(
        
        
            cutsceneEntName: 
        , 
        
        
            ped: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A56C06EBEF2B0D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A56C06EBEF2B0D9u64;
        
        let result = invoke_raw!(
            hash,
                cutsceneEntName, 
                ped, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cutscene_ped_component_variation_from_ped_raw(
        cutsceneEntName: , 
        ped: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2A56C06EBEF2B0D9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2A56C06EBEF2B0D9u64;

        invoke_raw_typed!(
            hash,
                cutsceneEntName, 
                ped, 
                modelHash
        )
    }
}

/// Gets the elapsed time of the current cutscene in



pub fn get_cutscene_time_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE625BEABBAFFDAB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE625BEABBAFFDAB9u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_cutscene_time_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE625BEABBAFFDAB9u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE625BEABBAFFDAB9u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn stop_cutscene_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7272775B4DC786Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7272775B4DC786Eu64;
        
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
pub fn stop_cutscene_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xC7272775B4DC786Eu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xC7272775B4DC786Eu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Only used in networked environment with MP cutscenes



pub fn set_cutscene_entity_streaming_flags_safe(
        
        
            cutsceneEntName: 
        , 
        
        
            p1: 
        , 
        
        
            p2: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C61C75BEE8184C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C61C75BEE8184C2u64;
        
        let result = invoke_raw!(
            hash,
                cutsceneEntName, 
                p1, 
                p2
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn set_cutscene_entity_streaming_flags_raw(
        cutsceneEntName: , 
        p1: , 
        p2: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C61C75BEE8184C2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C61C75BEE8184C2u64;

        invoke_raw_typed!(
            hash,
                cutsceneEntName, 
                p1, 
                p2
        )
    }
}

/// Returns the time of the cutscene's end accounting for [`REQUEST_CUTSCENE_WITH_PLAYBACK_LIST`](#_0xC23DE0E91C30B58C) 

If a cutscene is laid out with 10 second sections, and section 0 and 1 are enabled then it would be 20000ms.

```
NativeDB Introduced: v1734
```



pub fn _get_cutscene_end_time_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x971D7B15BCDBEF99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x971D7B15BCDBEF99u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _get_cutscene_end_time_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x971D7B15BCDBEF99u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x971D7B15BCDBEF99u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// Returns the handle of a cutscene entity, can be ped



pub fn get_entity_index_of_cutscene_entity_safe(
        
        
            cutsceneEntName: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A2E9FDB9A8C62F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A2E9FDB9A8C62F6u64;
        
        let result = invoke_raw!(
            hash,
                cutsceneEntName, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_entity_index_of_cutscene_entity_raw(
        cutsceneEntName: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x0A2E9FDB9A8C62F6u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x0A2E9FDB9A8C62F6u64;

        invoke_raw_typed!(
            hash,
                cutsceneEntName, 
                modelHash
        )
    }
}

/// This can only be run once [`CAN_REQUEST_ASSETS_FOR_CUTSCENE_ENTITY`](#_0xB56BBBCC2955D9CB) is true, but can be run before [`HAS_CUTSCENE_LOADED`](#_0xC59F528E9AB9F339)



pub fn register_entity_for_cutscene_safe(
        
        
            cutsceneEntity: 
        , 
        
        
            cutsceneEntName: 
        , 
        
        
            p2: 
        , 
        
        
            modelHash: 
        , 
        
        
            p4: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE40C1C56DF95C2E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE40C1C56DF95C2E8u64;
        
        let result = invoke_raw!(
            hash,
                cutsceneEntity, 
                cutsceneEntName, 
                p2, 
                modelHash, 
                p4
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn register_entity_for_cutscene_raw(
        cutsceneEntity: , 
        cutsceneEntName: , 
        p2: , 
        modelHash: , 
        p4: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE40C1C56DF95C2E8u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE40C1C56DF95C2E8u64;

        invoke_raw_typed!(
            hash,
                cutsceneEntity, 
                cutsceneEntName, 
                p2, 
                modelHash, 
                p4
        )
    }
}

/// ## Return value



pub fn _0xa0fe76168a189ddb_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0FE76168A189DDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0FE76168A189DDBu64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn _0xa0fe76168a189ddb_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xA0FE76168A189DDBu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xA0FE76168A189DDBu64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ```
Simply unloads the cutscene and doesn't do extra stuff that REMOVE_CUTSCENE does.
```



pub fn remove_cut_file_safe(
        
        
            cutsceneName: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD00D76A7DFC9D852u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD00D76A7DFC9D852u64;
        
        let result = invoke_raw!(
            hash,
                cutsceneName
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn remove_cut_file_raw(
        cutsceneName: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD00D76A7DFC9D852u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD00D76A7DFC9D852u64;

        invoke_raw_typed!(
            hash,
                cutsceneName
        )
    }
}

/// Similar to [`SET_CUTSCENE_ORIGIN`](#_0xB812B3FD1C01CF27) but without heading and doesn't need [`START_CUTSCENE`](#_0x186D5CB5E7B0FF7B)



pub fn start_cutscene_at_coords_safe(
        
        
            x: 
        , 
        
        
            y: 
        , 
        
        
            z: 
        , 
        
        
            flags: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C9ADDA3244A1FBFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C9ADDA3244A1FBFu64;
        
        let result = invoke_raw!(
            hash,
                x, 
                y, 
                z, 
                flags
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn start_cutscene_at_coords_raw(
        x: , 
        y: , 
        z: , 
        flags: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x1C9ADDA3244A1FBFu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x1C9ADDA3244A1FBFu64;

        invoke_raw_typed!(
            hash,
                x, 
                y, 
                z, 
                flags
        )
    }
}

/// ## Parameters
*



pub fn _0x2f137b508de238f2_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F137B508DE238F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F137B508DE238F2u64;
        
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
pub fn _0x2f137b508de238f2_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2F137B508DE238F2u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2F137B508DE238F2u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// Returns if the script can begin interacting with the registered entity. Primarly used for lead-outs of cutscenes.
Returns on frame after cutscene ends, so you cannot get is while using IsCutsceneActive()

Whether it is safe to start doing scripted actions on the entity, like simulating walking out of a cutscene.



pub fn can_set_exit_state_for_registered_entity_safe(
        
        
            cutsceneEntName: 
        , 
        
        
            modelHash: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C6A6451C79E4662u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C6A6451C79E4662u64;
        
        let result = invoke_raw!(
            hash,
                cutsceneEntName, 
                modelHash
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn can_set_exit_state_for_registered_entity_raw(
        cutsceneEntName: , 
        modelHash: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x4C6A6451C79E4662u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x4C6A6451C79E4662u64;

        invoke_raw_typed!(
            hash,
                cutsceneEntName, 
                modelHash
        )
    }
}

/// ```
Only used twice in R* scripts  
```



pub fn set_cutscene_trigger_area_safe(
        
        
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
        let hash = 0x9896CE4721BE84BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9896CE4721BE84BAu64;
        
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
pub fn set_cutscene_trigger_area_raw(
        p0: , 
        p1: , 
        p2: , 
        p3: , 
        p4: , 
        p5: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x9896CE4721BE84BAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x9896CE4721BE84BAu64;

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

