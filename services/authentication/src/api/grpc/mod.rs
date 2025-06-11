use std::sync::Arc;
use anyhow::Result;
use tracing::info;

use crate::config::ServerConfig;
use crate::domain::services::AuthService;

/// Создает и запускает gRPC сервер
pub async fn create_server(
    config: &ServerConfig,
    auth_service: Arc<AuthService>,
) -> Result<()> {
    info!("gRPC сервер аутентификации будет реализован в следующих итерациях");
    info!("Планируемый адрес gRPC сервера: {}", config.grpc_address());
    
    // Заглушка - в реальной реализации здесь будет gRPC сервер
    // Пока что просто ждем бесконечно
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
} 