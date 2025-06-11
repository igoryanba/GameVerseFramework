use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

/// Система друзей
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Friendship {
    pub id: Uuid,
    pub requester_id: Uuid,
    pub addressee_id: Uuid,
    pub status: FriendshipStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "friendship_status")]
pub enum FriendshipStatus {
    Pending,    // Запрос отправлен
    Accepted,   // Дружба подтверждена
    Blocked,    // Заблокирован
    Declined,   // Запрос отклонен
}

/// Кланы/Организации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clan {
    pub id: Uuid,
    pub name: String,
    pub tag: String,           // Короткий тег клана [TAG]
    pub description: String,
    pub logo_url: Option<String>,
    pub leader_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub member_count: u32,
    pub max_members: u32,
    pub is_recruiting: bool,
    pub clan_type: ClanType,
    pub region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "clan_type")]
pub enum ClanType {
    Casual,     // Казуальный клан
    Competitive, // Соревновательный
    Roleplay,   // Ролевой
    Criminal,   // Криминальная организация
    Business,   // Бизнес корпорация
}

/// Участие в клане
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClanMembership {
    pub id: Uuid,
    pub clan_id: Uuid,
    pub player_id: Uuid,
    pub role: ClanRole,
    pub joined_at: DateTime<Utc>,
    pub last_activity: Option<DateTime<Utc>>,
    pub contribution_points: u64,  // Очки вклада в клан
    pub status: MembershipStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "clan_role")]
pub enum ClanRole {
    Leader,     // Лидер клана
    Officer,    // Офицер
    Veteran,    // Ветеран
    Member,     // Обычный участник
    Recruit,    // Новобранец
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "membership_status")]
pub enum MembershipStatus {
    Active,     // Активный участник
    Inactive,   // Неактивный
    OnLeave,    // В отпуске
    Kicked,     // Исключен
}

/// Блок-лист игроков
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerBlock {
    pub id: Uuid,
    pub blocker_id: Uuid,   // Кто заблокировал
    pub blocked_id: Uuid,   // Кого заблокировали
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Запросы на дружбу
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct FriendshipRequest {
    #[validate(length(max = 500, message = "Сообщение слишком длинное"))]
    pub message: Option<String>,
}

/// Создание клана
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateClanRequest {
    #[validate(length(min = 3, max = 50, message = "Название клана от 3 до 50 символов"))]
    pub name: String,
    
    #[validate(length(min = 2, max = 5, message = "Тег клана от 2 до 5 символов"))]
    #[validate(regex(path = "CLAN_TAG_REGEX", message = "Тег содержит недопустимые символы"))]
    pub tag: String,
    
    #[validate(length(max = 1000, message = "Описание слишком длинное"))]
    pub description: String,
    
    #[validate(url(message = "Некорректный URL логотипа"))]
    pub logo_url: Option<String>,
    
    pub clan_type: ClanType,
    pub max_members: Option<u32>,
    pub region: Option<String>,
}

/// Приглашение в клан
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ClanInviteRequest {
    pub clan_id: Uuid,
    pub player_id: Uuid,
    pub role: Option<ClanRole>,
    
    #[validate(length(max = 500, message = "Сообщение слишком длинное"))]
    pub message: Option<String>,
}

/// Онлайн статус игрока  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPresence {
    pub player_id: Uuid,
    pub status: PresenceStatus,
    pub last_seen: DateTime<Utc>,
    pub current_server: Option<String>,
    pub current_activity: Option<String>,  // "В гонке", "В ограблении", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "presence_status")]
pub enum PresenceStatus {
    Online,     // В сети
    Away,       // Отошел
    Busy,       // Занят
    Invisible,  // Невидимый
    Offline,    // Не в сети
}

// Регулярные выражения для валидации
lazy_static::lazy_static! {
    static ref CLAN_TAG_REGEX: regex::Regex = regex::Regex::new(r"^[A-Z0-9]+$").unwrap();
}

impl Friendship {
    pub fn new(requester_id: Uuid, addressee_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            requester_id,
            addressee_id,
            status: FriendshipStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn accept(&mut self) {
        self.status = FriendshipStatus::Accepted;
        self.updated_at = Utc::now();
    }
    
    pub fn decline(&mut self) {
        self.status = FriendshipStatus::Declined;
        self.updated_at = Utc::now();
    }
    
    pub fn block(&mut self) {
        self.status = FriendshipStatus::Blocked;
        self.updated_at = Utc::now();
    }
} 