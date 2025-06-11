use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;
use sqlx::PgPool;

use gameverse_chat::*;
use gameverse_chat::domain::{SendMessageRequest, GetMessagesQuery, StartVoiceSessionRequest, UpdateVoicePositionRequest};
use gameverse_chat::infrastructure::{MessageRepository, ChannelRepository, RedisService, RedisConfig};
use gameverse_chat::application::services::*;

#[tokio::test]
async fn test_database_schema_and_migrations() {
    let pool = create_test_database_pool().await;
    
    // Проверяем что все таблицы созданы
    let tables = sqlx::query_scalar::<_, String>(
        "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'"
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to fetch tables");
    
    let expected_tables = vec![
        "chat_channels",
        "chat_messages", 
        "channel_members",
        "voice_sessions",
        "telegram_users",
        "telegram_channel_links",
        "message_statuses"
    ];
    
    for table in expected_tables {
        assert!(tables.contains(&table.to_string()), "Table {} not found", table);
    }
    
    cleanup_database(&pool).await;
}

#[tokio::test]
async fn test_message_repository_crud() {
    let pool = create_test_database_pool().await;
    let repo = MessageRepository::new(pool.clone());
    
    // Создаем тестовый канал
    let channel = create_test_channel(&pool).await;
    
    // Создание сообщения
    let message = ChatMessage::new(
        channel.id,
        Uuid::new_v4(), // sender_id
        "Test message".to_string(),
        MessageType::Text
    );
    
    let created_message = repo.create_message(&message).await
        .expect("Failed to create message");
    
    assert_eq!(created_message.content, "Test message");
    assert_eq!(created_message.channel_id, channel.id);
    
    // Получение сообщения
    let fetched_message = repo.get_message(created_message.id).await
        .expect("Failed to fetch message");
    
    assert_eq!(fetched_message.id, created_message.id);
    assert_eq!(fetched_message.content, "Test message");
    
    // Редактирование сообщения
    let updated_message = repo.update_message(created_message.id, "Edited message".to_string()).await
        .expect("Failed to update message");
    
    assert_eq!(updated_message.content, "Edited message");
    assert!(updated_message.edited_at.is_some());
    
    // Получение истории канала
    let query = GetMessagesQuery {
        limit: Some(10),
        offset: Some(0),
        before: None,
        after: None,
        message_type: None,
        search: None,
    };
    
    let messages = repo.get_messages(channel.id, &query).await
        .expect("Failed to get channel messages");
    
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].content, "Edited message");
    
    // Мягкое удаление
    repo.delete_message(created_message.id).await
        .expect("Failed to soft delete message");
    
    cleanup_database(&pool).await;
}

#[tokio::test]
async fn test_channel_repository_crud() {
    let pool = create_test_database_pool().await;
    let repo = ChannelRepository::new(pool.clone());
    
    // Создание канала
    let channel = ChatChannel::new(
        "Test Channel".to_string(),
        ChannelType::Group,
        Uuid::new_v4() // created_by
    );
    
    let created_channel = repo.create_channel(&channel).await
        .expect("Failed to create channel");
    
    assert_eq!(created_channel.name, "Test Channel");
    assert_eq!(created_channel.channel_type, ChannelType::Group);
    
    // Получение канала
    let fetched_channel = repo.get_channel(created_channel.id).await
        .expect("Failed to fetch channel");
    
    assert_eq!(fetched_channel.id, created_channel.id);
    
    // Добавление участника
    let user_id = Uuid::new_v4();
    repo.add_member(created_channel.id, user_id, MemberRole::Member).await
        .expect("Failed to add channel member");
    
    // Удаление участника
    repo.remove_member(created_channel.id, user_id).await
        .expect("Failed to remove channel member");
    
    cleanup_database(&pool).await;
}

#[tokio::test]
async fn test_redis_service_functionality() {
    let redis_config = RedisConfig {
        url: std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
        max_connections: 10,
        pool_timeout: 30,
        command_timeout: 5,
    };
    
    let redis_service = RedisService::new(&redis_config).await
        .expect("Failed to create Redis service");
    
    let channel_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    
    // Тестирование кеширования сообщений
    let message = ChatMessage::new(
        channel_id,
        user_id,
        "Cached message".to_string(),
        MessageType::Text
    );
    
    redis_service.cache_message(&message).await
        .expect("Failed to cache message");
    
    let cached_message = redis_service.get_cached_message(message.id).await
        .expect("Failed to get cached message");
    
    assert!(cached_message.is_some());
    assert_eq!(cached_message.unwrap().content, "Cached message");
    
    // Тестирование статуса онлайн
    redis_service.set_user_online(user_id, channel_id).await
        .expect("Failed to set user online");
    
    let online_users = redis_service.get_online_users(channel_id).await
        .expect("Failed to get online users");
    
    assert!(online_users.contains(&user_id));
    
    // Тестирование typing индикаторов
    redis_service.set_typing(user_id, channel_id).await
        .expect("Failed to set typing");
    
    let typing_users = redis_service.get_typing_users(channel_id).await
        .expect("Failed to get typing users");
    
    assert!(typing_users.contains(&user_id));
    
    // Тестирование rate limiting
    for _i in 0..5 {
        let allowed = redis_service.check_rate_limit(user_id, 10).await
            .expect("Failed to check rate limit");
        assert!(allowed);
    }
    
    cleanup_redis(&redis_service).await;
}

