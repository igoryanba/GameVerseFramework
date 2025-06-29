use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;
use anyhow::Result;
use validator::Validate;

use crate::domain::models::player::{
    Player, PlayerStatus, CreatePlayerRequest, UpdatePlayerRequest, 
    UpdateCurrencyRequest, AddExperienceRequest
};
use crate::domain::models::error::PlayerDataError;
use crate::domain::repositories::PlayerRepositoryTrait;

/// Service interface for player operations
#[async_trait]
pub trait PlayerServiceTrait: Send + Sync {
    /// Create a new player account
    async fn create_player(&self, request: CreatePlayerRequest) -> Result<Player, PlayerDataError>;
    
    /// Get player by ID
    async fn get_player(&self, id: Uuid) -> Result<Player, PlayerDataError>;
    
    /// Get player by username
    async fn get_player_by_username(&self, username: &str) -> Result<Player, PlayerDataError>;
    
    /// Update player profile
    async fn update_player_profile(&self, id: Uuid, request: UpdatePlayerRequest) -> Result<Player, PlayerDataError>;
    
    /// Update player currency (gold and gems)
    async fn update_player_currency(&self, id: Uuid, request: UpdateCurrencyRequest) -> Result<Player, PlayerDataError>;
    
    /// Add experience and handle level-ups
    async fn add_experience(&self, id: Uuid, request: AddExperienceRequest) -> Result<Player, PlayerDataError>;
    
    /// Ban/suspend player
    async fn moderate_player(&self, id: Uuid, status: PlayerStatus, reason: Option<String>) -> Result<(), PlayerDataError>;
    
    /// Start game session
    async fn start_session(&self, id: Uuid, region: Option<String>) -> Result<Uuid, PlayerDataError>;
    
    /// End game session
    async fn end_session(&self, id: Uuid, session_id: Uuid) -> Result<u64, PlayerDataError>;
    
    /// Get player leaderboards
    async fn get_leaderboard(&self, leaderboard_type: LeaderboardType, limit: u32) -> Result<Vec<Player>, PlayerDataError>;
    
    /// Search players
    async fn search_players(&self, query: &str, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Player>, PlayerDataError>;
    
    /// Get player statistics summary
    async fn get_player_summary(&self, id: Uuid) -> Result<PlayerSummary, PlayerDataError>;
}

/// Types of leaderboards
#[derive(Debug, Clone)]
pub enum LeaderboardType {
    Level,
    Experience,
    Playtime,
    Gold,
}

/// Player summary with additional calculated fields
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlayerSummary {
    pub player: Player,
    pub rank_by_level: Option<u32>,
    pub rank_by_experience: Option<u32>,
    pub achievements_count: u32,
    pub is_online: bool,
    pub playtime_hours: f64,
    pub level_progress_percent: f64,
}

/// Service implementation
pub struct PlayerService {
    player_repository: Arc<dyn PlayerRepositoryTrait>,
}

impl PlayerService {
    pub fn new(player_repository: Arc<dyn PlayerRepositoryTrait>) -> Self {
        Self { player_repository }
    }
    
    /// Calculate experience needed for a specific level
    fn calculate_experience_for_level(level: u32) -> u64 {
        let base_exp = 1000_f64;
        (base_exp * (level as f64).powf(1.5)) as u64
    }
    
