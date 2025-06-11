//! Ошибки конфигурации
//!
//! Этот модуль содержит типы ошибок, связанные с конфигурацией.

use std::fmt;
use config::ConfigError;

/// Ошибки, которые могут возникнуть при работе с конфигурацией
#[derive(Debug)]
pub enum ConfigurationError {
    /// Ошибка при загрузке конфигурации
    LoadError(String),
    /// Ошибка при разборе конфигурации
    ParseError(String),
    /// Отсутствует обязательное поле
    MissingField(String),
    /// Некорректное значение поля
    InvalidValue(String, String),
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigurationError::LoadError(msg) => write!(f, "Ошибка загрузки конфигурации: {}", msg),
            ConfigurationError::ParseError(msg) => write!(f, "Ошибка разбора конфигурации: {}", msg),
            ConfigurationError::MissingField(field) => write!(f, "Отсутствует обязательное поле: {}", field),
            ConfigurationError::InvalidValue(field, msg) => write!(f, "Некорректное значение поля {}: {}", field, msg),
        }
    }
}

impl std::error::Error for ConfigurationError {}

/// Преобразование ошибок конфигурации из библиотеки config
impl From<ConfigError> for ConfigurationError {
    fn from(err: ConfigError) -> Self {
        match err {
            ConfigError::Message(msg) if msg.contains("missing field") => {
                let field = msg.split("missing field").nth(1).unwrap_or("").trim_matches(|c| c == ' ' || c == '`' || c == '\'').to_string();
                ConfigurationError::MissingField(field)
            },
            _ => ConfigurationError::ParseError(err.to_string()),
        }
    }
}

/// Преобразование IO ошибок
impl From<std::io::Error> for ConfigurationError {
    fn from(err: std::io::Error) -> Self {
        ConfigurationError::LoadError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_error_display() {
        let error = ConfigurationError::MissingField("server.port".to_string());
        assert!(error.to_string().contains("server.port"));
        
        let error = ConfigurationError::InvalidValue("max_players".to_string(), "должно быть положительным".to_string());
        assert!(error.to_string().contains("max_players"));
        assert!(error.to_string().contains("должно быть положительным"));
    }
} 