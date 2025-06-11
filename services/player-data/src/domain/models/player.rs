use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

/// Основная модель игрока
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Player {
    /// Уникальный идентификатор игрока
    pub id: Uuid,
    
    /// Имя пользователя (уникальное)
    pub username: String,
    
    /// Отображаемое имя игрока
    pub display_name: String,
    
    /// URL аватара игрока
    pub avatar_url: Option<String>,
    
    /// Электронная почта
    pub email: String,
    
    /// Уровень игрока
    pub level: u32,
    
    /// Текущий опыт
    pub experience: u64,
    
    /// Опыт, необходимый для следующего уровня
    pub experience_to_next_level: u64,
    
    /// Игровая валюта (золото)
    pub gold: u64,
    
    /// Премиум валюта (кристаллы)
    pub gems: u64,
    
    /// Настройки игрока в JSON формате
    pub settings: serde_json::Value,
    
    /// Статус игрока
    pub status: PlayerStatus,
    
    /// Дата создания аккаунта
    pub created_at: DateTime<Utc>,
    
    /// Дата последнего обновления
    pub updated_at: DateTime<Utc>,
    
    /// Дата последнего входа в игру
    pub last_login: Option<DateTime<Utc>>,
    
    /// Общее время в игре (в секундах)
    pub total_playtime: u64,
    
    /// Текущая игровая сессия (если в игре)
    pub current_session_id: Option<Uuid>,
    
    /// Текущий регион/сервер
    pub current_region: Option<String>,
}

/// Статус игрока
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "player_status")]
pub enum PlayerStatus {
    /// Аккаунт активен
    Active,
    /// Аккаунт временно заблокирован
    Suspended,
    /// Аккаунт заблокирован навсегда
    Banned,
    /// Аккаунт неактивен (не подтвержден)
    Inactive,
}

impl Player {
    /// Создает нового игрока
    pub fn new(username: String, display_name: String, email: String) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            username,
            display_name,
            avatar_url: None,
            email,
            level: 1,
            experience: 0,
            experience_to_next_level: 1000, // Начальный опыт для уровня 2
            gold: 1000, // Стартовое золото
            gems: 100,  // Стартовые кристаллы
            settings: serde_json::json!({}), // Пустые настройки по умолчанию
            status: PlayerStatus::Active,
            created_at: now,
            updated_at: now,
            last_login: None,
            total_playtime: 0,
            current_session_id: None,
            current_region: None,
        }
    }
    
    /// Проверяет, активен ли игрок
    pub fn is_active(&self) -> bool {
        self.status == PlayerStatus::Active
    }
    
    /// Проверяет, заблокирован ли игрок
    pub fn is_banned(&self) -> bool {
        matches!(self.status, PlayerStatus::Banned | PlayerStatus::Suspended)
    }
    
    /// Добавляет опыт игроку и повышает уровень при необходимости
    pub fn add_experience(&mut self, exp: u64) {
        self.experience += exp;
        self.check_level_up();
        self.updated_at = Utc::now();
    }
    
    /// Проверяет и повышает уровень игрока
    fn check_level_up(&mut self) {
        while self.experience >= self.experience_to_next_level {
            self.experience -= self.experience_to_next_level;
            self.level += 1;
            
            // Увеличиваем требования к опыту для следующего уровня
            self.experience_to_next_level = self.calculate_experience_for_level(self.level + 1);
        }
    }
    
    /// Вычисляет необходимый опыт для определенного уровня
    fn calculate_experience_for_level(&self, level: u32) -> u64 {
        // Формула: базовый_опыт * уровень^1.5
        let base_exp = 1000_f64;
        (base_exp * (level as f64).powf(1.5)) as u64
    }
    
    /// Добавляет золото игроку
    pub fn add_gold(&mut self, amount: u64) {
        self.gold += amount;
        self.updated_at = Utc::now();
    }
    
    /// Тратит золото игрока
    pub fn spend_gold(&mut self, amount: u64) -> Result<(), String> {
        if self.gold < amount {
            return Err("Недостаточно золота".to_string());
        }
        
        self.gold -= amount;
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Добавляет кристаллы игроку
    pub fn add_gems(&mut self, amount: u64) {
        self.gems += amount;
        self.updated_at = Utc::now();
    }
    
    /// Тратит кристаллы игрока
    pub fn spend_gems(&mut self, amount: u64) -> Result<(), String> {
        if self.gems < amount {
            return Err("Недостаточно кристаллов".to_string());
        }
        
        self.gems -= amount;
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Обновляет время последнего входа
    pub fn update_last_login(&mut self) {
        self.last_login = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    /// Начинает новую игровую сессию
    pub fn start_session(&mut self, session_id: Uuid, region: Option<String>) {
        self.current_session_id = Some(session_id);
        self.current_region = region;
        self.update_last_login();
    }
    
    /// Завершает текущую игровую сессию
    pub fn end_session(&mut self, playtime_seconds: u64) {
        self.current_session_id = None;
        self.current_region = None;
        self.total_playtime += playtime_seconds;
        self.updated_at = Utc::now();
    }
    
    /// Обновляет настройки игрока
    pub fn update_settings(&mut self, settings: serde_json::Value) {
        self.settings = settings;
        self.updated_at = Utc::now();
    }
    
    /// Изменяет статус игрока
    pub fn set_status(&mut self, status: PlayerStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
    
    /// Обновляет отображаемое имя
    pub fn update_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
        self.updated_at = Utc::now();
    }
    
    /// Обновляет аватар
    pub fn update_avatar(&mut self, avatar_url: Option<String>) {
        self.avatar_url = avatar_url;
        self.updated_at = Utc::now();
    }
}

/// Запрос на создание игрока
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePlayerRequest {
    #[validate(length(min = 3, max = 50, message = "Имя пользователя должно быть от 3 до 50 символов"))]
    #[validate(regex(path = "USERNAME_REGEX", message = "Имя пользователя содержит недопустимые символы"))]
    pub username: String,
    
    #[validate(length(min = 1, max = 100, message = "Отображаемое имя должно быть от 1 до 100 символов"))]
    pub display_name: String,
    
    #[validate(email(message = "Некорректный формат email"))]
    pub email: String,
    
    #[validate(url(message = "Некорректный URL аватара"))]
    pub avatar_url: Option<String>,
}

/// Запрос на обновление профиля игрока
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePlayerRequest {
    #[validate(length(min = 1, max = 100, message = "Отображаемое имя должно быть от 1 до 100 символов"))]
    pub display_name: Option<String>,
    
    #[validate(url(message = "Некорректный URL аватара"))]
    pub avatar_url: Option<String>,
    
    pub settings: Option<serde_json::Value>,
}

/// Запрос на обновление валюты
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateCurrencyRequest {
    #[validate(range(min = 0, message = "Количество золота не может быть отрицательным"))]
    pub gold_change: Option<i64>,
    
    #[validate(range(min = 0, message = "Количество кристаллов не может быть отрицательным"))]
    pub gems_change: Option<i64>,
}

/// Запрос на добавление опыта
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AddExperienceRequest {
    #[validate(range(min = 1, max = 1000000, message = "Опыт должен быть от 1 до 1,000,000"))]
    pub experience: u64,
    
    pub reason: Option<String>,
}

// Регулярное выражение для валидации имени пользователя
lazy_static::lazy_static! {
    static ref USERNAME_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}

impl std::fmt::Display for PlayerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerStatus::Active => write!(f, "active"),
            PlayerStatus::Suspended => write!(f, "suspended"),
            PlayerStatus::Banned => write!(f, "banned"),
            PlayerStatus::Inactive => write!(f, "inactive"),
        }
    }
} 