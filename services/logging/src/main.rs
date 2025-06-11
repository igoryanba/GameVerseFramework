use gameverse_logging::{LogLevel, Logger, ConsoleLogHandler, FileLogHandler};
use gameverse_logging::grpc::LoggingServiceImpl;
use std::env;
use std::path::Path;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Запуск сервиса логирования GameVerse Framework...");
    
    // Настройка базового логирования для самого сервиса логирования
    let log_dir = env::var("GAMEVERSE_LOG_DIR").unwrap_or_else(|_| "logs".to_string());
    
    // Создаем директорию для логов, если она не существует
    if !Path::new(&log_dir).exists() {
        std::fs::create_dir_all(&log_dir)?;
    }
    
    let log_file = format!("{}/logging-service.log", log_dir);
    let service_logger = Arc::new(Logger::new("logging-service"));
    
    // Добавляем обработчики для консоли и файла
    service_logger.add_handler(ConsoleLogHandler).await;
    service_logger.add_handler(FileLogHandler::new(&log_file)).await;
    
    // Логируем запуск
    service_logger.info("startup", "Logging service starting...").await?;
    
    // Получаем конфигурацию
    let host = env::var("GAMEVERSE_LOG_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("GAMEVERSE_LOG_PORT").unwrap_or_else(|_| "50051".to_string());
    let address = format!("{}:{}", host, port);
    
    service_logger.info("config", &format!("Server configured to listen on {}", address)).await?;
    
    // Настройка и запуск gRPC сервера
    service_logger.info("grpc", "Setting up gRPC server...").await?;
    
    // Создаем адрес для сервера
    let addr = address.parse()?;
    
    // Создаем сервис логирования
    let logging_service = LoggingServiceImpl::new(service_logger.clone());
    
    // Логируем готовность к запуску
    service_logger.info("server", "Starting gRPC server...").await?;
    
    // Запускаем gRPC сервер
    tonic::transport::Server::builder()
        .add_service(gameverse_logging::grpc::proto::logging_service_server::LoggingServiceServer::new(logging_service))
        .serve(addr)
        .await?;
    
    service_logger.info("shutdown", "Logging service shutting down").await?;
    
    Ok(())
} 