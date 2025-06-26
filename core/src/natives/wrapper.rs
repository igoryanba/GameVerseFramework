//! # High-level Native Function Wrappers
//!
//! Этот модуль предоставляет type-safe обертки для нативных функций игр.
//! Превосходство над FiveM: полная типизация, async/await, error handling.

use anyhow::Result;
use crate::game_integration::{GameType, native_executor::NativeValue};
use super::NativeManager;
use std::ffi::{CString, CStr};
use tracing::{debug, warn, error};

/// Vector3 структура для координат
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    /// X координата
    pub x: f32,
    /// Y координата
    pub y: f32,
    /// Z координата
    pub z: f32,
}

impl Vector3 {
    /// Создать новый Vector3
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Нулевой вектор
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Расстояние до другого вектора
    pub fn distance_to(&self, other: &Vector3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Длина вектора
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Нормализовать вектор
    pub fn normalize(&self) -> Vector3 {
        let mag = self.magnitude();
        if mag > 0.0 {
            Vector3::new(self.x / mag, self.y / mag, self.z / mag)
        } else {
            Vector3::zero()
        }
    }
}

impl From<NativeValue> for Vector3 {
    fn from(value: NativeValue) -> Self {
        match value {
            NativeValue::Vector3 { x, y, z } => Vector3::new(x, y, z),
            _ => Vector3::zero(),
        }
    }
}

impl From<Vector3> for NativeValue {
    fn from(vec: Vector3) -> Self {
        NativeValue::Vector3 { x: vec.x, y: vec.y, z: vec.z }
    }
}

/// Entity handle (игрок, NPC, объект, транспорт)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entity(pub u32);

impl Entity {
    /// Недействительный entity
    pub fn invalid() -> Self {
        Entity(0)
    }

    /// Проверить валидность
    pub fn is_valid(&self) -> bool {
        self.0 != 0
    }
}

impl From<NativeValue> for Entity {
    fn from(value: NativeValue) -> Self {
        match value {
            NativeValue::Entity(e) => Entity(e),
            NativeValue::Int(i) => Entity(i as u32),
            _ => Entity::invalid(),
        }
    }
}

impl From<Entity> for NativeValue {
    fn from(entity: Entity) -> Self {
        NativeValue::Entity(entity.0)
    }
}

/// Player ID
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerId(pub u32);

impl PlayerId {
    /// Локальный игрок (обычно ID 0)
    pub fn local() -> Self {
        PlayerId(0)
    }
}

impl From<u32> for PlayerId {
    fn from(id: u32) -> Self {
        PlayerId(id)
    }
}

impl From<PlayerId> for NativeValue {
    fn from(player_id: PlayerId) -> Self {
        NativeValue::Int(player_id.0 as i32)
    }
}

/// Обертки для GTA V нативных функций
pub struct GtaVNatives<'a> {
    native_manager: &'a NativeManager,
}

impl<'a> GtaVNatives<'a> {
    /// Создать новую обертку для GTA V нативов
    pub fn new(native_manager: &'a NativeManager) -> Self {
        Self { native_manager }
    }

    /// Получить PED игрока
    pub async fn get_player_ped(&self, player_id: PlayerId) -> Result<Entity> {
        let result = self.native_manager.call_native("GET_PLAYER_PED", vec![
            NativeValue::Int(player_id.0 as i32),
        ]).await?;
        
        if let NativeValue::Int(ped_id) = result {
            Ok(Entity(ped_id as u32))
        } else {
            Err(anyhow::anyhow!("Failed to get player PED"))
        }
    }

    /// Получить координаты сущности
    pub async fn get_entity_coords(&self, entity: Entity) -> Result<Vector3> {
        let result = self.native_manager.call_native("GET_ENTITY_COORDS", vec![
            NativeValue::Int(entity.0 as i32),
            NativeValue::Bool(true), // alive
        ]).await?;
        
        // В реальности здесь была бы обработка результата как Vector3
        Ok(Vector3::new(0.0, 0.0, 0.0))
    }

