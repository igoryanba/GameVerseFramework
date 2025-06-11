use crate::domain::{ChatMessage, ChatChannel, VoiceSession, MessageType, ChannelType};
use uuid::Uuid;
use std::str::FromStr;

// Включаем сгенерированный код
pub mod gameverse {
    pub mod chat {
        pub mod v1 {
            tonic::include_proto!("gameverse.chat.v1");
        }
    }
}

use gameverse::chat::v1;

// Конвертеры для ChatMessage
impl From<ChatMessage> for v1::ChatMessage {
    fn from(msg: ChatMessage) -> Self {
        Self {
            id: msg.id.to_string(),
            channel_id: msg.channel_id.to_string(),
            user_id: msg.user_id.to_string(),
            username: msg.username,
            content: msg.content,
            message_type: v1::MessageType::from(msg.message_type) as i32,
            metadata: msg.metadata.map(|m| v1::MessageMetadata {
                reply_to: m.reply_to.map(|id| id.to_string()).unwrap_or_default(),
                forward_from: m.forward_from.map(|id| id.to_string()).unwrap_or_default(),
                attachments: m.attachments,
                mentions: m.mentions,
                custom_data: m.custom_data,
            }),
            created_at: Some(prost_types::Timestamp {
                seconds: msg.created_at.timestamp(),
                nanos: msg.created_at.timestamp_subsec_nanos() as i32,
            }),
            updated_at: msg.updated_at.map(|t| prost_types::Timestamp {
                seconds: t.timestamp(),
                nanos: t.timestamp_subsec_nanos() as i32,
            }),
        }
    }
}

impl TryFrom<v1::ChatMessage> for ChatMessage {
    type Error = anyhow::Error;

    fn try_from(msg: v1::ChatMessage) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::from_str(&msg.id)?,
            channel_id: Uuid::from_str(&msg.channel_id)?,
            user_id: Uuid::from_str(&msg.user_id)?,
            username: msg.username,
            content: msg.content,
            message_type: MessageType::from(v1::MessageType::try_from(msg.message_type)?),
            metadata: msg.metadata.map(|m| crate::domain::MessageMetadata {
                reply_to: if m.reply_to.is_empty() { None } else { Some(Uuid::from_str(&m.reply_to).ok()?) },
                forward_from: if m.forward_from.is_empty() { None } else { Some(Uuid::from_str(&m.forward_from).ok()?) },
                attachments: m.attachments,
                mentions: m.mentions,
                custom_data: m.custom_data,
            }),
            created_at: msg.created_at
                .map(|t| chrono::DateTime::from_timestamp(t.seconds, t.nanos as u32))
                .unwrap_or_default()
                .ok_or_else(|| anyhow::anyhow!("Invalid timestamp"))?,
            updated_at: msg.updated_at
                .map(|t| chrono::DateTime::from_timestamp(t.seconds, t.nanos as u32))
                .unwrap_or_default(),
        })
    }
}

// Конвертеры для ChatChannel
impl From<ChatChannel> for v1::ChatChannel {
    fn from(channel: ChatChannel) -> Self {
        Self {
            id: channel.id.to_string(),
            name: channel.name,
            description: channel.description,
            channel_type: v1::ChannelType::from(channel.channel_type) as i32,
            settings: Some(v1::ChannelSettings {
                is_private: channel.settings.is_private,
                require_permission: channel.settings.require_permission,
                max_members: channel.settings.max_members as i32,
                range_limit: channel.settings.range_limit,
                allowed_roles: channel.settings.allowed_roles,
            }),
            members: channel.members.into_iter().map(|id| id.to_string()).collect(),
            moderators: channel.moderators.into_iter().map(|id| id.to_string()).collect(),
            created_by: channel.created_by.to_string(),
            created_at: Some(prost_types::Timestamp {
                seconds: channel.created_at.timestamp(),
                nanos: channel.created_at.timestamp_subsec_nanos() as i32,
            }),
        }
    }
}

// Конвертеры для типов сообщений
impl From<MessageType> for v1::MessageType {
    fn from(msg_type: MessageType) -> Self {
        match msg_type {
            MessageType::Text => v1::MessageType::MessageTypeText,
            MessageType::Voice => v1::MessageType::MessageTypeVoice,
            MessageType::Image => v1::MessageType::MessageTypeImage,
            MessageType::System => v1::MessageType::MessageTypeSystem,
            MessageType::Command => v1::MessageType::MessageTypeCommand,
            MessageType::Emote => v1::MessageType::MessageTypeEmote,
        }
    }
}

impl From<v1::MessageType> for MessageType {
    fn from(msg_type: v1::MessageType) -> Self {
        match msg_type {
            v1::MessageType::MessageTypeText => MessageType::Text,
            v1::MessageType::MessageTypeVoice => MessageType::Voice,
            v1::MessageType::MessageTypeImage => MessageType::Image,
            v1::MessageType::MessageTypeSystem => MessageType::System,
            v1::MessageType::MessageTypeCommand => MessageType::Command,
            v1::MessageType::MessageTypeEmote => MessageType::Emote,
            v1::MessageType::MessageTypeUnspecified => MessageType::Text,
        }
    }
}

// Конвертеры для типов каналов
impl From<ChannelType> for v1::ChannelType {
    fn from(channel_type: ChannelType) -> Self {
        match channel_type {
            ChannelType::Global => v1::ChannelType::ChannelTypeGlobal,
            ChannelType::Local => v1::ChannelType::ChannelTypeLocal,
            ChannelType::Radio => v1::ChannelType::ChannelTypeRadio,
            ChannelType::Phone => v1::ChannelType::ChannelTypePhone,
            ChannelType::Group => v1::ChannelType::ChannelTypeGroup,
            ChannelType::Direct => v1::ChannelType::ChannelTypeDirect,
            ChannelType::Ooc => v1::ChannelType::ChannelTypeOoc,
            ChannelType::Admin => v1::ChannelType::ChannelTypeAdmin,
        }
    }
}

impl From<v1::ChannelType> for ChannelType {
    fn from(channel_type: v1::ChannelType) -> Self {
        match channel_type {
            v1::ChannelType::ChannelTypeGlobal => ChannelType::Global,
            v1::ChannelType::ChannelTypeLocal => ChannelType::Local,
            v1::ChannelType::ChannelTypeRadio => ChannelType::Radio,
            v1::ChannelType::ChannelTypePhone => ChannelType::Phone,
            v1::ChannelType::ChannelTypeGroup => ChannelType::Group,
            v1::ChannelType::ChannelTypeDirect => ChannelType::Direct,
            v1::ChannelType::ChannelTypeOoc => ChannelType::Ooc,
            v1::ChannelType::ChannelTypeAdmin => ChannelType::Admin,
            v1::ChannelType::ChannelTypeUnspecified => ChannelType::Global,
        }
    }
} 