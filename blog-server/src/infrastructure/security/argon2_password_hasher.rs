//! Хеширование паролей через `Argon2`.

use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher as _, PasswordVerifier};
use rand_core::OsRng;

use crate::application::ports::password_hasher::PasswordHasher;
use crate::domain::errors::DomainError;

/// Хешер паролей на базе `Argon2`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Argon2PasswordHasher;

impl Argon2PasswordHasher {
    /// Создает хешер паролей.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl PasswordHasher for Argon2PasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, DomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), &salt)?;

        Ok(hash.to_string())
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, DomainError> {
        let parsed = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();

        Ok(argon2.verify_password(password.as_bytes(), &parsed).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_password_creates_verifiable_hash() {
        let hasher = Argon2PasswordHasher::new();
        let hash = hasher
            .hash_password("password")
            .expect("password should be hashed");

        assert!(
            hasher
                .verify_password("password", &hash)
                .expect("hash should be parsed")
        );
        assert!(
            !hasher
                .verify_password("wrong-password", &hash)
                .expect("hash should be parsed")
        );
    }

    #[test]
    fn verify_password_returns_error_for_invalid_hash() {
        let hasher = Argon2PasswordHasher::new();

        assert!(hasher.verify_password("password", "invalid-hash").is_err());
    }
}
