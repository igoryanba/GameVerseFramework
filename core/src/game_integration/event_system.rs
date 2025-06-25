//! # Event System
//!
//! Асинхронная система событий для GameVerse Framework.
//! Превосходство над FiveM: type-safe events, async handlers, better performance.

use anyhow::{Result, bail};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock, mpsc};
use tracing::{debug, info, warn, error};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

/// Система событий
pub struct EventSystem {
    /// Каналы для различных типов событий
    channels: HashMap<String, broadcast::Sender<GameEvent>>,
    /// Обработчики событий
    handlers: Arc<RwLock<HashMap<String, Vec<EventHandler>>>>,
    /// Канал для отправки событий
    event_sender: mpsc::UnboundedSender<GameEvent>,
    /// Канал для получения событий
    event_receiver: Arc<RwLock<Option<mpsc::UnboundedReceiver<GameEvent>>>>,
    /// Состояние системы
    running: Arc<RwLock<bool>>,
}

/// Игровое событие
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    /// Тип события
    pub event_type: String,
    /// Источник события
    pub source: EventSource,
    /// Данные события
    pub data: EventData,
    /// Время создания
    pub timestamp: u64,
    /// ID события
    pub id: String,
}

/// Источник события
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventSource {
    /// Игрок
    Player(u32),
    /// Сервер
    Server,
    /// Система
    System,
    /// Плагин
    Plugin(String),
}

/// Данные события
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventData {
    /// Подключение игрока
    PlayerConnect {
        player_id: u32,
        name: String,
    },
    /// Отключение игрока
    PlayerDisconnect {
        player_id: u32,
        reason: String,
    },
    /// Изменение позиции игрока
    PlayerPositionUpdate {
        player_id: u32,
        x: f32,
        y: f32,
        z: f32,
    },
    /// Чат сообщение
    ChatMessage {
        player_id: u32,
        message: String,
    },
    /// Системное событие
    SystemEvent {
        message: String,
        level: LogLevel,
    },
    /// Пользовательские данные
    Custom {
        data: serde_json::Value,
    },
}

/// Уровень логирования для системных событий
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Обработчик события
pub type EventHandler = Arc<dyn Fn(GameEvent) -> Result<()> + Send + Sync>;

