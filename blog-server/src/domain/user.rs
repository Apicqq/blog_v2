use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Пользователь блога.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Идентификатор пользователя.
    pub id: Uuid,
    /// Имя пользователя.
    pub username: String,
    /// Электронная почта пользователя.
    pub email: String,
    /// Хеш пароля пользователя.
    pub password_hash: String,
    /// Время создания пользователя.
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Создает пользователя из регистрационных данных и хеша пароля.
    #[must_use]
    pub fn from_registration(registration: RegistrationData, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username: registration.username,
            email: registration.email,
            password_hash,
            created_at: Utc::now(),
        }
    }

    /// Проверяет, совпадает ли имя пользователя.
    #[must_use]
    pub fn username_matches(&self, username: &str) -> bool {
        self.username == username
    }
}

/// Данные регистрации пользователя.
#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationData {
    /// Имя пользователя.
    pub username: String,
    /// Электронная почта пользователя.
    pub email: String,
    /// Пароль пользователя.
    pub password: String,
}

impl RegistrationData {
    /// Создает данные регистрации пользователя.
    #[must_use]
    pub const fn new(username: String, email: String, password: String) -> Self {
        Self {
            username,
            email,
            password,
        }
    }
}

/// Учетные данные для входа пользователя.
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    /// Имя пользователя.
    pub username: String,
    /// Пароль пользователя.
    pub password: String,
}

impl LoginCredentials {
    /// Создает учетные данные для входа пользователя.
    #[must_use]
    pub const fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_registration_creates_user_with_expected_fields() {
        let registration = RegistrationData::new(
            "alice".to_string(),
            "alice@example.com".to_string(),
            "password".to_string(),
        );

        let user = User::from_registration(registration, "hashed-password".to_string());

        assert_eq!(user.username, "alice");
        assert_eq!(user.email, "alice@example.com");
        assert_eq!(user.password_hash, "hashed-password");
    }

    #[test]
    fn username_matches_checks_username() {
        let registration = RegistrationData::new(
            "alice".to_string(),
            "alice@example.com".to_string(),
            "password".to_string(),
        );
        let user = User::from_registration(registration, "hashed-password".to_string());

        assert!(user.username_matches("alice"));
        assert!(!user.username_matches("bob"));
    }

    #[test]
    fn login_credentials_keeps_username_and_password() {
        let credentials = LoginCredentials::new("alice".to_string(), "password".to_string());

        assert_eq!(credentials.username, "alice");
        assert_eq!(credentials.password, "password");
    }
}
