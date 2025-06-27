//! # Серверный рантайм GameVerse
//!
//! Этот модуль отвечает за запуск и управление жизненным циклом сервера,
//! включая инициализацию всех компонентов, обработку сигналов и корректное завершение.

use std::path::{Path, PathBuf};
use std::sync::{Arc, atomic::{AtomicBool, Ordering, AtomicU64}};
use tokio::sync::mpsc;
use anyhow::{Result, Context};
use tracing::{info, error, warn, debug};
use std::time::{Instant};
use serde_json;
use sysinfo::{System, RefreshKind, ProcessRefreshKind, Pid};
use once_cell::sync::Lazy;
use prometheus::{Encoder, TextEncoder, IntGauge, Gauge, register_int_gauge, register_gauge};

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
    /// Список ресурсов (ответ в IPC)
    ListResources,
    /// Запустить конкретный ресурс
    StartResource(String),
    /// Остановить конкретный ресурс
    StopResource(String),
    /// Перезагрузить (hot-reload) конкретный ресурс
    ReloadResource(String),
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
    /// Счётчик тиков основного цикла
    tick_counter: Arc<AtomicU64>,
}

#[derive(Clone)]
struct ApiState {
    running: Arc<AtomicBool>,
    resource_manager: ResourceManager,
    network_manager: NetworkManager,
    start_time: Instant,
    tick_counter: Arc<AtomicU64>,
    cmd_tx: mpsc::Sender<ServerCommand>,
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
            tick_counter: Arc::new(AtomicU64::new(0)),
        })
    }
    
    /// Запускает сервер и блокирует текущий поток до завершения
    pub async fn start(&mut self) -> Result<()> {
        self.status = ServerStatus::Starting;
        
        // Инициализируем канал для логов SSE
        init_log_channel();
        
        // Настраиваем tracing subscriber с нашим layer
        let subscriber = tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(LogBroadcastLayer);
        if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
            // Если уже установлен, игнорируем ошибку
            if e.to_string().contains("already been set") {
                warn!("Tracing subscriber already set, skipping: {}", e);
            } else {
                return Err(anyhow::anyhow!("Failed to set tracing subscriber: {}", e));
            }
        }
        
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
        
        // Запускаем REST Admin API
        if let Err(e)= self.spawn_admin_api().await {
            warn!("Admin API failed to start: {}", e);
        }
        
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
        // Инициализируем игровой движок (можно пропустить в dev-режиме)
        if self.dev_mode {
            debug!("[DEV] Skipping game engine initialization");
        } else {
            debug!("Initializing game engine...");
            self.engine.initialize().await
                .context("Failed to initialize game engine")?;
        }
        
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
                    self.tick_counter.fetch_add(1, Ordering::Relaxed);
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
            ServerCommand::ListResources => {
                let list = self.resource_manager.list_resources();
                info!("Resources loaded: {}", list.len());
                for res in list {
                    info!("  - {} ({:?})", res.name, res.state);
                }
            }
            ServerCommand::StartResource(name) => {
                if let Err(e) = self.resource_manager.start_resource(&name).await {
                    error!("Failed to start resource {}: {}", name, e);
                }
            }
            ServerCommand::StopResource(name) => {
                if let Err(e) = self.resource_manager.stop_resource(&name).await {
                    error!("Failed to stop resource {}: {}", name, e);
                }
            }
            ServerCommand::ReloadResource(name) => {
                if let Err(e) = self.resource_manager.reload_resource(&name).await {
                    error!("Failed to reload resource {}: {}", name, e);
                }
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
        let start_time = self.start_time;
        let tick_counter = self.tick_counter.clone();

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
                            let tokens: Vec<&str> = cmd_str.split_whitespace().collect();
                            let cmd_opt: Option<ServerCommand> = match tokens.as_slice() {
                                ["shutdown"] => Some(ServerCommand::Shutdown),
                                ["reload"] | ["reload_resources"] => Some(ServerCommand::ReloadResources),
                                ["status"] => Some(ServerCommand::RequestStatus),
                                ["resource", "list"] => Some(ServerCommand::ListResources),
                                ["resource", "start", name] => Some(ServerCommand::StartResource((*name).to_string())),
                                ["resource", "stop", name] => Some(ServerCommand::StopResource((*name).to_string())),
                                ["resource", "reload", name] => Some(ServerCommand::ReloadResource((*name).to_string())),
                                _ => None,
                            };
                            if let Some(ref c) = cmd_opt {
                                let _ = cmd_tx.send(c.clone()).await;
                            }

                            // Формируем быстрый ответ
                            let reply = match cmd_opt {
                                Some(ServerCommand::RequestStatus) => {
                                    let uptime_secs = start_time.elapsed().as_secs_f64();
                                    let ticks = tick_counter.load(Ordering::Relaxed).max(1);
                                    let avg_tick_ms = (uptime_secs * 1000.0) / ticks as f64;
                                    let status_json = serde_json::json!({
                                        "status": if running_flag.load(Ordering::SeqCst) {"running"} else {"stopped"},
                                        "uptime_secs": uptime_secs as u64,
                                        "players": network_manager.get_connection_stats().total(),
                                        "resources": resource_manager.list_resources().len(),
                                        "avg_tick_ms": avg_tick_ms
                                    });
                                    status_json.to_string()
                                }
                                Some(ServerCommand::ListResources) => {
                                    let list = resource_manager.list_resources();
                                    let arr: Vec<serde_json::Value> = list.iter().map(|r| {
                                        serde_json::json!({
                                            "name": r.name,
                                            "state": format!("{:?}", r.state)
                                        })
                                    }).collect();
                                    serde_json::json!({ "resources": arr }).to_string()
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
        let tick_counter = self.tick_counter.clone();

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
                            let tokens: Vec<&str> = cmd_str.split_whitespace().collect();
                            let cmd_opt: Option<ServerCommand> = match tokens.as_slice() {
                                ["shutdown"] => Some(ServerCommand::Shutdown),
                                ["reload"] | ["reload_resources"] => Some(ServerCommand::ReloadResources),
                                ["status"] => Some(ServerCommand::RequestStatus),
                                ["resource", "list"] => Some(ServerCommand::ListResources),
                                ["resource", "start", name] => Some(ServerCommand::StartResource((*name).to_string())),
                                ["resource", "stop", name] => Some(ServerCommand::StopResource((*name).to_string())),
                                ["resource", "reload", name] => Some(ServerCommand::ReloadResource((*name).to_string())),
                                _ => None,
                            };

                            if let Some(ref c) = cmd_opt {
                                let _ = cmd_tx.send(c.clone()).await;
                            }

                            // Сформировать ответ
                            let reply = match cmd_opt {
                                Some(ServerCommand::RequestStatus) => {
                                    let uptime_secs = start_time.elapsed().as_secs_f64();
                                    let ticks = tick_counter.load(Ordering::Relaxed).max(1);
                                    let avg_tick_ms = (uptime_secs * 1000.0) / ticks as f64;
                                    let status_json = serde_json::json!({
                                        "status": if running_flag.load(Ordering::SeqCst) {"running"} else {"stopped"},
                                        "uptime_secs": uptime_secs as u64,
                                        "players": network_manager.get_connection_stats().total(),
                                        "resources": resource_manager.list_resources().len(),
                                        "avg_tick_ms": avg_tick_ms
                                    });
                                    status_json.to_string()
                                }
                                Some(ServerCommand::ListResources) => {
                                    let list = resource_manager.list_resources();
                                    let arr: Vec<serde_json::Value> = list.iter().map(|r| {
                                        serde_json::json!({
                                            "name": r.name,
                                            "state": format!("{:?}", r.state)
                                        })
                                    }).collect();
                                    serde_json::json!({ "resources": arr }).to_string()
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

    /// Запускает REST Admin API (Axum) на `config.server.admin_port`
    async fn spawn_admin_api(&self) -> Result<()> {
        use axum::{Router, routing::{get, post}, Json};
        use axum::extract::State;
        use std::net::SocketAddr;
        use serde_json::json;
        use axum::{middleware, http::StatusCode};
        use axum::extract::Path as AxumPath;

        let state = ApiState {
            running: self.running.clone(),
            resource_manager: self.resource_manager.clone(),
            network_manager: self.network_manager.clone(),
            start_time: self.start_time,
            tick_counter: self.tick_counter.clone(),
            cmd_tx: self.command_tx.clone(),
        };

        async fn status_handler(State(_st): State<ApiState>) -> Json<serde_json::Value> {
            let uptime_secs = _st.start_time.elapsed().as_secs_f64();
            let ticks = _st.tick_counter.load(Ordering::Relaxed).max(1);
            let avg_tick_ms = (uptime_secs * 1000.0) / ticks as f64;

            Json(json!({
                "status": if _st.running.load(Ordering::SeqCst) {"running"} else {"stopped"},
                "uptime_secs": uptime_secs as u64,
                "players": _st.network_manager.get_connection_stats().total(),
                "resources": _st.resource_manager.list_resources().len(),
                "avg_tick_ms": avg_tick_ms
            }))
        }

        async fn shutdown_handler(State(st): State<ApiState>) -> Json<serde_json::Value> {
            let _ = st.cmd_tx.send(ServerCommand::Shutdown).await;
            Json(json!({"result":"ok"}))
        }

        async fn reload_handler(State(st): State<ApiState>) -> Json<serde_json::Value> {
            let _ = st.cmd_tx.send(ServerCommand::ReloadResources).await;
            Json(json!({"result":"ok"}))
        }

        // Extended metrics endpoint
        async fn metrics_handler(State(_st): State<ApiState>) -> Json<serde_json::Value> {
            use sysinfo::{SystemExt, ProcessExt};

            let mut sys = sysinfo::System::new();
            sys.refresh_processes_specifics(sysinfo::ProcessRefreshKind::new());
            let rss_mb = sysinfo::get_current_pid()
                .ok()
                .and_then(|pid| sys.process(pid))
                .map(|p| p.memory() / 1024)
                .unwrap_or(0);

            let uptime_secs = _st.start_time.elapsed().as_secs_f64();
            let ticks = _st.tick_counter.load(Ordering::Relaxed).max(1);
            let avg_tick_ms = (uptime_secs * 1000.0) / ticks as f64;

            Json(json!({
                "uptime_secs": uptime_secs as u64,
                "avg_tick_ms": avg_tick_ms,
                "rss_mb": rss_mb
            }))
        }

        // Prometheus exposition format
        async fn prometheus_metrics_handler(State(_st): State<ApiState>) -> (StatusCode, ([(axum::http::header::HeaderName, axum::http::HeaderValue); 1], String)) {
             // update gauges
             let uptime = _st.start_time.elapsed().as_secs() as i64;
             UPTIME_GAUGE.set(uptime);
             PLAYERS_GAUGE.set(_st.network_manager.get_connection_stats().total() as i64);
             let ticks = _st.tick_counter.load(Ordering::Relaxed).max(1);
             let avg_tick_ms = ((_st.start_time.elapsed().as_secs_f64() * 1000.0) / ticks as f64);
             TICK_GAUGE.set(avg_tick_ms);

             let mut buffer = Vec::with_capacity(1024);
             let encoder = TextEncoder::new();
             let mf = prometheus::gather();
             encoder.encode(&mf, &mut buffer).unwrap();
             let body = String::from_utf8(buffer).unwrap();

             let headers = [(
                 axum::http::header::CONTENT_TYPE,
                 axum::http::HeaderValue::from_static("text/plain; version=0.0.4"),
             )];
             (StatusCode::OK, (headers, body))
        }

        // List resources
        async fn resources_list_handler(State(_st): State<ApiState>) -> Json<serde_json::Value> {
            let list: Vec<serde_json::Value> = _st.resource_manager.list_resources()
                .iter()
                .map(|r| serde_json::json!({"name": r.name, "state": format!("{:?}", r.state)}))
                .collect();
            Json(json!({"resources": list}))
        }

        // Reload single resource
        async fn resource_reload_handler(AxumPath(name): AxumPath<String>, State(st): State<ApiState>) -> Json<serde_json::Value> {
            let _ = st.cmd_tx.send(ServerCommand::ReloadResource(name.clone())).await;
            Json(json!({"result":"ok", "resource": name}))
        }

        // Update JWT middleware with signature validation
        async fn require_jwt(
            req: axum::http::Request<axum::body::Body>,
            next: middleware::Next,
        ) -> Result<axum::response::Response, StatusCode> {
            if cfg!(debug_assertions) {
                return Ok(next.run(req).await);
            }
            let secret = std::env::var("ADMIN_JWT_SECRET").unwrap_or_else(|_| "changeme".into());
            let token_opt = req.headers().get("authorization").and_then(|v| v.to_str().ok()).and_then(|s| {
                if s.starts_with("Bearer ") { Some(&s[7..]) } else { None }
            });
            if let Some(token) = token_opt {
                #[derive(Debug, serde::Deserialize)]
                struct Claims { exp: Option<u64> }
                let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
                validation.validate_exp = false; // we check manually
                if let Ok(token_data) = jsonwebtoken::decode::<Claims>(token, &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()), &validation) {
                    if let Some(exp_ts) = token_data.claims.exp {
                        let now = chrono::Utc::now().timestamp() as u64;
                        if exp_ts < now { return Err(StatusCode::UNAUTHORIZED); }
                    }
                    return Ok(next.run(req).await);
                }
            }
            Err(StatusCode::UNAUTHORIZED)
        }

        let protected = Router::new()
            .route("/api/server/shutdown", post(shutdown_handler))
            .route("/api/server/reload", post(reload_handler))
            .route("/api/server/logs/stream", get(logs_stream))
            .route("/api/resources", get(resources_list_handler))
            .route("/api/resources/:name/reload", post(resource_reload_handler))
            .layer(middleware::from_fn(require_jwt));

        let router = Router::new()
            .route("/api/server/status", get(status_handler))
            .route("/api/server/metrics", get(metrics_handler))
            .route("/api/health", get(health_handler))
            .route("/api/server/metrics/stream", get(metrics_stream_handler))
            .route("/metrics", get(prometheus_metrics_handler))
            .merge(protected)
            .with_state(state);

        let addr = SocketAddr::from(([0,0,0,0], self.config.network.admin_port));
        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            if let Err(e) = axum::serve(listener, router).await {
                error!("Admin API server error: {}", e);
            }
        });

        info!("Admin API listening on http://{}", addr);
        Ok(())
    }

    pub fn parse_command(cmd: &str) -> Option<ServerCommand> {
        match cmd.trim().to_lowercase().as_str() {
            "shutdown" => Some(ServerCommand::Shutdown),
            "restart" => Some(ServerCommand::Restart),
            "reload" | "reload_resources" => Some(ServerCommand::ReloadResources),
            "status" => Some(ServerCommand::RequestStatus),
            cmd if cmd.starts_with("resource list") => Some(ServerCommand::ListResources),
            cmd if cmd.starts_with("resource start ") => {
                Some(ServerCommand::StartResource(cmd[15..].to_string()))
            }
            cmd if cmd.starts_with("resource stop ") => {
                Some(ServerCommand::StopResource(cmd[14..].to_string()))
            }
            cmd if cmd.starts_with("resource reload ") => {
                Some(ServerCommand::ReloadResource(cmd[16..].to_string()))
            }
            _ => None,
        }
    }
}

use axum::response::sse::{Sse, Event};
use std::convert::Infallible;
use tokio_stream::wrappers::BroadcastStream;
use tokio::sync::broadcast;
use futures_core::Stream;
use futures_util::stream::StreamExt as FuturesStreamExt;
use std::sync::OnceLock;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

// Глобальный канал для логов
static LOG_SENDER: OnceLock<broadcast::Sender<String>> = OnceLock::new();

/// Инициализирует глобальный лог-канал
pub fn init_log_channel() {
    let (tx, _) = broadcast::channel(1000);
    LOG_SENDER.set(tx).ok();
}

/// Получает receiver для логов
pub fn get_log_receiver() -> Option<broadcast::Receiver<String>> {
    LOG_SENDER.get().map(|tx| tx.subscribe())
}

/// Кастомный Layer для отправки логов в broadcast канал
struct LogBroadcastLayer;

impl<S> Layer<S> for LogBroadcastLayer 
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if let Some(sender) = LOG_SENDER.get() {
            let mut visitor = LogVisitor::new();
            event.record(&mut visitor);
            let level = event.metadata().level();
            let target = event.metadata().target();
            let timestamp = chrono::Utc::now().format("%H:%M:%S%.3f");
            let log_line = format!("[{}] {} {}: {}", timestamp, level, target, visitor.message);
            let _ = sender.send(log_line);
        }
    }
}

