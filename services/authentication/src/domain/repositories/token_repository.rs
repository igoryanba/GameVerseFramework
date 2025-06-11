use async_trait::async_trait;
use sqlx::{Pool, Postgres, query_as, query};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::models::{Token, TokenType, AuthError};

/// Трейт для репозитория токенов
#[async_trait]
pub trait ITokenRepository: Send + Sync {
    /// Находит токен по идентификатору
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Token>, AuthError>;
    
    /// Находит токен по значению
    async fn find_by_token(&self, token: &str) -> Result<Option<Token>, AuthError>;
    
    /// Находит все токены пользователя
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<Token>, AuthError>;
    
    /// Находит все активные токены пользователя
    async fn find_active_by_user_id(&self, user_id: &Uuid, token_type: TokenType) -> Result<Vec<Token>, AuthError>;
    
    /// Создает новый токен
    async fn create(&self, token: &Token) -> Result<Token, AuthError>;
    
    /// Отзывает токен
    async fn revoke(&self, id: &Uuid) -> Result<(), AuthError>;
    
    /// Отзывает все токены пользователя
    async fn revoke_all_by_user_id(&self, user_id: &Uuid, token_type: Option<TokenType>) -> Result<(), AuthError>;
    
    /// Удаляет истекшие токены
    async fn delete_expired(&self, before: DateTime<Utc>) -> Result<u64, AuthError>;
}

/// Репозиторий токенов
pub struct TokenRepository {
    pool: Pool<Postgres>,
}

impl TokenRepository {
    /// Создает новый репозиторий токенов
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ITokenRepository for TokenRepository {
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Token>, AuthError> {
        let token = query_as!(
            Token,
            r#"
            SELECT 
                id, 
                user_id, 
                token_type as "token_type: _", 
                token, 
                created_at, 
                expires_at, 
                revoked, 
                revoked_at, 
                ip_address, 
                user_agent 
            FROM tokens 
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(token)
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<Token>, AuthError> {
        let token = query_as!(
            Token,
            r#"
            SELECT 
                id, 
                user_id, 
                token_type as "token_type: _", 
                token, 
                created_at, 
                expires_at, 
                revoked, 
                revoked_at, 
                ip_address, 
                user_agent 
            FROM tokens 
            WHERE token = $1
            "#,
            token
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(token)
    }

    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Vec<Token>, AuthError> {
        let tokens = query_as!(
            Token,
            r#"
            SELECT 
                id, 
                user_id, 
                token_type as "token_type: _", 
                token, 
                created_at, 
                expires_at, 
                revoked, 
                revoked_at, 
                ip_address, 
                user_agent 
            FROM tokens 
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tokens)
    }

    async fn find_active_by_user_id(&self, user_id: &Uuid, token_type: TokenType) -> Result<Vec<Token>, AuthError> {
        let tokens = query_as!(
            Token,
            r#"
            SELECT 
                id, 
                user_id, 
                token_type as "token_type: _", 
                token, 
                created_at, 
                expires_at, 
                revoked, 
                revoked_at, 
                ip_address, 
                user_agent 
            FROM tokens 
            WHERE 
                user_id = $1 AND 
                token_type = $2 AND 
                revoked = false AND 
                expires_at > $3
            "#,
            user_id,
            token_type as _,
            Utc::now()
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tokens)
    }

    async fn create(&self, token: &Token) -> Result<Token, AuthError> {
        let created_token = query_as!(
            Token,
            r#"
            INSERT INTO tokens (
                id, 
                user_id, 
                token_type, 
                token, 
                created_at, 
                expires_at, 
                revoked, 
                ip_address, 
                user_agent
            ) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING 
                id, 
                user_id, 
                token_type as "token_type: _", 
                token, 
                created_at, 
                expires_at, 
                revoked, 
                revoked_at, 
                ip_address, 
                user_agent
            "#,
            token.id,
            token.user_id,
            token.token_type as _,
            token.token,
            token.created_at,
            token.expires_at,
            token.revoked,
            token.ip_address,
            token.user_agent
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_token)
    }

    async fn revoke(&self, id: &Uuid) -> Result<(), AuthError> {
        query!(
            "UPDATE tokens SET revoked = true, revoked_at = $2 WHERE id = $1",
            id,
            Utc::now()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn revoke_all_by_user_id(&self, user_id: &Uuid, token_type: Option<TokenType>) -> Result<(), AuthError> {
        match token_type {
            Some(token_type) => {
                query!(
                    "UPDATE tokens SET revoked = true, revoked_at = $3 WHERE user_id = $1 AND token_type = $2",
                    user_id,
                    token_type as _,
                    Utc::now()
                )
                .execute(&self.pool)
                .await?;
            }
            None => {
                query!(
                    "UPDATE tokens SET revoked = true, revoked_at = $2 WHERE user_id = $1",
                    user_id,
                    Utc::now()
                )
                .execute(&self.pool)
                .await?;
            }
        }

        Ok(())
    }

    async fn delete_expired(&self, before: DateTime<Utc>) -> Result<u64, AuthError> {
        let result = query!(
            "DELETE FROM tokens WHERE expires_at < $1",
            before
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
} 