//! Server management commands

use clap::Subcommand;
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::fs;
use std::io::{self, Write};
#[cfg(unix)]
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[cfg(windows)]
use tokio::net::windows::named_pipe::ClientOptions;

use crate::config::Config;
use tracing::{info, error, warn};
use serde_json;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use eventsource_stream::Eventsource;
use futures_util::{StreamExt, TryStreamExt};

use gameverse_core::config as core_config;

#[derive(Subcommand)]
pub enum ServerCommands {
    /// Start GameVerse server
    Start {
        /// Development mode
        #[arg(long)]
        dev: bool,
        /// Configuration file
        #[arg(short, long)]
        config: Option<String>,
        /// Generate JWT token for admin API access
        #[arg(long)]
        token: bool,
    },
    
    /// Stop GameVerse server
    Stop {
        /// Force stop
        #[arg(long)]
        force: bool,
    },
    
    /// Restart GameVerse server
    Restart {
        /// Development mode
        #[arg(long)]
        dev: bool,
    },
    
    /// Show server status
    Status,
    
    /// Show server logs
    Logs {
        /// Follow logs
        #[arg(short, long)]
        follow: bool,
        /// Number of lines to show
        #[arg(short, long, default_value = "100")]
        lines: usize,
    },
    
    /// Reload all resources on running server
    Reload,
    
    /// Attach to live stdout/stderr (console) of running server
    Console,
    
    /// Generate JWT token for admin API access
    Token,
    
    /// Validate server configuration file
    ValidateConfig {
        /// Path to config file (optional)
        #[arg(short, long)]
        config: Option<String>,
    },
    
    /// Resource management subcommands
    Resource {
        #[command(subcommand)]
        action: ResourceAction,
    },
    
    /// Initialize a new GameVerse server folder
    Init {
        /// Target directory (will be created if not exists)
        folder: String,
    },
}

/// Subcommands for resource manipulation
#[derive(Subcommand)]
pub enum ResourceAction {
    /// List all resources loaded on the server
    List,
    /// Start a resource by name
    Start {
        /// Resource name
        name: String,
    },
    /// Stop a resource by name
    Stop {
        /// Resource name
        name: String,
    },
    /// Reload (hot) a resource by name
    Reload {
        /// Resource name
        name: String,
    },
    /// Watch filesystem and auto-reload on changes
    Watch,
}

// Путь к серверному бинарнику
fn get_server_binary_path() -> Result<PathBuf> {
    // Сначала проверяем в текущей директории
    let local_path = Path::new("target/release/gameverse_server");
    if local_path.exists() {
        return Ok(local_path.to_path_buf());
    }
    
    // Затем проверяем в PATH
    if let Ok(path) = which::which("gameverse_server") {
        return Ok(path);
    }
    
    // Если не нашли, возвращаем ошибку
    Err(anyhow::anyhow!("Server binary not found. Please build the project first with 'cargo build --release'"))
}

// Путь к сокету управления сервером
fn get_server_socket_path() -> PathBuf {
    if cfg!(unix) {
        PathBuf::from("/tmp/gameverse_server.sock")
    } else {
        PathBuf::from(r"\\.\pipe\gameverse_server")
    }
}

// Путь к PID файлу
fn get_server_pid_path() -> PathBuf {
    if cfg!(unix) {
        PathBuf::from("/tmp/gameverse_server.pid")
    } else {
        PathBuf::from(r"C:\ProgramData\GameVerse\server.pid")
    }
}

