//! TESTCATEGORY native functions
//!
//! Functions for the testcategory category

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

/// Returns a dynamic array of integers. The length of the array is written to `count_out`.

pub fn get_dynamic_array_safe(
    count_out: &mut std::os::raw::c_int
) -> NativeResult<&mut i32> {
    // Variable declarations for CString, Vec<*const c_char> for char**, Vector3 mutable copy, etc.
    // Для out-параметра длины массива не нужно создавать CString или другие сложные обертки.
    // Он просто передается как мутабельная ссылка/указатель.

    debug!("Calling native function: GET_DYNAMIC_ARRAY ({:?}) with hash ", json!("count_out": count_out));

    let result = unsafe {
        crate::raw::GET_DYNAMIC_ARRAY( 
            count_out
        )
    };
    
    Ok(unsafe { *result })
}

/// Raw native function: GET_DYNAMIC_ARRAY
/// Hash: 
/// Description: Returns a dynamic array of integers. The length of the array is written to &#x60;count_out&#x60;.
/// 
/// # Safety
/// This function calls directly into the game's native function without validation or type conversion.
/// Ensure all parameters match the expected FFI types and that game state allows this call.
/// Use the safe wrapper `get_dynamic_array_safe` instead for most use cases.
#[allow(clippy::too_many_arguments)]
pub unsafe fn get_dynamic_array_raw(count_out: *mut std::os::raw::c_int) -> *mut std::os::raw::c_int {
    raw::GET_DYNAMIC_ARRAY(count_out)
}

