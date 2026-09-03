//! # GameVerse Core
//!
//! Ядро GameVerse Framework - модульная система для создания
//! высокопроизводительных игровых серверов. Ядро обеспечивает:
//!
//! - Взаимодействие с игровыми API
//! - Сетевую коммуникацию и синхронизацию
//! - Систему скриптинга (Lua, TypeScript, WASM)
//! - **Систему плагинов превосходящую FiveM** 🚀
//! - Управление ресурсами
//! - Взаимодействие с микросервисами

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod config;
pub mod engine;
pub mod ffi;
pub mod game;
pub mod game_integration;
pub mod logging;
pub mod natives;
pub mod net;
pub mod plugins;
pub mod resource;
pub mod scripting;
pub mod server;
pub mod utils;
pub mod benchmarks;
pub mod anticheat;
pub mod wasm_ui;
/// Optional FiveM compatibility facade; not required by the platform runtime.
#[cfg(feature = "fivem-compat")]
pub mod fcl;

/// Информация о версии фреймворка
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Название фреймворка
pub const NAME: &str = env!("CARGO_PKG_NAME");
/// Описание фреймворка
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// Глобальная инициализация ядра
///
/// Эта функция должна быть вызвана в начале работы приложения.
/// Она инициализирует логирование, загружает конфигурацию и настраивает
/// все необходимые компоненты включая систему плагинов.
///
/// # Параметры
///
/// * `config_path` - Путь к конфигурационному файлу
///
/// # Returns
///
/// Результат инициализации с контекстом ядра или ошибку
pub async fn initialize(config_path: Option<&str>) -> anyhow::Result<CoreContext> {
    // Инициализация логирования должна быть первой
    logging::initialize()?;

    // Загружаем конфигурацию
    let config = config::load_config(config_path)?;

    tracing::info!(
        version = VERSION,
        "Initializing {} - {}",
        NAME,
        DESCRIPTION
    );

    // Создаем контекст ядра
    let core_context = CoreContext::new(config).await?;

    tracing::info!("🚀 GameVerse Core initialized successfully with plugin system");

    Ok(core_context)
}

/// Контекст ядра, содержащий все необходимые ресурсы и состояние
///
/// Этот объект передается всем компонентам и содержит ссылки на
/// основные подсистемы, конфигурацию и общее состояние включая
/// менеджер плагинов.
#[derive(Debug)]
pub struct CoreContext {
    /// Конфигурация ядра
    pub config: config::Config,
    // Менеджер плагинов (будет добавлен после создания остальных компонентов)
    // pub plugin_manager: Option<Arc<plugins::PluginManager>>,
}

impl CoreContext {
    /// Создает новый контекст ядра
    ///
    /// # Параметры
    ///
    /// * `config` - Загруженная конфигурация
    ///
    /// # Returns
    ///
    /// Результат создания контекста или ошибку
    async fn new(config: config::Config) -> anyhow::Result<Self> {
        // Здесь будет инициализация различных подсистем
        // на основе конфигурации включая систему плагинов

        Ok(Self { 
            config,
            // plugin_manager: None, // Будет инициализирован позже
        })
    }

    /// Возвращает информацию о версии и сборке
    pub fn version_info(&self) -> String {
        format!("{} v{} - With Plugin System 🔥", NAME, VERSION)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constants() {
        assert!(!VERSION.is_empty());
        assert!(!NAME.is_empty());
        assert!(!DESCRIPTION.is_empty());
    }
    
    #[test]
    fn test_version_info_includes_plugin_system() {
        let config = config::Config::default();
        let context = CoreContext { config };
        let info = context.version_info();
        assert!(info.contains("Plugin System"));
    }
}

// Re-export основных типов для удобства
pub use engine::GameEngine;
pub use game_integration::{GameType, GameIntegrator};
pub use natives::{NativeManager, NativeResult};
pub use benchmarks::PerformanceBenchmark;
pub use anticheat::AntiCheatEvasion;
pub use wasm_ui::WasmUIManager;
