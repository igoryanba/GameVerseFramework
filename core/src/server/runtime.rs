//! # Серверный рантайм GameVerse
//!
//! Этот модуль отвечает за запуск и управление жизненным циклом сервера,
//! включая инициализацию всех компонентов, обработку сигналов и корректное завершение.

use std::path::{Path, PathBuf};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use tokio::sync::mpsc;
use anyhow::{Result, Context};
use tracing::{info, error, warn, debug};
use std::time::{Instant};
use serde_json;

use crate::config::{self, Config};
use crate::engine::GameEngine;
use crate::resource::ResourceManager;
use crate::net::NetworkManager;
use crate::logging;

/// Команды управления сервером
#[derive(Debug, Clone)]
pub enum ServerCommand {
    /// Остановить сервер
    Shutdown,
    /// Перезапустить сервер
    Restart,
    /// Перезагрузить ресурсы
    ReloadResources,
    /// Запросить статус сервера
    RequestStatus,
}

/// Статус сервера
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerStatus {
    /// Сервер запускается
    Starting,
    /// Сервер работает
    Running,
    /// Сервер останавливается
    Stopping,
    /// Сервер остановлен
    Stopped,
    /// Ошибка сервера
    Error,
}

/// Серверный рантайм
pub struct ServerRuntime {
    /// Конфигурация сервера
    config: Config,
    /// Игровой движок
    engine: GameEngine,
    /// Менеджер ресурсов
    resource_manager: ResourceManager,
    /// Сетевой менеджер
    network_manager: NetworkManager,
    /// Текущий статус сервера
    status: ServerStatus,
    /// Флаг работы сервера
    running: Arc<AtomicBool>,
    /// Канал для отправки команд серверу
    command_tx: mpsc::Sender<ServerCommand>,
    /// Канал для получения команд сервером
    command_rx: mpsc::Receiver<ServerCommand>,
    /// Режим разработчика
    dev_mode: bool,
    /// Путь к IPC сокету (только Unix)
    #[cfg(unix)]
    ipc_path: PathBuf,
    /// Время старта сервера
    start_time: Instant,
}

impl ServerRuntime {
    /// Создает новый экземпляр серверного рантайма
    pub fn new(config_path: Option<&Path>, dev_mode: bool) -> Result<Self> {
        // Загружаем конфигурацию
        let config_path_str = config_path.map(|p| p.to_string_lossy().to_string());
        let config = config::load_config(config_path_str.as_deref())
            .context("Failed to load server configuration")?;
        
        // Инициализируем логирование с конфигурацией из загруженного конфига
        let log_config = config.to_log_config();
        logging::initialize_with_config(log_config)
            .context("Failed to initialize logging")?;
        
        // Создаем канал для команд
        let (command_tx, command_rx) = mpsc::channel(100);
        
        // Создаем компоненты сервера
        let engine = GameEngine::new();
        let resource_manager = ResourceManager::new(config.server.resources_path.clone().into());
        let network_manager = NetworkManager::new();
        
        Ok(Self {
            config,
            engine,
            resource_manager,
            network_manager,
            status: ServerStatus::Stopped,
            running: Arc::new(AtomicBool::new(false)),
            command_tx,
            command_rx,
            dev_mode,
            #[cfg(unix)]
            ipc_path: PathBuf::from("/tmp/gameverse_server.sock"),
            start_time: Instant::now(),
        })
    }
    
    /// Запускает сервер и блокирует текущий поток до завершения
    pub async fn start(&mut self) -> Result<()> {
        self.status = ServerStatus::Starting;
        info!("Starting GameVerse server '{}'...", self.config.server.name);
        
        if self.dev_mode {
            info!("🔧 Running in DEVELOPMENT mode");
        }
        
        // Инициализируем компоненты
        self.initialize_components().await?;
        
        // Запускаем IPC-слушатель (Unix)
        #[cfg(unix)]
        self.spawn_ipc_listener()?;
        
        // Запускаем IPC-слушатель на Windows (Named Pipe)
        #[cfg(windows)]
        self.spawn_ipc_listener()?;
        
        // Устанавливаем флаг работы
        self.running.store(true, Ordering::SeqCst);
        self.status = ServerStatus::Running;
        
        info!("🚀 GameVerse server started successfully");
        info!("Server is listening on {}:{}", 
            self.config.server.bind_address, 
            self.config.server.port
        );
        
        // Основной цикл сервера
        self.run_server_loop().await?;
        
        // Завершение работы
        self.shutdown().await?;
        
        Ok(())
    }
    