    /// Handle level-up logic
    async fn handle_level_up(&self, mut player: Player) -> Result<Player, PlayerDataError> {
        let mut leveled_up = false;
        
        while player.experience >= player.experience_to_next_level {
            player.experience -= player.experience_to_next_level;
            player.level += 1;
            player.experience_to_next_level = Self::calculate_experience_for_level(player.level + 1);
            leveled_up = true;
            
            tracing::info!(
                "Player {} leveled up to level {}",
                player.username,
                player.level
            );
        }
        
        if leveled_up {
            // Update the player in database with new level/experience
            let update_request = UpdatePlayerRequest {
                display_name: None,
                avatar_url: None,
                settings: None,
            };
            
            // In a real implementation, you'd need a method to update level/experience specifically
            // For now, we'll return the modified player
        }
        
        Ok(player)
    }
}

#[async_trait]
impl PlayerServiceTrait for PlayerService {
    async fn create_player(&self, request: CreatePlayerRequest) -> Result<Player, PlayerDataError> {
        // Validate the request
        request.validate()
            .map_err(|e| PlayerDataError::ValidationError(e.to_string()))?;
        
        // Additional business logic validations
        if request.username.len() < 3 {
            return Err(PlayerDataError::ValidationError("Username too short".to_string()));
        }
        
        if request.display_name.trim().is_empty() {
            return Err(PlayerDataError::ValidationError("Display name cannot be empty".to_string()));
        }
        
        // Create player through repository
        let player = self.player_repository.create_player(request).await?;
        
        tracing::info!("Created new player: {} ({})", player.username, player.id);
        
        Ok(player)
    }
    
    async fn get_player(&self, id: Uuid) -> Result<Player, PlayerDataError> {
        self.player_repository
            .get_player_by_id(id)
            .await?
            .ok_or(PlayerDataError::PlayerNotFound(id))
    }
    
    async fn get_player_by_username(&self, username: &str) -> Result<Player, PlayerDataError> {
        self.player_repository
            .get_player_by_username(username)
            .await?
            .ok_or(PlayerDataError::PlayerNotFound(Uuid::nil())) // Using nil UUID as placeholder
    }
    
    async fn update_player_profile(&self, id: Uuid, request: UpdatePlayerRequest) -> Result<Player, PlayerDataError> {
        // Validate the request
        request.validate()
            .map_err(|e| PlayerDataError::ValidationError(e.to_string()))?;
        
        // Check if player exists
        let _existing = self.get_player(id).await?;
        
        // Update through repository
        let updated_player = self.player_repository.update_player(id, request).await?;
        
        tracing::info!("Updated player profile: {} ({})", updated_player.username, id);
        
        Ok(updated_player)
    }
    
    async fn update_player_currency(&self, id: Uuid, request: UpdateCurrencyRequest) -> Result<Player, PlayerDataError> {
        // Validate the request
        request.validate()
            .map_err(|e| PlayerDataError::ValidationError(e.to_string()))?;
        
        let gold_change = request.gold_change.unwrap_or(0);
        let gems_change = request.gems_change.unwrap_or(0);
        
        // Validate that changes don't result in negative values
        let current_player = self.get_player(id).await?;
        
        if gold_change < 0 && current_player.gold < (-gold_change) as u64 {
            return Err(PlayerDataError::InsufficientFunds("gold".to_string()));
        }
        
        if gems_change < 0 && current_player.gems < (-gems_change) as u64 {
            return Err(PlayerDataError::InsufficientFunds("gems".to_string()));
        }
        
        let updated_player = self.player_repository
            .update_player_currency(id, gold_change, gems_change)
            .await?;
        
        tracing::info!(
            "Updated currency for player {} ({}): gold {} → {}, gems {} → {}",
            updated_player.username,
            id,
            current_player.gold,
            updated_player.gold,
            current_player.gems,
            updated_player.gems
        );
        
        Ok(updated_player)
    }
    
    async fn add_experience(&self, id: Uuid, request: AddExperienceRequest) -> Result<Player, PlayerDataError> {
        // Validate the request
        request.validate()
            .map_err(|e| PlayerDataError::ValidationError(e.to_string()))?;
        
        // Get current player
        let mut player = self.get_player(id).await?;
        
        // Add experience
        player.add_experience(request.experience);
        
        // Handle potential level-ups
        player = self.handle_level_up(player).await?;
        
        // Update in repository
        let updated_player = self.player_repository
            .add_player_experience(id, request.experience)
            .await?;
        
        tracing::info!(
            "Added {} experience to player {} ({}): level {}, total exp {}",
            request.experience,
            updated_player.username,
            id,
            updated_player.level,
            updated_player.experience
        );
        
        Ok(updated_player)
    }
    
