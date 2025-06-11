pub mod handlers;
pub mod middleware;
pub mod responses;

use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router,
    middleware::from_fn_with_state,
};
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;
use hyper::Server;
use tracing::info;

use crate::config::ServerConfig;
use crate::domain::services::AuthService;
use self::middleware::auth_middleware;

/// Создает и настраивает HTTP сервер
pub async fn create_server(
    config: &ServerConfig,
    auth_service: Arc<AuthService>,
) -> Result<(), anyhow::Error> {
    info!("Запуск HTTP сервера на {}", config.rest_address());
    
    let app = create_router(auth_service);
    
    let listener = tokio::net::TcpListener::bind(config.rest_address()).await?;
    
    let server = Server::from_tcp(listener.into_std()?)?;
    server.serve(app.into_make_service()).await?;
    
    Ok(())
}

/// Создает маршруты для REST API
fn create_router(auth_service: Arc<AuthService>) -> Router {
    Router::new()
        // Публичные маршруты (без аутентификации)
        .route("/health", get(handlers::health_check))
        .route("/auth/register", post(handlers::register))
        .route("/auth/login", post(handlers::login))
        .route("/auth/refresh", post(handlers::refresh_token))
        
        // Защищенные маршруты (с аутентификацией)
        .route("/auth/logout", post(handlers::logout))
        .route("/auth/profile", get(handlers::get_profile))
        .layer(from_fn_with_state(auth_service.clone(), middleware::auth_middleware))
        
        // Middleware
        .layer(CorsLayer::permissive())
        .with_state(auth_service)
} 