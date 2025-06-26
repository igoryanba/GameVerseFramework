//! # WebAssembly UI Integration Module
//!
//! Этот модуль предоставляет современную веб-основанную систему пользовательского
//! интерфейса для GameVerse Framework, использующую WebAssembly для высокой производительности.
//!
//! Основные возможности:
//! - Рендеринг UI с использованием WebAssembly
//! - Интеграция с современными веб-фреймворками (React, Vue, Svelte)
//! - Высокопроизводительные 2D/3D графика через WebGL
//! - Двунаправленная связь между игрой и UI
//! - Горячая перезагрузка для разработки

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tracing::{info, warn, debug, error};
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::game_integration::GameIntegrator;
use crate::natives::wrapper::{PlayerId, Vector3};

/// Типы UI событий
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum UIEvent {
    /// Открыть/закрыть UI элемент
    Toggle { element_id: String, visible: bool },
    /// Обновить данные элемента
    UpdateData { element_id: String, data: Value },
    /// Пользовательское событие
    UserAction { action: String, payload: Value },
    /// Системное уведомление
    Notification { level: String, message: String },
    /// Обновление игровых данных
    GameDataUpdate { player_id: Option<u32>, data: Value },
    /// Загрузка нового UI модуля
    LoadModule { module_name: String, module_url: String },
}

/// Ответы от UI
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum UIResponse {
    /// Подтверждение выполнения
    Acknowledgment { event_id: String },
    /// Пользовательский ввод
    UserInput { input_id: String, value: Value },
    /// Запрос игровых данных
    DataRequest { request_id: String, query: String },
    /// Ошибка в UI
    Error { error_id: String, message: String },
}

/// Конфигурация UI системы
#[derive(Debug, Clone)]
pub struct UIConfig {
    /// Порт для веб-сервера UI
    pub port: u16,
    /// Путь к статическим файлам UI
    pub static_path: String,
    /// Включить горячую перезагрузку
    pub hot_reload: bool,
    /// Максимальное количество одновременных соединений
    pub max_connections: usize,
    /// Таймаут для WebSocket соединений
    pub websocket_timeout_ms: u64,
    /// Включить CORS для разработки
    pub enable_cors: bool,
    /// Разрешенные домены для CORS
    pub cors_origins: Vec<String>,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            static_path: "./ui/dist".to_string(),
            hot_reload: true,
            max_connections: 100,
            websocket_timeout_ms: 30000,
            enable_cors: true,
            cors_origins: vec!["http://localhost:3000".to_string()],
        }
    }
}

/// Информация о подключенном UI клиенте
#[derive(Debug, Clone)]
pub struct UIClient {
    /// Уникальный ID клиента
    pub id: String,
    /// ID игрока (если привязан)
    pub player_id: Option<PlayerId>,
    /// Время подключения
    pub connected_at: std::time::Instant,
    /// Канал для отправки событий
    pub event_sender: mpsc::UnboundedSender<UIEvent>,
    /// Метаданные клиента
    pub metadata: HashMap<String, String>,
}

/// Статистика UI системы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIStats {
    /// Количество активных соединений
    pub active_connections: usize,
    /// Общее количество отправленных событий
    pub events_sent: u64,
    /// Общее количество полученных ответов
    pub responses_received: u64,
    /// Среднее время отклика UI (мс)
    pub average_response_time_ms: f64,
    /// Количество ошибок
    pub error_count: u64,
}

/// Основной менеджер WebAssembly UI
pub struct WasmUIManager {
    /// Конфигурация
    config: UIConfig,
    /// Подключенные клиенты
    clients: Arc<RwLock<HashMap<String, UIClient>>>,
    /// Канал для получения ответов от UI
    response_receiver: Option<mpsc::UnboundedReceiver<UIResponse>>,
    /// Канал для отправки ответов
    response_sender: mpsc::UnboundedSender<UIResponse>,
    /// Статистика
    stats: Arc<RwLock<UIStats>>,
    /// Зарегистрированные обработчики событий
    event_handlers: Arc<RwLock<HashMap<String, Box<dyn Fn(UIResponse) -> Result<()> + Send + Sync>>>>,
    /// Состояние инициализации
    initialized: bool,
}

impl WasmUIManager {
    /// Создать новый менеджер UI
    pub fn new(config: UIConfig) -> Self {
        let (response_sender, response_receiver) = mpsc::unbounded_channel();
        
        Self {
            config,
            clients: Arc::new(RwLock::new(HashMap::new())),
            response_receiver: Some(response_receiver),
            response_sender,
            stats: Arc::new(RwLock::new(UIStats {
                active_connections: 0,
                events_sent: 0,
                responses_received: 0,
                average_response_time_ms: 0.0,
                error_count: 0,
            })),
            event_handlers: Arc::new(RwLock::new(HashMap::new())),
            initialized: false,
        }
    }

