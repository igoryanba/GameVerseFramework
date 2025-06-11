use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::warn;

/// Represents a collection of native functions organized by categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeCollection {
    pub categories: HashMap<String, NativeCategory>,
    pub metadata: CollectionMetadata,
}

/// Metadata about the entire native function collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionMetadata {
    pub source: String,
    pub version: String,
    pub generated_at: String,
    pub total_functions: usize,
    pub categories_count: usize,
}

/// A category of native functions (e.g., PLAYER, VEHICLE, PED)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeCategory {
    pub name: String,
    pub description: Option<String>,
    pub functions: Vec<NativeFunction>,
}

/// Represents a single native function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeFunction {
    pub name: String,
    pub hash: Option<String>,
    pub description: Option<String>,
    pub return_type: NativeType,
    pub typescript_name_override: Option<String>,
    pub typescript_return_type_override: Option<String>,
    pub parameters: Vec<NativeParameter>,
    pub category: String,
    pub examples: Vec<String>,
    pub deprecated: bool,
    pub platform_specific: Option<Vec<Platform>>,
    pub raw_data: HashMap<String, String>,
    pub return_array_length_out_param: Option<String>,

    // Новые поля для Rust-специфичных переопределений из native_configs.toml
    pub rust_mark_safe_wrapper_unsafe: Option<bool>,
    pub rust_prologue_code: Option<String>,
    pub rust_epilogue_code: Option<String>,
    pub rust_return_type_override: Option<String>,
}

/// A parameter for a native function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeParameter {
    pub name: String,
    pub param_type: NativeType,
    pub description: Option<String>,
    pub typescript_name_override: Option<String>,
    pub typescript_type_override: Option<String>,
    pub optional: bool,
    pub default_value: Option<String>,

    // Новые поля для Rust-специфичных переопределений из native_configs.toml
    pub rust_new_name: Option<String>,
    pub rust_type_override: Option<String>, // Строка с типом из конфига
    pub rust_any_type_hint: Option<String>,
    pub rust_transform_input_with: Option<String>,
    pub rust_default_value_for_optional: Option<String>,
}

/// Информация о размере массива
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ArraySizeInfo {
    /// Фиксированный размер, известный на этапе компиляции
    Known(usize),
    /// Динамический размер, получаемый через именованный out-параметр
    Dynamic { size_param_name: String },
    /// Параметр по индексу, содержащий размер
    SizeParamIndex(usize),
    /// Null-terminated массив (для строк)
    NullTerminated,
    /// Размер должен быть определен из контекста
    Infer,
}

/// Native function types mapped to safe Rust types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NativeType {
    // Primitive types
    Void,
    Bool,
    Int,
    Float,
    String,
    
    // Game-specific types
    Player,
    Ped,
    Vehicle,
    Entity,
    Object,
    Blip,
    Cam,
    Hash,
    Vector3,
    FireId,
    Interior,
    ItemSet,
    Pickup,
    Char, // For single characters, often used in char[]
    
    // Collections
    Array(Box<NativeType>, Option<ArraySizeInfo>),
    
    // Pointers and references
    Pointer(Box<NativeType>),
    Reference(Box<NativeType>),
    
    // Unknown/Any type for difficult cases
    Any(Option<String>), // Теперь может содержать описание или подсказку о типе
    FunctionCallback(Option<String>), // Опционально содержит строковое представление сигнатуры
    Opaque(Option<String>), // Представляет непрозрачную структуру или неизвестный тип с опциональным именем из C
}

/// Supported platforms for native functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Platform {
    PC,
    Console,
    All,
}

impl NativeCollection {
    pub fn new() -> Self {
        Self {
            categories: HashMap::new(),
            metadata: CollectionMetadata {
                source: String::new(),
                version: "1.0.0".to_string(),
                generated_at: chrono::Utc::now().to_rfc3339(),
                total_functions: 0,
                categories_count: 0,
            },
        }
    }
    
    pub fn add_category(&mut self, category: NativeCategory) {
        self.metadata.total_functions += category.functions.len();
        self.categories.insert(category.name.clone(), category);
        self.metadata.categories_count = self.categories.len();
    }
    
    pub fn total_functions(&self) -> usize {
        self.metadata.total_functions
    }
    
    pub fn categories(&self) -> Vec<&String> {
        self.categories.keys().collect()
    }
    
