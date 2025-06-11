use crate::{
    ChatError, ChatMessage, ChatChannel, VoiceSession, ChatEvent, MessageType, ChannelType,
    SendMessageRequest, CreateChannelRequest, GetMessagesQuery, EditMessageRequest,
    StartVoiceSessionRequest, UpdateVoicePositionRequest, MemberRole
};
use crate::infrastructure::{MessageRepository, ChannelRepository, RedisService, TelegramService};
use crate::config::ChatConfig;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, warn, error};
use uuid::Uuid;
use validator::Validate;

// ============================================================================
// Основной сервис чата
// ============================================================================

#[derive(Clone)]
pub struct ChatService {
    message_repo: Arc<MessageRepository>,
    channel_repo: Arc<ChannelRepository>,
    redis: Arc<RedisService>,
    config: Arc<ChatConfig>,
    event_sender: mpsc::Sender<ChatEvent>,
}

impl ChatService {
    pub fn new(
        message_repo: Arc<MessageRepository>,
        channel_repo: Arc<ChannelRepository>,
        redis: Arc<RedisService>,
        config: Arc<ChatConfig>,
        event_sender: mpsc::Sender<ChatEvent>,
    ) -> Self {
        Self {
            message_repo,
            channel_repo,
            redis,
            config,
            event_sender,
        }
    }

    pub async fn send_message(
        &self,
        channel_id: Uuid,
        sender_id: Uuid,
        request: SendMessageRequest,
    ) -> Result<ChatMessage, ChatError> {
        // Валидация запроса
        request.validate().map_err(ChatError::Validation)?;

        // Проверяем rate limiting
        let rate_limit = self.config.security.rate_limit_messages_per_minute;
        if !self.redis.check_rate_limit(sender_id, rate_limit).await? {
            return Err(ChatError::RateLimit {
                message: "Превышен лимит сообщений в минуту".to_string(),
            });
        }

        // Проверяем длину сообщения
        if request.content.len() > self.config.security.max_message_length {
            return Err(ChatError::MessageTooLong {
                length: request.content.len(),
                max_length: self.config.security.max_message_length,
            });
        }

        // Проверяем доступ к каналу
        self.check_channel_access(channel_id, sender_id).await?;

        // Создаем сообщение
        let mut message = ChatMessage::new(
            channel_id,
            sender_id,
            request.content,
            request.message_type.unwrap_or(MessageType::Text),
        );

        // Устанавливаем reply_to если указано
        if let Some(reply_to) = request.reply_to {
            // Проверяем что сообщение существует в том же канале
            let original = self.message_repo.get_message(reply_to).await?;
            if original.channel_id != channel_id {
                return Err(ChatError::MessageNotFound { id: reply_to.to_string() });
            }
            message.reply_to = Some(reply_to);
        }

        // Устанавливаем метаданные
        if let Some(metadata) = request.metadata {
            message.set_metadata(metadata);
        }

        // Сохраняем в базу данных
        let saved_message = self.message_repo.create_message(&message).await?;

        // Кешируем сообщение
        self.redis.cache_message(&saved_message).await?;

        // Увеличиваем счетчик сообщений
        self.redis.increment_message_count(channel_id).await?;

        // Отправляем событие
        let event = ChatEvent::NewMessage(saved_message.clone());
        if let Err(e) = self.event_sender.send(event).await {
            warn!("Failed to send chat event: {}", e);
        }

        // Публикуем в Redis для real-time уведомлений
        self.redis.publish_event(&ChatEvent::NewMessage(saved_message.clone())).await?;

        info!(
            message_id = %saved_message.id,
            channel_id = %channel_id,
            sender_id = %sender_id,
            "Message sent successfully"
        );

        Ok(saved_message)
    }

    pub async fn get_messages(
        &self,
        channel_id: Uuid,
        user_id: Uuid,
        query: GetMessagesQuery,
    ) -> Result<Vec<ChatMessage>, ChatError> {
        // Проверяем доступ к каналу
        self.check_channel_access(channel_id, user_id).await?;

        // Получаем сообщения из базы данных
        let messages = self.message_repo.get_messages(channel_id, &query).await?;

        Ok(messages)
    }

