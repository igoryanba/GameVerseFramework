use std::sync::Arc;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode, Request},
    middleware::Next,
    response::{Response, IntoResponse},
    body::Body,
};
use uuid::Uuid;
use tracing::{warn, error};

use crate::domain::services::AuthService;
use crate::domain::models::token::TokenClaims;
use crate::api::rest::responses::ApiResponse;

/// Middleware для проверки аутентификации
pub async fn auth_middleware(
    State(auth_service): State<Arc<AuthService>>,
    headers: HeaderMap,
    mut request: Request<Body>,
    next: Next<Body>,
) -> Result<Response, StatusCode> {
    // Извлекаем токен из заголовка Authorization
    let token = match extract_bearer_token(&headers) {
        Some(token) => token,
        None => {
            warn!("Отсутствует заголовок Authorization");
            return Ok(ApiResponse::<()>::error(
                "MISSING_TOKEN".to_string(),
                "Отсутствует токен аутентификации".to_string(),
            ).into_response());
        }
    };
    
    // Проверяем токен доступа
    match auth_service.verify_access_token(&token).await {
        Ok(claims) => {
            // Добавляем информацию о пользователе в расширения запроса
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Err(auth_error) => {
            warn!("Ошибка проверки токена: {:?}", auth_error);
            Ok(ApiResponse::<()>::from(auth_error).into_response())
        }
    }
}

/// Извлекает Bearer токен из заголовков
fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            if header.starts_with("Bearer ") {
                Some(header[7..].to_string())
            } else {
                None
            }
        })
}

/// Извлекает IP адрес клиента из заголовков
pub fn extract_client_ip(headers: &HeaderMap) -> Option<String> {
    // Пытаемся найти IP в различных заголовках
    headers
        .get("x-forwarded-for")
        .and_then(|header| header.to_str().ok())
        .map(|header| header.split(',').next().unwrap_or("").trim().to_string())
        .filter(|ip| !ip.is_empty())
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|header| header.to_str().ok())
                .map(|header| header.to_string())
        })
}

/// Извлекает User-Agent из заголовков
pub fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get("user-agent")
        .and_then(|header| header.to_str().ok())
        .map(|header| header.to_string())
} 