//! Сценарии регистрации и входа пользователей.

use std::sync::Arc;

use tracing::{instrument, warn};

use crate::application::ports::password_hasher::PasswordHasher;
use crate::application::ports::token_service::TokenService;
use crate::application::ports::user_repository::UserRepository;
use crate::domain::errors::DomainError;
use crate::domain::user::{LoginCredentials, RegistrationData, User};

/// Результат успешной аутентификации.
#[derive(Debug)]
pub struct AuthSession {
    /// JWT-токен пользователя.
    pub token: String,
    /// Пользователь, для которого выпущен токен.
    pub user: User,
}

impl AuthSession {
    /// Создает результат успешной аутентификации.
    #[must_use]
    pub const fn new(token: String, user: User) -> Self {
        Self { token, user }
    }
}

/// Сервис аутентификации прикладного слоя.
#[derive(Debug, Clone)]
pub struct AuthService<R, H, T> {
    repo: Arc<R>,
    password_hasher: Arc<H>,
    token_service: Arc<T>,
}

impl<R, H, T> AuthService<R, H, T>
where
    R: UserRepository,
    H: PasswordHasher,
    T: TokenService,
{
    /// Создает сервис аутентификации.
    #[must_use]
    pub const fn new(repo: Arc<R>, password_hasher: Arc<H>, token_service: Arc<T>) -> Self {
        Self {
            repo,
            password_hasher,
            token_service,
        }
    }

    /// Регистрирует нового пользователя и выпускает токен доступа.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если пользователь уже существует, пароль не удалось
    /// захешировать, пользователь не может быть сохранен или токен не удалось выпустить.
    #[instrument(skip(self, registration), fields(username = %registration.username()))]
    pub async fn register(
        &self,
        registration: RegistrationData,
    ) -> Result<AuthSession, DomainError> {
        if self
            .repo
            .exists_by_username(registration.username())
            .await?
        {
            warn!("registration rejected: username already exists");
            return Err(DomainError::UsernameAlreadyTaken);
        }

        if self.repo.exists_by_email(registration.email()).await? {
            warn!("registration rejected: email already exists");
            return Err(DomainError::EmailAlreadyTaken);
        }

        let password_hash = self
            .password_hasher
            .hash_password(registration.password())?;
        let user = User::from_registration(registration, password_hash);
        let user = self.repo.create(user).await?;
        let token = self.token_service.issue_new(user.id)?;

        Ok(AuthSession::new(token, user))
    }

    /// Выполняет вход пользователя и выпускает токен доступа.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если пользователь не найден, пароль неверный или токен не
    /// удалось выпустить.
    #[instrument(skip(self, credentials), fields(username = %credentials.username()))]
    pub async fn login(&self, credentials: LoginCredentials) -> Result<AuthSession, DomainError> {
        let user = self
            .repo
            .find_by_username(credentials.username())
            .await?
            .ok_or_else(|| {
                warn!("login rejected: invalid credentials");
                DomainError::InvalidCredentials
            })?;

        let is_valid = self
            .password_hasher
            .verify_password(credentials.password(), &user.password_hash)?;

        if !is_valid {
            warn!("login rejected: invalid credentials");
            return Err(DomainError::InvalidCredentials);
        }

        let token = self.token_service.issue_new(user.id)?;

        Ok(AuthSession::new(token, user))
    }
}
