use anyhow::Result;
use std::path::Path;
use tracing::info;
use std::fs;
use std::io::Write;

use crate::native_types::NativeCollection;

pub struct TypeScriptGenerator;

impl TypeScriptGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate_all(&self, collection: &NativeCollection, output_dir: &Path) -> Result<()> {
        // Убеждаемся, что директория существует
        std::fs::create_dir_all(output_dir)?;
        let mut ts_code = String::from("// Auto-generated TypeScript definitions\n\n");
        info!("📘 Generating TypeScript definitions for {} categories...", collection.categories.len());
        for category in collection.categories.values() {
            for func in &category.functions {
                let func_name = func.typescript_name_override.as_ref()
                    .map(String::clone)
                    .unwrap_or_else(|| func.typescript_function_name());
                
                let params_ts = func.parameters.iter()
                    .map(|p| {
                        let param_name = p.typescript_name_override.as_ref().unwrap_or(&p.name);
                        let param_type = p.typescript_type_override.as_ref()
                            .map(|s| s.clone())
                            .unwrap_or_else(|| map_native_type_to_ts(&p.param_type));
                        format!("{}: {}", param_name, param_type)
                    })
                    .collect::<Vec<_>>().join(", ");
                
                let ret_type_ts = func.typescript_return_type_override.as_ref()
                    .map(|s| s.clone())
                    .unwrap_or_else(|| map_native_type_to_ts(&func.return_type));

                // JSDoc generation
                let mut jsdoc = String::from("/**\n");
                if let Some(desc) = &func.description {
                    jsdoc.push_str(&format!(" * {}\n", desc.replace("\n", "\n * ")));
                }
                for param in &func.parameters {
                    let param_name_for_jsdoc = param.typescript_name_override.as_ref().unwrap_or(&param.name);
                    jsdoc.push_str(&format!(" * @param {} - {}\n", param_name_for_jsdoc, param.description.as_deref().unwrap_or("")));
                }
                jsdoc.push_str(&format!(" * @returns {}\n", ret_type_ts));
                if let Some(hash) = &func.hash {
                    jsdoc.push_str(&format!(" * @remarks Hash: {}\n", hash));
                }
                jsdoc.push_str(" */\n");

                ts_code.push_str(&jsdoc);
                ts_code.push_str(&format!("export function {}({}): {};\n\n", func_name, params_ts, ret_type_ts));
            }
        }
        let file_path = output_dir.join("natives.ts");
        let mut file = fs::File::create(&file_path)?;
        file.write_all(ts_code.as_bytes())?;
        info!("✅ Generated TypeScript definitions for {} functions", collection.total_functions());
        Ok(())
    }

    /// Пример: генерация одного файла с одной функцией (MVP)
    pub fn generate_single_function_example(&self, output_dir: &Path) -> Result<()> {
        std::fs::create_dir_all(output_dir)?;
        let ts_code = r#"// Auto-generated TypeScript definition
export function getPlayerName(playerId: number): string;
"#;
        let file_path = output_dir.join("example_natives.ts");
        let mut file = fs::File::create(&file_path)?;
        file.write_all(ts_code.as_bytes())?;
        Ok(())
    }
}

