use crate::{
    ChatMessage, ChatChannel, AppState
};
use crate::domain::{
    ChatError, ApiResponse,
    SendMessageRequest, CreateChannelRequest, GetMessagesQuery,
    EditMessageRequest, StartVoiceSessionRequest, UpdateVoicePositionRequest,
};
use crate::application::services::{ChatService, ChannelService, VoiceService};
use crate::infrastructure::redis::RedisService;
use crate::interfaces::websocket::WebSocketServer;
use crate::config::ChatConfig;
use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

// ============================================================================
// Вспомогательные типы для аутентификации
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub username: String,
    pub roles: Vec<String>,
}

// TODO: Middleware для аутентификации JWT
// Пока используем заглушку
async fn extract_auth_user() -> Result<AuthUser, ChatError> {
    // В реальности здесь должна быть валидация JWT токена
    Ok(AuthUser {
        user_id: Uuid::new_v4(),
        username: "test_user".to_string(),
        roles: vec!["user".to_string()],
    })
}

// ============================================================================
// Handlers для сообщений
// ============================================================================

/// Отправить сообщение в канал
pub async fn send_message(
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
    Json(request): Json<SendMessageRequest>,
) -> Result<Json<ApiResponse<ChatMessage>>, ChatError> {
    let auth_user = extract_auth_user().await?;
    
    let message = state.chat_service
        .send_message(channel_id, auth_user.user_id, request)
        .await?;

    Ok(Json(ApiResponse::success(message)))
}

/// Получить историю сообщений канала
pub async fn get_messages(
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
    Query(query): Query<GetMessagesQuery>,
) -> Result<Json<ApiResponse<Vec<ChatMessage>>>, ChatError> {
    let auth_user = extract_auth_user().await?;
    
    let messages = state.chat_service
        .get_messages(channel_id, auth_user.user_id, query)
        .await?;

    Ok(Json(ApiResponse::success(messages)))
}

/// Редактировать сообщение
pub async fn edit_message(
    State(state): State<AppState>,
    Path(message_id): Path<Uuid>,
    Json(request): Json<EditMessageRequest>,
) -> Result<Json<ApiResponse<ChatMessage>>, ChatError> {
    let auth_user = extract_auth_user().await?;
    
    let message = state.chat_service
        .edit_message(message_id, auth_user.user_id, request)
        .await?;

    Ok(Json(ApiResponse::success(message)))
}