    pub fn filter_categories(&self, category_names: &[String]) -> Self {
        let mut filtered = Self::new();
        filtered.metadata = self.metadata.clone();
        
        for name in category_names {
            if let Some(category) = self.categories.get(name) {
                filtered.add_category(category.clone());
            }
        }
        
        filtered
    }
    
    #[allow(dead_code)]
    pub fn get_category(&self, name: &str) -> Option<&NativeCategory> {
        self.categories.get(name)
    }
    
    #[allow(dead_code)]
    pub fn get_function(&self, category: &str, function_name: &str) -> Option<&NativeFunction> {
        self.categories
            .get(category)?
            .functions
            .iter()
            .find(|f| f.name == function_name)
    }
}

impl NativeFunction {
    pub fn new(name: String, category: String) -> Self {
        Self {
            name,
            category,
            hash: None,
            description: None,
            return_type: NativeType::Void,
            typescript_name_override: None,
            typescript_return_type_override: None,
            parameters: Vec::new(),
            examples: Vec::new(),
            deprecated: false,
            platform_specific: None,
            raw_data: HashMap::new(),
            return_array_length_out_param: None,
            rust_mark_safe_wrapper_unsafe: None,
            rust_prologue_code: None,
            rust_epilogue_code: None,
            rust_return_type_override: None,
        }
    }
    
    pub fn with_return_type(mut self, return_type: NativeType) -> Self {
        self.return_type = return_type;
        self
    }
    
    #[allow(dead_code)]
    pub fn with_parameter(mut self, param: NativeParameter) -> Self {
        self.parameters.push(param);
        self
    }
    
    #[allow(dead_code)]
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    pub fn with_hash(mut self, hash: String) -> Self {
        self.hash = Some(hash);
        self
    }
    
    /// Generate a safe Rust function name from the native name
    #[allow(dead_code)]
    pub fn rust_function_name(&self) -> String {
        self.name.to_lowercase()
    }
    
    /// Generate a TypeScript function name
    #[allow(dead_code)]
    pub fn typescript_function_name(&self) -> String {
        // Convert GET_PLAYER_PED to getPlayerPed
        let words: Vec<&str> = self.name.split('_').collect();
        if words.is_empty() {
            return self.name.clone();
        }
        
        let mut result = words[0].to_lowercase();
        for word in &words[1..] {
            if !word.is_empty() {
                result.push_str(&format!("{}{}", 
                    word.chars().next().unwrap().to_uppercase(),
                    word[1..].to_lowercase()
                ));
            }
        }
        
        result
    }
    
    /// Автоматически определяет параметры размера для всех массивов в функции
    pub fn detect_array_size_params(&mut self) {
        // Сначала ищем параметры с массивами
        let mut array_params = Vec::new();
        
        for (i, param) in self.parameters.iter().enumerate() {
            if let NativeType::Array(_, size_info) = &param.param_type {
                if size_info.is_none() {
                    array_params.push((i, param.name.clone()));
                }
            }
        }
        
        // Затем для каждого массива без размера ищем параметр размера
        for (array_idx, array_name) in array_params {
            // Ищем возможные параметры размера
            let possible_size_params = self.find_possible_size_params(&array_name);
            
            if !possible_size_params.is_empty() {
                // Берем первый найденный параметр размера
                let (size_param_idx, size_param_name) = possible_size_params[0].clone();
                
                // Обновляем тип массива с информацией о размере
                if let NativeType::Array(inner_type, size_info) = &mut self.parameters[array_idx].param_type {
                    *size_info = Some(ArraySizeInfo::Dynamic { 
                        size_param_name: size_param_name.clone() 
                    });
                    
                    // Для логирования
                    warn!("Автоматически определен параметр размера '{}' для массива '{}' в функции '{}'", 
                          size_param_name, array_name, self.name);
                }
            }
        }
        
        // Проверяем возвращаемое значение, если это массив
        if let NativeType::Array(_, size_info) = &mut self.return_type {
            if size_info.is_none() && self.return_array_length_out_param.is_some() {
                // Устанавливаем информацию о размере возвращаемого массива
                *size_info = Some(ArraySizeInfo::Dynamic { 
                    size_param_name: self.return_array_length_out_param.clone().unwrap() 
                });
                
                warn!("Установлен параметр размера '{}' для возвращаемого массива в функции '{}'", 
                      self.return_array_length_out_param.clone().unwrap(), self.name);
            }
        }
    }
    
