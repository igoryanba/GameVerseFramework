use crate::models::{Inventory, CreateInventoryData, UpdateInventoryData};
use crate::repositories::RepositoryResult;
use async_trait::async_trait;
use sqlx::types::Json;
use std::collections::HashMap;
use uuid::Uuid;

/// Интерфейс репозитория для работы с инвентарем
#[async_trait]
pub trait InventoryRepository: Send + Sync {
    /// Создает новый инвентарь
    async fn create(&self, data: CreateInventoryData) -> RepositoryResult<Inventory>;
    
    /// Находит инвентарь по ID
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<Inventory>;
    
    /// Находит инвентари по владельцу
    async fn find_by_owner(&self, owner_id: &str, owner_type: &str) -> RepositoryResult<Vec<Inventory>>;
    
    /// Обновляет инвентарь
    async fn update(&self, id: Uuid, data: UpdateInventoryData) -> RepositoryResult<Inventory>;
    
    /// Удаляет инвентарь
    async fn delete(&self, id: Uuid) -> RepositoryResult<()>;
    
    /// Находит все инвентари с пагинацией
    async fn find_all(&self, page: i32, page_size: i32) -> RepositoryResult<(Vec<Inventory>, i64)>;
}

/// Реализация репозитория инвентаря на PostgreSQL
pub struct PostgresInventoryRepository {
    pool: sqlx::PgPool,
}

impl PostgresInventoryRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InventoryRepository for PostgresInventoryRepository {
    async fn create(&self, data: CreateInventoryData) -> RepositoryResult<Inventory> {
        let inventory = Inventory::new(data);
        
        sqlx::query!(
            r#"
            INSERT INTO inventories (
                id, owner_id, owner_type, inventory_type, max_weight, 
                max_slots, name, metadata, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
            )
            "#,
            inventory.id,
            inventory.owner_id,
            inventory.owner_type,
            inventory.inventory_type as _,
            inventory.max_weight,
            inventory.max_slots,
            inventory.name,
            inventory.metadata as _,
            inventory.created_at,
            inventory.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(inventory)
    }
    
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<Inventory> {
        let inventory = sqlx::query_as!(
            Inventory,
            r#"
            SELECT 
                id, owner_id, owner_type, 
                inventory_type as "inventory_type: _", 
                max_weight, max_slots, name, 
                metadata as "metadata: Json<HashMap<String, String>>",
                created_at, updated_at
            FROM inventories
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| super::RepositoryError::inventory_not_found(id))?;
        
        Ok(inventory)
    }
    
    async fn find_by_owner(&self, owner_id: &str, owner_type: &str) -> RepositoryResult<Vec<Inventory>> {
        let inventories = sqlx::query_as!(
            Inventory,
            r#"
            SELECT 
                id, owner_id, owner_type, 
                inventory_type as "inventory_type: _", 
                max_weight, max_slots, name, 
                metadata as "metadata: Json<HashMap<String, String>>",
                created_at, updated_at
            FROM inventories
            WHERE owner_id = $1 AND owner_type = $2
            "#,
            owner_id,
            owner_type
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(inventories)
    }
    
    async fn update(&self, id: Uuid, data: UpdateInventoryData) -> RepositoryResult<Inventory> {
        let mut inventory = self.find_by_id(id).await?;
        inventory.apply_update(data);
        
        sqlx::query!(
            r#"
            UPDATE inventories
            SET max_weight = $1, max_slots = $2, name = $3, metadata = $4, updated_at = $5
            WHERE id = $6
            "#,
            inventory.max_weight,
            inventory.max_slots,
            inventory.name,
            inventory.metadata as _,
            inventory.updated_at,
            inventory.id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(inventory)
    }
    
    async fn delete(&self, id: Uuid) -> RepositoryResult<()> {
        let result = sqlx::query!(
            "DELETE FROM inventories WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        
        if result.rows_affected() == 0 {
            return Err(super::RepositoryError::inventory_not_found(id));
        }
        
        Ok(())
    }
    
    async fn find_all(&self, page: i32, page_size: i32) -> RepositoryResult<(Vec<Inventory>, i64)> {
        let offset = (page - 1) * page_size;
        
        let inventories = sqlx::query_as!(
            Inventory,
            r#"
            SELECT 
                id, owner_id, owner_type, 
                inventory_type as "inventory_type: _", 
                max_weight, max_slots, name, 
                metadata as "metadata: Json<HashMap<String, String>>",
                created_at, updated_at
            FROM inventories
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            page_size as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        
        let total_count = sqlx::query!(
            "SELECT COUNT(*) as count FROM inventories"
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap_or(0);
        
        Ok((inventories, total_count))
    }
} 