    /// Инициализирует все компоненты сервера
    async fn initialize_components(&mut self) -> Result<()> {
        // Инициализируем игровой движок
        debug!("Initializing game engine...");
        self.engine.initialize().await
            .context("Failed to initialize game engine")?;
        
        // Инициализируем менеджер ресурсов
        debug!("Initializing resource manager...");
        self.resource_manager.initialize().await
            .context("Failed to initialize resource manager")?;
        
        // Инициализируем сетевой стек
        debug!("Initializing network stack...");
        self.network_manager.initialize().await
            .context("Failed to initialize network manager")?;
        
        Ok(())
    }
    
    /// Основной цикл сервера
    async fn run_server_loop(&mut self) -> Result<()> {
        info!("Server is running. Press Ctrl+C to stop.");
        
        // Создаем обработчик Ctrl+C
        let running = self.running.clone();
        tokio::spawn(async move {
            match tokio::signal::ctrl_c().await {
                Ok(()) => {
                    info!("Shutdown signal received");
                    running.store(false, Ordering::SeqCst);
                },
                Err(err) => {
                    error!("Failed to listen for shutdown signal: {}", err);
                }
            }
        });
        
        // Основной цикл обработки команд
        while self.running.load(Ordering::SeqCst) {
            tokio::select! {
                Some(cmd) = self.command_rx.recv() => {
                    self.handle_command(cmd).await?;
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                    // Периодическая проверка состояния
                }
            }
        }
        
        Ok(())
    }
    
    /// Обрабатывает команду управления сервером
    async fn handle_command(&mut self, command: ServerCommand) -> Result<()> {
        match command {
            ServerCommand::Shutdown => {
                info!("Received shutdown command");
                self.running.store(false, Ordering::SeqCst);
            }
            ServerCommand::Restart => {
                info!("Received restart command");
                // TODO: Реализовать перезапуск сервера
                warn!("Server restart not yet implemented");
            }
            ServerCommand::ReloadResources => {
                info!("Received reload resources command");
                // Избегаем конфликтующих заимствований: копируем имена ресурсов во временный вектор
                let resource_names: Vec<String> = self
                    .resource_manager
                    .list_resources()
                    .iter()
                    .map(|res| res.name.clone())
                    .collect();

                for name in resource_names {
                    if let Err(e) = self.resource_manager.reload_resource(&name).await {
                        error!("Failed to reload resource {}: {}", name, e);
                    }
                }
            }
            ServerCommand::RequestStatus => {
                info!("Server status: {:?}", self.status);
                // TODO: Отправить подробный статус через канал ответа
            }
        }
        
        Ok(())
    }
    
    /// Корректно завершает работу сервера
    async fn shutdown(&mut self) -> Result<()> {
        self.status = ServerStatus::Stopping;
        info!("Shutting down GameVerse server...");
        
        // Аналогично: собираем имена ресурсов, затем вызываем stop_resource
        let resource_names: Vec<String> = self
            .resource_manager
            .list_resources()
            .iter()
            .map(|res| res.name.clone())
            .collect();

        for name in resource_names {
            if let Err(e) = self.resource_manager.stop_resource(&name).await {
                error!("Failed to stop resource {}: {}", name, e);
            }
        }
        
        // Закрываем все сетевые соединения
        // TODO: Реализовать корректное закрытие соединений
        
        self.status = ServerStatus::Stopped;
        info!("Server shutdown complete");
        
        Ok(())
    }
    
    /// Возвращает текущий статус сервера
    pub fn status(&self) -> ServerStatus {
        self.status.clone()
    }
    
    /// Возвращает отправителя команд для управления сервером извне
    pub fn command_sender(&self) -> mpsc::Sender<ServerCommand> {
        self.command_tx.clone()
    }
    
