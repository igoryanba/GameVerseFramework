//! # Native Function Registry
//!
//! Реестр для хранения информации о нативных функциях.

use anyhow::{Result, bail};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Реестр нативных функций
#[derive(Debug, Default)]
pub struct NativeRegistry {
    /// Функции по хешу
    functions_by_hash: HashMap<u64, NativeFunctionInfo>,
    /// Функции по имени
    functions_by_name: HashMap<String, u64>,
}

/// Информация о нативной функции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeFunctionInfo {
    /// Хеш функции
    pub hash: u64,
    /// Имя функции
    pub name: String,
    /// Параметры функции
    pub parameters: Vec<String>,
    /// Тип возвращаемого значения
    pub return_type: String,
    /// Описание функции (если есть)
    pub description: Option<String>,
    /// Примеры использования
    pub examples: Vec<String>,
}

impl NativeRegistry {
    /// Создать новый реестр
    pub fn new() -> Self {
        Self::default()
    }

    /// Зарегистрировать функцию
    pub fn register_function(
        &mut self,
        hash: u64,
        name: String,
        parameters: Vec<String>,
        return_type: String,
    ) -> Result<()> {
        // Проверяем, что функция еще не зарегистрирована
        if self.functions_by_hash.contains_key(&hash) {
            bail!("Function with hash 0x{:016X} already registered", hash);
        }

        if self.functions_by_name.contains_key(&name) {
            bail!("Function with name '{}' already registered", name);
        }

        let function_info = NativeFunctionInfo {
            hash,
            name: name.clone(),
            parameters,
            return_type,
            description: None,
            examples: Vec::new(),
        };

        self.functions_by_hash.insert(hash, function_info);
        self.functions_by_name.insert(name, hash);

        Ok(())
    }

    /// Получить функцию по хешу
    pub fn get_function_by_hash(&self, hash: u64) -> Option<&NativeFunctionInfo> {
        self.functions_by_hash.get(&hash)
    }

    /// Получить функцию по имени
    pub fn get_function_by_name(&self, name: &str) -> Option<&NativeFunctionInfo> {
        if let Some(&hash) = self.functions_by_name.get(name) {
            self.functions_by_hash.get(&hash)
        } else {
            None
        }
    }

    /// Получить все функции
    pub fn get_all_functions(&self) -> Vec<&NativeFunctionInfo> {
        self.functions_by_hash.values().collect()
    }

    /// Получить количество зарегистрированных функций
    pub fn count(&self) -> usize {
        self.functions_by_hash.len()
    }

    /// Очистить реестр
    pub fn clear(&mut self) {
        self.functions_by_hash.clear();
        self.functions_by_name.clear();
    }

    /// Загрузить функции из JSON файла
    pub fn load_from_json(&mut self, json_data: &str) -> Result<usize> {
        let functions: Vec<NativeFunctionInfo> = serde_json::from_str(json_data)?;
        let mut loaded_count = 0;

        for function in functions {
            if self.register_function(
                function.hash,
                function.name.clone(),
                function.parameters.clone(),
                function.return_type.clone(),
            ).is_ok() {
                // Обновляем дополнительную информацию
                if let Some(func) = self.functions_by_hash.get_mut(&function.hash) {
                    func.description = function.description;
                    func.examples = function.examples;
                }
                loaded_count += 1;
            }
        }

        Ok(loaded_count)
    }

    /// Сохранить функции в JSON
    pub fn save_to_json(&self) -> Result<String> {
        let functions: Vec<&NativeFunctionInfo> = self.functions_by_hash.values().collect();
        Ok(serde_json::to_string_pretty(&functions)?)
    }
} 