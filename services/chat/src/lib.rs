// GameVerse Chat Microservice
// Обеспечивает функциональность чата с поддержкой голосовой связи и Telegram интеграции

use std::sync::Arc;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod interfaces;
pub mod config;

// Re-exports для удобства использования
pub use domain::{
    ChatEvent, ChatMessage, ChatChannel, VoiceSession, GamePosition, VoiceRange, VoiceSessionType,
    ChatError, ApiResponse, MessageStatus, ChannelMember, MessageType, ChannelType,
    SendMessageRequest, CreateChannelRequest, GetMessagesQuery, EditMessageRequest,
    StartVoiceSessionRequest, UpdateVoicePositionRequest, MemberRole
};
pub use application::services::{ChatService, ChannelService, VoiceService};
pub use infrastructure::{
    database::{DatabaseService, MessageRepository, ChannelRepository},
    redis::RedisService,
};
pub use interfaces::{
    websocket::{WebSocketServer, WebSocketEvent, ClientCommand},
    http::*,
};
pub use config::ChatConfig;

// Общее состояние приложения
#[derive(Clone)]
pub struct AppState {
    pub chat_service: Arc<ChatService>,
    pub channel_service: Arc<ChannelService>,
    pub voice_service: Arc<VoiceService>,
    pub redis_service: Arc<RedisService>,
    pub websocket_server: Arc<WebSocketServer>,
    pub config: Arc<ChatConfig>,
} 