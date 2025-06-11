//! # Система плагинов GameVerse Framework
//!
//! Современная система плагинов, превосходящая FiveM ресурсы:
//! - Динамическая загрузка без перезапуска сервера
//! - TOML конфигурация с валидацией
//! - Type safety через Rust интерфейсы
//! - Система зависимостей с автоматическим разрешением
//! - Hot reload с проверкой типов
//! - Sandboxed execution для безопасности

use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::pin::Pin;
use std::future::Future;

pub mod manager;
pub mod registry;
pub mod config;
pub mod context;
pub mod dependency;

pub use manager::PluginManager;
pub use registry::PluginRegistry;
pub use config::{PluginConfig, PluginManifest};
pub use context::PluginContext;
pub use dependency::DependencyGraph;

/// Основной трейт для плагинов GameVerse (object-safe версия)
/// 
/// Превосходство над FiveM:
/// - Type safety через строгую типизацию Rust
/// - Object-safe design для dynamic dispatch
/// - Автоматическое управление жизненным циклом
pub trait GameVersePlugin: Send + Sync + std::fmt::Debug {
    /// Инициализация плагина
    fn initialize(&mut self, context: &PluginContext) -> Result<()>;
    
    /// Финализация плагина перед выгрузкой
    fn finalize(&mut self) -> Result<()> {
        Ok(())
    }
    
    /// Получение информации о плагине
    fn info(&self) -> PluginInfo;
    
    /// Получение API эндпоинтов, предоставляемых плагином
    fn get_api_endpoints(&self) -> Vec<ApiEndpoint> {
        Vec::new()
    }
    
    /// Клонирование плагина (для hot reload)
    fn clone_box(&self) -> Box<dyn GameVersePlugin>;
}

/// Асинхронные события плагина (отдельный trait для избежания object-safety проблем)
pub trait GameVersePluginAsync: Send + Sync {
    /// Обработка события подключения игрока
    fn on_player_connect(&self, player_id: u64) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
    
    /// Обработка события отключения игрока
    fn on_player_disconnect(&self, player_id: u64) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
    
    /// Обработка загрузки ресурса
    fn on_resource_load(&self, resource_name: &str) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
    
    /// Обработка выгрузки ресурса
    fn on_resource_unload(&self, resource_name: &str) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
    
    /// Обработка команды сервера
    fn on_server_command(&self, command: &str, args: &[&str]) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + '_>>;
}

/// Информация о плагине
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Уникальный идентификатор плагина
    pub id: String,
    /// Имя плагина
    pub name: String,
    /// Версия плагина (семантическое версионирование)
    pub version: String,
    /// Автор плагина
    pub author: String,
    /// Описание плагина
    pub description: String,
    /// Веб-сайт или репозиторий
    pub website: Option<String>,
    /// Лицензия
    pub license: Option<String>,
    /// Зависимости плагина
    pub dependencies: Vec<PluginDependency>,
    /// Теги для категоризации
    pub tags: Vec<String>,
}

/// Зависимость плагина
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// Имя зависимости
    pub name: String,
    /// Версия (поддерживает semver ranges)
    pub version: String,
    /// Обязательная ли зависимость
    pub required: bool,
}

/// API эндпоинт, предоставляемый плагином
#[derive(Debug, Clone)]
pub struct ApiEndpoint {
    /// HTTP метод
    pub method: HttpMethod,
    /// Путь эндпоинта
    pub path: String,
    /// Описание эндпоинта
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

/// HTTP запрос к API плагина
#[derive(Debug)]
pub struct ApiRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub query_params: HashMap<String, String>,
}

/// HTTP ответ от API плагина
#[derive(Debug)]
pub struct ApiResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

/// Состояние плагина
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginState {
    /// Плагин загружен, но не инициализирован
    Loaded,
    /// Плагин инициализирован и готов к работе
    Initialized,
    /// Плагин запущен и активен
    Running,
    /// Плагин приостановлен
    Paused,
    /// Плагин остановлен
    Stopped,
    /// Плагин выгружен
    Unloaded,
    /// Ошибка в плагине
    Error(String),
}

