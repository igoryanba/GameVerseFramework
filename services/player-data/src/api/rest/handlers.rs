use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domain::models::player::{
    CreatePlayerRequest, UpdatePlayerRequest, UpdateCurrencyRequest, AddExperienceRequest, PlayerStatus
};
use crate::domain::models::error::PlayerDataError;
use crate::domain::services::{PlayerServiceTrait, LeaderboardType};
use crate::api::rest::responses::{ApiResponse, ApiError};

/// Application state containing services
#[derive(Clone)]
pub struct AppState {
    pub player_service: Arc<dyn PlayerServiceTrait>,
}

/// Query parameters for player search
#[derive(Debug, Deserialize, Validate)]
pub struct SearchQuery {
    #[validate(length(min = 1, message = "Query cannot be empty"))]
    pub q: String,
    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
    pub limit: Option<u32>,
    #[validate(range(min = 0, message = "Offset cannot be negative"))]
    pub offset: Option<u32>,
}

/// Query parameters for leaderboards
#[derive(Debug, Deserialize, Validate)]
pub struct LeaderboardQuery {
    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100"))]
    pub limit: Option<u32>,
    pub leaderboard_type: Option<String>,
}

/// Request to moderate a player
#[derive(Debug, Deserialize, Validate)]
pub struct ModeratePlayerRequest {
    pub status: PlayerStatus,
    pub reason: Option<String>,
}

// === Player CRUD Handlers ===

/// Create a new player
pub async fn create_player(
    State(state): State<AppState>,
    Json(request): Json<CreatePlayerRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    let player = state.player_service.create_player(request).await?;
    
    Ok(Json(ApiResponse::success(
        serde_json::to_value(player).unwrap(),
        "Player created successfully".to_string(),
    )))
}

/// Get player by ID
pub async fn get_player(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    let player = state.player_service.get_player(id).await?;
    
    Ok(Json(ApiResponse::success(
        serde_json::to_value(player).unwrap(),
        "Player retrieved successfully".to_string(),
    )))
}

/// Get player by username
pub async fn get_player_by_username(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    let player = state.player_service.get_player_by_username(&username).await?;
    
    Ok(Json(ApiResponse::success(
        serde_json::to_value(player).unwrap(),
        "Player retrieved successfully".to_string(),
    )))
}

/// Update player profile
pub async fn update_player(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdatePlayerRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    let player = state.player_service.update_player_profile(id, request).await?;
    
    Ok(Json(ApiResponse::success(
        serde_json::to_value(player).unwrap(),
        "Player updated successfully".to_string(),
    )))
}

/// Get player summary with additional stats
pub async fn get_player_summary(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    let summary = state.player_service.get_player_summary(id).await?;
    
    Ok(Json(ApiResponse::success(
        serde_json::to_value(summary).unwrap(),
        "Player summary retrieved successfully".to_string(),
    )))
}

// === Currency Management ===

/// Update player currency (gold/gems)
pub async fn update_player_currency(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateCurrencyRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    let player = state.player_service.update_player_currency(id, request).await?;
    
    Ok(Json(ApiResponse::success(
        serde_json::to_value(player).unwrap(),
        "Player currency updated successfully".to_string(),
    )))
}

// === Experience Management ===

/// Add experience to player
pub async fn add_experience(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<AddExperienceRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    let player = state.player_service.add_experience(id, request).await?;
    
    Ok(Json(ApiResponse::success(
        serde_json::to_value(player).unwrap(),
        "Experience added successfully".to_string(),
    )))
}

// === Session Management ===

/// Start a new game session
pub async fn start_session(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    let region = params.get("region")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let session_id = state.player_service.start_session(id, region).await?;
    
    let response = serde_json::json!({
        "session_id": session_id,
        "player_id": id
    });
    
    Ok(Json(ApiResponse::success(
        response,
        "Session started successfully".to_string(),
    )))
}

