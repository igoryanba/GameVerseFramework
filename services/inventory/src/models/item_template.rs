use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;
use std::collections::HashMap;

/// Модель шаблона предмета в базе данных
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ItemTemplate {
    /// Уникальный идентификатор шаблона
    pub id: Uuid,
    
    /// Название предмета
    pub name: String,
    
    /// Описание предмета
    pub description: String,
    
    /// Вес предмета
    pub weight: f64,
    
    /// Можно ли складывать предметы в стак
    pub stackable: bool,
    
    /// Максимальный размер стака (если предмет можно складывать)
    pub max_stack: i32,
    
    /// Максимальная прочность предмета
    pub max_durability: f64,
    
    /// Иконка предмета
    pub icon: String,
    
    /// Категория предмета
    pub category: String,
    
    /// Дополнительные свойства предмета
    pub properties: Json<HashMap<String, String>>,
    
    /// Дата и время создания шаблона
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Дата и время последнего обновления шаблона
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Данные для создания нового шаблона предмета
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateItemTemplateData {
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

/// Данные для обновления шаблона предмета
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateItemTemplateData {
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

impl ItemTemplate {
    /// Создает новый шаблон предмета
    pub fn new(data: CreateItemTemplateData) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: data.name,
            description: data.description,
            weight: data.weight,
            stackable: data.stackable,
            max_stack: data.max_stack,
            max_durability: data.max_durability,
            icon: data.icon,
            category: data.category,
            properties: Json(data.properties),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Применяет обновления к шаблону предмета
    pub fn apply_update(&mut self, data: UpdateItemTemplateData) {
        if let Some(name) = data.name {
            self.name = name;
        }
        
        if let Some(description) = data.description {
            self.description = description;
        }
        
        if let Some(weight) = data.weight {
            self.weight = weight;
        }
        
        if let Some(stackable) = data.stackable {
            self.stackable = stackable;
        }
        
        if let Some(max_stack) = data.max_stack {
            self.max_stack = max_stack;
        }
        
        if let Some(max_durability) = data.max_durability {
            self.max_durability = max_durability;
        }
        
        if let Some(icon) = data.icon {
            self.icon = icon;
        }
        
        if let Some(category) = data.category {
            self.category = category;
        }
        
        if let Some(properties) = data.properties {
            self.properties = Json(properties);
        }
        
        self.updated_at = chrono::Utc::now();
    }
} 