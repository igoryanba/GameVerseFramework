use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row};
use anyhow::Result;

use crate::domain::models::player::{Player, PlayerStatus, CreatePlayerRequest, UpdatePlayerRequest};
use crate::domain::models::error::PlayerDataError;

/// Repository interface for player data operations
#[async_trait]
pub trait PlayerRepositoryTrait: Send + Sync {
    /// Create a new player
    async fn create_player(&self, request: CreatePlayerRequest) -> Result<Player, PlayerDataError>;
    
    /// Get player by ID
    async fn get_player_by_id(&self, id: Uuid) -> Result<Option<Player>, PlayerDataError>;
    
    /// Get player by username
    async fn get_player_by_username(&self, username: &str) -> Result<Option<Player>, PlayerDataError>;
    
    /// Get player by email
    async fn get_player_by_email(&self, email: &str) -> Result<Option<Player>, PlayerDataError>;
    
    /// Update player profile
    async fn update_player(&self, id: Uuid, request: UpdatePlayerRequest) -> Result<Player, PlayerDataError>;
    
    /// Update player status
    async fn update_player_status(&self, id: Uuid, status: PlayerStatus) -> Result<(), PlayerDataError>;
    
    /// Update player currency
    async fn update_player_currency(&self, id: Uuid, gold_change: i64, gems_change: i64) -> Result<Player, PlayerDataError>;
    
    /// Add experience to player
    async fn add_player_experience(&self, id: Uuid, experience: u64) -> Result<Player, PlayerDataError>;
    
    /// Update last login time
    async fn update_last_login(&self, id: Uuid) -> Result<(), PlayerDataError>;
    
    /// Start player session
    async fn start_session(&self, id: Uuid, session_id: Uuid, region: Option<String>) -> Result<(), PlayerDataError>;
    
    /// End player session
    async fn end_session(&self, id: Uuid, playtime_seconds: u64) -> Result<(), PlayerDataError>;
    
