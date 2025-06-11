use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// ============================================================================
// Модель сообщения чата
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatMessage {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub message_type: MessageType,
    pub reply_to: Option<Uuid>,
    pub forwarded_from: Option<Uuid>,
    pub edited_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageMetadata {
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    pub character_name: Option<String>,
    pub is_anonymous: bool,
    pub location: Option<GameLocation>,
    pub telegram_user_id: Option<i64>,
    pub telegram_message_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "message_type", rename_all = "snake_case")]
pub enum MessageType {
    Text,
    Action,      // /me команды
    Ooc,         // Out of Character
    Radio,       // Радиосвязь
    Phone,       // Телефонный звонок
    Admin,       // Административные сообщения
    System,      // Системные уведомления
    Dice,        // Результаты /dice
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameLocation {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub world: Option<String>,
    pub interior: Option<String>,
}

// ============================================================================
// Модель канала чата
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatChannel {
    pub id: Uuid,
    pub name: String,
    pub channel_type: ChannelType,
    pub created_by: Uuid,
    pub is_private: bool,
    pub is_temporary: bool,
    pub max_participants: Option<i32>,
    pub settings: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub telegram_chat_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSettings {
    pub description: Option<String>,
    pub welcome_message: Option<String>,
    pub rate_limit: Option<u32>,
    pub max_message_length: Option<usize>,
    pub profanity_filter: bool,
    pub require_approval: bool,
    pub auto_delete_messages: Option<u32>, // дни
    pub color: Option<String>,
    pub emoji: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "channel_type", rename_all = "snake_case")]
pub enum ChannelType {
    Global,
    Local,
    Radio,
    Phone,
    Group,
    Direct,
    Ooc,
    Admin,
    Telegram,  // Специальный тип для интеграции с Telegram
}

// ============================================================================
// Модель участников канала
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChannelMember {
    pub channel_id: Uuid,
    pub user_id: Uuid,
    pub role: MemberRole,
    pub joined_at: DateTime<Utc>,
    pub last_read_at: Option<DateTime<Utc>>,
    pub is_muted: bool,
    pub muted_until: Option<DateTime<Utc>>,
    pub notification_settings: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "member_role", rename_all = "snake_case")]
pub enum MemberRole {
    Owner,
    Admin,
    Moderator,
    Member,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub enable_push: bool,
    pub enable_telegram: bool,
    pub enable_sound: bool,
    pub keywords: Vec<String>,
    pub mention_only: bool,
}

// ============================================================================
// Модель голосовых сессий
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct VoiceSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_type: VoiceSessionType,
    pub channel_id: Option<Uuid>,
    pub voice_range: VoiceRange,
    pub is_muted: bool,
    pub is_deafened: bool,
    pub position: Option<serde_json::Value>,
    pub quality: VoiceQuality,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamePosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rotation: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "voice_session_type", rename_all = "snake_case")]
pub enum VoiceSessionType {
    Proximity,
    Radio,
    Phone,
    Group,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "voice_range", rename_all = "snake_case")]
pub enum VoiceRange {
    Whisper,   // 3m
    Normal,    // 10m
    Shout,     // 25m
    Megaphone, // 50m
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "voice_quality", rename_all = "snake_case")]
pub enum VoiceQuality {
    Low,
    Medium,
    High,
    Crystal,
}

// ============================================================================
// Модели для Telegram интеграции
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TelegramUser {
    pub user_id: Uuid,
    pub telegram_id: i64,
    pub username: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub is_verified: bool,
    pub notifications_enabled: bool,
    pub linked_at: DateTime<Utc>,
    pub last_activity: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TelegramChannelLink {
    pub channel_id: Uuid,
    pub telegram_chat_id: i64,
    pub chat_type: TelegramChatType,
    pub is_bidirectional: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub settings: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "telegram_chat_type", rename_all = "snake_case")]