// Проверка, запущен ли сервер
fn is_server_running() -> bool {
    let pid_path = get_server_pid_path();
    
    if !pid_path.exists() {
        return false;
    }
    
    match fs::read_to_string(&pid_path) {
        Ok(pid_str) => {
            if let Ok(pid) = pid_str.trim().parse::<u32>() {
                if cfg!(unix) {
                    // На Unix проверяем наличие процесса
                    Command::new("kill")
                        .arg("-0")
                        .arg(pid.to_string())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status()
                        .map(|status| status.success())
                        .unwrap_or(false)
                } else {
                    // На Windows используем tasklist
                    Command::new("tasklist")
                        .args(["/FI", &format!("PID eq {}", pid)])
                        .stdout(Stdio::piped())
                        .stderr(Stdio::null())
                        .status()
                        .map(|status| status.success())
                        .unwrap_or(false)
                }
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

// Получение PID сервера
fn get_server_pid() -> Option<u32> {
    let pid_path = get_server_pid_path();
    
    if !pid_path.exists() {
        return None;
    }
    
    match fs::read_to_string(&pid_path) {
        Ok(pid_str) => pid_str.trim().parse::<u32>().ok(),
        Err(_) => None,
    }
}

#[cfg(unix)]
async fn send_ipc_command(cmd: &str) -> Result<String> {
    let socket_path = get_server_socket_path();
    let mut stream = UnixStream::connect(&socket_path)
        .await
        .context("Failed to connect to IPC socket")?;

    stream.write_all(cmd.as_bytes()).await?;
    stream.shutdown().await?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

#[cfg(not(unix))]
async fn send_ipc_command(_cmd: &str) -> Result<String> {
    const PIPE_NAME: &str = r"\\.\pipe\gameverse_server";

    // TODO: рассмотреть Timeout/retry
    let mut client = ClientOptions::new()
        .open(PIPE_NAME)
        .context("Failed to connect to named pipe")?;

    client.write_all(_cmd.as_bytes()).await?;
    client.flush().await?;

    let mut buf = Vec::new();
    client.read_to_end(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn generate_and_print_jwt() -> Result<()> {
    let secret = b"devsecret";
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let claims = Claims {
        sub: "gameverse-cli".to_string(),
        exp: (now + 3600 * 24) as usize, // 24 часа
    };
    
    let header = Header::new(Algorithm::HS256);
    let token = encode(&header, &claims, &EncodingKey::from_secret(secret))
        .map_err(|e| anyhow::anyhow!("JWT generation failed: {}", e))?;
    
    println!("🔑 JWT Token (valid for 24h):");
    println!("{}", token);
    println!();
    println!("Usage: curl -H 'Authorization: Bearer {}' http://localhost:30121/api/server/status", token);
    
    Ok(())
}

pub async fn execute(cmd: ServerCommands, _config: &Config) -> Result<()> {
    match cmd {
        ServerCommands::Start { dev, config, token } => {
            if is_server_running() {
                println!("🔄 GameVerse server is already running");
                return Ok(());
            }
            
            println!("🚀 Starting GameVerse server...");
            
            // Получаем путь к серверному бинарнику
            let server_bin = get_server_binary_path()?;
            
            // Формируем команду запуска
            let mut command = Command::new(server_bin);
            
            // Добавляем аргументы
            if dev {
                command.arg("--dev");
                println!("Mode: Development");
            }
            
            if let Some(config_file) = config {
                command.arg(&config_file);
                println!("Config: {}", config_file);
            } else {
                // Проверяем наличие конфигурационного файла
                let default_config = Path::new("config/server.toml");
                if default_config.exists() {
                    command.arg(default_config);
                    println!("Config: {}", default_config.display());
                }
            }
            
            if token {
                generate_and_print_jwt()?;
            }
            
            // Запускаем сервер в фоновом режиме
            #[cfg(unix)]
            {
                command.stdout(Stdio::piped())
                       .stderr(Stdio::piped());
                
                let child = command.spawn()
                    .context("Failed to start server process")?;
                
                // Записываем PID в файл
                let pid = child.id();
                fs::write(get_server_pid_path(), pid.to_string())
                    .context("Failed to write PID file")?;
                
                println!("✅ Server started with PID: {}", pid);
            }
            
            #[cfg(windows)]
            {
                command.creation_flags(0x00000008) // CREATE_NO_WINDOW
                       .stdout(Stdio::piped())
                       .stderr(Stdio::piped());
                
                let child = command.spawn()
                    .context("Failed to start server process")?;
                
                // Записываем PID в файл
                let pid = child.id();
                
                // Создаем директорию если не существует
                let pid_path = get_server_pid_path();
                if let Some(parent) = pid_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                
                fs::write(pid_path, pid.to_string())
                    .context("Failed to write PID file")?;
                
                println!("✅ Server started with PID: {}", pid);
            }
        }
        
        ServerCommands::Stop { force } => {
            if !is_server_running() {
                println!("❌ GameVerse server is not running");
                return Ok(());
            }
            
            println!("🛑 Stopping GameVerse server...");
            
            if let Some(pid) = get_server_pid() {
            if force {
                    println!("Force stopping server...");
                    
                    #[cfg(unix)]
                    {
                        Command::new("kill")
                            .arg("-9")
                            .arg(pid.to_string())
                            .status()
                            .context("Failed to kill server process")?;
            }
                    
                    #[cfg(windows)]
                    {
                        Command::new("taskkill")
                            .args(["/F", "/PID", &pid.to_string()])
                            .status()
                            .context("Failed to kill server process")?;
                    }
                } else {
                    // Отправляем команду остановки через сокет
                    // TODO: Реализовать отправку команды через сокет
                    // Пока просто отправляем сигнал
                    
                    #[cfg(unix)]
                    {
                        Command::new("kill")
                            .arg("-15") // SIGTERM
                            .arg(pid.to_string())
                            .status()
                            .context("Failed to send termination signal to server")?;
                    }
                    
                    #[cfg(windows)]
                    {
                        Command::new("taskkill")
                            .args(["/PID", &pid.to_string()])
                            .status()
                            .context("Failed to terminate server process")?;
                    }
                }
                
                // Удаляем PID файл
                let _ = fs::remove_file(get_server_pid_path());
                
                println!("✅ Server stopped");
            } else {
                println!("⚠️ Could not determine server PID");
                return Err(anyhow::anyhow!("Could not determine server PID"));
            }
        }
        
        ServerCommands::Restart { dev } => {
            println!("🔄 Restarting GameVerse server...");
            
            // Сначала останавливаем сервер (не используем force)
            Box::pin(execute(ServerCommands::Stop { force: false }, _config)).await?;
            
            // Даем серверу корректно завершиться
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            
            // Затем запускаем снова
            Box::pin(execute(ServerCommands::Start { dev, config: None, token: false }, _config)).await?;
        }
        
        ServerCommands::Status => {
            println!("📊 GameVerse server status:");
            
            if !is_server_running() {
                println!("❌ Server is not running");
                return Ok(());
            }

            // Запрашиваем статус через IPC
            match send_ipc_command("status").await {
                Ok(resp) => {
                    if let Ok(val)= serde_json::from_str::<serde_json::Value>(&resp) {
                        println!("✅ Status: {}", val["status"]);
                        println!("Uptime: {} s", val["uptime_secs"]);
                        println!("Players: {}", val["players"]);
                        println!("Resources: {}", val["resources"]);
                        println!("Avg tick: {} ms", val["avg_tick_ms"]);
                        println!("Memory RSS: {} MB", val["mem_rss_mb"]);
                    } else {
                        println!("✅ Server response: {}", resp);
                    }
                }
                Err(e) => {
                    println!("⚠️ Failed to get status via IPC: {}", e);
                    // Fallback к простому PID
                    if let Some(pid) = get_server_pid() {
                        println!("Server PID: {} (IPC offline)", pid);
                    }
                }
            }
        }
        
        ServerCommands::Logs { follow, lines } => {
            println!("📋 GameVerse server logs (last {} lines):", lines);
            
            // Проверяем наличие директории логов
            let log_path = Path::new("logs/gameverse.log");
            
            if !log_path.exists() {
                println!("❌ Log file not found at {}", log_path.display());
                return Err(anyhow::anyhow!("Log file not found"));
            }
            
            // Читаем логи
            if follow {
                println!("Following logs... (Press Ctrl+C to stop)");
                
                #[cfg(unix)]
                {
                    Command::new("tail")
                        .args(["-f", "-n", &lines.to_string(), &log_path.to_string_lossy()])
                        .status()
                        .context("Failed to follow logs")?;
                }
                
                #[cfg(windows)]
                {
                    // На Windows используем PowerShell для эмуляции tail -f
                    Command::new("powershell")
                        .args([
                            "-Command", 
                            &format!("Get-Content -Path '{}' -Tail {} -Wait", 
                                    log_path.to_string_lossy(), 
                                    lines)
                        ])
                        .status()
                        .context("Failed to follow logs")?;
                }
            } else {
                // Просто показываем последние строки
                
                #[cfg(unix)]
                {
                    Command::new("tail")
                        .args(["-n", &lines.to_string(), &log_path.to_string_lossy()])
                        .status()
                        .context("Failed to display logs")?;
                }
                
                #[cfg(windows)]
                {
                    // На Windows используем PowerShell
                    Command::new("powershell")
                        .args([
                            "-Command", 
                            &format!("Get-Content -Path '{}' -Tail {}", 
                                    log_path.to_string_lossy(), 
                                    lines)
                        ])
                        .status()
                        .context("Failed to display logs")?;
                }
            }
        }
        
        ServerCommands::Reload => {
            // Реализация команды Reload
            println!("🔄 Reloading all resources on running server...");
            
            // Вызываем send_ipc_command для отправки команды на сервер
            let result = send_ipc_command("reload").await?;
            
            println!("🎉 Reload completed. Result: {}", result);
        }
        
        ServerCommands::Console => {
            println!("🖥️  Attaching to GameVerse server console via SSE...");

            // По умолчанию admin_port 30121
            let port = 30121;
            let url = format!("http://127.0.0.1:{}/api/server/logs/stream", port);

            let client = reqwest::Client::new();
            let response = client.get(&url).send().await?;

            if !response.status().is_success() {
                eprintln!("Console endpoint not available: {}", response.status());
                return Ok(());
            }

            // Читаем весь ответ как поток байтов
            let body = response.text().await?;
            println!("Console output:\n{}", body);
            return Ok(());
        }
        
        ServerCommands::Token => {
            generate_and_print_jwt()?;
        }
        
        ServerCommands::ValidateConfig { config } => {
            // Реализация команды ValidateConfig
            println!("🔄 Validating server configuration file...");
            
            // Проверяем наличие конфигурационного файла
            let config_path: Option<&str> = config.as_deref();
            match tokio::fs::read_to_string(config_path.unwrap_or("server-config.toml")).await {
                Ok(_) => {
                    match core_config::load_config(config_path) {
                        Ok(cfg) => {
                            println!("✅ Configuration is valid (server name: '{}', port: {})", cfg.server.name, cfg.server.port);
                        }
                        Err(e) => {
                            println!("❌ Configuration invalid: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Failed to read config file: {}", e);
                }
            }
        }
        
        ServerCommands::Resource { action } => {
            // Реализация команды Resource
            match action {
                ResourceAction::List => {
                    println!("🔄 Listing all resources loaded on the server...");
                    
                    // Вызываем send_ipc_command для получения списка ресурсов
                    let result = send_ipc_command("resource list").await?;
                    
                    println!("🎉 Resources: {}", result);
                }
                ResourceAction::Start { name } => {
                    println!("🚀 Starting resource: {}", name);
                    
                    // Вызываем send_ipc_command для запуска ресурса
                    let result = send_ipc_command(&format!("resource start {}", name)).await?;
                    
                    println!("🎉 Resource started. Result: {}", result);
                }
                ResourceAction::Stop { name } => {
                    println!("🛑 Stopping resource: {}", name);
                    
                    // Вызываем send_ipc_command для остановки ресурса
                    let result = send_ipc_command(&format!("resource stop {}", name)).await?;
                    
                    println!("🎉 Resource stopped. Result: {}", result);
                }
                ResourceAction::Reload { name } => {
                    println!("🔄 Reloading resource: {}", name);
                    
                    // Вызываем send_ipc_command для перезагрузки ресурса
                    let result = send_ipc_command(&format!("resource reload {}", name)).await?;
                    
                    println!("🎉 Resource reloaded. Result: {}", result);
                }
                ResourceAction::Watch => {
                    println!("🔄 Watching resources directory for changes (Ctrl+C to stop)...");
                    use notify::{RecommendedWatcher, Watcher, EventKind, RecursiveMode};
                    use tokio::sync::mpsc as tokio_mpsc;
                    use std::sync::mpsc as std_mpsc;

                    // Канал между notify (sync) и async контекстом
                    let (tx, rx) = std_mpsc::channel();
                    let mut watcher: RecommendedWatcher = RecommendedWatcher::new(tx, notify::Config::default())?;
                    watcher.watch(Path::new("resources"), RecursiveMode::Recursive)?;

                    // Переносим события в async контекст
                    let (async_tx, mut async_rx) = tokio_mpsc::channel::<notify::Event>(100);

                    std::thread::spawn(move || {
                        while let Ok(event_result) = rx.recv() {
                            if let Ok(event) = event_result {
                                let _ = async_tx.blocking_send(event);
                            }
                        }
                    });

                    while let Some(event) = async_rx.recv().await {
                        if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)) {
                            // Пытаемся угадать имя ресурса из пути
                            if let Some(path) = event.paths.get(0) {
                                if let Some(res_dir) = path.ancestors().find(|p| p.parent() == Some(Path::new("resources"))) {
                                    if let Some(name_os) = res_dir.file_name() {
                                        if let Some(name) = name_os.to_str() {
                                            let _ = send_ipc_command(&format!("resource reload {}", name)).await;
                                            println!("♻️  Hot-reloaded resource '{}'", name);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        ServerCommands::Init { folder } => {
            use std::path::Path;
            use tokio::io::AsyncWriteExt;

            let target = Path::new(&folder);
            tokio::fs::create_dir_all(target.join("config")).await?;
            tokio::fs::create_dir_all(target.join("resources")).await?;

            // server-config.toml stub
            let cfg_path = target.join("config/server-config.toml");
            if !cfg_path.exists() {
                let sample_cfg = r#"[server]
name = "MyGameVerseServer"
admin_port = 30121
jwt_secret = "CHANGE_ME"
resources_dir = "./resources"
"#;
                tokio::fs::write(&cfg_path, sample_cfg).await?;
            }

            // docker-compose.yml stub
            let compose_path = target.join("docker-compose.yml");
            if !compose_path.exists() {
                let content = r#"version: '3.8'
services:
  gameverse_server:
    image: ghcr.io/gameverse/server:latest
    volumes:
      - ./resources:/opt/gameverse/resources:ro
      - ./config:/opt/gameverse/config:ro
    environment:
      - GAMEVERSE_CONFIG=/opt/gameverse/config/server-config.toml
    ports:
      - "30121:30121" # admin API
    restart: unless-stopped
"#;
                tokio::fs::write(&compose_path, content).await?;
            }

            // systemd unit
            let systemd_dir = target.join("systemd");
            tokio::fs::create_dir_all(&systemd_dir).await?;
            let unit_path = systemd_dir.join("gameverse.service");
            if !unit_path.exists() {
                let unit = format!(r#"[Unit]
Description=GameVerse Server
After=network.target

[Service]
Type=simple
User=gameverse
WorkingDirectory={wd}
ExecStart={wd}/gameverse_server {wd}/config/server-config.toml
Restart=on-failure
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
"#, wd = target.display());
                tokio::fs::write(&unit_path, unit).await?;
            }

            // NSSM install script (PowerShell)
            let nssm_path = target.join("install_nssm.ps1");
            if !nssm_path.exists() {
                let script = r#"param(
    [string]$NssmExe = "C:\\nssm\\win64\\nssm.exe"
)

if (-Not (Test-Path $NssmExe)) {
    Write-Error "❌ NSSM not found at $NssmExe"
    exit 1
}

$serviceName = "GameVerseServer"
$exePath      = "$PSScriptRoot\\gameverse_server.exe"
$configPath   = "$PSScriptRoot\\config\\server-config.toml"

Write-Host "🚀 Installing service $serviceName via NSSM..."
& $NssmExe install $serviceName $exePath $configPath
& $NssmExe set $serviceName DisplayName "GameVerse Server"
& $NssmExe set $serviceName Start SERVICE_AUTO_START

Write-Host "✅ Service $serviceName installed successfully."
Write-Host "Use 'nssm start $serviceName' to start and 'nssm stop $serviceName' to stop."
"#;
                tokio::fs::write(&nssm_path, script).await?;
            }

            println!("✅ GameVerse server skeleton created at {}", folder);
            println!("📋 Next steps:");
            println!("   1. cd {}", folder);
            println!("   2. cargo build -p gameverse-core --bin gameverse_server --release");
            println!("   3. For auto-start:");
            println!("      Linux:   sudo cp systemd/gameverse.service /etc/systemd/system/ && sudo systemctl daemon-reload && sudo systemctl enable --now gameverse");
            println!("      Windows: .\\install_nssm.ps1 (run as Administrator)");
            println!("      Docker:  docker-compose up -d");
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::config::Config;

    #[tokio::test]
    async fn init_creates_required_files() {
        let tmp = tempdir().unwrap();
        let target = tmp.path().join("srv");
        let cfg = Config::default();
        // Execute Init command
        execute(ServerCommands::Init { folder: target.to_string_lossy().to_string() }, &cfg)
            .await
            .expect("Init should succeed");

        assert!(target.join("config/server-config.toml").exists());
        assert!(target.join("docker-compose.yml").exists());
        assert!(target.join("systemd/gameverse.service").exists());
        assert!(target.join("install_nssm.ps1").exists());
    }
} 