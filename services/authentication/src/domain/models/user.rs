use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Модель статуса пользователя
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    Active,
    Inactive,
    Banned,
    Pending,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Active => write!(f, "active"),
            UserStatus::Inactive => write!(f, "inactive"),
            UserStatus::Banned => write!(f, "banned"),
            UserStatus::Pending => write!(f, "pending"),
        }
    }
}

impl From<String> for UserStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "active" => UserStatus::Active,
            "inactive" => UserStatus::Inactive,
            "banned" => UserStatus::Banned,
            "pending" => UserStatus::Pending,
            _ => UserStatus::Pending,
        }
    }
}

/// Модель пользователя
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    /// Уникальный идентификатор пользователя
    pub id: Uuid,
    /// Имя пользователя
    pub username: String,
    /// Email пользователя
    pub email: String,
    /// Хеш пароля
    #[serde(skip_serializing)]
    pub password_hash: String,
    /// Статус пользователя
    pub status: UserStatus,
    /// Дата создания
    pub created_at: DateTime<Utc>,
    /// Дата последнего обновления
    pub updated_at: DateTime<Utc>,
    /// Дата последнего входа
    pub last_login: Option<DateTime<Utc>>,
    /// Роли пользователя
    pub roles: Vec<String>,
    /// Разрешения пользователя
    pub permissions: Vec<String>,
    /// Максимальное количество неудачных попыток входа
    pub failed_login_attempts: u32,
    /// Время блокировки после превышения количества попыток
    pub lockout_until: Option<DateTime<Utc>>,
    /// Ключ для двухфакторной аутентификации
    #[serde(skip_serializing)]
    pub totp_secret: Option<String>,
    /// Двухфакторная аутентификация включена
    pub totp_enabled: bool,
}

impl User {
    /// Создает нового пользователя
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            status: UserStatus::Pending,
            created_at: now,
            updated_at: now,
            last_login: None,
            roles: vec!["user".to_string()],
            permissions: vec![],
            failed_login_attempts: 0,
            lockout_until: None,
            totp_secret: None,
            totp_enabled: false,
        }
    }
    
    /// Проверяет, заблокирован ли пользователь
    pub fn is_locked(&self) -> bool {
        match self.lockout_until {
            Some(lockout_time) => lockout_time > Utc::now(),
            None => false,
        }
    }
    
    /// Проверяет, активен ли пользователь
    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active
    }
    
    /// Проверяет, имеет ли пользователь указанную роль
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }
    
    /// Проверяет, имеет ли пользователь указанное разрешение
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|p| p == permission)
    }
    
    /// Обновляет время последнего входа
    pub fn update_last_login(&mut self) {
        self.last_login = Some(Utc::now());
        self.failed_login_attempts = 0;
        self.lockout_until = None;
    }
    
    /// Увеличивает счетчик неудачных попыток входа
    pub fn increment_failed_login(&mut self, max_attempts: u32, lockout_duration: std::time::Duration) {
        self.failed_login_attempts += 1;
        if self.failed_login_attempts >= max_attempts {
            self.lockout_until = Some(Utc::now() + chrono::Duration::from_std(lockout_duration).unwrap());
        }
    }
}

/// Запрос на создание пользователя
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateUserRequest {
    /// Имя пользователя
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    
    /// Email пользователя
    #[validate(email)]
    pub email: String,
    
    /// Пароль пользователя
    #[validate(length(min = 8, max = 100))]
    pub password: String,
}

/// Запрос на обновление пользователя
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateUserRequest {
    /// Имя пользователя
    #[validate(length(min = 3, max = 32))]
    pub username: Option<String>,
    
    /// Email пользователя
    #[validate(email)]
    pub email: Option<String>,
    
    /// Статус пользователя
    pub status: Option<UserStatus>,
    
    /// Роли пользователя
    pub roles: Option<Vec<String>>,
    
    /// Разрешения пользователя
    pub permissions: Option<Vec<String>>,
}

/// Запрос на изменение пароля
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    /// Текущий пароль
    pub current_password: String,
    
    /// Новый пароль
    #[validate(length(min = 8, max = 100))]
    pub new_password: String,
}

/// Запрос на аутентификацию
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginRequest {
    /// Имя пользователя или email
    pub identifier: String,
    
    /// Пароль
    pub password: String,
    
    /// Код для двухфакторной аутентификации
    pub totp_code: Option<String>,
} 