/// Метаданные загруженного плагина
#[derive(Debug)]
pub struct LoadedPlugin {
    /// Информация о плагине
    pub info: PluginInfo,
    /// Экземпляр плагина
    pub instance: Box<dyn GameVersePlugin>,
    /// Текущее состояние
    pub state: PluginState,
    /// Время загрузки
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    /// Путь к файлу плагина
    pub path: PathBuf,
    /// UUID сессии загрузки
    pub session_id: Uuid,
}

/// Событие плагина для системы событий
#[derive(Debug, Clone)]
pub enum PluginEvent {
    /// Плагин загружен
    Loaded { plugin_id: String },
    /// Плагин инициализирован
    Initialized { plugin_id: String },
    /// Плагин запущен
    Started { plugin_id: String },
    /// Плагин остановлен
    Stopped { plugin_id: String },
    /// Плагин выгружен
    Unloaded { plugin_id: String },
    /// Ошибка в плагине
    Error { plugin_id: String, error: String },
    /// Hot reload плагина
    HotReloaded { plugin_id: String },
}

/// Конфигурация системы плагинов
#[derive(Debug, Clone)]
pub struct PluginSystemConfig {
    /// Директория с плагинами
    pub plugins_directory: PathBuf,
    /// Включить hot reload
    pub enable_hot_reload: bool,
    /// Интервал проверки изменений (мс)
    pub hot_reload_interval_ms: u64,
    /// Максимальное время загрузки плагина (мс)
    pub load_timeout_ms: u64,
    /// Включить sandboxing
    pub enable_sandboxing: bool,
    /// Максимальное количество API запросов в секунду на плагин
    pub api_rate_limit: u32,
    /// Разрешенные системные вызовы для плагинов
    pub allowed_syscalls: Vec<String>,
}

impl Default for PluginSystemConfig {
    fn default() -> Self {
        Self {
            plugins_directory: PathBuf::from("./plugins"),
            enable_hot_reload: true,
            hot_reload_interval_ms: 1000,
            load_timeout_ms: 10000,
            enable_sandboxing: true,
            api_rate_limit: 100,
            allowed_syscalls: vec![
                "read".to_string(),
                "write".to_string(),
                "open".to_string(),
                "close".to_string(),
            ],
        }
    }
}

pub type PluginResult<T> = Result<T, PluginError>;

/// Ошибки системы плагинов
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin not found: {plugin_id}")]
    NotFound { plugin_id: String },
    
    #[error("Plugin loading failed: {reason}")]
    LoadingFailed { reason: String },
    
    #[error("Plugin initialization failed: {reason}")]
    InitializationFailed { reason: String },
    
    #[error("Dependency resolution failed: {reason}")]
    DependencyError { reason: String },
    
    #[error("Configuration error: {reason}")]
    ConfigError { reason: String },
    
    #[error("Security violation: {reason}")]
    SecurityViolation { reason: String },
    
    #[error("API error: {reason}")]
    ApiError { reason: String },
    
    #[error("Hot reload failed: {reason}")]
    HotReloadFailed { reason: String },
    
    #[error("Plugin is in invalid state: {state:?}")]
    InvalidState { state: PluginState },
}

impl From<anyhow::Error> for PluginError {
    fn from(err: anyhow::Error) -> Self {
        PluginError::LoadingFailed {
            reason: err.to_string(),
        }
    }
}

/// Трейт для фабрики плагинов
pub trait PluginFactory: Send + Sync {
    /// Создать экземпляр плагина
    fn create_plugin(&self, config: &PluginConfig) -> PluginResult<Box<dyn GameVersePlugin>>;
    
    /// Проверить совместимость плагина с версией ядра
    fn is_compatible(&self, core_version: &str) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_info_serialization() {
        let info = PluginInfo {
            id: "test-plugin".to_string(),
            name: "Test Plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Test Author".to_string(),
            description: "A test plugin".to_string(),
            website: Some("https://example.com".to_string()),
            license: Some("MIT".to_string()),
            dependencies: Vec::new(),
            tags: vec!["test".to_string()],
        };

        let serialized = serde_json::to_string(&info).unwrap();
        let deserialized: PluginInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(info.id, deserialized.id);
    }

    #[test]
    fn test_plugin_state_transitions() {
        let state = PluginState::Loaded;
        assert_ne!(state, PluginState::Running);
        
        let error_state = PluginState::Error("Test error".to_string());
        assert!(matches!(error_state, PluginState::Error(_)));
    }
} 