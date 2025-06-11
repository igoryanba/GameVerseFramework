use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error};

mod api;
mod config;
mod domain;
mod infrastructure;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Инициализируем логирование
    init_logging()?;
    
    info!("Запуск сервиса аутентификации GameVerse Framework");
    
    // Загружаем конфигурацию
    let config = match config::load_config() {
        Ok(cfg) => {
            info!("Конфигурация загружена успешно");
            cfg
        },
        Err(e) => {
            error!("Ошибка загрузки конфигурации: {}", e);
            return Err(anyhow::anyhow!("Ошибка загрузки конфигурации: {}", e));
        }
    };
    
    // Создаем соединение с базой данных
    let db_pool = match infrastructure::database::create_pool(&config.database).await {
        Ok(pool) => {
            info!("Соединение с базой данных установлено");
            pool
        },
        Err(e) => {
            error!("Ошибка соединения с базой данных: {}", e);
            return Err(anyhow::anyhow!("Ошибка соединения с базой данных: {}", e));
        }
    };
    
    // Создаем соединение с Redis
    let redis_client = match infrastructure::cache::create_redis_multiplexed_connection(&config.database).await {
        Ok(client) => {
            info!("Соединение с Redis установлено");
            Arc::new(Mutex::new(client))
        },
        Err(e) => {
            error!("Ошибка соединения с Redis: {}", e);
            return Err(anyhow::anyhow!("Ошибка соединения с Redis: {}", e));
        }
    };
    
    // Создаем репозиторий пользователей
    let user_repository = Arc::new(domain::repositories::UserRepository::new(db_pool.clone()));
    
    // Создаем сервис аутентификации
    let auth_service = Arc::new(domain::services::AuthService::new(
        user_repository.clone(),
        redis_client.clone(),
        config.auth.clone(),
    )?);
    
    // Запускаем HTTP сервер
    let rest_server = api::rest::create_server(
        &config.server,
        auth_service.clone(),
    );
    
    // Запускаем gRPC сервер
    let grpc_server = api::grpc::create_server(
        &config.server,
        auth_service.clone(),
    );
    
    info!(
        "Сервис аутентификации запущен на REST {}:{} и gRPC {}:{}",
        config.server.rest_host, config.server.rest_port,
        config.server.grpc_host, config.server.grpc_port
    );
    
    // Запускаем оба сервера параллельно
    tokio::select! {
        result = rest_server => {
            if let Err(e) = result {
                error!("Ошибка REST сервера: {}", e);
                return Err(anyhow::anyhow!("Ошибка REST сервера: {}", e));
            }
        }
        result = grpc_server => {
            if let Err(e) = result {
                error!("Ошибка gRPC сервера: {}", e);
                return Err(anyhow::anyhow!("Ошибка gRPC сервера: {}", e));
            }
        }
    }
    
    Ok(())
}

/// Инициализирует систему логирования
fn init_logging() -> Result<()> {
    use tracing_subscriber::{
        fmt,
        EnvFilter,
        util::SubscriberInitExt,
        layer::SubscriberExt,
    };
    
    // Настраиваем фильтр на основе переменной окружения или уровня по умолчанию
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("gameverse_auth=debug,actix_web=info,tower_http=info")
    });
    
    // Создаем форматтер для консоли
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_ansi(true);
    
    // Настраиваем подписчика
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .try_init()
        .map_err(|e| anyhow::anyhow!("Ошибка инициализации логирования: {}", e))?;
    
    Ok(())
} 