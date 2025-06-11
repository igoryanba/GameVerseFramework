//! Модуль конфигурации
//!
//! Этот модуль отвечает за загрузку и управление конфигурацией GameVerse Framework.
//! Он поддерживает загрузку из файлов, переменных окружения и программной установки.

use std::path::Path;
use serde::{Deserialize, Serialize};
use config::{Config as ConfigLib, ConfigError, Environment, File};

use crate::logging::LogConfig;

mod error;

pub use error::*;

/// Основная структура конфигурации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Конфигурация сервера
    pub server: ServerConfig,
    /// Конфигурация сети
    pub network: NetworkConfig,
    /// Конфигурация логирования
    pub logging: LoggingConfig,
    /// Конфигурация базы данных
    pub database: DatabaseConfig,
    /// Конфигурация скриптовых движков
    pub scripting: ScriptingConfig,
}

/// Конфигурация сервера
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Название сервера
    pub name: String,
    /// Максимальное количество игроков
    pub max_players: u32,
    /// Порт для подключения
    pub port: u16,
    /// Адрес для привязки
    pub bind_address: String,
    /// Пароль сервера (опционально)
    pub password: Option<String>,
    /// Таймаут бездействия в секундах
    pub idle_timeout: u64,
    /// Путь к ресурсам
    pub resources_path: String,
}

/// Конфигурация сети
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Использовать сжатие трафика
    pub use_compression: bool,
    /// Размер буфера для сообщений
    pub buffer_size: usize,
    /// Максимальный размер пакета
    pub max_packet_size: usize,
    /// Интервал синхронизации в миллисекундах
    pub sync_interval_ms: u64,
    /// Порт для админ-API
    pub admin_port: u16,
    /// Радиус синхронизации для сущностей
    pub sync_radius: f32,
    /// Максимальное количество сущностей для одного клиента
    pub max_entities_per_client: u32,
}

/// Конфигурация логирования
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Уровень логирования (trace, debug, info, warn, error)
    pub level: String,
    /// Путь к директории для файлов логов
    pub log_dir: Option<String>,
    /// Максимальный размер файла лога перед ротацией
    pub max_file_size: usize,
    /// Использовать форматирование JSON
    pub json_format: bool,
    /// Включить OpenTelemetry для трассировки
    pub enable_tracing: bool,
    /// URL Jaeger коллектора (если enable_tracing = true)
    pub jaeger_endpoint: Option<String>,
}

/// Конфигурация базы данных
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// URL подключения к PostgreSQL
    pub postgres_url: Option<String>,
    /// URL подключения к Redis
    pub redis_url: Option<String>,
    /// Максимальное количество соединений в пуле
    pub max_connections: u32,
    /// Таймаут для соединений в секундах
    pub connection_timeout: u64,
    /// Интервал синхронизации с БД в секундах
    pub sync_interval: u64,
}

/// Конфигурация скриптовых движков
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptingConfig {
    /// Включить поддержку Lua
    pub enable_lua: bool,
    /// Включить поддержку TypeScript
    pub enable_typescript: bool,
    /// Включить поддержку WebAssembly
    pub enable_wasm: bool,
    /// Размер стека для Lua (в килобайтах)
    pub lua_stack_size: usize,
    /// Таймаут выполнения скрипта в миллисекундах
    pub script_timeout_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                name: "GameVerse Server".to_string(),
                max_players: 32,
                port: 30120,
                bind_address: "0.0.0.0".to_string(),
                password: None,
                idle_timeout: 300,
                resources_path: "resources".to_string(),
            },
            network: NetworkConfig {
                use_compression: true,
                buffer_size: 8192,
                max_packet_size: 16384,
                sync_interval_ms: 50,
                admin_port: 30121,
                sync_radius: 500.0,
                max_entities_per_client: 1000,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                log_dir: Some("logs".to_string()),
                max_file_size: 50 * 1024 * 1024, // 50MB
                json_format: false,
                enable_tracing: false,
                jaeger_endpoint: None,
            },
            database: DatabaseConfig {
                postgres_url: None,
                redis_url: None,
                max_connections: 10,
                connection_timeout: 30,
                sync_interval: 5,
            },
            scripting: ScriptingConfig {
                enable_lua: true,
                enable_typescript: true,
                enable_wasm: false,
                lua_stack_size: 2048,
                script_timeout_ms: 5000,
            },
        }
    }
}

impl Config {
    /// Преобразует конфигурацию логирования в LogConfig
    pub fn to_log_config(&self) -> LogConfig {
        let level = match self.logging.level.to_lowercase().as_str() {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => tracing::Level::INFO,
        };

        LogConfig {
            level,
            log_dir: self.logging.log_dir.clone(),
            max_file_size: self.logging.max_file_size,
            json_format: self.logging.json_format,
            enable_tracing: self.logging.enable_tracing,
            jaeger_endpoint: self.logging.jaeger_endpoint.clone(),
        }
    }
}

/// Загружает конфигурацию из файла или использует значения по умолчанию
///
/// # Параметры
///
/// * `config_path` - Опциональный путь к файлу конфигурации
///
/// # Returns
///
/// Загруженную конфигурацию или ошибку
pub fn load_config(config_path: Option<&str>) -> Result<Config, ConfigError> {
    // Инициализируем конфигурацию с значениями по умолчанию
    let mut builder = ConfigLib::builder();

    // Добавляем файл конфигурации, если он указан
    if let Some(path) = config_path {
        builder = builder.add_source(File::with_name(path));
    } else {
        // Ищем файл конфигурации в стандартных местах
        let paths = vec![
            Path::new("config/gameverse.toml"),
            Path::new("gameverse.toml"),
            Path::new("/etc/gameverse/config.toml"),
        ];

        for path in paths {
            if path.exists() {
                builder = builder.add_source(File::from(path));
                break;
            }
        }
    }

    // Добавляем переменные окружения с префиксом GAMEVERSE_
    builder = builder.add_source(Environment::with_prefix("GAMEVERSE").separator("_"));

    // Собираем конфигурацию
    let config_lib = builder.build()?;

    // Преобразуем в нашу структуру
    let config: Config = config_lib.try_deserialize()?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.name, "GameVerse Server");
        assert_eq!(config.server.port, 30120);
        assert_eq!(config.network.sync_interval_ms, 50);
    }
    
    #[test]
    fn test_to_log_config() {
        let config = Config::default();
        let log_config = config.to_log_config();
        assert_eq!(log_config.level, tracing::Level::INFO);
        assert_eq!(log_config.log_dir, Some("logs".to_string()));
    }
} 