/// Удалить сообщение
pub async fn delete_message(
    State(state): State<AppState>,
    Path(message_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ChatError> {
    let auth_user = extract_auth_user().await?;
    
    state.chat_service
        .delete_message(message_id, auth_user.user_id)
        .await?;

    Ok(Json(ApiResponse::success(())))
}

/// Начать печатать в канале
pub async fn start_typing(
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ChatError> {
    let auth_user = extract_auth_user().await?;
    
    state.chat_service
        .start_typing(channel_id, auth_user.user_id)
        .await?;

    Ok(Json(ApiResponse::success(())))
}

/// Прекратить печатать в канале
pub async fn stop_typing(
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ChatError> {
    let auth_user = extract_auth_user().await?;
    
    state.chat_service
        .stop_typing(channel_id, auth_user.user_id)
        .await?;

    Ok(Json(ApiResponse::success(())))
}

// ============================================================================
// Handlers для каналов
// ============================================================================

/// Создать новый канал
pub async fn create_channel(
    State(state): State<AppState>,
    Json(request): Json<CreateChannelRequest>,
) -> Result<Json<ApiResponse<ChatChannel>>, ChatError> {
    let auth_user = extract_auth_user().await?;
    
    let channel = state.channel_service
        .create_channel(auth_user.user_id, request)
        .await?;

    Ok(Json(ApiResponse::success(channel)))
}

/// Получить информацию о канале
pub async fn get_channel(
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
) -> Result<Json<ApiResponse<ChatChannel>>, ChatError> {
    let channel = state.channel_service
        .get_channel(channel_id)
        .await?;

    Ok(Json(ApiResponse::success(channel)))
}

/// Получить список каналов пользователя
pub async fn get_user_channels(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ChatChannel>>>, ChatError> {
    let auth_user = extract_auth_user().await?;
    
    let channels = state.channel_service
        .get_user_channels(auth_user.user_id)
        .await?;

    Ok(Json(ApiResponse::success(channels)))
}

/// Alias для get_user_channels для совместимости
pub async fn get_channels(
    state: State<AppState>,
) -> Result<Json<ApiResponse<Vec<ChatChannel>>>, ChatError> {
    get_user_channels(state).await
}

/// Присоединиться к каналу
pub async fn join_channel(
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ChatError> {
    let auth_user = extract_auth_user().await?;
    
    state.channel_service
        .join_channel(channel_id, auth_user.user_id)
        .await?;

    Ok(Json(ApiResponse::success(())))
}

/// Покинуть канал
pub async fn leave_channel(
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ChatError> {
    let auth_user = extract_auth_user().await?;
    
    state.channel_service
        .leave_channel(channel_id, auth_user.user_id)
        .await?;

    Ok(Json(ApiResponse::success(())))
}

// ============================================================================
// Handlers для голосового чата
// ============================================================================

/// Начать голосовую сессию
pub async fn start_voice_session(
    State(state): State<AppState>,
    Json(request): Json<StartVoiceSessionRequest>,
) -> Result<Json<ApiResponse<crate::VoiceSession>>, ChatError> {
    let auth_user = extract_auth_user().await?;
    
    let session = state.voice_service
        .start_voice_session(auth_user.user_id, request)
        .await?;

    Ok(Json(ApiResponse::success(session)))
}

/// Завершить голосовую сессию
pub async fn end_voice_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ChatError> {
    let auth_user = extract_auth_user().await?;
    
    state.voice_service
        .end_voice_session(session_id, auth_user.user_id)
        .await?;

    Ok(Json(ApiResponse::success(())))
}

/// Обновить позицию в голосовой сессии
pub async fn update_voice_position(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Json(request): Json<UpdateVoicePositionRequest>,
) -> Result<Json<ApiResponse<()>>, ChatError> {
    let auth_user = extract_auth_user().await?;
    
    state.voice_service
        .update_voice_position(session_id, auth_user.user_id, request)
        .await?;

    Ok(Json(ApiResponse::success(())))
}

// ============================================================================
// Health check
// ============================================================================

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now(),
    })
}

// ============================================================================
// Создание маршрутов
// ============================================================================

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Сообщения
        .route("/channels/:channel_id/messages", post(send_message))
        .route("/channels/:channel_id/messages", get(get_messages))
        .route("/messages/:message_id", put(edit_message))
        .route("/messages/:message_id", delete(delete_message))
        
        // Typing индикаторы
        .route("/channels/:channel_id/typing/start", post(start_typing))
        .route("/channels/:channel_id/typing/stop", post(stop_typing))
        
        // Каналы
        .route("/channels", post(create_channel))
        .route("/channels", get(get_channels))
        .route("/channels/:channel_id", get(get_channel))
        .route("/channels/:channel_id/join", post(join_channel))
        .route("/channels/:channel_id/leave", post(leave_channel))
        
        // Голосовой чат
        .route("/voice/sessions", post(start_voice_session))
        .route("/voice/sessions/:session_id", delete(end_voice_session))
        .route("/voice/sessions/:session_id/position", put(update_voice_position))
        
        .layer(CorsLayer::permissive())
        .with_state(state)
}

// ============================================================================
// Запуск HTTP сервера
// ============================================================================

pub async fn start_http_server(
    config: Arc<ChatConfig>,
    chat_service: Arc<ChatService>,
    channel_service: Arc<ChannelService>,
    voice_service: Arc<VoiceService>,
    redis_service: Arc<RedisService>,
    websocket_server: Arc<WebSocketServer>,
) -> Result<(), ChatError> {
    let state = AppState {
        chat_service,
        channel_service,
        voice_service,
        redis_service,
        websocket_server,
        config: config.clone(),
    };

    let app = create_routes(state);
    
    let addr = format!("{}:{}", config.server.host, config.server.http_port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| ChatError::Internal { 
            message: format!("Failed to bind to {}: {}", addr, e) 
        })?;

    tracing::info!("HTTP server listening on {}", addr);

    // Используем правильный способ для axum 0.7
    axum::serve(listener, app)
        .await
        .map_err(|e| ChatError::Internal { 
            message: format!("HTTP server error: {}", e) 
        })?;

    Ok(())
} 