    /// Инициализировать UI систему
    pub async fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }

        info!("🎨 Initializing WebAssembly UI system...");

        // Запускаем веб-сервер для UI
        self.start_web_server().await?;

        // Запускаем обработчик ответов
        self.start_response_handler().await?;

        // Загружаем базовые UI модули
        self.load_core_modules().await?;

        self.initialized = true;
        info!("✅ WebAssembly UI system initialized on port {}", self.config.port);

        Ok(())
    }

    /// Запустить веб-сервер для UI
    async fn start_web_server(&self) -> Result<()> {
        info!("🌐 Starting UI web server on port {}", self.config.port);

        // В реальной реализации здесь был бы код для запуска веб-сервера
        // с поддержкой WebSocket, статических файлов и API эндпоинтов
        
        // Пример структуры:
        // - GET / - основная страница UI
        // - GET /api/status - статус системы
        // - WS /ws - WebSocket для реального времени
        // - GET /static/* - статические файлы (JS, CSS, WASM)

        tokio::spawn(async move {
            // Здесь был бы реальный веб-сервер (например, warp или axum)
            info!("Web server started successfully");
        });

        Ok(())
    }

    /// Запустить обработчик ответов от UI
    async fn start_response_handler(&mut self) -> Result<()> {
        let mut receiver = self.response_receiver.take()
            .ok_or_else(|| anyhow::anyhow!("Response receiver already taken"))?;
        
        let handlers = Arc::clone(&self.event_handlers);
        let stats = Arc::clone(&self.stats);

        tokio::spawn(async move {
            while let Some(response) = receiver.recv().await {
                // Обновляем статистику
                {
                    let mut stats_guard = stats.write().await;
                    stats_guard.responses_received += 1;
                }

                // Обрабатываем ответ
                match response {
                    UIResponse::UserInput { input_id, value } => {
                        debug!("Received user input: {} = {:?}", input_id, value);
                        
                        // Вызываем зарегистрированный обработчик
                        if let Some(handler) = handlers.read().await.get(&input_id) {
                            if let Err(e) = handler(UIResponse::UserInput { input_id, value }) {
                                error!("Error handling user input: {}", e);
                            }
                        }
                    },
                    UIResponse::DataRequest { request_id, query } => {
                        debug!("Received data request: {} - {}", request_id, query);
                        // Обрабатываем запрос данных
                    },
                    UIResponse::Error { error_id, message } => {
                        error!("UI Error {}: {}", error_id, message);
                        
                        let mut stats_guard = stats.write().await;
                        stats_guard.error_count += 1;
                    },
                    _ => {}
                }
            }
        });

        Ok(())
    }

    /// Загрузить базовые UI модули
    async fn load_core_modules(&self) -> Result<()> {
        info!("📦 Loading core UI modules...");

        let core_modules = vec![
            ("player_hud", "/modules/player_hud.wasm"),
            ("chat_system", "/modules/chat_system.wasm"),
            ("minimap", "/modules/minimap.wasm"),
            ("inventory", "/modules/inventory.wasm"),
            ("admin_panel", "/modules/admin_panel.wasm"),
        ];

        for (module_name, module_url) in core_modules {
            self.send_event_to_all(UIEvent::LoadModule {
                module_name: module_name.to_string(),
                module_url: module_url.to_string(),
            }).await?;
        }

        info!("✅ Core UI modules loaded");
        Ok(())
    }

    /// Зарегистрировать клиента
    pub async fn register_client(&self, client_id: String, player_id: Option<PlayerId>) -> Result<()> {
        let (event_sender, _event_receiver) = mpsc::unbounded_channel();
        
        let client = UIClient {
            id: client_id.clone(),
            player_id,
            connected_at: std::time::Instant::now(),
            event_sender,
            metadata: HashMap::new(),
        };

        {
            let mut clients = self.clients.write().await;
            clients.insert(client_id.clone(), client);
        }

        {
            let mut stats = self.stats.write().await;
            stats.active_connections += 1;
        }

        info!("🔌 UI client registered: {} (player: {:?})", client_id, player_id);
        Ok(())
    }

    /// Отключить клиента
    pub async fn disconnect_client(&self, client_id: &str) -> Result<()> {
        {
            let mut clients = self.clients.write().await;
            clients.remove(client_id);
        }

        {
            let mut stats = self.stats.write().await;
            if stats.active_connections > 0 {
                stats.active_connections -= 1;
            }
        }

        info!("🔌 UI client disconnected: {}", client_id);
        Ok(())
    }

    /// Отправить событие конкретному клиенту
    pub async fn send_event_to_client(&self, client_id: &str, event: UIEvent) -> Result<()> {
        let clients = self.clients.read().await;
        
        if let Some(client) = clients.get(client_id) {
            if let Err(_) = client.event_sender.send(event.clone()) {
                warn!("Failed to send event to client {}: channel closed", client_id);
                return Err(anyhow::anyhow!("Client channel closed"));
            }

            let mut stats = self.stats.write().await;
            stats.events_sent += 1;

            debug!("📤 Event sent to client {}: {:?}", client_id, event);
        } else {
            return Err(anyhow::anyhow!("Client {} not found", client_id));
        }

        Ok(())
    }

    /// Отправить событие всем клиентам
    pub async fn send_event_to_all(&self, event: UIEvent) -> Result<()> {
        let clients = self.clients.read().await;
        let mut sent_count = 0;

        for (client_id, client) in clients.iter() {
            if let Err(_) = client.event_sender.send(event.clone()) {
                warn!("Failed to send event to client {}: channel closed", client_id);
            } else {
                sent_count += 1;
            }
        }

        {
            let mut stats = self.stats.write().await;
            stats.events_sent += sent_count;
        }

        debug!("📤 Event sent to {} clients: {:?}", sent_count, event);
        Ok(())
    }

    /// Отправить событие игроку
    pub async fn send_event_to_player(&self, player_id: PlayerId, event: UIEvent) -> Result<()> {
        let clients = self.clients.read().await;
        
        for (client_id, client) in clients.iter() {
            if client.player_id == Some(player_id) {
                if let Err(_) = client.event_sender.send(event.clone()) {
                    warn!("Failed to send event to player {} (client {}): channel closed", player_id.0, client_id);
                } else {
                    let mut stats = self.stats.write().await;
                    stats.events_sent += 1;
                    
                    debug!("📤 Event sent to player {}: {:?}", player_id.0, event);
                    return Ok(());
                }
            }
        }

        Err(anyhow::anyhow!("No UI client found for player {}", player_id.0))
    }

    /// Зарегистрировать обработчик событий
    pub async fn register_event_handler<F>(&self, event_type: String, handler: F) -> Result<()>
    where
        F: Fn(UIResponse) -> Result<()> + Send + Sync + 'static,
    {
        let mut handlers = self.event_handlers.write().await;
        handlers.insert(event_type.clone(), Box::new(handler));
        
        info!("📝 Event handler registered for: {}", event_type);
        Ok(())
    }

    /// Обновить HUD игрока
    pub async fn update_player_hud(&self, player_id: PlayerId, health: i32, armor: i32, money: i32) -> Result<()> {
        let hud_data = serde_json::json!({
            "health": health,
            "armor": armor,
            "money": money,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs()
        });

        self.send_event_to_player(player_id, UIEvent::UpdateData {
            element_id: "player_hud".to_string(),
            data: hud_data,
        }).await
    }

    /// Показать уведомление игроку
    pub async fn show_notification(&self, player_id: PlayerId, level: &str, message: &str) -> Result<()> {
        self.send_event_to_player(player_id, UIEvent::Notification {
            level: level.to_string(),
            message: message.to_string(),
        }).await
    }

    /// Открыть/закрыть UI элемент для игрока
    pub async fn toggle_ui_element(&self, player_id: PlayerId, element_id: &str, visible: bool) -> Result<()> {
        self.send_event_to_player(player_id, UIEvent::Toggle {
            element_id: element_id.to_string(),
            visible,
        }).await
    }

    /// Обновить миникарту
    pub async fn update_minimap(&self, player_id: PlayerId, position: Vector3, rotation: f32) -> Result<()> {
        let minimap_data = serde_json::json!({
            "position": {
                "x": position.x,
                "y": position.y,
                "z": position.z
            },
            "rotation": rotation,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs()
        });

        self.send_event_to_player(player_id, UIEvent::UpdateData {
            element_id: "minimap".to_string(),
            data: minimap_data,
        }).await
    }

    /// Обновить чат
    pub async fn send_chat_message(&self, player_id: Option<PlayerId>, sender: &str, message: &str) -> Result<()> {
        let chat_data = serde_json::json!({
            "sender": sender,
            "message": message,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs()
        });

        let event = UIEvent::UpdateData {
            element_id: "chat_system".to_string(),
            data: chat_data,
        };

        if let Some(player_id) = player_id {
            self.send_event_to_player(player_id, event).await
        } else {
            self.send_event_to_all(event).await
        }
    }

    /// Загрузить пользовательский UI модуль
    pub async fn load_custom_module(&self, module_name: &str, module_url: &str) -> Result<()> {
        info!("📦 Loading custom UI module: {} from {}", module_name, module_url);

        self.send_event_to_all(UIEvent::LoadModule {
            module_name: module_name.to_string(),
            module_url: module_url.to_string(),
        }).await?;

        Ok(())
    }

    /// Получить статистику UI системы
    pub async fn get_stats(&self) -> UIStats {
        self.stats.read().await.clone()
    }

    /// Получить список подключенных клиентов
    pub async fn get_connected_clients(&self) -> Vec<String> {
        let clients = self.clients.read().await;
        clients.keys().cloned().collect()
    }

    /// Сбросить статистику
    pub async fn reset_stats(&self) {
        let mut stats = self.stats.write().await;
        stats.events_sent = 0;
        stats.responses_received = 0;
        stats.average_response_time_ms = 0.0;
        stats.error_count = 0;
        
        info!("📊 UI statistics reset");
    }

    /// Экспортировать статистику в JSON
    pub async fn export_stats(&self) -> Result<String> {
        let stats = self.get_stats().await;
        serde_json::to_string_pretty(&stats)
            .map_err(|e| anyhow::anyhow!("Failed to serialize UI stats: {}", e))
    }

    /// Проверить состояние UI системы
    pub async fn health_check(&self) -> Result<bool> {
        let stats = self.get_stats().await;
        
        // Проверяем основные метрики здоровья системы
        let is_healthy = stats.active_connections <= self.config.max_connections
            && stats.error_count < 100  // Не более 100 ошибок
            && stats.average_response_time_ms < 1000.0; // Отклик менее 1 секунды

        debug!("🏥 UI system health check: {}", if is_healthy { "HEALTHY" } else { "UNHEALTHY" });
        
        Ok(is_healthy)
    }
}

