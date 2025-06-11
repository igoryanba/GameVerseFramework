use crate::models::{Inventory, Item, ItemTemplate};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// JSON-представление инвентаря
#[derive(Serialize, Deserialize)]
pub struct InventoryJson {
    pub id: String,
    pub owner_id: String,
    pub owner_type: String,
    pub inventory_type: String,
    pub max_weight: f64,
    pub max_slots: i32,
    pub name: String,
    pub metadata: HashMap<String, String>,
}

/// JSON-представление предмета
#[derive(Serialize, Deserialize)]
pub struct ItemJson {
    pub id: String,
    pub inventory_id: String,
    pub template_id: String,
    pub position: i32,
    pub quantity: i32,
    pub durability: f64,
    pub metadata: HashMap<String, String>,
}

/// JSON-представление шаблона предмета
#[derive(Serialize, Deserialize)]
pub struct ItemTemplateJson {
    pub id: String,
    pub name: String,
    pub description: String,
    pub weight: f64,
    pub stackable: bool,
    pub max_stack: i32,
    pub max_durability: f64,
    pub icon: String,
    pub category: String,
    pub properties: HashMap<String, String>,
}

/// Данные для создания инвентаря
#[derive(Deserialize)]
pub struct CreateInventoryJson {
    pub owner_id: String,
    pub owner_type: String,
    pub inventory_type: String,
    pub max_weight: f64,
    pub max_slots: i32,
    pub name: String,
    pub metadata: Option<HashMap<String, String>>,
}

/// Данные для обновления инвентаря
#[derive(Deserialize)]
pub struct UpdateInventoryJson {
    pub max_weight: Option<f64>,
    pub max_slots: Option<i32>,
    pub name: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

/// Данные для создания предмета
#[derive(Deserialize)]
pub struct CreateItemJson {
    pub template_id: String,
    pub position: i32,
    pub quantity: i32,
    pub metadata: Option<HashMap<String, String>>,
}

/// Данные для обновления предмета
#[derive(Deserialize)]
pub struct UpdateItemJson {
    pub position: Option<i32>,
    pub quantity: Option<i32>,
    pub durability: Option<f64>,
    pub metadata: Option<HashMap<String, String>>,
}

/// Данные для создания шаблона предмета
#[derive(Deserialize)]
pub struct CreateItemTemplateJson {
    pub name: String,
    pub description: String,
    pub weight: f64,
    pub stackable: bool,
    pub max_stack: i32,
    pub max_durability: f64,
    pub icon: String,
    pub category: String,
    pub properties: Option<HashMap<String, String>>,
}

/// Данные для обновления шаблона предмета
#[derive(Deserialize)]
pub struct UpdateItemTemplateJson {
    pub name: Option<String>,
    pub description: Option<String>,
    pub weight: Option<f64>,
    pub stackable: Option<bool>,
    pub max_stack: Option<i32>,
    pub max_durability: Option<f64>,
    pub icon: Option<String>,
    pub category: Option<String>,
    pub properties: Option<HashMap<String, String>>,
}

/// Преобразование из Inventory в InventoryJson
impl From<&Inventory> for InventoryJson {
    fn from(inventory: &Inventory) -> Self {
        InventoryJson {
            id: inventory.id.to_string(),
            owner_id: inventory.owner_id.clone(),
            owner_type: inventory.owner_type.clone(),
            inventory_type: inventory.inventory_type.to_string(),
            max_weight: inventory.max_weight,
            max_slots: inventory.max_slots,
            name: inventory.name.clone(),
            metadata: inventory.metadata.0.clone(),
        }
    }
}

/// Преобразование из Item в ItemJson
impl From<&Item> for ItemJson {
    fn from(item: &Item) -> Self {
        ItemJson {
            id: item.id.to_string(),
            inventory_id: item.inventory_id.to_string(),
            template_id: item.template_id.to_string(),
            position: item.position,
            quantity: item.quantity,
            durability: item.durability,
            metadata: item.metadata.0.clone(),
        }
    }
}

/// Преобразование из ItemTemplate в ItemTemplateJson
impl From<&ItemTemplate> for ItemTemplateJson {
    fn from(template: &ItemTemplate) -> Self {
        ItemTemplateJson {
            id: template.id.to_string(),
            name: template.name.clone(),
            description: template.description.clone(),
            weight: template.weight,
            stackable: template.stackable,
            max_stack: template.max_stack,
            max_durability: template.max_durability,
            icon: template.icon.clone(),
            category: template.category.clone(),
            properties: template.properties.0.clone(),
        }
    }
} 