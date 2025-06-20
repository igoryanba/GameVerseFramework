//! # Игровой движок GameVerse
//!
//! Интеграция с игровыми API и управление игровой логикой
//! Превосходство над FiveM: современная архитектура, type safety, лучшая производительность

use anyhow::{Result, Context};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, warn, error};

use crate::game_integration::{GameIntegrator, GameType, IntegrationState};

/// Основной игровой движок
#[derive(Debug)]
pub struct GameEngine {
    /// Состояние движка
    initialized: bool,
    /// Интегратор с игрой
    game_integrator: Option<Arc<RwLock<GameIntegrator>>>,
    /// Тип игры
    game_type: Option<GameType>,
}

impl GameEngine {
    /// Создать новый экземпляр движка
    pub fn new() -> Self {
        Self {
            initialized: false,
            game_integrator: None,
            game_type: None,
        }
    }
    
    /// Создать движок для конкретной игры
    pub fn for_game(game_type: GameType) -> Self {
        Self {
            initialized: false,
            game_integrator: None,
            game_type: Some(game_type),
        }
    }
    
    /// Инициализировать движок
    pub async fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }

        info!("🎮 Initializing GameVerse Engine...");

        // Определяем тип игры если не задан
        let game_type = match self.game_type {
            Some(gt) => gt,
            None => self.detect_game_type().await?,
        };

        info!("🎯 Detected game type: {:?}", game_type);

        // Создаем интегратор с игрой
        let integrator = GameIntegrator::new(game_type).await
            .context("Failed to create game integrator")?;

        self.game_integrator = Some(Arc::new(RwLock::new(integrator)));
        self.game_type = Some(game_type);

        // Подключаемся к игре
        self.connect_to_game().await
            .context("Failed to connect to game")?;

        self.initialized = true;
        info!("✅ GameVerse Engine initialized successfully");

        Ok(())
    }

    /// Автоматически определить тип игры
    async fn detect_game_type(&self) -> Result<GameType> {
        info!("🔍 Auto-detecting game type...");

        // Проверяем запущенные процессы
        if self.is_process_running("GTA5.exe").await || self.is_process_running("FiveM.exe").await {
            info!("🎯 Detected GTA V");
            return Ok(GameType::GtaV);
        }

        if self.is_process_running("RDR2.exe").await || self.is_process_running("RedM.exe").await {
            info!("🎯 Detected RDR2");
            return Ok(GameType::Rdr2);
        }

        // Если ничего не найдено, по умолчанию GTA V
        warn!("⚠️ Could not detect game type, defaulting to GTA V");
        Ok(GameType::GtaV)
    }

    /// Проверить, запущен ли процесс
    async fn is_process_running(&self, process_name: &str) -> bool {
        #[cfg(windows)]
        {
            use windows::Win32::System::Diagnostics::ToolHelp::*;
            use windows::Win32::Foundation::*;

            unsafe {
                if let Ok(snapshot) = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) {
                    let mut entry = PROCESSENTRY32W {
                        dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
                        ..Default::default()
                    };

                    if Process32FirstW(snapshot, &mut entry).is_ok() {
                        loop {
                            let current_name = String::from_utf16_lossy(&entry.szExeFile)
                                .trim_end_matches('\0');

                            if current_name.eq_ignore_ascii_case(process_name) {
                                CloseHandle(snapshot);
                                return true;
                            }

                            if Process32NextW(snapshot, &mut entry).is_err() {
                                break;
                            }
                        }
                    }

                    CloseHandle(snapshot);
                }
            }
        }

        #[cfg(unix)]
        {
            use std::fs;
            use std::str::FromStr;

            if let Ok(proc_dir) = fs::read_dir("/proc") {
                for entry in proc_dir.flatten() {
                    let path = entry.path();
                    
                    if let Some(pid_str) = path.file_name().and_then(|n| n.to_str()) {
                        if u32::from_str(pid_str).is_ok() {
                            let comm_path = path.join("comm");
                            if let Ok(comm) = fs::read_to_string(comm_path) {
                                let comm = comm.trim();
                                if comm.eq_ignore_ascii_case(process_name) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }

    /// Подключиться к игре
    async fn connect_to_game(&mut self) -> Result<()> {
        if let Some(integrator_arc) = &self.game_integrator {
            let mut integrator = integrator_arc.write().await;
            integrator.connect().await
                .context("Failed to establish game connection")?;

            info!("🔗 Successfully connected to game");
        }

        Ok(())
    }

    /// Отключиться от игры
    pub async fn disconnect_from_game(&mut self) -> Result<()> {
        if let Some(integrator_arc) = &self.game_integrator {
            let mut integrator = integrator_arc.write().await;
            integrator.disconnect().await
                .context("Failed to disconnect from game")?;

            info!("🔌 Disconnected from game");
        }

        Ok(())
    }

    /// Получить интегратор с игрой
    pub fn game_integrator(&self) -> Option<Arc<RwLock<GameIntegrator>>> {
        self.game_integrator.clone()
    }

    /// Получить тип игры
    pub fn game_type(&self) -> Option<GameType> {
        self.game_type
    }

    /// Проверить состояние подключения к игре
    pub async fn connection_state(&self) -> Option<IntegrationState> {
        if let Some(integrator_arc) = &self.game_integrator {
            let integrator = integrator_arc.read().await;
            Some(integrator.state())
        } else {
            None
        }
    }

    /// Проверить инициализацию
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Остановить движок
    pub async fn shutdown(&mut self) -> Result<()> {
        if !self.initialized {
            return Ok(());
        }

        info!("🛑 Shutting down GameVerse Engine...");

        // Отключаемся от игры
        self.disconnect_from_game().await?;

        self.game_integrator = None;
        self.initialized = false;

        info!("✅ GameVerse Engine shut down successfully");
        Ok(())
    }
}

impl Default for GameEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for GameEngine {
    fn drop(&mut self) {
        if self.initialized {
            warn!("GameEngine dropped while still initialized - this may cause issues");
        }
    }
} 