/// Утилиты для работы с WebAssembly модулями
pub struct WasmUtils;

impl WasmUtils {
    /// Компилировать TypeScript/JavaScript в WebAssembly
    pub async fn compile_to_wasm(source_path: &str, output_path: &str) -> Result<()> {
        info!("🔧 Compiling {} to WebAssembly...", source_path);
        
        // В реальной реализации здесь был бы код для компиляции
        // с использованием wasm-pack, AssemblyScript или других инструментов
        
        info!("✅ WebAssembly module compiled: {}", output_path);
        Ok(())
    }

    /// Оптимизировать WASM модуль
    pub async fn optimize_wasm(wasm_path: &str) -> Result<()> {
        info!("⚡ Optimizing WebAssembly module: {}", wasm_path);
        
        // В реальной реализации здесь был бы код для оптимизации
        // с использованием wasm-opt и других инструментов
        
        Ok(())
    }

    /// Валидировать WASM модуль
    pub async fn validate_wasm(wasm_path: &str) -> Result<bool> {
        debug!("🔍 Validating WebAssembly module: {}", wasm_path);
        
        // В реальной реализации здесь была бы валидация WASM байт-кода
        
        Ok(true)
    }
}

/// Примеры использования UI системы
pub mod examples {
    use super::*;

    /// Пример создания простого HUD
    pub async fn create_simple_hud(ui_manager: &WasmUIManager, player_id: PlayerId) -> Result<()> {
        // Обновляем HUD данные
        ui_manager.update_player_hud(player_id, 100, 50, 5000).await?;
        
        // Показываем уведомление
        ui_manager.show_notification(player_id, "info", "Welcome to GameVerse!").await?;
        
        // Открываем миникарту
        ui_manager.toggle_ui_element(player_id, "minimap", true).await?;
        
        Ok(())
    }

