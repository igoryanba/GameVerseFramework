// TODO: Реализация WebSocket сервера для real-time коммуникации 

use std::sync::Arc;
use tokio::sync::{broadcast, RwLock, mpsc};
use axum::{
    extract::{
        ws::{WebSocket, Message},
        WebSocketUpgrade, State, Query,
    },
    response::Response,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use tracing::{info, warn, error};

use crate::domain::{ChatEvent, GamePosition, VoiceRange, VoiceSessionType};
use crate::application::services::{ChatService, VoiceService};

// ============================================================================
// WebSocket Сервер для Real-Time коммуникации
// ============================================================================

/// Основная структура WebSocket сервера
#[derive(Clone)]
pub struct WebSocketServer {
    /// Активные WebSocket соединения
    connections: Arc<RwLock<HashMap<Uuid, Connection>>>,
    /// Broadcast канал для отправки событий всем клиентам
    broadcast_tx: broadcast::Sender<WebSocketEvent>,
    /// Сервисы
    chat_service: Arc<ChatService>,
    voice_service: Arc<VoiceService>,
}

/// Информация о WebSocket соединении
#[derive(Debug, Clone)]
pub struct Connection {
    pub user_id: Uuid,
    pub character_name: Option<String>,
    pub position: Option<GamePosition>,
    pub voice_range: VoiceRange,
    pub channel_ids: Vec<Uuid>,
    pub connected_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub is_admin: bool,
}

/// События WebSocket для real-time коммуникации
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketEvent {
    // Сообщения чата
    NewMessage {
        message_id: Uuid,
        channel_id: Uuid,
        sender_id: Uuid,
        sender_name: String,
        content: String,
        message_type: String,
        timestamp: DateTime<Utc>,
        position: Option<GamePosition>,
    },
    MessageEdited {
        message_id: Uuid,
        channel_id: Uuid,
        new_content: String,
        edited_at: DateTime<Utc>,
    },
    MessageDeleted {
        message_id: Uuid,
        channel_id: Uuid,
        deleted_by: Uuid,
    },
    
    // Typing индикаторы
    UserTyping {
        user_id: Uuid,
        username: String,
        channel_id: Uuid,
    },
    UserStoppedTyping {
        user_id: Uuid,
        channel_id: Uuid,
    },
    
    // Голосовые сессии и proximity
    VoiceSessionStarted {
        session_id: Uuid,
        user_id: Uuid,
        username: String,
        session_type: VoiceSessionType,
        channel_id: Option<Uuid>,
        voice_range: VoiceRange,
        position: Option<GamePosition>,
    },
    VoiceSessionEnded {
        session_id: Uuid,
        user_id: Uuid,
    },
    VoicePositionUpdate {
        user_id: Uuid,
        session_id: Uuid,
        position: GamePosition,
        voice_range: VoiceRange,
    },
    
    // Proximity события
    PlayerEnterRange {
        player_id: Uuid,
        player_name: String,
        distance: f64,
        voice_range: VoiceRange,
    },
    PlayerLeaveRange {
        player_id: Uuid,
    },
    PlayerMoved {
        player_id: Uuid,
        position: GamePosition,
        distance: f64,
    },
    
    // Каналы и участники
    UserJoinedChannel {
        user_id: Uuid,
        username: String,
        channel_id: Uuid,
        channel_name: String,
    },
    UserLeftChannel {
        user_id: Uuid,
        username: String,
        channel_id: Uuid,
    },
    ChannelCreated {
        channel_id: Uuid,
        channel_name: String,
        channel_type: String,
        created_by: Uuid,
    },
    ChannelUpdated {
        channel_id: Uuid,
        name: String,
        settings: serde_json::Value,
    },
    
    // Системные события
    ServerMessage {
        message: String,
        level: String, // info, warning, error
        timestamp: DateTime<Utc>,
    },
    Heartbeat {
        timestamp: DateTime<Utc>,
        server_time: DateTime<Utc>,
    },
    
    // Административные события
    AdminBroadcast {
        message: String,
        admin_name: String,
        timestamp: DateTime<Utc>,
    },
    UserModerationAction {
        target_user_id: Uuid,
        action: String, // mute, kick, ban
        reason: String,
        duration: Option<u32>,
        admin_id: Uuid,
    },
}

/// Входящие команды от клиентов
#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClientCommand {
    // Аутентификация
    Authenticate {
        token: String,
        character_name: Option<String>,
    },
    
    // Позиция и proximity
    UpdatePosition {
        position: GamePosition,
        voice_range: Option<VoiceRange>,
    },
    
    // Typing индикаторы
    StartTyping {
        channel_id: Uuid,
    },
    StopTyping {
        channel_id: Uuid,
    },
    
    // Управление каналами
    JoinChannel {
        channel_id: Uuid,
    },
    LeaveChannel {
        channel_id: Uuid,
    },
    
    // Голосовые команды
    StartVoiceSession {
        session_type: VoiceSessionType,
        channel_id: Option<Uuid>,
        voice_range: VoiceRange,
    },
    EndVoiceSession,
    MuteVoice {
        muted: bool,
    },
    
    // Heartbeat
    Ping,
    
    // Административные команды
    AdminCommand {
        command: String,
        target: Option<Uuid>,
        params: Option<serde_json::Value>,
    },
}

