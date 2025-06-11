use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;
use std::collections::HashMap;

/// Модель предмета в базе данных
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Item {
    /// Уникальный идентификатор предмета
    pub id: Uuid,
    
    /// Идентификатор инвентаря, которому принадлежит предмет
    pub inventory_id: Uuid,
    
    /// Идентификатор шаблона предмета
    pub template_id: Uuid,
    
    /// Позиция предмета в инвентаре (номер слота)
    pub position: i32,
    
    /// Количество предметов в стаке
    pub quantity: i32,
    
    /// Текущая прочность предмета
    pub durability: f64,
    
    /// Дополнительные метаданные предмета
    pub metadata: Json<HashMap<String, String>>,
    
    /// Дата и время создания предмета
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Дата и время последнего обновления предмета
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Данные для создания нового предмета
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItemData {
    pub inventory_id: Uuid,
    pub template_id: Uuid,
    pub position: i32,
    pub quantity: i32,
    pub durability: f64,
    pub metadata: HashMap<String, String>,
}

/// Данные для обновления предмета
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateItemData {
    pub position: Option<i32>,
    pub quantity: Option<i32>,
    pub durability: Option<f64>,
    pub metadata: Option<HashMap<String, String>>,
}

impl Item {
    /// Создает новый предмет
    pub fn new(data: CreateItemData) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            inventory_id: data.inventory_id,
            template_id: data.template_id,
            position: data.position,
            quantity: data.quantity,
            durability: data.durability,
            metadata: Json(data.metadata),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Применяет обновления к предмету
    pub fn apply_update(&mut self, data: UpdateItemData) {
        if let Some(position) = data.position {
            self.position = position;
        }
        
        if let Some(quantity) = data.quantity {
            self.quantity = quantity;
        }
        
        if let Some(durability) = data.durability {
            self.durability = durability;
        }
        
        if let Some(metadata) = data.metadata {
            self.metadata = Json(metadata);
        }
        
        self.updated_at = chrono::Utc::now();
    }
    
    /// Уменьшает количество предметов в стаке
    /// Возвращает true, если предмет нужно полностью удалить
    pub fn decrease_quantity(&mut self, amount: i32) -> bool {
        if amount <= 0 {
            return false;
        }
        
        if amount >= self.quantity {
            self.quantity = 0;
            return true;
        }
        
        self.quantity -= amount;
        self.updated_at = chrono::Utc::now();
        false
    }
    
    /// Увеличивает количество предметов в стаке
    pub fn increase_quantity(&mut self, amount: i32) {
        if amount <= 0 {
            return;
        }
        
        self.quantity += amount;
        self.updated_at = chrono::Utc::now();
    }
    
    /// Уменьшает прочность предмета
    /// Возвращает true, если предмет разрушен
    pub fn decrease_durability(&mut self, amount: f64) -> bool {
        if amount <= 0.0 {
            return false;
        }
        
        if amount >= self.durability {
            self.durability = 0.0;
            return true;
        }
        
        self.durability -= amount;
        self.updated_at = chrono::Utc::now();
        false
    }
    
    /// Перемещает предмет в другой инвентарь
    pub fn move_to_inventory(&mut self, inventory_id: Uuid, position: i32) {
        self.inventory_id = inventory_id;
        self.position = position;
        self.updated_at = chrono::Utc::now();
    }
} 