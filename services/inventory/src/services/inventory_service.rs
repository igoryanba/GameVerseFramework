use crate::models::{Inventory, CreateInventoryData, UpdateInventoryData};
use crate::repositories::{InventoryRepository, ItemRepository};
use crate::services::{ServiceResult, ServiceError};
use std::sync::Arc;
use uuid::Uuid;

/// Сервис для работы с инвентарями
pub struct InventoryService<I: InventoryRepository, T: ItemRepository> {
    inventory_repository: Arc<I>,
    item_repository: Arc<T>,
}

impl<I: InventoryRepository, T: ItemRepository> InventoryService<I, T> {
    /// Создает новый сервис для работы с инвентарями
    pub fn new(inventory_repository: Arc<I>, item_repository: Arc<T>) -> Self {
        Self {
            inventory_repository,
            item_repository,
        }
    }
    
    /// Создает новый инвентарь
    pub async fn create_inventory(&self, data: CreateInventoryData) -> ServiceResult<Inventory> {
        // Валидация данных
        if data.max_slots <= 0 {
            return Err(ServiceError::ValidationError("Максимальное количество слотов должно быть больше 0".to_string()));
        }
        
        if data.max_weight <= 0.0 {
            return Err(ServiceError::ValidationError("Максимальный вес должен быть больше 0".to_string()));
        }
        
        if data.name.is_empty() {
            return Err(ServiceError::ValidationError("Название инвентаря не может быть пустым".to_string()));
        }
        
        // Создаем инвентарь
        let inventory = self.inventory_repository.create(data).await?;
        
        Ok(inventory)
    }
    
    /// Получает инвентарь по ID
    pub async fn get_inventory(&self, id: Uuid) -> ServiceResult<Inventory> {
        let inventory = self.inventory_repository.find_by_id(id).await?;
        Ok(inventory)
    }
    
    /// Получает инвентари пользователя
    pub async fn get_owner_inventories(&self, owner_id: &str, owner_type: &str) -> ServiceResult<Vec<Inventory>> {
        let inventories = self.inventory_repository.find_by_owner(owner_id, owner_type).await?;
        Ok(inventories)
    }
    
    /// Обновляет инвентарь
    pub async fn update_inventory(&self, id: Uuid, data: UpdateInventoryData) -> ServiceResult<Inventory> {
        // Валидация данных
        if let Some(max_slots) = data.max_slots {
            if max_slots <= 0 {
                return Err(ServiceError::ValidationError("Максимальное количество слотов должно быть больше 0".to_string()));
            }
        }
        
        if let Some(max_weight) = data.max_weight {
            if max_weight <= 0.0 {
                return Err(ServiceError::ValidationError("Максимальный вес должен быть больше 0".to_string()));
            }
        }
        
        if let Some(ref name) = data.name {
            if name.is_empty() {
                return Err(ServiceError::ValidationError("Название инвентаря не может быть пустым".to_string()));
            }
        }
        
        // Обновляем инвентарь
        let inventory = self.inventory_repository.update(id, data).await?;
        
        Ok(inventory)
    }
    
    /// Удаляет инвентарь
    pub async fn delete_inventory(&self, id: Uuid) -> ServiceResult<()> {
        // Проверяем существование инвентаря
        let _inventory = self.inventory_repository.find_by_id(id).await?;
        
        // Удаляем инвентарь (каскадное удаление предметов обеспечивается базой данных)
        self.inventory_repository.delete(id).await?;
        
        Ok(())
    }
    
    /// Получает все инвентари с пагинацией
    pub async fn get_all_inventories(&self, page: i32, page_size: i32) -> ServiceResult<(Vec<Inventory>, i64)> {
        if page <= 0 || page_size <= 0 {
            return Err(ServiceError::ValidationError("Номер страницы и размер страницы должны быть больше 0".to_string()));
        }
        
        let (inventories, total) = self.inventory_repository.find_all(page, page_size).await?;
        Ok((inventories, total))
    }
    
    /// Проверяет, может ли инвентарь вместить предмет с указанным весом
    pub async fn can_fit_weight(&self, inventory_id: Uuid, weight: f64) -> ServiceResult<bool> {
        // Получаем инвентарь
        let inventory = self.inventory_repository.find_by_id(inventory_id).await?;
        
        // Получаем все предметы в инвентаре
        let items = self.item_repository.find_by_inventory(inventory_id).await?;
        
        // Считаем текущий вес инвентаря
        // Предполагаем, что у нас есть ItemTemplateRepository, и мы можем получить вес каждого предмета
        // Но пока что просто заглушка
        let current_weight = items.iter().map(|_| 0.0).sum::<f64>();
        
        // Проверяем, может ли инвентарь вместить дополнительный вес
        Ok(current_weight + weight <= inventory.max_weight)
    }
    
    /// Проверяет, есть ли в инвентаре свободный слот
    pub async fn has_free_slot(&self, inventory_id: Uuid, slot: i32) -> ServiceResult<bool> {
        // Получаем инвентарь
        let inventory = self.inventory_repository.find_by_id(inventory_id).await?;
        
        // Проверяем, что слот в пределах инвентаря
        if slot < 0 || slot >= inventory.max_slots {
            return Err(ServiceError::ValidationError(format!("Слот {} вне пределов инвентаря (0-{})", slot, inventory.max_slots - 1)));
        }
        
        // Проверяем, занят ли слот
        let item = self.item_repository.find_by_position(inventory_id, slot).await?;
        
        Ok(item.is_none())
    }
    
    /// Находит первый свободный слот в инвентаре
    pub async fn find_free_slot(&self, inventory_id: Uuid) -> ServiceResult<Option<i32>> {
        // Получаем инвентарь
        let inventory = self.inventory_repository.find_by_id(inventory_id).await?;
        
        // Получаем все предметы в инвентаре
        let items = self.item_repository.find_by_inventory(inventory_id).await?;
        
        // Создаем множество занятых слотов
        let occupied_slots: std::collections::HashSet<i32> = items.iter()
            .map(|item| item.position)
            .collect();
        
        // Ищем первый свободный слот
        for slot in 0..inventory.max_slots {
            if !occupied_slots.contains(&slot) {
                return Ok(Some(slot));
            }
        }
        
        // Если свободных слотов нет
        Ok(None)
    }
} 