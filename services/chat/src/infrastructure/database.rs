// TODO: Реализация репозиториев для PostgreSQL
// Этот модуль будет содержать репозитории для работы с базой данных

use crate::{ChatMessage, ChatChannel, ChannelMember, MessageStatus};
use crate::domain::{ChatError, SendMessageRequest, CreateChannelRequest, GetMessagesQuery};
use sqlx::{PgPool, Row, postgres::PgPoolOptions};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::time::Duration;

#[derive(Clone)]
pub struct DatabaseService {
    pool: PgPool,
}

pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: u64,
}

impl DatabaseService {
    pub async fn new(config: &DatabaseConfig) -> Result<Self, ChatError> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(config.connection_timeout))
            .connect(&config.url)
            .await
            .map_err(ChatError::Database)?;

        // Запускаем миграции
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|e| ChatError::Internal { 
                message: format!("Migration error: {}", e) 
            })?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

// ============================================================================
// Репозиторий для сообщений
// ============================================================================

#[derive(Clone)]
pub struct MessageRepository {
    pool: PgPool,
}

impl MessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_message(&self, message: &ChatMessage) -> Result<ChatMessage, ChatError> {
        // Используем простой query вместо макроса для избежания ошибок компиляции
        let result = sqlx::query_as::<_, ChatMessage>(
            r#"
            INSERT INTO chat_messages (
                id, channel_id, sender_id, content, message_type, 
                reply_to, forwarded_from, created_at, updated_at, metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, channel_id, sender_id, content, 
                     message_type, reply_to, forwarded_from,
                     edited_at, deleted_at, created_at, updated_at, metadata
            "#,
        )
        .bind(message.id)
        .bind(message.channel_id)
        .bind(message.sender_id)
        .bind(&message.content)
        .bind(&message.message_type)
        .bind(message.reply_to)
        .bind(message.forwarded_from)
        .bind(message.created_at)
        .bind(message.updated_at)
        .bind(&message.metadata)
        .fetch_one(&self.pool)
        .await
        .map_err(ChatError::Database)?;

        Ok(result)
    }

