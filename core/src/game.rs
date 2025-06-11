//! # Игровая логика GameVerse
//!
//! Основные компоненты игровой логики и состояния

use std::collections::HashMap;

/// Менеджер игровых объектов
#[derive(Debug)]
pub struct GameObjectManager {
    /// Активные игровые объекты
    objects: HashMap<u64, GameObject>,
    /// Счетчик идентификаторов
    next_id: u64,
}

/// Игровой объект
#[derive(Debug, Clone)]
pub struct GameObject {
    /// Уникальный идентификатор
    pub id: u64,
    /// Название объекта
    pub name: String,
    /// Позиция объекта
    pub position: (f64, f64, f64),
    /// Активен ли объект
    pub active: bool,
}

impl GameObjectManager {
    /// Создать новый менеджер
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            next_id: 1,
        }
    }
    
    /// Создать игровой объект
    pub fn create_object(&mut self, name: String, position: (f64, f64, f64)) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        
        let object = GameObject {
            id,
            name: name.clone(),
            position,
            active: true,
        };
        
        self.objects.insert(id, object);
        tracing::debug!("Created game object: {} (id: {})", name, id);
        
        id
    }
    
    /// Получить объект по ID
    pub fn get_object(&self, id: u64) -> Option<&GameObject> {
        self.objects.get(&id)
    }
    
    /// Удалить объект
    pub fn remove_object(&mut self, id: u64) -> Option<GameObject> {
        let removed = self.objects.remove(&id);
        if removed.is_some() {
            tracing::debug!("Removed game object: {}", id);
        }
        removed
    }
    
    /// Получить количество активных объектов
    pub fn active_count(&self) -> usize {
        self.objects.values().filter(|obj| obj.active).count()
    }
}

impl Default for GameObjectManager {
    fn default() -> Self {
        Self::new()
    }
} 