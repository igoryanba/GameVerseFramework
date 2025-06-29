use std::sync::Arc;
use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber::{fmt, EnvFilter};

mod domain;
mod api;
mod infrastructure;
mod config;
mod utils;

use crate::domain::repositories::PlayerRepository;
use crate::domain::services::PlayerService;
use crate::api::rest::{AppState, create_app};

/// Main application entry point
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("🚀 Starting GameVerse Player Data Service");

    // Load configuration
    let config = load_config().await?;
    
    // Initialize database connection
    let database_pool = initialize_database(&config.database_url).await?;
    
    // Run database migrations
    run_migrations(&database_pool).await?;
    
    // Initialize repositories
    let player_repository = Arc::new(PlayerRepository::new(database_pool.clone()));
    
    // Initialize services
    let player_service = Arc::new(PlayerService::new(player_repository));
    
    // Create application state
    let state = AppState {
        player_service,
    };
    
    // Build application with routes
    let app = create_app(state);
    
    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await?;
    
    info!("🌐 Server listening on http://{}", addr);
    info!("📖 API Documentation available at http://{}/api-docs/", addr);
    info!("💗 Health check available at http://{}/health", addr);
    
    // Start serving requests
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// Configuration structure
#[derive(Debug, Clone)]
struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub log_level: String,
}

/// Load application configuration
async fn load_config() -> Result<Config> {
    // Load from environment variables with fallbacks
    let config = Config {
        host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        port: std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080),
        database_url: std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://gameverse:gameverse@localhost:5432/gameverse_player_data".to_string()),
        log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
    };
    
    info!("📋 Configuration loaded:");
    info!("  Host: {}", config.host);
    info!("  Port: {}", config.port);
    info!("  Database: {}", mask_database_url(&config.database_url));
    info!("  Log Level: {}", config.log_level);
    
    Ok(config)
}

/// Initialize database connection pool
async fn initialize_database(database_url: &str) -> Result<Pool<Postgres>> {
    info!("🔗 Connecting to database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .connect(database_url)
        .await?;
    
    info!("✅ Database connection established");
    
    Ok(pool)
}

/// Run database migrations
async fn run_migrations(pool: &Pool<Postgres>) -> Result<()> {
    info!("📦 Running database migrations...");
    
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    
    info!("✅ Database migrations completed");
    
    Ok(())
}

/// Mask sensitive parts of database URL for logging
fn mask_database_url(url: &str) -> String {
    if let Ok(parsed) = url::Url::parse(url) {
        let masked_password = if parsed.password().is_some() {
            "***"
        } else {
            ""
        };
        
        format!(
            "{}://{}:{}@{}:{}/{}",
            parsed.scheme(),
            parsed.username(),
            masked_password,
            parsed.host_str().unwrap_or("localhost"),
            parsed.port().unwrap_or(5432),
            parsed.path().trim_start_matches('/')
        )
    } else {
        "invalid_url".to_string()
    }
}

/// Graceful shutdown handler
async fn shutdown_signal() {
    use tokio::signal;
    
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };
    
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };
    
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    
    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal");
        },
        _ = terminate => {
            info!("Received terminate signal");
        },
    }
    
    info!("🛑 Shutting down gracefully...");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mask_database_url() {
        let url = "postgresql://user:password@localhost:5432/database";
        let masked = mask_database_url(url);
        assert_eq!(masked, "postgresql://user:***@localhost:5432/database");
    }
    
    #[test]
    fn test_mask_database_url_without_password() {
        let url = "postgresql://user@localhost:5432/database";
        let masked = mask_database_url(url);
        assert_eq!(masked, "postgresql://user:@localhost:5432/database");
    }
}