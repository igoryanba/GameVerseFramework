use thiserror::Error;
use uuid::Uuid;

/// Ошибки, которые могут возникнуть при работе с репозиториями
#[derive(Error, Debug)]
pub enum RepositoryError {
    /// Сущность не найдена
    #[error("Entity not found: {entity_type} with ID {entity_id}")]
    NotFound {
        entity_type: String,
        entity_id: String,
    },
    
    /// Ошибка базы данных
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    /// Ошибка валидации данных
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    /// Ошибка кэша
    #[error("Cache error: {0}")]
    CacheError(String),
    
    /// Дубликат уникального ключа
    #[error("Duplicate key: {0}")]
    DuplicateKey(String),
    
    /// Внутренняя ошибка
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl RepositoryError {
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

/// Результат операции репозитория
pub type RepositoryResult<T> = Result<T, RepositoryError>; 