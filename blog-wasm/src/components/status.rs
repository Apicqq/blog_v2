//! Компонент статуса аутентификации.

use dioxus::prelude::*;

/// Индикатор статуса аутентификации.
#[component]
pub(crate) fn StatusBadge(is_authenticated: bool) -> Element {
    rsx! {
        div { class: "status-pill",
            if is_authenticated { "Залогинен" } else { "Гость" }
        }
    }
}