/// Параметры подключения WebSocket
#[derive(Debug, Deserialize)]
pub struct WsConnectParams {
    pub token: Option<String>,
    pub character_name: Option<String>,
}

impl WebSocketServer {
    /// Создает новый WebSocket сервер
    pub fn new(
        chat_service: Arc<ChatService>,
        voice_service: Arc<VoiceService>,
    ) -> Self {
        let (broadcast_tx, _) = broadcast::channel(1000);
        
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            broadcast_tx,
            chat_service,
            voice_service,
        }
    }
    
    /// Обработчик WebSocket подключений
    pub async fn handle_websocket(
        State(server): State<Arc<WebSocketServer>>,
        Query(params): Query<WsConnectParams>,
        ws: WebSocketUpgrade,
    ) -> Response {
        ws.on_upgrade(move |socket| {
            server.handle_socket(socket, params)
        })
    }
    
    /// Обработка WebSocket соединения
    async fn handle_socket(
        self: Arc<Self>,
        socket: WebSocket,
        params: WsConnectParams,
    ) {
        let (mut sender, mut receiver) = socket.split();
        let connection_id = Uuid::new_v4();
        
        // Канал для отправки сообщений в WebSocket
        let (tx, mut rx) = mpsc::unbounded_channel::<WebSocketEvent>();
        
        // Подписываемся на broadcast события
        let mut broadcast_rx = self.broadcast_tx.subscribe();
        
        // Задача для отправки broadcast событий - клонируем tx
        let tx_clone = tx.clone();
        let broadcast_task = tokio::spawn(async move {
            while let Ok(event) = broadcast_rx.recv().await {
                if let Err(_) = tx_clone.send(event) {
                    break;
                }
            }
        });
        
        // Задача для отправки сообщений в WebSocket
        let send_task = tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                let message = match serde_json::to_string(&event) {
                    Ok(json) => Message::Text(json),
                    Err(e) => {
                        error!("Failed to serialize WebSocket event: {}", e);
                        continue;
                    }
                };
                
                if sender.send(message).await.is_err() {
                    break;
                }
            }
        });
        
        // Основная обработка входящих сообщений
        let mut user_id: Option<Uuid> = None;
        let mut last_heartbeat = Utc::now();
        
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str::<ClientCommand>(&text) {
                        Ok(command) => {
                            match self.handle_client_command(
                                connection_id,
                                command,
                                &mut user_id,
                                &params,
                            ).await {
                                Ok(response) => {
                                    if let Some(response) = response {
                                        let _ = tx.send(response);
                                    }
                                },
                                Err(e) => {
                                    warn!("Command error: {}", e);
                                    let error_event = WebSocketEvent::ServerMessage {
                                        message: format!("Error: {}", e),
                                        level: "error".to_string(),
                                        timestamp: Utc::now(),
                                    };
                                    let _ = tx.send(error_event);
                                }
                            }
                            last_heartbeat = Utc::now();
                        },
                        Err(e) => {
                            warn!("Failed to parse client command: {}", e);
                        }
                    }
                },
                Ok(Message::Binary(_)) => {
                    // Можно использовать для передачи аудиоданных в будущем
                },
                Ok(Message::Ping(_data)) => {
                    let _ = tx.send(WebSocketEvent::Heartbeat {
                        timestamp: Utc::now(),
                        server_time: Utc::now(),
                    });
                },
                Ok(Message::Close(_)) => {
                    info!("WebSocket connection closed");
                    break;
                },
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
            
            // Проверка timeout
            if Utc::now().timestamp() - last_heartbeat.timestamp() > 30 {
                warn!("WebSocket connection timeout");
                break;
            }
        }
        
        // Очистка при отключении
        if let Some(uid) = user_id {
            self.cleanup_connection(connection_id, uid).await;
        }
        
        broadcast_task.abort();
        send_task.abort();
    }
    
    /// Обработка команд от клиента
    async fn handle_client_command(
        &self,
        connection_id: Uuid,
        command: ClientCommand,
        user_id: &mut Option<Uuid>,
        _params: &WsConnectParams,
    ) -> Result<Option<WebSocketEvent>, String> {
        match command {
            ClientCommand::Authenticate { token: _token, character_name } => {
                // TODO: Валидация токена
                let uid = Uuid::new_v4(); // В реальности получаем из токена
                *user_id = Some(uid);
                
                let connection = Connection {
                    user_id: uid,
                    character_name,
                    position: None,
                    voice_range: VoiceRange::Normal,
                    channel_ids: Vec::new(),
                    connected_at: Utc::now(),
                    last_activity: Utc::now(),
                    is_admin: false, // TODO: Проверка прав
                };
                
                self.connections.write().await.insert(connection_id, connection);
                
                Ok(Some(WebSocketEvent::ServerMessage {
                    message: "Successfully authenticated".to_string(),
                    level: "info".to_string(),
                    timestamp: Utc::now(),
                }))
            },
            
            ClientCommand::UpdatePosition { position, voice_range } => {
                if let Some(uid) = user_id {
                    if let Some(connection) = self.connections.write().await.get_mut(&connection_id) {
                        let old_position = connection.position.clone();
                        connection.position = Some(position.clone());
                        
                        if let Some(vr) = voice_range {
                            connection.voice_range = vr;
                        }
                        
                        // Проверяем proximity с другими игроками
                        self.check_proximity_changes(*uid, &position, old_position).await;
                        
                        // Уведомляем о движении
                        let event = WebSocketEvent::PlayerMoved {
                            player_id: *uid,
                            position,
                            distance: 0.0, // Будет вычислено для каждого получателя
                        };
                        
                        let _ = self.broadcast_tx.send(event);
                    }
                }
                Ok(None)
            },
            
            ClientCommand::StartTyping { channel_id } => {
                if let Some(uid) = user_id {
                    if let Some(connection) = self.connections.read().await.get(&connection_id) {
                        let event = WebSocketEvent::UserTyping {
                            user_id: *uid,
                            username: connection.character_name.clone()
                                .unwrap_or("Unknown".to_string()),
                            channel_id,
                        };
                        
                        let _ = self.broadcast_tx.send(event);
                    }
                }
                Ok(None)
            },
            
            ClientCommand::StopTyping { channel_id } => {
                if let Some(uid) = user_id {
                    let event = WebSocketEvent::UserStoppedTyping {
                        user_id: *uid,
                        channel_id,
                    };
                    
                    let _ = self.broadcast_tx.send(event);
                }
                Ok(None)
            },
            
            ClientCommand::Ping => {
                Ok(Some(WebSocketEvent::Heartbeat {
                    timestamp: Utc::now(),
                    server_time: Utc::now(),
                }))
            },
            
            _ => {
                Ok(Some(WebSocketEvent::ServerMessage {
                    message: "Command not implemented yet".to_string(),
                    level: "warning".to_string(),
                    timestamp: Utc::now(),
                }))
            }
        }
    }
    
    /// Проверка изменений proximity между игроками
    async fn check_proximity_changes(
        &self,
        user_id: Uuid,
        new_position: &GamePosition,
        old_position: Option<GamePosition>,
    ) {
        let connections = self.connections.read().await;
        
        for (_, connection) in connections.iter() {
            if connection.user_id == user_id {
                continue;
            }
            
            if let Some(other_pos) = &connection.position {
                let distance = calculate_distance(new_position, other_pos);
                let voice_range = connection.voice_range.distance();
                
                // Проверяем, нужно ли отправить уведомление о входе/выходе из зоны
                let was_in_range = if let Some(old_pos) = &old_position {
                    calculate_distance(old_pos, other_pos) <= voice_range
                } else {
                    false
                };
                
                let is_in_range = distance <= voice_range;
                
                if !was_in_range && is_in_range {
                    // Игрок вошел в зону слышимости
                    let event = WebSocketEvent::PlayerEnterRange {
                        player_id: user_id,
                        player_name: connection.character_name.clone()
                            .unwrap_or("Unknown".to_string()),
                        distance,
                        voice_range: connection.voice_range.clone(),
                    };
                    let _ = self.broadcast_tx.send(event);
                } else if was_in_range && !is_in_range {
                    // Игрок вышел из зоны слышимости
                    let event = WebSocketEvent::PlayerLeaveRange {
                        player_id: user_id,
                    };
                    let _ = self.broadcast_tx.send(event);
                }
            }
        }
    }
    
    /// Очистка соединения при отключении
    async fn cleanup_connection(&self, connection_id: Uuid, user_id: Uuid) {
        self.connections.write().await.remove(&connection_id);
        
        // Уведомляем о том, что пользователь отключился
        let event = WebSocketEvent::ServerMessage {
            message: format!("User {} disconnected", user_id),
            level: "info".to_string(),
            timestamp: Utc::now(),
        };
        
        let _ = self.broadcast_tx.send(event);
    }
    
    /// Отправка события всем подключенным клиентам
    pub async fn broadcast_event(&self, event: WebSocketEvent) {
        let _ = self.broadcast_tx.send(event);
    }
    
    /// Отправка события конкретным пользователям
    pub async fn send_to_users(&self, _user_ids: Vec<Uuid>, event: WebSocketEvent) {
        // TODO: Реализовать таргетированную отправку
        let _ = self.broadcast_tx.send(event);
    }
    
    /// Получение количества активных соединений
    pub async fn get_connection_count(&self) -> usize {
        self.connections.read().await.len()
    }
    
    /// Получение списка активных пользователей
    pub async fn get_active_users(&self) -> Vec<(Uuid, String, Option<GamePosition>)> {
        self.connections
            .read()
            .await
            .values()
            .map(|conn| (
                conn.user_id,
                conn.character_name.clone().unwrap_or("Unknown".to_string()),
                conn.position.clone(),
            ))
            .collect()
    }
}