    async fn moderate_player(&self, id: Uuid, status: PlayerStatus, reason: Option<String>) -> Result<(), PlayerDataError> {
        // Check if player exists
        let player = self.get_player(id).await?;
        
        // Update status
        self.player_repository.update_player_status(id, status.clone()).await?;
        
        tracing::warn!(
            "Player moderation action: {} ({}) status changed to {:?}, reason: {:?}",
            player.username,
            id,
            status,
            reason
        );
        
        Ok(())
    }
    
    async fn start_session(&self, id: Uuid, region: Option<String>) -> Result<Uuid, PlayerDataError> {
        // Check if player exists and is active
        let player = self.get_player(id).await?;
        
        if !player.is_active() {
            return Err(PlayerDataError::PlayerInactive(id));
        }
        
        // Generate new session ID
        let session_id = Uuid::new_v4();
        
        // Start session in repository
        self.player_repository.start_session(id, session_id, region.clone()).await?;
        
        // Update last login
        self.player_repository.update_last_login(id).await?;
        
        tracing::info!(
            "Started session for player {} ({}): session {}, region {:?}",
            player.username,
            id,
            session_id,
            region
        );
        
        Ok(session_id)
    }
    
    async fn end_session(&self, id: Uuid, session_id: Uuid) -> Result<u64, PlayerDataError> {
        // Get current player to verify session
        let player = self.get_player(id).await?;
        
        if player.current_session_id != Some(session_id) {
            return Err(PlayerDataError::InvalidSession(session_id));
        }
        
        // Calculate session duration (simplified - in practice you'd track session start time)
        let session_duration = 3600; // Placeholder: 1 hour in seconds
        
        // End session in repository
        self.player_repository.end_session(id, session_duration).await?;
        
        tracing::info!(
            "Ended session for player {} ({}): session {}, duration {}s",
            player.username,
            id,
            session_id,
            session_duration
        );
        
        Ok(session_duration)
    }
    
    async fn get_leaderboard(&self, leaderboard_type: LeaderboardType, limit: u32) -> Result<Vec<Player>, PlayerDataError> {
        match leaderboard_type {
            LeaderboardType::Level => {
                self.player_repository.get_top_players_by_level(limit).await
            }
            LeaderboardType::Experience => {
                self.player_repository.get_top_players_by_experience(limit).await
            }
            LeaderboardType::Playtime => {
                // For now, delegate to level-based leaderboard
                // In a full implementation, you'd add a playtime-specific method
                self.player_repository.get_top_players_by_level(limit).await
            }
            LeaderboardType::Gold => {
                // For now, delegate to level-based leaderboard
                // In a full implementation, you'd add a gold-specific method
                self.player_repository.get_top_players_by_level(limit).await
            }
        }
    }
    
    async fn search_players(&self, query: &str, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<Player>, PlayerDataError> {
        if query.trim().is_empty() {
            return Err(PlayerDataError::ValidationError("Search query cannot be empty".to_string()));
        }
        
        self.player_repository.search_players(query, limit, offset).await
    }
    
    async fn get_player_summary(&self, id: Uuid) -> Result<PlayerSummary, PlayerDataError> {
        let player = self.get_player(id).await?;
        
        // Calculate additional fields
        let is_online = player.current_session_id.is_some();
        let playtime_hours = player.total_playtime as f64 / 3600.0;
        let level_progress_percent = if player.experience_to_next_level > 0 {
            (player.experience as f64 / player.experience_to_next_level as f64) * 100.0
        } else {
            100.0
        };
        
        // In a full implementation, you'd calculate ranks and achievements
        let summary = PlayerSummary {
            player,
            rank_by_level: None, // Would calculate from leaderboard position
            rank_by_experience: None, // Would calculate from leaderboard position
            achievements_count: 0, // Would query achievements service
            is_online,
            playtime_hours,
            level_progress_percent,
        };
        
        Ok(summary)
    }
}