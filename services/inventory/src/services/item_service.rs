use crate::models::{Item, CreateItemData, UpdateItemData, ItemTemplate, CreateItemTemplateData, UpdateItemTemplateData};
use crate::repositories::{ItemRepository, ItemTemplateRepository, InventoryRepository};
use crate::services::{ServiceResult, ServiceError, InventoryService};
use std::sync::Arc;
use uuid::Uuid;

/// Сервис для работы с предметами
pub struct ItemService<I: ItemRepository, T: ItemTemplateRepository, V: InventoryRepository> {
    item_repository: Arc<I>,
    item_template_repository: Arc<T>,
    inventory_service: Arc<InventoryService<V, I>>,
}

impl<I: ItemRepository, T: ItemTemplateRepository, V: InventoryRepository> ItemService<I, T, V> {
    /// Создает новый сервис для работы с предметами
    pub fn new(
        item_repository: Arc<I>,
        item_template_repository: Arc<T>,
        inventory_service: Arc<InventoryService<V, I>>,
    ) -> Self {
        Self {
            item_repository,
            item_template_repository,
            inventory_service,
        }
    }
    
    // ===== Методы для работы с предметами =====
    
    /// Создает новый предмет в инвентаре
    pub async fn create_item(&self, data: CreateItemData) -> ServiceResult<Item> {
        // Валидация данных
        if data.quantity <= 0 {
            return Err(ServiceError::ValidationError("Количество должно быть больше 0".to_string()));
        }
        
        if data.durability < 0.0 {
            return Err(ServiceError::ValidationError("Прочность не может быть отрицательной".to_string()));
        }
        
        // Проверяем существование шаблона предмета
        let template = self.item_template_repository.find_by_id(data.template_id).await?;
        
        // Проверяем, что у предмета допустимое количество
        if !template.stackable && data.quantity > 1 {
            return Err(ServiceError::ValidationError("Этот предмет нельзя складывать в стак".to_string()));
        }
        
        if template.stackable && data.quantity > template.max_stack {
            return Err(ServiceError::ValidationError(format!("Превышен максимальный размер стака: {}", template.max_stack)));
        }
        
        // Проверяем, что инвентарь может вместить предмет
        let weight = template.weight * (data.quantity as f64);
        if !self.inventory_service.can_fit_weight(data.inventory_id, weight).await? {
            return Err(ServiceError::NotEnoughSpace);
        }
        
        // Проверяем, не занят ли указанный слот
        if !self.inventory_service.has_free_slot(data.inventory_id, data.position).await? {
            return Err(ServiceError::SlotOccupied(data.position));
        }
        
        // Создаем предмет
        let item = self.item_repository.create(data).await?;
        
        Ok(item)
    }
    
    /// Получает предмет по ID
    pub async fn get_item(&self, id: Uuid) -> ServiceResult<Item> {
        let item = self.item_repository.find_by_id(id).await?;
        Ok(item)
    }
    
    /// Получает все предметы в инвентаре
    pub async fn get_inventory_items(&self, inventory_id: Uuid) -> ServiceResult<Vec<Item>> {
        let items = self.item_repository.find_by_inventory(inventory_id).await?;
        Ok(items)
    }
    
    /// Обновляет предмет
    pub async fn update_item(&self, id: Uuid, data: UpdateItemData) -> ServiceResult<Item> {
        // Получаем текущий предмет
        let item = self.item_repository.find_by_id(id).await?;
        
        // Получаем шаблон предмета
        let template = self.item_template_repository.find_by_id(item.template_id).await?;
        
        // Валидация данных
        if let Some(quantity) = data.quantity {
            if quantity <= 0 {
                return Err(ServiceError::ValidationError("Количество должно быть больше 0".to_string()));
            }
            
            if !template.stackable && quantity > 1 {
                return Err(ServiceError::ValidationError("Этот предмет нельзя складывать в стак".to_string()));
            }
            
            if template.stackable && quantity > template.max_stack {
                return Err(ServiceError::ValidationError(format!("Превышен максимальный размер стака: {}", template.max_stack)));
            }
        }
        
        if let Some(durability) = data.durability {
            if durability < 0.0 || durability > template.max_durability {
                return Err(ServiceError::ValidationError(format!("Прочность должна быть в пределах 0-{}", template.max_durability)));
            }
        }
        
        // Если меняется позиция, проверяем, не занята ли она
        if let Some(position) = data.position {
            if position != item.position {
                if !self.inventory_service.has_free_slot(item.inventory_id, position).await? {
                    return Err(ServiceError::SlotOccupied(position));
                }
            }
        }
        
        // Обновляем предмет
        let updated_item = self.item_repository.update(id, data).await?;
        
        Ok(updated_item)
    }
    
    /// Удаляет предмет
    pub async fn delete_item(&self, id: Uuid) -> ServiceResult<()> {
        // Проверяем существование предмета
        let _item = self.item_repository.find_by_id(id).await?;
        
        // Удаляем предмет
        self.item_repository.delete(id).await?;
        
        Ok(())
    }
    
