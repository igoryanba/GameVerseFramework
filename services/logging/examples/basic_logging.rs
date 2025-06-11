use gameverse_logging::{LogLevel, Logger, ConsoleLogHandler, FileLogHandler};
use std::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Создаем логгер для сервиса "auth-service"
    let logger = Logger::new("auth-service");
    
    // Добавляем обработчики вывода в консоль и в файл
    logger.add_handler(ConsoleLogHandler).await;
    logger.add_handler(FileLogHandler::new("logs/auth-service.log")).await;
    
    // Логируем сообщения разных уровней
    logger.info("startup", "Auth service starting...").await?;
    logger.debug("config", "Loaded configuration from environment").await?;
    
    // Создаем уникальный request_id для запроса
    let request_id = Uuid::new_v4();
    
    // Логируем с контекстом
    let start = std::time::Instant::now();
    tokio::time::sleep(Duration::from_millis(100)).await;
    let duration = start.elapsed().as_millis() as u64;
    
    // Создаем запись лога с контекстом и метриками
    let log_entry = gameverse_logging::LogEntry::new(
        LogLevel::Info, 
        "auth-service", 
        "login", 
        "User login successful"
    )
    .with_request_id(request_id)
    .with_user_id("user123")
    .with_data("ip_address", "192.168.1.1")
    .with_data("browser", "Chrome 98.0")
    .with_duration(duration);
    
    // Отправляем готовую запись лога
    logger.log_entry(log_entry).await?;
    
    // Логируем ошибку
    logger.error("database", "Failed to connect to PostgreSQL: connection timeout").await?;
    
    // Критическая ошибка
    logger.fatal("system", "Out of memory, service is shutting down").await?;
    
    Ok(())
} 