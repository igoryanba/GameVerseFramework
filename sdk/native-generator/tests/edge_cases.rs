use super::*;
use crate::native_types::*;
use crate::rust_generator::RustWrapperGenerator;

#[test]
fn test_empty_array_handling() {
    let func = NativeFunction {
        name: "TEST_EMPTY_ARRAY".to_string(),
        return_type: NativeType::Array(Box::new(NativeType::Int), None),
        parameters: vec![
            NativeParameter {
                name: "arr".to_string(),
                param_type: NativeType::Array(Box::new(NativeType::Int), None),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let generator = RustWrapperGenerator::new();
    let code = generator.generate_safe_wrapper(&func).unwrap();
    assert!(code.contains("if arr.is_empty()"));
    assert!(code.contains("return Ok(vec![]);"));
}

#[test]
fn test_null_terminated_strings() {
    let func = NativeFunction {
        name: "TEST_NULL_TERMINATED".to_string(),
        return_type: NativeType::String,
        parameters: vec![
            NativeParameter {
                name: "str".to_string(),
                param_type: NativeType::String,
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let generator = RustWrapperGenerator::new();
    let code = generator.generate_safe_wrapper(&func).unwrap();
    assert!(code.contains("CString::new(str).unwrap()"));
    assert!(code.contains(".to_string_lossy().into_owned()"));
}

#[test]
fn test_automatic_array_size_detection() {
    let mut func = NativeFunction {
        name: "TEST_AUTO_SIZE".to_string(),
        return_type: NativeType::Void,
        parameters: vec![
            NativeParameter {
                name: "data".to_string(),
                param_type: NativeType::Array(Box::new(NativeType::Int), None),
                ..Default::default()
            },
            NativeParameter {
                name: "count".to_string(),
                param_type: NativeType::Int,
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    // Автоматически определяем параметр размера
    func.detect_array_size_param();

    if let NativeType::Array(_, size_info) = &func.parameters[0].param_type {
        assert_eq!(size_info.as_ref().unwrap(), "count");
    } else {
        panic!("Array size param not detected");
    }
}

#[test]
fn test_function_callback_handling() {
    let func = NativeFunction {
        name: "TEST_CALLBACK".to_string(),
        return_type: NativeType::Void,
        parameters: vec![
            NativeParameter {
                name: "callback".to_string(),
                param_type: NativeType::FunctionCallback(Some("fn(i32) -> bool".to_string())),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let generator = RustWrapperGenerator::new();
    let code = generator.generate_safe_wrapper(&func).unwrap();
    assert!(code.contains("extern \"C\" fn callback_wrapper"));
    assert!(code.contains("let user_callback: fn(i32) -> bool"));
} 