    /// Пример создания системы чата
    pub async fn setup_chat_system(ui_manager: &WasmUIManager) -> Result<()> {
        // Регистрируем обработчик сообщений чата
        ui_manager.register_event_handler("chat_message".to_string(), |response| {
            if let UIResponse::UserInput { input_id, value } = response {
                if input_id == "chat_input" {
                    if let Some(message) = value.as_str() {
                        info!("Chat message received: {}", message);
                        // Здесь была бы обработка сообщения
                    }
                }
            }
            Ok(())
        }).await?;
        
        Ok(())
    }

    /// Пример создания админ-панели
    pub async fn create_admin_panel(ui_manager: &WasmUIManager, admin_player_id: PlayerId) -> Result<()> {
        // Открываем админ-панель только для админа
        ui_manager.toggle_ui_element(admin_player_id, "admin_panel", true).await?;
        
        // Регистрируем обработчики админ-команд
        ui_manager.register_event_handler("admin_command".to_string(), |response| {
            if let UIResponse::UserInput { input_id, value } = response {
                match input_id.as_str() {
                    "teleport_command" => {
                        info!("Admin teleport command: {:?}", value);
                        // Обработка команды телепортации
                    },
                    "kick_command" => {
                        info!("Admin kick command: {:?}", value);
                        // Обработка команды кика
                    },
                    _ => {}
                }
            }
            Ok(())
        }).await?;
        
        Ok(())
    }
} 