//! Ошибки логирования
//!
//! Этот модуль содержит типы ошибок, связанные с системой логирования.

use std::fmt;

/// Ошибки, которые могут возникнуть при работе с системой логирования
#[derive(Debug)]
pub enum LogError {
    /// Ошибка инициализации системы логирования
    InitError(String),
    /// Ошибка ввода-вывода при работе с файлами логов
    IoError(String),
    /// Ошибка конфигурации логирования
    ConfigError(String),
    /// Ошибка форматирования логов
    FormatError(String),
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogError::InitError(msg) => write!(f, "Ошибка инициализации логирования: {}", msg),
            LogError::IoError(msg) => write!(f, "Ошибка ввода-вывода при логировании: {}", msg),
            LogError::ConfigError(msg) => write!(f, "Ошибка конфигурации логирования: {}", msg),
            LogError::FormatError(msg) => write!(f, "Ошибка форматирования логов: {}", msg),
        }
    }
}

impl std::error::Error for LogError {}

/// Преобразование IO ошибок в LogError
impl From<std::io::Error> for LogError {
    fn from(err: std::io::Error) -> Self {
        LogError::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_error_display() {
        let error = LogError::InitError("test error".to_string());
        assert!(error.to_string().contains("test error"));
        
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let log_error = LogError::from(io_error);
        assert!(matches!(log_error, LogError::IoError(_)));
    }
} 