    pub async fn edit_message(
        &self,
        message_id: Uuid,
        user_id: Uuid,
        request: EditMessageRequest,
    ) -> Result<ChatMessage, ChatError> {
        // Валидация запроса
        request.validate().map_err(ChatError::Validation)?;

        // Получаем исходное сообщение
        let original_message = self.message_repo.get_message(message_id).await?;

        // Проверяем права на редактирование
        if original_message.sender_id != user_id {
            return Err(ChatError::Authorization {
                message: "Нет прав на редактирование этого сообщения".to_string(),
            });
        }

        // Обновляем сообщение
        let updated_message = self.message_repo
            .update_message(message_id, request.content)
            .await?;

        // Обновляем кеш
        self.redis.cache_message(&updated_message).await?;

        // Отправляем событие
        let event = ChatEvent::MessageEdited(updated_message.clone());
        if let Err(e) = self.event_sender.send(event).await {
            warn!("Failed to send chat event: {}", e);
        }

        self.redis.publish_event(&ChatEvent::MessageEdited(updated_message.clone())).await?;

        Ok(updated_message)
    }

    pub async fn delete_message(
        &self,
        message_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), ChatError> {
        // Получаем сообщение
        let message = self.message_repo.get_message(message_id).await?;

        // Проверяем права на удаление
        if message.sender_id != user_id {
            return Err(ChatError::Authorization {
                message: "Нет прав на удаление этого сообщения".to_string(),
            });
        }

        // Удаляем сообщение
        self.message_repo.delete_message(message_id).await?;

        // Отправляем событие
        let event = ChatEvent::MessageDeleted {
            message_id,
            channel_id: message.channel_id,
        };
        if let Err(e) = self.event_sender.send(event).await {
            warn!("Failed to send chat event: {}", e);
        }

        self.redis.publish_event(&ChatEvent::MessageDeleted {
            message_id,
            channel_id: message.channel_id,
        }).await?;

        Ok(())
    }

    pub async fn start_typing(
        &self,
        channel_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), ChatError> {
        // Проверяем доступ к каналу
        self.check_channel_access(channel_id, user_id).await?;

        // Устанавливаем статус typing в Redis
        self.redis.set_typing(user_id, channel_id).await?;

        // Отправляем событие
        let event = ChatEvent::UserTyping { user_id, channel_id };
        self.redis.publish_event(&event).await?;

        Ok(())
    }

    pub async fn stop_typing(
        &self,
        channel_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), ChatError> {
        // Отправляем событие
        let event = ChatEvent::UserStoppedTyping { user_id, channel_id };
        self.redis.publish_event(&event).await?;

        Ok(())
    }

    async fn check_channel_access(&self, channel_id: Uuid, user_id: Uuid) -> Result<(), ChatError> {
        // Получаем информацию о канале
        let channel = self.channel_repo.get_channel(channel_id).await?;

        // Для публичных каналов проверяем участников
        if channel.is_private {
            // TODO: Проверить участие в приватном канале
            // Пока пропускаем
        }

        Ok(())
    }
}

// ============================================================================
// Сервис управления каналами
// ============================================================================

#[derive(Clone)]
pub struct ChannelService {
    channel_repo: Arc<ChannelRepository>,
    redis: Arc<RedisService>,
    config: Arc<ChatConfig>,
    event_sender: mpsc::Sender<ChatEvent>,
}

impl ChannelService {
    pub fn new(
        channel_repo: Arc<ChannelRepository>,
        redis: Arc<RedisService>,
        config: Arc<ChatConfig>,
        event_sender: mpsc::Sender<ChatEvent>,
    ) -> Self {
        Self {
            channel_repo,
            redis,
            config,
            event_sender,
        }
    }