/// Простое отображение NativeType -> TypeScript тип
fn map_native_type_to_ts(ty: &crate::native_types::NativeType) -> String {
    use crate::native_types::NativeType;
    match ty {
        NativeType::Void => "void".to_string(),
        NativeType::Bool => "boolean".to_string(),
        NativeType::Int => "number".to_string(),
        NativeType::Float => "number".to_string(),
        NativeType::String => "string".to_string(),
        NativeType::Player | NativeType::Ped | NativeType::Vehicle | NativeType::Entity | NativeType::Object | NativeType::Blip | NativeType::Cam | NativeType::Hash | NativeType::FireId | NativeType::Interior | NativeType::ItemSet | NativeType::Pickup => "number".to_string(),
        NativeType::Vector3 => "{ x: number, y: number, z: number }".to_string(),
        NativeType::Char => "number".to_string(), // или string, если char как символ
        NativeType::Horse | NativeType::HorseEntity | NativeType::Camp | NativeType::Prompt | NativeType::Volume => "number".to_string(),
        NativeType::Array { element_type, .. } => format!("{}[]", map_native_type_to_ts(element_type)),
        NativeType::Pointer(inner) | NativeType::Reference(inner) => map_native_type_to_ts(inner),
        NativeType::Any(_) => "any".to_string(),
        NativeType::FunctionCallback(_) => "(...args: any[]) => any".to_string(),
        NativeType::Opaque(_) => "any".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_types::{NativeCollection, NativeCategory, NativeFunction, NativeParameter, NativeType};
    use std::fs;
    use tempfile::tempdir;

    fn create_test_function(name: &str, ts_name_override: Option<&str>, ts_return_override: Option<&str>, params: Vec<NativeParameter>) -> NativeFunction {
        NativeFunction {
            name: name.to_string(),
            hash: Some(format!("0xHASH_{}", name)),
            description: Some(format!("Description for {}", name)),
            return_type: NativeType::Void, // Default, can be overridden by ts_return_override or specific tests
            typescript_name_override: ts_name_override.map(String::from),
            typescript_return_type_override: ts_return_override.map(String::from),
            parameters: params,
            category: "TEST_OVERRIDE".to_string(),
            examples: vec![],
            deprecated: false,
            platform_specific: None,
            raw_data: Default::default(),
            return_array_length_out_param: None,
            rust_mark_safe_wrapper_unsafe: None,
            rust_prologue_code: None,
            rust_epilogue_code: None,
            rust_return_type_override: None,
        }
    }

    fn create_test_param(name: &str, native_type: NativeType, ts_name_override: Option<&str>, ts_type_override: Option<&str>) -> NativeParameter {
        NativeParameter {
            name: name.to_string(),
            param_type: native_type,
            description: Some(format!("Desc for {}", name)),
            typescript_name_override: ts_name_override.map(String::from),
            typescript_type_override: ts_type_override.map(String::from),
            optional: false,
            default_value: None,
            rust_new_name: None,
            rust_type_override: None,
            rust_any_type_hint: None,
            rust_transform_input_with: None,
            rust_default_value_for_optional: None,
        }
    }

    #[test]
    fn test_typescript_overrides() {
        let mut collection = NativeCollection::new();
        let mut category = NativeCategory {
            name: "TEST_OVERRIDE".to_string(),
            description: None,
            functions: vec![
                create_test_function(
                    "GET_PLAYER_OLD_NAME", 
                    Some("getPlayerNewName"), 
                    Some("Promise<string>"), 
                    vec![
                        create_test_param("player_id", NativeType::Int, Some("newPlayerId"), Some("PlayerHandle")),
                        create_test_param("unused_param", NativeType::Bool, None, None) // No override
                    ]
                ),
                create_test_function(
                    "DO_SOMETHING",
                    None, // No function name override
                    None, // No return type override (should use mapped NativeType::Void -> "void")
                    vec![
                        create_test_param("value", NativeType::Float, None, Some("CustomFloatType"))
                    ]
                )
            ],
        };
        collection.add_category(category);

        let gen = TypeScriptGenerator::new();
        let dir = tempdir().unwrap();
        gen.generate_all(&collection, dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join("natives.ts")).unwrap();

        // Check first function with all overrides
        let expected_func1_sig = "export function getPlayerNewName(newPlayerId: PlayerHandle, unused_param: boolean): Promise<string>;";
        let expected_func1_jsdoc_param1 = "@param newPlayerId - Desc for player_id";
        let expected_func1_jsdoc_param2 = "@param unused_param - Desc for unused_param";
        let expected_func1_jsdoc_return = "@returns Promise<string>";
        assert!(content.contains(expected_func1_sig), "Signature for getPlayerNewName not found or incorrect. Content:\n{}", content);
        assert!(content.contains(expected_func1_jsdoc_param1), "JSDoc for newPlayerId not found or incorrect. Content:\n{}", content);
        assert!(content.contains(expected_func1_jsdoc_param2), "JSDoc for unused_param not found or incorrect. Content:\n{}", content);
        assert!(content.contains(expected_func1_jsdoc_return), "JSDoc return for getPlayerNewName not found or incorrect. Content:\n{}", content);

        // Check second function with partial overrides
        let expected_func2_sig = "export function doSomething(value: CustomFloatType): void;"; // Mapped NativeType::Void
        let expected_func2_jsdoc_param = "@param value - Desc for value";
        let expected_func2_jsdoc_return = "@returns void"; // Mapped NativeType::Void
        assert!(content.contains(expected_func2_sig), "Signature for doSomething not found or incorrect. Content:\n{}", content);
        assert!(content.contains(expected_func2_jsdoc_param), "JSDoc for value in doSomething not found or incorrect. Content:\n{}", content);
        assert!(content.contains(expected_func2_jsdoc_return), "JSDoc return for doSomething not found or incorrect. Content:\n{}", content);
    }

    #[test]
    fn test_generate_multiple_functions() {
        let mut collection = NativeCollection::new();
        let mut cat = NativeCategory {
            name: "PLAYER".to_string(),
            description: None,
            functions: vec![
                NativeFunction {
                    name: "GET_PLAYER_NAME".to_string(),
                    hash: None,
                    description: None,
                    return_type: NativeType::String,
                    typescript_name_override: None,
                    typescript_return_type_override: None,
                    parameters: vec![NativeParameter {
                        name: "playerId".to_string(),
                        param_type: NativeType::Int,
                        description: None,
                        typescript_name_override: None,
                        typescript_type_override: None,
                        optional: false,
                        default_value: None,
                        rust_new_name: None,
                        rust_type_override: None,
                        rust_any_type_hint: None,
                        rust_transform_input_with: None,
                        rust_default_value_for_optional: None,
                    }],
                    category: "PLAYER".to_string(),
                    examples: vec![],
                    deprecated: false,
                    platform_specific: None,
                    raw_data: Default::default(),
                    return_array_length_out_param: None,
                    rust_mark_safe_wrapper_unsafe: None,
                    rust_prologue_code: None,
                    rust_epilogue_code: None,
                    rust_return_type_override: None,
                },
                NativeFunction {
                    name: "IS_PLAYER_DEAD".to_string(),
                    hash: None,
                    description: None,
                    return_type: NativeType::Bool,
                    typescript_name_override: None,
                    typescript_return_type_override: None,
                    parameters: vec![NativeParameter {
                        name: "playerId".to_string(),
                        param_type: NativeType::Int,
                        description: None,
                        typescript_name_override: None,
                        typescript_type_override: None,
                        optional: false,
                        default_value: None,
                        rust_new_name: None,
                        rust_type_override: None,
                        rust_any_type_hint: None,
                        rust_transform_input_with: None,
                        rust_default_value_for_optional: None,
                    }],
                    category: "PLAYER".to_string(),
                    examples: vec![],
                    deprecated: false,
                    platform_specific: None,
                    raw_data: Default::default(),
                    return_array_length_out_param: None,
                    rust_mark_safe_wrapper_unsafe: None,
                    rust_prologue_code: None,
                    rust_epilogue_code: None,
                    rust_return_type_override: None,
                },
            ],
        };
        collection.add_category(cat);
        let gen = TypeScriptGenerator::new();
        let dir = tempdir().unwrap();
        gen.generate_all(&collection, dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join("natives.ts")).unwrap();
        assert!(content.contains("export function getPlayerName(playerId: number): string"));
        assert!(content.contains("export function isPlayerDead(playerId: number): boolean"));
    }

    #[test]
    fn test_generate_jsdoc_comments() {
        let mut collection = NativeCollection::new();
        let func_with_docs = NativeFunction {
            name: "TEST_FUNCTION_WITH_DOCS".to_string(),
            hash: Some("0x12345678".to_string()),
            description: Some("This is a test function.\nIt has multiple lines.".to_string()),
            return_type: NativeType::Int,
            typescript_name_override: None,
            typescript_return_type_override: None,
            parameters: vec![
                NativeParameter {
                    name: "param1".to_string(),
                    param_type: NativeType::String,
                    description: Some("The first parameter.".to_string()),
                    typescript_name_override: None,
                    typescript_type_override: None,
                    optional: false,
                    default_value: None,
                    rust_new_name: None,
                    rust_type_override: None,
                    rust_any_type_hint: None,
                    rust_transform_input_with: None,
                    rust_default_value_for_optional: None,
                },
                NativeParameter {
                    name: "param2".to_string(),
                    param_type: NativeType::Bool,
                    description: None, // No description for this one
                    typescript_name_override: None,
                    typescript_type_override: None,
                    optional: true,
                    default_value: Some("true".to_string()),
                    rust_new_name: None,
                    rust_type_override: None,
                    rust_any_type_hint: None,
                    rust_transform_input_with: None,
                    rust_default_value_for_optional: None,
                },
            ],
            category: "TEST_DOCS".to_string(),
            examples: vec![],
            deprecated: false,
            platform_specific: None,
            raw_data: Default::default(),
            return_array_length_out_param: None,
            rust_mark_safe_wrapper_unsafe: None,
            rust_prologue_code: None,
            rust_epilogue_code: None,
            rust_return_type_override: None,
        };
        let mut cat = NativeCategory {
            name: "TEST_DOCS".to_string(),
            description: None,
            functions: vec![func_with_docs],
        };
        collection.add_category(cat);

        let gen = TypeScriptGenerator::new();
        let dir = tempdir().unwrap();
        gen.generate_all(&collection, dir.path()).unwrap();
        let content = fs::read_to_string(dir.path().join("natives.ts")).unwrap();
        
        let expected_jsdoc = r#"/**
 * This is a test function.
 * It has multiple lines.
 * @param param1 - The first parameter.
 * @param param2 - 
 * @returns number
 * @remarks Hash: 0x12345678
 */"#;
        assert!(content.contains(expected_jsdoc), "Generated JSDoc does not match expected. Generated: \n{}", content);
        assert!(content.contains("export function testFunctionWithDocs(param1: string, param2: boolean): number;"));
    }

    #[test]
    fn test_map_native_type_to_ts_primitives() {
        assert_eq!(map_native_type_to_ts(&NativeType::Int), "number");
        assert_eq!(map_native_type_to_ts(&NativeType::Bool), "boolean");
        assert_eq!(map_native_type_to_ts(&NativeType::String), "string");
        assert_eq!(map_native_type_to_ts(&NativeType::Vector3), "{ x: number, y: number, z: number }");
    }

    #[test]
    fn test_map_native_type_to_ts_arrays() {
        let arr_num = NativeType::Array { element_type: Box::new(NativeType::Int), size_info: None };
        assert_eq!(map_native_type_to_ts(&arr_num), "number[]");
        let arr_str = NativeType::Array { element_type: Box::new(NativeType::String), size_info: None };
        assert_eq!(map_native_type_to_ts(&arr_str), "string[]");
        let arr_vec3 = NativeType::Array { element_type: Box::new(NativeType::Vector3), size_info: None };
        assert_eq!(map_native_type_to_ts(&arr_vec3), "{ x: number, y: number, z: number }[]");
    }

    #[test]
    fn test_map_native_type_to_ts_nested_arrays() {
        let arr_arr_num = NativeType::Array { element_type: Box::new(NativeType::Array { element_type: Box::new(NativeType::Int), size_info: None }), size_info: None };
        assert_eq!(map_native_type_to_ts(&arr_arr_num), "number[][]");
        let arr_arr_vec3 = NativeType::Array { element_type: Box::new(NativeType::Array { element_type: Box::new(NativeType::Vector3), size_info: None }), size_info: None };
        assert_eq!(map_native_type_to_ts(&arr_arr_vec3), "{ x: number, y: number, z: number }[][]");
    }
} 