use serde::{Deserialize, Serialize};
use std::env;

/// Конфигурация микросервиса инвентаря
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Настройки базы данных
    pub database: DatabaseConfig,
    
    /// Настройки Redis
    pub redis: RedisConfig,
    
    /// Настройки gRPC сервера
    pub grpc: GrpcConfig,
    
    /// Настройки HTTP сервера
    pub http: HttpConfig,
    
    /// Настройки логирования
    pub logging: LoggingConfig,
}

/// Настройки базы данных
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// URL подключения к базе данных PostgreSQL
    pub url: String,
    
    /// Максимальное количество соединений в пуле
    pub max_connections: u32,
    
    /// Тайм-аут для операций с базой данных
    pub timeout_seconds: u64,
}

/// Настройки Redis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    /// URL подключения к Redis
    pub url: String,
    
    /// Время жизни кэша (в секундах)
    pub cache_ttl_seconds: u64,
}

/// Настройки gRPC сервера
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcConfig {
    /// Адрес для прослушивания
    pub host: String,
    
    /// Порт для прослушивания
    pub port: u16,
}

/// Настройки HTTP сервера
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    /// Адрес для прослушивания
    pub host: String,
    
    /// Порт для прослушивания
    pub port: u16,
}

/// Настройки логирования
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Адрес сервиса логирования
    pub service_address: String,
    
    /// Уровень логирования
    pub level: String,
}

impl Config {
    /// Загружает конфигурацию из переменных окружения
    pub fn from_env() -> Self {
        Self {
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .unwrap_or_else(|_| "postgres://gameverse:gameverse@localhost:5432/gameverse_inventory".to_string()),
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(10),
                timeout_seconds: env::var("DATABASE_TIMEOUT_SECONDS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(30),
            },
            redis: RedisConfig {
                url: env::var("REDIS_URL")
                    .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
                cache_ttl_seconds: env::var("REDIS_CACHE_TTL_SECONDS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(3600),
            },
            grpc: GrpcConfig {
                host: env::var("GRPC_HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("GRPC_PORT")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(50052),
            },
            http: HttpConfig {
                host: env::var("HTTP_HOST")
                    .unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("HTTP_PORT")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(8081),
            },
            logging: LoggingConfig {
                service_address: env::var("LOGGING_SERVICE_ADDRESS")
                    .unwrap_or_else(|_| "http://localhost:50051".to_string()),
                level: env::var("LOG_LEVEL")
                    .unwrap_or_else(|_| "info".to_string()),
            },
        }
    }
}