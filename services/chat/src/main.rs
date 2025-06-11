use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;
use dotenv;

use axum::{
    routing::get,
    Router,
    extract::{DefaultBodyLimit, State},
    Json,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use serde_json::json;

use gameverse_chat::{
    AppState, ChatService, ChannelService, VoiceService, 
    MessageRepository, ChannelRepository, RedisService, ChatConfig
};
use gameverse_chat::interfaces::{
    http::health_check,
    websocket::WebSocketServer,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация логирования
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "gameverse_chat=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Загрузка переменных окружения
    dotenv::dotenv().ok();

    // Загрузка конфигурации
    let config = ChatConfig::default();

    info!("Starting GameVerse Chat Service");
    info!("HTTP Server will run on {}:{}", config.server.host, config.server.http_port);
    info!("gRPC Server will run on {}:{}", config.server.host, config.server.grpc_port);
    info!("WebSocket Server will run on {}:{}", config.server.host, config.server.websocket_port);
    
    if config.telegram.enabled {
        info!("Telegram integration enabled");
    }

    // Создаем канал для событий чата
    let (event_sender, _event_receiver) = mpsc::channel(1000);

    // Инициализация базы данных
    info!("Initializing database connection...");
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://gameverse:gameverse@localhost:5434/gameverse_chat".to_string());
    
    let pool = sqlx::PgPool::connect(&database_url).await?;
    
    // Запуск миграций
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    // Создание сервисов
    let message_repository = Arc::new(MessageRepository::new(pool.clone()));
    let channel_repository = Arc::new(ChannelRepository::new(pool.clone()));
    
    let redis_config = gameverse_chat::infrastructure::redis::RedisConfig {
        url: std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
        max_connections: 10,
        pool_timeout: 30,
        command_timeout: 5,
    };
    let redis_service = Arc::new(RedisService::new(&redis_config).await?);
    info!("Redis initialized successfully");

    // Создание сервисов приложения
    let chat_service = Arc::new(ChatService::new(
        message_repository.clone(),
        channel_repository.clone(),
        redis_service.clone(),
        Arc::new(config.clone()),
        event_sender.clone(),
    ));

    let channel_service = Arc::new(ChannelService::new(
        channel_repository.clone(),
        redis_service.clone(),
        Arc::new(config.clone()),
        event_sender.clone(),
    ));

    let voice_service = Arc::new(VoiceService::new(
        redis_service.clone(),
        Arc::new(config.clone()),
        event_sender.clone(),
    ));

    // Создание WebSocket сервера
    let websocket_server = Arc::new(WebSocketServer::new(
        chat_service.clone(),
        voice_service.clone(),
    ));

    // Настройка CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    // Создание состояния приложения
    let app_state = AppState {
        chat_service,
        channel_service,
        voice_service,
        redis_service,
        websocket_server: websocket_server.clone(),
        config: Arc::new(config),
    };
    
    // Роуты для основного API (используют AppState)
    let api_routes = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/ws/stats", get(get_websocket_stats))
        .route("/api/v1/ws/users", get(get_active_users))
        .with_state(app_state);

    // Роуты для WebSocket (используют Arc<WebSocketServer>)
    let ws_routes = Router::new()
        .route("/ws", get(WebSocketServer::handle_websocket))
        .with_state(websocket_server);
    
    // Объединяем роуты
    let app = Router::new()
        .merge(api_routes)
        .merge(ws_routes)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
                .layer(DefaultBodyLimit::max(16 * 1024 * 1024)) // 16MB
        );
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    
    tracing::info!("🚀 GameVerse Chat Server запущен на http://0.0.0.0:8080");
    tracing::info!("📡 WebSocket endpoint: ws://0.0.0.0:8080/ws");
    tracing::info!("🏥 Health check: http://0.0.0.0:8080/health");
    tracing::info!("📝 API документация будет доступна в будущем");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// Получение статистики WebSocket соединений
async fn get_websocket_stats(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let connection_count = state.websocket_server.get_connection_count().await;
    
    Json(json!({
        "active_connections": connection_count,
        "server_uptime": "TODO", // Добавить позже
        "total_messages_sent": "TODO", // Добавить счетчик
        "status": "healthy"
    }))
}

/// Получение списка активных пользователей
async fn get_active_users(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    let users = state.websocket_server.get_active_users().await;
    
    let user_list: Vec<serde_json::Value> = users
        .into_iter()
        .map(|(user_id, username, position)| {
            json!({
                "user_id": user_id,
                "username": username,
                "position": position,
                "status": "online"
            })
        })
        .collect();
    
    Json(json!({
        "users": user_list,
        "total_count": user_list.len()
    }))
} 