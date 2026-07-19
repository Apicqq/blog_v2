//! Корневой компонент WASM-приложения.

use crate::api;
use crate::components::{AuthPanel, NotificationHost, NotificationState, PostsPanel, StatusBadge};
use crate::models::User;
use crate::storage;
use dioxus::prelude::*;

/// Корневой компонент клиентского приложения блога.
#[component]
pub(crate) fn App() -> Element {
    let token = use_signal(storage::get_token_from_storage);
    let mut current_user = use_signal(|| None::<User>);
    let notification = use_signal(|| None::<NotificationState>);

    use_resource(move || async move {
        let Some(current_token) = token.read().clone() else {
            current_user.set(None);
            return;
        };

        if let Ok(user) = api::current_user(&current_token).await {
            current_user.set(Some(user));
        }
    });

    rsx! {
        main { class: "app-shell",
            NotificationHost { notification }

            section { class: "topbar",
                div {
                    h1 { "Blog" }
                    p { "WASM-клиент для HTTP API" }
                }
                StatusBadge {
                    has_token: token.read().is_some(),
                    username: current_user.read().as_ref().map(|user| user.username.clone()),
                }
            }

            section { class: "workspace",
                AuthPanel { token, current_user, notification }
                PostsPanel { token, notification }
            }
        }
    }
}