    /// Get players by status
    async fn get_players_by_status(&self, status: PlayerStatus, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Player>, PlayerDataError>;
    
    /// Get player count
    async fn get_player_count(&self) -> Result<u64, PlayerDataError>;
    
    /// Search players by username pattern
    async fn search_players(&self, pattern: &str, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Player>, PlayerDataError>;
    
    /// Delete player (soft delete by setting status to inactive)
    async fn delete_player(&self, id: Uuid) -> Result<(), PlayerDataError>;
    
    /// Get top players by level
    async fn get_top_players_by_level(&self, limit: u32) -> Result<Vec<Player>, PlayerDataError>;
    
    /// Get top players by experience
    async fn get_top_players_by_experience(&self, limit: u32) -> Result<Vec<Player>, PlayerDataError>;
}

/// PostgreSQL implementation of player repository
pub struct PlayerRepository {
    pool: Pool<Postgres>,
}

impl PlayerRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PlayerRepositoryTrait for PlayerRepository {
    async fn create_player(&self, request: CreatePlayerRequest) -> Result<Player, PlayerDataError> {
        // Check if username already exists
        if let Some(_) = self.get_player_by_username(&request.username).await? {
            return Err(PlayerDataError::DuplicateUsername(request.username));
        }
        
        // Check if email already exists
        if let Some(_) = self.get_player_by_email(&request.email).await? {
            return Err(PlayerDataError::DuplicateEmail(request.email));
        }
        
        let player = Player::new(request.username, request.display_name, request.email);
        
        let query = r#"
            INSERT INTO players (
                id, username, display_name, avatar_url, email, level, experience, 
                experience_to_next_level, gold, gems, settings, status, created_at, 
                updated_at, last_login, total_playtime, current_session_id, current_region
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            RETURNING *
        "#;
        
        let row = sqlx::query(query)
            .bind(player.id)
            .bind(&player.username)
            .bind(&player.display_name)
            .bind(&player.avatar_url)
            .bind(&player.email)
            .bind(player.level as i32)
            .bind(player.experience as i64)
            .bind(player.experience_to_next_level as i64)
            .bind(player.gold as i64)
            .bind(player.gems as i64)
            .bind(&player.settings)
            .bind(&player.status)
            .bind(player.created_at)
            .bind(player.updated_at)
            .bind(player.last_login)
            .bind(player.total_playtime as i64)
            .bind(player.current_session_id)
            .bind(&player.current_region)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| PlayerDataError::DatabaseError(e.to_string()))?;
        
        Ok(self.row_to_player(row)?)
    }
    
    async fn get_player_by_id(&self, id: Uuid) -> Result<Option<Player>, PlayerDataError> {
        let query = "SELECT * FROM players WHERE id = $1";
        
        match sqlx::query(query).bind(id).fetch_optional(&self.pool).await {
            Ok(Some(row)) => Ok(Some(self.row_to_player(row)?)),
            Ok(None) => Ok(None),
            Err(e) => Err(PlayerDataError::DatabaseError(e.to_string())),
        }
    }
    
    async fn get_player_by_username(&self, username: &str) -> Result<Option<Player>, PlayerDataError> {
        let query = "SELECT * FROM players WHERE username = $1";
        
        match sqlx::query(query).bind(username).fetch_optional(&self.pool).await {
            Ok(Some(row)) => Ok(Some(self.row_to_player(row)?)),
            Ok(None) => Ok(None),
            Err(e) => Err(PlayerDataError::DatabaseError(e.to_string())),
        }
    }
    
    async fn get_player_by_email(&self, email: &str) -> Result<Option<Player>, PlayerDataError> {
        let query = "SELECT * FROM players WHERE email = $1";
        
        match sqlx::query(query).bind(email).fetch_optional(&self.pool).await {
            Ok(Some(row)) => Ok(Some(self.row_to_player(row)?)),
            Ok(None) => Ok(None),
            Err(e) => Err(PlayerDataError::DatabaseError(e.to_string())),
        }
    }
    
    async fn update_player(&self, id: Uuid, request: UpdatePlayerRequest) -> Result<Player, PlayerDataError> {
        let mut query_parts = Vec::new();
        let mut values: Vec<&(dyn sqlx::Encode<Postgres> + sqlx::Type<Postgres> + Sync)> = Vec::new();
        let mut param_count = 1;
        
        if let Some(ref display_name) = request.display_name {
            query_parts.push(format!("display_name = ${}", param_count));
            values.push(display_name);
            param_count += 1;
        }
        
        if let Some(ref avatar_url) = request.avatar_url {
            query_parts.push(format!("avatar_url = ${}", param_count));
            values.push(avatar_url);
            param_count += 1;
        }
        
        if let Some(ref settings) = request.settings {
            query_parts.push(format!("settings = ${}", param_count));
            values.push(settings);
            param_count += 1;
        }
        
        if query_parts.is_empty() {
            return self.get_player_by_id(id).await?
                .ok_or(PlayerDataError::PlayerNotFound(id));
        }
        
        query_parts.push("updated_at = NOW()".to_string());
        
        let query = format!(
            "UPDATE players SET {} WHERE id = ${} RETURNING *",
            query_parts.join(", "),
            param_count
        );
        
        let mut query_builder = sqlx::query(&query);
        for value in values {
            query_builder = query_builder.bind(value);
        }
        query_builder = query_builder.bind(id);
        
        let row = query_builder
            .fetch_one(&self.pool)
            .await
            .map_err(|e| PlayerDataError::DatabaseError(e.to_string()))?;
        
        Ok(self.row_to_player(row)?)
    }
    
    async fn update_player_status(&self, id: Uuid, status: PlayerStatus) -> Result<(), PlayerDataError> {
        let query = "UPDATE players SET status = $1, updated_at = NOW() WHERE id = $2";
        
        let result = sqlx::query(query)
            .bind(&status)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| PlayerDataError::DatabaseError(e.to_string()))?;
        
        if result.rows_affected() == 0 {
            return Err(PlayerDataError::PlayerNotFound(id));
        }
        
        Ok(())
    }
    
    async fn update_player_currency(&self, id: Uuid, gold_change: i64, gems_change: i64) -> Result<Player, PlayerDataError> {
        let query = r#"
            UPDATE players 
            SET gold = GREATEST(0, gold + $1), 
                gems = GREATEST(0, gems + $2),
                updated_at = NOW() 
            WHERE id = $3 
            RETURNING *
        "#;
        
        let row = sqlx::query(query)
            .bind(gold_change)
            .bind(gems_change)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| PlayerDataError::DatabaseError(e.to_string()))?;
        
        Ok(self.row_to_player(row)?)
    }
    
    async fn add_player_experience(&self, id: Uuid, experience: u64) -> Result<Player, PlayerDataError> {
        // This is a simplified version - in practice, you'd handle level-ups in the service layer
        let query = r#"
            UPDATE players 
            SET experience = experience + $1,
                updated_at = NOW() 
            WHERE id = $2 
            RETURNING *
        "#;
        
        let row = sqlx::query(query)
            .bind(experience as i64)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| PlayerDataError::DatabaseError(e.to_string()))?;
        
