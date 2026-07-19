//! Компонент списка постов.

use crate::api;
use crate::components::NotificationState;
use crate::errors::ApiError;
use crate::models::{Post, PostPage};
use dioxus::prelude::*;

/// Панель списка постов.
#[component]
pub(crate) fn PostsPanel(
    token: Signal<Option<String>>,
    notification: Signal<Option<NotificationState>>,
) -> Element {
    let posts = use_resource(|| async { api::list_posts(10, 0).await });
    let title = use_signal(String::new);
    let content = use_signal(String::new);
    let edit_post_id = use_signal(|| None::<i64>);
    let edit_title = use_signal(String::new);
    let edit_content = use_signal(String::new);

    rsx! {
        article { class: "posts-panel",
            div { class: "panel-heading",
                h2 { "Посты" }
            }

            CreatePostForm {
                token,
                posts,
                title,
                content,
                notification,
            }
            PostsList {
                token,
                posts,
                edit_post_id,
                edit_title,
                edit_content,
                notification,
            }
        }
    }
}

#[component]
fn CreatePostForm(
    token: Signal<Option<String>>,
    mut posts: Resource<Result<PostPage, ApiError>>,
    mut title: Signal<String>,
    mut content: Signal<String>,
    notification: Signal<Option<NotificationState>>,
) -> Element {
    rsx! {
        if token.read().is_some() {
            div { class: "post-form",
                label {
                    "Title"
                    input {
                        value: "{title}",
                        placeholder: "Новый пост",
                        oninput: move |event| title.set(event.value()),
                    }
                }
                label {
                    "Content"
                    textarea {
                        value: "{content}",
                        placeholder: "Текст поста",
                        oninput: move |event| content.set(event.value()),
                    }
                }
                button {
                    onclick: move |_| {
                        let Some(current_token) = token.read().clone() else {
                            set_error(notification, "Нужно войти, чтобы создать пост");
                            return;
                        };
                        let current_title = title.read().clone();
                        let current_content = content.read().clone();

                        spawn(async move {
                            match api::create_post(&current_token, &current_title, &current_content)
                                .await
                            {
                                Ok(post) => {
                                    title.set(String::new());
                                    content.set(String::new());
                                    notification.set(Some(NotificationState::success(format!(
                                        "Пост создан: {}",
                                        post.title
                                    ))));
                                    posts.restart();
                                }
                                Err(api_error) => {
                                    notification.set(Some(NotificationState::error(
                                        api_error.user_message(),
                                    )));
                                }
                            }
                        });
                    },
                    "Create post"
                }
            }
        } else {
            div { class: "empty-state",
                strong { "Создание постов недоступно" }
                span { "Войдите или зарегистрируйтесь, чтобы опубликовать пост." }
            }
        }
    }
}

#[component]
fn PostsList(
    token: Signal<Option<String>>,
    posts: Resource<Result<PostPage, ApiError>>,
    edit_post_id: Signal<Option<i64>>,
    edit_title: Signal<String>,
    edit_content: Signal<String>,
    notification: Signal<Option<NotificationState>>,
) -> Element {
    rsx! {
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
                            PostCard {
                                key: "{post.id}",
                                post: post.clone(),
                                token,
                                posts,
                                edit_post_id,
                                edit_title,
                                edit_content,
                                notification,
                            }
                        }
                    }
                },
                Some(Err(api_error)) => rsx! {
                    div { class: "error-message",
                        strong { "Не удалось загрузить посты" }
                        span { "{api_error}" }
                    }
                },
                None => rsx! {
                    p { "Загружаем посты..." }
                },
            }
        }
    }
}

