//! Компонент списка постов.

use crate::api;
use crate::components::NotificationState;
use crate::errors::ApiError;
use crate::models::{Post, PostPage};
use crate::session;
use crate::validation;
use dioxus::prelude::*;

const POSTS_LIMIT: u64 = 10;

/// Панель списка постов.
#[component]
pub(crate) fn PostsPanel(
    token: Signal<Option<String>>,
    notification: Signal<Option<NotificationState>>,
) -> Element {
    let offset = use_signal(|| 0_u64);
    let posts = use_resource(move || async move { api::list_posts(POSTS_LIMIT, offset()).await });
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
                offset,
                notification,
            }
            PostsList {
                token,
                posts,
                edit_post_id,
                edit_title,
                edit_content,
                offset,
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
    mut offset: Signal<u64>,
    notification: Signal<Option<NotificationState>>,
) -> Element {
    rsx! {
        if token.read().is_some() {
            div { class: "post-form",
                label {
                    "Заголовок"
                    input {
                        value: "{title}",
                        placeholder: "Новый пост",
                        oninput: move |event| title.set(event.value()),
                    }
                }
                label {
                    "Текст"
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

                        if let Some(message) = validation::validate_post(&current_title, &current_content) {
                            set_error(notification, &message);
                            return;
                        }

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
                                    offset.set(0);
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
                    "Опубликовать"
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
    offset: Signal<u64>,
    notification: Signal<Option<NotificationState>>,
) -> Element {
    rsx! {
        div { class: "post-list",
            match posts.read().as_ref() {
                Some(Ok(page)) => rsx! {
                    p { class: "posts-summary", "Всего постов: {page.total}." }
                    if page.posts.is_empty() {
                        div { class: "empty-state",
                            strong { "Постов пока нет" }
                            span { "Но совсем скоро они появятся именно здесь." }
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
                        PaginationControls {
                            total: page.total,
                            limit: page.limit,
                            offset: page.offset,
                            offset_signal: offset,
                        }
                    }
                },
                Some(Err(api_error)) => {
                    let message = api_error.user_message();

                    rsx! {
                        div { class: "error-message",
                            strong { "Не удалось загрузить посты" }
                            span { "{message}" }
                        }
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
fn PaginationControls(
    total: u64,
    limit: u64,
    offset: u64,
    mut offset_signal: Signal<u64>,
) -> Element {
    let current_page = offset / limit + 1;
    let total_pages = total.div_ceil(limit).max(1);
    let has_previous = offset > 0;
    let has_next = offset + limit < total;

    rsx! {
        div { class: "pagination",
            button {
                class: "secondary",
                disabled: !has_previous,
                onclick: move |_| {
                    offset_signal.set(offset.saturating_sub(limit));
                },
                "Назад"
            }
            span { "Страница {current_page} из {total_pages}" }
            button {
                class: "secondary",
                disabled: !has_next,
                onclick: move |_| {
                    offset_signal.set(offset + limit);
                },
                "Вперед"
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

            if is_current_user_author(token, &post.author_id) {
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
                        "Редактировать"
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
                        "Удалить"
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
                "Заголовок"
                input {
                    value: "{edit_title}",
                    oninput: move |event| edit_title.set(event.value()),
                }
            }
            label {
                "Текст"
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

                        if let Some(message) = validation::validate_post(&current_title, &current_content) {
                            set_error(notification, &message);
                            return;
                        }

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
                    "Сохранить"
                }
                button {
                    class: "secondary",
                    onclick: move |_| {
                        edit_post_id.set(None);
                        edit_title.set(String::new());
                        edit_content.set(String::new());
                    },
                    "Отмена"
                }
            }
        }
    }
}

fn is_current_user_author(token: Signal<Option<String>>, author_id: &str) -> bool {
    token
        .read()
        .as_deref()
        .and_then(session::user_id_from_token)
        .is_some_and(|user_id| user_id == author_id)
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