    /// Проверяет, запущен ли сервер
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    #[cfg(unix)]
    fn spawn_ipc_listener(&self) -> Result<()> {
        use tokio::net::UnixListener;
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        // Удаляем старый сокет, если существует
        if self.ipc_path.exists() {
            std::fs::remove_file(&self.ipc_path)?;
        }

        let listener = UnixListener::bind(&self.ipc_path)?;
        info!("IPC listener started at {}", self.ipc_path.display());

        // Клонируем канал для передачи команд
        let cmd_tx = self.command_tx.clone();
        let running_flag = self.running.clone();

        // Для динамических метрик клонируем необходимые менеджеры
        let resource_manager = self.resource_manager.clone();
        let network_manager = self.network_manager.clone();

        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((mut stream, _addr)) => {
                        let mut buf = Vec::new();
                        if let Ok(_n) = stream.read_to_end(&mut buf).await {
                            if buf.is_empty() {
                                continue;
                            }
                            let msg = String::from_utf8_lossy(&buf);
                            let cmd_str = msg.trim().to_lowercase();
                            let cmd_opt = match cmd_str.as_str() {
                                "shutdown" => Some(ServerCommand::Shutdown),
                                "reload" | "reload_resources" => Some(ServerCommand::ReloadResources),
                                "status" => Some(ServerCommand::RequestStatus),
                                _ => None,
                            };
                            if let Some(ref c) = cmd_opt {
                                let _ = cmd_tx.send(c.clone()).await;
                            }

                            // Формируем быстрый ответ
                            let reply = match cmd_opt {
                                Some(ServerCommand::RequestStatus) => {
                                    let uptime_secs = self.start_time.elapsed().as_secs();
                                    let status_json = serde_json::json!({
                                        "status": if running_flag.load(Ordering::SeqCst) {"running"} else {"stopped"},
                                        "uptime_secs": uptime_secs,
                                        "players": network_manager.get_connection_stats().total(),
                                        "resources": resource_manager.list_resources().len()
                                    });
                                    status_json.to_string()
                                }
                                Some(_) => "ok".to_string(),
                                None => "unknown".to_string(),
                            };
                            let _ = stream.write_all(reply.as_bytes()).await;
                        }
                    }
                    Err(e) => {
                        error!("IPC accept error: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// Запускает IPC-слушатель на Windows (Named Pipe)
    #[cfg(windows)]
    fn spawn_ipc_listener(&self) -> Result<()> {
        use tokio::net::windows::named_pipe::{ServerOptions, NamedPipeServer};
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        // Имя пайпа
        const PIPE_NAME: &str = r"\\.\pipe\gameverse_server";

        // Создаём серверный пайп (first instance)
        let server: NamedPipeServer = ServerOptions::new()
            .first_pipe_instance(true)
            .create(PIPE_NAME)?;

        tracing::info!("IPC listener started at {} (named pipe)", PIPE_NAME);

        // Клонируем каналы/флаги/менеджеры для обработчика
        let cmd_tx = self.command_tx.clone();
        let running_flag = self.running.clone();
        let resource_manager = self.resource_manager.clone();
        let network_manager = self.network_manager.clone();
        let start_time = self.start_time;

        tokio::spawn(async move {
            // Обрабатываем каждое подключение последовательно
            let mut server = server;
            loop {
                // Ожидаем подключения клиента
                match server.connect().await {
                    Ok(()) => {
                        let mut buf = Vec::new();
                        if let Ok(_n) = server.read_to_end(&mut buf).await {
                            if buf.is_empty() {
                                let _ = server.disconnect();
                                continue;
                            }
                            let msg = String::from_utf8_lossy(&buf);
                            let cmd_str = msg.trim().to_lowercase();
                            let cmd_opt = match cmd_str.as_str() {
                                "shutdown" => Some(ServerCommand::Shutdown),
                                "reload" | "reload_resources" => Some(ServerCommand::ReloadResources),
                                "status" => Some(ServerCommand::RequestStatus),
                                _ => None,
                            };

                            if let Some(ref c) = cmd_opt {
                                let _ = cmd_tx.send(c.clone()).await;
                            }

                            // Сформировать ответ
                            let reply = match cmd_opt {
                                Some(ServerCommand::RequestStatus) => {
                                    let uptime_secs = start_time.elapsed().as_secs();
                                    let status_json = serde_json::json!({
                                        "status": if running_flag.load(std::sync::atomic::Ordering::SeqCst) {"running"} else {"stopped"},
                                        "uptime_secs": uptime_secs,
                                        "players": network_manager.get_connection_stats().total(),
                                        "resources": resource_manager.list_resources().len()
                                    });
                                    status_json.to_string()
                                }
                                Some(_) => "ok".to_string(),
                                None => "unknown".to_string(),
                            };

                            let _ = server.write_all(reply.as_bytes()).await;
                        }

                        // Отключаемся и переходим к ожиданию следующего клиента
                        let _ = server.disconnect();
                    }
                    Err(e) => {
                        tracing::error!("IPC accept error (named pipe): {}", e);
                        // Без sleep loop перегрузит CPU. Ждём немного
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                }
            }
        });

        Ok(())
    }
}

#[cfg(test)]
mod runtime_tests; 