/// Вычисление расстояния между двумя точками в 3D
fn calculate_distance(pos1: &GamePosition, pos2: &GamePosition) -> f64 {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    let dz = pos1.z - pos2.z;
    
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Интеграция с основными сервисами для событий
impl WebSocketServer {
    /// Обработка событий чата
    pub async fn handle_chat_event(&self, event: ChatEvent) {
        let ws_event = match event {
            ChatEvent::NewMessage(message) => {
                WebSocketEvent::NewMessage {
                    message_id: message.id,
                    channel_id: message.channel_id,
                    sender_id: message.sender_id,
                    sender_name: "Unknown".to_string(), // TODO: Получить имя из сервиса
                    content: message.content,
                    message_type: format!("{:?}", message.message_type),
                    timestamp: message.created_at,
                    position: None, // TODO: Получить позицию отправителя
                }
            },
            ChatEvent::MessageEdited(message) => {
                WebSocketEvent::MessageEdited {
                    message_id: message.id,
                    channel_id: message.channel_id,
                    new_content: message.content,
                    edited_at: message.edited_at.unwrap_or(message.updated_at),
                }
            },
            ChatEvent::MessageDeleted { message_id, channel_id } => {
                WebSocketEvent::MessageDeleted {
                    message_id,
                    channel_id,
                    deleted_by: Uuid::new_v4(), // TODO: Получить ID удалившего
                }
            },
            _ => return, // Другие события обрабатываются отдельно
        };
        
        self.broadcast_event(ws_event).await;
    }
} 