    pub async fn get_message(&self, message_id: Uuid) -> Result<ChatMessage, ChatError> {
        let message = sqlx::query_as::<_, ChatMessage>(
            r#"
            SELECT id, channel_id, sender_id, content,
                   message_type, reply_to, forwarded_from,
                   edited_at, deleted_at, created_at, updated_at, metadata
            FROM chat_messages 
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(message_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(ChatError::Database)?
        .ok_or_else(|| ChatError::MessageNotFound { 
            id: message_id.to_string() 
        })?;

        Ok(message)
    }

    pub async fn get_messages(
        &self, 
        channel_id: Uuid, 
        query: &GetMessagesQuery
    ) -> Result<Vec<ChatMessage>, ChatError> {
        let limit = query.limit.unwrap_or(50).min(100) as i64;
        let offset = query.offset.unwrap_or(0) as i64;

        // Упрощенная версия без динамических параметров
        let messages = sqlx::query_as::<_, ChatMessage>(
            r#"
            SELECT id, channel_id, sender_id, content,
                   message_type, reply_to, forwarded_from,
                   edited_at, deleted_at, created_at, updated_at, metadata
            FROM chat_messages 
            WHERE channel_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(channel_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(ChatError::Database)?;

        Ok(messages)
    }

    pub async fn update_message(
        &self, 
        message_id: Uuid, 
        new_content: String
    ) -> Result<ChatMessage, ChatError> {
        let message = sqlx::query_as::<_, ChatMessage>(
            r#"
            UPDATE chat_messages 
            SET content = $2, edited_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, channel_id, sender_id, content,
                     message_type, reply_to, forwarded_from,
                     edited_at, deleted_at, created_at, updated_at, metadata
            "#,
        )
        .bind(message_id)
        .bind(new_content)
        .fetch_optional(&self.pool)
        .await
        .map_err(ChatError::Database)?
        .ok_or_else(|| ChatError::MessageNotFound { 
            id: message_id.to_string() 
        })?;

        Ok(message)
    }

    pub async fn delete_message(&self, message_id: Uuid) -> Result<(), ChatError> {
        let result = sqlx::query(
            "UPDATE chat_messages SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
        )
        .bind(message_id)
        .execute(&self.pool)
        .await
        .map_err(ChatError::Database)?;

        if result.rows_affected() == 0 {
            return Err(ChatError::MessageNotFound { 
                id: message_id.to_string() 
            });
        }

        Ok(())
    }
}

// ============================================================================
// Репозиторий для каналов
// ============================================================================

#[derive(Clone)]
pub struct ChannelRepository {
    pool: PgPool,
}

impl ChannelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_channel(&self, channel: &ChatChannel) -> Result<ChatChannel, ChatError> {
        let result = sqlx::query_as::<_, ChatChannel>(
            r#"
            INSERT INTO chat_channels (
                id, name, channel_type, created_by, is_private, 
                is_temporary, max_participants, settings, created_at, 
                expires_at, telegram_chat_id
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, name, channel_type, created_by,
                     is_private, is_temporary, max_participants, settings,
                     created_at, expires_at, telegram_chat_id
            "#,
        )
        .bind(channel.id)
        .bind(&channel.name)
        .bind(&channel.channel_type)
        .bind(channel.created_by)
        .bind(channel.is_private)
        .bind(channel.is_temporary)
        .bind(channel.max_participants)
        .bind(&channel.settings)
        .bind(channel.created_at)
        .bind(channel.expires_at)
        .bind(channel.telegram_chat_id)
        .fetch_one(&self.pool)
        .await
        .map_err(ChatError::Database)?;

        Ok(result)
    }

    pub async fn get_channel(&self, channel_id: Uuid) -> Result<ChatChannel, ChatError> {
        let channel = sqlx::query_as::<_, ChatChannel>(
            r#"
            SELECT id, name, channel_type, created_by,
                   is_private, is_temporary, max_participants, settings,
                   created_at, expires_at, telegram_chat_id
            FROM chat_channels 
            WHERE id = $1
            "#,
        )
        .bind(channel_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(ChatError::Database)?
        .ok_or_else(|| ChatError::ChannelNotFound { 
            id: channel_id.to_string() 
        })?;

        Ok(channel)
    }

    pub async fn get_user_channels(&self, user_id: Uuid) -> Result<Vec<ChatChannel>, ChatError> {
        let channels = sqlx::query_as::<_, ChatChannel>(
            r#"
            SELECT c.id, c.name, c.channel_type, c.created_by,
                   c.is_private, c.is_temporary, c.max_participants, c.settings,
                   c.created_at, c.expires_at, c.telegram_chat_id
            FROM chat_channels c
            INNER JOIN channel_members cm ON c.id = cm.channel_id
            WHERE cm.user_id = $1
            ORDER BY c.created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(ChatError::Database)?;

        Ok(channels)
    }

    pub async fn add_member(
        &self, 
        channel_id: Uuid, 
        user_id: Uuid,
        role: crate::domain::MemberRole
    ) -> Result<(), ChatError> {
        sqlx::query(
            r#"
            INSERT INTO channel_members (channel_id, user_id, role, joined_at)
            VALUES ($1, $2, $3, NOW())
            ON CONFLICT (channel_id, user_id) DO NOTHING
            "#,
        )
        .bind(channel_id)
        .bind(user_id)
        .bind(&role)
        .execute(&self.pool)
        .await
        .map_err(ChatError::Database)?;

        Ok(())
    }

    pub async fn remove_member(&self, channel_id: Uuid, user_id: Uuid) -> Result<(), ChatError> {
        sqlx::query(
            "DELETE FROM channel_members WHERE channel_id = $1 AND user_id = $2",
        )
        .bind(channel_id)
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(ChatError::Database)?;

        Ok(())
    }
}

// ============================================================================
// Настройка базы данных
// ============================================================================

pub async fn setup_database(config: &DatabaseConfig) -> Result<DatabaseService, ChatError> {
    DatabaseService::new(config).await
} 