use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::ValidateEmail;

use crate::domain::errors::DomainError;
// трейдофф чтобы самому не писать логику валидации по стандарту RFC 5322

const MIN_USERNAME_LENGTH: usize = 3;
const MAX_USERNAME_LENGTH: usize = 32;
const MAX_EMAIL_LENGTH: usize = 128;
const MIN_PASSWORD_LENGTH: usize = 8;
const MAX_PASSWORD_LENGTH: usize = 128;

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
        let (username, email, _) = registration.into_parts();

        Self {
            id: Uuid::new_v4(),
            username,
            email,
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

/// Валидное имя пользователя.
#[derive(Debug, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    /// Создает валидное имя пользователя.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку валидации, если имя пользователя короче 3 или длиннее 32 символов.
    pub fn parse(value: &str) -> Result<Self, DomainError> {
        let value = value.trim().to_string();
        let length = value.chars().count();

        if length < MIN_USERNAME_LENGTH {
            return Err(DomainError::Validation(
                "username must contain at least 3 characters".to_string(),
            ));
        }

        if length > MAX_USERNAME_LENGTH {
            return Err(DomainError::Validation(
                "username must contain at most 32 characters".to_string(),
            ));
        }

        Ok(Self(value))
    }

    /// Преобразует имя пользователя в строку.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

/// Валидная электронная почта пользователя.
#[derive(Debug, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    /// Создает валидную электронную почту пользователя.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку валидации, если адрес пустой, длиннее 128 символов или не похож на email.
    pub fn parse(value: &str) -> Result<Self, DomainError> {
        let value = value.trim().to_lowercase();
        let length = value.chars().count();

        if length == 0 {
            return Err(DomainError::Validation(
                "email must not be empty".to_string(),
            ));
        }

        if length > MAX_EMAIL_LENGTH {
            return Err(DomainError::Validation(
                "email must contain at most 128 characters".to_string(),
            ));
        }

        if !value.validate_email() {
            return Err(DomainError::Validation("email is invalid".to_string()));
        }

        Ok(Self(value))
    }

    /// Преобразует электронную почту в строку.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

/// Валидный пароль в открытом виде.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlainPassword(String);

impl PlainPassword {
    /// Создает валидный пароль в открытом виде.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку валидации, если пароль короче 8 или длиннее 128 символов.
    pub fn parse(value: String) -> Result<Self, DomainError> {
        let length = value.chars().count();

        if length < MIN_PASSWORD_LENGTH {
            return Err(DomainError::Validation(
                "password must contain at least 8 characters".to_string(),
            ));
        }

        if length > MAX_PASSWORD_LENGTH {
            return Err(DomainError::Validation(
                "password must contain at most 128 characters".to_string(),
            ));
        }

        Ok(Self(value))
    }

    /// Преобразует пароль в строку.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

/// Данные регистрации пользователя.
#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationData {
    /// Имя пользователя.
    username: Username,
    /// Электронная почта пользователя.
    email: Email,
    /// Пароль пользователя.
    password: PlainPassword,
}

impl RegistrationData {
    /// Создает данные регистрации пользователя.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку валидации, если имя пользователя, email или пароль некорректны.
    pub fn new(username: &str, email: &str, password: String) -> Result<Self, DomainError> {
        Ok(Self {
            username: Username::parse(username)?,
            email: Email::parse(email)?,
            password: PlainPassword::parse(password)?,
        })
    }

    /// Возвращает имя пользователя.
    #[must_use]
    pub fn username(&self) -> &str {
        &self.username.0
    }

    /// Возвращает электронную почту пользователя.
    #[must_use]
    pub fn email(&self) -> &str {
        &self.email.0
    }

    /// Возвращает пароль пользователя.
    #[must_use]
    pub fn password(&self) -> &str {
        &self.password.0
    }

    /// Разбирает данные регистрации на строки.
    #[must_use]
    pub fn into_parts(self) -> (String, String, String) {
        (
            self.username.into_inner(),
            self.email.into_inner(),
            self.password.into_inner(),
        )
    }
}

/// Учетные данные для входа пользователя.
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    /// Имя пользователя.
    username: Username,
    /// Пароль пользователя.
    password: PlainPassword,
}

impl LoginCredentials {
    /// Создает учетные данные для входа пользователя.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку валидации, если имя пользователя или пароль некорректны.
    pub fn new(username: &str, password: String) -> Result<Self, DomainError> {
        Ok(Self {
            username: Username::parse(username)?,
            password: PlainPassword::parse(password)?,
        })
    }

    /// Возвращает имя пользователя.
    #[must_use]
    pub fn username(&self) -> &str {
        &self.username.0
    }

    /// Возвращает пароль пользователя.
    #[must_use]
    pub fn password(&self) -> &str {
        &self.password.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn username_trims_valid_value() {
        let username = Username::parse("  alice  ").expect("username should be valid");

        assert_eq!(username.into_inner(), "alice");
    }

    #[test]
    fn username_rejects_short_value() {
        let error = Username::parse("ab").expect_err("short username should be rejected");

        assert!(matches!(error, DomainError::Validation(_)));
    }

    #[test]
    fn email_normalizes_valid_value() {
        let email = Email::parse("  Alice@Example.COM  ").expect("email should be valid");

        assert_eq!(email.into_inner(), "alice@example.com");
    }

    #[test]
    fn email_rejects_invalid_value() {
        let error = Email::parse("invalid-email").expect_err("invalid email should be rejected");

        assert!(matches!(error, DomainError::Validation(_)));
    }

    #[test]
    fn plain_password_rejects_short_value() {
        let error = PlainPassword::parse("short".to_string())
            .expect_err("short password should be rejected");

        assert!(matches!(error, DomainError::Validation(_)));
    }

    #[test]
    fn from_registration_creates_user_with_expected_fields() {
        let registration =
            RegistrationData::new("alice", "alice@example.com", "password".to_string())
                .expect("registration data should be valid");

        let user = User::from_registration(registration, "hashed-password".to_string());

        assert_eq!(user.username, "alice");
        assert_eq!(user.email, "alice@example.com");
        assert_eq!(user.password_hash, "hashed-password");
    }

    #[test]
    fn username_matches_checks_username() {
        let registration =
            RegistrationData::new("alice", "alice@example.com", "password".to_string())
                .expect("registration data should be valid");
        let user = User::from_registration(registration, "hashed-password".to_string());

        assert!(user.username_matches("alice"));
        assert!(!user.username_matches("bob"));
    }

    #[test]
    fn login_credentials_keeps_username_and_password() {
        let credentials = LoginCredentials::new("alice", "password".to_string())
            .expect("login credentials should be valid");

        assert_eq!(credentials.username(), "alice");
        assert_eq!(credentials.password(), "password");
    }
}
