use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChatError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    
    #[error("Telegram API error: {0}")]
    Telegram(String),
    
    #[error("Authentication error: {message}")]
    Authentication { message: String },
    
    #[error("Authorization error: {message}")]
    Authorization { message: String },
    
    #[error("Message not found: {id}")]
    MessageNotFound { id: String },
    
    #[error("Channel not found: {id}")]
    ChannelNotFound { id: String },
    
    #[error("User not found: {id}")]
    UserNotFound { id: String },
    
    #[error("Voice session not found: {id}")]
    VoiceSessionNotFound { id: String },
    
    #[error("Rate limit exceeded: {message}")]
    RateLimit { message: String },
    
    #[error("Message too long: {length} > {max_length}")]
    MessageTooLong { length: usize, max_length: usize },
    
    #[error("Channel full: {current} >= {max}")]
    ChannelFull { current: u32, max: u32 },
    
    #[error("Profanity detected in message")]
    ProfanityDetected,
    
    #[error("Invalid channel type for operation")]
    InvalidChannelType,
    
    #[error("Voice session already active")]
    VoiceSessionActive,
    
    #[error("Radio frequency not available: {frequency}")]
    RadioFrequencyUnavailable { frequency: String },
    
    #[error("Missing required item: {item}")]
    MissingRequiredItem { item: String },
    
    #[error("User is muted until: {until}")]
    UserMuted { until: chrono::DateTime<chrono::Utc> },
    
    #[error("User is banned from channel")]
    UserBanned,
    
    #[error("Invalid proximity: too far from other players")]
    InvalidProximity,
    
    #[error("Configuration error: {message}")]
    Configuration { message: String },
    
    #[error("External service error: {service} - {message}")]
    ExternalService { service: String, message: String },
    
    #[error("Internal server error: {message}")]
    Internal { message: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }
    
    pub fn error(error: ChatError) -> ApiResponse<()> {
        let api_error = ApiError::from(error);
        ApiResponse {
            success: false,
            data: None,
            error: Some(api_error),
            timestamp: chrono::Utc::now(),
        }
    }
}

impl From<ChatError> for ApiError {
    fn from(error: ChatError) -> Self {
        let (code, message, details) = match &error {
            ChatError::Database(_) => {
                ("DATABASE_ERROR".to_string(), "Ошибка базы данных".to_string(), None)
            }
            ChatError::Redis(_) => {
                ("REDIS_ERROR".to_string(), "Ошибка Redis".to_string(), None)
            }
            ChatError::Validation(errors) => {
                ("VALIDATION_ERROR".to_string(), 
                 "Ошибка валидации данных".to_string(), 
                 Some(serde_json::to_value(errors).unwrap_or_default()))
            }
            ChatError::Authentication { message } => {
                ("AUTHENTICATION_ERROR".to_string(), message.clone(), None)
            }
            ChatError::Authorization { message } => {
                ("AUTHORIZATION_ERROR".to_string(), message.clone(), None)
            }
            ChatError::MessageNotFound { id } => {
                ("MESSAGE_NOT_FOUND".to_string(), 
                 format!("Сообщение не найдено: {}", id), None)
            }
            ChatError::ChannelNotFound { id } => {
                ("CHANNEL_NOT_FOUND".to_string(), 
                 format!("Канал не найден: {}", id), None)
            }
            ChatError::UserNotFound { id } => {
                ("USER_NOT_FOUND".to_string(), 
                 format!("Пользователь не найден: {}", id), None)
            }
            ChatError::RateLimit { message } => {
                ("RATE_LIMIT_EXCEEDED".to_string(), message.clone(), None)
            }
            ChatError::MessageTooLong { length, max_length } => {
                ("MESSAGE_TOO_LONG".to_string(), 
                 format!("Сообщение слишком длинное: {} > {}", length, max_length), None)
            }
            ChatError::ChannelFull { current, max } => {
                ("CHANNEL_FULL".to_string(), 
                 format!("Канал переполнен: {} >= {}", current, max), None)
            }
            ChatError::ProfanityDetected => {
                ("PROFANITY_DETECTED".to_string(), 
                 "Обнаружена нецензурная лексика".to_string(), None)
            }
            ChatError::UserMuted { until } => {
                ("USER_MUTED".to_string(), 
                 format!("Пользователь заглушен до: {}", until), None)
            }
            ChatError::UserBanned => {
                ("USER_BANNED".to_string(), 
                 "Пользователь забанен в канале".to_string(), None)
            }
            ChatError::RadioFrequencyUnavailable { frequency } => {
                ("RADIO_FREQUENCY_UNAVAILABLE".to_string(), 
                 format!("Радиочастота недоступна: {}", frequency), None)
            }
            ChatError::MissingRequiredItem { item } => {
                ("MISSING_REQUIRED_ITEM".to_string(), 
                 format!("Отсутствует необходимый предмет: {}", item), None)
            }
            _ => {
                ("INTERNAL_ERROR".to_string(), "Внутренняя ошибка сервера".to_string(), None)
            }
        };
        
        Self { code, message, details }
    }
}

impl IntoResponse for ChatError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            ChatError::Authentication { .. } => StatusCode::UNAUTHORIZED,
            ChatError::Authorization { .. } => StatusCode::FORBIDDEN,
            ChatError::MessageNotFound { .. } 
            | ChatError::ChannelNotFound { .. }
            | ChatError::UserNotFound { .. }
            | ChatError::VoiceSessionNotFound { .. } => StatusCode::NOT_FOUND,
            ChatError::Validation { .. } 
            | ChatError::MessageTooLong { .. }
            | ChatError::InvalidChannelType
            | ChatError::ProfanityDetected
            | ChatError::MissingRequiredItem { .. }
            | ChatError::InvalidProximity => StatusCode::BAD_REQUEST,
            ChatError::RateLimit { .. } => StatusCode::TOO_MANY_REQUESTS,
            ChatError::ChannelFull { .. } 
            | ChatError::VoiceSessionActive
            | ChatError::RadioFrequencyUnavailable { .. } => StatusCode::CONFLICT,
            ChatError::UserMuted { .. } 
            | ChatError::UserBanned => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        
        let response = ApiResponse::<()>::error(self);
        (status_code, Json(response)).into_response()
    }
}

// Удобные конструкторы для часто используемых ошибок
impl ChatError {
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Authentication { message: message.into() }
    }
    
    pub fn authorization(message: impl Into<String>) -> Self {
        Self::Authorization { message: message.into() }
    }
    
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal { message: message.into() }
    }
    
    pub fn external_service(service: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ExternalService { 
            service: service.into(), 
            message: message.into() 
        }
    }
} 