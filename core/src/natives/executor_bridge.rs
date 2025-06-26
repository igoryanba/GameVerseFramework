//! # Executor Bridge
//!
//! Мост между реестром нативных функций и исполнителем.

use anyhow::{Result, bail, Context};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, warn};

use crate::game_integration::{GameType, native_executor::{NativeExecutor, NativeValue}};
use super::registry::NativeFunctionInfo;

/// Мост к исполнителю нативных функций
#[derive(Debug)]
pub struct ExecutorBridge {
    /// Исполнитель нативов
    native_executor: Arc<RwLock<NativeExecutor>>,
    /// Тип игры
    game_type: GameType,
    /// Кэш функций по имени
    name_to_hash: HashMap<String, u64>,
}

impl ExecutorBridge {
    /// Создать новый мост
    pub async fn new(native_executor: Arc<RwLock<NativeExecutor>>, game_type: GameType) -> Result<Self> {
        Ok(Self {
            native_executor,
            game_type,
            name_to_hash: HashMap::new(),
        })
    }

    /// Зарегистрировать функцию в мосту
    pub fn register_function(&mut self, function: NativeFunctionInfo) -> Result<()> {
        self.name_to_hash.insert(function.name.clone(), function.hash);
        debug!("Registered native function: {} (0x{:016X})", function.name, function.hash);
        Ok(())
    }

    /// Вызвать функцию по имени
    pub async fn call_by_name(&self, name: &str, args: Vec<NativeValue>) -> Result<NativeValue> {
        let hash = self.name_to_hash.get(name)
            .ok_or_else(|| anyhow::anyhow!("Native function '{}' not found", name))?;

        self.call_by_hash(*hash, args).await
            .with_context(|| format!("Failed to call native function '{}'", name))
    }

    /// Вызвать функцию по хешу
    pub async fn call_by_hash(&self, hash: u64, args: Vec<NativeValue>) -> Result<NativeValue> {
        debug!("Calling native 0x{:016X} with {} args", hash, args.len());

        // Проверяем, что исполнитель инициализирован
        if !self.native_executor.read().await.is_initialized() {
            bail!("Native executor not initialized");
        }

        // Выполняем нативную функцию
        self.native_executor.read().await.execute_native(hash, args).await
            .with_context(|| format!("Failed to execute native 0x{:016X}", hash))
    }

    /// Получить информацию о функции из исполнителя
    pub async fn get_native_info(&self, hash: u64) -> Option<crate::game_integration::native_executor::NativeFunction> {
        self.native_executor.read().await.get_native_info(hash).cloned()
    }

    /// Получить количество зарегистрированных функций
    pub fn function_count(&self) -> usize {
        self.name_to_hash.len()
    }

    /// Проверить, зарегистрирована ли функция
    pub fn has_function(&self, name: &str) -> bool {
        self.name_to_hash.contains_key(name)
    }

    /// Получить хеш функции по имени
    pub fn get_hash_by_name(&self, name: &str) -> Option<u64> {
        self.name_to_hash.get(name).copied()
    }
} 