#[component]
fn PostCard(
    post: Post,
    token: Signal<Option<String>>,
    mut posts: Resource<Result<PostPage, ApiError>>,
    mut edit_post_id: Signal<Option<i64>>,
    mut edit_title: Signal<String>,
    mut edit_content: Signal<String>,
    notification: Signal<Option<NotificationState>>,
) -> Element {
    rsx! {
        article { class: "post-item",
            strong { "{post.title}" }
            span { "Автор: {post.author_username}" }
            span { "Создан: {format_datetime(&post.created_at)}" }
            if let Some(updated_at) = &post.updated_at {
                span { "Обновлен: {format_datetime(updated_at)}" }
            }
            p { "{post.content}" }

            if token.read().is_some() {
                div { class: "post-actions",
                    button {
                        class: "secondary",
                        onclick: {
                            let post_id = post.id;
                            let post_title = post.title.clone();
                            let post_content = post.content.clone();

                            move |_| {
                                edit_post_id.set(Some(post_id));
                                edit_title.set(post_title.clone());
                                edit_content.set(post_content.clone());
                                notification.set(None);
                            }
                        },
                        "Edit"
                    }
                    button {
                        class: "danger",
                        onclick: {
                            let post_id = post.id;

                            move |_| {
                                let Some(current_token) = token.read().clone() else {
                                    set_error(notification, "Нужно войти, чтобы удалить пост");
                                    return;
                                };

                                spawn(async move {
                                    match api::delete_post(&current_token, post_id).await {
                                        Ok(()) => {
                                            notification.set(Some(NotificationState::success(
                                                "Пост удален",
                                            )));
                                            if edit_post_id.read().is_some_and(|id| id == post_id) {
                                                edit_post_id.set(None);
                                            }
                                            posts.restart();
                                        }
                                        Err(api_error) => {
                                            notification.set(Some(NotificationState::error(
                                                api_error.user_message(),
                                            )));
                                        }
                                    }
                                });
                            }
                        },
                        "Delete"
                    }
                }
            }

            if edit_post_id.read().is_some_and(|id| id == post.id) {
                EditPostForm {
                    post_id: post.id,
                    token,
                    posts,
                    edit_post_id,
                    edit_title,
                    edit_content,
                    notification,
                }
            }
        }
    }
}

#[component]
fn EditPostForm(
    post_id: i64,
    token: Signal<Option<String>>,
    mut posts: Resource<Result<PostPage, ApiError>>,
    mut edit_post_id: Signal<Option<i64>>,
    mut edit_title: Signal<String>,
    mut edit_content: Signal<String>,
    notification: Signal<Option<NotificationState>>,
) -> Element {
    rsx! {
        div { class: "edit-form",
            label {
                "Title"
                input {
                    value: "{edit_title}",
                    oninput: move |event| edit_title.set(event.value()),
                }
            }
            label {
                "Content"
                textarea {
                    value: "{edit_content}",
                    oninput: move |event| edit_content.set(event.value()),
                }
            }
            div { class: "post-actions",
                button {
                    onclick: move |_| {
                        let Some(current_token) = token.read().clone() else {
                            set_error(notification, "Нужно войти, чтобы обновить пост");
                            return;
                        };
                        let current_title = edit_title.read().clone();
                        let current_content = edit_content.read().clone();

                        spawn(async move {
                            match api::update_post(
                                &current_token,
                                post_id,
                                &current_title,
                                &current_content,
                            )
                            .await
                            {
                                Ok(post) => {
                                    edit_post_id.set(None);
                                    notification.set(Some(NotificationState::success(format!(
                                        "Пост обновлен: {}",
                                        post.title
                                    ))));
                                    posts.restart();
                                }
                                Err(api_error) => {
                                    notification.set(Some(NotificationState::error(
                                        api_error.user_message(),
                                    )));
                                }
                            }
                        });
                    },
                    "Save"
                }
                button {
                    class: "secondary",
                    onclick: move |_| {
                        edit_post_id.set(None);
                        edit_title.set(String::new());
                        edit_content.set(String::new());
                    },
                    "Cancel"
                }
            }
        }
    }
}

fn set_error(mut notification: Signal<Option<NotificationState>>, value: &str) {
    notification.set(Some(NotificationState::error(value)));
}

fn format_datetime(value: &str) -> String {
    let without_timezone = value
        .strip_suffix('Z')
        .unwrap_or(value)
        .split_once('.')
        .map_or(value, |(datetime, _)| datetime);

    without_timezone.replace('T', " ")
}