pub enum TelegramChatType {
    Private,
    Group,
    Supergroup,
    Channel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramLinkSettings {
    pub forward_from_game: bool,
    pub forward_to_game: bool,
    pub format_messages: bool,
    pub include_player_names: bool,
    pub filter_message_types: Vec<MessageType>,
    pub admin_only: bool,
}

// ============================================================================
// Модели для статистики и аналитики
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MessageStatus {
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub status: ReadStatus,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "read_status", rename_all = "snake_case")]
pub enum ReadStatus {
    Sent,
    Delivered,
    Read,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingStatus {
    pub channel_id: Uuid,
    pub user_id: Uuid,
    pub started_at: DateTime<Utc>,
}

// ============================================================================
// DTOs для API запросов
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SendMessageRequest {
    #[validate(length(min = 1, max = 2000))]
    pub content: String,
    pub message_type: Option<MessageType>,
    pub reply_to: Option<Uuid>,
    pub metadata: Option<ChatMessageMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateChannelRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub channel_type: ChannelType,
    pub is_private: Option<bool>,
    pub max_participants: Option<i32>,
    pub settings: Option<ChannelSettings>,
    pub telegram_chat_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EditMessageRequest {
    #[validate(length(min = 1, max = 2000))]
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardMessageRequest {
    pub target_channel_id: Uuid,
    pub with_attribution: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartVoiceSessionRequest {
    pub session_type: VoiceSessionType,
    pub channel_id: Option<Uuid>,
    pub voice_range: Option<VoiceRange>,
    pub position: Option<GamePosition>,
    pub quality: Option<VoiceQuality>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateVoicePositionRequest {
    pub position: GamePosition,
    pub voice_range: Option<VoiceRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LinkTelegramRequest {
    pub telegram_id: i64,
    #[validate(length(min = 1))]
    pub verification_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessagesQuery {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub before: Option<DateTime<Utc>>,
    pub after: Option<DateTime<Utc>>,
    pub message_type: Option<MessageType>,
    pub search: Option<String>,
}

// ============================================================================
// События для WebSocket и real-time уведомлений
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ChatEvent {
    NewMessage(ChatMessage),
    MessageEdited(ChatMessage),
    MessageDeleted { message_id: Uuid, channel_id: Uuid },
    UserTyping { user_id: Uuid, channel_id: Uuid },
    UserStoppedTyping { user_id: Uuid, channel_id: Uuid },
    UserJoinedChannel { user_id: Uuid, channel_id: Uuid },
    UserLeftChannel { user_id: Uuid, channel_id: Uuid },
    ChannelCreated(ChatChannel),
    ChannelUpdated(ChatChannel),
    ChannelDeleted { channel_id: Uuid },
    VoiceSessionStarted(VoiceSession),
    VoiceSessionEnded { session_id: Uuid },
    VoicePositionUpdate { session_id: Uuid, position: GamePosition },
    MessageStatusUpdate { message_id: Uuid, user_id: Uuid, status: ReadStatus },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub event: ChatEvent,
    pub timestamp: DateTime<Utc>,
    pub recipients: Option<Vec<Uuid>>, // Если None, то broadcast
}

// ============================================================================
// Реализация методов для моделей
// ============================================================================

impl ChatMessage {
    pub fn new(
        channel_id: Uuid,
        sender_id: Uuid,
        content: String,
        message_type: MessageType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            channel_id,
            sender_id,
            content,
            message_type,
            reply_to: None,
            forwarded_from: None,
            edited_at: None,
            deleted_at: None,
            created_at: now,
            updated_at: now,
            metadata: Some(serde_json::json!({})),
        }
    }
    
    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
    
    pub fn is_edited(&self) -> bool {
        self.edited_at.is_some()
    }
    
    pub fn set_metadata(&mut self, metadata: ChatMessageMetadata) {
        self.metadata = Some(serde_json::to_value(metadata).unwrap_or_default());
    }
    
    pub fn get_metadata(&self) -> Option<ChatMessageMetadata> {
        self.metadata.as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

impl ChatChannel {
    pub fn new(
        name: String,
        channel_type: ChannelType,
        created_by: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            channel_type,
            created_by,
            is_private: false,
            is_temporary: false,
            max_participants: None,
            settings: Some(serde_json::json!({})),
            created_at: Utc::now(),
            expires_at: None,
            telegram_chat_id: None,
        }
    }
    
    pub fn set_settings(&mut self, settings: ChannelSettings) {
        self.settings = Some(serde_json::to_value(settings).unwrap_or_default());
    }
    
    pub fn get_settings(&self) -> Option<ChannelSettings> {
        self.settings.as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
    
    pub fn is_expired(&self) -> bool {
        self.expires_at.map_or(false, |exp| exp < Utc::now())
    }
}

impl VoiceSession {
    pub fn new(
        user_id: Uuid,
        session_type: VoiceSessionType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            session_type,
            channel_id: None,
            voice_range: VoiceRange::Normal,
            is_muted: false,
            is_deafened: false,
            position: Some(serde_json::json!({})),
            quality: VoiceQuality::Medium,
            started_at: Utc::now(),
            ended_at: None,
        }
    }
    
    pub fn set_position(&mut self, position: GamePosition) {
        self.position = Some(serde_json::to_value(position).unwrap_or_default());
    }
    
    pub fn get_position(&self) -> Option<GamePosition> {
        self.position.as_ref()
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
    
    pub fn is_active(&self) -> bool {
        self.ended_at.is_none()
    }
}

impl VoiceRange {
    pub fn distance(&self) -> f64 {
        match self {
            VoiceRange::Whisper => 3.0,
            VoiceRange::Normal => 10.0,
            VoiceRange::Shout => 25.0,
            VoiceRange::Megaphone => 50.0,
        }
    }
} 