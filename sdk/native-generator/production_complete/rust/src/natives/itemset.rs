//! ITEMSET native functions
//! 
//! Functions for the itemset category (local)

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// ## Parameters
*



pub fn get_indexed_item_in_itemset_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A197E2521EE2BABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A197E2521EE2BABu64;
        
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
pub fn get_indexed_item_in_itemset_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x7A197E2521EE2BABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x7A197E2521EE2BABu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn destroy_itemset_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE18220B1C183EDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE18220B1C183EDAu64;
        
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
pub fn destroy_itemset_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xDE18220B1C183EDAu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xDE18220B1C183EDAu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn create_itemset_safe(
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35AD299F50D91B24u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35AD299F50D91B24u64;
        
        let result = invoke_raw!(
            hash,
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn create_itemset_raw(
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x35AD299F50D91B24u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x35AD299F50D91B24u64;

        invoke_raw_typed!(
            hash,
        )
    }
}

/// ## Parameters
*



pub fn get_itemset_size_safe(
        
        
            x: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9127E83ABF7C631u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9127E83ABF7C631u64;
        
        let result = invoke_raw!(
            hash,
                x
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}

 // Пример фильтрации, если нужно
pub fn get_itemset_size_raw(
        x: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xD9127E83ABF7C631u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xD9127E83ABF7C631u64;

        invoke_raw_typed!(
            hash,
                x
        )
    }
}

/// ## Parameters
*



pub fn remove_from_itemset_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25E68244B0177686u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25E68244B0177686u64;
        
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
pub fn remove_from_itemset_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x25E68244B0177686u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x25E68244B0177686u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn is_itemset_valid_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1B1EA596344DFABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1B1EA596344DFABu64;
        
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
pub fn is_itemset_valid_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xB1B1EA596344DFABu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xB1B1EA596344DFABu64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn clean_itemset_safe(
        
        
            p0: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41BC0D722FC04221u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41BC0D722FC04221u64;
        
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
pub fn clean_itemset_raw(
        p0: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x41BC0D722FC04221u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x41BC0D722FC04221u64;

        invoke_raw_typed!(
            hash,
                p0
        )
    }
}

/// ## Parameters
*



pub fn is_in_itemset_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D0FC594D1E9C107u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D0FC594D1E9C107u64;
        
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
pub fn is_in_itemset_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0x2D0FC594D1E9C107u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0x2D0FC594D1E9C107u64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

/// ## Parameters
*



pub fn add_to_itemset_safe(
        
        
            p0: 
        , 
        
        
            p1: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3945201F14637DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3945201F14637DDu64;
        
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
pub fn add_to_itemset_raw(
        p0: , 
        p1: 
) ->  {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xE3945201F14637DDu32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xE3945201F14637DDu64;

        invoke_raw_typed!(
            hash,
                p0, 
                p1
        )
    }
}

