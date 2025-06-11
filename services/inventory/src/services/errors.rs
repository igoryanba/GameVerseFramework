use thiserror::Error;
use uuid::Uuid;
use crate::repositories::RepositoryError;

/// Ошибки, которые могут возникнуть при работе с сервисами
#[derive(Error, Debug)]
pub enum ServiceError {
    /// Сущность не найдена
    #[error("Entity not found: {entity_type} with ID {entity_id}")]
    NotFound {
        entity_type: String,
        entity_id: String,
    },
    
    /// Ошибка репозитория
    #[error("Repository error: {0}")]
    RepositoryError(#[from] RepositoryError),
    
    /// Ошибка валидации
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    /// Недостаточно места в инвентаре
    #[error("Not enough space in inventory")]
    NotEnoughSpace,
    
    /// Недостаточно предметов
    #[error("Not enough items: required {required}, available {available}")]
    NotEnoughItems {
        required: i32,
        available: i32,
    },
    
    /// Предмет не подходит для этой операции
    #[error("Item not suitable for this operation: {0}")]
    ItemNotSuitable(String),
    
    /// Слот занят
    #[error("Slot {0} is already occupied")]
    SlotOccupied(i32),
    
    /// Внутренняя ошибка
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl ServiceError {
    /// Создает ошибку "Инвентарь не найден"
    pub fn inventory_not_found(id: Uuid) -> Self {
        Self::NotFound {
            entity_type: "Inventory".to_string(),
            entity_id: id.to_string(),
        }
    }
    
    /// Создает ошибку "Предмет не найден"
    pub fn item_not_found(id: Uuid) -> Self {
        Self::NotFound {
            entity_type: "Item".to_string(),
            entity_id: id.to_string(),
        }
    }
    
    /// Создает ошибку "Шаблон предмета не найден"
    pub fn item_template_not_found(id: Uuid) -> Self {
        Self::NotFound {
            entity_type: "ItemTemplate".to_string(),
            entity_id: id.to_string(),
        }
    }
}

/// Результат операции сервиса
pub type ServiceResult<T> = Result<T, ServiceError>; 