    /// Перемещает предмет в другой инвентарь
    pub async fn move_item(&self, item_id: Uuid, target_inventory_id: Uuid, target_position: i32) -> ServiceResult<Item> {
        // Получаем предмет
        let item = self.item_repository.find_by_id(item_id).await?;
        
        // Если целевой инвентарь тот же самый, используем обновление позиции
        if item.inventory_id == target_inventory_id {
            let update_data = UpdateItemData {
                position: Some(target_position),
                quantity: None,
                durability: None,
                metadata: None,
            };
            return self.update_item(item_id, update_data).await;
        }
        
        // Проверяем существование целевого инвентаря
        let _target_inventory = self.inventory_service.get_inventory(target_inventory_id).await?;
        
        // Проверяем, не занят ли целевой слот
        if !self.inventory_service.has_free_slot(target_inventory_id, target_position).await? {
            return Err(ServiceError::SlotOccupied(target_position));
        }
        
        // Получаем шаблон предмета для проверки веса
        let template = self.item_template_repository.find_by_id(item.template_id).await?;
        
        // Проверяем, может ли целевой инвентарь вместить предмет
        let weight = template.weight * (item.quantity as f64);
        if !self.inventory_service.can_fit_weight(target_inventory_id, weight).await? {
            return Err(ServiceError::NotEnoughSpace);
        }
        
        // Обновляем инвентарь предмета
        let _update_data = UpdateItemData {
            position: Some(target_position),
            quantity: None,
            durability: None,
            metadata: None,
        };
        
        // Вручную меняем инвентарь, потому что update_item не меняет inventory_id
        let mut updated_item = item.clone();
        updated_item.inventory_id = target_inventory_id;
        updated_item.position = target_position;
        
        // Удаляем старый предмет и создаем новый в целевом инвентаре
        self.item_repository.delete(item_id).await?;
        
        let create_data = CreateItemData {
            inventory_id: target_inventory_id,
            template_id: item.template_id,
            position: target_position,
            quantity: item.quantity,
            durability: item.durability,
            metadata: item.metadata.0.clone(),
        };
        
        let new_item = self.item_repository.create(create_data).await?;
        
        Ok(new_item)
    }
    
    /// Стакует предметы одного типа
    pub async fn stack_items(&self, source_item_id: Uuid, target_item_id: Uuid) -> ServiceResult<()> {
        // Получаем исходный и целевой предметы
        let source_item = self.item_repository.find_by_id(source_item_id).await?;
        let target_item = self.item_repository.find_by_id(target_item_id).await?;
        
        // Проверяем, что предметы одного типа
        if source_item.template_id != target_item.template_id {
            return Err(ServiceError::ValidationError("Нельзя объединить предметы разных типов".to_string()));
        }
        
        // Получаем шаблон предмета
        let template = self.item_template_repository.find_by_id(source_item.template_id).await?;
        
        // Проверяем, можно ли стакать предметы
        if !template.stackable {
            return Err(ServiceError::ValidationError("Этот предмет нельзя складывать в стак".to_string()));
        }
        
        // Проверяем, что не превышен максимальный размер стака
        let total_quantity = source_item.quantity + target_item.quantity;
        if total_quantity > template.max_stack {
            return Err(ServiceError::ValidationError(format!("Превышен максимальный размер стака: {}", template.max_stack)));
        }
        
        // Обновляем целевой предмет
        let update_data = UpdateItemData {
            position: None,
            quantity: Some(total_quantity),
            durability: None,
            metadata: None,
        };
        
        self.item_repository.update(target_item_id, update_data).await?;
        
        // Удаляем исходный предмет
        self.item_repository.delete(source_item_id).await?;
        
        Ok(())
    }
    
    /// Разделяет стак предметов
    pub async fn split_stack(&self, item_id: Uuid, quantity: i32, new_position: i32) -> ServiceResult<Item> {
        // Получаем предмет
        let item = self.item_repository.find_by_id(item_id).await?;
        
        // Проверяем, что количество предметов для разделения корректно
        if quantity <= 0 || quantity >= item.quantity {
            return Err(ServiceError::ValidationError("Количество для разделения должно быть больше 0 и меньше текущего количества".to_string()));
        }
        
        // Получаем шаблон предмета
        let template = self.item_template_repository.find_by_id(item.template_id).await?;
        
        // Проверяем, можно ли стакать предметы
        if !template.stackable {
            return Err(ServiceError::ValidationError("Этот предмет нельзя разделить, так как он не стакается".to_string()));
        }
        
        // Проверяем, не занят ли новый слот
        if !self.inventory_service.has_free_slot(item.inventory_id, new_position).await? {
            return Err(ServiceError::SlotOccupied(new_position));
        }
        
        // Уменьшаем количество в исходном предмете
        let update_data = UpdateItemData {
            position: None,
            quantity: Some(item.quantity - quantity),
            durability: None,
            metadata: None,
        };
        
        self.item_repository.update(item_id, update_data).await?;
        
        // Создаем новый предмет
        let create_data = CreateItemData {
            inventory_id: item.inventory_id,
            template_id: item.template_id,
            position: new_position,
            quantity,
            durability: item.durability,
            metadata: item.metadata.0.clone(),
        };
        
        let new_item = self.item_repository.create(create_data).await?;
        
        Ok(new_item)
    }
    