    pub async fn create_channel(
        &self,
        creator_id: Uuid,
        request: CreateChannelRequest,
    ) -> Result<ChatChannel, ChatError> {
        // Валидация запроса
        request.validate().map_err(ChatError::Validation)?;

        // Создаем канал
        let mut channel = ChatChannel::new(
            request.name,
            request.channel_type,
            creator_id,
        );

        // Устанавливаем дополнительные параметры
        if let Some(is_private) = request.is_private {
            channel.is_private = is_private;
        }

        if let Some(max_participants) = request.max_participants {
            channel.max_participants = Some(max_participants);
        }

        if let Some(settings) = request.settings {
            channel.set_settings(settings);
        }

        if let Some(telegram_chat_id) = request.telegram_chat_id {
            channel.telegram_chat_id = Some(telegram_chat_id);
        }

        // Сохраняем в базу данных
        let saved_channel = self.channel_repo.create_channel(&channel).await?;

        // Добавляем создателя как владельца
        self.channel_repo
            .add_member(saved_channel.id, creator_id, MemberRole::Owner)
            .await?;

        // Отправляем событие
        let event = ChatEvent::ChannelCreated(saved_channel.clone());
        if let Err(e) = self.event_sender.send(event).await {
            warn!("Failed to send chat event: {}", e);
        }

        info!(
            channel_id = %saved_channel.id,
            creator_id = %creator_id,
            channel_name = %saved_channel.name,
            "Channel created successfully"
        );

        Ok(saved_channel)
    }

    pub async fn join_channel(
        &self,
        channel_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), ChatError> {
        // Получаем информацию о канале
        let channel = self.channel_repo.get_channel(channel_id).await?;

        // Проверяем лимит участников
        if let Some(max_participants) = channel.max_participants {
            let online_users = self.redis.get_online_users(channel_id).await?;
            if online_users.len() >= max_participants as usize {
                return Err(ChatError::ChannelFull {
                    current: online_users.len() as u32,
                    max: max_participants as u32,
                });
            }
        }

        // Добавляем участника
        self.channel_repo
            .add_member(channel_id, user_id, MemberRole::Member)
            .await?;

        // Отмечаем пользователя как онлайн в канале
        self.redis.set_user_online(user_id, channel_id).await?;

        // Отправляем событие
        let event = ChatEvent::UserJoinedChannel { user_id, channel_id };
        self.redis.publish_event(&event).await?;

        Ok(())
    }

    pub async fn leave_channel(
        &self,
        channel_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), ChatError> {
        // Удаляем участника
        self.channel_repo.remove_member(channel_id, user_id).await?;

        // Отмечаем пользователя как оффлайн
        self.redis.set_user_offline(user_id, channel_id).await?;

        // Отправляем событие
        let event = ChatEvent::UserLeftChannel { user_id, channel_id };
        self.redis.publish_event(&event).await?;

        Ok(())
    }

    pub async fn get_user_channels(&self, user_id: Uuid) -> Result<Vec<ChatChannel>, ChatError> {
        self.channel_repo.get_user_channels(user_id).await
    }

    pub async fn get_channel(&self, channel_id: Uuid) -> Result<ChatChannel, ChatError> {
        self.channel_repo.get_channel(channel_id).await
    }
}

// ============================================================================
// Сервис голосового чата
// ============================================================================

#[derive(Clone)]
pub struct VoiceService {
    redis: Arc<RedisService>,
    config: Arc<ChatConfig>,
    event_sender: mpsc::Sender<ChatEvent>,
}

impl VoiceService {
    pub fn new(
        redis: Arc<RedisService>,
        config: Arc<ChatConfig>,
        event_sender: mpsc::Sender<ChatEvent>,
    ) -> Self {
        Self {
            redis,
            config,
            event_sender,
        }
    }

