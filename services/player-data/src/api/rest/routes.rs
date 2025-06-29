use axum::{
    routing::{get, post, put, patch},
    Router,
};
use tower_http::cors::CorsLayer;

use crate::api::rest::handlers::{
    AppState,
    // Player CRUD
    create_player,
    get_player,
    get_player_by_username,
    update_player,
    get_player_summary,
    // Currency & Experience
    update_player_currency,
    add_experience,
    // Sessions
    start_session,
    end_session,
    // Moderation
    moderate_player,
    // Search & Leaderboards
    search_players,
    get_leaderboard,
    // Health checks
    health_check,
    readiness_check,
};

/// Build the REST API router
pub fn build_routes(state: AppState) -> Router {
    Router::new()
        // Health checks
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        
        // Player management routes
        .nest("/api/v1/players", player_routes())
        
        // Administrative routes
        .nest("/api/v1/admin", admin_routes())
        
        // Public/search routes
        .nest("/api/v1/public", public_routes())
        
        // Add CORS layer
        .layer(CorsLayer::permissive())
        
        // Add application state
        .with_state(state)
}

/// Player management routes (authenticated)
fn player_routes() -> Router<AppState> {
    Router::new()
        // Basic CRUD
        .route("/", post(create_player))
        .route("/:id", get(get_player))
        .route("/:id", put(update_player))
        .route("/:id/summary", get(get_player_summary))
        .route("/username/:username", get(get_player_by_username))
        
        // Currency management
        .route("/:id/currency", patch(update_player_currency))
        
        // Experience management  
        .route("/:id/experience", post(add_experience))
        
        // Session management
        .route("/:id/sessions", post(start_session))
        .route("/:id/sessions/:session_id", patch(end_session))
}

/// Administrative routes (admin authentication required)
fn admin_routes() -> Router<AppState> {
    Router::new()
        // Player moderation
        .route("/players/:id/moderate", post(moderate_player))
        
        // Future admin endpoints:
        // .route("/players/bulk-update", post(bulk_update_players))
        // .route("/players/export", get(export_players))
        // .route("/statistics/overview", get(get_admin_statistics))
}

/// Public routes (no authentication required)
fn public_routes() -> Router<AppState> {
    Router::new()
        // Search and leaderboards
        .route("/players/search", get(search_players))
        .route("/leaderboards", get(get_leaderboard))
        
        // Public statistics
        // .route("/statistics", get(get_public_statistics))
}

/// Build router with specific middleware for different route groups
pub fn build_routes_with_middleware(state: AppState) -> Router {
    let app = Router::new()
        // Health checks (no middleware needed)
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        
        // Public routes (rate limiting only)
        .nest("/api/v1/public", 
            public_routes()
                // Add rate limiting middleware here
                // .layer(rate_limiting_layer())
        )
        
        // Player routes (authentication + rate limiting)
        .nest("/api/v1/players",
            player_routes()
                // Add authentication middleware here
                // .layer(auth_middleware())
                // .layer(rate_limiting_layer())
        )
        
        // Admin routes (admin authentication + audit logging)
        .nest("/api/v1/admin",
            admin_routes()
                // Add admin authentication middleware here
                // .layer(admin_auth_middleware())
                // .layer(audit_logging_middleware())
        )
        
        // Global middleware
        .layer(CorsLayer::permissive())
        // .layer(request_logging_middleware())
        // .layer(metrics_middleware())
        
        .with_state(state);
    
    app
}

/// API documentation routes (if using something like utoipa/swagger)
pub fn build_docs_routes() -> Router {
    Router::new()
        // OpenAPI spec
        .route("/api-docs/openapi.json", get(serve_openapi_spec))
        // Swagger UI
        .route("/api-docs/", get(serve_swagger_ui))
        .route("/api-docs/*path", get(serve_swagger_ui))
}

// Placeholder handlers for documentation routes
async fn serve_openapi_spec() -> &'static str {
    // In a real implementation, this would serve the OpenAPI specification
    "OpenAPI spec would be served here"
}

async fn serve_swagger_ui() -> &'static str {
    // In a real implementation, this would serve Swagger UI
    "Swagger UI would be served here"
}

/// Route groups for different API versions
pub fn build_versioned_routes(state: AppState) -> Router {
    Router::new()
        // Version 1 API
        .nest("/api/v1", build_v1_routes())
        
        // Future version 2 API
        // .nest("/api/v2", build_v2_routes())
        
        // Health and docs (versionless)
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        .merge(build_docs_routes())
        
        .with_state(state)
}

fn build_v1_routes() -> Router<AppState> {
    Router::new()
        .nest("/players", player_routes())
        .nest("/admin", admin_routes())
        .nest("/public", public_routes())
}

/// Helper function to create a complete application with all middleware
pub fn create_app(state: AppState) -> Router {
    build_routes_with_middleware(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use hyper::Body;
    use std::sync::Arc;
    use tower::ServiceExt;
    
    // Mock implementations would go here for testing
    // 
    // #[tokio::test]
    // async fn test_health_endpoint() {
    //     let state = create_test_state().await;
    //     let app = create_app(state);
    //     
    //     let request = Request::builder()
    //         .uri("/health")
    //         .body(Body::empty())
    //         .unwrap();
    //     
    //     let response = app.oneshot(request).await.unwrap();
    //     assert_eq!(response.status(), StatusCode::OK);
    // }
}