    // ===== Методы для работы с шаблонами предметов =====
    
    /// Создает новый шаблон предмета
    pub async fn create_item_template(&self, data: CreateItemTemplateData) -> ServiceResult<ItemTemplate> {
        // Валидация данных
        if data.name.is_empty() {
            return Err(ServiceError::ValidationError("Название предмета не может быть пустым".to_string()));
        }
        
        if data.weight < 0.0 {
            return Err(ServiceError::ValidationError("Вес не может быть отрицательным".to_string()));
        }
        
        if data.max_durability <= 0.0 {
            return Err(ServiceError::ValidationError("Максимальная прочность должна быть больше 0".to_string()));
        }
        
        if data.stackable && data.max_stack <= 1 {
            return Err(ServiceError::ValidationError("Максимальный размер стака должен быть больше 1".to_string()));
        }
        
        // Проверяем, существует ли уже шаблон с таким именем
        if let Some(_) = self.item_template_repository.find_by_name(&data.name).await? {
            return Err(ServiceError::ValidationError(format!("Шаблон предмета с именем '{}' уже существует", data.name)));
        }
        
        // Создаем шаблон предмета
        let template = self.item_template_repository.create(data).await?;
        
        Ok(template)
    }
    
    /// Получает шаблон предмета по ID
    pub async fn get_item_template(&self, id: Uuid) -> ServiceResult<ItemTemplate> {
        let template = self.item_template_repository.find_by_id(id).await?;
        Ok(template)
    }
    
    /// Получает шаблон предмета по имени
    pub async fn get_item_template_by_name(&self, name: &str) -> ServiceResult<Option<ItemTemplate>> {
        let template = self.item_template_repository.find_by_name(name).await?;
        Ok(template)
    }
    
    /// Получает шаблоны предметов по категории
    pub async fn get_item_templates_by_category(&self, category: &str) -> ServiceResult<Vec<ItemTemplate>> {
        let templates = self.item_template_repository.find_by_category(category).await?;
        Ok(templates)
    }
    
    /// Получает все шаблоны предметов с пагинацией
    pub async fn get_all_item_templates(&self, page: i32, page_size: i32) -> ServiceResult<(Vec<ItemTemplate>, i64)> {
        if page <= 0 || page_size <= 0 {
            return Err(ServiceError::ValidationError("Номер страницы и размер страницы должны быть больше 0".to_string()));
        }
        
        let (templates, total) = self.item_template_repository.find_all(page, page_size).await?;
        Ok((templates, total))
    }
    
    /// Обновляет шаблон предмета
    pub async fn update_item_template(&self, id: Uuid, data: UpdateItemTemplateData) -> ServiceResult<ItemTemplate> {
        // Валидация данных
        if let Some(ref name) = data.name {
            if name.is_empty() {
                return Err(ServiceError::ValidationError("Название предмета не может быть пустым".to_string()));
            }
            
            // Проверяем, существует ли уже шаблон с таким именем
            if let Some(existing) = self.item_template_repository.find_by_name(name).await? {
                if existing.id != id {
                    return Err(ServiceError::ValidationError(format!("Шаблон предмета с именем '{}' уже существует", name)));
                }
            }
        }
        
        if let Some(weight) = data.weight {
            if weight < 0.0 {
                return Err(ServiceError::ValidationError("Вес не может быть отрицательным".to_string()));
            }
        }
        
        if let Some(max_durability) = data.max_durability {
            if max_durability <= 0.0 {
                return Err(ServiceError::ValidationError("Максимальная прочность должна быть больше 0".to_string()));
            }
        }
        
        if let Some(stackable) = data.stackable {
            if stackable {
                if let Some(max_stack) = data.max_stack {
                    if max_stack <= 1 {
                        return Err(ServiceError::ValidationError("Максимальный размер стака должен быть больше 1".to_string()));
                    }
                }
            }
        }
        
        // Обновляем шаблон предмета
        let template = self.item_template_repository.update(id, data).await?;
        
        Ok(template)
    }
    
    /// Удаляет шаблон предмета
    pub async fn delete_item_template(&self, id: Uuid) -> ServiceResult<()> {
        // Проверяем существование шаблона
        let _template = self.item_template_repository.find_by_id(id).await?;
        
        // Проверяем, используется ли шаблон в предметах
        let items = self.item_repository.find_by_template(id).await?;
        if !items.is_empty() {
            return Err(ServiceError::ValidationError(format!("Нельзя удалить шаблон, так как он используется в {} предметах", items.len())));
        }
        
        // Удаляем шаблон
        self.item_template_repository.delete(id).await?;
        
        Ok(())
    }
} 