    pub async fn start_voice_session(
        &self,
        user_id: Uuid,
        request: StartVoiceSessionRequest,
    ) -> Result<VoiceSession, ChatError> {
        // Проверяем что голосовой чат включен
        if !self.config.voice.enabled {
            return Err(ChatError::Configuration {
                message: "Голосовой чат отключен".to_string(),
            });
        }

        // Создаем голосовую сессию
        let mut session = VoiceSession::new(user_id, request.session_type);

        if let Some(channel_id) = request.channel_id {
            session.channel_id = Some(channel_id);
        }

        if let Some(voice_range) = request.voice_range {
            session.voice_range = voice_range;
        }

        if let Some(position) = request.position {
            session.set_position(position);
        }

        if let Some(quality) = request.quality {
            session.quality = quality;
        }

        // Сохраняем сессию в Redis
        self.redis.store_voice_session(&session).await?;

        // Отправляем событие
        let event = ChatEvent::VoiceSessionStarted(session.clone());
        if let Err(e) = self.event_sender.send(event).await {
            warn!("Failed to send voice event: {}", e);
        }

        self.redis.publish_event(&ChatEvent::VoiceSessionStarted(session.clone())).await?;

        info!(
            session_id = %session.id,
            user_id = %user_id,
            "Voice session started"
        );

        Ok(session)
    }

    pub async fn end_voice_session(
        &self,
        session_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), ChatError> {
        // Получаем сессию
        let session = self.redis.get_voice_session(session_id).await?
            .ok_or_else(|| ChatError::VoiceSessionNotFound {
                id: session_id.to_string(),
            })?;

        // Проверяем права
        if session.user_id != user_id {
            return Err(ChatError::Authorization {
                message: "Нет прав на завершение этой голосовой сессии".to_string(),
            });
        }

        // Удаляем сессию
        self.redis.remove_voice_session(session_id, user_id).await?;

        // Отправляем событие
        let event = ChatEvent::VoiceSessionEnded { session_id };
        self.redis.publish_event(&event).await?;

        Ok(())
    }

    pub async fn update_voice_position(
        &self,
        session_id: Uuid,
        user_id: Uuid,
        request: UpdateVoicePositionRequest,
    ) -> Result<(), ChatError> {
        // Получаем сессию
        let mut session = self.redis.get_voice_session(session_id).await?
            .ok_or_else(|| ChatError::VoiceSessionNotFound {
                id: session_id.to_string(),
            })?;

        // Проверяем права
        if session.user_id != user_id {
            return Err(ChatError::Authorization {
                message: "Нет прав на обновление этой голосовой сессии".to_string(),
            });
        }

        // Обновляем позицию
        session.set_position(request.position.clone());
        if let Some(voice_range) = request.voice_range {
            session.voice_range = voice_range;
        }

        // Сохраняем обновленную сессию
        self.redis.store_voice_session(&session).await?;

        // Отправляем событие
        let event = ChatEvent::VoicePositionUpdate {
            session_id,
            position: request.position,
        };
        self.redis.publish_event(&event).await?;

        Ok(())
    }
}

// ============================================================================
// Сервис интеграции с Telegram
// ============================================================================

#[derive(Clone)]
pub struct TelegramIntegrationService {
    telegram: Arc<TelegramService>,
    chat_service: Arc<ChatService>,
    config: Arc<ChatConfig>,
}

impl TelegramIntegrationService {
    pub fn new(
        telegram: Arc<TelegramService>,
        chat_service: Arc<ChatService>,
        config: Arc<ChatConfig>,
    ) -> Self {
        Self {
            telegram,
            chat_service,
            config,
        }
    }

    pub async fn forward_game_message_to_telegram(
        &self,
        message: &ChatMessage,
        telegram_chat_id: i64,
        player_name: Option<&str>,
    ) -> Result<(), ChatError> {
        if !self.config.telegram.enabled {
            return Ok(());
        }

        self.telegram
            .forward_game_message(message, telegram_chat_id, player_name)
            .await
    }

    pub async fn handle_telegram_message(
        &self,
        telegram_message: crate::infrastructure::TelegramMessage,
        linked_channel_id: Uuid,
    ) -> Result<(), ChatError> {
        self.telegram
            .handle_telegram_message(telegram_message, linked_channel_id)
            .await
    }
} 