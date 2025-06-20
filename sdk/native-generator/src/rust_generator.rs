use anyhow::{Result, Context};
use std::fs;
use std::path::Path;
use handlebars::{Handlebars, Helper, Context as HbContext, RenderContext, Output, HelperResult, RenderError};
use serde_json::json;
use tracing::{info, debug, warn, error};
// use serde_json::to_value;

use crate::native_types::{NativeCollection, NativeCategory, NativeType, NativeParameter, NativeFunction};

pub struct RustWrapperGenerator {
    handlebars: Handlebars<'static>,
}

impl RustWrapperGenerator {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        
        // Register templates
        handlebars.register_template_string("module", RUST_MODULE_TEMPLATE).unwrap();
        handlebars.register_template_string("function", RUST_FUNCTION_TEMPLATE).unwrap();
        handlebars.register_template_string("types", RUST_TYPES_TEMPLATE).unwrap();
        handlebars.register_template_string("lib", RUST_LIB_TEMPLATE).unwrap();
        
        // Register helpers
        handlebars.register_helper("uppercase", Box::new(uppercase_helper));
        handlebars.register_helper("lowercase", Box::new(lowercase_helper));
        handlebars.register_helper("snake_case", Box::new(snake_case_helper));
        handlebars.register_helper("rust_type", Box::new(rust_type_helper));
        handlebars.register_helper("raw_rust_type", Box::new(raw_rust_type_helper));
        handlebars.register_helper("ffi_call_param", Box::new(ffi_call_param_helper));
        handlebars.register_helper("safe_return_wrap", Box::new(safe_return_wrap_helper));
        handlebars.register_helper("is_type_condition", Box::new(is_type_condition_helper));
        handlebars.register_helper("is_array_and_has_dynamic_size_param", Box::new(is_array_and_has_dynamic_size_param_helper));
        handlebars.register_helper("get_size_param_for_array", Box::new(get_size_param_for_array_helper));
        handlebars.register_helper("get_pointed_type_raw_rust_str", Box::new(get_pointed_type_raw_rust_str_helper));
        
        Self { handlebars }
    }
    
    /// Generate all Rust wrappers from native collection
    pub fn generate_all(&self, collection: &NativeCollection, output_dir: &Path) -> Result<()> {
        info!("🦀 Generating Rust native function wrappers...");
        
        // Create output directory structure
        fs::create_dir_all(output_dir)?;
        fs::create_dir_all(output_dir.join("src"))?;
        fs::create_dir_all(output_dir.join("src/natives"))?;
        
        // Generate Cargo.toml
        self.generate_cargo_toml(collection, output_dir)?;
        
        // Generate lib.rs
        self.generate_lib_rs(collection, output_dir)?;
        
        // Generate types.rs (с учётом Opaque/FunctionCallback псевдотипов)
        self.generate_types_rs(collection, output_dir)?;
        
        // Generate error types
        self.generate_error_types(output_dir)?;
        
        // Generate module for each category
        for (category_name, category) in &collection.categories {
            debug!("Generating Rust module for category: {}", category_name);
            self.generate_category_module(category, output_dir)?;
        }
        
        // Generate mod.rs for natives module
        self.generate_natives_mod_rs(collection, output_dir)?;
        
        info!("✅ Generated Rust wrappers for {} functions", collection.total_functions());
        Ok(())
    }
    
    /// Generate Cargo.toml for the native wrappers crate
    fn generate_cargo_toml(&self, collection: &NativeCollection, output_dir: &Path) -> Result<()> {
        let cargo_toml = format!(r#"[package]
name = "gameverse-natives"
version = "0.1.0"
edition = "2021"
description = "Type-safe GameVerse native function wrappers"
license = "MIT"

# Generated from: {}
# Total functions: {}
# Generated at: {}

[dependencies]
# Core dependencies
anyhow = "1.0"
thiserror = "1.0"
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"

# Math and geometry
nalgebra = "0.32"

# FFI and low-level
libc = "0.2"

# Logging
tracing = "0.1"

[dependencies.gameverse-core]
path = "../../../core"
version = "0.1.0"

[lib]
name = "gameverse_natives"
crate-type = ["cdylib", "rlib"]

[features]
default = ["safe-wrappers"]
safe-wrappers = []
raw-access = []
debug-logging = ["tracing"]
"#, 
            collection.metadata.source,
            collection.total_functions(),
            collection.metadata.generated_at
        );
        
        fs::write(output_dir.join("Cargo.toml"), cargo_toml)?;
        Ok(())
    }
    
    /// Generate lib.rs
    fn generate_lib_rs(&self, collection: &NativeCollection, output_dir: &Path) -> Result<()> {
        let data = json!({
            "collection": collection,
            "categories": collection.categories.keys().collect::<Vec<_>>()
        });
        
        let content = self.handlebars.render("lib", &data)
            .context("Failed to render lib.rs template")?;
            
        fs::write(output_dir.join("src/lib.rs"), content)?;
        Ok(())
    }
    
    /// Generate types.rs with GameVerse-specific types + авто-сгенерированные псевдотипы (Opaque / FunctionCallback)
    fn generate_types_rs(&self, collection: &NativeCollection, output_dir: &Path) -> Result<()> {
        use std::collections::HashSet;

        // 1) Рендерим базовый шаблон
        let mut content = self
            .handlebars
            .render("types", &json!({}))
            .context("Failed to render types template")?;

        // 2) Собираем уникальные имена Opaque и флаг наличия FunctionCallback
        let mut opaque_names: HashSet<String> = HashSet::new();
        let mut has_function_callback = false;

        // рекурсивная функция обхода типов
        fn collect_types(nt: &crate::native_types::NativeType, opaques: &mut HashSet<String>, has_fc: &mut bool) {
            use crate::native_types::NativeType::*;
            match nt {
                Opaque(opt_name) => {
                    let name = opt_name.clone().unwrap_or_else(|| "Opaque".to_string());
                    opaques.insert(name);
                }
                FunctionCallback(_) => {
                    *has_fc = true;
                }
                Array { element_type, .. } => collect_types(element_type, opaques, has_fc),
                Pointer(inner) | Reference(inner) => collect_types(inner, opaques, has_fc),
                _ => {}
            }
        }

        for category in collection.categories.values() {
            for func in &category.functions {
                collect_types(&func.return_type, &mut opaque_names, &mut has_function_callback);
                for param in &func.parameters {
                    collect_types(&param.param_type, &mut opaque_names, &mut has_function_callback);
                }
            }
        }

        // 3) Добавляем секцию с псевдотипами
        content.push_str("\n\n// ===== Auto-generated pseudo-types =====\n");

        if has_function_callback {
            content.push_str("/// Generic function callback (signature unknown)\n");
            content.push_str("pub type FunctionCallback = Option<unsafe extern \"C\" fn()>;\n\n");
        }

        for name in &opaque_names {
            // Приводим имя к корректному идентификатору Rust (CamelCase, без пробелов)
            let rust_name = name
                .split(|c: char| !c.is_alphanumeric())
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let mut chrs = s.chars();
                    match chrs.next() {
                        Some(first) => first.to_uppercase().collect::<String>() + chrs.as_str(),
                        None => String::new(),
                    }
                })
                .collect::<String>();

            content.push_str(&format!(
                "/// Opaque pointer type automatically generated for `{}`\n#[repr(transparent)]\npub struct {}(pub *mut std::ffi::c_void);\nimpl {} {{\n    pub fn null() -> Self {{ Self(std::ptr::null_mut()) }}\n}}\n\n",
                name, rust_name, rust_name
            ));
        }

        fs::write(output_dir.join("src/types.rs"), content)?;
        Ok(())
    }
    
    /// Generate error types for native functions
    fn generate_error_types(&self, output_dir: &Path) -> Result<()> {
        let error_content = r#"//! Error types for GameVerse native function wrappers

use thiserror::Error;

/// Errors that can occur when calling native functions
#[derive(Error, Debug, Clone)]
pub enum NativeError {
    #[error("Invalid player ID: {0:?}")]
    InvalidPlayer(super::types::PlayerId),
    
    #[error("Invalid ped entity: {0:?}")]
    InvalidPed(super::types::PedEntity),
    
    #[error("Invalid vehicle entity: {0:?}")]
    InvalidVehicle(super::types::VehicleEntity),
    
    #[error("Invalid entity ID: {0:?}")]
    InvalidEntity(super::types::EntityId),
    
    #[error("Invalid object entity: {0:?}")]
    InvalidObject(super::types::ObjectEntity),
    
    #[error("Invalid blip ID: {0:?}")]
    InvalidBlip(super::types::BlipId),
    
    #[error("Invalid camera ID: {0:?}")]
    InvalidCamera(super::types::CameraId),
    
    #[error("Invalid position: {0:?}")]
    InvalidPosition(nalgebra::Vector3<f32>),
    
    #[error("Native function returned null")]
    NullReturn,
    
    #[error("Native function call failed: {0}")]
    CallFailed(String),
    
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Feature not available on this platform")]
    PlatformNotSupported,
}

/// Result type for native function calls
pub type NativeResult<T> = Result<T, NativeError>;

/// Validation helpers
pub mod validation {
    use super::*;
    use crate::types::*;
    
    /// Validate player ID
    pub fn validate_player_id(player_id: PlayerId) -> NativeResult<()> {
        if player_id.0 < 0 || player_id.0 > 255 {
            Err(NativeError::InvalidPlayer(player_id))
        } else {
            Ok(())
        }
    }
    
    /// Validate entity handle
    pub fn validate_entity(entity: EntityId) -> NativeResult<()> {
        if entity.0 == 0 {
            Err(NativeError::InvalidEntity(entity))
        } else {
            Ok(())
        }
    }
    
    /// Validate position coordinates
    pub fn validate_position(pos: nalgebra::Vector3<f32>) -> NativeResult<()> {
        if pos.x.is_finite() && pos.y.is_finite() && pos.z.is_finite() {
            Ok(())
        } else {
            Err(NativeError::InvalidPosition(pos))
        }
    }
}
"#;
        
        fs::write(output_dir.join("src/errors.rs"), error_content)?;
        Ok(())
    }
    
    /// Generate module for a specific category
    fn generate_category_module(&self, category: &NativeCategory, output_dir: &Path) -> Result<()> {
        // Оборачиваем каждую функцию для предоставления консистентного контекста для ffi_call_param_helper
        let functions_with_context: Vec<_> = category.functions.iter().map(|f_ref| {
            json!({
                "function_data": f_ref, // Оригинальная NativeFunction
                "native_function_parameters": f_ref.parameters, // Для ffi_call_param_helper
                "return_array_length_out_param_original_name": f_ref.return_array_length_out_param // Для ffi_call_param_helper
                // length_temp_var_name здесь не нужен, т.к. это для параметров функции, а не для возврата массива
            })
        }).collect();

        let data = json!({
            "category": category,
            "module_name": category.name.to_lowercase(),
            "functions": functions_with_context // Используем обернутые функции
        });
        
        let content = self.handlebars.render("module", &data)
            .context("Failed to render category module")?;
            
        let module_file = format!("{}.rs", category.name.to_lowercase());
        fs::write(output_dir.join("src/natives").join(module_file), content)?;
        
        Ok(())
    }
    
    /// Generate mod.rs for natives module
    fn generate_natives_mod_rs(&self, collection: &NativeCollection, output_dir: &Path) -> Result<()> {
        let mut mod_content = String::from("//! GameVerse native function modules\n\n");
        
        for category_name in collection.categories.keys() {
            let module_name = category_name.to_lowercase();
            mod_content.push_str(&format!("pub mod {};\n", module_name));
        }
        
        mod_content.push_str("\n// Re-export all functions for convenience\n");
        for category_name in collection.categories.keys() {
            let module_name = category_name.to_lowercase();
            mod_content.push_str(&format!("pub use {}::*;\n", module_name));
        }
        
        fs::write(output_dir.join("src/natives/mod.rs"), mod_content)?;
        Ok(())
    }
}

