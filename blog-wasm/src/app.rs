//! Корневой компонент WASM-приложения.

use crate::api;
use crate::storage;
use dioxus::prelude::*;

/// Корневой компонент клиентского приложения блога.
#[component]
pub(crate) fn App() -> Element {
    let posts = use_resource(|| async { api::list_posts(10, 0).await });
    let token = use_signal(storage::get_token_from_storage);

    rsx! {
        main { class: "app-shell",
            section { class: "topbar",
                div {
                    h1 { "Blog" }
                    p { "WASM-клиент для HTTP API" }
                }
                div { class: "status-pill", if token.read().is_some() { "Залогинен" } else { "Гость" } }
            }

            section { class: "workspace",
                article { class: "auth-panel",
                    h2 { "Аккаунт" }
                    label {
                        "Username"
                        input { placeholder: "very_cool_username" }
                    }
                    label {
                        "Email"
                        input { placeholder: "name@example.com" }
                    }
                    label {
                        "Password"
                        input { r#type: "password", placeholder: "minimum 8 characters" }
                    }
                    div { class: "button-row",
                        button { "Register" }
                        button { class: "secondary", "Login" }
                    }
                }

                article { class: "posts-panel",
                    div { class: "panel-heading",
                        h2 { "Посты" }
                        button { "New post" }
                    }
                    div { class: "post-list",
                        match posts.read().as_ref() {
                            Some(Ok(page)) => rsx! {
                                p { class: "posts-summary", "Всего постов: {page.total}. Лимит: {page.limit}, смещение: {page.offset}." }
                                if page.posts.is_empty() {
                                    div { class: "empty-state",
                                        strong { "Постов пока нет" }
                                        span { "Когда в API появятся посты, они отобразятся здесь." }
                                    }
                                } else {
                                    for post in &page.posts {
                                        article { class: "post-item", key: "{post.id}",
                                            strong { "{post.title}" }
                                            span { "Автор: {post.author_id}" }
                                            span { "Создан: {post.created_at}" }
                                            if let Some(updated_at) = &post.updated_at {
                                                span { "Обновлен: {updated_at}" }
                                            }
                                            p { "{post.content}" }
                                        }
                                    }
                                }
                            },
                            Some(Err(error)) => rsx! {
                                div { class: "error-message",
                                    strong { "Не удалось загрузить посты" }
                                    span { "{error}" }
                                }
                            },
                            None => rsx! {
                                p { "Загружаем посты..." }
                            },
                        }
                    }
                }
            }
        }
    }
}
