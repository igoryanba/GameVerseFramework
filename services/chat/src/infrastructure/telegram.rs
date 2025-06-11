use crate::domain::{ChatError, ChatMessage, MessageType, ChatEvent};
use crate::config::TelegramConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use teloxide::{Bot, types::{ChatId, MessageId}, requests::Requester, payloads::SendMessageSetters};
use tokio::sync::mpsc;
use tracing::{info, error};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TelegramService {
    bot: Bot,
    config: TelegramConfig,
    client: Client,
    event_sender: Option<mpsc::Sender<ChatEvent>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelegramMessage {
    pub chat_id: i64,
    pub message_id: i32,
    pub from_user_id: Option<i64>,
    pub text: Option<String>,
    pub date: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub chat_id: i64,
    pub text: String,
    pub parse_mode: Option<String>,
    pub reply_to_message_id: Option<i32>,
    pub disable_notification: Option<bool>,
}

impl TelegramService {
    pub fn new(
        config: TelegramConfig,
        event_sender: Option<mpsc::Sender<ChatEvent>>,
    ) -> Result<Self, ChatError> {
        if config.bot_token.is_empty() {
            return Err(ChatError::Configuration { 
                message: "Telegram bot token is not configured".to_string() 
            });
        }

        let bot = Bot::new(&config.bot_token);
        let client = Client::new();

        Ok(Self {
            bot,
            config,
            client,
            event_sender,
        })
    }

    /// Отправляет сообщение в Telegram чат
    pub async fn send_message(
        &self,
        chat_id: i64,
        text: String,
        reply_to: Option<i32>,
    ) -> Result<i32, ChatError> {
        if !self.config.enabled {
            return Err(ChatError::Configuration { 
                message: "Telegram integration is disabled".to_string() 
            });
        }

        let mut request = self.bot.send_message(ChatId(chat_id), text);
        
        if let Some(reply_id) = reply_to {
            request = request.reply_to_message_id(MessageId(reply_id));
        }

        match request.await {
            Ok(message) => {
                info!("Message sent to Telegram chat {}: {}", chat_id, message.id.0);
                Ok(message.id.0)
            }
            Err(e) => {
                error!("Failed to send message to Telegram: {}", e);
                Err(ChatError::Telegram(format!("Failed to send message: {}", e)))
            }
        }
    }

    /// Форматирует игровое сообщение для отправки в Telegram
    pub fn format_game_message(&self, message: &ChatMessage, player_name: Option<&str>) -> String {
        let player_name = player_name.unwrap_or("Unknown Player");
        
        let prefix = match message.message_type {
            MessageType::Text => "",
            MessageType::Action => "🎭 ",
            MessageType::Ooc => "💬 [OOC] ",
            MessageType::Radio => "📻 [RADIO] ",
            MessageType::Phone => "📞 [PHONE] ",
            MessageType::Admin => "⚠️ [ADMIN] ",
            MessageType::System => "🔧 [SYSTEM] ",
            MessageType::Dice => "🎲 ",
        };

        let content = if message.message_type == MessageType::Action {
            format!("*{} {}*", player_name, message.content)
        } else {
            format!("**{}**: {}", player_name, message.content)
        };

        format!("{}{}", prefix, content)
    }

    /// Отправляет игровое сообщение в связанный Telegram чат
    pub async fn forward_game_message(
        &self,
        message: &ChatMessage,
        telegram_chat_id: i64,
        player_name: Option<&str>,
    ) -> Result<(), ChatError> {
        let formatted_message = self.format_game_message(message, player_name);
        
        self.send_message(telegram_chat_id, formatted_message, None).await?;
        
        Ok(())
    }

    /// Обрабатывает входящее сообщение из Telegram
    pub async fn handle_telegram_message(
        &self,
        telegram_message: TelegramMessage,
        linked_channel_id: Uuid,
    ) -> Result<(), ChatError> {
        if let Some(text) = telegram_message.text.clone() {
            // Фильтруем команды бота
            if text.starts_with('/') {
                return self.handle_telegram_command(telegram_message, text).await;
            }

            // Создаем игровое сообщение из Telegram
            let game_message = ChatMessage {
                id: Uuid::new_v4(),
                channel_id: linked_channel_id,
                sender_id: Uuid::new_v4(), // TODO: связать с реальным пользователем
                content: format!("📱 {}", text),
                message_type: MessageType::Text,
                reply_to: None,
                forwarded_from: None,
                edited_at: None,
                deleted_at: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                metadata: Some(serde_json::json!({
                    "telegram_user_id": telegram_message.from_user_id,
                    "telegram_message_id": telegram_message.message_id,
                    "telegram_chat_id": telegram_message.chat_id
                })),
            };

            // Отправляем событие о новом сообщении
            if let Some(event_sender) = &self.event_sender {
                let event = ChatEvent::NewMessage(game_message);
                if let Err(e) = event_sender.send(event).await {
                    error!("Failed to send chat event: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Обрабатывает команды Telegram бота
    async fn handle_telegram_command(
        &self,
        telegram_message: TelegramMessage,
        command: String,
    ) -> Result<(), ChatError> {
        let parts: Vec<&str> = command.splitn(2, ' ').collect();
        let cmd = parts[0];
        let args = parts.get(1).unwrap_or(&"");

        let response = match cmd {
            "/start" => {
                "🎮 Добро пожаловать в GameVerse Chat!\n\n\
                Доступные команды:\n\
                /link <код> - Связать аккаунт Telegram с игровым\n\
                /status - Показать статус связи\n\
                /help - Показать эту справку"
            }
            "/help" => {
                "📋 Команды бота:\n\n\
                /start - Начать работу с ботом\n\
                /link <код> - Связать Telegram с игровым аккаунтом\n\
                /unlink - Отвязать аккаунт\n\
                /status - Показать статус связи\n\
                /notify on/off - Включить/выключить уведомления\n\
                /channels - Показать доступные каналы"
            }
            "/link" => {
                if args.is_empty() {
                    "❌ Укажите код привязки: /link <код>\n\
                    Получить код можно в игре командой /telegram"
                } else {
                    // TODO: Обработать привязку аккаунта
                    "✅ Код принят! Проверка привязки..."
                }
            }
            "/status" => {
                // TODO: Показать статус связи с игровым аккаунтом
                "📊 Статус: Аккаунт не привязан\n\
                Используйте /link <код> для привязки"
            }
            _ => "❓ Неизвестная команда. Используйте /help для справки."
        };

        self.send_message(telegram_message.chat_id, response.to_string(), None).await?;
        
        Ok(())
    }

    /// Отправляет уведомление в административный чат
    pub async fn send_admin_notification(&self, message: String) -> Result<(), ChatError> {
        if let Some(admin_chat_id) = &self.config.admin_chat_id {
            if let Ok(chat_id) = admin_chat_id.parse::<i64>() {
                let formatted_message = format!("🔔 **Admin Notification**\n\n{}", message);
                self.send_message(chat_id, formatted_message, None).await?;
            }
        }
        Ok(())
    }

    /// Отправляет уведомление о системном событии
    pub async fn send_system_notification(
        &self,
        chat_id: i64,
        event_type: &str,
        details: &str,
    ) -> Result<(), ChatError> {
        let emoji = match event_type {
            "player_join" => "👋",
            "player_leave" => "👋",
            "server_restart" => "🔄",
            "maintenance" => "🔧",
            "error" => "❌",
            _ => "📢",
        };

        let message = format!("{} **System Notification**\n\n{}", emoji, details);
        self.send_message(chat_id, message, None).await?;
        
        Ok(())
    }

    /// Проверяет доступность Telegram API
    pub async fn check_connection(&self) -> Result<bool, ChatError> {
        match self.bot.get_me().await {
            Ok(user) => {
                info!("Telegram bot connected: @{}", user.username.clone().unwrap_or_default());
                Ok(true)
            }
            Err(e) => {
                error!("Failed to connect to Telegram: {}", e);
                Err(ChatError::Telegram(format!("Connection failed: {}", e)))
            }
        }
    }

    /// Получает информацию о чате
    pub async fn get_chat_info(&self, chat_id: i64) -> Result<serde_json::Value, ChatError> {
        match self.bot.get_chat(ChatId(chat_id)).await {
            Ok(chat) => {
                let info = serde_json::json!({
                    "id": chat.id,
                    "type": format!("{:?}", chat.kind),
                    "title": chat.title(),
                    "username": chat.username(),
                    "description": chat.description(),
                });
                Ok(info)
            }
            Err(e) => {
                error!("Failed to get chat info: {}", e);
                Err(ChatError::Telegram(format!("Failed to get chat info: {}", e)))
            }
        }
    }
}

/// Настройка и запуск Telegram бота
pub async fn setup_telegram_bot(
    config: &TelegramConfig,
    event_sender: mpsc::Sender<ChatEvent>,
) -> Result<Arc<TelegramService>, ChatError> {
    if !config.enabled {
        info!("Telegram integration disabled");
        return Err(ChatError::Configuration { 
            message: "Telegram integration is disabled".to_string() 
        });
    }

    let service = Arc::new(TelegramService::new(config.clone(), Some(event_sender))?);
    
    // Проверяем соединение
    service.check_connection().await?;
    
    info!("Telegram bot service initialized successfully");
    
    Ok(service)
}

/// Типы Telegram сообщений для обработки
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelegramEventType {
    Message,
    EditedMessage,
    CallbackQuery,
    InlineQuery,
    ChatMember,
}

/// Обработчик событий Telegram
pub struct TelegramEventHandler {
    service: Arc<TelegramService>,
}

impl TelegramEventHandler {
    pub fn new(service: Arc<TelegramService>) -> Self {
        Self { service }
    }

    pub async fn handle_event(
        &self,
        event_type: TelegramEventType,
        data: serde_json::Value,
    ) -> Result<(), ChatError> {
        match event_type {
            TelegramEventType::Message => {
                // TODO: Обработать входящее сообщение
                info!("Received Telegram message: {:?}", data);
            }
            TelegramEventType::EditedMessage => {
                // TODO: Обработать редактирование сообщения
                info!("Received edited Telegram message: {:?}", data);
            }
            TelegramEventType::CallbackQuery => {
                // TODO: Обработать callback от inline кнопок
                info!("Received Telegram callback query: {:?}", data);
            }
            _ => {
                info!("Unhandled Telegram event type: {:?}", event_type);
            }
        }
        
        Ok(())
    }
} 