// Handlebars templates

const RUST_LIB_TEMPLATE: &str = r#"//! GameVerse Native Function Wrappers
//!
//! This crate provides type-safe Rust wrappers for all GameVerse native functions.
//! Generated from FiveM documentation with enhanced type safety and error handling.
//!
//! # Example
//!
//! ```rust
//! use gameverse_natives::prelude::*;
//!
//! fn example() -> NativeResult<()> {
//!     let player_id = PlayerId(0);
//!     let ped = get_player_ped_safe(player_id)?;
//!     let position = get_entity_coords_safe(ped.into())?;
//!     println!("Player position: {:?}", position);
//!     Ok(())
//! }
//! ```

pub mod types;
pub mod errors;
pub mod natives;

// Re-export commonly used items
pub mod prelude {
    pub use crate::types::*;
    pub use crate::errors::{NativeError, NativeResult};
    pub use crate::natives::*;
}

pub use prelude::*;
"#;

const RUST_TYPES_TEMPLATE: &str = r#"//! GameVerse native function types
//!
//! Type-safe wrappers for game entities and values

use serde::{Deserialize, Serialize};
use nalgebra::Vector3;

/// Player identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub i32);

/// Ped entity handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PedEntity(pub i32);

/// Vehicle entity handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VehicleEntity(pub i32);

/// Generic entity handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(pub i32);

/// Object entity handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectEntity(pub i32);

/// Blip identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlipId(pub i32);

/// Camera identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CameraId(pub i32);

// Conversion traits for type safety
impl From<PedEntity> for EntityId {
    fn from(ped: PedEntity) -> Self {
        EntityId(ped.0)
    }
}

impl From<VehicleEntity> for EntityId {
    fn from(vehicle: VehicleEntity) -> Self {
        EntityId(vehicle.0)
    }
}

impl From<ObjectEntity> for EntityId {
    fn from(object: ObjectEntity) -> Self {
        EntityId(object.0)
    }
}

impl TryFrom<EntityId> for PedEntity {
    type Error = crate::errors::NativeError;
    
    fn try_from(entity: EntityId) -> Result<Self, Self::Error> {
        // TODO: Add entity type validation
        Ok(PedEntity(entity.0))
    }
}

impl TryFrom<EntityId> for VehicleEntity {
    type Error = crate::errors::NativeError;
    
    fn try_from(entity: EntityId) -> Result<Self, Self::Error> {
        // TODO: Add entity type validation
        Ok(VehicleEntity(entity.0))
    }
}

/// 3D position vector
pub type Position = Vector3<f32>;

/// 3D rotation vector (in degrees)
pub type Rotation = Vector3<f32>;

/// RGBA color value
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }
}
"#;

const RUST_MODULE_TEMPLATE: &str = r#"//! {{category.name}} native functions
//! 
//! {{#if category.description}}{{category.description}}{{/if}}

use crate::types::*;
use crate::errors::{NativeError, NativeResult, validation};
use nalgebra::Vector3;
use tracing::{debug, warn};