#[tokio::test]
async fn test_chat_service_end_to_end() {
    let pool = create_test_database_pool().await;
    let redis_config = RedisConfig {
        url: std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
        max_connections: 10,
        pool_timeout: 30,
        command_timeout: 5,
    };
    
    let redis_service = Arc::new(RedisService::new(&redis_config).await
        .expect("Failed to create Redis service"));
    let message_repo = Arc::new(MessageRepository::new(pool.clone()));
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let config = Arc::new(ChatConfig::default());
    let (event_sender, _event_receiver) = mpsc::channel(100);
    
    let chat_service = ChatService::new(
        message_repo,
        channel_repo.clone(),
        redis_service.clone(),
        config.clone(),
        event_sender.clone(),
    );
    
    // Создаем тестовый канал через репозиторий
    let channel = create_test_channel(&pool).await;
    
    // Отправляем сообщение
    let request = SendMessageRequest {
        content: "Hello, world!".to_string(),
        message_type: Some(MessageType::Text),
        reply_to: None,
        metadata: None,
    };
    
    let sent_message = chat_service.send_message(channel.id, Uuid::new_v4(), request).await
        .expect("Failed to send message");
    
    assert_eq!(sent_message.content, "Hello, world!");
    
    // Получаем историю сообщений
    let query = GetMessagesQuery {
        limit: Some(10),
        offset: None,
        before: None,
        after: None,
        message_type: None,
        search: None,
    };
    
    let messages = chat_service.get_messages(channel.id, Uuid::new_v4(), query).await
        .expect("Failed to get messages");
    
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].content, "Hello, world!");
    
    cleanup_database(&pool).await;
    cleanup_redis(&redis_service).await;
}

#[tokio::test]
async fn test_voice_session_management() {
    let redis_config = RedisConfig {
        url: std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
        max_connections: 10,
        pool_timeout: 30,
        command_timeout: 5,
    };
    
    let redis_service = Arc::new(RedisService::new(&redis_config).await
        .expect("Failed to create Redis service"));
    let config = Arc::new(ChatConfig::default());
    let (event_sender, _event_receiver) = mpsc::channel(100);
    
    let voice_service = VoiceService::new(
        redis_service.clone(),
        config.clone(),
        event_sender.clone(),
    );
    
    let user_id = Uuid::new_v4();
    
    // Начинаем голосовую сессию
    let request = StartVoiceSessionRequest {
        session_type: VoiceSessionType::Proximity,
        channel_id: None,
        voice_range: Some(VoiceRange::Normal),
        position: None,
        quality: Some(VoiceQuality::Medium),
    };
    
    let session = voice_service.start_voice_session(user_id, request).await
        .expect("Failed to start voice session");
    
    assert_eq!(session.user_id, user_id);
    assert_eq!(session.session_type, VoiceSessionType::Proximity);
    
    // Обновляем позицию
    let position_update = UpdateVoicePositionRequest {
        position: GamePosition {
            x: 100.0,
            y: 200.0,
            z: 30.0,
            rotation: Some(90.0),
        },
        voice_range: Some(VoiceRange::Shout),
    };
    
    voice_service.update_voice_position(session.id, user_id, position_update).await
        .expect("Failed to update voice position");
    
    // Завершаем сессию
    voice_service.end_voice_session(session.id, user_id).await
        .expect("Failed to end voice session");
    
    cleanup_redis(&redis_service).await;
}

// Вспомогательные функции

async fn create_test_database_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://gameverse:gameverse@localhost:5434/gameverse_chat".to_string());
    
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Проверяем, выполнены ли уже миграции
    let migration_check = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_name = '_sqlx_migrations'"
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(0);
    
    // Запускаем миграции только если они еще не выполнены
    if migration_check == 0 {
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");
    }
    
    pool
}

async fn create_test_channel(pool: &PgPool) -> ChatChannel {
    let channel = ChatChannel::new(
        "Test Channel".to_string(),
        ChannelType::Group,
        Uuid::new_v4()
    );
    
    let result = sqlx::query_as::<_, ChatChannel>(
        r#"
        INSERT INTO chat_channels (
            id, name, channel_type, created_by, is_private, is_temporary,
            max_participants, settings, created_at, expires_at, telegram_chat_id
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
    .fetch_one(pool)
    .await
    .expect("Failed to create test channel");

    result
}

async fn cleanup_database(pool: &PgPool) {
    // Очищаем тестовые данные
    let _ = sqlx::query("DELETE FROM message_statuses").execute(pool).await;
    let _ = sqlx::query("DELETE FROM telegram_channel_links").execute(pool).await;
    let _ = sqlx::query("DELETE FROM telegram_users").execute(pool).await;
    let _ = sqlx::query("DELETE FROM voice_sessions").execute(pool).await;
    let _ = sqlx::query("DELETE FROM channel_members").execute(pool).await;
    let _ = sqlx::query("DELETE FROM chat_messages").execute(pool).await;
    let _ = sqlx::query("DELETE FROM chat_channels").execute(pool).await;
}

async fn cleanup_redis(_redis_service: &RedisService) {
    // В Redis очистка происходит автоматически через TTL
    // Или можно добавить специальный метод для очистки тестовых данных
} 