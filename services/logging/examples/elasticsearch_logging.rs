use gameverse_logging::{LogLevel, Logger, ConsoleLogHandler, FileLogHandler};
use gameverse_logging::elasticsearch::ElasticsearchLogHandler;
use std::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Пример логирования в ElasticSearch");
    
    // Создаем логгер для сервиса
    let logger = Logger::new("elasticsearch-example");
    
    // Добавляем обработчик вывода в консоль для локальных логов
    logger.add_handler(ConsoleLogHandler).await;
    
    // Добавляем обработчик вывода в файл
    logger.add_handler(FileLogHandler::new("logs/elasticsearch-example.log")).await;
    
    logger.info("startup", "Запуск примера логирования в ElasticSearch...").await?;
    
    // Получаем настройки ElasticSearch из переменных окружения или используем значения по умолчанию
    let es_url = std::env::var("ELASTICSEARCH_URL").unwrap_or_else(|_| "http://localhost:9200".to_string());
    let es_index = std::env::var("ELASTICSEARCH_INDEX").unwrap_or_else(|_| "gameverse-logs".to_string());
    
    logger.info("config", &format!("Подключение к ElasticSearch по адресу: {}", es_url)).await?;
    
    // Создаем обработчик ElasticSearch
    match ElasticsearchLogHandler::new(&es_url, &es_index, None, None).await {
        Ok(es_handler) => {
            // Добавляем обработчик ElasticSearch в логгер
            logger.add_handler(es_handler).await;
            logger.info("elasticsearch", "Успешное подключение к ElasticSearch").await?;
            
            // Отправляем различные типы логов
            for i in 1..=10 {
                let level = match i % 5 {
                    0 => LogLevel::Trace,
                    1 => LogLevel::Debug,
                    2 => LogLevel::Info,
                    3 => LogLevel::Warn,
                    _ => LogLevel::Error,
                };
                
                let request_id = Uuid::new_v4();
                
                // Имитируем выполнение операции
                let start = std::time::Instant::now();
                tokio::time::sleep(Duration::from_millis(50 * i)).await;
                let duration = start.elapsed().as_millis() as u64;
                
                // Создаем запись лога с контекстом и метриками
                let log_entry = gameverse_logging::LogEntry::new(
                    level,
                    "elasticsearch-example",
                    "test-operation",
                    format!("Тестовое сообщение #{}", i)
                )
                .with_request_id(request_id)
                .with_user_id(format!("test-user-{}", i % 3))
                .with_entity_id(format!("test-entity-{}", i))
                .with_data("test_param", format!("value-{}", i))
                .with_data("tags", vec!["test", "example", "elasticsearch"])
                .with_duration(duration)
                .with_memory_usage(1024 * i);
                
                // Отправляем лог
                logger.log_entry(log_entry).await?;
                
                // Небольшая пауза между логами
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            
            // Генерируем ошибку
            logger.error(
                "test-error", 
                "Тестовая ошибка для проверки логирования ошибок"
            ).await?;
            
            // Ждем, чтобы убедиться, что все логи отправлены (ElasticSearch обработчик отправляет пакетами)
            logger.info("wait", "Ожидание отправки всех логов...").await?;
            tokio::time::sleep(Duration::from_secs(6)).await;
            
            logger.info("shutdown", "Завершение работы примера").await?;
        },
        Err(e) => {
            logger.error("elasticsearch", &format!("Ошибка подключения к ElasticSearch: {}", e)).await?;
            logger.warn("fallback", "Логирование только в консоль и файл").await?;
            
            // Если не удалось подключиться к ElasticSearch, все равно выполняем некоторое логирование
            for i in 1..=3 {
                logger.info("fallback-log", &format!("Тестовый лог #{} без ElasticSearch", i)).await?;
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }
    
    Ok(())
} 