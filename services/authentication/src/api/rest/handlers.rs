use std::sync::Arc;
use axum::{
    extract::{Extension, State},
    http::HeaderMap,
    Json,
};
use serde::Deserialize;
use tracing::{info, warn, error};
use validator::Validate;

use crate::domain::models::{
    user::{CreateUserRequest, LoginRequest},
    token::{TokenClaims, TokenResponse},
    error::AuthError,
};
use crate::domain::repositories::IUserRepository;
use crate::domain::services::AuthService;
use crate::api::rest::responses::{
    ApiResponse, 
    RegisterResponse, 
    LoginResponse, 
    UserResponse,
    HealthResponse,
};
use crate::api::rest::middleware::{extract_client_ip, extract_user_agent};

/// Проверка здоровья сервиса
pub async fn health_check() -> ApiResponse<HealthResponse> {
    ApiResponse::success(HealthResponse::healthy())
}

/// Регистрация нового пользователя
pub async fn register(
    State(auth_service): State<Arc<AuthService>>,
    Json(request): Json<CreateUserRequest>,
) -> ApiResponse<RegisterResponse> {
    info!("Запрос регистрации пользователя: {}", request.username);
    
    // Валидация данных
    if let Err(validation_errors) = request.validate() {
        let error_message = format!("Ошибки валидации: {:?}", validation_errors);
        warn!("Ошибка валидации при регистрации: {}", error_message);
        return ApiResponse::error(
            "VALIDATION_ERROR".to_string(),
            error_message,
        );
    }
    
    // Регистрируем пользователя
    match auth_service.register(request).await {
        Ok(user) => {
            info!("Пользователь {} успешно зарегистрирован", user.id);
            ApiResponse::success_with_message(
                RegisterResponse {
                    user: user.into(),
                },
                "Пользователь успешно зарегистрирован".to_string(),
            )
        }
        Err(auth_error) => {
            error!("Ошибка регистрации: {:?}", auth_error);
            auth_error.into()
        }
    }
}

/// Вход в систему
pub async fn login(
    State(auth_service): State<Arc<AuthService>>,
    headers: HeaderMap,
    Json(request): Json<LoginRequest>,
) -> ApiResponse<LoginResponse> {
    info!("Запрос входа для пользователя: {}", request.identifier);
    
    // Валидация данных
    if let Err(validation_errors) = request.validate() {
        let error_message = format!("Ошибки валидации: {:?}", validation_errors);
        warn!("Ошибка валидации при входе: {}", error_message);
        return ApiResponse::error(
            "VALIDATION_ERROR".to_string(),
            error_message,
        );
    }
    
    // Извлекаем метаданные клиента
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);
    
    // Аутентификация
    match auth_service.login(request, ip_address, user_agent).await {
        Ok(tokens) => {
            // Получаем информацию о пользователе для ответа
            match auth_service.get_profile(&tokens.access_token).await {
                Ok(user) => {
                    info!("Пользователь {} успешно вошел в систему", user.id);
                    ApiResponse::success_with_message(
                        LoginResponse {
                            user: user.into(),
                            tokens,
                        },
                        "Успешный вход в систему".to_string(),
                    )
                }
                Err(auth_error) => {
                    error!("Ошибка получения профиля после входа: {:?}", auth_error);
                    auth_error.into()
                }
            }
        }
        Err(auth_error) => {
            error!("Ошибка входа: {:?}", auth_error);
            auth_error.into()
        }
    }
}

/// Обновление токена доступа
#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

pub async fn refresh_token(
    State(auth_service): State<Arc<AuthService>>,
    headers: HeaderMap,
    Json(request): Json<RefreshTokenRequest>,
) -> ApiResponse<TokenResponse> {
    info!("Запрос обновления токена");
    
    // Извлекаем метаданные клиента
    let ip_address = extract_client_ip(&headers);
    let user_agent = extract_user_agent(&headers);
    
    // Обновляем токен
    match auth_service.refresh_token(&request.refresh_token, ip_address, user_agent).await {
        Ok(tokens) => {
            info!("Токен успешно обновлен");
            ApiResponse::success_with_message(
                tokens,
                "Токен успешно обновлен".to_string(),
            )
        }
        Err(auth_error) => {
            error!("Ошибка обновления токена: {:?}", auth_error);
            auth_error.into()
        }
    }
}

/// Выход из системы
pub async fn logout(
    State(auth_service): State<Arc<AuthService>>,
    headers: HeaderMap,
) -> ApiResponse<()> {
    info!("Запрос выхода из системы");
    
    // Извлекаем токен из заголовка
    let token = match extract_bearer_token(&headers) {
        Some(token) => token,
        None => {
            warn!("Отсутствует токен для выхода из системы");
            return ApiResponse::error(
                "MISSING_TOKEN".to_string(),
                "Отсутствует токен аутентификации".to_string(),
            );
        }
    };
    
    // Выход из системы
    match auth_service.logout(&token).await {
        Ok(()) => {
            info!("Пользователь успешно вышел из системы");
            ApiResponse::success_simple("Успешный выход из системы".to_string())
        }
        Err(auth_error) => {
            error!("Ошибка выхода из системы: {:?}", auth_error);
            auth_error.into()
        }
    }
}

/// Получение профиля пользователя
pub async fn get_profile(
    State(auth_service): State<Arc<AuthService>>,
    Extension(claims): Extension<TokenClaims>,
) -> ApiResponse<UserResponse> {
    info!("Запрос профиля пользователя: {}", claims.sub);
    
    // Получаем ID пользователя из claims
    match uuid::Uuid::parse_str(&claims.sub) {
        Ok(user_id) => {
            // Получаем профиль пользователя
            match auth_service.get_user_repo().find_by_id(&user_id).await {
                Ok(user_opt) => {
                    if let Some(user) = user_opt {
                        info!("Профиль пользователя {} успешно получен", user.id);
                        ApiResponse::success(user.into())
                    } else {
                        ApiResponse::from(AuthError::UserNotFound)
                    }
                }
                Err(e) => ApiResponse::from(e)
            }
        }
        Err(_) => {
            error!("Неверный формат ID пользователя в токене: {}", claims.sub);
            ApiResponse::error(
                "INVALID_TOKEN".to_string(),
                "Неверный формат токена".to_string(),
            )
        }
    }
}

/// Вспомогательная функция для извлечения Bearer токена
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