    /// Находит возможные параметры размера для массива
    fn find_possible_size_params(&self, array_name: &str) -> Vec<(usize, String)> {
        let mut possible_params = Vec::new();
        
        // Извлекаем базовое имя массива (без индексов или суффиксов)
        let base_name = array_name.split('_').next().unwrap_or(array_name);
        
        for (i, param) in self.parameters.iter().enumerate() {
            let param_name = &param.name;
            
            // Проверяем различные шаблоны именования параметров размера
            let is_size_param = 
                // Проверяем точное совпадение шаблонов
                param_name == &format!("{}_size", base_name) ||
                param_name == &format!("{}_length", base_name) ||
                param_name == &format!("{}_count", base_name) ||
                // Проверяем общие шаблоны
                param_name.contains("size") ||
                param_name.contains("length") ||
                param_name.contains("count") ||
                // Проверяем короткие имена
                param_name == "size" ||
                param_name == "length" ||
                param_name == "count" ||
                param_name == "n";
            
            if is_size_param && matches!(param.param_type, NativeType::Int) {
                possible_params.push((i, param_name.clone()));
            }
        }
        
        possible_params
    }
    
    /// Применяет переопределения из конфигурационного файла
    pub fn apply_overrides(&mut self, overrides: &HashMap<String, serde_json::Value>) {
        if let Some(function_overrides) = overrides.get(&self.name) {
            // Здесь применяем различные переопределения к функции
            if let Some(return_type_override) = function_overrides.get("return_type_override") {
                if let Some(return_type_str) = return_type_override.as_str() {
                    self.rust_return_type_override = Some(return_type_str.to_string());
                }
            }
            
            // Применяем переопределения к параметрам
            if let Some(param_overrides) = function_overrides.get("parameter_rust_overrides") {
                if let Some(param_overrides_array) = param_overrides.as_array() {
                    for param_override in param_overrides_array {
                        if let Some(original_name) = param_override.get("original_name").and_then(|v| v.as_str()) {
                            // Находим параметр по имени
                            if let Some(param) = self.parameters.iter_mut().find(|p| p.name == original_name) {
                                // Применяем переопределения к параметру
                                if let Some(new_name) = param_override.get("rust_new_name").and_then(|v| v.as_str()) {
                                    param.rust_new_name = Some(new_name.to_string());
                                }
                                
                                if let Some(type_override) = param_override.get("rust_type_override").and_then(|v| v.as_str()) {
                                    param.rust_type_override = Some(type_override.to_string());
                                }
                                
                                if let Some(any_hint) = param_override.get("rust_any_type_hint").and_then(|v| v.as_str()) {
                                    param.rust_any_type_hint = Some(any_hint.to_string());
                                }
                                
                                if let Some(transform) = param_override.get("rust_transform_input_with").and_then(|v| v.as_str()) {
                                    param.rust_transform_input_with = Some(transform.to_string());
                                }
                                
                                if let Some(default_value) = param_override.get("rust_default_value_for_optional").and_then(|v| v.as_str()) {
                                    param.rust_default_value_for_optional = Some(default_value.to_string());
                                    param.optional = true; // Если есть значение по умолчанию, параметр опциональный
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl NativeParameter {
    pub fn new(name: String, param_type: NativeType) -> Self {
        Self {
            name,
            param_type,
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
        }
    }
    
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
    
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    pub fn with_default(mut self, default: String) -> Self {
        self.default_value = Some(default);
        self
    }
}

impl NativeType {
    /// Parse a FiveM native type into our internal representation
    pub fn from_fivem_type(type_str: &str) -> Self {
        let type_str = type_str.trim();
        
        if type_str.is_empty() || type_str == "void" {
            return NativeType::Void;
        }
        
        // Handle arrays with size: Type[Size]
        if let Some(array_match) = type_str.find('[') {
            if type_str.ends_with(']') {
                let base_type = &type_str[0..array_match].trim();
                let size_str = &type_str[array_match+1..type_str.len()-1].trim();
                
                let inner_type = NativeType::from_fivem_type(base_type);
                
                // Парсинг размера массива
                let array_size_info = if size_str.is_empty() {
                    // Пустой размер: тип[]
                    Some(ArraySizeInfo::Infer)
                } else if let Ok(size) = size_str.parse::<usize>() {
                    // Фиксированный размер: тип[42]
                    Some(ArraySizeInfo::Known(size))
                } else {
                    // Здесь могут быть другие случаи, например, тип[MAX_SIZE]
                    // Пока просто помечаем как Infer
                    Some(ArraySizeInfo::Infer)
                };
                
                return NativeType::Array(Box::new(inner_type), array_size_info);
            }
        }
        
        // Handle pointers and references
        if type_str.ends_with('*') {
            let inner_type = NativeType::from_fivem_type(&type_str[0..type_str.len()-1].trim());
            return NativeType::Pointer(Box::new(inner_type));
        }
        
        if type_str.ends_with('&') {
            let inner_type = NativeType::from_fivem_type(&type_str[0..type_str.len()-1].trim());
            return NativeType::Reference(Box::new(inner_type));
        }
        
        // Handle char* as String
        if type_str == "char*" || type_str == "const char*" {
            return NativeType::String;
        }
        
        // Handle char as Char
        if type_str == "char" {
            return NativeType::Char;
        }
        
        // Handle known types
        match type_str {
            "bool" => NativeType::Bool,
            "int" | "long" | "short" | "BOOL" => NativeType::Int,
            "float" | "double" => NativeType::Float,
            "Vector3" => NativeType::Vector3,
            "Entity" => NativeType::Entity,
            "Ped" => NativeType::Ped,
            "Vehicle" => NativeType::Vehicle,
            "Player" => NativeType::Player,
            "Object" => NativeType::Object,
            "Blip" => NativeType::Blip,
            "Cam" => NativeType::Cam,
            "Hash" | "uint" => NativeType::Hash,
            "FireId" => NativeType::FireId,
            "Interior" => NativeType::Interior,
            "ItemSet" => NativeType::ItemSet,
            "Pickup" => NativeType::Pickup,
            "Any" => NativeType::Any(None),
            "FUNC_..." | "callback" => NativeType::FunctionCallback(None),
            // Обработка неизвестных типов (они могут быть typedef'ами)
            _ => {
                if type_str.contains("callback") || type_str.contains("FUNC") {
                    NativeType::FunctionCallback(Some(type_str.to_string()))
                } else {
                    // Opaque используется для типов, которые мы не можем точно определить
                    // но хотим сохранить их имя для потенциального использования
                    NativeType::Opaque(Some(type_str.to_string()))
                }
            }
        }
    }
    
    /// Convert our internal type to a Rust type string for safe wrappers
    pub fn to_rust_type_string(&self) -> String {
        match self {
            NativeType::Void => "()".to_string(),
            NativeType::Bool => "bool".to_string(),
            NativeType::Int => "i64".to_string(), // Используем i64 для совместимости с JavaScript/TypeScript
            NativeType::Float => "f32".to_string(),
            NativeType::String => "String".to_string(),
            NativeType::Player => "crate::types::PlayerId".to_string(),
            NativeType::Ped => "crate::types::PedEntity".to_string(),
            NativeType::Vehicle => "crate::types::VehicleEntity".to_string(),
            NativeType::Entity => "crate::types::EntityId".to_string(),
            NativeType::Object => "crate::types::ObjectEntity".to_string(),
            NativeType::Blip => "crate::types::BlipId".to_string(),
            NativeType::Cam => "crate::types::CameraId".to_string(),
            NativeType::Hash => "u32".to_string(),
            NativeType::Vector3 => "Vector3<f32>".to_string(),
            NativeType::FireId => "i64".to_string(),
            NativeType::Interior => "i64".to_string(),
            NativeType::ItemSet => "i64".to_string(),
            NativeType::Pickup => "i64".to_string(),
            NativeType::Char => "char".to_string(),
            
            NativeType::Array(inner_type, size_info) => {
                let inner_rust_type = inner_type.to_rust_type_string();
                
                match size_info {
                    Some(ArraySizeInfo::Known(size)) => {
                        // Фиксированный размер массива: [Type; N]
                        format!("[{}; {}]", inner_rust_type, size)
                    },
                    _ => {
                        // Динамический размер: Vec<Type>
                        format!("Vec<{}>", inner_rust_type)
                    }
                }
            },
            
            NativeType::Pointer(inner_type) => {
                match **inner_type {
                    NativeType::Void => "*mut std::ffi::c_void".to_string(),
                    _ => format!("*mut {}", inner_type.to_rust_type_string())
                }
            },
            
            NativeType::Reference(inner_type) => {
                format!("&mut {}", inner_type.to_rust_type_string())
            },
            
            NativeType::Any(hint) => {
                if let Some(hint_str) = hint {
                    format!("/* {} */ serde_json::Value", hint_str)
                } else {
                    "serde_json::Value".to_string()
                }
            },
            
            NativeType::FunctionCallback(signature) => {
                if let Some(sig) = signature {
                    format!("/* {} */ Box<dyn Fn(serde_json::Value) -> serde_json::Value>", sig)
                } else {
                    "Box<dyn Fn(serde_json::Value) -> serde_json::Value>".to_string()
                }
            },
            
            NativeType::Opaque(name) => {
                if let Some(type_name) = name {
                    // Можно создать псевдоним для опакового типа
                    format!("/* {} */ *mut std::ffi::c_void", type_name)
                } else {
                    "*mut std::ffi::c_void".to_string()
                }
            }
        }
    }
    
    /// Convert our internal type to a raw Rust FFI type string for unsafe wrappers
    pub fn to_raw_rust_type_string(&self) -> String {
        match self {
            NativeType::Void => "std::ffi::c_void".to_string(),
            NativeType::Bool => "bool".to_string(),
            NativeType::Int => "i64".to_string(),
            NativeType::Float => "f32".to_string(),
            NativeType::String => "*const std::os::raw::c_char".to_string(),
            NativeType::Player => "i32".to_string(),
            NativeType::Ped => "i32".to_string(),
            NativeType::Vehicle => "i32".to_string(),
            NativeType::Entity => "i32".to_string(),
            NativeType::Object => "i32".to_string(),
            NativeType::Blip => "i32".to_string(),
            NativeType::Cam => "i32".to_string(),
            NativeType::Hash => "u32".to_string(),
            NativeType::Vector3 => "*mut Vector3_raw".to_string(),
            NativeType::FireId => "i64".to_string(),
            NativeType::Interior => "i64".to_string(),
            NativeType::ItemSet => "i64".to_string(),
            NativeType::Pickup => "i64".to_string(),
            NativeType::Char => "std::os::raw::c_char".to_string(),
            
            NativeType::Array(inner_type, _) => {
                format!("*mut {}", inner_type.to_raw_rust_type_string())
            },
            
            NativeType::Pointer(_) | NativeType::Reference(_) | NativeType::Opaque(_) => {
                "*mut std::os::raw::c_void".to_string()
            },
            
            NativeType::Any(_) => "*mut std::os::raw::c_void".to_string(),
            
            NativeType::FunctionCallback(_) => {
                // Для функциональных указателей используем обобщенный тип функции
                "extern \"C\" fn(*mut std::os::raw::c_void) -> *mut std::os::raw::c_void".to_string()
            }
        }
    }
    
    /// Проверяет, требует ли тип валидации перед использованием
    pub fn requires_validation(&self) -> bool {
        match self {
            NativeType::Player | 
            NativeType::Ped |
            NativeType::Vehicle |
            NativeType::Entity |
            NativeType::Object |
            NativeType::Blip |
            NativeType::Cam |
            NativeType::Vector3 => true,
            _ => false
        }
    }
}

impl Default for NativeParameter {
    fn default() -> Self {
        Self {
            name: String::new(),
            param_type: NativeType::Any(None),
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
        }
    }
}

impl Default for NativeFunction {
    fn default() -> Self {
        Self {
            name: String::new(),
            hash: None,
            description: None,
            return_type: NativeType::Void,
            typescript_name_override: None,
            typescript_return_type_override: None,
            parameters: Vec::new(),
            category: "UNKNOWN".to_string(),
            examples: Vec::new(),
            deprecated: false,
            platform_specific: None,
            raw_data: HashMap::new(),
            return_array_length_out_param: None,
            rust_mark_safe_wrapper_unsafe: None,
            rust_prologue_code: None,
            rust_epilogue_code: None,
            rust_return_type_override: None,
        }
    }
}

// Для обратной совместимости
impl NativeFunction {
    pub fn detect_array_size_param(&mut self) {
        self.detect_array_size_params();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_native_type_conversion() {
        assert_eq!(NativeType::from_fivem_type("int"), NativeType::Int);
        assert_eq!(NativeType::from_fivem_type("char*"), NativeType::String);
        assert_eq!(NativeType::from_fivem_type("Vector3"), NativeType::Vector3);
        assert_eq!(NativeType::from_fivem_type("int*"), NativeType::Pointer(Box::new(NativeType::Int)));
        assert_eq!(NativeType::from_fivem_type("float&"), NativeType::Reference(Box::new(NativeType::Float)));
    }
    
    #[test]
    fn test_rust_type_generation() {
        assert_eq!(NativeType::Int.to_rust_type_string(), "i64");
        assert_eq!(NativeType::String.to_rust_type_string(), "String");
        assert_eq!(NativeType::Vector3.to_rust_type_string(), "Vector3<f32>");
        assert_eq!(NativeType::Pointer(Box::new(NativeType::Int)).to_rust_type_string(), "*mut i64");
    }
    
    #[test]
    fn test_function_name_generation() {
        let func = NativeFunction::new("GET_PLAYER_PED".to_string(), "PLAYER".to_string());
        assert_eq!(func.typescript_function_name(), "getPlayerPed");
    }
    
    #[test]
    fn test_collection_management() {
        let mut collection = NativeCollection::new();
        
        let category = NativeCategory {
            name: "TEST".to_string(),
            description: None,
            functions: vec![
                NativeFunction::new("TEST_FUNC".to_string(), "TEST".to_string())
            ],
        };
        
        collection.add_category(category);
        
        assert_eq!(collection.total_functions(), 1);
        assert_eq!(collection.categories().len(), 1);
        assert!(collection.get_category("TEST").is_some());
        assert!(collection.get_function("TEST", "TEST_FUNC").is_some());
    }
    
    #[test]
    fn test_from_fivem_type_advanced() {
        // Проверка различных вариантов массивов
        if let NativeType::Array(inner, size_info) = NativeType::from_fivem_type("int[5]") {
            assert_eq!(*inner, NativeType::Int);
            assert_eq!(size_info, Some(ArraySizeInfo::Known(5)));
        } else {
            panic!("Expected Array type");
        }
        
        if let NativeType::Array(inner, size_info) = NativeType::from_fivem_type("float[]") {
            assert_eq!(*inner, NativeType::Float);
            assert_eq!(size_info, Some(ArraySizeInfo::Infer));
        } else {
            panic!("Expected Array type");
        }
        
        // Проверка преобразования функциональных типов
        if let NativeType::FunctionCallback(sig) = NativeType::from_fivem_type("FUNC_CALLBACK") {
            assert_eq!(sig, None);
        } else {
            panic!("Expected FunctionCallback type");
        }
        
        if let NativeType::FunctionCallback(sig) = NativeType::from_fivem_type("MyCustomCallback") {
            assert_eq!(sig, Some("MyCustomCallback".to_string()));
        } else {
            panic!("Expected FunctionCallback type with signature");
        }
    }
    
    #[test]
    fn test_detect_array_size_params() {
        // Создаем функцию с массивом и параметром размера
        let mut func = NativeFunction {
            name: "TEST_SIZE_DETECTION".to_string(),
            parameters: vec![
                NativeParameter::new(
                    "data".to_string(), 
                    NativeType::Array(Box::new(NativeType::Int), None)
                ),
                NativeParameter::new(
                    "data_size".to_string(), 
                    NativeType::Int
                ),
            ],
            ..Default::default()
        };
        
        // Запускаем автоопределение параметров размера
        func.detect_array_size_params();
        
        // Проверяем, что размер был правильно определен
        if let NativeType::Array(_, size_info) = &func.parameters[0].param_type {
            assert!(size_info.is_some());
            if let Some(ArraySizeInfo::Dynamic { size_param_name }) = size_info {
                assert_eq!(size_param_name, "data_size");
            } else {
                panic!("Expected Dynamic size info");
            }
        } else {
            panic!("Expected Array type");
        }
    }
    
    #[test]
    fn test_array_size_info() {
        // Тест с фиксированным размером
        let array_fixed = NativeType::Array(
            Box::new(NativeType::Int), 
            Some(ArraySizeInfo::Known(10))
        );
        assert_eq!(array_fixed.to_rust_type_string(), "[i64; 10]");
        
        // Тест с динамическим размером
        let array_dynamic = NativeType::Array(
            Box::new(NativeType::String), 
            Some(ArraySizeInfo::Dynamic { size_param_name: "count".to_string() })
        );
        assert_eq!(array_dynamic.to_rust_type_string(), "Vec<String>");
        
        // Тест с null-terminated массивом
        let array_null_term = NativeType::Array(
            Box::new(NativeType::Char), 
            Some(ArraySizeInfo::NullTerminated)
        );
        assert_eq!(array_null_term.to_rust_type_string(), "Vec<char>");
    }
} 