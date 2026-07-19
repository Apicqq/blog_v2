//! Компонент регистрации и входа.

use crate::api;
use crate::components::NotificationState;
use crate::models::User;
use crate::storage;
use dioxus::prelude::*;

/// Панель регистрации и входа пользователя.
#[component]
pub(crate) fn AuthPanel(
    token: Signal<Option<String>>,
    current_user: Signal<Option<User>>,
    notification: Signal<Option<NotificationState>>,
) -> Element {
    rsx! {
        article { class: "auth-panel",
            h2 { "Аккаунт" }

            RegisterForm { token, current_user, notification }
            LoginForm { token, current_user, notification }
            LogoutButton { token, current_user, notification }
        }
    }
}

#[component]
fn RegisterForm(
    mut token: Signal<Option<String>>,
    mut current_user: Signal<Option<User>>,
    mut notification: Signal<Option<NotificationState>>,
) -> Element {
    let mut username = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);

    rsx! {
        div { class: "auth-form",
            label {
                "Username"
                input {
                    value: "{username}",
                    placeholder: "very_cool_username",
                    oninput: move |event| username.set(event.value()),
                }
            }
            label {
                "Email"
                input {
                    value: "{email}",
                    placeholder: "name@example.com",
                    oninput: move |event| email.set(event.value()),
                }
            }
            label {
                "Password"
                input {
                    r#type: "password",
                    value: "{password}",
                    placeholder: "minimum 8 characters",
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
                "Register"
            }
        }
    }
}

#[component]
fn LoginForm(
    mut token: Signal<Option<String>>,
    mut current_user: Signal<Option<User>>,
    mut notification: Signal<Option<NotificationState>>,
) -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);

    rsx! {
        div { class: "auth-form",
            label {
                "Username"
                input {
                    value: "{username}",
                    placeholder: "very_cool_username",
                    oninput: move |event| username.set(event.value()),
                }
            }
            label {
                "Password"
                input {
                    r#type: "password",
                    value: "{password}",
                    placeholder: "minimum 8 characters",
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
                "Login"
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
        if token.read().is_some() {
            button {
                class: "secondary",
                onclick: move |_| {
                    storage::clear_session();
                    token.set(None);
                    current_user.set(None);
                    notification.set(Some(NotificationState::success("Выполнен выход")));
                },
                "Logout"
            }
        }
    }
}
