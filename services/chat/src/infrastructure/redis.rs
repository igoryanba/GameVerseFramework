// TODO: Реализация Redis клиента для real-time сообщений и кеширования

use crate::domain::{ChatError, ChatEvent, ChatMessage, VoiceSession};
use redis::{aio::ConnectionManager, AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Clone)]
pub struct RedisService {
    connection: ConnectionManager,
    pub_sub_connection: ConnectionManager,
}

pub struct RedisConfig {
    pub url: String,
    pub max_connections: u32,
    pub pool_timeout: u64,
    pub command_timeout: u64,
}

impl RedisService {
    pub async fn new(config: &RedisConfig) -> Result<Self, ChatError> {
        let client = Client::open(config.url.as_str())
            .map_err(|e| ChatError::Redis(e))?;

        let connection = ConnectionManager::new(client.clone())
            .await
            .map_err(ChatError::Redis)?;

        let pub_sub_connection = ConnectionManager::new(client)
            .await
            .map_err(ChatError::Redis)?;

        Ok(Self {
            connection,
            pub_sub_connection,
        })
    }

    // ============================================================================
    // Кеширование сообщений
    // ============================================================================

    pub async fn cache_message(&self, message: &ChatMessage) -> Result<(), ChatError> {
        let key = format!("message:{}", message.id);
        let value = serde_json::to_string(message)
            .map_err(ChatError::Json)?;

        let mut conn = self.connection.clone();
        let _: () = conn.set_ex(&key, value, 3600).await
            .map_err(ChatError::Redis)?;

        Ok(())
    }

    pub async fn get_cached_message(&self, message_id: Uuid) -> Result<Option<ChatMessage>, ChatError> {
        let key = format!("message:{}", message_id);
        let mut conn = self.connection.clone();
        
        let cached: Option<String> = conn.get(&key).await
            .map_err(ChatError::Redis)?;

        if let Some(data) = cached.as_ref() {
            let message = serde_json::from_str(data)
                .map_err(ChatError::Json)?;
            Ok(Some(message))
        } else {
            Ok(None)
        }
    }

    // ============================================================================
    // Управление активными пользователями
    // ============================================================================

    pub async fn set_user_online(&self, user_id: Uuid, channel_id: Uuid) -> Result<(), ChatError> {
        let key = format!("online_users:{}", channel_id);
        let mut conn = self.connection.clone();

        let _: () = conn.sadd(&key, user_id.to_string()).await
            .map_err(ChatError::Redis)?;
        
        let _: () = conn.expire(&key, 300).await // TTL 5 минут
            .map_err(ChatError::Redis)?;

        Ok(())
    }

    pub async fn set_user_offline(&self, user_id: Uuid, channel_id: Uuid) -> Result<(), ChatError> {
        let key = format!("online_users:{}", channel_id);
        let mut conn = self.connection.clone();

        let _: () = conn.srem(&key, user_id.to_string()).await
            .map_err(ChatError::Redis)?;

        Ok(())
    }

    pub async fn get_online_users(&self, channel_id: Uuid) -> Result<Vec<Uuid>, ChatError> {
        let key = format!("online_users:{}", channel_id);
        let mut conn = self.connection.clone();

        let user_ids: Vec<String> = conn.smembers(&key).await
            .map_err(ChatError::Redis)?;

        let users = user_ids.into_iter()
            .filter_map(|id| id.parse().ok())
            .collect();

        Ok(users)
    }

    // ============================================================================
    // Typing индикаторы
    // ============================================================================

    pub async fn set_typing(&self, user_id: Uuid, channel_id: Uuid) -> Result<(), ChatError> {
        let key = format!("typing:{}:{}", channel_id, user_id);
        let mut conn = self.connection.clone();

        let _: () = conn.set_ex(&key, "typing", 5).await // TTL 5 секунд
            .map_err(ChatError::Redis)?;

        Ok(())
    }

    pub async fn get_typing_users(&self, channel_id: Uuid) -> Result<Vec<Uuid>, ChatError> {
        let pattern = format!("typing:{}:*", channel_id);
        let mut conn = self.connection.clone();

        let keys: Vec<String> = conn.keys(&pattern).await
            .map_err(ChatError::Redis)?;

        let users = keys.into_iter()
            .filter_map(|key| {
                let parts: Vec<&str> = key.split(':').collect();
                if parts.len() >= 3 {
                    parts[2].parse().ok()
                } else {
                    None
                }
            })
            .collect();

        Ok(users)
    }

    // ============================================================================
    // Rate limiting
    // ============================================================================

    pub async fn check_rate_limit(&self, user_id: Uuid, limit: u32) -> Result<bool, ChatError> {
        let key = format!("rate_limit:{}", user_id);
        let mut conn = self.connection.clone();

        let current: Option<u32> = conn.get(&key).await
            .map_err(ChatError::Redis)?;

        match current {
            Some(count) if count >= limit => Ok(false),
            Some(_count) => {
                let _: () = conn.incr(&key, 1).await
                    .map_err(ChatError::Redis)?;
                Ok(true)
            }
            None => {
                let _: () = conn.set_ex(&key, 1, 60).await // 1 минута TTL
                    .map_err(ChatError::Redis)?;
                Ok(true)
            }
        }
    }

    // ============================================================================
    // Голосовые сессии
    // ============================================================================

    pub async fn store_voice_session(&self, session: &VoiceSession) -> Result<(), ChatError> {
        let key = format!("voice_session:{}", session.id);
        let value = serde_json::to_string(session)
            .map_err(ChatError::Json)?;

        let mut conn = self.connection.clone();
        let _: () = conn.set_ex(&key, value, 3600).await
            .map_err(ChatError::Redis)?;

        // Добавляем в список активных сессий пользователя
        let user_sessions_key = format!("user_voice_sessions:{}", session.user_id);
        let _: () = conn.sadd(&user_sessions_key, session.id.to_string()).await
            .map_err(ChatError::Redis)?;
        let _: () = conn.expire(&user_sessions_key, 3600).await
            .map_err(ChatError::Redis)?;

        Ok(())
    }