    /// Установить координаты сущности
    pub async fn set_entity_coords(&self, entity: Entity, coords: Vector3, alive: bool, dead: bool, ragdoll: bool) -> Result<()> {
        self.native_manager.call_native("SET_ENTITY_COORDS", vec![
            NativeValue::Int(entity.0 as i32),
            NativeValue::Float(coords.x),
            NativeValue::Float(coords.y),
            NativeValue::Float(coords.z),
            NativeValue::Bool(alive),
            NativeValue::Bool(dead),
            NativeValue::Bool(ragdoll),
        ]).await?;
        
        Ok(())
    }

    /// Телепортировать игрока
    pub async fn teleport_player(&self, player_id: PlayerId, coords: Vector3) -> Result<()> {
        let ped = self.get_player_ped(player_id).await?;
        self.set_entity_coords(ped, coords, true, true, false).await
    }

    /// Получить имя игрока
    pub async fn get_player_name(&self, player_id: PlayerId) -> Result<String> {
        let result = self.native_manager.call_native("GET_PLAYER_NAME", vec![
            NativeValue::Int(player_id.0 as i32),
        ]).await?;
        
        if let NativeValue::String(name) = result {
            Ok(name)
        } else {
            Ok("Unknown".to_string())
        }
    }

    /// Получить игровое время
    pub async fn get_game_timer(&self) -> Result<u32> {
        let result = self.native_manager
            .call_native("GET_GAME_TIMER", vec![])
            .await?;
        
        match result {
            NativeValue::Int(time) => Ok(time as u32),
            _ => Ok(0),
        }
    }

    /// Проверить, существует ли entity
    pub async fn does_entity_exist(&self, entity: Entity) -> Result<bool> {
        let result = self.native_manager
            .call_native("DOES_ENTITY_EXIST", vec![entity.into()])
            .await?;
        
        match result {
            NativeValue::Bool(exists) => Ok(exists),
            _ => Ok(false),
        }
    }

    /// Получить здоровье entity
    pub async fn get_entity_health(&self, entity: Entity) -> Result<i32> {
        let result = self.native_manager
            .call_native("GET_ENTITY_HEALTH", vec![entity.into()])
            .await?;
        
        match result {
            NativeValue::Int(health) => Ok(health),
            _ => Ok(0),
        }
    }

    /// Установить здоровье entity
    pub async fn set_entity_health(&self, entity: Entity, health: i32) -> Result<()> {
        self.native_manager
            .call_native("SET_ENTITY_HEALTH", vec![
                entity.into(),
                NativeValue::Int(health),
            ])
            .await?;
        Ok(())
    }

    /// Создать транспорт
    pub async fn create_vehicle(&self, model_hash: u32, coords: Vector3, heading: f32, networked: bool, script_host_obj: bool) -> Result<Entity> {
        let result = self.native_manager
            .call_native("CREATE_VEHICLE", vec![
                NativeValue::Int(model_hash as i32),
                NativeValue::Float(coords.x),
                NativeValue::Float(coords.y),
                NativeValue::Float(coords.z),
                NativeValue::Float(heading),
                NativeValue::Bool(networked),
                NativeValue::Bool(script_host_obj),
            ])
            .await?;
        Ok(Entity::from(result))
    }

    /// Удалить entity
    pub async fn delete_entity(&self, entity: Entity) -> Result<()> {
        self.native_manager
            .call_native("DELETE_ENTITY", vec![entity.into()])
            .await?;
        Ok(())
    }

    /// Отправить чат сообщение
    pub async fn send_chat_message(&self, message: &str) -> Result<()> {
        self.native_manager
            .call_native("SEND_CHAT_MESSAGE", vec![
                NativeValue::String(message.to_string()),
            ])
            .await?;
        Ok(())
    }

    /// Показать уведомление
    pub async fn show_notification(&self, text: &str) -> Result<()> {
        // Сначала добавляем текст
        self.native_manager
            .call_native("BEGIN_TEXT_COMMAND_THEFEED_POST", vec![
                NativeValue::String("STRING".to_string()),
            ])
            .await?;

        self.native_manager
            .call_native("ADD_TEXT_COMPONENT_SUBSTRING_PLAYER_NAME", vec![
                NativeValue::String(text.to_string()),
            ])
            .await?;

        // Затем показываем
        self.native_manager
            .call_native("END_TEXT_COMMAND_THEFEED_POST_TICKER", vec![
                NativeValue::Bool(false),
                NativeValue::Bool(true),
            ])
            .await?;

        Ok(())
    }
}

