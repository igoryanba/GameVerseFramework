//! TEST_CATEGORY native functions
//! 
//! Functions for the test_category category

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// This is a test native function to verify Rust-specific overrides from `native_configs.toml`.
It includes various parameter types and configurations to test renaming, type overriding,
prologue/epilogue code injection, and unsafe marking.



pub fn test_function_for_rust_overrides_safe(
        
        
            p0: 
        , 
        
        
            p1_some_struct_ptr: 
        , 
        
        
            p2_any_type: 
        , 
        
        
            p3_invalid_type_str: 
        , 
        
        
            p4_with_transform: 
        , 
        
        
            p5_optional_val: 
        
) -> NativeResult<> {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = 0xABCDEF01u32;
        #[cfg(target_pointer_width = "64")]
        let hash = 0xABCDEF01u64;
        
        let result = invoke_raw!(
            hash,
                p0, 
                p1_some_struct_ptr, 
                p2_any_type, 
                p3_invalid_type_str, 
                p4_with_transform, 
                p5_optional_val
        )?;


        // -------- Epilogue Start --------
        
        // -------- Epilogue End ----------
    }
    Ok(result)
}


