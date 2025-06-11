use std::sync::Arc;
use std::time::Duration;
use anyhow::Result;
use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use tokio::sync::Mutex;
use tracing::{info, warn, error};
use uuid::Uuid;
use bcrypt;
use serde_json;
use redis::aio::MultiplexedConnection;

use crate::domain::models::{
    user::{User, CreateUserRequest, LoginRequest, UserStatus},
    token::{Token, TokenClaims, TokenType, TokenResponse},
    error::AuthError,
};
use crate::domain::repositories::{IUserRepository, ITokenRepository, UserRepository, TokenRepository};
use crate::config::AuthConfig;

/// Сервис аутентификации
pub struct AuthService {
    user_repo: Arc<UserRepository>,
    token_repo: Arc<TokenRepository>,
    redis_client: Arc<Mutex<MultiplexedConnection>>,
    config: AuthConfig,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl AuthService {
    /// Создает новый экземпляр сервиса аутентификации
    pub fn new(
        user_repo: Arc<UserRepository>,
        redis_client: Arc<Mutex<MultiplexedConnection>>,
        config: AuthConfig,
    ) -> Result<Self> {
        let encoding_key = EncodingKey::from_secret(config.jwt_secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_bytes());
        
        let token_repo = Arc::new(TokenRepository::new(user_repo.get_pool().clone()));
        
        Ok(Self {
            user_repo,
            token_repo,
            redis_client,
            config,
            encoding_key,
            decoding_key,
        })
    }
    
    /// Получает ссылку на репозиторий пользователей
    pub fn get_user_repo(&self) -> &Arc<UserRepository> {
        &self.user_repo
    }
    
    /// Получает ссылку на репозиторий токенов
    pub fn get_token_repo(&self) -> &Arc<TokenRepository> {
        &self.token_repo
    }
    
    /// Регистрирует нового пользователя
    pub async fn register(&self, request: CreateUserRequest) -> Result<User, AuthError> {
        info!("Регистрация пользователя: {}", request.username);
        
        // Проверяем, что пользователь с таким именем или email не существует
        if let Ok(_) = self.user_repo.find_by_username(&request.username).await {
            warn!("Попытка регистрации с существующим именем пользователя: {}", request.username);
            return Err(AuthError::UserAlreadyExists);
        }
        
        if let Ok(_) = self.user_repo.find_by_email(&request.email).await {
            warn!("Попытка регистрации с существующим email: {}", request.email);
            return Err(AuthError::EmailAlreadyExists);
        }
        
        // Хешируем пароль
        let password_hash = self.hash_password(&request.password)?;
        
        // Создаем пользователя
        let mut user = User::new(request.username, request.email, password_hash);
        user.status = UserStatus::Active; // Автоматически активируем пользователя
        
        // Сохраняем в базе данных
        let created_user = self.user_repo.create(&user).await
            .map_err(|e| {
                error!("Ошибка создания пользователя: {}", e);
                AuthError::DatabaseError(e.to_string())
            })?;
        
        info!("Пользователь успешно зарегистрирован: {}", created_user.id);
        Ok(created_user)
    }
    
    /// Аутентификация пользователя
    pub async fn login(&self, request: LoginRequest, ip_address: Option<String>, user_agent: Option<String>) -> Result<TokenResponse, AuthError> {
        info!("Попытка входа для пользователя: {}", request.identifier);
        
        // Ищем пользователя по username или email
        let mut user = self.find_user_by_identifier(&request.identifier).await?;
        
        // Проверяем, не заблокирован ли пользователь
        if user.is_locked() {
            warn!("Попытка входа заблокированного пользователя: {}", user.id);
            return Err(AuthError::UserLocked);
        }
        
        // Проверяем, активен ли пользователь
        if !user.is_active() {
            warn!("Попытка входа неактивного пользователя: {}", user.id);
            return Err(AuthError::UserInactive);
        }
        
        // Проверяем пароль
        if !self.verify_password(&request.password, &user.password_hash)? {
            warn!("Неверный пароль для пользователя: {}", user.id);
            
            // Увеличиваем счетчик неудачных попыток
            user.increment_failed_login(
                self.config.max_failed_attempts,
                Duration::from_secs(self.config.lockout_duration_seconds),
            );
            
            self.user_repo.update(&user).await
                .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
            
            return Err(AuthError::InvalidCredentials);
        }
        
        // Проверяем TOTP код, если включена двухфакторная аутентификация
        if user.totp_enabled {
            if let Some(totp_code) = request.totp_code {
                if !self.verify_totp(&user, &totp_code)? {
                    warn!("Неверный TOTP код для пользователя: {}", user.id);
                    return Err(AuthError::InvalidTotpCode);
                }
            } else {
                warn!("TOTP код не предоставлен для пользователя с включенной 2FA: {}", user.id);
                return Err(AuthError::TotpCodeRequired);
            }
        }
        
        // Обновляем время последнего входа
        user.update_last_login();
        self.user_repo.update(&user).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        
        // Создаем токены
        let tokens = self.create_tokens(&user, ip_address, user_agent).await?;
        
        info!("Успешный вход пользователя: {}", user.id);
        Ok(tokens)
    }
    
    /// Обновляет токен доступа используя refresh токен
    pub async fn refresh_token(&self, refresh_token: &str, ip_address: Option<String>, user_agent: Option<String>) -> Result<TokenResponse, AuthError> {
        info!("Обновление токена доступа");
        
        // Проверяем refresh токен
        let claims = self.verify_token(refresh_token)?;
        
        if claims.token_type != TokenType::Refresh.to_string() {
            warn!("Попытка использовать не refresh токен для обновления");
            return Err(AuthError::InvalidToken);
        }
        
        // Проверяем, что токен есть в базе данных и активен
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AuthError::InvalidToken)?;
        
        let token_opt = self.token_repo.find_by_token(refresh_token).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        
        let token = token_opt.ok_or(AuthError::InvalidToken)?;
        
        if !token.is_active() {
            warn!("Попытка использовать неактивный refresh токен");
            return Err(AuthError::InvalidToken);
        }
        
        // Получаем пользователя
        let user_opt = self.user_repo.find_by_id(&user_id).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        
        let user = user_opt.ok_or(AuthError::UserNotFound)?;
        
        if !user.is_active() {
            warn!("Попытка обновления токена для неактивного пользователя: {}", user.id);
            return Err(AuthError::UserInactive);
        }
        
        // Отзываем старый refresh токен
        self.revoke_token(refresh_token).await?;
        
        // Создаем новые токены
        let tokens = self.create_tokens(&user, ip_address, user_agent).await?;
        
        info!("Токен успешно обновлен для пользователя: {}", user.id);
        Ok(tokens)
    }
    