/// End a game session
pub async fn end_session(
    State(state): State<AppState>,
    Path((id, session_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    let duration = state.player_service.end_session(id, session_id).await?;
    
    let response = serde_json::json!({
        "duration_seconds": duration,
        "player_id": id,
        "session_id": session_id
    });
    
    Ok(Json(ApiResponse::success(
        response,
        "Session ended successfully".to_string(),
    )))
}

// === Moderation ===

/// Moderate a player (ban, suspend, etc.)
pub async fn moderate_player(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<ModeratePlayerRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    state.player_service.moderate_player(id, request.status, request.reason).await?;
    
    Ok(Json(ApiResponse::success(
        serde_json::json!({"player_id": id}),
        "Player moderated successfully".to_string(),
    )))
}

// === Search and Leaderboards ===

/// Search players by username/display name
pub async fn search_players(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    // Validate query parameters
    params.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;
    
    let players = state.player_service
        .search_players(&params.q, params.limit, params.offset)
        .await?;
    
    let response = serde_json::json!({
        "players": players,
        "query": params.q,
        "total": players.len()
    });
    
    Ok(Json(ApiResponse::success(
        response,
        "Player search completed successfully".to_string(),
    )))
}

/// Get player leaderboards
pub async fn get_leaderboard(
    State(state): State<AppState>,
    Query(params): Query<LeaderboardQuery>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    // Validate query parameters
    params.validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;
    
    let leaderboard_type = match params.leaderboard_type.as_deref() {
        Some("level") => LeaderboardType::Level,
        Some("experience") => LeaderboardType::Experience,
        Some("playtime") => LeaderboardType::Playtime,
        Some("gold") => LeaderboardType::Gold,
        _ => LeaderboardType::Level, // Default
    };
    
    let limit = params.limit.unwrap_or(10);
    let players = state.player_service.get_leaderboard(leaderboard_type, limit).await?;
    
    let response = serde_json::json!({
        "leaderboard": players,
        "type": params.leaderboard_type.unwrap_or_else(|| "level".to_string()),
        "limit": limit
    });
    
    Ok(Json(ApiResponse::success(
        response,
        "Leaderboard retrieved successfully".to_string(),
    )))
}

// === Health Check ===

/// Health check endpoint
pub async fn health_check() -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    let response = serde_json::json!({
        "status": "healthy",
        "service": "player-data",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    Ok(Json(ApiResponse::success(
        response,
        "Service is healthy".to_string(),
    )))
}

/// Readiness check endpoint
pub async fn readiness_check(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    // You can add database connectivity checks here
    // For now, just check if the service is available
    
    let response = serde_json::json!({
        "status": "ready",
        "service": "player-data",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "dependencies": {
            "database": "connected",
            "player_service": "available"
        }
    });
    
    Ok(Json(ApiResponse::success(
        response,
        "Service is ready".to_string(),
    )))
}

// === Error Handling ===

impl From<PlayerDataError> for ApiError {
    fn from(err: PlayerDataError) -> Self {
        match err {
            PlayerDataError::PlayerNotFound(id) => {
                ApiError::NotFound(format!("Player with ID {} not found", id))
            }
            PlayerDataError::DuplicateUsername(username) => {
                ApiError::Conflict(format!("Username '{}' already exists", username))
            }
            PlayerDataError::DuplicateEmail(email) => {
                ApiError::Conflict(format!("Email '{}' already exists", email))
            }
            PlayerDataError::ValidationError(msg) => {
                ApiError::BadRequest(msg)
            }
            PlayerDataError::InsufficientFunds(currency) => {
                ApiError::BadRequest(format!("Insufficient {} balance", currency))
            }
            PlayerDataError::PlayerInactive(id) => {
                ApiError::Forbidden(format!("Player {} is inactive", id))
            }
            PlayerDataError::InvalidSession(session_id) => {
                ApiError::BadRequest(format!("Invalid session ID: {}", session_id))
            }
            PlayerDataError::DatabaseError(msg) => {
                tracing::error!("Database error: {}", msg);
                ApiError::InternalServerError("Internal database error".to_string())
            }
        }
    }
}