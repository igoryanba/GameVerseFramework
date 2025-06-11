use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use anyhow::{Result, anyhow};
use rand::{Rng, thread_rng};
use base32;

use crate::config::AuthConfig;
use crate::domain::models::AuthError;

/// Хеширует пароль с использованием Argon2id
pub fn hash_password(password: &str, config: &AuthConfig) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(
            config.password_memory_cost,
            config.password_iterations,
            1,
            Some(config.password_salt_length),
        ).map_err(|e| AuthError::HashingError(e.to_string()))?,
    );
    
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AuthError::HashingError(e.to_string()))?
        .to_string();
    
    Ok(password_hash)
}

/// Проверяет соответствие пароля хешу
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AuthError> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| AuthError::HashingError(e.to_string()))?;
    
    let result = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_or(false, |_| true);
    
    Ok(result)
}

/// Генерирует секрет для TOTP
pub fn generate_totp_secret() -> String {
    let mut rng = thread_rng();
    let secret: Vec<u8> = (0..20).map(|_| rng.gen::<u8>()).collect();
    base32::encode(base32::Alphabet::RFC4648 { padding: false }, &secret)
}

/// Проверяет TOTP код
pub fn verify_totp(secret: &str, code: &str) -> Result<bool> {
    // TODO: Реализовать проверку TOTP кода
    // Это заглушка, которая всегда возвращает true для кода "123456"
    if code == "123456" {
        Ok(true)
    } else {
        Ok(false)
    }
} 