    pub async fn get_voice_session(&self, session_id: Uuid) -> Result<Option<VoiceSession>, ChatError> {
        let key = format!("voice_session:{}", session_id);
        let mut conn = self.connection.clone();

        let cached: Option<String> = conn.get(&key).await
            .map_err(ChatError::Redis)?;

        if let Some(data) = cached.as_ref() {
            let session = serde_json::from_str(data)
                .map_err(ChatError::Json)?;
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    pub async fn remove_voice_session(&self, session_id: Uuid, user_id: Uuid) -> Result<(), ChatError> {
        let key = format!("voice_session:{}", session_id);
        let user_sessions_key = format!("user_voice_sessions:{}", user_id);
        
        let mut conn = self.connection.clone();
        let _: () = conn.del(&key).await
            .map_err(ChatError::Redis)?;
        let _: () = conn.srem(&user_sessions_key, session_id.to_string()).await
            .map_err(ChatError::Redis)?;

        Ok(())
    }

    // ============================================================================
    // Real-time события (Pub/Sub)
    // ============================================================================

    pub async fn publish_event(&self, event: &ChatEvent) -> Result<(), ChatError> {
        let channel = match event {
            ChatEvent::NewMessage(msg) => format!("channel:{}", msg.channel_id),
            ChatEvent::MessageEdited(msg) => format!("channel:{}", msg.channel_id),
            ChatEvent::MessageDeleted { channel_id, .. } => format!("channel:{}", channel_id),
            ChatEvent::UserTyping { channel_id, .. } => format!("channel:{}", channel_id),
            ChatEvent::UserStoppedTyping { channel_id, .. } => format!("channel:{}", channel_id),
            ChatEvent::UserJoinedChannel { channel_id, .. } => format!("channel:{}", channel_id),
            ChatEvent::UserLeftChannel { channel_id, .. } => format!("channel:{}", channel_id),
            ChatEvent::VoiceSessionStarted(_session) => format!("voice:proximity"),
            ChatEvent::VoiceSessionEnded { .. } => format!("voice:proximity"),
            ChatEvent::VoicePositionUpdate { .. } => format!("voice:proximity"),
            _ => "general".to_string(),
        };

        let message = serde_json::to_string(event)
            .map_err(ChatError::Json)?;

        let mut conn = self.pub_sub_connection.clone();
        let _: () = conn.publish(&channel, message).await
            .map_err(ChatError::Redis)?;

        Ok(())
    }

    pub async fn subscribe_to_channel(&self, _channel_id: Uuid) -> Result<mpsc::Receiver<ChatEvent>, ChatError> {
        // Пока возвращаем пустой канал, pubsub требует более сложной реализации
        let (_tx, rx) = mpsc::channel(100);
        Ok(rx)
    }

    // ============================================================================
    // Кеширование каналов
    // ============================================================================

    pub async fn cache_channel_members(&self, channel_id: Uuid, members: &[Uuid]) -> Result<(), ChatError> {
        let key = format!("channel_members:{}", channel_id);
        let mut conn = self.connection.clone();

        // Очищаем старый список
        let _: () = conn.del(&key).await
            .map_err(ChatError::Redis)?;

        // Добавляем новых участников
        if !members.is_empty() {
            let member_strings: Vec<String> = members.iter()
                .map(|id| id.to_string())
                .collect();
            
            let _: () = conn.sadd(&key, member_strings).await
                .map_err(ChatError::Redis)?;
            
            let _: () = conn.expire(&key, 600).await // TTL 10 минут
                .map_err(ChatError::Redis)?;
        }

        Ok(())
    }

    pub async fn get_cached_channel_members(&self, channel_id: Uuid) -> Result<Option<Vec<Uuid>>, ChatError> {
        let key = format!("channel_members:{}", channel_id);
        let mut conn = self.connection.clone();

        let exists: bool = conn.exists(&key).await
            .map_err(ChatError::Redis)?;

        if !exists {
            return Ok(None);
        }

        let member_strings: Vec<String> = conn.smembers(&key).await
            .map_err(ChatError::Redis)?;

        let members = member_strings.into_iter()
            .filter_map(|id| id.parse().ok())
            .collect();

        Ok(Some(members))
    }

    // ============================================================================
    // Статистика
    // ============================================================================

    pub async fn increment_message_count(&self, channel_id: Uuid) -> Result<(), ChatError> {
        let key = format!("stats:messages:{}", channel_id);
        let mut conn = self.connection.clone();

        let _: () = conn.incr(&key, 1).await
            .map_err(ChatError::Redis)?;

        Ok(())
    }

    pub async fn get_message_count(&self, channel_id: Uuid) -> Result<u64, ChatError> {
        let key = format!("stats:messages:{}", channel_id);
        let mut conn = self.connection.clone();

        let count: Option<u64> = conn.get(&key).await
            .map_err(ChatError::Redis)?;

        Ok(count.unwrap_or(0))
    }
}

// ============================================================================
// Настройка Redis
// ============================================================================

pub async fn setup_redis(config: &RedisConfig) -> Result<RedisService, ChatError> {
    let service = RedisService::new(config).await?;
    
    // Проверяем соединение простой командой GET
    let mut conn = service.connection.clone();
    let _: Option<String> = conn.get("test_connection").await
        .map_err(ChatError::Redis)?;
    
    info!("Redis service initialized successfully");
    Ok(service)
} 