impl EventSystem {
    /// Создать новую систему событий
    pub fn new() -> Self {
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        
        Self {
            channels: HashMap::new(),
            handlers: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            event_receiver: Arc::new(RwLock::new(Some(event_receiver))),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Инициализировать систему событий
    pub async fn initialize(&self) -> Result<()> {
        let mut running = self.running.write().await;
        if *running {
            return Ok(());
        }

        info!("🎯 Initializing event system...");

        // Запускаем обработчик событий
        self.start_event_processor().await?;

        *running = true;
        info!("✅ Event system initialized");

        Ok(())
    }

    /// Запустить обработчик событий
    async fn start_event_processor(&self) -> Result<()> {
        let mut receiver_guard = self.event_receiver.write().await;
        let receiver = receiver_guard.take()
            .ok_or_else(|| anyhow::anyhow!("Event receiver already taken"))?;

        let handlers = self.handlers.clone();
        
        tokio::spawn(async move {
            let mut receiver = receiver;
            
            while let Some(event) = receiver.recv().await {
                if let Err(e) = Self::process_event(event, handlers.clone()).await {
                    error!("Failed to process event: {}", e);
                }
            }
            
            debug!("Event processor stopped");
        });

        Ok(())
    }

    /// Обработать событие
    async fn process_event(
        event: GameEvent,
        handlers: Arc<RwLock<HashMap<String, Vec<EventHandler>>>>
    ) -> Result<()> {
        debug!("Processing event: {} from {:?}", event.event_type, event.source);

        let handlers_map = handlers.read().await;
        
        // Обработчики для конкретного типа события
        if let Some(event_handlers) = handlers_map.get(&event.event_type) {
            for handler in event_handlers {
                if let Err(e) = handler(event.clone()) {
                    error!("Event handler failed for {}: {}", event.event_type, e);
                }
            }
        }

        // Обработчики для всех событий (wildcard)
        if let Some(all_handlers) = handlers_map.get("*") {
            for handler in all_handlers {
                if let Err(e) = handler(event.clone()) {
                    error!("Wildcard event handler failed: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Отправить событие
    pub async fn emit(&self, event: GameEvent) -> Result<()> {
        self.event_sender.send(event)
            .map_err(|e| anyhow::anyhow!("Failed to send event: {}", e))?;
        Ok(())
    }

    /// Отправить событие с автогенерацией ID и времени
    pub async fn emit_event(
        &self,
        event_type: String,
        source: EventSource,
        data: EventData
    ) -> Result<()> {
        let event = GameEvent {
            event_type,
            source,
            data,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            id: uuid::Uuid::new_v4().to_string(),
        };

        self.emit(event).await
    }

    /// Зарегистрировать обработчик события
    pub async fn on<F>(&self, event_type: String, handler: F) -> Result<()>
    where
        F: Fn(GameEvent) -> Result<()> + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.write().await;
        
        handlers
            .entry(event_type.clone())
            .or_insert_with(Vec::new)
            .push(Arc::new(handler));

        debug!("Registered handler for event type: {}", event_type);
        Ok(())
    }

    /// Отправить событие подключения игрока
    pub async fn emit_player_connect(&self, player_id: u32, name: String) -> Result<()> {
        self.emit_event(
            "playerConnect".to_string(),
            EventSource::Player(player_id),
            EventData::PlayerConnect { player_id, name }
        ).await
    }

    /// Отправить событие отключения игрока
    pub async fn emit_player_disconnect(&self, player_id: u32, reason: String) -> Result<()> {
        self.emit_event(
            "playerDisconnect".to_string(),
            EventSource::Player(player_id),
            EventData::PlayerDisconnect { player_id, reason }
        ).await
    }

    /// Отправить событие обновления позиции игрока
    pub async fn emit_player_position_update(&self, player_id: u32, x: f32, y: f32, z: f32) -> Result<()> {
        self.emit_event(
            "playerPositionUpdate".to_string(),
            EventSource::Player(player_id),
            EventData::PlayerPositionUpdate { player_id, x, y, z }
        ).await
    }

    /// Отправить событие чат сообщения
    pub async fn emit_chat_message(&self, player_id: u32, message: String) -> Result<()> {
        self.emit_event(
            "chatMessage".to_string(),
            EventSource::Player(player_id),
            EventData::ChatMessage { player_id, message }
        ).await
    }

    /// Отправить системное событие
    pub async fn emit_system_event(&self, message: String, level: LogLevel) -> Result<()> {
        self.emit_event(
            "systemEvent".to_string(),
            EventSource::System,
            EventData::SystemEvent { message, level }
        ).await
    }

    /// Остановить систему событий
    pub async fn shutdown(&self) -> Result<()> {
        let mut running = self.running.write().await;
        if !*running {
            return Ok(());
        }

        info!("🔌 Shutting down event system...");

        // Закрываем канал отправки событий
        // receiver автоматически завершится
        
        *running = false;
        info!("✅ Event system shut down");

        Ok(())
    }

    /// Проверить, запущена ли система
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}

impl Default for EventSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for EventSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventSystem")
            .field("channels", &self.channels)
            .field("handlers", &"<function handlers>")
            .field("event_sender", &self.event_sender)
            .field("event_receiver", &"<event receiver>")
            .field("running", &self.running)
            .finish()
    }
}

/// Удобные макросы для работы с событиями
#[macro_export]
macro_rules! emit_event {
    ($event_system:expr, $event_type:expr, $source:expr, $data:expr) => {
        $event_system.emit_event($event_type.to_string(), $source, $data).await
    };
}

#[macro_export]
macro_rules! on_event {
    ($event_system:expr, $event_type:expr, $handler:expr) => {
        $event_system.on($event_type.to_string(), $handler).await
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_event_system_basic() {
        let event_system = EventSystem::new();
        event_system.initialize().await.unwrap();

        let received_events = Arc::new(Mutex::new(Vec::<GameEvent>::new()));
        let received_events_clone = received_events.clone();

        event_system.on("test".to_string(), move |event| {
            received_events_clone.lock().unwrap().push(event);
            Ok(())
        }).await.unwrap();

        event_system.emit_event(
            "test".to_string(),
            EventSource::System,
            EventData::Custom { data: serde_json::json!({"test": true}) }
        ).await.unwrap();

        // Даем время на обработку
        sleep(Duration::from_millis(10)).await;

        let received_events = received_events.lock().unwrap();
        assert_eq!(received_events.len(), 1);
        assert_eq!(received_events[0].event_type, "test");
    }
} 