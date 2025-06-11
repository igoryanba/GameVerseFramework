use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::domain::models::{
    user::User,
    token::TokenResponse,
    error::AuthError,
};

/// Стандартный ответ API
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Структура ошибки API
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl<T> ApiResponse<T> 
where 
    T: Serialize,
{
    /// Создает успешный ответ
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: None,
        }
    }
    
    /// Создает успешный ответ с сообщением
    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            message: Some(message),
        }
    }
    
    /// Создает ответ с ошибкой (generic версия)
    pub fn error(code: String, message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                code,
                message,
                details: None,
            }),
            message: None,
        }
    }
    
    /// Создает ответ с ошибкой и деталями (generic версия)
    pub fn error_with_details(code: String, message: String, details: serde_json::Value) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ApiError {
                code,
                message,
                details: Some(details),
            }),
            message: None,
        }
    }
}

impl ApiResponse<()> {
    /// Создает простой успешный ответ без данных
    pub fn success_simple(message: String) -> Self {
        Self {
            success: true,
            data: None,
            error: None,
            message: Some(message),
        }
    }
}

/// Реализация IntoResponse для ApiResponse
impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = if self.success {
            StatusCode::OK
        } else {
            // Определяем HTTP статус на основе кода ошибки
            match self.error.as_ref().map(|e| e.code.as_str()) {
                Some("INVALID_CREDENTIALS") => StatusCode::UNAUTHORIZED,
                Some("USER_NOT_FOUND") => StatusCode::NOT_FOUND,
                Some("USER_ALREADY_EXISTS") => StatusCode::CONFLICT,
                Some("EMAIL_ALREADY_EXISTS") => StatusCode::CONFLICT,
                Some("INVALID_TOKEN") => StatusCode::UNAUTHORIZED,
                Some("USER_LOCKED") => StatusCode::LOCKED,
                Some("USER_INACTIVE") => StatusCode::FORBIDDEN,
                Some("TOTP_CODE_REQUIRED") => StatusCode::UNAUTHORIZED,
                Some("INVALID_TOTP_CODE") => StatusCode::UNAUTHORIZED,
                Some("VALIDATION_ERROR") => StatusCode::BAD_REQUEST,
                Some("DATABASE_ERROR") => StatusCode::INTERNAL_SERVER_ERROR,
                Some("INTERNAL_ERROR") => StatusCode::INTERNAL_SERVER_ERROR,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        };
        
        (status, Json(self)).into_response()
    }
}

/// Конвертация AuthError в ApiResponse (generic версия)
impl<T> From<AuthError> for ApiResponse<T>
where
    T: Serialize,
{
    fn from(error: AuthError) -> Self {
        let (code, message) = match error {
            AuthError::UserNotFound => ("USER_NOT_FOUND".to_string(), "Пользователь не найден".to_string()),
            AuthError::UsernameAlreadyExists | AuthError::UserAlreadyExists => ("USER_ALREADY_EXISTS".to_string(), "Пользователь уже существует".to_string()),
            AuthError::EmailAlreadyExists => ("EMAIL_ALREADY_EXISTS".to_string(), "Email уже используется".to_string()),
            AuthError::InvalidCredentials => ("INVALID_CREDENTIALS".to_string(), "Неверные учетные данные".to_string()),
            AuthError::InvalidToken => ("INVALID_TOKEN".to_string(), "Недействительный токен".to_string()),
            AuthError::TokenExpired => ("TOKEN_EXPIRED".to_string(), "Токен истек".to_string()),
            AuthError::TokenRevoked => ("TOKEN_REVOKED".to_string(), "Токен отозван".to_string()),
            AuthError::UserLocked => ("USER_LOCKED".to_string(), "Пользователь заблокирован".to_string()),
            AuthError::UserInactive => ("USER_INACTIVE".to_string(), "Пользователь неактивен".to_string()),
            AuthError::InvalidTotpCode => ("INVALID_TOTP_CODE".to_string(), "Неверный код двухфакторной аутентификации".to_string()),
            AuthError::TotpRequired | AuthError::TotpCodeRequired => ("TOTP_REQUIRED".to_string(), "Требуется двухфакторная аутентификация".to_string()),
            AuthError::InsufficientPermissions => ("INSUFFICIENT_PERMISSIONS".to_string(), "Недостаточно прав".to_string()),
            AuthError::DatabaseError(msg) => ("DATABASE_ERROR".to_string(), format!("Ошибка базы данных: {}", msg)),
            AuthError::HashingError(msg) => ("HASHING_ERROR".to_string(), format!("Ошибка хеширования: {}", msg)),
            AuthError::JwtError(msg) => ("JWT_ERROR".to_string(), format!("Ошибка JWT: {}", msg)),
            AuthError::InternalError(msg) => ("INTERNAL_ERROR".to_string(), format!("Внутренняя ошибка: {}", msg)),
            AuthError::RedisError(msg) => ("REDIS_ERROR".to_string(), format!("Ошибка Redis: {}", msg)),
            AuthError::ValidationError(msg) => ("VALIDATION_ERROR".to_string(), format!("Ошибка валидации: {}", msg)),
        };
        
        ApiResponse::error(code, message)
    }
}

/// Ответ для регистрации пользователя
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub user: UserResponse,
}

/// Ответ для входа в систему
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub tokens: TokenResponse,
}

/// Ответ с информацией о пользователе (без чувствительных данных)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub status: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub created_at: String,
    pub last_login: Option<String>,
    pub totp_enabled: bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            status: user.status.to_string(),
            roles: user.roles,
            permissions: user.permissions,
            created_at: user.created_at.to_rfc3339(),
            last_login: user.last_login.map(|dt| dt.to_rfc3339()),
            totp_enabled: user.totp_enabled,
        }
    }
}

/// Ответ для проверки здоровья сервиса
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
}

impl HealthResponse {
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
} 