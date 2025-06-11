use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Тип токена
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    Access,
    Refresh,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Access => write!(f, "access"),
            TokenType::Refresh => write!(f, "refresh"),
        }
    }
}

/// Клеймы для JWT токена
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    /// Идентификатор токена
    pub jti: String,
    /// Время создания
    pub iat: i64,
    /// Время истечения
    pub exp: i64,
    /// Издатель
    pub iss: String,
    /// Аудитория
    pub aud: String,
    /// Идентификатор пользователя
    pub sub: String,
    /// Тип токена
    pub token_type: String,
    /// Разрешения пользователя
    pub permissions: Vec<String>,
}

/// Модель токена
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    /// Идентификатор токена
    pub id: Uuid,
    /// Идентификатор пользователя
    pub user_id: Uuid,
    /// Тип токена
    pub token_type: TokenType,
    /// Токен
    pub token: String,
    /// Время создания
    pub created_at: DateTime<Utc>,
    /// Время истечения
    pub expires_at: DateTime<Utc>,
    /// Отозван ли токен
    pub revoked: bool,
    /// Время отзыва
    pub revoked_at: Option<DateTime<Utc>>,
    /// IP-адрес, с которого был создан токен
    pub ip_address: Option<String>,
    /// User-Agent, с которого был создан токен
    pub user_agent: Option<String>,
}

impl Token {
    /// Создает новый токен
    pub fn new(
        user_id: Uuid,
        token_type: TokenType,
        token: String,
        expires_at: DateTime<Utc>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            token_type,
            token,
            created_at: now,
            expires_at,
            revoked: false,
            revoked_at: None,
            ip_address,
            user_agent,
        }
    }
    
    /// Проверяет, истек ли токен
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
    
    /// Отзывает токен
    pub fn revoke(&mut self) {
        self.revoked = true;
        self.revoked_at = Some(Utc::now());
    }
    
    /// Проверяет, активен ли токен (не истек и не отозван)
    pub fn is_active(&self) -> bool {
        !self.is_expired() && !self.revoked
    }
}

/// Модель для ответа с токенами
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    /// Токен доступа
    pub access_token: String,
    /// Refresh токен
    pub refresh_token: String,
    /// Тип токена
    pub token_type: String,
    /// Время истечения токена доступа в секундах
    pub expires_in: i64,
}

impl TokenResponse {
    /// Создает новый ответ с токенами
    pub fn new(access_token: String, refresh_token: String, expires_in: i64) -> Self {
        Self {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in,
        }
    }
} 