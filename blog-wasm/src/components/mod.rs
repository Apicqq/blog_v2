//! Компоненты WASM-приложения.

mod auth;
mod posts;
mod status;

pub(crate) use auth::AuthPanel;
pub(crate) use posts::PostsPanel;
pub(crate) use status::StatusBadge;
