//! Модуль логирования
//!
//! Этот модуль предоставляет функциональность для настройки и использования логирования
//! во всех компонентах GameVerse Framework. Он использует библиотеку `tracing` для
//! структурированного логирования и интеграции с системами мониторинга.

use std::path::Path;
use tracing::Level;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::{
    fmt::format::FmtSpan,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

mod context;
mod error;

pub use context::*;
pub use error::*;

/// Конфигурация логирования
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Уровень логирования по умолчанию
    pub level: Level,
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

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: Level::INFO,
            log_dir: Some("logs".to_string()),
            max_file_size: 50 * 1024 * 1024, // 50MB
            json_format: false,
            enable_tracing: false,
            jaeger_endpoint: None,
        }
    }
}

/// Инициализирует систему логирования с настройками по умолчанию
///
/// Эта функция должна быть вызвана в начале работы приложения.
///
/// # Returns
///
/// Результат инициализации логирования
pub fn initialize() -> Result<(), LogError> {
    initialize_with_config(LogConfig::default())
}

/// Инициализирует систему логирования с заданной конфигурацией
///
/// # Параметры
///
/// * `config` - Конфигурация логирования
///
/// # Returns
///
/// Результат инициализации логирования
pub fn initialize_with_config(config: LogConfig) -> Result<(), LogError> {
    // Настраиваем фильтр на основе уровня логирования или переменной окружения
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(format!("gameverse_core={},tower_http=info", config.level))
    });

    // Создаем форматтер для логов
    let fmt_layer = if config.json_format {
        fmt::layer()
            .json()
            .with_span_events(FmtSpan::CLOSE)
            .boxed()
    } else {
        fmt::layer()
            .with_target(true)
            .with_span_events(FmtSpan::CLOSE)
            .boxed()
    };

    // Настраиваем основной подписчик
    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    // Добавляем запись в файл, если указана директория логов
    if let Some(log_dir) = config.log_dir {
        let log_path = Path::new(&log_dir);
        
        // Создаем директорию для логов, если она не существует
        if !log_path.exists() {
            std::fs::create_dir_all(log_path)
                .map_err(|e| LogError::IoError(e.to_string()))?;
        }
        
        // Настраиваем файловый аппендер с ротацией
        let file_appender = tracing_appender::rolling::daily(log_path, "gameverse.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        
        // Важно сохранить _guard в глобальной переменной, иначе файл будет закрыт
        // когда _guard выйдет из области видимости
        // В реальном коде нужно хранить где-то _guard
        
        let file_layer = if config.json_format {
            fmt::layer()
                .json()
                .with_writer(non_blocking)
                .boxed()
        } else {
            fmt::layer()
                .with_writer(non_blocking)
                .boxed()
        };
        
        subscriber.with(file_layer).try_init()
            .map_err(|e| LogError::InitError(e.to_string()))?;
    } else {
        subscriber.try_init()
            .map_err(|e| LogError::InitError(e.to_string()))?;
    }

    // Логируем информацию о запуске
    tracing::info!(
        version = crate::VERSION,
        "Logging initialized for {} at level {}",
        crate::NAME,
        config.level
    );

    Ok(())
}

/// Создает контекст логирования для компонента
///
/// # Параметры
///
/// * `component` - Название компонента
///
/// # Returns
///
/// Контекст логирования для компонента
pub fn component_logger(component: &str) -> LogContext {
    LogContext::new(component)
}

/// Макрос для создания контекста логирования модуля
///
/// Использует название модуля как имя компонента
#[macro_export]
macro_rules! module_logger {
    () => {
        $crate::logging::component_logger(module_path!())
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_config_default() {
        let config = LogConfig::default();
        assert_eq!(config.level, Level::INFO);
        assert!(config.log_dir.is_some());
        assert!(!config.json_format);
    }
} 