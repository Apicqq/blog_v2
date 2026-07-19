//! Компонент статуса аутентификации.

use dioxus::prelude::*;

/// Индикатор статуса аутентификации.
#[component]
pub(crate) fn StatusBadge(has_token: bool, username: Option<String>) -> Element {
    rsx! {
        div { class: "status-pill",
            if let Some(username) = username {
                "@{username}"
            } else if has_token {
                "Залогинен"
            } else {
                "Гость"
            }
        }
    }
}
