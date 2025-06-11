use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

/// Основная структура конфигурации
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Конфигурация сервера
    pub server: ServerConfig,
    /// Конфигурация логирования
    pub logging: LoggingConfig,
    /// Конфигурация базы данных
    pub database: DatabaseConfig,
    /// Конфигурация аутентификации
    pub auth: AuthConfig,
    /// Конфигурация безопасности
    pub security: SecurityConfig,
}

/// Конфигурация сервера
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    /// Хост для REST API
    pub rest_host: String,
    /// Порт для REST API
    pub rest_port: u16,
    /// Хост для gRPC
    pub grpc_host: String,
    /// Порт для gRPC
    pub grpc_port: u16,
    /// Таймаут запросов в секундах
    pub request_timeout: u64,
    /// Максимальное количество соединений
    pub max_connections: usize,
}

impl ServerConfig {
    /// Получает таймаут в виде Duration
    pub fn request_timeout(&self) -> Duration {
        Duration::from_secs(self.request_timeout)
    }
    
    /// Получает полный адрес для REST API
    pub fn rest_address(&self) -> String {
        format!("{}:{}", self.rest_host, self.rest_port)
    }
    
    /// Получает полный адрес для gRPC
    pub fn grpc_address(&self) -> String {
        format!("{}:{}", self.grpc_host, self.grpc_port)
    }
}

/// Конфигурация логирования
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    /// Уровень логирования
    pub level: String,
    /// Директория для файлов логов
    pub log_dir: String,
    /// Использовать JSON формат
    pub json_format: bool,
    /// Отправлять логи в центральный сервис логирования
    pub send_to_central: bool,
    /// Адрес центрального сервиса логирования
    pub central_logging_url: String,
}

/// Конфигурация базы данных
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    /// Строка подключения к PostgreSQL
    pub postgres_url: String,
    /// Строка подключения к Redis
    pub redis_url: String,
    /// Максимальное количество соединений в пуле
    pub max_connections: u32,
    /// Таймаут соединения в секундах
    pub connection_timeout: u64,
    /// Интервал проверки соединения в секундах
    pub health_check_interval: u64,
}

impl DatabaseConfig {
    /// Получает таймаут соединения в виде Duration
    pub fn connection_timeout(&self) -> Duration {
        Duration::from_secs(self.connection_timeout)
    }
    
    /// Получает интервал проверки в виде Duration
    pub fn health_check_interval(&self) -> Duration {
        Duration::from_secs(self.health_check_interval)
    }
}

/// Конфигурация аутентификации
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthConfig {
    /// Время жизни токена доступа в секундах
    pub access_token_lifetime_seconds: i64,
    /// Время жизни refresh токена в секундах
    pub refresh_token_lifetime_seconds: i64,
    /// Секрет для подписи JWT токенов
    pub jwt_secret: String,
    /// Issuer для JWT токенов
    pub issuer: String,
    /// Audience для JWT токенов
    pub audience: String,
    /// Максимальное количество неудачных попыток входа
    pub max_failed_attempts: u32,
    /// Время блокировки после превышения количества попыток в секундах
    pub lockout_duration_seconds: u64,
    /// Путь к приватному ключу для подписи JWT (опционально)
    pub private_key_path: Option<String>,
    /// Путь к публичному ключу для верификации JWT (опционально)
    pub public_key_path: Option<String>,
    /// Алгоритм подписи JWT
    pub jwt_algorithm: String,
    /// Сложность хеширования паролей (memory cost)
    pub password_memory_cost: u32,
    /// Количество итераций хеширования
    pub password_iterations: u32,
    /// Длина соли в байтах
    pub password_salt_length: usize,
}

impl AuthConfig {
    /// Получает время жизни токена доступа в виде Duration
    pub fn access_token_lifetime(&self) -> Duration {
        Duration::from_secs(self.access_token_lifetime_seconds as u64)
    }
    
    /// Получает время жизни refresh токена в виде Duration
    pub fn refresh_token_lifetime(&self) -> Duration {
        Duration::from_secs(self.refresh_token_lifetime_seconds as u64)
    }
}

/// Конфигурация безопасности
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecurityConfig {
    /// Разрешенные источники для CORS
    pub cors_origins: Vec<String>,
    /// Максимальное количество попыток ввода пароля
    pub max_login_attempts: u32,
    /// Время блокировки после превышения количества попыток в минутах
    pub lockout_duration: u64,
    /// Включить двухфакторную аутентификацию
    pub enable_2fa: bool,
}

impl SecurityConfig {
    /// Получает время блокировки в виде Duration
    pub fn lockout_duration(&self) -> Duration {
        Duration::from_secs(self.lockout_duration * 60)
    }
}

/// Загружает конфигурацию из файла
pub fn load_config() -> Result<Config> {
    use std::env;
    use config::{Config as ConfigLib, File, Environment};
    
    // Ищем путь к конфигурации в переменных окружения или используем значение по умолчанию
    let config_path = env::var("GAMEVERSE_AUTH_CONFIG")
        .unwrap_or_else(|_| "config/config.toml".to_string());
    
    // Проверяем существование файла конфигурации
    if !Path::new(&config_path).exists() {
        return Err(anyhow::anyhow!("Файл конфигурации не найден: {}", config_path));
    }
    
    // Строим конфигурацию
    let mut builder = ConfigLib::builder();
    
    // Добавляем файл конфигурации
    builder = builder.add_source(File::with_name(&config_path));
    
    // Добавляем переменные окружения с префиксом GAMEVERSE_AUTH_
    builder = builder.add_source(
        Environment::with_prefix("GAMEVERSE_AUTH")
            .separator("_")
    );
    
    // Собираем конфигурацию
    let config: Config = builder
        .build()
        .context("Ошибка при построении конфигурации")?
        .try_deserialize()
        .context("Ошибка при десериализации конфигурации")?;
    
    Ok(config)
} 