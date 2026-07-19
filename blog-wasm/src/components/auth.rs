//! Компоненты регистрации, входа и выхода.

use crate::api;
use crate::components::NotificationState;
use crate::models::User;
use crate::storage;
use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AuthMode {
    Register,
    Login,
}

/// Кнопки аутентификации и модальное окно входа.
#[component]
pub(crate) fn AuthPanel(
    token: Signal<Option<String>>,
    current_user: Signal<Option<User>>,
    notification: Signal<Option<NotificationState>>,
) -> Element {
    let is_authenticated = token.read().is_some();
    let mut is_dialog_open = use_signal(|| false);

    rsx! {
        div { class: "auth-actions",
            if is_authenticated {
                LogoutButton { token, current_user, notification }
            } else {
                button {
                    class: "secondary",
                    onclick: move |_| is_dialog_open.set(true),
                    "Регистрация"
                }
            }
        }

        if is_dialog_open() && !is_authenticated {
            AuthDialog { token, current_user, notification, is_dialog_open }
        }
    }
}

#[component]
fn AuthDialog(
    token: Signal<Option<String>>,
    current_user: Signal<Option<User>>,
    notification: Signal<Option<NotificationState>>,
    is_dialog_open: Signal<bool>,
) -> Element {
    let mut mode = use_signal(|| AuthMode::Register);

    rsx! {
        div { class: "modal-backdrop",
            article { class: "auth-dialog",
                div { class: "dialog-heading",
                    h2 { if mode() == AuthMode::Register { "Создать аккаунт" } else { "Войти" } }
                    button {
                        class: "icon-button",
                        aria_label: "Закрыть окно входа",
                        onclick: move |_| is_dialog_open.set(false),
                        "×"
                    }
                }

                if mode() == AuthMode::Register {
                    RegisterForm { token, current_user, notification, is_dialog_open }
                    div { class: "auth-switch",
                        span { "Уже зарегистрированы?" }
                        button {
                            class: "text-button",
                            onclick: move |_| mode.set(AuthMode::Login),
                            "Войти"
                        }
                    }
                } else {
                    LoginForm { token, current_user, notification, is_dialog_open }
                    div { class: "auth-switch",
                        span { "Еще нет аккаунта?" }
                        button {
                            class: "text-button",
                            onclick: move |_| mode.set(AuthMode::Register),
                            "Зарегистрироваться"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RegisterForm(
    mut token: Signal<Option<String>>,
    mut current_user: Signal<Option<User>>,
    mut notification: Signal<Option<NotificationState>>,
    mut is_dialog_open: Signal<bool>,
) -> Element {
    let mut username = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);

    rsx! {
        div { class: "auth-form",
            label {
                "Имя пользователя"
                input {
                    value: "{username}",
                    placeholder: "ivan_ivanov",
                    oninput: move |event| username.set(event.value()),
                }
            }
            label {
                "Почта"
                input {
                    value: "{email}",
                    placeholder: "name@example.com",
                    oninput: move |event| email.set(event.value()),
                }
            }
            label {
                "Пароль"
                input {
                    r#type: "password",
                    value: "{password}",
                    placeholder: "Минимум 8 символов",
                    oninput: move |event| password.set(event.value()),
                }
            }
            button {
                onclick: move |_| {
                    let username = username.read().clone();
                    let email = email.read().clone();
                    let password = password.read().clone();

                    spawn(async move {
                        match api::register(&username, &email, &password).await {
                            Ok(response) => {
                                let user = response.user;
                                let current_username = user.username.clone();
                                let current_email = user.email.clone();

                                token.set(Some(response.token));
                                current_user.set(Some(user));
                                is_dialog_open.set(false);
                                notification.set(Some(NotificationState::success(format!(
                                    "Зарегистрирован пользователь {current_username} ({current_email})"
                                ))));
                            }
                            Err(api_error) => {
                                notification.set(Some(NotificationState::error(
                                    api_error.user_message(),
                                )));
                            }
                        }
                    });
                },
                "Зарегистрироваться"
            }
        }
    }
}

#[component]
fn LoginForm(
    mut token: Signal<Option<String>>,
    mut current_user: Signal<Option<User>>,
    mut notification: Signal<Option<NotificationState>>,
    mut is_dialog_open: Signal<bool>,
) -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);

    rsx! {
        div { class: "auth-form",
            label {
                "Имя пользователя"
                input {
                    value: "{username}",
                    placeholder: "ivan_ivanov",
                    oninput: move |event| username.set(event.value()),
                }
            }
            label {
                "Пароль"
                input {
                    r#type: "password",
                    value: "{password}",
                    placeholder: "Минимум 8 символов",
                    oninput: move |event| password.set(event.value()),
                }
            }
            button {
                class: "secondary",
                onclick: move |_| {
                    let username = username.read().clone();
                    let password = password.read().clone();

                    spawn(async move {
                        match api::login(&username, &password).await {
                            Ok(response) => {
                                let user = response.user;
                                let current_username = user.username.clone();
                                let current_email = user.email.clone();

                                token.set(Some(response.token));
                                current_user.set(Some(user));
                                is_dialog_open.set(false);
                                notification.set(Some(NotificationState::success(format!(
                                    "Выполнен вход: {current_username} ({current_email})"
                                ))));
                            }
                            Err(api_error) => {
                                notification.set(Some(NotificationState::error(
                                    api_error.user_message(),
                                )));
                            }
                        }
                    });
                },
                "Войти"
            }
        }
    }
}

#[component]
fn LogoutButton(
    mut token: Signal<Option<String>>,
    mut current_user: Signal<Option<User>>,
    mut notification: Signal<Option<NotificationState>>,
) -> Element {
    rsx! {
        button {
            class: "secondary",
            onclick: move |_| {
                storage::clear_session();
                token.set(None);
                current_user.set(None);
                notification.set(Some(NotificationState::success("Выполнен выход")));
            },
            "Выйти"
        }
    }
}
