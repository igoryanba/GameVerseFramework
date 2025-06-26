//! # Native Functions Integration
//!
//! Интеграция с генератором нативных функций GameVerse.
//! Связывает сгенерированные Rust wrappers с runtime исполнителем.

pub mod registry;
pub mod wrapper;
pub mod executor_bridge;

use anyhow::{Result, Context};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};

use crate::game_integration::{GameType, native_executor::{NativeExecutor, NativeValue}};

/// Результат выполнения нативной функции
pub type NativeResult = Result<NativeValue>;

/// Менеджер нативных функций
#[derive(Debug)]
pub struct NativeManager {
    /// Реестр нативных функций
    registry: registry::NativeRegistry,
    /// Мост к исполнителю
    executor_bridge: Option<Arc<RwLock<executor_bridge::ExecutorBridge>>>,
    /// Тип игры
    game_type: GameType,
    /// Состояние инициализации
    initialized: bool,
}

impl NativeManager {
    /// Создать новый менеджер нативов
    pub fn new(game_type: GameType) -> Self {
        Self {
            registry: registry::NativeRegistry::new(),
            executor_bridge: None,
            game_type,
            initialized: false,
        }
    }

    /// Инициализировать менеджер
    pub async fn initialize(&mut self, native_executor: Arc<RwLock<NativeExecutor>>) -> Result<()> {
        if self.initialized {
            return Ok(());
        }

        info!("🔧 Initializing native manager for {:?}...", self.game_type);

        // Загружаем нативные функции из генератора
        self.load_generated_natives().await
            .context("Failed to load generated natives")?;

        // Создаем мост к исполнителю
        let bridge = executor_bridge::ExecutorBridge::new(native_executor, self.game_type).await?;
        self.executor_bridge = Some(Arc::new(RwLock::new(bridge)));

        // Регистрируем функции в мосту
        self.register_natives_in_bridge().await
            .context("Failed to register natives in bridge")?;

        self.initialized = true;
        info!("✅ Native manager initialized with {} functions", self.registry.count());

        Ok(())
    }

    /// Загрузить сгенерированные нативные функции
    async fn load_generated_natives(&mut self) -> Result<()> {
        debug!("Loading generated native functions...");

        // Загружаем функции в зависимости от типа игры
        match self.game_type {
            GameType::GtaV => self.load_gta5_natives().await?,
            GameType::Rdr2 => self.load_rdr2_natives().await?,
        }

        Ok(())
    }

    /// Загрузить нативы GTA V
    async fn load_gta5_natives(&mut self) -> Result<()> {
        // Здесь будет интеграция с сгенерированными файлами из native-generator
        // Пока что добавляем несколько примеров

        self.registry.register_function(
            0x4F8644AF03D0E0D6, // GET_PLAYER_PED
            "GET_PLAYER_PED".to_string(),
            vec!["playerId".to_string()],
            "Entity".to_string(),
        )?;

        self.registry.register_function(
            0x6E192E33AD436366, // GET_ENTITY_COORDS
            "GET_ENTITY_COORDS".to_string(),
            vec!["entity".to_string()],
            "Vector3".to_string(),
        )?;

        self.registry.register_function(
            0x06843DA7060A026B, // SET_ENTITY_COORDS
            "SET_ENTITY_COORDS".to_string(),
            vec!["entity".to_string(), "x".to_string(), "y".to_string(), "z".to_string()],
            "void".to_string(),
        )?;

        debug!("Loaded {} GTA V native functions", self.registry.count());
        Ok(())
    }

    /// Загрузить нативы RDR2
    async fn load_rdr2_natives(&mut self) -> Result<()> {
        // Аналогично для RDR2
        self.registry.register_function(
            0x275F255ED201B937, // GET_PLAYER_PED
            "GET_PLAYER_PED".to_string(),
            vec!["playerId".to_string()],
            "Entity".to_string(),
        )?;

        debug!("Loaded {} RDR2 native functions", self.registry.count());
        Ok(())
    }

    /// Зарегистрировать нативы в мосту
    async fn register_natives_in_bridge(&self) -> Result<()> {
        if let Some(bridge) = &self.executor_bridge {
            for function in self.registry.get_all_functions() {
                bridge.write().await.register_function(function.clone())?;
            }
        }

        Ok(())
    }

    /// Выполнить нативную функцию по имени
    pub async fn call_native(&self, name: &str, args: Vec<NativeValue>) -> Result<NativeValue> {
        if !self.initialized {
            anyhow::bail!("Native manager not initialized");
        }

        if let Some(bridge) = &self.executor_bridge {
            bridge.read().await.call_by_name(name, args).await
        } else {
            anyhow::bail!("Executor bridge not available")
        }
    }

    /// Выполнить нативную функцию по хешу
    pub async fn call_native_by_hash(&self, hash: u64, args: Vec<NativeValue>) -> Result<NativeValue> {
        if !self.initialized {
            anyhow::bail!("Native manager not initialized");
        }

        if let Some(bridge) = &self.executor_bridge {
            bridge.read().await.call_by_hash(hash, args).await
        } else {
            anyhow::bail!("Executor bridge not available")
        }
    }

    /// Получить информацию о функции
    pub fn get_function_info(&self, name: &str) -> Option<&registry::NativeFunctionInfo> {
        self.registry.get_function_by_name(name)
    }

    /// Получить список всех функций
    pub fn list_functions(&self) -> Vec<&registry::NativeFunctionInfo> {
        self.registry.get_all_functions()
    }

    /// Проверить инициализацию
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Получить количество зарегистрированных функций
    pub fn function_count(&self) -> usize {
        self.registry.count()
    }
}

// Удобные макросы для вызова нативов
#[macro_export]
macro_rules! call_native {
    ($manager:expr, $name:expr, $($arg:expr),*) => {
        $manager.call_native($name, vec![$($arg),*]).await
    };
}

#[macro_export]
macro_rules! call_native_hash {
    ($manager:expr, $hash:expr, $($arg:expr),*) => {
        $manager.call_native_by_hash($hash, vec![$($arg),*]).await
    };
} 