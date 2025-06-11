use async_trait::async_trait;
use sqlx::{Pool, Postgres, query_as, query};
use uuid::Uuid;
use chrono::Utc;

use crate::domain::models::{User, UserStatus, AuthError};

/// Трейт для репозитория пользователей
#[async_trait]
pub trait IUserRepository: Send + Sync {
    /// Находит пользователя по идентификатору
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, AuthError>;
    
    /// Находит пользователя по имени пользователя
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AuthError>;
    
    /// Находит пользователя по email
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError>;
    
    /// Создает нового пользователя
    async fn create(&self, user: &User) -> Result<User, AuthError>;
    
    /// Обновляет пользователя
    async fn update(&self, user: &User) -> Result<User, AuthError>;
    
    /// Удаляет пользователя
    async fn delete(&self, id: &Uuid) -> Result<(), AuthError>;
    
    /// Обновляет статус пользователя
    async fn update_status(&self, id: &Uuid, status: UserStatus) -> Result<(), AuthError>;
    
    /// Обновляет последний вход пользователя
    async fn update_last_login(&self, id: &Uuid) -> Result<(), AuthError>;
    
    /// Обновляет счетчик неудачных попыток входа
    async fn update_failed_login_attempts(&self, id: &Uuid, attempts: u32, lockout_until: Option<chrono::DateTime<Utc>>) -> Result<(), AuthError>;
}

/// Репозиторий пользователей
pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    /// Создает новый репозиторий пользователей
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    
    /// Получает ссылку на пул соединений
    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, AuthError> {
        let user = query_as!(
            User,
            r#"
            SELECT 
                id, 
                username, 
                email, 
                password_hash, 
                status as "status: _", 
                created_at, 
                updated_at, 
                last_login, 
                roles, 
                permissions, 
                failed_login_attempts, 
                lockout_until,
                totp_secret,
                totp_enabled
            FROM users 
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, AuthError> {
        let user = query_as!(
            User,
            r#"
            SELECT 
                id, 
                username, 
                email, 
                password_hash, 
                status as "status: _", 
                created_at, 
                updated_at, 
                last_login, 
                roles, 
                permissions, 
                failed_login_attempts, 
                lockout_until,
                totp_secret,
                totp_enabled
            FROM users 
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError> {
        let user = query_as!(
            User,
            r#"
            SELECT 
                id, 
                username, 
                email, 
                password_hash, 
                status as "status: _", 
                created_at, 
                updated_at, 
                last_login, 
                roles, 
                permissions, 
                failed_login_attempts, 
                lockout_until,
                totp_secret,
                totp_enabled
            FROM users 
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn create(&self, user: &User) -> Result<User, AuthError> {
        // Проверяем, существует ли пользователь с таким именем
        if let Some(_) = self.find_by_username(&user.username).await? {
            return Err(AuthError::UsernameAlreadyExists);
        }

        // Проверяем, существует ли пользователь с таким email
        if let Some(_) = self.find_by_email(&user.email).await? {
            return Err(AuthError::EmailAlreadyExists);
        }

        let created_user = query_as!(
            User,
            r#"
            INSERT INTO users (
                id, 
                username, 
                email, 
                password_hash, 
                status, 
                created_at, 
                updated_at, 
                roles, 
                permissions, 
                failed_login_attempts, 
                totp_enabled
            ) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING 
                id, 
                username, 
                email, 
                password_hash, 
                status as "status: _", 
                created_at, 
                updated_at, 
                last_login, 
                roles, 
                permissions, 
                failed_login_attempts, 
                lockout_until,
                totp_secret,
                totp_enabled
            "#,
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.status as _,
            user.created_at,
            user.updated_at,
            &user.roles,
            &user.permissions,
            user.failed_login_attempts,
            user.totp_enabled
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created_user)
    }

    async fn update(&self, user: &User) -> Result<User, AuthError> {
        let updated_user = query_as!(
            User,
            r#"
            UPDATE users 
            SET 
                username = $2, 
                email = $3, 
                password_hash = $4, 
                status = $5, 
                updated_at = $6, 
                last_login = $7, 
                roles = $8, 
                permissions = $9, 
                failed_login_attempts = $10, 
                lockout_until = $11,
                totp_secret = $12,
                totp_enabled = $13
            WHERE id = $1
            RETURNING 
                id, 
                username, 
                email, 
                password_hash, 
                status as "status: _", 
                created_at, 
                updated_at, 
                last_login, 
                roles, 
                permissions, 
                failed_login_attempts, 
                lockout_until,
                totp_secret,
                totp_enabled
            "#,
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.status as _,
            user.updated_at,
            user.last_login,
            &user.roles,
            &user.permissions,
            user.failed_login_attempts,
            user.lockout_until,
            user.totp_secret,
            user.totp_enabled
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated_user)
    }

    async fn delete(&self, id: &Uuid) -> Result<(), AuthError> {
        query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_status(&self, id: &Uuid, status: UserStatus) -> Result<(), AuthError> {
        query!(
            "UPDATE users SET status = $2, updated_at = $3 WHERE id = $1",
            id,
            status as _,
            Utc::now()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_last_login(&self, id: &Uuid) -> Result<(), AuthError> {
        query!(
            "UPDATE users SET last_login = $2, failed_login_attempts = 0, lockout_until = NULL, updated_at = $3 WHERE id = $1",
            id,
            Utc::now(),
            Utc::now()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_failed_login_attempts(&self, id: &Uuid, attempts: u32, lockout_until: Option<chrono::DateTime<Utc>>) -> Result<(), AuthError> {
        query!(
            "UPDATE users SET failed_login_attempts = $2, lockout_until = $3, updated_at = $4 WHERE id = $1",
            id,
            attempts as i32,
            lockout_until,
            Utc::now()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
} 