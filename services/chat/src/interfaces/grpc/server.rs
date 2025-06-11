use std::sync::Arc;
use tonic::{Request, Response, Status, Result};
use uuid::Uuid;
use std::str::FromStr;

use crate::application::services::{ChatService, ChannelService, VoiceService};
use crate::domain::{ChatError, SendMessageRequest, CreateChannelRequest, GetMessagesQuery, EditMessageRequest, StartVoiceSessionRequest, UpdateVoicePositionRequest};

use super::converters::gameverse::chat::v1::*;

pub struct ChatGrpcService {
    pub chat_service: Arc<ChatService>,
    pub channel_service: Arc<ChannelService>, 
    pub voice_service: Arc<VoiceService>,
}

impl ChatGrpcService {
    pub fn new(
        chat_service: Arc<ChatService>,
        channel_service: Arc<ChannelService>,
        voice_service: Arc<VoiceService>,
    ) -> Self {
        Self {
            chat_service,
            channel_service,
            voice_service,
        }
    }
}

#[tonic::async_trait]
impl chat_service_server::ChatService for ChatGrpcService {
    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let req = request.into_inner();
        
        let channel_id = Uuid::from_str(&req.channel_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid channel_id: {}", e)))?;
        let user_id = Uuid::from_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user_id: {}", e)))?;

        let send_request = crate::domain::SendMessageRequest {
            content: req.content,
            message_type: super::converters::MessageType::from(
                super::converters::v1::MessageType::try_from(req.message_type)
                    .map_err(|e| Status::invalid_argument(format!("Invalid message type: {}", e)))?
            ),
            metadata: req.metadata.map(|m| crate::domain::MessageMetadata {
                reply_to: if m.reply_to.is_empty() { None } else { Some(Uuid::from_str(&m.reply_to).ok().ok_or_else(|| Status::invalid_argument("Invalid reply_to UUID"))?) },
                forward_from: if m.forward_from.is_empty() { None } else { Some(Uuid::from_str(&m.forward_from).ok().ok_or_else(|| Status::invalid_argument("Invalid forward_from UUID"))?) },
                attachments: m.attachments,
                mentions: m.mentions,
                custom_data: m.custom_data,
            }),
        };

        let message = self.chat_service
            .send_message(channel_id, user_id, send_request)
            .await
            .map_err(|e| match e {
                ChatError::NotFound { .. } => Status::not_found(e.to_string()),
                ChatError::Unauthorized { .. } => Status::permission_denied(e.to_string()),
                ChatError::Validation { .. } => Status::invalid_argument(e.to_string()),
                ChatError::RateLimit { .. } => Status::resource_exhausted(e.to_string()),
                _ => Status::internal(e.to_string()),
            })?;

        Ok(Response::new(SendMessageResponse {
            message: Some(message.into()),
        }))
    }