        Ok(self.row_to_player(row)?)
    }
    
    async fn update_last_login(&self, id: Uuid) -> Result<(), PlayerDataError> {
        let query = "UPDATE players SET last_login = NOW(), updated_at = NOW() WHERE id = $1";
        
        let result = sqlx::query(query)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| PlayerDataError::DatabaseError(e.to_string()))?;
        
        if result.rows_affected() == 0 {
            return Err(PlayerDataError::PlayerNotFound(id));
        }
        
        Ok(())
    }
    
    async fn start_session(&self, id: Uuid, session_id: Uuid, region: Option<String>) -> Result<(), PlayerDataError> {
        let query = r#"
            UPDATE players 
            SET current_session_id = $1, 
                current_region = $2, 
                last_login = NOW(),
                updated_at = NOW() 
            WHERE id = $3
        "#;
        
        let result = sqlx::query(query)
            .bind(session_id)
            .bind(region)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| PlayerDataError::DatabaseError(e.to_string()))?;
        
        if result.rows_affected() == 0 {
            return Err(PlayerDataError::PlayerNotFound(id));
        }
        
        Ok(())
    }
    
    async fn end_session(&self, id: Uuid, playtime_seconds: u64) -> Result<(), PlayerDataError> {
        let query = r#"
            UPDATE players 
            SET current_session_id = NULL, 
                current_region = NULL,
                total_playtime = total_playtime + $1,
                updated_at = NOW() 
            WHERE id = $2
        "#;
        
        let result = sqlx::query(query)
            .bind(playtime_seconds as i64)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| PlayerDataError::DatabaseError(e.to_string()))?;
        
        if result.rows_affected() == 0 {
            return Err(PlayerDataError::PlayerNotFound(id));
        }
        
        Ok(())
    }
    
    async fn get_players_by_status(&self, status: PlayerStatus, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Player>, PlayerDataError> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        
        let query = "SELECT * FROM players WHERE status = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3";
        
        let rows = sqlx::query(query)
            .bind(&status)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| PlayerDataError::DatabaseError(e.to_string()))?;
        
        let mut players = Vec::new();
        for row in rows {
            players.push(self.row_to_player(row)?);
        }
        
        Ok(players)
    }
    
    async fn get_player_count(&self) -> Result<u64, PlayerDataError> {
        let query = "SELECT COUNT(*) as count FROM players";
        
        let row = sqlx::query(query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| PlayerDataError::DatabaseError(e.to_string()))?;
        
        let count: i64 = row.get("count");
        Ok(count as u64)
    }
    
    async fn search_players(&self, pattern: &str, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Player>, PlayerDataError> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);
        let search_pattern = format!("%{}%", pattern);
        
        let query = r#"
            SELECT * FROM players 
            WHERE username ILIKE $1 OR display_name ILIKE $1 
            ORDER BY username 
            LIMIT $2 OFFSET $3
        "#;
        
        let rows = sqlx::query(query)
            .bind(&search_pattern)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| PlayerDataError::DatabaseError(e.to_string()))?;
        
        let mut players = Vec::new();
        for row in rows {
            players.push(self.row_to_player(row)?);
        }
        
        Ok(players)
    }
    
    async fn delete_player(&self, id: Uuid) -> Result<(), PlayerDataError> {
        // Soft delete by setting status to inactive
        self.update_player_status(id, PlayerStatus::Inactive).await
    }
    
    async fn get_top_players_by_level(&self, limit: u32) -> Result<Vec<Player>, PlayerDataError> {
        let query = r#"
            SELECT * FROM players 
            WHERE status = 'active' 
            ORDER BY level DESC, experience DESC 
            LIMIT $1
        "#;
        
        let rows = sqlx::query(query)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| PlayerDataError::DatabaseError(e.to_string()))?;
        
        let mut players = Vec::new();
        for row in rows {
            players.push(self.row_to_player(row)?);
        }
        
        Ok(players)
    }
    
    async fn get_top_players_by_experience(&self, limit: u32) -> Result<Vec<Player>, PlayerDataError> {
        let query = r#"
            SELECT * FROM players 
            WHERE status = 'active' 
            ORDER BY experience DESC, level DESC 
            LIMIT $1
        "#;
        
        let rows = sqlx::query(query)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| PlayerDataError::DatabaseError(e.to_string()))?;
        
        let mut players = Vec::new();
        for row in rows {
            players.push(self.row_to_player(row)?);
        }
        
        Ok(players)
    }
}

impl PlayerRepository {
    /// Convert database row to Player model
    fn row_to_player(&self, row: sqlx::postgres::PgRow) -> Result<Player, PlayerDataError> {
        Ok(Player {
            id: row.get("id"),
            username: row.get("username"),
            display_name: row.get("display_name"),
            avatar_url: row.get("avatar_url"),
            email: row.get("email"),
            level: row.get::<i32, _>("level") as u32,
            experience: row.get::<i64, _>("experience") as u64,
            experience_to_next_level: row.get::<i64, _>("experience_to_next_level") as u64,
            gold: row.get::<i64, _>("gold") as u64,
            gems: row.get::<i64, _>("gems") as u64,
            settings: row.get("settings"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            last_login: row.get("last_login"),
            total_playtime: row.get::<i64, _>("total_playtime") as u64,
            current_session_id: row.get("current_session_id"),
            current_region: row.get("current_region"),
        })
    }
}