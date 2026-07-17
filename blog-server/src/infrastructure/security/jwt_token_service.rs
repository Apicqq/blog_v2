//! Выпуск и проверка JWT-токенов.

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::ports::token_service::TokenService;
use crate::domain::errors::DomainError;

/// Сервис JWT-токенов на симметричном секрете.
#[derive(Debug, Clone)]
pub struct JwtTokenService {
    secret: String,
    ttl: Duration,
}

impl JwtTokenService {
    /// Создает сервис JWT-токенов.
    #[must_use]
    pub const fn new(secret: String, ttl: Duration) -> Self {
        Self { secret, ttl }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}

impl TokenService for JwtTokenService {
    fn issue_new(&self, user_id: Uuid) -> Result<String, DomainError> {
        let issued_at = Utc::now();
        let expires_at = issued_at
            .checked_add_signed(self.ttl)
            .ok_or_else(|| DomainError::Internal("token expiration overflow".to_string()))?;
        let iat = usize::try_from(issued_at.timestamp())
            .map_err(|err| DomainError::Internal(err.to_string()))?;
        let exp = usize::try_from(expires_at.timestamp())
            .map_err(|err| DomainError::Internal(err.to_string()))?;
        let claims = Claims {
            sub: user_id.to_string(),
            iat,
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|err| DomainError::Internal(err.to_string()))
    }

    fn verify(&self, token: &str) -> Result<Uuid, DomainError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| DomainError::Unauthorized)?;

        Uuid::parse_str(&token_data.claims.sub).map_err(|_| DomainError::Unauthorized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_new_creates_verifiable_token() {
        let service = JwtTokenService::new("secret".to_string(), Duration::minutes(15));
        let user_id = Uuid::new_v4();

        let token = service.issue_new(user_id).expect("token should be issued");
        let verified_user_id = service.verify(&token).expect("token should be valid");

        assert_eq!(verified_user_id, user_id);
    }

    #[test]
    fn issue_new_adds_issued_at_claim() {
        let ttl = Duration::minutes(15);
        let service = JwtTokenService::new("secret".to_string(), ttl);

        let token = service
            .issue_new(Uuid::new_v4())
            .expect("token should be issued");
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(service.secret.as_bytes()),
            &Validation::default(),
        )
        .expect("token should be decodable");

        assert!(token_data.claims.iat <= token_data.claims.exp);
        assert_eq!(
            token_data.claims.exp - token_data.claims.iat,
            usize::try_from(ttl.num_seconds()).expect("ttl should fit into usize")
        );
    }

    #[test]
    fn verify_rejects_invalid_token() {
        let service = JwtTokenService::new("secret".to_string(), Duration::minutes(15));

        let result = service.verify("invalid-token");

        assert!(matches!(result, Err(DomainError::Unauthorized)));
    }

    #[test]
    fn verify_rejects_token_signed_with_another_secret() {
        let issuer = JwtTokenService::new("secret".to_string(), Duration::minutes(15));
        let verifier = JwtTokenService::new("another-secret".to_string(), Duration::minutes(15));
        let token = issuer
            .issue_new(Uuid::new_v4())
            .expect("token should be issued");

        let result = verifier.verify(&token);

        assert!(matches!(result, Err(DomainError::Unauthorized)));
    }
}
