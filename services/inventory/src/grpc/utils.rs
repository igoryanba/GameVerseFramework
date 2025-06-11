use crate::models::{
    Inventory, InventoryType, Item, ItemTemplate,
    CreateInventoryData, UpdateInventoryData,
    CreateItemData, UpdateItemData,
    CreateItemTemplateData, UpdateItemTemplateData,
};
use crate::proto::inventory::InventoryType as ProtoInventoryType;
use crate::proto::inventory::{
    Inventory as ProtoInventory, Item as ProtoItem, ItemTemplate as ProtoItemTemplate,
};
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

/// Конвертирует InventoryType в ProtoInventoryType
pub fn inventory_type_to_proto(inv_type: InventoryType) -> i32 {
    match inv_type {
        InventoryType::Player => ProtoInventoryType::Player as i32,
        InventoryType::Vehicle => ProtoInventoryType::Vehicle as i32,
        InventoryType::Container => ProtoInventoryType::Container as i32,
        InventoryType::Shop => ProtoInventoryType::Shop as i32,
        InventoryType::Temp => ProtoInventoryType::Temp as i32,
    }
}

/// Конвертирует ProtoInventoryType в InventoryType
pub fn inventory_type_from_proto(proto_type: i32) -> Result<InventoryType, String> {
    match proto_type {
        x if x == ProtoInventoryType::Player as i32 => Ok(InventoryType::Player),
        x if x == ProtoInventoryType::Vehicle as i32 => Ok(InventoryType::Vehicle),
        x if x == ProtoInventoryType::Container as i32 => Ok(InventoryType::Container),
        x if x == ProtoInventoryType::Shop as i32 => Ok(InventoryType::Shop),
        x if x == ProtoInventoryType::Temp as i32 => Ok(InventoryType::Temp),
        _ => Err("Неизвестный тип инвентаря".to_string()),
    }
}

/// Конвертирует Inventory в ProtoInventory
pub fn inventory_to_proto(inventory: &Inventory) -> ProtoInventory {
    ProtoInventory {
        id: inventory.id.to_string(),
        owner_id: inventory.owner_id.clone(),
        owner_type: inventory.owner_type.clone(),
        inventory_type: inventory_type_to_proto(inventory.inventory_type),
        max_weight: inventory.max_weight as f32,
        max_slots: inventory.max_slots,
        name: inventory.name.clone(),
        metadata: inventory.metadata.0.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
    }
}

/// Конвертирует Item в ProtoItem
pub fn item_to_proto(item: &Item) -> ProtoItem {
    ProtoItem {
        id: item.id.to_string(),
        inventory_id: item.inventory_id.to_string(),
        template_id: item.template_id.to_string(),
        position: item.position,
        quantity: item.quantity,
        durability: item.durability as f32,
        metadata: item.metadata.0.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
    }
}

/// Конвертирует ItemTemplate в ProtoItemTemplate
pub fn item_template_to_proto(template: &ItemTemplate) -> ProtoItemTemplate {
    ProtoItemTemplate {
        id: template.id.to_string(),
        name: template.name.clone(),
        description: template.description.clone(),
        weight: template.weight as f32,
        stackable: template.stackable,
        max_stack: template.max_stack,
        max_durability: template.max_durability as f32,
        icon: template.icon.clone(),
        category: template.category.clone(),
        properties: template.properties.0.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
    }
}

/// Создает CreateInventoryData из запроса
pub fn create_inventory_data_from_request(
    owner_id: String,
    owner_type: String,
    inventory_type: i32,
    max_weight: f32,
    max_slots: i32,
    name: String,
    metadata: HashMap<String, String>,
) -> Result<CreateInventoryData, String> {
    let inv_type = inventory_type_from_proto(inventory_type)?;
    
    Ok(CreateInventoryData {
        owner_id,
        owner_type,
        inventory_type: inv_type,
        max_weight: max_weight as f64,
        max_slots,
        name,
        metadata,
    })
}

/// Создает UpdateInventoryData из параметров запроса
pub fn update_inventory_data_from_request(
    max_weight: Option<f32>,
    max_slots: Option<i32>,
    name: Option<String>,
    metadata: Option<HashMap<String, String>>,
) -> UpdateInventoryData {
    UpdateInventoryData {
        max_weight: max_weight.map(|w| w as f64),
        max_slots,
        name,
        metadata,
    }
}

/// Создает CreateItemData из запроса
pub fn create_item_data_from_request(
    inventory_id: String,
    template_id: String,
    position: i32,
    quantity: i32,
    metadata: HashMap<String, String>,
) -> Result<CreateItemData, String> {
    let inventory_uuid = Uuid::from_str(&inventory_id)
        .map_err(|_| format!("Некорректный UUID инвентаря: {}", inventory_id))?;
        
    let template_uuid = Uuid::from_str(&template_id)
        .map_err(|_| format!("Некорректный UUID шаблона: {}", template_id))?;
        
    Ok(CreateItemData {
        inventory_id: inventory_uuid,
        template_id: template_uuid,
        position,
        quantity,
        durability: 100.0,
        metadata,
    })
}

/// Создает UpdateItemData из параметров запроса
pub fn update_item_data_from_request(
    position: Option<i32>,
    quantity: Option<i32>,
    durability: Option<f32>,
    metadata: Option<HashMap<String, String>>,
) -> UpdateItemData {
    UpdateItemData {
        position,
        quantity,
        durability: durability.map(|d| d as f64),
        metadata,
    }
}

/// Создает CreateItemTemplateData из запроса
pub fn create_item_template_data_from_request(
    name: String,
    description: String,
    weight: f32,
    stackable: bool,
    max_stack: i32,
    max_durability: f32,
    icon: String,
    category: String,
    properties: HashMap<String, String>,
) -> CreateItemTemplateData {
    CreateItemTemplateData {
        name,
        description,
        weight: weight as f64,
        stackable,
        max_stack,
        max_durability: max_durability as f64,
        icon,
        category,
        properties,
    }
}

/// Создает UpdateItemTemplateData из параметров запроса
pub fn update_item_template_data_from_request(
    name: Option<String>,
    description: Option<String>,
    weight: Option<f32>,
    stackable: Option<bool>,
    max_stack: Option<i32>,
    max_durability: Option<f32>,
    icon: Option<String>,
    category: Option<String>,
    properties: Option<HashMap<String, String>>,
) -> UpdateItemTemplateData {
    UpdateItemTemplateData {
        name,
        description,
        weight: weight.map(|w| w as f64),
        stackable,
        max_stack,
        max_durability: max_durability.map(|d| d as f64),
        icon,
        category,
        properties,
    }
} 