//! Преобразование ошибок слоя хранения.

use sea_orm::DbErr;

use crate::domain::errors::DomainError;

impl From<DbErr> for DomainError {
    fn from(err: DbErr) -> Self {
        Self::Internal(err.to_string())
    }
}
