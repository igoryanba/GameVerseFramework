use crate::models::{Item, CreateItemData, UpdateItemData};
use crate::repositories::RepositoryResult;
use async_trait::async_trait;
use sqlx::types::Json;
use std::collections::HashMap;
use uuid::Uuid;

/// Интерфейс репозитория для работы с предметами
#[async_trait]
pub trait ItemRepository: Send + Sync {
    /// Создает новый предмет
    async fn create(&self, data: CreateItemData) -> RepositoryResult<Item>;
    
    /// Находит предмет по ID
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<Item>;
    
    /// Находит предметы по инвентарю
    async fn find_by_inventory(&self, inventory_id: Uuid) -> RepositoryResult<Vec<Item>>;
    
    /// Находит предмет по позиции в инвентаре
    async fn find_by_position(&self, inventory_id: Uuid, position: i32) -> RepositoryResult<Option<Item>>;
    
    /// Обновляет предмет
    async fn update(&self, id: Uuid, data: UpdateItemData) -> RepositoryResult<Item>;
    
    /// Удаляет предмет
    async fn delete(&self, id: Uuid) -> RepositoryResult<()>;
    
    /// Находит все предметы с пагинацией
    async fn find_all(&self, page: i32, page_size: i32) -> RepositoryResult<(Vec<Item>, i64)>;
    
    /// Находит предметы по шаблону
    async fn find_by_template(&self, template_id: Uuid) -> RepositoryResult<Vec<Item>>;
}

/// Реализация репозитория предметов на PostgreSQL
pub struct PostgresItemRepository {
    pool: sqlx::PgPool,
}

impl PostgresItemRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ItemRepository for PostgresItemRepository {
    async fn create(&self, data: CreateItemData) -> RepositoryResult<Item> {
        let item = Item::new(data);
        
        sqlx::query!(
            r#"
            INSERT INTO items (
                id, inventory_id, template_id, position, quantity,
                durability, metadata, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            "#,
            item.id,
            item.inventory_id,
            item.template_id,
            item.position,
            item.quantity,
            item.durability,
            item.metadata as _,
            item.created_at,
            item.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(item)
    }
    
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<Item> {
        let item = sqlx::query_as!(
            Item,
            r#"
            SELECT 
                id, inventory_id, template_id, position, quantity, durability,
                metadata as "metadata: Json<HashMap<String, String>>",
                created_at, updated_at
            FROM items
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| super::RepositoryError::item_not_found(id))?;
        
        Ok(item)
    }
    
    async fn find_by_inventory(&self, inventory_id: Uuid) -> RepositoryResult<Vec<Item>> {
        let items = sqlx::query_as!(
            Item,
            r#"
            SELECT 
                id, inventory_id, template_id, position, quantity, durability,
                metadata as "metadata: Json<HashMap<String, String>>",
                created_at, updated_at
            FROM items
            WHERE inventory_id = $1
            ORDER BY position
            "#,
            inventory_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(items)
    }
    
    async fn find_by_position(&self, inventory_id: Uuid, position: i32) -> RepositoryResult<Option<Item>> {
        let item = sqlx::query_as!(
            Item,
            r#"
            SELECT 
                id, inventory_id, template_id, position, quantity, durability,
                metadata as "metadata: Json<HashMap<String, String>>",
                created_at, updated_at
            FROM items
            WHERE inventory_id = $1 AND position = $2
            "#,
            inventory_id,
            position
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(item)
    }
    
    async fn update(&self, id: Uuid, data: UpdateItemData) -> RepositoryResult<Item> {
        let mut item = self.find_by_id(id).await?;
        item.apply_update(data);
        
        sqlx::query!(
            r#"
            UPDATE items
            SET position = $1, quantity = $2, durability = $3, metadata = $4, updated_at = $5
            WHERE id = $6
            "#,
            item.position,
            item.quantity,
            item.durability,
            item.metadata as _,
            item.updated_at,
            item.id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(item)
    }
    
    async fn delete(&self, id: Uuid) -> RepositoryResult<()> {
        let result = sqlx::query!(
            "DELETE FROM items WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        
        if result.rows_affected() == 0 {
            return Err(super::RepositoryError::item_not_found(id));
        }
        
        Ok(())
    }
    
    async fn find_all(&self, page: i32, page_size: i32) -> RepositoryResult<(Vec<Item>, i64)> {
        let offset = (page - 1) * page_size;
        
        let items = sqlx::query_as!(
            Item,
            r#"
            SELECT 
                id, inventory_id, template_id, position, quantity, durability,
                metadata as "metadata: Json<HashMap<String, String>>",
                created_at, updated_at
            FROM items
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            page_size as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        
        let total_count = sqlx::query!(
            "SELECT COUNT(*) as count FROM items"
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap_or(0);
        
        Ok((items, total_count))
    }
    
    async fn find_by_template(&self, template_id: Uuid) -> RepositoryResult<Vec<Item>> {
        let items = sqlx::query_as!(
            Item,
            r#"
            SELECT 
                id, inventory_id, template_id, position, quantity, durability,
                metadata as "metadata: Json<HashMap<String, String>>",
                created_at, updated_at
            FROM items
            WHERE template_id = $1
            "#,
            template_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(items)
    }
}