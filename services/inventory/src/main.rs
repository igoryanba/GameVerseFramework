use gameverse_inventory::{
    Config,
    repositories::{
        PostgresInventoryRepository, PostgresItemRepository, PostgresItemTemplateRepository
    },
    services::{InventoryService, ItemService},
    http::server::create_http_server,
    grpc::service::GrpcInventoryService
};
use gameverse_logging::Logger;
use sqlx::PgPool;
use std::sync::Arc;
use std::net::SocketAddr;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🚀 Запуск микросервиса инвентаря GameVerse Framework...");
    
    // Загружаем конфигурацию
    let config = Config::from_env();
    
    // Настраиваем логирование
    let logger = Arc::new(Logger::new("inventory-service"));
    logger.add_handler(gameverse_logging::ConsoleLogHandler).await;
    
    println!("✅ Конфигурация загружена:");
    println!("   - GRPC: {}:{}", config.grpc.host, config.grpc.port);
    println!("   - HTTP: {}:{}", config.http.host, config.http.port);
    println!("   - Database: {}", config.database.url);
    
    logger.info("startup", "Сервис инвентаря GameVerse Framework запущен").await.ok();
    logger.info("config", &format!("Конфигурация: GRPC {}:{}, HTTP {}:{}", 
        config.grpc.host, config.grpc.port, config.http.host, config.http.port)).await.ok();
    
    // Подключение к базе данных
    logger.info("database", "Подключение к базе данных PostgreSQL...").await.ok();
    println!("🔌 Подключение к базе данных...");
    
    let pool = PgPool::connect(&config.database.url)
        .await
        .map_err(|e| {
            eprintln!("❌ Ошибка подключения к базе данных: {}", e);
            e
        })?;
    
    println!("✅ Подключение к базе данных установлено");
    logger.info("database", "Успешное подключение к PostgreSQL").await.ok();
    
    // Создание репозиториев
    println!("🏗️  Инициализация репозиториев...");
    let inventory_repository = Arc::new(PostgresInventoryRepository::new(pool.clone()));
    let item_repository = Arc::new(PostgresItemRepository::new(pool.clone()));
    let item_template_repository = Arc::new(PostgresItemTemplateRepository::new(pool.clone()));
    
    // Создание сервисов
    println!("⚙️  Инициализация сервисов...");
    let inventory_service = Arc::new(InventoryService::new(
        inventory_repository.clone(),
        item_repository.clone(),
    ));
    let item_service = Arc::new(ItemService::new(
        item_repository.clone(),
        item_template_repository.clone(),
        inventory_service.clone(),
    ));
    
    logger.info("services", "Сервисы инициализированы").await.ok();
    
    // Подготовка адресов серверов
    let http_addr: SocketAddr = format!("{}:{}", config.http.host, config.http.port)
        .parse()
        .map_err(|e| {
            eprintln!("❌ Некорректный адрес HTTP сервера: {}", e);
            e
        })?;
    
    let grpc_addr: SocketAddr = format!("{}:{}", config.grpc.host, config.grpc.port)
        .parse()
        .map_err(|e| {
            eprintln!("❌ Некорректный адрес gRPC сервера: {}", e);
            e
        })?;
    
    println!("🎯 Микросервис инвентаря готов к работе!");
    println!("📋 Доступные endpoints:");
    println!("   - HTTP API: http://{}", http_addr);
    println!("   - gRPC API: grpc://{}", grpc_addr);
    println!("   - Документация: http://{}/docs (TODO)", http_addr);
    println!("   - Health check: http://{}/health (TODO)", http_addr);
    
    // Создание gRPC сервера
    let grpc_service = GrpcInventoryService::create_server(
        inventory_service.clone(),
        item_service.clone(),
    );
    
    println!("🌐 Запуск серверов...");
    logger.info("servers", "Запуск HTTP и gRPC серверов").await.ok();
    
    // Запуск обоих серверов параллельно
    let http_server = tokio::spawn(async move {
        println!("🌐 HTTP сервер запускается на {}...", http_addr);
        create_http_server(inventory_service, item_service, http_addr).await
    });
    
    let grpc_server = tokio::spawn(async move {
        println!("🔗 gRPC сервер запускается на {}...", grpc_addr);
        Server::builder()
            .add_service(grpc_service)
            .serve(grpc_addr)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    });
    
    // Ждем завершения любого из серверов
    tokio::select! {
        http_result = http_server => {
            match http_result {
                Ok(Ok(_)) => println!("✅ HTTP сервер завершил работу"),
                Ok(Err(e)) => {
                    eprintln!("❌ Ошибка HTTP сервера: {}", e);
                    logger.info("http_error", &format!("Ошибка HTTP сервера: {}", e)).await.ok();
                    return Err(e.into());
                }
                Err(e) => {
                    eprintln!("❌ Ошибка задачи HTTP сервера: {}", e);
                    return Err(e.into());
                }
            }
        }
        grpc_result = grpc_server => {
            match grpc_result {
                Ok(Ok(_)) => println!("✅ gRPC сервер завершил работу"),
                Ok(Err(e)) => {
                    eprintln!("❌ Ошибка gRPC сервера: {}", e);
                    logger.info("grpc_error", &format!("Ошибка gRPC сервера: {}", e)).await.ok();
                    return Err(e.into());
                }
                Err(e) => {
                    eprintln!("❌ Ошибка задачи gRPC сервера: {}", e);
                    return Err(e.into());
                }
            }
        }
    }
    
    // Graceful shutdown
    logger.info("shutdown", "Завершение работы сервиса инвентаря").await.ok();
    println!("👋 Сервис инвентаря остановлен");
    
    Ok(())
} 