{{#each functions}}
{{!-- Контекст 'this' здесь это объект {"function_data": ..., "native_function_parameters": ..., ...} --}}
{{!-- Передаем this.function_data как контекст для partial 'function' --}}
{{!-- ffi_call_param, вызываемый внутри 'function', будет иметь доступ к this.native_function_parameters и this.return_array_length_out_param_original_name --}}
{{> function this.function_data}}

{{/each}}
"#;

const RUST_FUNCTION_TEMPLATE: &str = r#"/// {{#if description}}{{{description}}}{{else}}{{name}} native function{{/if}}
{{#if deprecated}}#[deprecated]{{/if}}
{{log param}}
{{log param.param_type}}
pub {{#if rust_mark_safe_wrapper_unsafe}}unsafe {{/if}}fn {{snake_case name}}_safe(
    {{#each parameters as |param|}}
        {{log param}}
        {{log param.param_type}}
        {{#if (eq param.name ../return_array_length_out_param)}}
            {{#if param.rust_new_name}}{{param.rust_new_name}}{{else}}{{param.name}}{{/if}}: &mut {{get_pointed_type_raw_rust_str param.param_type}}
        {{else}}
            {{#if param.rust_new_name}}{{param.rust_new_name}}{{else}}{{param.name}}{{/if}}: {{! rust_type param is_return_type_flag=false func_context=../ }}
        {{/if}}
        {{#unless @last}}, {{/unless}}
    {{/each}}
) -> NativeResult<{{#if return_type}}{{! rust_type this is_return_type_flag=true func_context=this }}{{else}}Void{{/if}}> {
    {{#if rust_prologue_code}}
    // -------- Prologue Start --------
    {{{rust_prologue_code}}}
    // -------- Prologue End ----------
    {{/if}}
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = {{hash}}u32;
        #[cfg(target_pointer_width = "64")]
        let hash = {{hash}}u64;
        
        let result = invoke_raw!(
            hash,
            {{#each parameters as |param|}}
                {{#if param.rust_new_name}}{{param.rust_new_name}}{{else}}{{param.name}}{{/if}}{{#unless @last}}, {{/unless}}
            {{/each}}
        )?;

        {{#if rust_return_type_override}}
        let result: {{rust_return_type_override}} = result.try_into()?;
        {{/if}}

        // -------- Epilogue Start --------
        {{{rust_epilogue_code}}}
        // -------- Epilogue End ----------
    }
    Ok(result)
}

{{#if (ne name "TEST_FUNCTION_FOR_RUST_OVERRIDES")}} // Пример фильтрации, если нужно
pub {{#if rust_mark_safe_wrapper_unsafe}}unsafe {{/if}}fn {{snake_case name}}_raw(
    {{#each parameters as |param|}}
        {{#if param.rust_new_name}}{{param.rust_new_name}}{{else}}{{param.name}}{{/if}}: {{! raw_rust_type param.param_type }}{{#unless @last}}, {{/unless}}
    {{/each}}
) -> {{! raw_rust_type return_type }} {
    unsafe {
        #[cfg(target_pointer_width = "32")]
        let hash = {{hash}}u32;
        #[cfg(target_pointer_width = "64")]
        let hash = {{hash}}u64;

        invoke_raw_typed!(
            hash,
            {{#each parameters as |param|}}
                {{#if param.rust_new_name}}{{param.rust_new_name}}{{else}}{{param.name}}{{/if}}{{#unless @last}}, {{/unless}}
            {{/each}}
        )
    }
}
{{/if}}
"#;

// Handlebars helpers

fn uppercase_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(&param.to_uppercase())?;
    Ok(())
}

fn lowercase_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(&param.to_lowercase())?;
    Ok(())
}

fn snake_case_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    let snake_case = param.to_lowercase();
    out.write(&snake_case)?;
    Ok(())
}

// Helper function to convert NativeType to its Rust string representation for safe wrappers
fn rust_type_helper(
    h: &Helper,
    _: &Handlebars,
    ctx: &HbContext, // Используем ctx для логирования
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    // Первый параметр - это либо NativeParameter (для параметров функции),
    // либо NativeFunction (для возвращаемого значения, если is_return_type_flag = true).
    let main_data_json = h.param(0).ok_or_else(|| RenderError::new("rust_type_helper: Main data (NativeParameter/NativeFunction) not found"))?.value().clone();

    let is_return_type_flag = h.param(1)
        .and_then(|p| p.value().as_bool())
        .unwrap_or(false);

    let func_context_param_value = h.param(2).ok_or_else(|| RenderError::new("rust_type_helper: func_context (NativeFunction) parameter not found"))?.value();
    debug!("[RUST_TYPE_HELPER_DEBUG] func_context_param_value (raw before clone): {:?}", func_context_param_value);
    debug!("[RUST_TYPE_HELPER_DEBUG] entire HBS context from helper: {:?}", ctx.data());

    let func_context_json = func_context_param_value.clone();
    debug!("[RUST_TYPE_HELPER_DEBUG] func_context_json (cloned): {:?}", func_context_json);

    let func_context: NativeFunction = match serde_json::from_value(func_context_json.clone()) { // Клонируем еще раз для лога в случае ошибки
        Ok(fc) => fc,
        Err(e) => {
            error!("[RUST_TYPE_HELPER_ERROR] Failed to deserialize func_context. Error: {}. JSON value was: {:?}", e, func_context_json);
            return Err(RenderError::new(format!(
                "Failed to deserialize func_context in rust_type_helper: {}. Input JSON: {:?}",
                e,
                func_context_json
            )));
        }
    };

    let original_type: NativeType;
    let type_override_str: Option<String>;

    if is_return_type_flag {
        // main_data_json это NativeFunction
        let native_function: NativeFunction = match serde_json::from_value(main_data_json.clone()) { // Клонируем для лога
            Ok(nf) => nf,
            Err(e) => {
                error!("[RUST_TYPE_HELPER_ERROR] Failed to deserialize NativeFunction for return type. Error: {}. JSON value was: {:?}", e, main_data_json);
                return Err(RenderError::new(format!(
                    "Failed to deserialize NativeFunction for return type in rust_type_helper: {}. Input JSON: {:?}",
                    e,
                    main_data_json
                )));
            }
        };
        original_type = native_function.return_type.clone();
        type_override_str = native_function.rust_return_type_override.clone(); 
    } else {
        // main_data_json это NativeParameter
        let native_param: NativeParameter = match serde_json::from_value(main_data_json.clone()) { // Клонируем для лога
            Ok(np) => np,
            Err(e) => {
                error!("[RUST_TYPE_HELPER_ERROR] Failed to deserialize NativeParameter. Error: {}. JSON value was: {:?}", e, main_data_json);
                return Err(RenderError::new(format!(
                    "Failed to deserialize NativeParameter in rust_type_helper: {}. Input JSON: {:?}",
                    e,
                    main_data_json
                )));
            }
        };
        original_type = native_param.param_type.clone();
        type_override_str = native_param.rust_type_override.clone();
    }

    let rust_type_str = determine_rust_type(original_type, type_override_str.as_ref(), Some(&func_context), is_return_type_flag);
    out.write(&rust_type_str)?;
    Ok(())
}

// Standalone logic for determining Rust type string, callable recursively.
// Параметр is_return_type теперь напрямую используется из флага.
fn determine_rust_type(original_native_type: NativeType, type_override_str: Option<&String>, func_context: Option<&NativeFunction>, is_return_type_flag: bool) -> String {
    let mut effective_native_type = original_native_type.clone(); // Клонируем, чтобы можно было изменить

    if let Some(override_str) = type_override_str {
        if !override_str.trim().is_empty() {
            // Пытаемся распарсить override_str. 
            // Важно: NativeType::from_fivem_type может быть не идеален для всех Rust-специфичных переопределений.
            // Возможно, понадобится более гибкий парсер или явное указание формата.
            // Пока что используем существующий, предполагая, что он достаточно гибок или форматы совместимы.
            match NativeType::from_fivem_type(override_str) { // TODO: Убедиться, что from_fivem_type подходит
                parsed_type if !matches!(parsed_type, NativeType::Any(_)) || override_str.eq_ignore_ascii_case("any") => {
                    debug!("Applied type override: '{}' -> {:?}", override_str, parsed_type);
                    effective_native_type = parsed_type;
                }
                _ => {
                    warn!("Failed to parse rust_type_override '{}' into a valid NativeType or it resolved to Any unexpectedly. Using original type {:?}.", override_str, original_native_type);
                    // Остаемся с original_native_type (которое уже в effective_native_type)
                }
            }
        }
    }

    // Дальнейшая логика использует effective_native_type
    match effective_native_type {
        NativeType::String => "String".to_string(),
        NativeType::Int => "i32".to_string(),
        NativeType::Float => "f32".to_string(),
        NativeType::Bool => "bool".to_string(),
        NativeType::Void => "()".to_string(),
        NativeType::Player => "Player".to_string(),
        NativeType::Ped => "Ped".to_string(),
        NativeType::Vehicle => "Vehicle".to_string(),
        NativeType::Entity => "Entity".to_string(),
        NativeType::Object => "Object".to_string(),
        NativeType::Blip => "Blip".to_string(),
        NativeType::Cam => "Cam".to_string(),
        NativeType::Hash => "Hash".to_string(),
        NativeType::Vector3 => "Vector3".to_string(),
        NativeType::Any(_) => "Any".to_string(),
        NativeType::Pointer(inner) => match *inner {
            NativeType::Vector3 => "&mut Vector3".to_string(),
            NativeType::Any(_) => "*mut std::ffi::c_void".to_string(),
            NativeType::Char => "*mut std::os::raw::c_char".to_string(),
            it => format!("*mut {}", determine_rust_type(it, None, None, false)),
        },
        NativeType::Array { element_type: ref inner_array_type, size_info } => {
            // Для Char внутри массива всегда u8
            let inner_rust_type = match **inner_array_type {
                NativeType::Char => "u8".to_string(),
                _ => determine_rust_type(*inner_array_type.clone(), None, None, false),
            };
            match size_info {
                Some(crate::native_types::ArraySizeInfo::Known(n)) => {
                    format!("[{}; {}]", inner_rust_type, n)
                },
                Some(crate::native_types::ArraySizeInfo::NullTerminated)
                | Some(crate::native_types::ArraySizeInfo::Infer)
                | Some(crate::native_types::ArraySizeInfo::Dynamic { .. })
                | Some(crate::native_types::ArraySizeInfo::SizeParamIndex(_))
                | None => {
                    // Особый случай: char[] null-terminated → String
                    if matches!(**inner_array_type, NativeType::Char) {
                        return "String".to_string();
                    }
                    // Для возврата Vec<T> только если это не фиксированный Known(N)
                    if is_return_type_flag {
                        if let Some(func) = func_context {
                            if func.return_array_length_out_param.is_some() {
                                return format!("Vec<{}>", inner_rust_type);
                            }
                        }
                        let raw_inner_ffi_type = determine_raw_rust_ffi_type(*inner_array_type.clone());
                        if raw_inner_ffi_type.starts_with("*mut") || raw_inner_ffi_type.starts_with("*const") {
                            return raw_inner_ffi_type;
                        } else {
                            return format!("*mut {}", raw_inner_ffi_type);
                        }
                    }
                    format!("Vec<{}>", inner_rust_type)
                }
            }
        },
        NativeType::Char => "char".to_string(),
        NativeType::FunctionCallback(_) => "FunctionCallback".to_string(),
        NativeType::Opaque(opt_name) => opt_name.unwrap_or_else(|| "Opaque".to_string()),
        NativeType::Pickup => "std::os::raw::c_int".to_string(),
        NativeType::Horse | NativeType::HorseEntity | NativeType::Camp | NativeType::Prompt | NativeType::Volume => "std::os::raw::c_int".to_string(),
        _ => "std::os::raw::c_int".to_string(),
    }
}

// Helper function to convert NativeType to its raw Rust FFI string representation
fn raw_rust_type_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbContext,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param_type_json = h.param(0).ok_or_else(|| handlebars::RenderError::new("raw_rust_type_helper: Parameter type not found"))?.value().clone();
    println!("[DEBUG raw_rust_type_helper] param_type_json: type = {:?}, value = {}", param_type_json, param_type_json);
    let param_type: NativeType = if param_type_json.is_string() {
        match param_type_json.as_str().unwrap() {
            "Int" => NativeType::Int,
            "Float" => NativeType::Float,
            "Bool" => NativeType::Bool,
            "String" => NativeType::String,
            "Char" => NativeType::Char,
            _ => NativeType::Any(None),
        }
    } else if param_type_json.is_object() {
        let obj = param_type_json.as_object().unwrap();
        if let Some(ptr_val) = obj.get("Pointer") {
            let inner = if ptr_val.is_string() {
                match ptr_val.as_str().unwrap() {
                    "Int" => NativeType::Int,
                    "Float" => NativeType::Float,
                    "Bool" => NativeType::Bool,
                    "String" => NativeType::String,
                    "Char" => NativeType::Char,
                    _ => NativeType::Any(None),
                }
            } else {
                serde_json::from_value(ptr_val.clone()).unwrap_or(NativeType::Any(None))
            };
            NativeType::Pointer(Box::new(inner))
        } else if let Some(arr_val) = obj.get("Array") {
            if arr_val.is_object() {
                let arr_obj = arr_val.as_object().unwrap();
                if let Some(el_val) = arr_obj.get("element_type") {
                    let element_type = if el_val.is_string() {
                        match el_val.as_str().unwrap() {
                            "Int" => NativeType::Int,
                            "Float" => NativeType::Float,
                            "Bool" => NativeType::Bool,
                            "String" => NativeType::String,
                            "Char" => NativeType::Char,
                            _ => NativeType::Any(None),
                        }
                    } else {
                        serde_json::from_value(el_val.clone()).unwrap_or(NativeType::Any(None))
                    };
                    NativeType::Array { element_type: Box::new(element_type), size_info: None }
                } else {
                    NativeType::Any(None)
                }
            } else {
                NativeType::Any(None)
            }
        } else if let Some(ref_val) = obj.get("Reference") {
            let inner = if ref_val.is_string() {
                match ref_val.as_str().unwrap() {
                    "Int" => NativeType::Int,
                    "Float" => NativeType::Float,
                    "Bool" => NativeType::Bool,
                    "String" => NativeType::String,
                    "Char" => NativeType::Char,
                    _ => NativeType::Any(None),
                }
            } else {
                serde_json::from_value(ref_val.clone()).unwrap_or(NativeType::Any(None))
            };
            NativeType::Reference(Box::new(inner))
        } else {
            serde_json::from_value(param_type_json.clone()).unwrap_or(NativeType::Any(None))
        }
    } else {
        serde_json::from_value(param_type_json.clone()).unwrap_or(NativeType::Any(None))
    };
    out.write(&determine_raw_rust_ffi_type(param_type))?;
    Ok(())
}

// Standalone logic for determining raw Rust FFI type string.
fn determine_raw_rust_ffi_type(param_type: NativeType) -> String {
    match param_type {
        NativeType::Void => "()".to_string(),
        NativeType::Bool => "bool".to_string(), 
        NativeType::Int => "std::os::raw::c_int".to_string(), 
        NativeType::Float => "f32".to_string(),
        NativeType::String => "*const std::os::raw::c_char".to_string(),
        NativeType::Pointer(ref inner_type) if **inner_type == NativeType::Char => "*const std::os::raw::c_char".to_string(),
        NativeType::Char => "std::os::raw::c_char".to_string(), 
        NativeType::Player => "std::os::raw::c_int".to_string(), 
        NativeType::Ped => "std::os::raw::c_int".to_string(),    
        NativeType::Vehicle => "std::os::raw::c_int".to_string(), 
        NativeType::Entity => "std::os::raw::c_int".to_string(),   
        NativeType::Object => "std::os::raw::c_int".to_string(), 
        NativeType::Blip => "std::os::raw::c_int".to_string(),     
        NativeType::Cam => "std::os::raw::c_int".to_string(),   
        NativeType::Hash => "u32".to_string(), 
        NativeType::Vector3 => "*mut nalgebra::Vector3<f32>".to_string(), 
        NativeType::FireId => "std::os::raw::c_int".to_string(),   
        NativeType::Interior => "std::os::raw::c_int".to_string(), 
        NativeType::ItemSet => "std::os::raw::c_int".to_string(),   
        NativeType::Pickup => "std::os::raw::c_int".to_string(),     
        NativeType::Horse | NativeType::HorseEntity | NativeType::Camp | NativeType::Prompt | NativeType::Volume => "std::os::raw::c_int".to_string(),
        NativeType::Array { element_type, size_info: _ } => {
            if matches!(*element_type, NativeType::Char) {
                "*mut std::os::raw::c_char".to_string()
            } else {
                format!("*mut {}", element_type.to_raw_rust_type_string())
            }
        }
        NativeType::Pointer(inner_type) => { // Handles non-char pointers
            if matches!(*inner_type, NativeType::Any(_)) {
                "*mut std::ffi::c_void".to_string()
            } else {
                let inner_ffi_type = determine_raw_rust_ffi_type(*inner_type);
                format!("*mut {}", inner_ffi_type)
            }
        }
        NativeType::Reference(inner_type) => {
            let inner_ffi_type = determine_raw_rust_ffi_type(*inner_type);
            format!("*mut {}", inner_ffi_type) 
        }
        NativeType::Any(_) => "*mut std::ffi::c_void".to_string(),
        NativeType::FunctionCallback(_) => r#"Option<unsafe extern "C" fn()>"#.to_string(),
        NativeType::Opaque(Some(name)) => format!("/* Opaque C type: {} */ *mut std::ffi::c_void", name),
        NativeType::Opaque(None) => "/* Opaque C type */ *mut std::ffi::c_void".to_string(),
        _ => "std::os::raw::c_int".to_string(),
    }
}

// Simple local snake_case converter
fn to_snake_case_local(name: &str) -> String {
    name.to_lowercase() // Basic for now, can be improved with a regex or crate later if needed for complex names
}

// Helper to generate parameter passing logic for the raw FFI call
fn ffi_call_param_helper(
    h: &Helper,
    _hb: &Handlebars,
    ctx: &HbContext,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let current_param_json = h.param(0)
        .ok_or_else(|| RenderError::new("ffi_call_param_helper: Current parameter (NativeParameter) not found as param 0"))?
        .value();
    println!("[DEBUG ffi_call_param_helper] current_param_json: type = {:?}, value = {}", current_param_json, current_param_json);
    let current_param: NativeParameter = serde_json::from_value(current_param_json.clone())
        .map_err(|e| RenderError::new(format!("FFI Call Param: Failed to deserialize CurrentParam (NativeParameter): {}\nValue: {}", e, current_param_json)))?;

    // Получаем все параметры функции из контекста (передаваемого из safe_return_wrap_helper)
    let all_params: Vec<NativeParameter> = ctx.data().get("native_function_parameters")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    // Получаем имя сконфигурированного out-параметра длины из контекста
    let configured_length_out_param_name: Option<String> = ctx.data().get("return_array_length_out_param_original_name")
        .and_then(|v| v.as_str().map(String::from)
        .filter(|s| !s.is_empty()));

    // Получаем имя временной переменной для out-параметра длины из контекста
    let length_temp_var_name: Option<String> = ctx.data().get("length_temp_var_name")
        .and_then(|v| v.as_str().map(String::from)
        .filter(|s| !s.is_empty()));

    let current_param_name_snake = to_snake_case_local(&current_param.name);

    // Проверяем, является ли текущий параметр тем самым out-параметром длины
    if let Some(ref configured_name) = configured_length_out_param_name {
        println!("[DEBUG ffi_call_param_helper] current_param.name = {:?}, configured_length_out_param_name = {:?}, length_temp_var_name = {:?}", current_param.name, configured_name, length_temp_var_name);
        if current_param.name == *configured_name {
            if let Some(ref temp_var_name_for_length) = length_temp_var_name {
                println!("[DEBUG ffi_call_param_helper] Использую временную переменную для out-параметра длины: {}", temp_var_name_for_length);
                // Это out-параметр для длины. Формируем указатель на временную переменную.
                let temp_var_actual_type = match current_param.param_type {
                    NativeType::Pointer(ref inner_ptr_type) => determine_rust_type(*inner_ptr_type.clone(), None, None, false),
                    _ => "i32".to_string(), // По умолчанию, если не указатель (должно быть уточнено)
                };
                out.write(&format!("&mut {} as *mut {}", temp_var_name_for_length, temp_var_actual_type))?;
                return Ok(());
            } else {
                println!("[DEBUG ffi_call_param_helper] temp_var_name_for_length отсутствует!");
            }
        } else {
            println!("[DEBUG ffi_call_param_helper] current_param.name != configured_length_out_param_name");
        }
    } else {
        println!("[DEBUG ffi_call_param_helper] configured_length_out_param_name отсутствует!");
    }

    // Существующая логика для определения, является ли параметр параметром размера для другого массива
    let mut is_size_param_for_array: Option<String> = None;
    if let Some(array_param) = all_params.iter().find(|p| {
        if p.name == current_param.name { return false; } // не сам на себя
        if let NativeType::Array { ref size_info, .. } = p.param_type {
            if let Some(si) = size_info {
                match si {
                    crate::native_types::ArraySizeInfo::Dynamic { ref size_param_name } => size_param_name == &current_param.name,
                    crate::native_types::ArraySizeInfo::SizeParamIndex(param_idx) => {
                        // Определяем индекс текущего параметра (current_param)
                        let current_param_actual_idx = all_params.iter().position(|iter_p| iter_p.name == current_param.name);
                        current_param_actual_idx.map_or(false, |idx_val| *param_idx == idx_val)
                    }
                    _ => false,
                }
            } else { false }
        } else { false }
    }) {
        is_size_param_for_array = Some(to_snake_case_local(&array_param.name));
    }

    if let Some(array_name_snake) = is_size_param_for_array {
        let len_param_str = format!("{}_len_param", array_name_snake);
        // Эта переменная (например, my_array_len_param) должна быть уже объявлена в шаблоне safe_function_template
        // на основе Vec<T>.len() для параметра my_array
        out.write(&len_param_str)?;
        return Ok(());
    }
    
    // Основная логика преобразования параметра для FFI вызова
    let output_str_content = match &current_param.param_type {
        NativeType::String => format!("{}_cstr.as_ptr()", current_param_name_snake),
        NativeType::Pointer(ref inner_type) if **inner_type == NativeType::Char => format!("{}_cstr.as_ptr()", current_param_name_snake),
        NativeType::Array { element_type, size_info } => {
            if let NativeType::Char = **element_type {
                if let Some(crate::native_types::ArraySizeInfo::NullTerminated) = size_info {
                    format!("{}_cstr.as_ptr()", current_param_name_snake)
                } else {
                    // Для char[] который не null-terminated, и не имеет другого size_info, передаем как есть (должен быть Vec<u8>)
                    format!("{}.as_mut_ptr() as *mut _", current_param_name_snake)
                }
            } else if let NativeType::String = **element_type {
                 // Для массива строк (Vec<String>), нужно передать массив *const c_char
                 // Это обрабатывается в RUST_FUNCTION_TEMPLATE созданием `_param_name_raw_ptrs`
                 if let Some(crate::native_types::ArraySizeInfo::NullTerminated) = size_info { // Обычно для const char* argv[]
                    format!("_{}_raw_ptrs.as_ptr() as *mut _", current_param_name_snake)
                 } else {
                    format!("_{}_raw_ptrs.as_mut_ptr() as *mut _", current_param_name_snake) //  Если нужен *mut *mut char
                 }
            }
            else {
                // Для других Vec<T>, передаем как сырой указатель на данные
                format!("{}.as_mut_ptr() as *mut _", current_param_name_snake)
            }
        }
        NativeType::Vector3 => {
            // Vector3 передается как *mut nalgebra::Vector3<f32>
            // В RUST_FUNCTION_TEMPLATE создается let mut param_name_val_for_ffi = param_name.clone() или param_name;
            format!("&mut {}_val_for_ffi", current_param_name_snake)
        }
        NativeType::Pointer(_) | NativeType::Reference(_) => { 
            // Если это уже указатель в безопасной сигнатуре, передаем как есть
            current_param_name_snake.clone()
        }
        _ => { // Для i32, f32, bool, Hash и т.д.
            current_param_name_snake.clone()
        }
    };

    out.write(&output_str_content)?;
    Ok(())
}

// Helper to generate result wrapping logic for the safe function
fn safe_return_wrap_helper(
    h: &Helper,
    hb: &Handlebars,
    _ctx: &HbContext,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let func_json = h.param(0).ok_or_else(|| RenderError::new("safe_return_wrap_helper: NativeFunction (param 0) not found"))?.value();
    let native_function: NativeFunction = match serde_json::from_value(func_json.clone()) {
        Ok(val) => val,
        Err(e) => {
            println!("[DEBUG ERROR] Не удалось десериализовать NativeFunction: {}. Value: {:#?}", e, func_json);
            return Err(RenderError::new(format!("safe_return_wrap_helper: Failed to deserialize NativeFunction (param 0): {}. Value: {:?}", e, func_json)));
        }
    };

    // ВРЕМЕННАЯ ОТЛАДКА
    println!("[DEBUG safe_return_wrap_helper] native_function = {:#?}", native_function);

    let is_void_return = native_function.return_type == NativeType::Void;
    
    let mut template_data_for_render = serde_json::Map::new();
    
    let mut is_return_array_with_length_param = false;
    let mut length_out_param_details: Option<(String, String, NativeType)> = None; // (name_snake, original_name, original_type)
    let mut array_inner_rust_type = "()".to_string();

    if let NativeType::Array { element_type: ref inner_array_type, .. } = native_function.return_type {
        array_inner_rust_type = determine_rust_type(*inner_array_type.clone(), None, None, true);
        if let Some(ref len_param_name) = native_function.return_array_length_out_param {
            // ВРЕМЕННАЯ ОТЛАДКА
            println!("[DEBUG safe_return_wrap_helper] len_param_name = {:?}", len_param_name);
            println!("[DEBUG safe_return_wrap_helper] all param names = {:?}", native_function.parameters.iter().map(|p| &p.name).collect::<Vec<_>>());
            if let Some(param_info) = native_function.parameters.iter().find(|p| p.name == *len_param_name) {
                is_return_array_with_length_param = true;
                length_out_param_details = Some((
                    to_snake_case_local(len_param_name),
                    len_param_name.clone(),
                    param_info.param_type.clone()
                )); 
            } else {
                warn!("safe_return_wrap_helper: Length out parameter '{}' not found in function '{}' parameters.", len_param_name, native_function.name);
            }
        }
    }
    
    template_data_for_render.insert("result_var_name".to_string(), json!("result")); // Имя переменной с результатом FFI вызова
    template_data_for_render.insert("native_function_name_snake".to_string(), json!(to_snake_case_local(&native_function.name)));
    template_data_for_render.insert("is_void_return".to_string(), json!(is_void_return));
    template_data_for_render.insert("is_return_array_with_length_param".to_string(), json!(is_return_array_with_length_param));
    template_data_for_render.insert("array_inner_rust_type".to_string(), json!(array_inner_rust_type));
    
    let mut pre_call_statements_for_template = Vec::new();

    if let Some((ref len_param_snake_name, ref _orig_name, ref len_param_original_native_type)) = length_out_param_details {
        template_data_for_render.insert("length_param_snake_name".to_string(), json!(len_param_snake_name));
        let temp_var_name = format!("__{}_len_val", len_param_snake_name);
        let temp_var_type = match len_param_original_native_type {
            NativeType::Pointer(inner_ptr_type) => determine_rust_type(*inner_ptr_type.clone(), None, None, false),
            _ => "i32".to_string(), 
        };
        pre_call_statements_for_template.push(format!("let mut {}: {} = Default::default();", temp_var_name, temp_var_type));
        template_data_for_render.insert("length_temp_var_name".to_string(), json!(temp_var_name));
    } else {
        // Убедимся, что length_temp_var_name всегда присутствует в данных для шаблона, даже если пустой
        template_data_for_render.insert("length_temp_var_name".to_string(), json!("")); 
    }
    
    template_data_for_render.insert("native_function_parameters".to_string(), json!(native_function.parameters));
    template_data_for_render.insert("return_array_length_out_param_original_name".to_string(), 
        json!(native_function.return_array_length_out_param));
    template_data_for_render.insert("parameters".to_string(), serde_json::to_value(&native_function.parameters).unwrap());

    let (is_return_string, is_return_simple_array_no_length, is_return_pointer_non_special) = 
        match native_function.return_type {
            NativeType::String => (true, false, false),
            NativeType::Array { .. } if !is_return_array_with_length_param => (false, true, false),
            NativeType::Pointer(ref inner) if **inner != NativeType::Char && !matches!(**inner, NativeType::Array { .. }) => (false, false, true),
            _ => (false, false, false),
        };
    template_data_for_render.insert("is_return_string".to_string(), json!(is_return_string));
    template_data_for_render.insert("is_return_simple_array_no_length".to_string(), json!(is_return_simple_array_no_length));
    template_data_for_render.insert("is_return_pointer_non_special".to_string(), json!(is_return_pointer_non_special));

    let template_str = r#"
{{#if is_void_return}}
    // FFI call already happened to `result` variable, which is not used for void.
    // For void returns, the FFI call is made directly inside the template logic.
    // We should re-evaluate if `result` is even needed for void.
    // For now, assume the unsafe block already called the raw function.
    Ok(())
{{else if is_return_array_with_length_param}}
    {{{pre_call_statements_joined}}}
    // FFI call to `result` happened outside. Now use it.
    let ffi_result = {{result_var_name}};
    if ffi_result.is_null() {
        debug!("Native function {{native_function_name_snake}} returned a null pointer for an array, but had a length output parameter {{length_param_snake_name}} (value: {{length_temp_var_name}}). Returning empty Vec.");
        Ok(Vec::new())
    } else {
        let len = {{length_temp_var_name}} as usize; 
        if len == 0 {
            Ok(Vec::new())
        } else {
            let slice = unsafe { std::slice::from_raw_parts(ffi_result as *const {{array_inner_rust_type}}, len) };
            Ok(slice.iter().cloned().collect::<Vec<_>>())
        }
    }
{{else}}
    let value = {{result_var_name}};
    {{#if is_return_string}}
        if value.is_null() {
            Err(NativeError::NullReturn)
        } else {
            let c_str = unsafe { std::ffi::CStr::from_ptr(value as *const std::os::raw::c_char) };
            Ok(c_str.to_string_lossy().into_owned())
        }
    {{else if is_return_simple_array_no_length}}
        Ok(value) 
    {{else if is_return_pointer_non_special}}
        Ok(value) 
    {{else}}
        Ok(value.into())
    {{/if}}
{{/if}}
"#;
    
    template_data_for_render.insert("pre_call_statements_joined".to_string(), json!(pre_call_statements_for_template.join("\n    ")));

    let render_data = serde_json::Value::Object(template_data_for_render);

    match hb.render_template(template_str, &render_data) {
        Ok(s) => out.write(&s)?,
        Err(e) => return Err(RenderError::new(format!("Failed to render safe_return_wrap_helper template: {}\nData: {:#?}\nNativeFunction Used: {:#?}", e, render_data, native_function))),
    }

    Ok(())
}

// Helper to check complex type conditions
fn is_type_condition_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbContext,
    _rc: &mut RenderContext,
    out: &mut dyn Output, 
) -> HelperResult {
    let type_json = h.param(0).ok_or_else(|| RenderError::new("is_type_condition_helper: NativeType not found"))?.value();
    let native_type: NativeType = serde_json::from_value(type_json.clone())
        .map_err(|e| RenderError::new(format!("is_type_condition_helper: Failed to deserialize NativeType: {}", e)))?;
    let condition = h.param(1).ok_or_else(|| RenderError::new("is_type_condition_helper: Condition string not found"))?.value().as_str().unwrap_or("");

    let result = match condition {
        "String" => matches!(native_type, NativeType::String) || matches!(native_type, NativeType::Pointer(ref inner) if **inner == NativeType::Char),
        "Array" => matches!(native_type, NativeType::Array { .. }),
        "ArrayCharNullTerminated" => matches!(native_type, NativeType::Array { element_type: ref et, size_info: Some(crate::native_types::ArraySizeInfo::NullTerminated), .. } if **et == NativeType::Char),
        "ArrayStringNullTerminated" => matches!(native_type, NativeType::Array { element_type: ref et, size_info: Some(crate::native_types::ArraySizeInfo::NullTerminated), .. } if **et == NativeType::String),
        "ArrayWithSizeParam" => matches!(native_type, 
            NativeType::Array { size_info: Some(crate::native_types::ArraySizeInfo::Dynamic { .. }), .. } | 
            NativeType::Array { size_info: Some(crate::native_types::ArraySizeInfo::SizeParamIndex(_)), .. }
        ),
        "Vector3PointerOrRef" => 
            matches!(native_type, NativeType::Pointer(ref inner) if **inner == NativeType::Vector3) || 
            matches!(native_type, NativeType::Reference(ref inner) if **inner == NativeType::Vector3),
        _ => false,
    };

    if result {
        out.write(condition)?;
    }
    Ok(())
}

// Helper: Checks if a param is an array whose size is determined by another named or indexed parameter.
fn is_array_and_has_dynamic_size_param_helper(
    h: &Helper,
    _: &Handlebars,
    _ctx: &HbContext,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let type_json = h.param(0).ok_or_else(|| RenderError::new("is_array_and_has_dynamic_size_param: NativeType not found"))?.value();
    let native_type: NativeType = serde_json::from_value(type_json.clone()).map_err(|e| RenderError::new(format!("Deserialize error: {}", e)))?;
    
    let result = match native_type {
        NativeType::Array { size_info: Some(crate::native_types::ArraySizeInfo::Dynamic { .. }), .. } => true,
        NativeType::Array { size_info: Some(crate::native_types::ArraySizeInfo::SizeParamIndex(_)), .. } => true,
        _ => false,
    };

    if result { out.write("true")?; }
    Ok(())
}

// Helper: For an array parameter, finds its corresponding size parameter from the list of all function parameters.
// Returns the size parameter object if found, otherwise nothing (so #with works).
fn get_size_param_for_array_helper(
    h: &Helper,
    _: &Handlebars,
    _ctx: &HbContext, 
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let array_param_json = h.param(0).ok_or_else(|| RenderError::new("get_size_param_for_array: Array param not found"))?.value();
    let array_param: NativeParameter = serde_json::from_value(array_param_json.clone()).map_err(|e| RenderError::new(format!("Deserialize array_param: {}",e)))?;
    
    let all_params_json = h.param(1).ok_or_else(|| RenderError::new("get_size_param_for_array: All params not found"))?.value();
    let all_params: Vec<NativeParameter> = serde_json::from_value(all_params_json.clone()).map_err(|e| RenderError::new(format!("Deserialize all_params: {}",e)))?;

    let mut found_size_param: Option<NativeParameter> = None;

    if let NativeType::Array { size_info: Some(ref si), .. } = array_param.param_type {
        match si {
            crate::native_types::ArraySizeInfo::Dynamic { ref size_param_name } => {
                found_size_param = all_params.into_iter().find(|p| p.name == *size_param_name);
            }
            crate::native_types::ArraySizeInfo::SizeParamIndex(idx) => {
                found_size_param = all_params.into_iter().nth(*idx);
            }
            _ => {}
        }
    }

    if let Some(p) = found_size_param {
        out.write(&serde_json::to_string(&p)?)?;
    }
    Ok(())
}

// Новый хелпер для получения raw FFI типа для значения под указателем
fn get_pointed_type_raw_rust_str_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbContext,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param_type_json = h.param(0).ok_or_else(|| RenderError::new("get_pointed_type_raw_rust_str_helper: Parameter type not found"))?.value().clone();
    let param_type: NativeType = serde_json::from_value(param_type_json)
        .map_err(|e| RenderError::new(format!("Failed to deserialize NativeType in get_pointed_type_raw_rust_str_helper: {}", e)))?;

    match param_type {
        NativeType::Pointer(inner_type) => {
            out.write(&determine_raw_rust_ffi_type(*inner_type))?;
        }
        NativeType::Reference(inner_type) => { // Также обработаем Reference, на всякий случай
            out.write(&determine_raw_rust_ffi_type(*inner_type))?;
        }
        _ => {
            // Если это не Pointer/Reference, то это неожиданно для этого хелпера.
            // Возможно, стоит вернуть ошибку или просто raw_rust_type от самого типа.
            // Пока что вернем ошибку в виде комментария в выводе, чтобы было видно.
            // Или лучше RenderError? RenderError остановит генерацию.
            warn!("get_pointed_type_raw_rust_str_helper called with non-pointer type: {:?}. Falling back to raw_rust_type of the type itself.", param_type);
            out.write(&determine_raw_rust_ffi_type(param_type))?; 
        }
    }
    Ok(())
}

fn generate_string_array_marshaling(param: &NativeParameter) -> String {
    format!(
        r#"let {name}_cstr: Vec<std::ffi::CString> = {name}.iter()
            .map(|s| std::ffi::CString::new(s.as_str()).unwrap())
            .collect();
        let {name}_ptrs: Vec<*const c_char> = {name}_cstr.iter()
            .map(|cs| cs.as_ptr())
            .collect();"#,
        name = param.name
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_types::*;
    use tempfile::TempDir;

    #[test]
    fn test_rust_generator() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path();
        
        // Create test collection
        let mut collection = NativeCollection::new();
        
        let functions = vec![
            NativeFunction::new("GET_PLAYER_PED".to_string(), "PLAYER".to_string())
                .with_return_type(NativeType::Ped)
                .with_parameter(NativeParameter::new("playerId".to_string(), NativeType::Player)),
        ];
        
        let category = NativeCategory {
            name: "PLAYER".to_string(),
            description: Some("Player functions".to_string()),
            functions,
        };
        
        collection.add_category(category);
        
        // Generate Rust wrappers
        let generator = RustWrapperGenerator::new();
        generator.generate_all(&collection, output_path).unwrap();
        
        // Verify files were created
        assert!(output_path.join("Cargo.toml").exists());
        assert!(output_path.join("src/lib.rs").exists());
        assert!(output_path.join("src/types.rs").exists());
        assert!(output_path.join("src/errors.rs").exists());
        assert!(output_path.join("src/natives/mod.rs").exists());
        assert!(output_path.join("src/natives/player.rs").exists());
    }
    
    #[test]
    fn test_template_rendering_basic_function() {
        let generator = RustWrapperGenerator::new();
        let function = NativeFunction::new("GET_PLAYER_PED".to_string(), "PLAYER".to_string())
            .with_return_type(NativeType::Ped)
            .with_parameter(NativeParameter::new("playerId".to_string(), NativeType::Player));
        let function_json = serde_json::to_value(&function).unwrap();
        let mut wrapper = serde_json::Map::new();
        wrapper.insert("function_data".to_string(), function_json.clone());
        let wrapper_value = serde_json::Value::Object(wrapper);
        let result = generator.handlebars.render("function", &wrapper_value);
        let content = result.unwrap();
        assert!(content.contains("pub fn get_player_ped_safe("));
        assert!(content.contains("playerId: Player"));
        assert!(content.contains("-> NativeResult<Ped>"));
    }

    #[test]
    fn test_return_array_with_out_param_length() {
        use handlebars::Handlebars;
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("function", super::RUST_FUNCTION_TEMPLATE).unwrap();
        handlebars.register_helper("snake_case", Box::new(super::snake_case_helper));
        handlebars.register_helper("rust_type", Box::new(super::rust_type_helper));
        handlebars.register_helper("raw_rust_type", Box::new(super::raw_rust_type_helper));
        handlebars.register_helper("ffi_call_param", Box::new(super::ffi_call_param_helper));
        handlebars.register_helper("safe_return_wrap", Box::new(super::safe_return_wrap_helper));
        handlebars.register_helper("is_type_condition", Box::new(super::is_type_condition_helper));
        handlebars.register_helper("is_array_and_has_dynamic_size_param", Box::new(super::is_array_and_has_dynamic_size_param_helper));
        handlebars.register_helper("get_size_param_for_array", Box::new(super::get_size_param_for_array_helper));
        handlebars.register_helper("get_pointed_type_raw_rust_str", Box::new(super::get_pointed_type_raw_rust_str_helper));
        handlebars.register_helper("uppercase", Box::new(super::uppercase_helper));
        handlebars.register_helper("lowercase", Box::new(super::lowercase_helper));
        let function = NativeFunction::new(
            "GET_ARRAY_OF_INTS_EXTERNALLY_SIZED_TEST2".to_string(), "ARRAYS_TEST2".to_string()
        )
        .with_return_type(NativeType::Array { element_type: Box::new(NativeType::Int), size_info: None })
        .with_parameter(NativeParameter::new(
            "outputSize".to_string(), 
            NativeType::Pointer(Box::new(NativeType::Int))
        ));
        let mut function = function;
        function.return_array_length_out_param = Some("outputSize".to_string());
        let function_json = serde_json::to_value(&function).unwrap();
        let mut wrapper = serde_json::Map::new();
        wrapper.insert("function_data".to_string(), function_json.clone());
        wrapper.insert("length_temp_var_name".to_string(), serde_json::Value::String("__outputsize_len_val".to_string()));
        wrapper.insert("pre_call_statements_joined".to_string(), serde_json::Value::String("let mut __outputsize_len_val: i32 = Default::default();".to_string()));
        wrapper.insert("array_inner_rust_type".to_string(), serde_json::Value::String("i32".to_string()));
        let wrapper_value = serde_json::Value::Object(wrapper);
        let result = handlebars.render("function", &wrapper_value);
        println!("[DEBUG TEST] data = {}", serde_json::to_string_pretty(&wrapper_value).unwrap());
        assert!(result.is_ok(), "Failed to render 'function' for array return (corrected): {:?}\nRender data: {:#?}", result.err(), wrapper_value);
        let content = result.unwrap();
        println!("[DEBUG GENERATED CONTENT]\n{}\n[END GENERATED CONTENT]", content);
        assert!(content.contains("pub fn get_array_of_ints_externally_sized_test2_safe("), "Signature not found");
        assert!(content.contains("outputSize: &mut std::os::raw::c_int"), "Param not found");
        assert!(content.contains("-> NativeResult<Vec<i32>>"), "Return type not found");
        assert!(content.contains("let mut __outputsize_len_val: i32 = Default::default();"), "Missing or incorrect temp length var. Got: {}", content);
        assert!(content.contains("let len = __outputsize_len_val as usize;"), "Incorrect usage of temp length var. Got: {}", content);
        assert!(content.contains("std::slice::from_raw_parts(result as *const i32, len)"), "Incorrect slice creation. Got: {}", content);
        assert!(content.contains("slice.iter().cloned().collect::<Vec<_>>()"), "Incorrect Vec collection. Got: {}", content);
    }

    #[test]
    fn test_generate_safe_wrapper_for_array_with_length_param() {
        use handlebars::Handlebars;
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("function", super::RUST_FUNCTION_TEMPLATE).unwrap();
        handlebars.register_helper("snake_case", Box::new(super::snake_case_helper));
        handlebars.register_helper("rust_type", Box::new(super::rust_type_helper));
        handlebars.register_helper("raw_rust_type", Box::new(super::raw_rust_type_helper));
        handlebars.register_helper("ffi_call_param", Box::new(super::ffi_call_param_helper));
        handlebars.register_helper("safe_return_wrap", Box::new(super::safe_return_wrap_helper));
        handlebars.register_helper("is_type_condition", Box::new(super::is_type_condition_helper));
        handlebars.register_helper("is_array_and_has_dynamic_size_param", Box::new(super::is_array_and_has_dynamic_size_param_helper));
        handlebars.register_helper("get_size_param_for_array", Box::new(super::get_size_param_for_array_helper));
        handlebars.register_helper("get_pointed_type_raw_rust_str", Box::new(super::get_pointed_type_raw_rust_str_helper));
        handlebars.register_helper("uppercase", Box::new(super::uppercase_helper));
        handlebars.register_helper("lowercase", Box::new(super::lowercase_helper));
        let function = NativeFunction::new(
            "GET_PLAYERS_TEST1".to_string(), "PLAYER_TEST1".to_string()
        )
        .with_return_type(NativeType::Array { element_type: Box::new(NativeType::Int), size_info: None })
        .with_parameter(NativeParameter::new("length_out".to_string(), NativeType::Pointer(Box::new(NativeType::Int))));
        let mut function = function;
        function.return_array_length_out_param = Some("length_out".to_string());
        let function_json = serde_json::to_value(&function).unwrap();
        let mut wrapper = serde_json::Map::new();
        wrapper.insert("function_data".to_string(), function_json.clone());
        wrapper.insert("length_temp_var_name".to_string(), serde_json::Value::String("__length_out_len_val".to_string()));
        wrapper.insert("pre_call_statements_joined".to_string(), serde_json::Value::String("let mut __length_out_len_val: i32 = Default::default();".to_string()));
        wrapper.insert("array_inner_rust_type".to_string(), serde_json::Value::String("i32".to_string()));
        let wrapper_value = serde_json::Value::Object(wrapper);
        let result = handlebars.render("function", &wrapper_value);
        println!("[DEBUG TEST] data = {}", serde_json::to_string_pretty(&wrapper_value).unwrap());
        assert!(result.is_ok(), "Failed to render 'function' for array return (test_generate_safe_wrapper_for_array_with_length_param): {:?}\nRender data: {:#?}", result.err(), wrapper_value);
        let content = result.unwrap();
        println!("[DEBUG GENERATED CONTENT]\n{}\n[END GENERATED CONTENT]", content);
        assert!(content.contains("pub fn get_players_test1_safe("), "Signature not found");
        assert!(content.contains("length_out: &mut std::os::raw::c_int"), "Param not found");
        assert!(content.contains("-> NativeResult<Vec<i32>>"), "Return type not found");
        assert!(content.contains("let mut __length_out_len_val: i32 = Default::default();"), "Missing or incorrect temp length var. Got: {}", content);
        assert!(content.contains("let len = __length_out_len_val as usize;"), "Incorrect usage of temp length var. Got: {}", content);
        assert!(content.contains("std::slice::from_raw_parts(result as *const i32, len)"), "Incorrect slice creation. Got: {}", content);
        assert!(content.contains("slice.iter().cloned().collect::<Vec<_>>()"), "Incorrect Vec collection. Got: {}", content);
    }

    #[test]
    fn test_generate_safe_wrapper_for_fixed_size_char_array() {
        use handlebars::Handlebars;
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("function", super::RUST_FUNCTION_TEMPLATE).unwrap();
        handlebars.register_helper("snake_case", Box::new(super::snake_case_helper));
        handlebars.register_helper("rust_type", Box::new(super::rust_type_helper));
        handlebars.register_helper("raw_rust_type", Box::new(super::raw_rust_type_helper));
        handlebars.register_helper("ffi_call_param", Box::new(super::ffi_call_param_helper));
        handlebars.register_helper("safe_return_wrap", Box::new(super::safe_return_wrap_helper));
        handlebars.register_helper("is_type_condition", Box::new(super::is_type_condition_helper));
        handlebars.register_helper("is_array_and_has_dynamic_size_param", Box::new(super::is_array_and_has_dynamic_size_param_helper));
        handlebars.register_helper("get_size_param_for_array", Box::new(super::get_size_param_for_array_helper));
        handlebars.register_helper("get_pointed_type_raw_rust_str", Box::new(super::get_pointed_type_raw_rust_str_helper));
        handlebars.register_helper("uppercase", Box::new(super::uppercase_helper));
        handlebars.register_helper("lowercase", Box::new(super::lowercase_helper));
        let function = NativeFunction::new(
            "GET_LABEL".to_string(), "STRINGS".to_string()
        )
        .with_return_type(NativeType::Array { element_type: Box::new(NativeType::Char), size_info: Some(crate::native_types::ArraySizeInfo::Known(32)) })
        .with_parameter(NativeParameter::new("buffer".to_string(), NativeType::Array { element_type: Box::new(NativeType::Char), size_info: Some(crate::native_types::ArraySizeInfo::Known(32)) }));
        let function_json = serde_json::to_value(&function).unwrap();
        let mut wrapper = serde_json::Map::new();
        wrapper.insert("function_data".to_string(), function_json.clone());
        let wrapper_value = serde_json::Value::Object(wrapper);
        let result = handlebars.render("function", &wrapper_value);
        let content = result.unwrap();
        println!("[DEBUG GENERATED FUNCTION]\n{}\n[END GENERATED FUNCTION]", content);
        assert!(content.contains("pub fn get_label_safe("));
        assert!(content.contains("buffer: [u8; 32]"));
        assert!(content.contains("-> NativeResult<[u8; 32]>"));
    }

    #[test]
    fn test_generate_safe_wrapper_for_fixed_size_string_array() {
        use handlebars::Handlebars;
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("function", super::RUST_FUNCTION_TEMPLATE).unwrap();
        handlebars.register_helper("snake_case", Box::new(super::snake_case_helper));
        handlebars.register_helper("rust_type", Box::new(super::rust_type_helper));
        handlebars.register_helper("raw_rust_type", Box::new(super::raw_rust_type_helper));
        handlebars.register_helper("ffi_call_param", Box::new(super::ffi_call_param_helper));
        handlebars.register_helper("safe_return_wrap", Box::new(super::safe_return_wrap_helper));
        handlebars.register_helper("is_type_condition", Box::new(super::is_type_condition_helper));
        handlebars.register_helper("is_array_and_has_dynamic_size_param", Box::new(super::is_array_and_has_dynamic_size_param_helper));
        handlebars.register_helper("get_size_param_for_array", Box::new(super::get_size_param_for_array_helper));
        handlebars.register_helper("get_pointed_type_raw_rust_str", Box::new(super::get_pointed_type_raw_rust_str_helper));
        handlebars.register_helper("uppercase", Box::new(super::uppercase_helper));
        handlebars.register_helper("lowercase", Box::new(super::lowercase_helper));
        let function = NativeFunction::new(
            "GET_NAMES".to_string(), "STRINGS".to_string()
        )
        .with_return_type(NativeType::Array { element_type: Box::new(NativeType::String), size_info: Some(crate::native_types::ArraySizeInfo::Known(4)) })
        .with_parameter(NativeParameter::new("names".to_string(), NativeType::Array { element_type: Box::new(NativeType::String), size_info: Some(crate::native_types::ArraySizeInfo::Known(4)) }));
        let function_json = serde_json::to_value(&function).unwrap();
        let mut wrapper = serde_json::Map::new();
        wrapper.insert("function_data".to_string(), function_json.clone());
        let wrapper_value = serde_json::Value::Object(wrapper);
        let result = handlebars.render("function", &wrapper_value);
        let content = result.unwrap();
        assert!(content.contains("pub fn get_names_safe("));
        assert!(content.contains("names: [String; 4]"));
        assert!(content.contains("-> NativeResult<[String; 4]>"));
    }

    #[test]
    fn test_generate_safe_wrapper_for_fixed_size_struct_array() {
        use handlebars::Handlebars;
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("function", super::RUST_FUNCTION_TEMPLATE).unwrap();
        handlebars.register_helper("snake_case", Box::new(super::snake_case_helper));
        handlebars.register_helper("rust_type", Box::new(super::rust_type_helper));
        handlebars.register_helper("raw_rust_type", Box::new(super::raw_rust_type_helper));
        handlebars.register_helper("ffi_call_param", Box::new(super::ffi_call_param_helper));
        handlebars.register_helper("safe_return_wrap", Box::new(super::safe_return_wrap_helper));
        handlebars.register_helper("is_type_condition", Box::new(super::is_type_condition_helper));
        handlebars.register_helper("is_array_and_has_dynamic_size_param", Box::new(super::is_array_and_has_dynamic_size_param_helper));
        handlebars.register_helper("get_size_param_for_array", Box::new(super::get_size_param_for_array_helper));
        handlebars.register_helper("get_pointed_type_raw_rust_str", Box::new(super::get_pointed_type_raw_rust_str_helper));
        handlebars.register_helper("uppercase", Box::new(super::uppercase_helper));
        handlebars.register_helper("lowercase", Box::new(super::lowercase_helper));
        let function = NativeFunction::new(
            "GET_POSITIONS".to_string(), "VEC3".to_string()
        )
        .with_return_type(NativeType::Array { element_type: Box::new(NativeType::Vector3), size_info: Some(crate::native_types::ArraySizeInfo::Known(8)) })
        .with_parameter(NativeParameter::new("positions".to_string(), NativeType::Array { element_type: Box::new(NativeType::Vector3), size_info: Some(crate::native_types::ArraySizeInfo::Known(8)) }));
        let function_json = serde_json::to_value(&function).unwrap();
        let mut wrapper = serde_json::Map::new();
        wrapper.insert("function_data".to_string(), function_json.clone());
        let wrapper_value = serde_json::Value::Object(wrapper);
        let result = handlebars.render("function", &wrapper_value);
        let content = result.unwrap();
        assert!(content.contains("pub fn get_positions_safe("));
        assert!(content.contains("positions: [Vector3; 8]"));
        assert!(content.contains("-> NativeResult<[Vector3; 8]>"));
    }
} 