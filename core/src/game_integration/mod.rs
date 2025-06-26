//! # Game Integration Layer
//!
//! Этот модуль отвечает за интеграцию с игровыми процессами (GTA V, RDR2).
//! Превосходство над FiveM:
//! - Современная архитектура с Rust безопасностью
//! - Type-safe native calls
//! - Memory-safe игровые хуки
//! - Асинхронная обработка событий
//! - Cross-platform поддержка

pub mod game_hook;
pub mod memory_manager;
pub mod native_executor;
pub mod player_manager;
pub mod event_system;

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Основной интегратор с игрой
#[derive(Debug)]
pub struct GameIntegrator {
    /// Хук к игровому процессу
    game_hook: Arc<RwLock<game_hook::GameHook>>,
    /// Менеджер памяти игры
    memory_manager: Arc<RwLock<memory_manager::MemoryManager>>,
    /// Исполнитель нативных функций
    native_executor: Arc<RwLock<native_executor::NativeExecutor>>,
    /// Менеджер игроков
    player_manager: Arc<RwLock<player_manager::PlayerManager>>,
    /// Система событий
    event_system: Arc<event_system::EventSystem>,
    /// Тип игры
    game_type: GameType,
    /// Состояние интеграции
    integration_state: IntegrationState,
}

/// Поддерживаемые игры
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameType {
    /// Grand Theft Auto V
    GtaV,
    /// Red Dead Redemption 2
    Rdr2,
}

/// Состояние интеграции с игрой
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationState {
    /// Не подключено
    Disconnected,
    /// Подключение в процессе
    Connecting,
    /// Подключено и готово
    Connected,
    /// Ошибка подключения
    Error,
}

impl GameIntegrator {
    /// Создать новый интегратор
    pub async fn new(game_type: GameType) -> Result<Self> {
        let game_hook = Arc::new(RwLock::new(game_hook::GameHook::new(game_type).await?));
        let memory_manager = Arc::new(RwLock::new(memory_manager::MemoryManager::new(game_type).await?));
        let native_executor = Arc::new(RwLock::new(native_executor::NativeExecutor::new(
            game_type,
            memory_manager.clone()
        ).await?));
        let player_manager = Arc::new(RwLock::new(player_manager::PlayerManager::new()));
        let event_system = Arc::new(event_system::EventSystem::new());

        Ok(Self {
            game_hook,
            memory_manager,
            native_executor,
            player_manager,
            event_system,
            game_type,
            integration_state: IntegrationState::Disconnected,
        })
    }

    /// Подключиться к игре
    pub async fn connect(&mut self) -> Result<()> {
        self.integration_state = IntegrationState::Connecting;
        
        tracing::info!("🎮 Connecting to {:?} game process...", self.game_type);

        // Подключаемся к игровому процессу
        {
            let mut hook = self.game_hook.write().await;
            hook.attach().await?;
        }

        // Инициализируем менеджер памяти
        {
            let mut memory_manager = self.memory_manager.write().await;
            memory_manager.set_game_hook(self.game_hook.clone());
            memory_manager.initialize().await?;
        }

        // Инициализируем исполнитель нативов
        {
            let mut native_executor = self.native_executor.write().await;
            native_executor.initialize().await?;
        }

        // Инициализируем систему событий
        self.event_system.initialize().await?;

        self.integration_state = IntegrationState::Connected;
        
        tracing::info!("✅ Successfully connected to {:?} game process", self.game_type);
        
        Ok(())
    }

    /// Отключиться от игры
    pub async fn disconnect(&mut self) -> Result<()> {
        tracing::info!("🔌 Disconnecting from {:?} game process...", self.game_type);

        // Останавливаем систему событий
        self.event_system.shutdown().await?;

        // Отключаемся от игрового процесса
        {
            let mut hook = self.game_hook.write().await;
            hook.detach().await?;
        }

        self.integration_state = IntegrationState::Disconnected;
        
        tracing::info!("✅ Successfully disconnected from game process");
        
        Ok(())
    }

    /// Получить состояние интеграции
    pub fn state(&self) -> IntegrationState {
        self.integration_state
    }

    /// Получить тип игры
    pub fn game_type(&self) -> GameType {
        self.game_type
    }

    /// Получить исполнитель нативных функций
    pub fn native_executor(&self) -> Arc<RwLock<native_executor::NativeExecutor>> {
        self.native_executor.clone()
    }

    /// Получить менеджер игроков
    pub fn player_manager(&self) -> Arc<RwLock<player_manager::PlayerManager>> {
        self.player_manager.clone()
    }

    /// Получить систему событий
    pub fn event_system(&self) -> Arc<event_system::EventSystem> {
        self.event_system.clone()
    }
}

impl Drop for GameIntegrator {
    fn drop(&mut self) {
        if self.integration_state == IntegrationState::Connected {
            tracing::warn!("GameIntegrator dropped while still connected - this may cause issues");
        }
    }
} 