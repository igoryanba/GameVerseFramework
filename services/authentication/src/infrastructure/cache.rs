use anyhow::Result;
use redis::{Client, Commands, Connection};
use redis::aio::MultiplexedConnection;
use tracing::{debug, info, error};

use crate::config::DatabaseConfig;

/// Создает клиент Redis
pub fn create_redis_client(config: &DatabaseConfig) -> Result<Client> {
    debug!("Создание клиента Redis: {}", config.redis_url);
    
    let client = redis::Client::open(config.redis_url.as_str())?;
    
    // Проверяем соединение с Redis
    let mut conn = client.get_connection()?;
    debug!("Проверка соединения с Redis");
    let pong: String = redis::cmd("PING").query(&mut conn)?;
    debug!("Соединение с Redis установлено, ответ: {}", pong);
    
    info!("Клиент Redis успешно создан");
    Ok(client)
}

/// Создает multiplexed соединение Redis для async операций
pub async fn create_redis_multiplexed_connection(config: &DatabaseConfig) -> Result<MultiplexedConnection> {
    debug!("Создание multiplexed соединения Redis: {}", config.redis_url);
    
    let client = redis::Client::open(config.redis_url.as_str())?;
    let connection = client.get_multiplexed_async_connection().await?;
    
    info!("Multiplexed соединение Redis успешно создано");
    Ok(connection)
}

/// Класс для работы с кешем Redis
pub struct RedisCache {
    connection: Connection,
}

impl RedisCache {
    /// Создает новый экземпляр кеша Redis
    pub fn new(client: &Client) -> Result<Self> {
        let connection = client.get_connection()?;
        Ok(Self { connection })
    }
    
    /// Устанавливает значение в кеш с TTL
    pub fn set_with_expiry<T: ToString>(&mut self, key: &str, value: T, ttl_seconds: usize) -> Result<()> {
        let result: () = self.connection.set_ex(key, value.to_string(), ttl_seconds)?;
        Ok(result)
    }
    
    /// Получает значение из кеша
    pub fn get(&mut self, key: &str) -> Result<Option<String>> {
        let result: Option<String> = self.connection.get(key)?;
        Ok(result)
    }
    
    /// Удаляет значение из кеша
    pub fn delete(&mut self, key: &str) -> Result<()> {
        let result: () = self.connection.del(key)?;
        Ok(result)
    }
    
    /// Проверяет существование ключа
    pub fn exists(&mut self, key: &str) -> Result<bool> {
        let result: bool = self.connection.exists(key)?;
        Ok(result)
    }
    
    /// Добавляет значение в blacklist с TTL
    pub fn add_to_blacklist(&mut self, token: &str, ttl_seconds: usize) -> Result<()> {
        let key = format!("blacklist:{}", token);
        self.set_with_expiry(&key, "1", ttl_seconds)
    }
    
    /// Проверяет, находится ли токен в blacklist
    pub fn is_blacklisted(&mut self, token: &str) -> Result<bool> {
        let key = format!("blacklist:{}", token);
        self.exists(&key)
    }
    
    /// Добавляет ключ блокировки для ограничения попыток входа
    pub fn add_login_attempt(&mut self, identifier: &str) -> Result<u32> {
        let key = format!("login_attempts:{}", identifier);
        let count: u32 = self.connection.incr(&key, 1)?;
        
        // Устанавливаем TTL, если это первая попытка
        if count == 1 {
            let _: () = self.connection.expire(key.as_str(), 3600)?; // 1 час
        }
        
        Ok(count)
    }
    
    /// Сбрасывает счетчик попыток входа
    pub fn reset_login_attempts(&mut self, identifier: &str) -> Result<()> {
        let key = format!("login_attempts:{}", identifier);
        self.delete(&key)
    }
    
    /// Получает количество попыток входа
    pub fn get_login_attempts(&mut self, identifier: &str) -> Result<u32> {
        let key = format!("login_attempts:{}", identifier);
        match self.get(&key)? {
            Some(value) => {
                match value.parse::<u32>() {
                    Ok(count) => Ok(count),
                    Err(e) => {
                        error!("Ошибка при парсинге количества попыток входа: {}", e);
                        Ok(0)
                    }
                }
            },
            None => Ok(0),
        }
    }
} 