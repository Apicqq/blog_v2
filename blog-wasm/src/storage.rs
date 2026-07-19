//! Хранение токена пользователя в браузере.

use gloo_storage::{LocalStorage, Storage};

const TOKEN_KEY: &str = "blog_token";

pub(crate) fn get_token_from_storage() -> Option<String> {
    LocalStorage::get(TOKEN_KEY).ok()
}

#[allow(dead_code)]
pub(crate) fn save_token_to_storage(token: &str) {
    let _ = LocalStorage::set(TOKEN_KEY, token).ok();
}

pub(crate) fn clear_session() {
    LocalStorage::delete(TOKEN_KEY);
}
