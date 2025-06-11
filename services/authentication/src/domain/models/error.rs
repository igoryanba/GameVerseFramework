use thiserror::Error;

/// Ошибки домена аутентификации
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Неверные учетные данные")]
    InvalidCredentials,
    
    #[error("Пользователь заблокирован")]
    UserLocked,
    
    #[error("Пользователь не активен")]
    UserInactive,
    
    #[error("Пользователь не найден")]
    UserNotFound,
    
    #[error("Пользователь с таким именем уже существует")]
    UsernameAlreadyExists,
    
    /// Альтернативный вариант для совместимости
    #[error("Пользователь уже существует")]
    UserAlreadyExists,
    
    #[error("Пользователь с таким email уже существует")]
    EmailAlreadyExists,
    
    #[error("Токен истек")]
    TokenExpired,
    
    #[error("Токен отозван")]
    TokenRevoked,
    
    #[error("Недопустимый токен")]
    InvalidToken,
    
    #[error("Недостаточно прав")]
    InsufficientPermissions,
    
    #[error("Недопустимый код двухфакторной аутентификации")]
    InvalidTotpCode,
    
    #[error("Требуется двухфакторная аутентификация")]
    TotpRequired,
    
    /// Альтернативный вариант для совместимости  
    #[error("Требуется код двухфакторной аутентификации")]
    TotpCodeRequired,
    
    #[error("Ошибка доступа к базе данных: {0}")]
    DatabaseError(String),
    
    #[error("Ошибка хеширования пароля: {0}")]
    HashingError(String),
    
    #[error("Ошибка JWT: {0}")]
    JwtError(String),
    
    #[error("Внутренняя ошибка сервера: {0}")]
    InternalError(String),
    
    #[error("Ошибка Redis: {0}")]
    RedisError(String),
    
    #[error("Ошибка валидации: {0}")]
    ValidationError(String),
}

impl From<sqlx::Error> for AuthError {
    fn from(err: sqlx::Error) -> Self {
        AuthError::DatabaseError(err.to_string())
    }
}

impl From<redis::RedisError> for AuthError {
    fn from(err: redis::RedisError) -> Self {
        AuthError::RedisError(err.to_string())
    }
}

impl From<argon2::Error> for AuthError {
    fn from(err: argon2::Error) -> Self {
        AuthError::HashingError(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AuthError::JwtError(err.to_string())
    }
}

impl From<validator::ValidationErrors> for AuthError {
    fn from(err: validator::ValidationErrors) -> Self {
        AuthError::ValidationError(err.to_string())
    }
}

impl From<std::io::Error> for AuthError {
    fn from(err: std::io::Error) -> Self {
        AuthError::InternalError(err.to_string())
    }
} 