    async fn get_messages(
        &self,
        request: Request<GetMessagesRequest>,
    ) -> Result<Response<GetMessagesResponse>, Status> {
        let req = request.into_inner();
        
        let channel_id = Uuid::from_str(&req.channel_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid channel_id: {}", e)))?;
        let user_id = Uuid::from_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user_id: {}", e)))?;

        let query = GetMessagesQuery {
            limit: req.limit.max(1).min(100) as usize,
            offset: req.offset.max(0) as usize,
            before: req.before.map(|t| chrono::DateTime::from_timestamp(t.seconds, t.nanos as u32)).flatten(),
            after: req.after.map(|t| chrono::DateTime::from_timestamp(t.seconds, t.nanos as u32)).flatten(),
        };

        let messages = self.chat_service
            .get_messages(channel_id, user_id, query)
            .await
            .map_err(|e| match e {
                ChatError::NotFound { .. } => Status::not_found(e.to_string()),
                ChatError::Unauthorized { .. } => Status::permission_denied(e.to_string()),
                _ => Status::internal(e.to_string()),
            })?;

        Ok(Response::new(GetMessagesResponse {
            messages: messages.into_iter().map(|m| m.into()).collect(),
            total_count: messages.len() as i32,
            has_more: messages.len() == query.limit,
        }))
    }

    async fn edit_message(
        &self,
        request: Request<EditMessageRequest>,
    ) -> Result<Response<EditMessageResponse>, Status> {
        let req = request.into_inner();
        
        let message_id = Uuid::from_str(&req.message_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid message_id: {}", e)))?;
        let user_id = Uuid::from_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user_id: {}", e)))?;

        let edit_request = crate::domain::EditMessageRequest {
            new_content: req.new_content,
            metadata: req.metadata.map(|m| crate::domain::MessageMetadata {
                reply_to: if m.reply_to.is_empty() { None } else { Some(Uuid::from_str(&m.reply_to).ok().ok_or_else(|| Status::invalid_argument("Invalid reply_to UUID"))?) },
                forward_from: if m.forward_from.is_empty() { None } else { Some(Uuid::from_str(&m.forward_from).ok().ok_or_else(|| Status::invalid_argument("Invalid forward_from UUID"))?) },
                attachments: m.attachments,
                mentions: m.mentions,
                custom_data: m.custom_data,
            }),
        };

        let message = self.chat_service
            .edit_message(message_id, user_id, edit_request)
            .await
            .map_err(|e| match e {
                ChatError::NotFound { .. } => Status::not_found(e.to_string()),
                ChatError::Unauthorized { .. } => Status::permission_denied(e.to_string()),
                ChatError::Validation { .. } => Status::invalid_argument(e.to_string()),
                _ => Status::internal(e.to_string()),
            })?;

        Ok(Response::new(EditMessageResponse {
            message: Some(message.into()),
        }))
    }

    async fn delete_message(
        &self,
        request: Request<DeleteMessageRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        
        let message_id = Uuid::from_str(&req.message_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid message_id: {}", e)))?;
        let user_id = Uuid::from_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user_id: {}", e)))?;

        self.chat_service
            .delete_message(message_id, user_id)
            .await
            .map_err(|e| match e {
                ChatError::NotFound { .. } => Status::not_found(e.to_string()),
                ChatError::Unauthorized { .. } => Status::permission_denied(e.to_string()),
                _ => Status::internal(e.to_string()),
            })?;

        Ok(Response::new(()))
    }

    async fn start_typing(
        &self,
        request: Request<TypingRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        
        let channel_id = Uuid::from_str(&req.channel_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid channel_id: {}", e)))?;
        let user_id = Uuid::from_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user_id: {}", e)))?;

        self.chat_service
            .start_typing(channel_id, user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
    }

    async fn stop_typing(
        &self,
        request: Request<TypingRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        
        let channel_id = Uuid::from_str(&req.channel_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid channel_id: {}", e)))?;
        let user_id = Uuid::from_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user_id: {}", e)))?;

        self.chat_service
            .stop_typing(channel_id, user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(()))
    }
}

#[tonic::async_trait]
impl channel_service_server::ChannelService for ChatGrpcService {
    async fn create_channel(
        &self,
        request: Request<CreateChannelRequest>,
    ) -> Result<Response<CreateChannelResponse>, Status> {
        let req = request.into_inner();
        
        let created_by = Uuid::from_str(&req.created_by)
            .map_err(|e| Status::invalid_argument(format!("Invalid created_by: {}", e)))?;

        let create_request = crate::domain::CreateChannelRequest {
            name: req.name,
            description: req.description,
            channel_type: super::converters::ChannelType::from(
                super::converters::v1::ChannelType::try_from(req.channel_type)
                    .map_err(|e| Status::invalid_argument(format!("Invalid channel type: {}", e)))?
            ),
            settings: req.settings.map(|s| crate::domain::ChannelSettings {
                is_private: s.is_private,
                require_permission: s.require_permission,
                max_members: s.max_members as usize,
                range_limit: s.range_limit,
                allowed_roles: s.allowed_roles,
            }).unwrap_or_default(),
        };

        let channel = self.channel_service
            .create_channel(created_by, create_request)
            .await
            .map_err(|e| match e {
                ChatError::Validation { .. } => Status::invalid_argument(e.to_string()),
                ChatError::Unauthorized { .. } => Status::permission_denied(e.to_string()),
                _ => Status::internal(e.to_string()),
            })?;

        Ok(Response::new(CreateChannelResponse {
            channel: Some(channel.into()),
        }))
    }

    async fn get_channel(
        &self,
        request: Request<GetChannelRequest>,
    ) -> Result<Response<GetChannelResponse>, Status> {
        let req = request.into_inner();
        
        let channel_id = Uuid::from_str(&req.channel_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid channel_id: {}", e)))?;

        let channel = self.channel_service
            .get_channel(channel_id)
            .await
            .map_err(|e| match e {
                ChatError::NotFound { .. } => Status::not_found(e.to_string()),
                _ => Status::internal(e.to_string()),
            })?;

        Ok(Response::new(GetChannelResponse {
            channel: Some(channel.into()),
        }))
    }

    async fn get_user_channels(
        &self,
        request: Request<GetUserChannelsRequest>,
    ) -> Result<Response<GetUserChannelsResponse>, Status> {
        let req = request.into_inner();
        
        let user_id = Uuid::from_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user_id: {}", e)))?;

        let channels = self.channel_service
            .get_user_channels(user_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetUserChannelsResponse {
            channels: channels.into_iter().map(|c| c.into()).collect(),
        }))
    }

    async fn join_channel(
        &self,
        request: Request<JoinChannelRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        
        let channel_id = Uuid::from_str(&req.channel_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid channel_id: {}", e)))?;
        let user_id = Uuid::from_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user_id: {}", e)))?;

        self.channel_service
            .join_channel(channel_id, user_id)
            .await
            .map_err(|e| match e {
                ChatError::NotFound { .. } => Status::not_found(e.to_string()),
                ChatError::Unauthorized { .. } => Status::permission_denied(e.to_string()),
                ChatError::Validation { .. } => Status::invalid_argument(e.to_string()),
                _ => Status::internal(e.to_string()),
            })?;

        Ok(Response::new(()))
    }

    async fn leave_channel(
        &self,
        request: Request<LeaveChannelRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        
        let channel_id = Uuid::from_str(&req.channel_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid channel_id: {}", e)))?;
        let user_id = Uuid::from_str(&req.user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid user_id: {}", e)))?;

        self.channel_service
            .leave_channel(channel_id, user_id)
            .await
            .map_err(|e| match e {
                ChatError::NotFound { .. } => Status::not_found(e.to_string()),
                _ => Status::internal(e.to_string()),
            })?;

        Ok(Response::new(()))
    }
}

pub async fn start_grpc_server(
    chat_service: Arc<ChatService>,
    channel_service: Arc<ChannelService>,
    voice_service: Arc<VoiceService>,
    addr: std::net::SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let grpc_service = ChatGrpcService::new(chat_service, channel_service, voice_service);

    let server = tonic::transport::Server::builder()
        .add_service(chat_service_server::ChatServiceServer::new(grpc_service.clone()))
        .add_service(channel_service_server::ChannelServiceServer::new(grpc_service.clone()))
        .serve(addr);

    tracing::info!("🚀 gRPC server listening on {}", addr);

    server.await?;

    Ok(())
} 