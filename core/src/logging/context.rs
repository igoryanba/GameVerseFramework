//! Контекст логирования
//!
//! Этот модуль предоставляет контекст логирования, который позволяет
//! добавлять дополнительную информацию к логам (компонент, request_id и т.д.)

use std::fmt;
use tracing::{debug, error, info, trace, warn, Level};
use uuid::Uuid;

/// Контекст логирования
///
/// Предоставляет методы для логирования с дополнительным контекстом.
#[derive(Debug, Clone)]
pub struct LogContext {
    /// Название компонента
    component: String,
    /// Идентификатор запроса (если есть)
    request_id: Option<String>,
    /// Дополнительные метаданные
    metadata: Vec<(String, String)>,
}

impl LogContext {
    /// Создает новый контекст логирования для компонента
    ///
    /// # Параметры
    ///
    /// * `component` - Название компонента
    pub fn new(component: &str) -> Self {
        Self {
            component: component.to_string(),
            request_id: None,
            metadata: Vec::new(),
        }
    }

    /// Добавляет идентификатор запроса к контексту
    ///
    /// # Параметры
    ///
    /// * `request_id` - Идентификатор запроса
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// Генерирует новый идентификатор запроса и добавляет его к контексту
    pub fn with_new_request_id(self) -> Self {
        self.with_request_id(Uuid::new_v4().to_string())
    }

    /// Добавляет метаданные к контексту
    ///
    /// # Параметры
    ///
    /// * `key` - Ключ метаданных
    /// * `value` - Значение метаданных
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl fmt::Display) -> Self {
        self.metadata.push((key.into(), value.to_string()));
        self
    }

    /// Логирует сообщение с уровнем TRACE
    ///
    /// # Параметры
    ///
    /// * `message` - Сообщение для логирования
    pub fn trace(&self, message: impl fmt::Display) {
        self.log(Level::TRACE, message);
    }

    /// Логирует сообщение с уровнем DEBUG
    ///
    /// # Параметры
    ///
    /// * `message` - Сообщение для логирования
    pub fn debug(&self, message: impl fmt::Display) {
        self.log(Level::DEBUG, message);
    }

    /// Логирует сообщение с уровнем INFO
    ///
    /// # Параметры
    ///
    /// * `message` - Сообщение для логирования
    pub fn info(&self, message: impl fmt::Display) {
        self.log(Level::INFO, message);
    }

    /// Логирует сообщение с уровнем WARN
    ///
    /// # Параметры
    ///
    /// * `message` - Сообщение для логирования
    pub fn warn(&self, message: impl fmt::Display) {
        self.log(Level::WARN, message);
    }

    /// Логирует сообщение с уровнем ERROR
    ///
    /// # Параметры
    ///
    /// * `message` - Сообщение для логирования
    pub fn error(&self, message: impl fmt::Display) {
        self.log(Level::ERROR, message);
    }

    /// Логирует сообщение с указанным уровнем
    ///
    /// # Параметры
    ///
    /// * `level` - Уровень логирования
    /// * `message` - Сообщение для логирования
    fn log(&self, level: Level, message: impl fmt::Display) {
        let message = message.to_string();
        
        match level {
            Level::TRACE => {
                trace!(
                    component = self.component,
                    request_id = self.request_id.as_deref().unwrap_or(""),
                    metadata = ?self.metadata,
                    "{}", message
                );
            }
            Level::DEBUG => {
                debug!(
                    component = self.component,
                    request_id = self.request_id.as_deref().unwrap_or(""),
                    metadata = ?self.metadata,
                    "{}", message
                );
            }
            Level::INFO => {
                info!(
                    component = self.component,
                    request_id = self.request_id.as_deref().unwrap_or(""),
                    metadata = ?self.metadata,
                    "{}", message
                );
            }
            Level::WARN => {
                warn!(
                    component = self.component,
                    request_id = self.request_id.as_deref().unwrap_or(""),
                    metadata = ?self.metadata,
                    "{}", message
                );
            }
            Level::ERROR => {
                error!(
                    component = self.component,
                    request_id = self.request_id.as_deref().unwrap_or(""),
                    metadata = ?self.metadata,
                    "{}", message
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_context_creation() {
        let logger = LogContext::new("test-component");
        assert_eq!(logger.component, "test-component");
        assert!(logger.request_id.is_none());
        assert!(logger.metadata.is_empty());
    }
    
    #[test]
    fn test_log_context_with_request_id() {
        let logger = LogContext::new("test-component").with_request_id("123");
        assert_eq!(logger.request_id, Some("123".to_string()));
    }
    
    #[test]
    fn test_log_context_with_metadata() {
        let logger = LogContext::new("test-component")
            .with_metadata("key1", "value1")
            .with_metadata("key2", 42);
            
        assert_eq!(logger.metadata.len(), 2);
        assert_eq!(logger.metadata[0], ("key1".to_string(), "value1".to_string()));
        assert_eq!(logger.metadata[1], ("key2".to_string(), "42".to_string()));
    }
} 