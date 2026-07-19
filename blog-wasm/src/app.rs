//! Корневой компонент WASM-приложения.

use crate::components::{AuthPanel, NotificationHost, NotificationState, PostsPanel, StatusBadge};
use crate::storage;
use dioxus::prelude::*;

/// Корневой компонент клиентского приложения блога.
#[component]
pub(crate) fn App() -> Element {
    let token = use_signal(storage::get_token_from_storage);
    let notification = use_signal(|| None::<NotificationState>);

    rsx! {
        main { class: "app-shell",
            NotificationHost { notification }

            section { class: "topbar",
                div {
                    h1 { "Blog" }
                    p { "WASM-клиент для HTTP API" }
                }
                StatusBadge { is_authenticated: token.read().is_some() }
            }

            section { class: "workspace",
                AuthPanel { token, notification }
                PostsPanel { token, notification }
            }
        }
    }
}