    /// Выход из системы (отзыв токенов)
    pub async fn logout(&self, access_token: &str) -> Result<(), AuthError> {
        info!("Выход из системы");
        
        let claims = self.verify_token(access_token)?;
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AuthError::InvalidToken)?;
        
        // Отзываем все токены пользователя
        self.revoke_all_user_tokens(user_id).await?;
        
        // Удаляем токен из Redis кеша
        self.remove_token_from_cache(access_token).await?;
        
        info!("Пользователь {} успешно вышел из системы", user_id);
        Ok(())
    }
    
    /// Проверяет токен доступа
    pub async fn verify_access_token(&self, token: &str) -> Result<TokenClaims, AuthError> {
        // Сначала проверяем кеш Redis
        if let Ok(cached_claims) = self.get_token_from_cache(token).await {
            return Ok(cached_claims);
        }
        
        // Если в кеше нет, проверяем JWT
        let claims = self.verify_token(token)?;
        
        if claims.token_type != TokenType::Access.to_string() {
            return Err(AuthError::InvalidToken);
        }
        
        // Сохраняем в кеш для последующих запросов
        self.cache_token(token, &claims).await?;
        
        Ok(claims)
    }
    
    /// Получает профиль пользователя по токену
    pub async fn get_profile(&self, access_token: &str) -> Result<User, AuthError> {
        let claims = self.verify_access_token(access_token).await?;
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AuthError::InvalidToken)?;
        
        let user_opt = self.user_repo.find_by_id(&user_id).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        
        user_opt.ok_or(AuthError::UserNotFound)
    }
    
    // Приватные методы
    
    async fn find_user_by_identifier(&self, identifier: &str) -> Result<User, AuthError> {
        // Пытаемся найти по username
        if let Ok(user_opt) = self.user_repo.find_by_username(identifier).await {
            if let Some(user) = user_opt {
                return Ok(user);
            }
        }
        
        // Пытаемся найти по email
        if let Ok(user_opt) = self.user_repo.find_by_email(identifier).await {
            if let Some(user) = user_opt {
                return Ok(user);
            }
        }
        
        Err(AuthError::UserNotFound)
    }
    
    fn hash_password(&self, password: &str) -> Result<String, AuthError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|e| {
                error!("Ошибка хеширования пароля: {}", e);
                AuthError::InternalError("Ошибка хеширования пароля".to_string())
            })
    }
    
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AuthError> {
        bcrypt::verify(password, hash)
            .map_err(|e| {
                error!("Ошибка проверки пароля: {}", e);
                AuthError::InternalError("Ошибка проверки пароля".to_string())
            })
    }
    
    fn verify_totp(&self, user: &User, code: &str) -> Result<bool, AuthError> {
        // Упрощенная реализация TOTP - в реальности используйте библиотеку типа totp-rs
        // Для демонстрации просто проверяем что код состоит из 6 цифр
        if code.len() == 6 && code.chars().all(|c| c.is_ascii_digit()) {
            // В реальной реализации здесь должна быть проверка TOTP кода
            // с использованием секрета пользователя
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    async fn create_tokens(&self, user: &User, ip_address: Option<String>, user_agent: Option<String>) -> Result<TokenResponse, AuthError> {
        let now = Utc::now();
        let access_expires_at = now + chrono::Duration::seconds(self.config.access_token_lifetime_seconds);
        let refresh_expires_at = now + chrono::Duration::seconds(self.config.refresh_token_lifetime_seconds);
        
        // Создаем access token
        let access_claims = TokenClaims {
            jti: Uuid::new_v4().to_string(),
            iat: now.timestamp(),
            exp: access_expires_at.timestamp(),
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
            sub: user.id.to_string(),
            token_type: TokenType::Access.to_string(),
            permissions: user.permissions.clone(),
        };
        
        let access_token = encode(&Header::default(), &access_claims, &self.encoding_key)
            .map_err(|e| {
                error!("Ошибка создания access token: {}", e);
                AuthError::InternalError("Ошибка создания access token".to_string())
            })?;
        
        // Создаем refresh token
        let refresh_claims = TokenClaims {
            jti: Uuid::new_v4().to_string(),
            iat: now.timestamp(),
            exp: refresh_expires_at.timestamp(),
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
            sub: user.id.to_string(),
            token_type: TokenType::Refresh.to_string(),
            permissions: vec![], // refresh токены не содержат разрешений
        };
        
        let refresh_token = encode(&Header::default(), &refresh_claims, &self.encoding_key)
            .map_err(|e| {
                error!("Ошибка создания refresh token: {}", e);
                AuthError::InternalError("Ошибка создания refresh token".to_string())
            })?;
        
        // Сохраняем токены в базе данных
        let access_token_model = Token::new(
            user.id,
            TokenType::Access,
            access_token.clone(),
            access_expires_at,
            ip_address.clone(),
            user_agent.clone(),
        );
        
        let refresh_token_model = Token::new(
            user.id,
            TokenType::Refresh,
            refresh_token.clone(),
            refresh_expires_at,
            ip_address,
            user_agent,
        );
        
        self.token_repo.create(&access_token_model).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        
        self.token_repo.create(&refresh_token_model).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        
        Ok(TokenResponse::new(
            access_token,
            refresh_token,
            self.config.access_token_lifetime_seconds,
        ))
    }
    
    fn verify_token(&self, token: &str) -> Result<TokenClaims, AuthError> {
        let validation = Validation::new(Algorithm::HS256);
        
        decode::<TokenClaims>(token, &self.decoding_key, &validation)
            .map(|token_data| token_data.claims)
            .map_err(|e| {
                warn!("Ошибка проверки токена: {}", e);
                AuthError::InvalidToken
            })
    }
    
    async fn revoke_token(&self, token: &str) -> Result<(), AuthError> {
        let token_opt = self.token_repo.find_by_token(token).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        
        let mut token_model = token_opt.ok_or(AuthError::InvalidToken)?;
        
        token_model.revoke();
        
        // Пока у нас нет метода update в TokenRepository, используем revoke
        self.token_repo.revoke(&token_model.id).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        
        // Удаляем из кеша
        self.remove_token_from_cache(token).await?;
        
        Ok(())
    }
    
    async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<(), AuthError> {
        // Отзываем Access токены
        let access_tokens = self.token_repo.find_active_by_user_id(&user_id, TokenType::Access).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        
        for token in access_tokens {
            self.token_repo.revoke(&token.id).await
                .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
            
            // Удаляем из кеша
            self.remove_token_from_cache(&token.token).await?;
        }
        
        // Отзываем Refresh токены  
        let refresh_tokens = self.token_repo.find_active_by_user_id(&user_id, TokenType::Refresh).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        
        for token in refresh_tokens {
            self.token_repo.revoke(&token.id).await
                .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
            
            // Удаляем из кеша
            self.remove_token_from_cache(&token.token).await?;
        }
        
        Ok(())
    }
    
    async fn cache_token(&self, token: &str, claims: &TokenClaims) -> Result<(), AuthError> {
        use redis::AsyncCommands;
        
        let mut redis_conn = self.redis_client.lock().await;
        let serialized_claims = serde_json::to_string(claims)
            .map_err(|e| {
                error!("Ошибка сериализации claims: {}", e);
                AuthError::InternalError("Ошибка сериализации claims".to_string())
            })?;
        
        let ttl = claims.exp - Utc::now().timestamp();
        if ttl > 0 {
            let _: () = redis_conn.set_ex(
                format!("token:{}", token),
                serialized_claims,
                ttl as usize,
            )
            .await
            .map_err(|e| {
                error!("Ошибка кеширования токена: {}", e);
                AuthError::InternalError("Ошибка кеширования токена".to_string())
            })?;
        }
        
        Ok(())
    }
    
    async fn get_token_from_cache(&self, token: &str) -> Result<TokenClaims, AuthError> {
        use redis::AsyncCommands;
        
        let mut redis_conn = self.redis_client.lock().await;
        
        let cached_data: Option<String> = redis_conn.get(format!("token:{}", token))
            .await
            .map_err(|e| {
                error!("Ошибка получения токена из кеша: {}", e);
                AuthError::InternalError("Ошибка получения токена из кеша".to_string())
            })?;
        
        match cached_data {
            Some(data) => {
                serde_json::from_str(&data)
                    .map_err(|e| {
                        error!("Ошибка десериализации claims: {}", e);
                        AuthError::InternalError("Ошибка десериализации claims".to_string())
                    })
            }
            None => Err(AuthError::InvalidToken)
        }
    }
    
    async fn remove_token_from_cache(&self, token: &str) -> Result<(), AuthError> {
        use redis::AsyncCommands;
        
        let mut redis_conn = self.redis_client.lock().await;
        
        let _: () = redis_conn.del(format!("token:{}", token))
            .await
            .map_err(|e| {
                error!("Ошибка удаления токена из кеша: {}", e);
                AuthError::InternalError("Ошибка удаления токена из кеша".to_string())
            })?;
        
        Ok(())
    }
} 