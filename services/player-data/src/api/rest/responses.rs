use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Standard API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
    pub timestamp: String,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// Create a successful response
    pub fn success(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    /// Create an error response
    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// API error types
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),
    
    #[error("Too many requests: {0}")]
    TooManyRequests(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

impl ApiError {
    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
    
    /// Get the error code string
    pub fn error_code(&self) -> &'static str {
        match self {
            ApiError::BadRequest(_) => "BAD_REQUEST",
            ApiError::Unauthorized(_) => "UNAUTHORIZED",
            ApiError::Forbidden(_) => "FORBIDDEN",
            ApiError::NotFound(_) => "NOT_FOUND",
            ApiError::Conflict(_) => "CONFLICT",
            ApiError::UnprocessableEntity(_) => "UNPROCESSABLE_ENTITY",
            ApiError::TooManyRequests(_) => "TOO_MANY_REQUESTS",
            ApiError::InternalServerError(_) => "INTERNAL_SERVER_ERROR",
            ApiError::ServiceUnavailable(_) => "SERVICE_UNAVAILABLE",
        }
    }
}

/// Error response body
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorDetails,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_response = ErrorResponse {
            success: false,
            error: ErrorDetails {
                code: self.error_code().to_string(),
                message: self.to_string(),
                details: None,
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        
        (status, Json(error_response)).into_response()
    }
}

/// Success response helpers
pub struct ResponseHelper;

impl ResponseHelper {
    /// Create a paginated response
    pub fn paginated<T>(
        data: Vec<T>,
        page: u32,
        per_page: u32,
        total: u64,
        message: String,
    ) -> ApiResponse<serde_json::Value>
    where
        T: Serialize,
    {
        let response_data = serde_json::json!({
            "items": data,
            "pagination": {
                "page": page,
                "per_page": per_page,
                "total": total,
                "total_pages": (total + per_page as u64 - 1) / per_page as u64
            }
        });
        
        ApiResponse::success(response_data, message)
    }
    
    /// Create a simple success response with message only
    pub fn success_message(message: String) -> ApiResponse<serde_json::Value> {
        ApiResponse::success(serde_json::json!({}), message)
    }
    
    /// Create a response with metadata
    pub fn with_metadata<T>(
        data: T,
        metadata: serde_json::Value,
        message: String,
    ) -> ApiResponse<serde_json::Value>
    where
        T: Serialize,
    {
        let response_data = serde_json::json!({
            "data": data,
            "metadata": metadata
        });
        
        ApiResponse::success(response_data, message)
    }
}

/// Validation error response
#[derive(Debug, Serialize)]
pub struct ValidationErrorResponse {
    pub success: bool,
    pub error: ValidationErrorDetails,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct ValidationErrorDetails {
    pub code: String,
    pub message: String,
    pub validation_errors: Vec<FieldError>,
}

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
    pub rejected_value: Option<serde_json::Value>,
}

impl ValidationErrorResponse {
    pub fn new(field_errors: Vec<FieldError>) -> Self {
        Self {
            success: false,
            error: ValidationErrorDetails {
                code: "VALIDATION_ERROR".to_string(),
                message: "Request validation failed".to_string(),
                validation_errors: field_errors,
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl IntoResponse for ValidationErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

/// Rate limit error response
#[derive(Debug, Serialize)]
pub struct RateLimitErrorResponse {
    pub success: bool,
    pub error: RateLimitErrorDetails,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct RateLimitErrorDetails {
    pub code: String,
    pub message: String,
    pub retry_after: u64, // seconds
    pub limit: u32,
    pub window: u64, // seconds
}

impl RateLimitErrorResponse {
    pub fn new(retry_after: u64, limit: u32, window: u64) -> Self {
        Self {
            success: false,
            error: RateLimitErrorDetails {
                code: "RATE_LIMIT_EXCEEDED".to_string(),
                message: format!("Rate limit exceeded. Try again in {} seconds", retry_after),
                retry_after,
                limit,
                window,
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl IntoResponse for RateLimitErrorResponse {
    fn into_response(self) -> Response {
        (StatusCode::TOO_MANY_REQUESTS, Json(self)).into_response()
    }
}