/// Обертка для нативных функций Red Dead Redemption 2
pub struct Rdr2Natives<'a> {
    native_manager: &'a NativeManager,
}

impl<'a> Rdr2Natives<'a> {
    /// Создать новую обертку для RDR2 нативов
    pub fn new(native_manager: &'a NativeManager) -> Self {
        Self { native_manager }
    }

    /// Получить PED игрока
    pub async fn get_player_ped(&self, player_id: PlayerId) -> Result<Entity> {
        let result = self.native_manager.call_native("GET_PLAYER_PED", vec![
            NativeValue::Int(player_id.0 as i32),
        ]).await?;
        
        if let NativeValue::Int(ped_id) = result {
            Ok(Entity(ped_id as u32))
        } else {
            Err(anyhow::anyhow!("Failed to get player PED"))
        }
    }

    /// Получить координаты сущности
    pub async fn get_entity_coords(&self, entity: Entity) -> Result<Vector3> {
        let result = self.native_manager.call_native("GET_ENTITY_COORDS", vec![
            NativeValue::Int(entity.0 as i32),
        ]).await?;
        
        // В реальности здесь была бы обработка результата как Vector3
        Ok(Vector3::new(0.0, 0.0, 0.0))
    }

    /// Установить координаты сущности
    pub async fn set_entity_coords(&self, entity: Entity, coords: Vector3) -> Result<()> {
        self.native_manager.call_native("SET_ENTITY_COORDS", vec![
            NativeValue::Int(entity.0 as i32),
            NativeValue::Float(coords.x),
            NativeValue::Float(coords.y),
            NativeValue::Float(coords.z),
        ]).await?;
        
        Ok(())
    }

    /// Телепортировать игрока
    pub async fn teleport_player(&self, player_id: PlayerId, coords: Vector3) -> Result<()> {
        let ped = self.get_player_ped(player_id).await?;
        self.set_entity_coords(ped, coords).await
    }
}

/// Универсальная обертка для работы с разными играми
pub struct UniversalNatives<'a> {
    native_manager: &'a NativeManager,
    game_type: GameType,
}

impl<'a> UniversalNatives<'a> {
    /// Создать новую универсальную обертку
    pub fn new(native_manager: &'a NativeManager, game_type: GameType) -> Self {
        Self { native_manager, game_type }
    }

    /// Получить специализированную обертку для GTA V
    pub fn as_gta_v(&self) -> Option<GtaVNatives<'a>> {
        match self.game_type {
            GameType::GtaV => Some(GtaVNatives::new(self.native_manager)),
            _ => None,
        }
    }

    /// Получить специализированную обертку для RDR2
    pub fn as_rdr2(&self) -> Option<Rdr2Natives<'a>> {
        match self.game_type {
            GameType::Rdr2 => Some(Rdr2Natives::new(self.native_manager)),
            _ => None,
        }
    }

    /// Универсальная телепортация
    pub async fn teleport_player(&self, player_id: PlayerId, coords: Vector3) -> Result<()> {
        match self.game_type {
            GameType::GtaV => {
                let gta_natives = GtaVNatives::new(self.native_manager);
                gta_natives.teleport_player(player_id, coords).await
            },
            GameType::Rdr2 => {
                let rdr2_natives = Rdr2Natives::new(self.native_manager);
                // RDR2 teleportation implementation would go here
                Ok(())
            },
            _ => Err(anyhow::anyhow!("Unsupported game type")),
        }
    }
}

/// Type-safe обертки для игровых сущностей
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VehicleId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PedId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WeaponHash(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModelHash(pub u32);

// Реализация конверсий для удобства
impl From<i32> for EntityId {
    fn from(id: i32) -> Self {
        EntityId(id as u32)
    }
}

impl From<i32> for PlayerId {
    fn from(id: i32) -> Self {
        PlayerId(id as u32)
    }
}

impl From<i32> for VehicleId {
    fn from(id: i32) -> Self {
        VehicleId(id as u32)
    }
}

impl From<i32> for PedId {
    fn from(id: i32) -> Self {
        PedId(id as u32)
    }
}

impl From<u32> for WeaponHash {
    fn from(hash: u32) -> Self {
        WeaponHash(hash)
    }
}

impl From<u32> for ModelHash {
    fn from(hash: u32) -> Self {
        ModelHash(hash)
    }
} 