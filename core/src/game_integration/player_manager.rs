//! # Player Manager
//!
//! Модуль для управления игроками в игровом мире.
//! Превосходство над FiveM: type-safe player management, async operations.

use anyhow::{Result, bail};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Менеджер игроков
#[derive(Debug)]
pub struct PlayerManager {
    /// Список активных игроков
    players: HashMap<PlayerId, Player>,
    /// Максимальное количество игроков
    max_players: u32,
    /// Счетчик для генерации ID
    next_player_id: u32,
}

/// Уникальный ID игрока
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(pub u32);

/// Информация об игроке
#[derive(Debug, Clone)]
pub struct Player {
    /// ID игрока
    pub id: PlayerId,
    /// Имя игрока
    pub name: String,
    /// Сетевой ID
    pub net_id: Option<u32>,
    /// PED игрока в игре
    pub ped_handle: Option<u32>,
    /// Позиция в мире
    pub position: Vector3,
    /// Состояние игрока
    pub state: PlayerState,
    /// Время подключения
    pub connected_at: std::time::Instant,
    /// Ping
    pub ping: u32,
}

/// Позиция в трехмерном пространстве
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Состояние игрока
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    /// Подключается
    Connecting,
    /// Загружается в игру
    Loading,
    /// Активен в игре
    Active,
    /// Неактивен (AFK)
    Idle,
    /// Отключается
    Disconnecting,
}

impl PlayerManager {
    /// Создать новый менеджер игроков
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            max_players: 64, // По умолчанию
            next_player_id: 1,
        }
    }

    /// Добавить нового игрока
    pub async fn add_player(&mut self, name: String, net_id: Option<u32>) -> Result<PlayerId> {
        if self.players.len() >= self.max_players as usize {
            bail!("Server is full");
        }

        let player_id = PlayerId(self.next_player_id);
        self.next_player_id += 1;

        let player = Player {
            id: player_id,
            name: name.clone(),
            net_id,
            ped_handle: None,
            position: Vector3::new(0.0, 0.0, 0.0),
            state: PlayerState::Connecting,
            connected_at: std::time::Instant::now(),
            ping: 0,
        };

        self.players.insert(player_id, player);
        
        info!("Player {} connected with ID {:?}", name, player_id);
        
        Ok(player_id)
    }

    /// Удалить игрока
    pub async fn remove_player(&mut self, player_id: PlayerId) -> Result<()> {
        if let Some(player) = self.players.remove(&player_id) {
            info!("Player {} disconnected", player.name);
            Ok(())
        } else {
            bail!("Player {:?} not found", player_id)
        }
    }

    /// Получить игрока по ID
    pub fn get_player(&self, player_id: PlayerId) -> Option<&Player> {
        self.players.get(&player_id)
    }

    /// Получить мутабельную ссылку на игрока
    pub fn get_player_mut(&mut self, player_id: PlayerId) -> Option<&mut Player> {
        self.players.get_mut(&player_id)
    }

    /// Обновить позицию игрока
    pub async fn update_player_position(&mut self, player_id: PlayerId, position: Vector3) -> Result<()> {
        if let Some(player) = self.players.get_mut(&player_id) {
            player.position = position;
            debug!("Updated position for player {}: {:?}", player.name, position);
            Ok(())
        } else {
            bail!("Player {:?} not found", player_id)
        }
    }

    /// Обновить состояние игрока
    pub async fn update_player_state(&mut self, player_id: PlayerId, state: PlayerState) -> Result<()> {
        if let Some(player) = self.players.get_mut(&player_id) {
            let old_state = player.state;
            player.state = state;
            debug!("Player {} state changed from {:?} to {:?}", player.name, old_state, state);
            Ok(())
        } else {
            bail!("Player {:?} not found", player_id)
        }
    }

    /// Получить список всех игроков
    pub fn get_all_players(&self) -> Vec<&Player> {
        self.players.values().collect()
    }

    /// Получить количество активных игроков
    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    /// Получить максимальное количество игроков
    pub fn max_players(&self) -> u32 {
        self.max_players
    }

    /// Установить максимальное количество игроков
    pub fn set_max_players(&mut self, max_players: u32) {
        self.max_players = max_players;
        info!("Max players set to {}", max_players);
    }

    /// Найти игрока по имени
    pub fn find_player_by_name(&self, name: &str) -> Option<&Player> {
        self.players.values().find(|player| player.name == name)
    }

    /// Получить список игроков в радиусе от позиции
    pub fn get_players_in_range(&self, center: Vector3, radius: f32) -> Vec<&Player> {
        self.players.values()
            .filter(|player| center.distance_to(player.position) <= radius)
            .collect()
    }
}

impl Vector3 {
    /// Создать новый вектор
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Нулевой вектор
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Вычислить расстояние до другой точки
    pub fn distance_to(&self, other: Vector3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Вычислить расстояние до другой точки (2D, без Z)
    pub fn distance_to_2d(&self, other: Vector3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

impl Default for PlayerManager {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player({})", self.0)
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
} 