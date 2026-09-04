//! Password and opaque-session-token primitives for the alpha control plane.
use anyhow::{Context, Result};
use argon2::{
    password_hash::{phc::PasswordHash, PasswordHasher, PasswordVerifier},
    Algorithm, Argon2, Params, Version,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const ACCESS_TTL: Duration = Duration::from_secs(15 * 60);
pub const REFRESH_TTL: Duration = Duration::from_secs(30 * 24 * 60 * 60);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub access_expires_at_ms: u64,
    pub refresh_expires_at_ms: u64,
}

pub fn hash_password(password: &str) -> Result<String> {
    anyhow::ensure!(
        (8..=256).contains(&password.len()),
        "invalid password length"
    );
    let params = Params::new(64 * 1024, 3, 1, None).context("argon2 parameters")?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    Ok(argon2
        .hash_password(password.as_bytes())
        .context("hash password")?
        .to_string())
}

pub fn verify_password(password: &str, encoded: &str) -> Result<bool> {
    let hash = PasswordHash::new(encoded).context("parse password hash")?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok())
}

pub fn issue_tokens(now: SystemTime) -> Result<TokenPair> {
    let now_ms = now
        .duration_since(UNIX_EPOCH)
        .context("clock precedes unix epoch")?
        .as_millis() as u64;
    Ok(TokenPair {
        access_token: random_token(),
        refresh_token: random_token(),
        access_expires_at_ms: now_ms + ACCESS_TTL.as_millis() as u64,
        refresh_expires_at_ms: now_ms + REFRESH_TTL.as_millis() as u64,
    })
}

pub fn token_hash(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn invite_hash(invite: &str) -> String {
    token_hash(invite.trim())
}

pub fn issue_invite() -> String {
    random_token()
}

fn random_token() -> String {
    let mut bytes = [0_u8; 32];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_hash_uses_required_argon2id_cost_and_verifies() {
        let encoded = hash_password("correct horse battery staple").unwrap();
        assert!(encoded.starts_with("$argon2id$v=19$m=65536,t=3,p=1$"));
        assert!(verify_password("correct horse battery staple", &encoded).unwrap());
        assert!(!verify_password("wrong password", &encoded).unwrap());
    }

    #[test]
    fn tokens_are_opaque_unique_and_only_the_hash_is_stable() {
        let first = issue_tokens(UNIX_EPOCH + Duration::from_secs(1_000)).unwrap();
        let second = issue_tokens(UNIX_EPOCH + Duration::from_secs(1_000)).unwrap();
        assert_ne!(first.access_token, second.access_token);
        assert_ne!(first.refresh_token, second.refresh_token);
        assert_eq!(first.access_token.len(), 43);
        assert_eq!(token_hash(&first.refresh_token).len(), 64);
        assert_eq!(first.access_expires_at_ms, 1_900_000);
        assert_eq!(first.refresh_expires_at_ms, 2_593_000_000);
    }
}
