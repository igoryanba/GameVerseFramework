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
    }
    
    Ok(())
} 