struct LogVisitor {
    message: String,
}

impl LogVisitor {
    fn new() -> Self {
        Self { message: String::new() }
    }
}

impl tracing::field::Visit for LogVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }
}

async fn logs_stream() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // Создаем поток логов
    let stream = if let Some(rx) = get_log_receiver() {
        // Реальные логи из broadcast канала
        FuturesStreamExt::boxed(
            FuturesStreamExt::filter_map(BroadcastStream::new(rx), |msg| {
                futures_util::future::ready(match msg {
                    Ok(log_line) => Some(Ok(Event::default().data(log_line))),
                    Err(_) => None,
                })
            })
        )
    } else {
        // Fallback поток
        FuturesStreamExt::boxed(
            tokio_stream::once(Ok(Event::default().data("Log channel not initialized")))
        )
    };
    
    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}

async fn health_handler() -> (axum::http::StatusCode, &'static str) {
    (axum::http::StatusCode::OK, "ok")
}

// SSE metrics stream
async fn metrics_stream_handler(axum::extract::State(_st): axum::extract::State<ApiState>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    use tokio_stream::wrappers::IntervalStream;
    use tokio::time::{interval, Duration};

    // Interval every second
    let stream = IntervalStream::new(interval(Duration::from_secs(1))).map(move |_| {
        let uptime_secs = _st.start_time.elapsed().as_secs();
        let ticks = _st.tick_counter.load(Ordering::Relaxed).max(1);
        let avg_tick_ms = (uptime_secs * 1000) / ticks;
        let players = _st.network_manager.get_connection_stats().total();

        let payload = serde_json::json!({
            "uptime_secs": uptime_secs,
            "players": players,
            "avg_tick_ms": avg_tick_ms
        }).to_string();

        Ok(Event::default().data(payload))
    });
    Sse::new(stream)
}

// Prometheus metrics
static UPTIME_GAUGE: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!("server_uptime_seconds", "Server uptime in seconds").unwrap()
});

static PLAYERS_GAUGE: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!("players_connected_total", "Total connected players").unwrap()
});

static TICK_GAUGE: Lazy<Gauge> = Lazy::new(|| {
    register_gauge!("avg_tick_ms", "Average tick duration (ms)").unwrap()
}); 