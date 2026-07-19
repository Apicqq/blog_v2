//! Работа с клиентским состоянием авторизации.

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct JwtClaims {
    sub: String,
}

/// Извлекает идентификатор пользователя из JWT-токена.
#[must_use]
pub(crate) fn user_id_from_token(token: &str) -> Option<String> {
    let payload = token.split('.').nth(1)?;
    let decoded_payload = URL_SAFE_NO_PAD.decode(payload).ok()?;
    let claims = serde_json::from_slice::<JwtClaims>(&decoded_payload).ok()?;

    Some(claims.sub)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_id_from_token_extracts_sub_claim() {
        let payload = URL_SAFE_NO_PAD.encode(r#"{"sub":"user-1"}"#);
        let token = format!("header.{payload}.signature");

        assert_eq!(user_id_from_token(&token).as_deref(), Some("user-1"));
    }

    #[test]
    fn user_id_from_token_returns_none_for_invalid_token() {
        assert_eq!(user_id_from_token("invalid-token"), None);
    }
}
