use thiserror::Error;

/// Ошибки домена управления данными игроков
#[derive(Debug, Error)]
pub enum PlayerDataError {
    #[error("Игрок не найден")]
    PlayerNotFound,
    
    #[error("Игрок с таким именем пользователя уже существует")]
    UsernameAlreadyExists,
    
    #[error("Игрок с таким email уже существует")]
    EmailAlreadyExists,
    
    #[error("Игрок заблокирован")]
    PlayerBanned,
    
    #[error("Игрок неактивен")]
    PlayerInactive,
    
    #[error("Недостаточно золота")]
    InsufficientGold,
    
    #[error("Недостаточно кристаллов")]
    InsufficientGems,
    
    #[error("Сессия не найдена")]
    SessionNotFound,
    
    #[error("Игрок уже в игре")]
    PlayerAlreadyInGame,
    
    #[error("Игрок не в игре")]
    PlayerNotInGame,
    
    #[error("Достижение не найдено")]
    AchievementNotFound,
    
    #[error("Достижение уже получено")]
    AchievementAlreadyUnlocked,
    
    #[error("Статистика не найдена")]
    StatsNotFound,
    
    #[error("Недопустимые данные статистики")]
    InvalidStatsData,
    
    #[error("Ошибка доступа к базе данных: {0}")]
    DatabaseError(String),
    
    #[error("Ошибка Redis: {0}")]
    RedisError(String),
    
    #[error("Ошибка валидации: {0}")]
    ValidationError(String),
    
    #[error("Внутренняя ошибка сервера: {0}")]
    InternalError(String),
    
    #[error("Ошибка сериализации JSON: {0}")]
    JsonError(String),
    
    #[error("Недостаточно прав для выполнения операции")]
    InsufficientPermissions,
    
    #[error("Регион не поддерживается: {0}")]
    UnsupportedRegion(String),
    
    #[error("Превышен лимит операций")]
    RateLimitExceeded,
}

impl From<sqlx::Error> for PlayerDataError {
    fn from(err: sqlx::Error) -> Self {
        PlayerDataError::DatabaseError(err.to_string())
    }
}

impl From<redis::RedisError> for PlayerDataError {
    fn from(err: redis::RedisError) -> Self {
        PlayerDataError::RedisError(err.to_string())
    }
}

impl From<validator::ValidationErrors> for PlayerDataError {
    fn from(err: validator::ValidationErrors) -> Self {
        PlayerDataError::ValidationError(err.to_string())
    }
}

impl From<serde_json::Error> for PlayerDataError {
    fn from(err: serde_json::Error) -> Self {
        PlayerDataError::JsonError(err.to_string())
    }
}

impl From<std::io::Error> for PlayerDataError {
    fn from(err: std::io::Error) -> Self {
        PlayerDataError::InternalError(err.to_string())
    }
} 