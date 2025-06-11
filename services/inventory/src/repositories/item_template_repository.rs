use crate::models::{ItemTemplate, CreateItemTemplateData, UpdateItemTemplateData};
use crate::repositories::RepositoryResult;
use async_trait::async_trait;
use sqlx::types::Json;
use std::collections::HashMap;
use uuid::Uuid;

/// Интерфейс репозитория для работы с шаблонами предметов
#[async_trait]
pub trait ItemTemplateRepository: Send + Sync {
    /// Создает новый шаблон предмета
    async fn create(&self, data: CreateItemTemplateData) -> RepositoryResult<ItemTemplate>;
    
    /// Находит шаблон предмета по ID
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<ItemTemplate>;
    
    /// Находит шаблон предмета по имени
    async fn find_by_name(&self, name: &str) -> RepositoryResult<Option<ItemTemplate>>;
    
    /// Находит шаблоны предметов по категории
    async fn find_by_category(&self, category: &str) -> RepositoryResult<Vec<ItemTemplate>>;
    
    /// Обновляет шаблон предмета
    async fn update(&self, id: Uuid, data: UpdateItemTemplateData) -> RepositoryResult<ItemTemplate>;
    
    /// Удаляет шаблон предмета
    async fn delete(&self, id: Uuid) -> RepositoryResult<()>;
    
    /// Находит все шаблоны предметов с пагинацией
    async fn find_all(&self, page: i32, page_size: i32) -> RepositoryResult<(Vec<ItemTemplate>, i64)>;
}

/// Реализация репозитория шаблонов предметов на PostgreSQL
pub struct PostgresItemTemplateRepository {
    pool: sqlx::PgPool,
}

impl PostgresItemTemplateRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ItemTemplateRepository for PostgresItemTemplateRepository {
    async fn create(&self, data: CreateItemTemplateData) -> RepositoryResult<ItemTemplate> {
        let template = ItemTemplate::new(data);
        
        sqlx::query!(
            r#"
            INSERT INTO item_templates (
                id, name, description, weight, stackable, max_stack,
                max_durability, icon, category, properties, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            )
            "#,
            template.id,
            template.name,
            template.description,
            template.weight,
            template.stackable,
            template.max_stack,
            template.max_durability,
            template.icon,
            template.category,
            template.properties as _,
            template.created_at,
            template.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(template)
    }
    
    async fn find_by_id(&self, id: Uuid) -> RepositoryResult<ItemTemplate> {
        let template = sqlx::query_as!(
            ItemTemplate,
            r#"
            SELECT 
                id, name, description, weight, stackable, max_stack, max_durability,
                icon, category, properties as "properties: Json<HashMap<String, String>>",
                created_at, updated_at
            FROM item_templates
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| super::RepositoryError::item_template_not_found(id))?;
        
        Ok(template)
    }
    
    async fn find_by_name(&self, name: &str) -> RepositoryResult<Option<ItemTemplate>> {
        let template = sqlx::query_as!(
            ItemTemplate,
            r#"
            SELECT 
                id, name, description, weight, stackable, max_stack, max_durability,
                icon, category, properties as "properties: Json<HashMap<String, String>>",
                created_at, updated_at
            FROM item_templates
            WHERE name = $1
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(template)
    }
    
    async fn find_by_category(&self, category: &str) -> RepositoryResult<Vec<ItemTemplate>> {
        let templates = sqlx::query_as!(
            ItemTemplate,
            r#"
            SELECT 
                id, name, description, weight, stackable, max_stack, max_durability,
                icon, category, properties as "properties: Json<HashMap<String, String>>",
                created_at, updated_at
            FROM item_templates
            WHERE category = $1
            ORDER BY name
            "#,
            category
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(templates)
    }
    
    async fn update(&self, id: Uuid, data: UpdateItemTemplateData) -> RepositoryResult<ItemTemplate> {
        let mut template = self.find_by_id(id).await?;
        template.apply_update(data);
        
        sqlx::query!(
            r#"
            UPDATE item_templates
            SET name = $1, description = $2, weight = $3, stackable = $4, max_stack = $5,
                max_durability = $6, icon = $7, category = $8, properties = $9, updated_at = $10
            WHERE id = $11
            "#,
            template.name,
            template.description,
            template.weight,
            template.stackable,
            template.max_stack,
            template.max_durability,
            template.icon,
            template.category,
            template.properties as _,
            template.updated_at,
            template.id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(template)
    }
    
    async fn delete(&self, id: Uuid) -> RepositoryResult<()> {
        let result = sqlx::query!(
            "DELETE FROM item_templates WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        
        if result.rows_affected() == 0 {
            return Err(super::RepositoryError::item_template_not_found(id));
        }
        
        Ok(())
    }
    
    async fn find_all(&self, page: i32, page_size: i32) -> RepositoryResult<(Vec<ItemTemplate>, i64)> {
        let offset = (page - 1) * page_size;
        
        let templates = sqlx::query_as!(
            ItemTemplate,
            r#"
            SELECT 
                id, name, description, weight, stackable, max_stack, max_durability,
                icon, category, properties as "properties: Json<HashMap<String, String>>",
                created_at, updated_at
            FROM item_templates
            ORDER BY name
            LIMIT $1 OFFSET $2
            "#,
            page_size as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        
        let total_count = sqlx::query!(
            "SELECT COUNT(*) as count FROM item_templates"
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap_or(0);
        
        Ok((templates, total_count))
    }
} 