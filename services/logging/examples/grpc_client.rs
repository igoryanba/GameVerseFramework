use gameverse_logging::{LogLevel, Logger, ConsoleLogHandler, FileLogHandler};
use gameverse_logging::grpc::GrpcLogHandler;
use uuid::Uuid;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Пример клиента gRPC для отправки логов");
    
    // Создаем логгер для локального сервиса
    let logger = Logger::new("example-client");
    
    // Добавляем обработчик вывода в консоль для локальных логов
    logger.add_handler(ConsoleLogHandler).await;
    
    logger.info("startup", "Запуск примера gRPC клиента...").await?;
    
    // Настраиваем подключение к gRPC серверу логирования
    // Примечание: сервер должен быть запущен на указанном адресе
    let server_addr = std::env::var("LOGGING_SERVER").unwrap_or_else(|_| "127.0.0.1:50051".to_string());
    
    logger.info("config", &format!("Подключение к серверу логирования по адресу: {}", server_addr)).await?;
    
    // Создаем gRPC обработчик логов
    match GrpcLogHandler::new("example-client", &server_addr).await {
        Ok(grpc_handler) => {
            // Добавляем gRPC обработчик в логгер
            logger.add_handler(grpc_handler).await;
            logger.info("grpc", "Успешное подключение к серверу логирования").await?;
            
            // Теперь отправляем логи, которые будут направлены и в консоль, и на сервер
            logger.debug("example", "Это отладочное сообщение").await?;
            
            // Генерируем уникальный ID запроса
            let request_id = Uuid::new_v4();
            
            // Логируем с контекстом и метриками
            for i in 1..=5 {
                // Имитируем выполнение операции
                let start = std::time::Instant::now();
                tokio::time::sleep(Duration::from_millis(100 * i)).await;
                let duration = start.elapsed().as_millis() as u64;
                
                // Создаем запись лога с контекстом
                let log_entry = gameverse_logging::LogEntry::new(
                    LogLevel::Info,
                    "example-client",
                    "operation",
                    format!("Выполнена операция #{}", i)
                )
                .with_request_id(request_id)
                .with_user_id("test-user")
                .with_data("iteration", i)
                .with_data("status", "success")
                .with_duration(duration);
                
                // Отправляем лог
                logger.log_entry(log_entry).await?;
            }
            
            // Генерируем ошибку
            logger.error(
                "database", 
                "Не удалось подключиться к базе данных: timeout after 5000ms"
            ).await?;
            
            // Ждем, чтобы убедиться, что все логи отправлены
            tokio::time::sleep(Duration::from_secs(6)).await;
            
            logger.info("shutdown", "Завершение работы примера").await?;
        },
        Err(e) => {
            logger.error("grpc", &format!("Ошибка подключения к серверу логирования: {}", e)).await?;
        }
    }
    
    Ok(())
} 