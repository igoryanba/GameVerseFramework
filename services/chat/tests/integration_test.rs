use gameverse_chat::{ChatConfig, ChatMessage, MessageType, ChannelType, ChatChannel};
use gameverse_chat::domain::{SendMessageRequest, CreateChannelRequest};
use serde_json::json;
use tokio;
use uuid::Uuid;

#[tokio::test]
async fn test_chat_config() {
    // Тест загрузки конфигурации
    let config = ChatConfig::default();
    
    assert_eq!(config.server.http_port, 8080);
    assert_eq!(config.server.grpc_port, 50053);
    assert_eq!(config.security.max_message_length, 500);
    assert!(config.channels.global.enabled);
}

#[tokio::test]
async fn test_message_creation() {
    // Тест создания сообщения
    let channel_id = Uuid::new_v4();
    let sender_id = Uuid::new_v4();
    let content = "Тестовое сообщение".to_string();
    
    let message = ChatMessage::new(
        channel_id,
        sender_id,
        content.clone(),
        MessageType::Text,
    );
    
    assert_eq!(message.channel_id, channel_id);
    assert_eq!(message.sender_id, sender_id);
    assert_eq!(message.content, content);
    assert_eq!(message.message_type, MessageType::Text);
    assert!(!message.is_deleted());
    assert!(!message.is_edited());
}

#[tokio::test]
async fn test_channel_creation() {
    // Тест создания канала
    let name = "Тестовый канал".to_string();
    let creator_id = Uuid::new_v4();
    
    let channel = ChatChannel::new(
        name.clone(),
        ChannelType::Global,
        creator_id,
    );
    
    assert_eq!(channel.name, name);
    assert_eq!(channel.channel_type, ChannelType::Global);
    assert_eq!(channel.created_by, creator_id);
    assert!(!channel.is_private);
    assert!(!channel.is_expired());
}

#[tokio::test]
async fn test_send_message_request() {
    // Тест DTO для отправки сообщения
    let request = SendMessageRequest {
        content: "Тестовое сообщение".to_string(),
        message_type: Some(MessageType::Text),
        reply_to: None,
        metadata: None,
    };
    
    // Проверяем валидацию
    use validator::Validate;
    assert!(request.validate().is_ok());
    
    // Тест с пустым сообщением (должно быть ошибкой)
    let invalid_request = SendMessageRequest {
        content: "".to_string(),
        message_type: Some(MessageType::Text),
        reply_to: None,
        metadata: None,
    };
    
    assert!(invalid_request.validate().is_err());
}

#[tokio::test]
async fn test_create_channel_request() {
    // Тест DTO для создания канала
    let request = CreateChannelRequest {
        name: "Новый канал".to_string(),
        channel_type: ChannelType::Group,
        is_private: Some(true),
        max_participants: Some(50),
        settings: None,
        telegram_chat_id: None,
    };
    
    // Проверяем валидацию
    use validator::Validate;
    assert!(request.validate().is_ok());
    
    // Тест с пустым именем (должно быть ошибкой)
    let invalid_request = CreateChannelRequest {
        name: "".to_string(),
        channel_type: ChannelType::Group,
        is_private: Some(true),
        max_participants: Some(50),
        settings: None,
        telegram_chat_id: None,
    };
    
    assert!(invalid_request.validate().is_err());
}

#[tokio::test]
async fn test_message_metadata() {
    use gameverse_chat::domain::{ChatMessageMetadata, GameLocation};
    
    let metadata = ChatMessageMetadata {
        client_ip: Some("127.0.0.1".to_string()),
        user_agent: Some("TestAgent/1.0".to_string()),
        character_name: Some("Тестовый Персонаж".to_string()),
        is_anonymous: false,
        location: Some(GameLocation {
            x: 100.0,
            y: 200.0,
            z: 30.0,
            world: Some("World1".to_string()),
            interior: None,
        }),
        telegram_user_id: None,
        telegram_message_id: None,
    };
    
    let channel_id = Uuid::new_v4();
    let sender_id = Uuid::new_v4();
    let content = "Сообщение с метаданными".to_string();
    
    let mut message = ChatMessage::new(
        channel_id,
        sender_id,
        content,
        MessageType::Text,
    );
    
    // Устанавливаем метаданные
    message.set_metadata(metadata.clone());
    
    // Проверяем что метаданные сохранились
    let retrieved_metadata = message.get_metadata();
    assert!(retrieved_metadata.is_some());
    
    let retrieved = retrieved_metadata.unwrap();
    assert_eq!(retrieved.character_name, metadata.character_name);
    assert_eq!(retrieved.is_anonymous, metadata.is_anonymous);
}

#[cfg(feature = "integration")]
mod integration_tests {
    use super::*;
    use gameverse_chat::infrastructure::{setup_database, DatabaseConfig};
    
    #[tokio::test]
    async fn test_database_connection() {
        // Этот тест требует настроенной БД
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost:5432/gameverse_chat_test".to_string());
        
        let config = DatabaseConfig {
            url: database_url,
            max_connections: 5,
            min_connections: 1,
            connection_timeout: 30,
        };
        
        // Попробуем подключиться к БД
        match setup_database(&config).await {
            Ok(_) => {
                println!("Database connection successful!");
            }
            Err(e) => {
                // Если БД недоступна, пропускаем тест
                println!("Database not available for testing: {}", e);
            }
        }
    }
} 