//! Компоненты WASM-приложения.

mod auth;
mod notification;
mod posts;
mod status;

pub(crate) use auth::AuthPanel;
pub(crate) use notification::{NotificationHost, NotificationState};
pub(crate) use posts::PostsPanel;
pub(crate) use status::StatusBadge;
