use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;
use std::collections::HashMap;
use std::fmt;

/// Типы инвентарей
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "inventory_type", rename_all = "snake_case")]
pub enum InventoryType {
    Player,     // Персональный инвентарь игрока
    Vehicle,    // Инвентарь транспортного средства
    Container,  // Инвентарь контейнера в мире
    Shop,       // Инвентарь магазина
    Temp,       // Временный инвентарь (например, обмен)
}

impl fmt::Display for InventoryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InventoryType::Player => write!(f, "player"),
            InventoryType::Vehicle => write!(f, "vehicle"),
            InventoryType::Container => write!(f, "container"),
            InventoryType::Shop => write!(f, "shop"),
            InventoryType::Temp => write!(f, "temp"),
        }
    }
}

/// Модель инвентаря в базе данных
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Inventory {
    /// Уникальный идентификатор инвентаря
    pub id: Uuid,
    
    /// Идентификатор владельца инвентаря (игрок, транспорт, объект)
    pub owner_id: String,
    
    /// Тип владельца (player, vehicle, container и т.д.)
    pub owner_type: String,
    
    /// Тип инвентаря
    pub inventory_type: InventoryType,
    
    /// Максимальный вес, который может содержать инвентарь
    pub max_weight: f64,
    
    /// Максимальное количество слотов в инвентаре
    pub max_slots: i32,
    
    /// Название инвентаря
    pub name: String,
    
    /// Дополнительные метаданные инвентаря
    pub metadata: Json<HashMap<String, String>>,
    
    /// Дата и время создания инвентаря
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Дата и время последнего обновления инвентаря
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Данные для создания нового инвентаря
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInventoryData {
    pub owner_id: String,
    pub owner_type: String,
    pub inventory_type: InventoryType,
    pub max_weight: f64,
    pub max_slots: i32,
    pub name: String,
    pub metadata: HashMap<String, String>,
}

/// Данные для обновления инвентаря
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInventoryData {
    pub max_weight: Option<f64>,
    pub max_slots: Option<i32>,
    pub name: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

impl Inventory {
    /// Создает новый инвентарь
    pub fn new(data: CreateInventoryData) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            owner_id: data.owner_id,
            owner_type: data.owner_type,
            inventory_type: data.inventory_type,
            max_weight: data.max_weight,
            max_slots: data.max_slots,
            name: data.name,
            metadata: Json(data.metadata),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Проверяет, может ли инвентарь вместить дополнительный вес
    pub fn can_fit_weight(&self, additional_weight: f64) -> bool {
        // TODO: Реализовать получение текущего веса из предметов
        let current_weight = 0.0;
        current_weight + additional_weight <= self.max_weight
    }
    
    /// Проверяет, есть ли в инвентаре свободные слоты
    pub fn has_free_slots(&self, required_slots: i32) -> bool {
        // TODO: Реализовать получение текущего количества предметов
        let current_items = 0;
        current_items + required_slots <= self.max_slots
    }
    
    /// Применяет обновления к инвентарю
    pub fn apply_update(&mut self, data: UpdateInventoryData) {
        if let Some(max_weight) = data.max_weight {
            self.max_weight = max_weight;
        }
        
        if let Some(max_slots) = data.max_slots {
            self.max_slots = max_slots;
        }
        
        if let Some(name) = data.name {
            self.name = name;
        }
        
        if let Some(metadata) = data.metadata {
            self.metadata = Json(metadata);
        }
        
        self.updated_at = chrono::Utc::now();
    }
} 